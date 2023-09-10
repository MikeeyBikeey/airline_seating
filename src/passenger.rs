use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Passenger {
    pub name: String,
    pub ffid: String,
    pub seat: Seat,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Seat {
    pub column: char,
    pub row: char,
}
