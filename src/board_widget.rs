use druid::piet::d2d::Bitmap;
use druid::piet::{D2DTextLayout, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::{Env, Rect, RenderContext, Size, Widget};
use log::{trace, warn};

use crate::grid::Grid;
use crate::{Board, Color, Kind, Move, MoveKind, Piece, Square};

pub struct BoardWidget {
    selected: Option<(Square, Vec<Move>)>,
    size: Size,
    labels: Option<([D2DTextLayout;8],[D2DTextLayout;8])>
}

impl BoardWidget {
    pub fn new() -> Self {
        Self {
            selected: None,
            size: Size::default(),
            labels: None
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

    pub fn move_color<'a,'b>(mv: &Move, env: &Env) -> druid::Color {
        match mv.kind {
            MoveKind::Move() => env.get(theme::MOVE),
            MoveKind::Take(_) => env.get(theme::TAKE),
            MoveKind::EnPassant() => env.get(theme::EN_PASSANT),
            MoveKind::Castle(_, _) => env.get(theme::CASTLE),
        }
    }
}

impl Widget<Board> for BoardWidget {
    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Board, env: &Env) {
        let grid = Grid::new(ctx.size());

        for i in 0..8 {
            for j in 0..8 {
                let brush = if (i + j) % 2 == 0 { env.get(theme::LIGHT) } else { env.get(theme::DARK) };
                ctx.fill(grid.square(i, j), &brush);
            }
        }
        for j in 0..=8 {
            ctx.stroke(grid.horz_line(j), &env.get(theme::GRID_LINE), 1.0);
        }
        for i in 0..=8 {
            ctx.stroke(grid.vert_line(i), &env.get(theme::GRID_LINE), 1.0);
        }

        if let Some(labels) = &self.labels{
            const O: f64 = -2.0;
            for j in 0..8 {
                let rect =  grid.square(0, j).inflate(O, O); 
                // let text = ctx.text()
                //     .new_text_layout(format!("{}", 8-j))
                //     .font(druid::FontFamily::MONOSPACE, 16.0)
                //     .text_color(druid::Color::rgb8(64,64,64))
                //     .build().unwrap();        
                ctx.draw_text(&labels.1[j as usize], rect.origin());        
                // ctx.draw_text(&text, rect.origin());
            }
            for i in 0..8 {
                let rect =  grid.square(i, 7).inflate(O, O); 
                // let text = ctx.text()
                //     .new_text_layout(format!("{}", (('a' as u8)+(i as u8))as char))
                //     .font(druid::FontFamily::MONOSPACE, 16.0)
                //     .text_color(druid::Color::rgb8(64,64,64))
                //     .max_width(rect.width())                
                //     .alignment(druid::TextAlignment::End)
                //     .build().unwrap();                                        
                ctx.draw_text(&labels.0[i as usize], rect.origin());
            }
        }

        if ctx.has_focus() {
            if let Some((sq, moves)) = &self.selected {
                let rect = grid.rect(*sq).inflate(-5.0, -5.0);
                ctx.stroke(rect, &env.get(theme::SELECTED), 5.0);

                for mv in moves {
                    let c = Self::move_color(mv, &env);
                    let rect = grid.rect(mv.target).inflate(-5.0, -5.0);
                    ctx.stroke(rect, &c, 5.0);
                }
            }
        }

        let mut sprites = Sprites::new(ctx);

        for j in 0..8 {
            for i in 0..8 {
                let rect = grid.square(i, j);

                if let Some(piece) = data.piece(j, i) {
                    sprites.draw(piece, rect);
                }
            }
        }
    }

    fn event(&mut self, ctx: &mut druid::EventCtx, _event: &druid::Event, board: &mut Board, _env: &Env) {
        let grid = Grid::new(ctx.size());

        match _event {
            druid::Event::MouseDown(mouse) => {
                let square = grid.square_from_mouse(mouse);

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

                ctx.set_focus(ctx.widget_id());
                ctx.window().invalidate();
            }
            _ => {}
        }
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &Board, _env: &Env) -> druid::Size {
        trace!("{:?}", bc);
        // let size = if bc.is_width_bounded() && bc.is_height_bounded() {
        //     let size = Size::new(1328.0, 1328.0);
        //     bc.constrain(size)
        // } else {
        //     bc.max()
        // };        
        self.size = Size::new(500.0, 500.0);
        self.size
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &Board, _env: &Env) {}

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &Board, _data: &Board, _env: &Env) {
        
        let grid = Grid::new(ctx.size());
        const O: f64 = -2.0;

        let mut buildv =  |j: i32| -> D2DTextLayout {
            // let rect =  grid.square(0, j).inflate(O, O); 
            ctx.text()
                .new_text_layout(format!("{}", 8-j))
                .font(druid::FontFamily::MONOSPACE, 16.0)
                .text_color(druid::Color::rgb8(64,64,64))
                .build()
                .unwrap()
        };
        let v = [ 
            buildv(0), buildv(1), buildv(2), buildv(3),
            buildv(4), buildv(5), buildv(6), buildv(7)
        ];
        let mut buildhh = move |i: i32| -> D2DTextLayout {
            let rect =  grid.square(i, 7).inflate(O, O); 
            ctx.text()
                .new_text_layout(format!("{}", (('a' as u8)+(i as u8))as char))
                .font(druid::FontFamily::MONOSPACE, 16.0)
                .text_color(druid::Color::rgb8(64,64,64))
                .max_width(rect.width())                
                .alignment(druid::TextAlignment::End)
                .build()
                .unwrap()       
        };
        let h = [ 
            buildhh(0), buildhh(1), buildhh(2), buildhh(3),
            buildhh(4), buildhh(5), buildhh(6), buildhh(7)
        ];

        self.labels = Some((h, v));
        warn!("labels initialized");
    }
}


pub mod theme {
    use druid::{Color, Key};

    pub const GRID_LINE: Key<Color> = Key::new("grid-line");
    pub const LIGHT: Key<Color> = Key::new("light");
    pub const DARK: Key<Color> = Key::new("dark");
    pub const LINE: Key<Color> = Key::new("line"); 
    pub const SELECTED: Key<Color> = Key::new("selected");
    pub const MOVE: Key<Color> = Key::new("move");
    pub const TAKE: Key<Color> = Key::new("take");
    pub const EN_PASSANT: Key<Color> = Key::new("en_passant");
    pub const CASTLE: Key<Color> = Key::new("castle");
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
