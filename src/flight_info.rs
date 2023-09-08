use crate::Passenger;

#[derive(Default)]
pub struct FlightInfo {
    pub bag_cost: i32,
    pub bag_count: i32,
    pub ticket_cost: i32,
    pub passengers: Vec<Passenger>,
}

impl FlightInfo {
    pub fn total_cost(&self) -> i32 {
        self.bag_cost * self.bag_count + self.ticket_cost * self.passengers.len() as i32
    }
}

/// Helper trait to simplify repeated calls to `Cursive::user_data`.
pub trait GetFlightInfo {
    fn flight_info(&mut self) -> &mut FlightInfo;
}

impl GetFlightInfo for cursive::Cursive {
    fn flight_info(&mut self) -> &mut FlightInfo {
        self.user_data().unwrap()
    }
}
