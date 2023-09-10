use crate::{flight::GetFlight as _, Flight};
use cursive::{
    traits::*,
    view::IntoBoxedView,
    views::{EditView, LinearLayout, ListView, PaddedView, Panel, TextView},
    Cursive,
};

pub fn costs_view(flight: &Flight) -> Box<dyn View> {
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

pub fn update_total_cost(app: &mut Cursive) {
    let total_cost = app.flight().total_cost();
    app.call_on_name("total_cost", |text_view: &mut TextView| {
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

fn on_edit_ticket_cost(app: &mut Cursive, _text: &str, _size: usize) {
    if let Some(ticket_cost) = app.call_on_name("ticket_cost", assure_edit_view_is_integer) {
        let flight = app.flight();
        flight.ticket_cost = ticket_cost;
        update_total_cost(app);
    }
}

fn on_edit_bag_cost(app: &mut Cursive, _text: &str, _size: usize) {
    if let Some(bag_cost) = app.call_on_name("bag_cost", assure_edit_view_is_integer) {
        let flight = app.flight();
        flight.bag_cost = bag_cost;
        update_total_cost(app);
    }
}

fn on_edit_bag_count(app: &mut Cursive, _text: &str, _size: usize) {
    if let Some(bag_count) = app.call_on_name("bag_count", assure_edit_view_is_integer) {
        let flight = app.flight();
        flight.bag_count = bag_count;
        update_total_cost(app);
    }
}
