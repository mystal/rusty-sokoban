extern crate ggez;
extern crate rusty_sokoban;

use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::{conf, graphics, timer};
use ggez::event::*;
use ggez::graphics::{Color, DrawMode, Point};
use rusty_sokoban::level::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameState {
    Play,
    Win,
    Quit,
}

struct Game {
    world: World,
    state: GameState,
    tile_size: f32,
}

impl Game {
    fn new(ctx: &mut Context) -> Self {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        let world = World::new();

        // Determine the size of each tile for drawing.
        let tile_size = (WIDTH as f32 / world.map.width as f32).min(HEIGHT as f32 / world.map.height as f32);

        Game {
            world,
            state: GameState::Play,
            tile_size,
        }
    }

    // TODO: Move to rusty_sokoban lib.
    fn try_move_player(&mut self, dx: isize, dy: isize) -> bool {
        // FIXME: Assuming walls will prevent going negative.
        let (new_x, new_y) = ((self.world.player_pos.0 as isize + dx) as usize,
                              (self.world.player_pos.1 as isize + dy) as usize);

        match self.world.get_tile(new_x, new_y) {
            // New position is a wall, don't do anything.
            Tile::Wall => false,
            _ => if !self.world.boxes.contains(&(new_x, new_y)) {
                // No box blocking the new position, so move there!
                self.world.player_pos = (new_x, new_y);
                true
            } else {
                // Box in the way, so try to push it.
                let (new_box_x, new_box_y) = ((new_x as isize + dx) as usize,
                                              (new_y as isize + dy) as usize);
                match self.world.get_tile(new_box_x, new_box_y) {
                    // Box blocked by a wall, so don't do anything.
                    Tile::Wall => false,
                    // Box can be pushed, so push it and move!
                    _ => if !self.world.boxes.contains(&(new_box_x, new_box_y)) {
                        self.world.boxes.remove(&(new_x, new_y));
                        self.world.boxes.insert((new_box_x, new_box_y));
                        self.world.player_pos = (new_x, new_y);
                        true
                    } else {
                        false
                    },
                }
            },
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        const DESIRED_FPS: u64 = 60;
        if !timer::check_update_time(ctx, DESIRED_FPS) {
            return Ok(());
        }
        let seconds = 1.0 / (DESIRED_FPS as f64);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen...
        graphics::clear(ctx);

        // Draw tiles.
        for (tile, x, y) in self.world.iter_tiles() {
            //let (draw_x, draw_y) = (x + self.map_pos.0, y + self.map_pos.1);
            match tile {
                Tile::Floor | Tile::Empty => {},
                Tile::Wall => {
                    // NOTE: Using the default WHITE color.
                    graphics::set_color(ctx, graphics::WHITE);
                    graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect {
                        x: (x as f32 * self.tile_size) + self.tile_size / 2.0,
                        y: (y as f32 * self.tile_size) + self.tile_size / 2.0,
                        w: self.tile_size,
                        h: self.tile_size,
                    })?;
                },
                Tile::Goal => {
                    graphics::set_color(ctx, Color {
                        r: 1.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    });
                    graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect {
                        x: (x as f32 * self.tile_size) + self.tile_size / 2.0,
                        y: (y as f32 * self.tile_size) + self.tile_size / 2.0,
                        w: self.tile_size,
                        h: self.tile_size,
                    })?;
                },
            }
        }

        // Draw boxes.
        for &(x, y) in &self.world.boxes {
            let color = match self.world.get_tile(x, y) {
                Tile::Goal => Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                },
                _ => Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            };
            graphics::set_color(ctx, color);
            graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect {
                x: (x as f32 * self.tile_size) + self.tile_size / 2.0,
                y: (y as f32 * self.tile_size) + self.tile_size / 2.0,
                w: self.tile_size * 3.0 / 4.0,
                h: self.tile_size * 3.0 / 4.0,
            })?;
        }

        // Draw player.
        {
            let (x, y) = self.world.player_pos;
            let (draw_x, draw_y) = ((x as f32 * self.tile_size) + self.tile_size / 2.0,
                                    (y as f32 * self.tile_size) + self.tile_size / 2.0);
            graphics::set_color(ctx, Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            })?;
            graphics::circle(ctx, DrawMode::Fill, Point::new(draw_x, draw_y), self.tile_size / 2.0 - 2.0, 22)?;
        }


        // Then we flip the screen...
        graphics::present(ctx);

        // And sleep for 0 seconds.
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This prevents the game from using 100% CPU all the time
        // even if vsync is off.
        timer::sleep(Duration::from_secs(0));
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        // NOTE: Hit Escape to quit by default in ggez.
        if self.state == GameState::Play {
            let moved = match keycode {
                Keycode::Up => self.try_move_player(0, -1),
                Keycode::Down => self.try_move_player(0, 1),
                Keycode::Left => self.try_move_player(-1, 0),
                Keycode::Right => self.try_move_player(1, 0),
                _ => false,
            };

            // If moved, check for victory!
            if moved {
                if self.world.boxes.iter().all(|&(x, y)| self.world.get_tile(x, y) == Tile::Goal) {
                    self.state = GameState::Win;
                }
            }
        }
    }
}

fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "Rusty Sokoban".to_string();
    c.window_width = WIDTH;
    c.window_height = HEIGHT;
    //c.window_icon = "/player.png".to_string();

    let ctx = &mut Context::load_from_conf("rusty_sokoban", "mystal", c)
        .expect("Could not create the ggez context.");
    let mut game = Game::new(ctx);
    let result = run(ctx, &mut game);
    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

