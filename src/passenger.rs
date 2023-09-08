#[derive(Default)]
pub struct Passenger {
    pub name: String,
    pub ffid: String,
    pub seat: Seat,
}

#[derive(Default)]
pub struct Seat {
    pub column: char,
    pub row: char,
}
