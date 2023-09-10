use crate::{costs_view, flight::GetFlight as _, map_view, Passenger};
use cursive::{
    traits::*,
    view::IntoBoxedView,
    views::{Button, EditView, LinearLayout, Panel, SelectView, TextView},
    Cursive,
};

pub fn passengers_view(passengers: &[Passenger]) -> Box<dyn View> {
    Panel::new(
        LinearLayout::vertical()
            .child(TextView::new(
                "Name                 FFID    Seat             ",
            ))
            .child(
                LinearLayout::vertical()
                    .with(|layout| {
                        for passenger in passengers {
                            layout.add_child(single_passenger_view(passenger));
                        }
                    })
                    .with_name("passengers")
                    .scrollable(),
            )
            .child(Button::new("Board Passenger", on_board_passenger)),
    )
    .title("Passengers")
    .into_boxed_view()
}

fn on_board_passenger(app: &mut Cursive) {
    let passenger = Passenger::default();
    app.call_on_name("passengers", |passengers: &mut LinearLayout| {
        passengers.add_child(single_passenger_view(&passenger));
    });
    let flight = app.flight();
    flight.passengers.push(passenger);
    costs_view::update_total_cost(app);
}

fn focused_passenger_index(app: &mut Cursive) -> Option<usize> {
    app.call_on_name("passengers", |passengers: &mut LinearLayout| {
        passengers.get_focus_index() - 1 // `- 1` because the first child isn't a passenger
    })
}

fn on_submit_passenger_seat_row(app: &mut Cursive, row: &str) {
    if let Some(passenger_index) = focused_passenger_index(app) {
        let passenger = &mut app.flight().passengers[passenger_index];
        passenger.seat.row = row.chars().next().unwrap();
    }
    map_view::update_map(app);
}

fn on_submit_passenger_seat_column(app: &mut Cursive, column: &str) {
    if let Some(passenger_index) = focused_passenger_index(app) {
        let passenger = &mut app.flight().passengers[passenger_index];
        passenger.seat.column = column.chars().next().unwrap();
    }
    map_view::update_map(app);
}

fn on_edit_passenger_ffid(app: &mut Cursive, ffid: &str, _size: usize) {
    if let Some(passenger_index) = focused_passenger_index(app) {
        let flight = app.flight();
        flight.passengers[passenger_index].ffid = ffid.to_string();
    }
}

fn on_edit_passenger_name(app: &mut Cursive, name: &str, _size: usize) {
    if let Some(passenger_index) = focused_passenger_index(app) {
        let flight = app.flight();
        flight.passengers[passenger_index].name = name.to_string();
    }
}

fn single_passenger_view(passenger: &Passenger) -> Box<dyn View> {
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
                    for row in map_view::ROWS {
                        view.add_item_str(row.to_string());
                    }
                    // Selects the correct row for the passenger
                    view.set_selection(
                        map_view::ROWS
                            .iter()
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
                    for column in map_view::COLUMNS {
                        view.add_item_str(column.to_string());
                    }
                    // Selects the correct column for the passenger
                    view.set_selection(
                        map_view::COLUMNS
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

fn on_unboard_passenger(app: &mut Cursive) {
    let passenger_index = app.call_on_name("passengers", |passengers: &mut LinearLayout| {
        let passenger_index = passengers.get_focus_index();
        passengers.remove_child(passenger_index);
        passenger_index
    });
    if let Some(passenger_index) = passenger_index {
        let flight = app.flight();
        flight.passengers.remove(passenger_index - 1); // `- 1` because the first child isn't a passenger
    }
    costs_view::update_total_cost(app);
    map_view::update_map(app);
}
