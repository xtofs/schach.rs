use druid::widget::{Align, Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};
use schach::{Board, BoardWidget};

fn main() {
    stderrlog::new().module(module_path!()).verbosity(4).init().expect("log setup failed");

    // describe the main window
    const WINDOW_TITLE: LocalizedString<Board> = LocalizedString::new("Schach!");
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((800.0, 1000.0))
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

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

fn build_root_widget() -> impl Widget<Board> {
    let label = Label::new(|board: &Board, _env: &druid::Env| format!("{:?}'s move ({})", board.active, board.fullmove_number + 1));

    let castling = Label::new(|board: &Board, _env: &druid::Env| format!("castling: {}", board.castling));
    let en_passant = Label::new(|board: &Board, _env: &druid::Env| format!("en_passant: {:?}", board.en_passant));
    let captures = Label::new(|board: &Board, _env: &druid::Env| format!("captures: {}", board.captures));

    let reset = Button::new("reset").on_click(|_ctx, data: &mut Board, _env| *data = Board::default());

    let board = BoardWidget::new();

    // arrange the two widgets vertically, with some padding
    let rhs = Flex::column()
        .with_child(castling)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(en_passant)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(captures)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(reset)
        .must_fill_main_axis(true);

    let lhs = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(board.padding(20.0))
        .must_fill_main_axis(true);

    let layout = Flex::row().with_child(lhs).with_spacer(VERTICAL_WIDGET_SPACING).with_child(rhs).center();

    layout
}
