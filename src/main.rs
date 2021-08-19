mod coord;
use ggez::{
    timer,
    Context,
    ContextBuilder,
    GameResult,
    GameError,
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    input::{
        mouse::MouseButton,
        keyboard::{
            self,
            KeyCode,
            KeyMods,
        },
    },
    graphics::{
        self,
        Rect,
        Color,
        DrawMode,
        DrawParam,
    },
};
use rand::prelude::*;

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
        Self {
            conf,
            style,
            grid: Game::_new_grid(&conf),
        }
    }

    fn _new_grid(conf: &GameConf) -> Vec<Vec<Cell>> {
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
        grid
    }

    fn new_grid(&self) -> Vec<Vec<Cell>> {
        Game::_new_grid(&self.conf)
    }

    fn draw_cells(&mut self, ctx: &mut Context) -> GameResult {
        let w = self.conf.board_size.x as usize;
        let h = self.conf.board_size.y as usize;
        let mut mb = graphics::MeshBuilder::new();
        let mut empty = true;
        for i in 0..h {
            for j in 0..w {
                let cell = self.grid[i][j];
                if cell.visible() {
                    empty = false;
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
        if !empty {
            let mesh = mb.build(ctx)?;
            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }
        Ok(())
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

    fn in_grid(&self, x: usize, y: usize) -> bool {
        let w = self.conf.board_size.x as usize;
        let h = self.conf.board_size.y as usize;
        x < w && y < h
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == Cell::Air
    }

    fn check_cell(&self, x: usize, y: usize) -> bool {
        self.in_grid(x, y) && self.is_empty(x, y) 
    }

    fn move_cell(&mut self, x: usize, y: usize, to_x: usize, to_y: usize){
        assert!(self.check_cell(to_x, to_y));
        let cell = self.grid[y][x];
        self.grid[y][x] = Cell::Air;
        self.grid[to_y][to_x] = cell;
    }
    
    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if self.in_grid(x, y) {
            self.grid[y][x] = cell;
        }
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !timer::check_update_time(ctx, self.conf.fps) {
            return Ok(());
        }
        let w = self.conf.board_size.x as usize;
        let h = self.conf.board_size.y as usize;
        for i in (0..h).rev() {
            for j in 0..w {
                let cell = self.grid[i][j];
                match cell {
                    Cell::Air => (),
                    Cell::Sand => {
                        if self.check_cell(j, i + 1) { // cell below
                            self.move_cell(j, i, j, i + 1);
                        } else {
                            match (
                                self.check_cell(j + 1, i + 1),
                                self.check_cell(j.wrapping_sub(1), i + 1),
                            ) {
                                (true, true) => if rand::random() {
                                    self.move_cell(j, i, j + 1, i + 1)
                                } else {
                                    self.move_cell(j, i, j.wrapping_sub(1), i + 1)
                                },
                                (true, false) => self.move_cell(j, i, j + 1, i + 1),
                                (false, true) => self.move_cell(j, i, j.wrapping_sub(1), i + 1),
                                (false, false) => (),
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.draw_cells(ctx)?;
        if self.conf.show_grid_lines {
            self.draw_grid(ctx)?;
        }
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

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) {
        match button {
            MouseButton::Left => {
                let x = (x / self.conf.cell_size) as usize;
                let y = (y / self.conf.cell_size) as usize;
                let s = self.conf.brush_size;
                for i in *y.checked_sub(s).get_or_insert(0)..=(y + s) {
                    for j in *x.checked_sub(s).get_or_insert(0)..=(x + s) {
                        self.set_cell(j, i, Cell::Sand);
                    }
                }
            },
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
    fps: u32,
    brush_size: usize,
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
            board_size: Point::new(120., 90.),
            cell_size: 20.,
            show_grid_lines: false,
            fps: 10,
            brush_size: 10,
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
    let conf = GameConf::default();
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
