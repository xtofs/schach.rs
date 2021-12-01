use druid::piet::d2d::Bitmap;
use druid::piet::{ImageFormat, InterpolationMode};
use druid::{Env, Rect, RenderContext, Size, Widget};

use crate::grid::Grid;
use crate::{Board, Color, Kind, Move, MoveKind, Piece, Square};

pub struct BoardWidget {
    selected: Option<(Square, Vec<Move>)>,
    size: Size,
}

impl BoardWidget {
    pub fn new() -> Self {
        Self {
            selected: None,
            size: Size::default(),
        }
    }

    fn find_selected_move(&self, square: Square) -> Option<&Move> {
        if let Some((_, moves)) = &self.selected {
            if let Some(mv) = moves.iter().find(|mv| mv.target == square) {
                return Some(mv);
            }
        }
        None
    }

    pub fn move_color(mv: &Move) -> &druid::Color {
        match mv.kind {
            MoveKind::Move() => theme::MOVE,
            MoveKind::Take(_) => theme::TAKE,
            MoveKind::EnPassant() => theme::EN_PASSANT,
            MoveKind::Castle(_, _) => theme::CASTLE,
        }
    }
}

impl Widget<Board> for BoardWidget {
    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &Board, _env: &Env) {
        let grid = Grid::new(ctx.size(), 8, 8);

        for i in 0..8 {
            for j in 0..8 {
                ctx.fill(grid.square(i, j), if (i + j) % 2 == 0 { theme::LIGHT } else { theme::DARK });
            }
        }
        for j in 0..=8 {
            ctx.stroke(grid.horz_line(j), theme::LINE, 1.0);
        }
        for i in 0..=8 {
            ctx.stroke(grid.vert_line(i), theme::LINE, 1.0);
        }

        if ctx.has_focus() {
            if let Some((sq, moves)) = &self.selected {
                let rect = grid.rect(*sq).inflate(-5.0, -5.0);
                ctx.stroke(rect, theme::SELECTED, 5.0);

                for mv in moves {
                    let c = Self::move_color(mv);
                    let rect = grid.rect(mv.target).inflate(-5.0, -5.0);
                    ctx.stroke(rect, c, 5.0);
                }
            }
        }

        let mut sprites = Sprites::new(ctx);

        for j in 0..8 {
            for i in 0..8 {
                let rect = grid.square(i, j);

                if let Some(piece) = _data.piece(j, i) {
                    sprites.draw(piece, rect);
                }
            }
        }
    }

    fn event(&mut self, ctx: &mut druid::EventCtx, _event: &druid::Event, board: &mut Board, _env: &Env) {
        match _event {
            druid::Event::MouseDown(mouse) => {
                let (x, y) = (mouse.pos.x / self.size.width * 8.0, mouse.pos.y / self.size.height * 8.0);
                let square = Square::new(x as i32, y as i32);

                let next = if let Some(mv) = self.find_selected_move(square) {
                    // click on a square of the move list of the selected piece
                    board.apply(mv);
                    None
                } else if let Some(Piece { color, kind: _ }) = board[square] {
                    // click on a square with a piece?
                    if color == board.active {
                        let moves = board.get_valid_moves(square);
                        Some((square, moves))
                    } else {
                        None
                    }
                } else {
                    None
                };
                self.selected = next;

                // if mouse.button == MouseButton::Right {
                //     // is a piece selected and this is a click on a square of the move list?
                //     if let Some(mv) = self.find_selected_move(square) {
                //         board.apply(mv);
                //         self.selected = None;
                //     }
                // } else if mouse.button == MouseButton::Left {
                //     // is this a click on a square with a piece?
                //     if let Some(Piece { color, kind: _ }) = board[square] {
                //         if color == board.active {
                //             let moves = board.get_valid_moves(square);
                //             self.selected = Some((square, moves));
                //         }
                //     } else {
                //         self.selected = None;
                //     }
                // }
                ctx.set_focus(ctx.widget_id());
                ctx.window().invalidate();
            }
            _ => {}
        }
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &Board, _env: &Env) -> druid::Size {
        let size = if bc.is_width_bounded() && bc.is_height_bounded() {
            let size = Size::new(1328.0, 1328.0);
            bc.constrain(size)
        } else {
            bc.max()
        };
        self.size = Size::new(size.min_side(), size.min_side());
        self.size
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &Board, _env: &Env) {}

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &Board, _data: &Board, _env: &Env) {}
}

mod theme {
    use druid::Color;

    pub const LIGHT: &Color = &Color::rgb8(128, 128, 128);
    pub const DARK: &Color = &Color::rgb8(192, 192, 192);
    pub const LINE: &Color = &Color::rgb8(96, 96, 96);
    pub const SELECTED: &Color = &Color::rgb8(64, 255, 0);
    pub const MOVE: &Color = &Color::rgb8(255, 255, 0);
    pub const TAKE: &Color = &Color::rgb8(255, 0, 0);
    pub const EN_PASSANT: &Color = &Color::rgb8(255, 128, 0);
    pub const CASTLE: &Color = &Color::rgb8(255, 192, 0);
}

struct Sprites<'x, 'a, 'b, 'c> {
    ctx: &'x mut druid::PaintCtx<'a, 'b, 'c>,
    image: Bitmap,
    size: (f64, f64),
}

impl<'x, 'a, 'b, 'c> Sprites<'x, 'a, 'b, 'c> {
    fn new(ctx: &'x mut druid::PaintCtx<'a, 'b, 'c>) -> Self {
        let image = ctx
            .make_image(6 * 166, 2 * 166, &SPRITES, ImageFormat::RgbaSeparate)
            .expect("can't create image");
        let sz = image.get_size();
        let w = (sz.width / 6.0) as f64;
        let h = (sz.height / 2.0) as f64;

        Sprites {
            ctx,
            image: image,
            size: (w, h),
        }
    }

    fn draw(&mut self, piece: Piece, rect: Rect) {
        let src_rect = self.source_rect(piece);

        self.ctx.draw_image_area(&self.image, src_rect, rect, InterpolationMode::Bilinear);
    }

    fn source_rect(&self, piece: Piece) -> Rect {
        let (i, j) = match (piece.color, piece.kind) {
            (Color::White, Kind::King) => (0, 0),
            (Color::White, Kind::Queen) => (1, 0),
            (Color::White, Kind::Bishop) => (2, 0),
            (Color::White, Kind::Knight) => (3, 0),
            (Color::White, Kind::Rook) => (4, 0),
            (Color::White, Kind::Pawn) => (5, 0),

            (Color::Black, Kind::King) => (0, 1),
            (Color::Black, Kind::Queen) => (1, 1),
            (Color::Black, Kind::Bishop) => (2, 1),
            (Color::Black, Kind::Knight) => (3, 1),
            (Color::Black, Kind::Rook) => (4, 1),
            (Color::Black, Kind::Pawn) => (5, 1),
        };

        let (w, h) = self.size;
        Rect::new(i as f64 * w, j as f64 * h, (i + 1) as f64 * w, (j + 1) as f64 * h)
    }
}

lazy_static! {
    static ref SPRITES: Vec<u8> = image::load_from_memory_with_format(include_bytes!("images/pieces.gif"), image::ImageFormat::Gif)
        .unwrap()
        .into_bytes();
}
