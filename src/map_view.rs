use crate::{flight::GetFlight as _, Passenger};
use cursive::{
    traits::*,
    view::IntoBoxedView,
    views::{PaddedView, Panel, TextView},
    Cursive,
};

pub const COLUMNS: [char; 4] = ['A', 'B', 'C', 'D'];
pub const ROWS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn map_view(passengers: &[Passenger]) -> Box<dyn View> {
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

pub fn update_map(app: &mut Cursive) {
    // `passengers` is temporarily taken to avoid borrow issues
    let passengers = std::mem::take(&mut app.flight().passengers);
    app.call_on_name("map", |map: &mut TextView| {
        map.set_content(create_map_display(&passengers))
    });
    app.flight().passengers = passengers;
}

fn is_seat_taken(passengers: &[Passenger], column: char, row: char) -> bool {
    for passenger in passengers {
        if passenger.seat.column == column && passenger.seat.row == row {
            return true;
        }
    }
    false
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
