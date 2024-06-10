use std::io::Cursor;

use gtfs_structures::Gtfs;
use prost::Message;

use crate::{gtfsrt, mercury_structs::MercuryDelays};

// No bus because bus api can be queried per stop
#[derive(Default)]
pub struct FeedHandler {
  pub subway_static_feed: Gtfs,
  pub subway_realtime_feed: Option<Vec<gtfsrt::FeedMessage>>,
  pub bus_static_feed: Vec<Gtfs>,
  pub service_alerts_realtime_feed: Option<MercuryDelays>,
}

impl FeedHandler {
  pub fn new() -> Self {
    FeedHandler { ..Default::default() }
  }

  pub fn refresh_realtime(&mut self) {
    self.refresh_subway_realtime();
    self.refresh_service_alerts_realtime()
  }

  fn refresh_subway_realtime(&mut self) {
    let mut subway: Vec<gtfsrt::FeedMessage> = Default::default();
    let feed_uris = [
      "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-ace",
      "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-bdfm",
      "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-g",
      "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-jz",
      "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-nqrw",
      "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-l",
      "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs",
    ];
    for uri in feed_uris {
      let resp = match minreq::get(uri).send() {
        Ok(a) => a,
        Err(_) => {
          self.subway_realtime_feed = None;
          return;
        } // HTTP request failed.
      };
      let bytes = resp.as_bytes();
      let feed = match gtfsrt::FeedMessage::decode(bytes) {
        Ok(a) => a,
        Err(_) => {
          self.subway_realtime_feed = None;
          return;
        }
      };
      subway.push(feed);
    }

    self.subway_realtime_feed = Some(subway);
  }

  fn refresh_service_alerts_realtime(&mut self) {
    let resp =
      match minreq::get("https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/camsys%2Fsubway-alerts.json").send() {
        Ok(a) => a,
        Err(_) => {
          self.service_alerts_realtime_feed = None;
          return;
        }
      };
    let bytes = resp.as_bytes();
    let service_alerts: MercuryDelays = match serde_json::from_slice(bytes) {
      Ok(r) => r,
      Err(_) => Default::default(),
    };

    self.service_alerts_realtime_feed = Some(service_alerts);
  }

  pub fn refresh_static(&mut self) {
    self.bus_static_feed.clear();

    // Bus
    let feed_uris = [
      "http://web.mta.info/developers/data/nyct/bus/google_transit_bronx.zip",
      "http://web.mta.info/developers/data/nyct/bus/google_transit_brooklyn.zip",
      "http://web.mta.info/developers/data/nyct/bus/google_transit_manhattan.zip",
      "http://web.mta.info/developers/data/nyct/bus/google_transit_queens.zip",
      "http://web.mta.info/developers/data/nyct/bus/google_transit_staten_island.zip",
      "http://web.mta.info/developers/data/busco/google_transit.zip",
    ];
    for uri in feed_uris {
      let resp = match minreq::get(uri).send() {
        Ok(a) => a,
        Err(_) => continue,
      };
      let bytes = resp.as_bytes();
      let gtfs = match Gtfs::from_reader(Cursor::new(bytes)) {
        Ok(a) => a,
        Err(_) => continue,
      };
      self.bus_static_feed.push(gtfs);
    }

    // Subway
    let resp = match minreq::get("http://web.mta.info/developers/data/nyct/subway/google_transit.zip").send() {
      Ok(a) => a,
      Err(_) => {
        return;
      } // Relying on no data will probably bork transit board
    };
    let bytes = resp.as_bytes();
    let gtfs = match Gtfs::from_reader(Cursor::new(bytes)) {
      Ok(a) => a,
      Err(_) => return,
    };
    self.subway_static_feed = gtfs;
  }
}
