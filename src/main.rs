use cursive::{
    traits::*,
    view::IntoBoxedView,
    views::{
        Button, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView, Panel, SelectView,
        TextView,
    },
    Cursive,
};

pub mod flight;
pub use flight::{Flight, GetFlight as _};
pub mod passenger;
pub use passenger::Passenger;

const COLUMNS: [char; 4] = ['A', 'B', 'C', 'D'];
const ROWS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

// COSTS VIEW

fn update_total_cost(cursive: &mut Cursive) {
    let total_cost = cursive.flight().total_cost();
    cursive.call_on_name("total_cost", |text_view: &mut TextView| {
        text_view.set_content(format!("Total Cost: ${}", total_cost));
    });
}

/// Assures that the `EditView`'s contents represent an integer.
/// Returns the integer value of the contents.
fn assure_edit_view_is_integer(edit_view: &mut EditView) -> i32 {
    let text_numbers: String = edit_view
        .get_content()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();
    edit_view.set_content(&text_numbers);
    text_numbers.parse().unwrap_or(0)
}

fn on_edit_ticket_cost(cursive: &mut Cursive, _text: &str, _size: usize) {
    if let Some(ticket_cost) = cursive.call_on_name("ticket_cost", assure_edit_view_is_integer) {
        let flight = cursive.flight();
        flight.ticket_cost = ticket_cost;
        update_total_cost(cursive);
    }
}

fn on_edit_bag_cost(cursive: &mut Cursive, _text: &str, _size: usize) {
    if let Some(bag_cost) = cursive.call_on_name("bag_cost", assure_edit_view_is_integer) {
        let flight = cursive.flight();
        flight.bag_cost = bag_cost;
        update_total_cost(cursive);
    }
}

fn on_edit_bag_count(cursive: &mut Cursive, _text: &str, _size: usize) {
    if let Some(bag_count) = cursive.call_on_name("bag_count", assure_edit_view_is_integer) {
        let flight = cursive.flight();
        flight.bag_count = bag_count;
        update_total_cost(cursive);
    }
}

fn costs_view(flight: &Flight) -> Box<dyn View> {
    const DIGITS: usize = 4;
    Panel::new(PaddedView::lrtb(
        2,
        2,
        1,
        1,
        LinearLayout::vertical()
            .child(
                ListView::new()
                    .child(
                        "Ticket Cost:   $",
                        EditView::new()
                            .content(flight.ticket_cost.to_string())
                            .max_content_width(DIGITS)
                            .on_edit(on_edit_ticket_cost)
                            .with_name("ticket_cost")
                            .fixed_width(DIGITS + 1),
                    )
                    .delimiter()
                    .child(
                        "Bag Cost:      $",
                        EditView::new()
                            .content(flight.bag_cost.to_string())
                            .max_content_width(DIGITS)
                            .on_edit(on_edit_bag_cost)
                            .with_name("bag_cost")
                            .fixed_width(DIGITS + 1),
                    )
                    .delimiter()
                    .child(
                        "Bag Count:",
                        EditView::new()
                            .content(flight.bag_count.to_string())
                            .max_content_width(DIGITS)
                            .on_edit(on_edit_bag_count)
                            .with_name("bag_count")
                            .fixed_width(DIGITS + 1),
                    )
                    .delimiter(),
            )
            .child(
                TextView::new(format!("Total Cost: ${}", flight.total_cost()))
                    .with_name("total_cost"),
            ),
    ))
    .title("Costs")
    .into_boxed_view()
}

// MAP VIEW

fn is_seat_taken(passengers: &[Passenger], column: char, row: char) -> bool {
    for passenger in passengers {
        if passenger.seat.column == column && passenger.seat.row == row {
            return true;
        }
    }
    false
}

fn update_map(cursive: &mut Cursive) {
    // `passengers` is temporarily taken to avoid borrow issues
    let passengers = std::mem::take(&mut cursive.flight().passengers);
    cursive.call_on_name("map", |map: &mut TextView| {
        map.set_content(create_map_display(&passengers))
    });
    cursive.flight().passengers = passengers;
}

/// Returns a `String` for displaying on the map.
/// For updating which seats are taken on the displayed map.
/// This function is highly unoptimized, but it works!
fn create_map_display(passengers: &[Passenger]) -> String {
    // TODO: optimize `update_map` function
    let mut text = " ".to_string();
    for row in COLUMNS {
        text += &format!("  {}", row);
    }
    for row in ROWS {
        text += &format!("\n{}", row);
        for column in COLUMNS {
            text += "  ";
            if is_seat_taken(passengers, column, row) {
                text += "X";
            } else {
                text += "_";
            }
        }
    }
    text
}

fn map_view(passengers: &[Passenger]) -> Box<dyn View> {
    Panel::new(PaddedView::lrtb(
        2,
        2,
        1,
        1,
        TextView::new(create_map_display(passengers)).with_name("map"),
    ))
    .title("Map")
    .into_boxed_view()
}

// PASSENGERS VIEW

fn focused_passenger_index(cursive: &mut Cursive) -> Option<usize> {
    cursive.call_on_name("passengers", |passengers: &mut LinearLayout| {
        passengers.get_focus_index() - 1 // `- 1` because the first child isn't a passenger
    })
}

fn on_board_passenger(cursive: &mut Cursive) {
    let passenger = Passenger::default();
    cursive.call_on_name("passengers", |passengers: &mut LinearLayout| {
        passengers.add_child(passenger_view(&passenger));
    });
    let flight = cursive.flight();
    flight.passengers.push(passenger);
    update_total_cost(cursive);
}

fn on_unboard_passenger(cursive: &mut Cursive) {
    let passenger_index = cursive.call_on_name("passengers", |passengers: &mut LinearLayout| {
        let passenger_index = passengers.get_focus_index();
        passengers.remove_child(passenger_index);
        passenger_index
    });
    if let Some(passenger_index) = passenger_index {
        let flight = cursive.flight();
        flight.passengers.remove(passenger_index - 1); // `- 1` because the first child isn't a passenger
    }
    update_total_cost(cursive);
    update_map(cursive);
}

fn on_submit_passenger_seat_row(cursive: &mut Cursive, row: &str) {
    if let Some(passenger_index) = focused_passenger_index(cursive) {
        let passenger = &mut cursive.flight().passengers[passenger_index];
        passenger.seat.row = row.chars().next().unwrap();
    }
    update_map(cursive);
}

fn on_submit_passenger_seat_column(cursive: &mut Cursive, column: &str) {
    if let Some(passenger_index) = focused_passenger_index(cursive) {
        let passenger = &mut cursive.flight().passengers[passenger_index];
        passenger.seat.column = column.chars().next().unwrap();
    }
    update_map(cursive);
}

fn on_edit_passenger_ffid(cursive: &mut Cursive, ffid: &str, _size: usize) {
    if let Some(passenger_index) = focused_passenger_index(cursive) {
        let flight = cursive.flight();
        flight.passengers[passenger_index].ffid = ffid.to_string();
    }
}

fn on_edit_passenger_name(cursive: &mut Cursive, name: &str, _size: usize) {
    if let Some(passenger_index) = focused_passenger_index(cursive) {
        let flight = cursive.flight();
        flight.passengers[passenger_index].name = name.to_string();
    }
}

fn passenger_view(passenger: &Passenger) -> Box<dyn View> {
    LinearLayout::horizontal()
        .child(
            EditView::new()
                .on_edit(on_edit_passenger_name)
                .content(&passenger.name)
                // .with_name("passenger_name")
                .fixed_width(20),
        )
        .child(TextView::new(" "))
        .child(
            EditView::new()
                .on_edit(on_edit_passenger_ffid)
                .content(&passenger.ffid)
                .max_content_width(6)
                // .with_name("passenger_ffid")
                .fixed_width(7),
        )
        .child(TextView::new(" "))
        .child(
            SelectView::new()
                .popup()
                .item_str("*")
                .with(|view| {
                    for row in ROWS {
                        view.add_item_str(row.to_string());
                    }
                    // Selects the correct row for the passenger
                    view.set_selection(
                        ROWS.iter()
                            .position(|row| *row == passenger.seat.row)
                            .map(|idx| idx + 1) // `+ 1` to take into account the first item: `"*"`
                            .unwrap_or(0),
                    );
                })
                .on_submit(on_submit_passenger_seat_row)
                .with_name("passenger_seat_row"),
        )
        .child(TextView::new(" "))
        .child(
            SelectView::new()
                .popup()
                .item_str("*")
                .with(|view| {
                    for column in COLUMNS {
                        view.add_item_str(column.to_string());
                    }
                    // Selects the correct column for the passenger
                    view.set_selection(
                        COLUMNS
                            .iter()
                            .position(|column| *column == passenger.seat.column)
                            .map(|idx| idx + 1) // `+ 1` to take into account the first item: `"*"`
                            .unwrap_or(0),
                    );
                })
                .on_submit(on_submit_passenger_seat_column)
                .with_name("passenger_seat_column"),
        )
        .child(TextView::new(" "))
        .child(Button::new("Unboard", on_unboard_passenger).with_name("passenger_remove_button"))
        .into_boxed_view()
}

fn all_passengers_view(passengers: &[Passenger]) -> Box<dyn View> {
    Panel::new(
        LinearLayout::vertical()
            .child(TextView::new(
                "Name                 FFID    Seat             ",
            ))
            .with(|layout| {
                for passenger in passengers {
                    layout.add_child(passenger_view(passenger));
                }
            })
            .with_name("passengers")
            .scrollable(),
    )
    .title("Passengers")
    .into_boxed_view()
}

// AIRLINE SEATING VIEW

fn show_alert<T: Into<String>>(cursive: &mut Cursive, message: T) {
    cursive.add_layer(
        Dialog::new()
            .title("Alert")
            .button("Close", |c| {
                c.pop_layer();
            })
            .content(PaddedView::lrtb(
                1,
                1,
                1,
                0,
                TextView::new(message.into()).fixed_width(32),
            )),
    )
}

fn on_confirm_save(cursive: &mut Cursive) {
    // TODO: simplify function

    // Saves the flight info
    let flight = serde_json::to_string_pretty(cursive.flight()).unwrap();
    let save_result = cursive.call_on_name("save_file_path", |view: &mut EditView| {
        let mut result = String::default();
        let path = view.get_content();
        if !std::path::Path::new(&*path).exists() {
            if let Err(error) = std::fs::write(&*path, &flight) {
                result = error.to_string();
            }
        } else {
            result = format!("File already exists at path: \"{}\"", path);
        }
        result
    });

    // Reports errors and pops layer
    if save_result.as_deref() == Some("") {
        cursive.pop_layer();
    } else if let Some(save_result) = save_result {
        show_alert(cursive, format!("Unable to save file: {}", save_result));
    } else {
        show_alert(cursive, "An unknown error occurred while saving file.")
    }
}

fn save_view() -> Box<dyn View> {
    Dialog::new()
        .title("Save File Path")
        .button("Save", on_confirm_save)
        .button("Cancel", |c| {
            c.pop_layer();
        })
        .content(PaddedView::lrtb(
            1,
            1,
            1,
            0,
            EditView::new().with_name("save_file_path").fixed_width(32),
        ))
        .into_boxed_view()
}

fn on_confirm_load(cursive: &mut Cursive) {
    // // TODO: simplify function

    // Loads and parses the flight info
    let load_result = cursive.call_on_name(
        "load_file_path",
        |view: &mut EditView| -> Result<Flight, Box<dyn std::error::Error>> {
            let path = view.get_content();
            Ok(serde_json::from_str(&std::fs::read_to_string(&*path)?)?)
        },
    );

    // Reports errors and pops layer
    match load_result {
        Some(Ok(flight)) => {
            cursive.pop_layer(); // pops this message view
            cursive.pop_layer(); // (hopefully) pops airline seating view
            cursive.add_layer(airline_seating_view(&flight));
            cursive.set_user_data(flight);
        }
        Some(Err(error)) => {
            show_alert(
                cursive,
                format!("Unable to load file: {}", error.to_string()),
            );
        }
        None => show_alert(cursive, "An unknown error occurred while loading file."),
    }
}

fn load_view() -> Box<dyn View> {
    Dialog::new()
        .title("Load File Path")
        .button("Load", on_confirm_load)
        .button("Cancel", |c| {
            c.pop_layer();
        })
        .content(PaddedView::lrtb(
            1,
            1,
            1,
            0,
            EditView::new().with_name("load_file_path").fixed_width(32),
        ))
        .into_boxed_view()
}

fn airline_seating_view(flight: &Flight) -> Box<dyn View> {
    Dialog::new()
        .title("Advanced Airline Seating Systems®")
        .button("Load", |s| s.add_layer(load_view()))
        .button("Save", |s| s.add_layer(save_view()))
        .button("Submit", |s| s.quit())
        .content(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(map_view(&flight.passengers))
                        .child(costs_view(flight)),
                )
                .child(all_passengers_view(&flight.passengers))
                .child(Button::new("Board Passenger", on_board_passenger))
                .child(DummyView)
                .child(TextView::new("©1960s Fresh Airlines").center()),
        )
        .into_boxed_view()
}

fn main() -> Result<(), std::io::Error> {
    let mut app = Cursive::default();

    let flight = Flight::default();
    app.add_layer(airline_seating_view(&flight));
    app.set_user_data(flight);

    // This particular backend helps to reduce jittering
    let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
        Ok(Box::new(buffered_backend))
    };

    app.try_run_with(backend_init)
}
