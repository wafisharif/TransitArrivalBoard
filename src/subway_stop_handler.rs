use std::{
  cmp::Ordering,
  collections::BTreeMap,
  sync::{Arc, RwLock},
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{feed_handler::FeedHandler, Stop, Vehicle};

#[derive(Default)]
pub struct SubwayStopHandler {
  feed_data: Arc<RwLock<FeedHandler>>,
  pub stop_ids: Vec<String>,
  pub trips: Vec<Vehicle>,
  pub destinations: BTreeMap<String, BTreeMap<String, Vec<Vehicle>>>,
}

impl SubwayStopHandler {
  pub fn new(feed_data: Arc<RwLock<FeedHandler>>, stop_ids: Vec<String>) -> Self {
    Self {
      feed_data,
      stop_ids,
      ..Default::default()
    }
  }

  pub fn refresh(&mut self) {
    if self.feed_data.read().unwrap().subway_realtime_feed.is_none() {
      self.predict();
      return;
    }

    let mut trips: Vec<Vehicle> = Default::default();
    let mut destinations: BTreeMap<String, BTreeMap<String, Vec<Vehicle>>> = Default::default();

    // Crash if UNIX_EPOCH fails to convert to i64
    let current_time = i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()).unwrap();

    // Crash if feed_data becomes poisoned
    let data = self.feed_data.read().unwrap();

    for message in data.subway_realtime_feed.as_ref().unwrap().to_owned().iter() {
      for entity in &message.entity {
        if let Some(trip_update) = &entity.trip_update {
          for stop in trip_update.stop_time_update.iter() {
            if self.stop_ids.contains(&stop.stop_id().to_string()) {
              // Duration
              let arrival_time = (match &stop.arrival {
                Some(a) => a,
                None => continue,
              })
              .time();
              let duration = ((arrival_time - current_time).max(0) / 60) as i32;

              // Route
              let route_id = trip_update
                .trip
                .trip_id()
                .split('_')
                .last()
                .unwrap() // No station name can be "_"
                .split("..")
                .next()
                .unwrap(); // No station name can be "_.."
              let route_name = match data.subway_static_feed.get_route(route_id) {
                // Theoretically, all routes should have a route name
                Ok(route_name) => route_name.short_name.as_ref().unwrap(),
                Err(_) => continue,
              };

              // Destination
              // Should have atleast the station we are looking at, thus should not fail
              let destination_id = trip_update.stop_time_update.last().unwrap().stop_id();
              let destination_name = match data.subway_static_feed.get_stop(destination_id) {
                // Theoritcally, all destination ids should reference a name
                Ok(a) => a.name.as_ref().unwrap(),
                Err(_) => return,
              };

              // Input data into trips
              trips.push(Vehicle {
                route_id: route_id.to_owned(),
                route_name: route_name.to_owned(),
                destination_id: destination_id.to_owned(),
                destination_name: destination_name.to_owned(),
                minutes_until_arrival: duration,
              });

              // All data from lines 90-110 are verified via the if statements
              // Input data into routes
              if !destinations.contains_key(route_id) {
                destinations.insert(route_id.to_owned(), BTreeMap::new());
              }
              if !destinations.get(route_id).unwrap().contains_key(destination_id) {
                destinations
                  .get_mut(route_id)
                  .unwrap()
                  .insert(destination_id.to_owned(), Vec::new());
              };
              destinations
                .get_mut(route_id)
                .unwrap()
                .get_mut(destination_id)
                .unwrap()
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
    Stop {
      name: self
        .feed_data
        .read()
        .unwrap() // If RwLock poisoned, server should crash
        .subway_static_feed
        .get_stop(self.stop_ids.first().unwrap())
        .unwrap() // Theoretically every stop id should have a stop name
        .name
        .as_ref()
        .unwrap() // A stop should have name
        .to_string(),
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
