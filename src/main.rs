use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 10.0;
const TEXT_BOX_WIDTH: f64 = 300.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
    .title(WINDOW_TITLE)
    .window_size((600.0, 600.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "...".into(),
    };

    //start the application
    AppLauncher::with_window(main_window)
    .launch(initial_state)
    .expect("Failed, loser");

}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbook that modifies 'name'
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}
