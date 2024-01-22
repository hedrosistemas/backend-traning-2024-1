use log::debug;

use crate::services::messages::MQTTMessage;

pub trait BridgeService {
    fn exec(&self, msg: &MQTTMessage) -> Result<(), ()>;
}

pub struct BridgeServiceImpl {}

impl BridgeServiceImpl {
    pub fn new() -> Self {
        BridgeServiceImpl {}
    }
}

impl BridgeService for BridgeServiceImpl {
    fn exec(&self, _msg: &MQTTMessage) -> Result<(), ()> {
        debug!("BridgeService::exec() -> TODO!");
        Ok(())
    }
}
