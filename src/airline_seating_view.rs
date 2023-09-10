use crate::{costs_view, flight::GetFlight as _, map_view, passengers_view, Flight};
use cursive::{
    traits::*,
    view::IntoBoxedView,
    views::{Dialog, DummyView, EditView, LinearLayout, PaddedView, TextView},
    Cursive,
};

pub fn airline_seating_view(flight_info: &Flight) -> Box<dyn View> {
    Dialog::new()
        .title("Advanced Airline Seating Systems®")
        .button("Load", |s| s.add_layer(load_view()))
        .button("Save", |s| s.add_layer(save_view()))
        .button("Submit", |s| s.quit())
        .content(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(map_view(&flight_info.passengers))
                        .child(costs_view(flight_info)),
                )
                .child(passengers_view(&flight_info.passengers))
                .child(DummyView)
                .child(TextView::new("©1960s Fresh Airlines").center()),
        )
        .into_boxed_view()
}

fn show_alert<T: Into<String>>(app: &mut Cursive, message: T) {
    app.add_layer(
        Dialog::new()
            .title("Alert")
            .button("Close", |app| {
                app.pop_layer();
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

fn on_confirm_save(app: &mut Cursive) {
    // TODO: simplify function

    // Saves the flight info
    let flight_info = serde_json::to_string_pretty(app.flight()).unwrap();
    let save_result = app.call_on_name("save_file_path", |view: &mut EditView| {
        let mut result = String::default();
        let path = view.get_content();
        if !std::path::Path::new(&*path).exists() {
            if let Err(error) = std::fs::write(&*path, &flight_info) {
                result = error.to_string();
            }
        } else {
            result = format!("File already exists at path: \"{}\"", path);
        }
        result
    });

    // Reports errors and pops layer
    if save_result.as_deref() == Some("") {
        app.pop_layer();
    } else if let Some(save_result) = save_result {
        show_alert(app, format!("Unable to save file: {}", save_result));
    } else {
        show_alert(app, "An unknown error occurred while saving file.")
    }
}

fn save_view() -> Box<dyn View> {
    Dialog::new()
        .title("Save File Path")
        .button("Save", on_confirm_save)
        .button("Cancel", |app| {
            app.pop_layer();
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

fn on_confirm_load(app: &mut Cursive) {
    // // TODO: simplify function

    // Loads and parses the flight info
    let load_result = app.call_on_name(
        "load_file_path",
        |view: &mut EditView| -> Result<Flight, Box<dyn std::error::Error>> {
            let path = view.get_content();
            Ok(serde_json::from_str(&std::fs::read_to_string(&*path)?)?)
        },
    );

    // Reports errors and pops layer
    match load_result {
        Some(Ok(flight_info)) => {
            app.pop_layer(); // pops this message view
            app.pop_layer(); // (hopefully) pops airline seating view
            app.add_layer(airline_seating_view(&flight_info));
            app.set_user_data(flight_info);
        }
        Some(Err(error)) => {
            show_alert(app, format!("Unable to load file: {}", error.to_string()));
        }
        None => show_alert(app, "An unknown error occurred while loading file."),
    }
}

fn load_view() -> Box<dyn View> {
    Dialog::new()
        .title("Load File Path")
        .button("Load", on_confirm_load)
        .button("Cancel", |app| {
            app.pop_layer();
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
