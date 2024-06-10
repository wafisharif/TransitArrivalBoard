use chrono::DateTime;

use crate::{feed_handler::FeedHandler, siri_structs::BusData, Stop, Vehicle};
use std::{
  cmp::Ordering,
  collections::BTreeMap,
  sync::{Arc, RwLock},
  time::{SystemTime, UNIX_EPOCH},
};

#[derive(Default)]
pub struct BusStopHandler {
  feed_data: Arc<RwLock<FeedHandler>>,
  api_key: Arc<String>,
  pub stop_ids: Vec<String>,
  pub trips: Vec<Vehicle>,
  pub destinations: BTreeMap<String, BTreeMap<String, Vec<Vehicle>>>,
}

impl BusStopHandler {
  pub fn new(feed_data: Arc<RwLock<FeedHandler>>, api_key: Arc<String>, stop_ids: Vec<String>) -> Self {
    Self {
      feed_data,
      api_key,
      stop_ids,
      ..Default::default()
    }
  }

  pub fn refresh(&mut self) {
    let mut trips: Vec<Vehicle> = Default::default();
    let mut destinations: BTreeMap<String, BTreeMap<String, Vec<Vehicle>>> = Default::default();

    // If we cannot get a duration from the UNIX_EPOCH, or make that into an i64, we have problems.
    // Should not fail else we can't get any time data at all.
    let current_time = i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()).unwrap();

    for id in self.stop_ids.to_owned().iter() {
      let resp = match minreq::get("https://bustime.mta.info/api/siri/stop-monitoring.json")
        .with_param("key", &*self.api_key)
        .with_param("version", "2")
        .with_param("OperatorRef", "MTA")
        .with_param("MonitoringRef", id)
        .send()
      {
        Ok(a) => a,
        Err(_) => {
          self.predict();
          return;
        }
      };
      let bytes = resp.as_bytes();
      let data: BusData = match serde_json::from_slice(bytes) {
        Ok(a) => a,
        Err(_) => {
          return;
        }
      };

      if let Some(service_delivery) = data.siri.service_delivery {
        for stop_data in service_delivery.stop_monitoring_delivery {
          if let Some(monitored_visit) = stop_data.monitored_stop_visit {
            for visit in monitored_visit {
              // Duration
              let arrival_time = match visit.monitored_vehicle_journey.monitored_call.expected_departure_time {
                Some(a) => a,
                None => continue,
              };

              // If parsing fails, then the MTA Bus API may no longer use iso8601 for times
              let arrival_time = DateTime::parse_from_rfc3339(arrival_time.as_str())
              .expect("Error parsing the bus time data.\nIf this happens consistently contact the board maintainers (Error line 91, bus_stop_handler.rs)")
              .timestamp();
              let duration = ((arrival_time - current_time).max(0) / 60) as i32;

              // Route
              let route_id = visit
                .monitored_vehicle_journey
                .line_ref
                .split('_')
                .last()
                .unwrap_or(&visit.monitored_vehicle_journey.line_ref);
              let route_name = visit.monitored_vehicle_journey.published_line_name.first().unwrap();

              // Destination
              let destination_id = visit
                .monitored_vehicle_journey
                .destination_ref
                .split('_')
                .last() // There should be at least one element
                .unwrap(); // However, there should actually be two, because of MTA formating
              let destination_name = visit
                .monitored_vehicle_journey
                .destination_name
                .first() // Should have a destination_name
                .unwrap();

              // Input data into trips
              trips.push(Vehicle {
                route_id: route_id.to_owned(),
                route_name: route_name.to_owned(),
                destination_id: destination_id.to_owned(),
                destination_name: destination_name.to_owned(),
                minutes_until_arrival: duration,
              });

              // Input data into routes
              if !destinations.contains_key(route_id) {
                destinations.insert(route_id.to_owned(), BTreeMap::new());
              }
              if !destinations.get(route_id).unwrap().contains_key(destination_id) {
                // Key must exist because of above line
                destinations
                  .get_mut(route_id) // Key must exist for same reason above
                  .unwrap()
                  .insert(destination_id.to_owned(), Vec::new());
              };

              destinations
                .get_mut(route_id) // Key exists
                .unwrap()
                .get_mut(destination_id)
                .unwrap() // Key exists due to above if statement check
                .push(Vehicle {
                  route_id: route_id.to_owned(),
                  route_name: route_name.to_owned(),
                  destination_id: destination_id.to_owned(),
                  destination_name: destination_name.to_owned(),
                  minutes_until_arrival: duration,
                });
            }
          }
        }
      }
    }

    trips.sort_by(|a, b| {
      if a.minutes_until_arrival > b.minutes_until_arrival {
        return Ordering::Greater;
      }
      if a.minutes_until_arrival < b.minutes_until_arrival {
        return Ordering::Less;
      }
      Ordering::Equal
    });

    self.trips = trips;
    self.destinations = destinations;
  }

  pub fn serialize(&self) -> Stop {
    let mut name: String = Default::default();
    // If RwLock fail, then something went wrong
    let feed_data = self.feed_data.read().unwrap();
    for gtfs in feed_data.bus_static_feed.iter() {
      // We just added a stop id in the refresh function, so exists
      name = match gtfs.get_stop(self.stop_ids.first().unwrap()) {
        Ok(a) => a.name.to_owned().unwrap(),
        Err(_) => continue,
      };
    }

    Stop {
      name: name.to_owned(),
      trips: self.trips.to_owned(),
      destinations: self.destinations.to_owned(),
    }
  }

  pub fn predict(&mut self) {
    self.destinations.clear();

    for trip in &mut self.trips {
      trip.minutes_until_arrival -= 1;

      if trip.minutes_until_arrival < 0 {
        continue;
      }

      // All the unwraps are verified between lines 161-191
      if !self.destinations.contains_key(&trip.route_id) {
        self.destinations.insert(trip.route_id.to_owned(), BTreeMap::new());
      }
      if !self
        .destinations
        .get(&trip.route_id)
        .unwrap()
        .contains_key(&trip.destination_id)
      {
        self
          .destinations
          .get_mut(&trip.route_id)
          .unwrap()
          .insert(trip.destination_id.to_owned(), Vec::new());
      };
      self
        .destinations
        .get_mut(&trip.route_id)
        .unwrap()
        .get_mut(&trip.destination_id)
        .unwrap()
        .push(Vehicle {
          route_id: trip.route_id.to_owned(),
          route_name: trip.route_name.to_owned(),
          destination_id: trip.destination_id.to_owned(),
          destination_name: trip.destination_name.to_owned(),
          minutes_until_arrival: trip.minutes_until_arrival,
        });
    }

    self.trips.retain(|a| a.minutes_until_arrival >= 0);
  }
}
