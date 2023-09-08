use cursive::{
    traits::*,
    view::IntoBoxedView,
    views::{
        Button, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView, Panel, SelectView,
        TextView,
    },
    Cursive,
};

pub mod flight_info;
pub use flight_info::{FlightInfo, GetFlightInfo as _};

fn update_total_cost(cursive: &mut Cursive) {
    let total_cost = cursive.flight_info().total_cost();
    cursive.call_on_name("total_cost", |text_view: &mut TextView| {
        text_view.set_content(format!("Total Cost: ${}", total_cost));
    });
}

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

// VIEWS

fn costs_view() -> Box<dyn View> {
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
                            .max_content_width(4)
                            .on_edit(on_edit_ticket_cost)
                            .with_name("ticket_cost")
                            .fixed_width(5),
                    )
                    .delimiter()
                    .child(
                        "Bag Cost:      $",
                        EditView::new()
                            .max_content_width(4)
                            .on_edit(on_edit_bag_cost)
                            .with_name("bag_cost")
                            .fixed_width(5),
                    )
                    .delimiter()
                    .child(
                        "Bag Count:",
                        EditView::new()
                            .max_content_width(4)
                            .on_edit(on_edit_bag_count)
                            .with_name("bag_count")
                            .fixed_width(5),
                    )
                    .delimiter(),
            )
            .child(TextView::new("Total Cost: $0").with_name("total_cost")),
    ))
    .title("Costs")
    .into_boxed_view()
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
        )),
    ))
    .title("Map")
    .into_boxed_view()
}

fn passengers_view() -> Box<dyn View> {
    Panel::new(
        LinearLayout::vertical()
            .child(TextView::new("Name                 FFID    Seat"))
            .child(
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
                            .item_str("1")
                            .item_str("2")
                            .item_str("3")
                            .item_str("4")
                            .item_str("5")
                            .item_str("6")
                            .item_str("7")
                            .item_str("8")
                            .item_str("9")
                            .with_name("passenger_seat_number"),
                    )
                    .child(TextView::new(" "))
                    .child(
                        SelectView::new()
                            .popup()
                            .item_str("A")
                            .item_str("B")
                            .item_str("C")
                            .item_str("D")
                            .with_name("passenger_seat_letter"),
                    )
                    .child(TextView::new(" "))
                    .child(Button::new("Unboard", |_| ()).with_name("passenger_remove_button")),
            )
            .scrollable(),
    )
    .title("Passengers")
    .into_boxed_view()
}

fn main() -> Result<(), std::io::Error> {
    let mut app = Cursive::default();

    app.set_user_data(FlightInfo::default());

    app.add_layer(
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
                    .child(passengers_view())
                    .child(Button::new("Board Passenger", |_| ()))
                    .child(DummyView)
                    .child(TextView::new("©1960s Fresh Airlines").center()),
            ),
    );

    let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
        Ok(Box::new(buffered_backend))
    };

    app.try_run_with(backend_init)
}
