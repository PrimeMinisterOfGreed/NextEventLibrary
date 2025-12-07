use std::{
    borrow::Borrow, cell::RefCell, collections::VecDeque, path::Component, rc::Rc, sync::Arc,
};

use crate::{
    Engines::Engine,
    Events::{DefaultType, Event},
    Stations::Station::ComputeResult,
};

use super::{Station::IEventManager, StationData::StationData};

pub struct FCFSPolicyManager {
    eventQueue: VecDeque<Event>,
    eventUnderProcess: Option<Event>,
}

impl IEventManager for FCFSPolicyManager {
    fn process_event(
        &mut self,
        event: &Event,
        data: &mut super::StationData::StationData,
    ) -> ComputeResult {
        data.update(event.occurTime);
        return match DefaultType::from(event.kind) {
            DefaultType::ARRIVAL => self.ProcessArrival(event, data),
            DefaultType::DEPARTURE => self.ProcessDeparture(event, data),
            _ => ComputeResult::Error("you should not be here".to_string()),
        };
    }
}

impl FCFSPolicyManager {
    pub fn new() -> Self {
        FCFSPolicyManager {
            eventQueue: VecDeque::new(),
            eventUnderProcess: None,
        }
    }

    pub fn ProcessArrival(&mut self, evt: &Event, data: &mut StationData) -> ComputeResult {
        let mut event = evt.clone();
        if evt.subType != DefaultType::INPROCESS {
            data.client_arrived(evt.arrivalTime);
            event.subType = DefaultType::INPROCESS.into();
        }
        if self.eventUnderProcess.is_none() {
            let mut newevt = event;
            let clock = data.clock;
            newevt.arrivalTime = clock;
            newevt.occurTime = clock + newevt.serviceTime;
            newevt.createTime = clock;
            newevt.kind = DefaultType::DEPARTURE.into();
            self.eventUnderProcess = Some(newevt.clone());
            ComputeResult::Event(newevt)
        } else {
            self.eventQueue.push_back(event);
            ComputeResult::Ok
        }
    }

    pub fn ProcessDeparture(&mut self, evt: &Event, data: &mut StationData) -> ComputeResult {
        debug_assert!(
            !(self.eventUnderProcess.is_none()
                || *self.eventUnderProcess.as_ref().unwrap() != *evt
                || evt.subType != DefaultType::INPROCESS),
            "Event departure requested not in process"
        );
        self.eventUnderProcess = None;
        data.client_departure();
        if data.sysClients > 1 {
            debug_assert!(
                !self.eventQueue.is_empty(),
                "Event queue should not be empty while more than 1 client is in the system"
            );
            let mut new_evt = self.eventQueue.pop_front().unwrap();
            let clock = data.clock;
            new_evt.arrivalTime = clock;
            new_evt.createTime = clock;
            new_evt.occurTime = clock + new_evt.serviceTime;
            new_evt.kind = DefaultType::DEPARTURE.into();
            self.eventUnderProcess = Some(new_evt.clone());
            ComputeResult::Event(new_evt)
        } else {
            self.eventUnderProcess = None;
            ComputeResult::Ok
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Random::{rngs::RandomGenerator, rvgs::Exponential},
        Stations::Station::Station,
    };

    use super::*;

    #[test]
    fn test_station_arrival() {}
}
