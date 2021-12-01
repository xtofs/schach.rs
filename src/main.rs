use druid::widget::{Align, Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};
use schach::{Board, BoardWidget};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
// const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<Board> = LocalizedString::new("Schach!");

fn main() {
    stderrlog::new().module(module_path!()).verbosity(4).init().expect("log setup failed");

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1000.0, 1200.0))
        .with_min_size((300.0, 400.0))
        .set_window_state(druid::WindowState::RESTORED);

    // create the initial app state
    let initial_state = Board::default();
    // let initial_state = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq").unwrap();

    // start the application
    AppLauncher::with_window(main_window)
        // .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<Board> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &Board, _env: &druid::Env| format!("{:?}'s move ({})", data.active, data.fullmove_number + 1));

    let castling = Label::new(|board: &Board, _env: &druid::Env| format!("castling: {}", board.castling));
    let en_passant = Label::new(|board: &Board, _env: &druid::Env| format!("en_passant: {:?}", board.en_passant));

    // // a textbox that modifies `name`.
    // let textbox = TextBox::new()
    //     .with_placeholder("Who are we greeting?")
    //     .fix_width(TEXT_BOX_WIDTH)
    //     .lens(Board::name);
    let reset = Button::new("reset").on_click(|_ctx, data: &mut Board, _env| *data = Board::default());

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(Label::new("Schach"))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(castling)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(en_passant)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(reset)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        // .with_child(textbox)
        // .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(BoardWidget::new().padding(10.0));

    // center the two widgets in the available space
    Align::centered(layout)
}
