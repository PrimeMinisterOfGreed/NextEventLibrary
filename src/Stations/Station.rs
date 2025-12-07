use std::{cell::RefCell, sync::Arc};

use log::error;
use plotters::prelude::DynElement;

use crate::Events::{DefaultType, Event};

use super::StationData::StationData;

pub enum ComputeResult {
    Ok,
    Event(Event),
    Error(String),
}

pub trait IEventManager {
    fn process_event(&mut self, event: &Event, data: &mut StationData) -> ComputeResult;
}

pub trait IStation {
    fn handle(&mut self, event: &Event) -> ComputeResult;
    fn get_data(&self) -> &StationData;
    fn name(&self) -> &String;
}

pub struct Station<T: IEventManager> {
    name: String,
    data: StationData,
    policy: T,
}

impl<T: IEventManager> Station<T> {
    pub fn new(name: &str, policy: T) -> Self {
        Station {
            name: name.to_string(),
            data: StationData::new(),
            policy,
        }
    }
}

impl<T: IEventManager> IStation for Station<T> {
    fn handle(&mut self, event: &Event) -> ComputeResult {
        self.policy.process_event(event, &mut self.data)
    }

    fn get_data(&self) -> &StationData {
        &self.data
    }

    fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use once_cell::sync::Lazy;

    use crate::{Engines::Engine, Stations::FCFSRuler::FCFSPolicyManager};

    use super::*;
    struct MockStation {}
    impl IStation for MockStation {
        fn handle(&mut self, _event: &Event) -> ComputeResult {
            ComputeResult::Event(_event.clone())
        }
        fn get_data(&self) -> &StationData {
            static DATA: Lazy<StationData> = Lazy::new(|| StationData::new());
            &DATA
        }
        fn name(&self) -> &String {
            static NAME: Lazy<String> = Lazy::new(|| "mock".to_string());
            &NAME
        }
    }

    #[test]
    fn test_name_equality() {}

    #[test]
    fn test_forwarder() {
        let mut engine = Engine::new();
        let mut station = MockStation {};
        engine.register_station(&mut station);
        let mut event = Event::gen_arrival(0.1);
        event.destination = "mock".to_string();
        engine.enqueue(event.clone());
        engine.tick();
        println!("{:?}", engine.stations()[0].get_data());
        assert!(!engine.events().is_empty());
        assert!(*engine.events().front().unwrap() == event);
    }
}
