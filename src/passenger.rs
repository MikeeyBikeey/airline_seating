use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Passenger {
    pub name: String,
    pub ffid: String,
    pub seat: Seat,
}

#[derive(Default, Serialize)]
pub struct Seat {
    pub column: char,
    pub row: char,
}
