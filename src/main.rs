use cursive::{
    traits::*,
    utils::lines::simple::Row,
    view::IntoBoxedView,
    views::{
        Button, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView, Panel, SelectView,
        TextView,
    },
    Cursive,
};

pub mod flight_info;
pub use flight_info::{FlightInfo, GetFlightInfo as _};
pub mod passenger;
pub use passenger::Passenger;

const COLUMNS: [char; 4] = ['A', 'B', 'C', 'D'];
const ROWS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

// COSTS VIEW

fn update_total_cost(cursive: &mut Cursive) {
    let total_cost = cursive.flight_info().total_cost();
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
        let flight_info = cursive.flight_info();
        flight_info.ticket_cost = ticket_cost;
        update_total_cost(cursive);
    }
}

fn on_edit_bag_cost(cursive: &mut Cursive, _text: &str, _size: usize) {
    if let Some(bag_cost) = cursive.call_on_name("bag_cost", assure_edit_view_is_integer) {
        let flight_info = cursive.flight_info();
        flight_info.bag_cost = bag_cost;
        update_total_cost(cursive);
    }
}

fn on_edit_bag_count(cursive: &mut Cursive, _text: &str, _size: usize) {
    if let Some(bag_count) = cursive.call_on_name("bag_count", assure_edit_view_is_integer) {
        let flight_info = cursive.flight_info();
        flight_info.bag_count = bag_count;
        update_total_cost(cursive);
    }
}

fn costs_view() -> Box<dyn View> {
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
                            .max_content_width(DIGITS)
                            .on_edit(on_edit_ticket_cost)
                            .with_name("ticket_cost")
                            .fixed_width(DIGITS + 1),
                    )
                    .delimiter()
                    .child(
                        "Bag Cost:      $",
                        EditView::new()
                            .max_content_width(DIGITS)
                            .on_edit(on_edit_bag_cost)
                            .with_name("bag_cost")
                            .fixed_width(DIGITS + 1),
                    )
                    .delimiter()
                    .child(
                        "Bag Count:",
                        EditView::new()
                            .max_content_width(DIGITS)
                            .on_edit(on_edit_bag_count)
                            .with_name("bag_count")
                            .fixed_width(DIGITS + 1),
                    )
                    .delimiter(),
            )
            .child(TextView::new("Total Cost: $0").with_name("total_cost")),
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

/// Updates which seats are taken on the displayed map.
/// This function is highly unoptimized, but it works!
fn update_map(cursive: &mut Cursive) {
    let passengers = &cursive.flight_info().passengers;
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
    cursive.call_on_name("map", |map: &mut TextView| map.set_content(text));
}

fn map_view() -> Box<dyn View> {
    Panel::new(PaddedView::lrtb(
        2,
        2,
        1,
        1,
        TextView::new(concat!(
            "   A  B  C  D\n",
            "1  _  _  _  _\n",
            "2  _  _  _  _\n",
            "3  _  _  _  _\n",
            "4  _  _  _  _\n",
            "5  _  _  _  _\n",
            "6  _  _  _  _\n",
            "7  _  _  _  _\n",
            "8  _  _  _  _\n",
            "9  _  _  _  _",
        ))
        .with_name("map"),
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
    let flight_info = cursive.flight_info();
    flight_info.passengers.push(Passenger::default());
    cursive.call_on_name("passengers", |passengers: &mut LinearLayout| {
        passengers.add_child(passenger_view());
    });
    update_total_cost(cursive);
}

fn on_unboard_passenger(cursive: &mut Cursive) {
    let passenger_index = cursive.call_on_name("passengers", |passengers: &mut LinearLayout| {
        let passenger_index = passengers.get_focus_index();
        passengers.remove_child(passenger_index);
        passenger_index
    });
    if let Some(passenger_index) = passenger_index {
        let flight_info = cursive.flight_info();
        flight_info.passengers.remove(passenger_index - 1); // `- 1` because the first child isn't a passenger
    }
    update_total_cost(cursive);
    update_map(cursive);
}

fn on_submit_passenger_seat_row(cursive: &mut Cursive, row: &str) {
    if let Some(passenger_index) = focused_passenger_index(cursive) {
        let passenger = &mut cursive.flight_info().passengers[passenger_index];
        passenger.seat.row = row.chars().next().unwrap();
    }
    update_map(cursive);
}

fn on_submit_passenger_seat_column(cursive: &mut Cursive, column: &str) {
    if let Some(passenger_index) = focused_passenger_index(cursive) {
        let passenger = &mut cursive.flight_info().passengers[passenger_index];
        passenger.seat.column = column.chars().next().unwrap();
    }
    update_map(cursive);
}

fn passenger_view() -> Box<dyn View> {
    LinearLayout::horizontal()
        .child(EditView::new().with_name("passenger_name").fixed_width(20))
        .child(TextView::new(" "))
        .child(
            EditView::new()
                .max_content_width(6)
                .fixed_width(7)
                .with_name("passenger_ffid"),
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
                })
                .on_submit(on_submit_passenger_seat_column)
                .with_name("passenger_seat_column"),
        )
        .child(TextView::new(" "))
        .child(Button::new("Unboard", on_unboard_passenger).with_name("passenger_remove_button"))
        .into_boxed_view()
}

fn all_passengers_view() -> Box<dyn View> {
    Panel::new(
        LinearLayout::vertical()
            .child(TextView::new(
                "Name                 FFID    Seat             ",
            ))
            .with_name("passengers")
            .scrollable(),
    )
    .title("Passengers")
    .into_boxed_view()
}

// AIRLINE SEATING VIEW

fn airline_seating_view() -> Box<dyn View> {
    Dialog::new()
        .title("Advanced Airline Seating Systems®")
        .button("Ok", |s| s.quit())
        .content(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(map_view())
                        .child(costs_view()),
                )
                .child(all_passengers_view())
                .child(Button::new("Board Passenger", on_board_passenger))
                .child(DummyView)
                .child(TextView::new("©1960s Fresh Airlines").center()),
        )
        .into_boxed_view()
}

fn main() -> Result<(), std::io::Error> {
    let mut app = Cursive::default();

    app.set_user_data(FlightInfo::default());
    app.add_layer(airline_seating_view());

    // This particular backend helps to reduce jittering
    let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
        Ok(Box::new(buffered_backend))
    };

    app.try_run_with(backend_init)
}
