use crate::Passenger;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Flight {
    pub bag_cost: i32,
    pub bag_count: i32,
    pub ticket_cost: i32,
    pub passengers: Vec<Passenger>,
}

impl Flight {
    pub fn total_cost(&self) -> i32 {
        self.bag_cost * self.bag_count + self.ticket_cost * self.passengers.len() as i32
    }
}

/// Helper trait to simplify repeated calls to `Cursive::user_data`.
pub trait GetFlight {
    fn flight(&mut self) -> &mut Flight;
}

impl GetFlight for cursive::Cursive {
    fn flight(&mut self) -> &mut Flight {
        self.user_data().unwrap()
    }
}
