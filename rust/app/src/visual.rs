//! This program was automatically generated by Visual Embedded Rust. Don't edit here!
extern crate macros as mynewt_macros;   //  Declare the Mynewt Procedural Macros library
use mynewt_macros::infer_type;          //  Import Mynewt procedural macros
use mynewt::{
  result::*,              // Import Mynewt API Result and Error types
  sys::console,           // Import Mynewt Console API
};
use druid::{
  AppLauncher, Data, EventCtx, LocalizedString, Widget, WindowDesc,
  widget::{
      Align, Button, Column, Label, Padding,
  },
  argvalue::ArgValue,
  env::Env,
};

/// Application State
#[infer_type]  //  Infer the missing types
#[derive(Clone, Data, Default)]
struct State {
    count: _,
}

/// Will be run upon startup to launch the app
pub fn on_start() -> MynewtResult<()> {
    console::print("on_start\n");
    //  Build a new window
    let main_window = WindowDesc::new(ui_builder);
    //  Create application state
    let state = State::default();
    state.count = 0;

    //  Launch the window with the application state
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)
        .expect("launch failed");
    //  Return success to `main()` function
    Ok(())
}

/// Build the UI for the window
fn ui_builder() -> impl Widget {  //  `State` is the Application State
    console::print("Rust UI builder\n"); console::flush();
    //  Create a line of text
    let my_label_text = LocalizedString::new("hello-counter")
        .with_arg("count", on_my_label_show);  //  Call `on_my_label_show` to get label text
    //  Create a label widget my_label
    let my_label = Label::new(my_label_text);
    //  Create a button widget my_button
    let my_button = Button::new("increment", on_my_button_press);  //  Call `on_my_button_press` when pressed

    //  Create a column
    let mut col = Column::new();
    //  Add the label widget to the column, centered with padding
    col.add_child(
        Align::centered(
            Padding::new(5.0,
                my_label
            )
        ),
        1.0
    );
    //  Add the button widget to the column, with padding
    col.add_child(
        Padding::new(5.0,
            my_button
        ),
        1.0
    );
    //  Return the column containing the widgets
    col
}

///  Callback function that will be called to create the formatted text for the label my_label
#[infer_type]  //  Infer the missing types
fn on_my_label_show(state: _, env: _) -> MynewtResult {
    console::print("on_my_label_show\n");
    Ok(state.count)
}

///  Callback function that will be called when the button my_button is pressed
#[infer_type]  //  Infer the missing types
fn on_my_button_press(ctx: _, state: _, env: _) -> MynewtResult<()> {
    console::print("on_my_button_press\n");
    state.count = state.count + 1;
    Ok(())
}