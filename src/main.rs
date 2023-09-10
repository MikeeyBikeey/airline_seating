pub mod flight;
pub use flight::{Flight, GetFlight as _};
pub mod passenger;
pub use passenger::Passenger;
pub mod airline_seating_view;
pub use airline_seating_view::airline_seating_view;
pub mod passengers_view;
pub use passengers_view::passengers_view;
pub mod costs_view;
pub use costs_view::costs_view;
pub mod map_view;
pub use map_view::map_view;

fn main() -> Result<(), std::io::Error> {
    let mut app = cursive::default();

    let flight_info = Flight::default();
    app.add_layer(airline_seating_view(&flight_info));
    app.set_user_data(flight_info);

    app.try_run_with(backend_init)
}

/// Backend initialization of the `cursive` crate.
/// This particular backend helps to reduce jittering.
fn backend_init() -> std::io::Result<Box<dyn cursive::backend::Backend>> {
    let backend = cursive::backends::crossterm::Backend::init()?;
    let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
    Ok(Box::new(buffered_backend))
}
