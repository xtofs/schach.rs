use druid::widget::{Align, Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};
use schach::{Board, BoardWidget};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
// const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<Board> = LocalizedString::new("Schach!");

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1000.0, 1200.0))
        .with_min_size((300.0, 400.0))
        .set_window_state(druid::WindowState::RESTORED);

    // create the initial app state
    let initial_state = Board::default();

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<Board> {
    // // a label that will determine its text based on the current app data.
    // let label = Label::new(|data: &Board, _env: &Env| format!("Hello {}!", data.name));
    // // a textbox that modifies `name`.
    // let textbox = TextBox::new()
    //     .with_placeholder("Who are we greeting?")
    //     .fix_width(TEXT_BOX_WIDTH)
    //     .lens(Board::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(Label::new("Schach"))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        // .with_child(textbox)
        // .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(BoardWidget::new().padding(10.0));

    // center the two widgets in the available space
    Align::centered(layout)
}
