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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Air,
    Sand,
}

impl Cell {
    fn color(&self) -> Color {
        use Cell::*;
        match self {
            Air => Color::new(0., 0., 0., 0.),
            Sand => Color::from_rgb_u32(0xc2b280),
        }
    }

    fn visible(&self) -> bool {
        use Cell::*;
        *self != Air
    }
}

struct Game {
    conf: GameConf,
    style: Style,
    grid: Vec<Vec<Cell>>,
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf, style: Style) -> Self {
        let w = conf.board_size.x as usize;
        let h = conf.board_size.y as usize;
        let mut grid = Vec::with_capacity(h);
        for _i in 0..h {
            let mut v = Vec::with_capacity(w);
            for _j in 0..w {
                v.push(Cell::Air);
            }
            grid.push(v);
        }
        Self {
            conf,
            style,
            grid,
        }
    }

    fn draw_cells(&mut self, ctx: &mut Context) -> GameResult {
        let w = self.conf.board_size.x as usize;
        let h = self.conf.board_size.y as usize;
        let mut mb = graphics::MeshBuilder::new();
        for i in 0..h {
            for j in 0..w {
                let cell = self.grid[i][j];
                if cell.visible() {
                    mb.rectangle(
                        DrawMode::fill(),
                        Rect::new(
                            j as f32 * self.conf.cell_size,
                            i as f32 * self.conf.cell_size,
                            self.conf.cell_size,
                            self.conf.cell_size,
                        ),
                        cell.color(),
                    )?;
                }
            }
        }
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())
    }

    fn draw_grid(&mut self, ctx: &mut Context) -> GameResult {
        let mut mb = graphics::MeshBuilder::new();
        for i in 0..(self.conf.board_size.y as i32) {
            let y = i as f32 * self.conf.cell_size;
            mb.line(
                &[
                    Point::new(0., y),
                    Point::new(self.conf.window_size.x, y),
                ],
                self.style.grid_line_width,
                self.style.grid_line_color,
                )?;
        }
        for i in 0..(self.conf.board_size.x as i32) {
            let x = i as f32 * self.conf.cell_size;
            mb.line(
                &[
                    Point::new(x, 0.),
                    Point::new(x, self.conf.window_size.y),
                ],
                self.style.grid_line_width,
                self.style.grid_line_color,
                )?;
        }
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.draw_cells(ctx)?;
        self.draw_grid(ctx)?;
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
    show_grid_lines: bool,
}

impl GameConf {
    fn new(window_size: Point, cell_size: f32) -> Self {
        let board_size = window_size * (1. / cell_size);
        Self {
            window_size,
            board_size,
            cell_size,
            ..Default::default()
        }
    }
}

impl Default for GameConf {
    fn default() -> Self {
        Self {
            window_size: Point::new(2400., 1800.),
            board_size: Point::new(480., 360.),
            cell_size: 5.,
            show_grid_lines: false,
        }
    }
}

struct Style {
    grid_line_width: f32,
    grid_line_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            grid_line_width: 1.,
            grid_line_color: Color::from_rgb_u32(0x505050),
        }
    }
}

fn main() -> GameResult {
    let conf = GameConf::new(Point::new(2400., 1800.), 60.);
    // let conf = GameConf::default();
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
    let game = Game::new(&mut ctx, conf, Default::default());
    graphics::set_window_position(&ctx, Point::new(20., 20.))?;
    event::run(ctx, event_loop, game)
}
