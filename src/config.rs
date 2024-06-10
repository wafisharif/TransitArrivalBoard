use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
  bus_stop_handler::BusStopHandler, feed_handler::FeedHandler, service_alert_handler::ServiceAlertHandler,
  subway_stop_handler::SubwayStopHandler,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[ts(export, rename = "Export")]
pub struct Config {
  subway: Vec<Vec<String>>,
  bus: Vec<Vec<String>>,
}

impl Config {
  pub fn get_subway_handlers(&self, feed_data: Arc<RwLock<FeedHandler>>) -> Vec<SubwayStopHandler> {
    self
      .subway
      .iter()
      .map(|a| SubwayStopHandler::new(feed_data.to_owned(), a.to_owned()))
      .collect()
  }

  pub fn get_bus_handlers(&self, feed_data: Arc<RwLock<FeedHandler>>, api_key: Arc<String>) -> Vec<BusStopHandler> {
    self
      .bus
      .iter()
      .map(|a| BusStopHandler::new(feed_data.to_owned(), api_key.to_owned(), a.to_owned()))
      .collect()
  }

  pub fn get_service_alerts_handler(&self, feed_data: Arc<RwLock<FeedHandler>>) -> ServiceAlertHandler {
    ServiceAlertHandler::new(feed_data)
  }
}
