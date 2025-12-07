use std::{cell::RefCell, collections::VecDeque, ops::DerefMut, sync::Arc};

use crate::{
    Events::Event,
    Stations::Station::{IStation, Station},
};

pub struct Engine<'engine> {
    queue: VecDeque<Event>,
    stations: Vec<&'engine mut dyn IStation>,
}

impl<'engine> Engine<'engine> {
    pub fn new() -> Self {
        Engine {
            queue: VecDeque::new(),
            stations: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, event: Event) {
        let mut iter = (&mut self.queue).into_iter();
        if let Some(i) = iter.position(|evt| evt.occurTime > event.occurTime) {
            self.queue.insert(i, event.clone());
        }
        self.queue.push_back(event);
    }

    pub fn tick(&mut self) {
        if !self.queue.is_empty() {
            let evt = self.queue.pop_front().unwrap();
            let dest = &evt.destination;

            for station in self.stations.iter_mut() {
                if station.name() == dest {
                    let res = station.handle(&evt);
                    match res {
                        crate::Stations::Station::ComputeResult::Ok => (),
                        crate::Stations::Station::ComputeResult::Event(event) => {
                            self.enqueue(event);
                        }
                        crate::Stations::Station::ComputeResult::Error(_) => {
                            panic!("Error processing event at station {}", dest);
                        }
                    }
                    return;
                }
            }
            panic!("Not found event with destination {}", dest);
        }
    }

    pub fn events(&self) -> &VecDeque<Event> {
        &self.queue
    }

    pub fn stations(&self) -> &Vec<&'engine mut dyn IStation> {
        &self.stations
    }

    pub fn register_station(&mut self, station: &'engine mut dyn IStation) {
        self.stations.push(station);
    }

    pub fn has_events(&self) -> bool {
        !self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Events::DefaultType,
        Stations::{Station::ComputeResult, StationData::StationData},
    };

    use super::*;

    struct MockStation {}
    impl IStation for MockStation {
        fn handle(&mut self, _event: &Event) -> ComputeResult {
            unimplemented!()
        }
        fn get_data(&self) -> &StationData {
            unimplemented!()
        }
        fn name(&self) -> &String {
            unimplemented!()
        }
    }

    #[test]
    fn test_enqueue() {
        let mut engine = Engine::new();
        let event = Event::new(
            DefaultType::ARRIVAL.into(),
            0.0,
            0.0,
            0.0,
            0.0,
            "none".to_string(),
        );
        engine.enqueue(event);
    }

    #[test]
    fn test_add_station() {
        let mut engine = Engine::new();
        let mut station = MockStation {};
        engine.register_station(&mut station);
        assert_eq!(engine.stations().len(), 1);
    }
    #[test]
    fn test_ptr_cast() {
        let event = &mut Event::new(
            DefaultType::ARRIVAL.into(),
            0.0,
            0.0,
            0.0,
            0.0,
            "none".to_string(),
        ) as *mut Event;

        let mut casted = event as *mut i32;
        unsafe { println!("{:?}", *casted) };
    }
}
