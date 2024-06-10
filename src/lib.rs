use std::collections::{BTreeMap, HashMap};
use ts_rs::TS;

use serde::{Deserialize, Serialize};

pub mod bus_stop_handler;
pub mod config;
pub mod feed_handler;
pub mod mercury_structs;
pub mod service_alert_handler;
pub mod siri_structs;
pub mod subway_stop_handler;
pub mod gtfsrt {
  include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct Vehicle {
  pub route_id: String,
  pub route_name: String,
  pub destination_id: String,
  pub destination_name: String,
  pub minutes_until_arrival: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct Stop {
  pub name: String,
  pub trips: Vec<Vehicle>,
  pub destinations: BTreeMap<String, BTreeMap<String, Vec<Vehicle>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct Alert {
  pub route_id: String,
  pub sort_order: i32,
  pub header_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct Route {
  pub route_id: String,
  pub route_name: String,
  pub route_color: String,
  pub route_text_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[ts(export, rename = "Import")]
pub struct Export {
  pub stops_realtime: BTreeMap<String, Stop>,
  pub service_alerts_realtime: Vec<Alert>,
  pub routes_static: HashMap<String, Route>,
}
