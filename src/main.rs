mod coord;
use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    GameError,
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    input::keyboard::{
        self,
        KeyCode,
        KeyMods,
    },
    graphics::{
        self,
        Rect,
        Color,
        DrawMode,
        DrawParam,
    },
};

type Point = coord::Coord;

struct Game {
    conf: GameConf,
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf) -> Self {
        Self {
            conf,
        }
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
        match keycode {
            KeyCode::Q |
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct GameConf {
    window_size: Point,
    board_size: Point,
    cell_size: f32,
}

impl GameConf {
    fn new(window_size: Point, cell_size: f32) -> Self {
        let board_size = window_size * (1. / cell_size);
        Self {
            window_size,
            board_size,
            cell_size,
        }
    }
}

fn main() -> GameResult {
    let conf = GameConf::new(Point::new(2400., 1800.), 300.);
    dbg!(conf);
    let (mut ctx, event_loop) = ContextBuilder::new("Falling Sand", "KermitPurple")
        .window_setup(WindowSetup{
            title: String::from("Falling Sand"),
            ..Default::default()
        })
        .window_mode(WindowMode{
            width: conf.window_size.x,
            height: conf.window_size.y,
            ..Default::default()
        })
        .build()?;
    let game = Game::new(&mut ctx, conf);
    graphics::set_window_position(&ctx, Point::new(20., 20.))?;
    event::run(ctx, event_loop, game)
}
