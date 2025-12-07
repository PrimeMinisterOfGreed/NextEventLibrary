use core::fmt;

use NESLib_macros::LwItem;

use crate::Collections::LightweightList::ILwItem;
use crate::Collections::LightweightList::LwHeader;
use crate::Random::rvgs::Exponential;

#[derive(Clone, Copy, PartialEq)]
pub enum DefaultType {
    ARRIVAL,
    DEPARTURE,
    INPROCESS,
    END,
    PROBE,
    MAINTENANCE,
    NOEVENT,
}

impl DefaultType {
    pub fn to_char(&self) -> char {
        match self {
            DefaultType::ARRIVAL => 'A',
            DefaultType::DEPARTURE => 'D',
            DefaultType::INPROCESS => 'I',
            DefaultType::END => 'E',
            DefaultType::PROBE => 'P',
            DefaultType::MAINTENANCE => 'M',
            DefaultType::NOEVENT => 'N',
        }
    }
}

impl PartialEq<char> for DefaultType {
    fn eq(&self, other: &char) -> bool {
        *other == self.to_char()
    }
}

impl PartialEq<DefaultType> for char {
    fn eq(&self, other: &DefaultType) -> bool {
        *self == other.to_char()
    }
}

impl Into<char> for DefaultType {
    fn into(self) -> char {
        self.to_char()
    }
}

impl From<char> for DefaultType {
    fn from(value: char) -> Self {
        match value {
            'A' => DefaultType::ARRIVAL,
            'D' => DefaultType::DEPARTURE,
            'I' => DefaultType::INPROCESS,
            'E' => DefaultType::END,
            'P' => DefaultType::PROBE,
            'M' => DefaultType::MAINTENANCE,
            'N' => DefaultType::NOEVENT,
            _ => DefaultType::NOEVENT,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Event {
    pub kind: char,
    pub createTime: f64,
    pub occurTime: f64,
    pub serviceTime: f64,
    pub arrivalTime: f64,
    pub subType: char,
    pub destination: String,
}

impl Event {
    pub fn new(
        kind: char,
        createTime: f64,
        occurTime: f64,
        serviceTime: f64,
        arrivalTime: f64,
        destination: String,
    ) -> Self {
        Event {
            kind,
            createTime,
            occurTime,
            serviceTime,
            arrivalTime,
            subType: 'N',
            destination,
        }
    }

    pub fn gen_arrival(clock: f64) -> Self {
        Event {
            kind: DefaultType::ARRIVAL.to_char(),
            createTime: clock,
            occurTime: clock + Exponential(3.0),
            serviceTime: Exponential(5.0),
            arrivalTime: clock + Exponential(1.0),
            subType: DefaultType::ARRIVAL.to_char(),
            destination: "None".to_string(),
        }
    }
    pub fn gen_departure(clock: f64) -> Self {
        Event {
            kind: DefaultType::DEPARTURE.to_char(),
            createTime: clock,
            occurTime: clock + Exponential(3.0),
            serviceTime: 0.0,
            arrivalTime: clock - Exponential(3.0),
            subType: DefaultType::NOEVENT.to_char(),
            destination: "None".to_string(),
        }
    }
}

impl fmt::Display for DefaultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: &str = "";
        match self {
            DefaultType::ARRIVAL => result = "Arrival",
            DefaultType::DEPARTURE => result = "Departure",
            DefaultType::END => result = "End",
            DefaultType::PROBE => result = "Probe",
            DefaultType::MAINTENANCE => result = "Maintenance",
            DefaultType::NOEVENT => result = "NoEvent",
            DefaultType::INPROCESS => result = "InProcess",
        }
        f.write_str(result)
    }
}
