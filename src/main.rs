extern crate rustbox;

use std::collections::HashSet;
use std::iter::Enumerate;
use std::slice::Iter;

use rustbox::{Color, Event, Key, RustBox, Style};

const WALL_CELL: Cell = Cell {
    ch: '#',
    style: rustbox::RB_NORMAL,
    fg: Color::Default,
    bg: Color::Default,
};
const GOAL_CELL: Cell = Cell {
    ch: '.',
    style: rustbox::RB_BOLD,
    fg: Color::Default,
    bg: Color::Default,
};
const PLAYER_CELL: Cell = Cell {
    ch: '@',
    style: rustbox::RB_BOLD,
    fg: Color::Default,
    bg: Color::Default,
};
const PLAYER_ON_GOAL_CELL: Cell = Cell {
    ch: '+',
    style: rustbox::RB_BOLD,
    fg: Color::Default,
    bg: Color::Default,
};
const BOX_CELL: Cell = Cell {
    ch: '$',
    style: rustbox::RB_BOLD,
    fg: Color::Default,
    bg: Color::Default,
};
const BOX_ON_GOAL_CELL: Cell = Cell {
    ch: '*',
    style: rustbox::RB_BOLD,
    fg: Color::White,
    bg: Color::Green,
};

struct Cell {
    ch: char,
    style: Style,
    fg: Color,
    bg: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Floor,
    Goal,
    Wall,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn iter_tiles(&self) -> TileIterator {
        TileIterator {
            inner: self.tiles.iter().enumerate(),
            width: self.width,
            height: self.height,
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        let i = (y * self.width) + x;
        self.tiles[i]
    }
}

struct TileIterator<'a> {
    inner: Enumerate<Iter<'a, Tile>>,
    width: usize,
    height: usize,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = (Tile, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.inner.next();
        tile.map(|(i, tile)| {
            let (x, y) = (i % self.width, i / self.width);
            (*tile, x, y)
        })
    }
}

#[derive(Debug)]
struct World {
    map: Map,
    boxes: HashSet<(usize, usize)>,
    player_pos: (usize, usize),
}

impl World {
    fn new() -> Self {
        use Tile::*;

        let mut boxes = HashSet::new();

        /*
         * #####
         * #@$.#
         * #####
         */
        //let player_pos = (1, 1);
        //boxes.insert((2, 1));
        //let tiles = vec![
        //    Wall, Wall, Wall, Wall, Wall,
        //    Wall, Floor, Floor, Floor, Wall,
        //    Wall, Wall, Wall, Wall, Wall,
        //];
        //let width = 5;
        //let height = 3;

        /*
         * #######
         * #.@ # #
         * #$* $ #
         * #   $ #
         * # ..  #
         * #  *  #
         * #######
         */
        let player_pos = (2, 1);
        boxes.insert((1, 2));
        boxes.insert((2, 2));
        boxes.insert((4, 2));
        boxes.insert((4, 3));
        boxes.insert((3, 5));
        let tiles = vec![
            Wall, Wall, Wall, Wall, Wall, Wall, Wall,
            Wall, Goal, Floor, Floor, Wall, Floor, Wall,
            Wall, Floor, Goal, Floor, Floor, Floor, Wall,
            Wall, Floor, Floor, Floor, Floor, Floor, Wall,
            Wall, Floor, Goal, Goal, Floor, Floor, Wall,
            Wall, Floor, Floor, Goal, Floor, Floor, Wall,
            Wall, Wall, Wall, Wall, Wall, Wall, Wall,
        ];
        let width = 7;
        let height = 7;

        let map = Map {
            tiles,
            width,
            height,
        };

        World {
            map,
            boxes,
            player_pos,
        }
    }

    fn iter_tiles(&self) -> TileIterator {
        self.map.iter_tiles()
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.map.get_tile(x, y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameState {
    Play,
    Win,
    Quit,
}

struct Game {
    rb: RustBox,
    world: World,
    map_pos: (usize, usize),
    actions_pos: (usize, usize),
    status_pos: (usize, usize),
    //mines_pos: (usize, usize),
    state: GameState,
}

impl Game {
    fn new(rb: RustBox) -> Game {
        let world = World::new();
        let map_pos = (20, 1);
        let status_pos = (0, map_pos.1 + world.map.height + 3);

        Game {
            rb,
            world,
            map_pos,
            actions_pos: (0, 2),
            status_pos,
            //mines_pos: (0, 0),
            state: GameState::Play,
        }
    }

    //fn reset(&mut self) {
    //    self.status_pos = (0, self.grid_pos.1 + self.grid.height() as usize + 3);
    //    self.mines_pos = (self.grid_pos.0 + self.grid.width() as usize / 2, 0);
    //    self.state = GameState::Play;
    //}

    fn try_move_player(&mut self, dx: isize, dy: isize) {
        // FIXME: Assuming walls will prevent going negative.
        let (new_x, new_y) = ((self.world.player_pos.0 as isize + dx) as usize,
                              (self.world.player_pos.1 as isize + dy) as usize);

        match self.world.get_tile(new_x, new_y) {
            // New position is a wall, don't do anything.
            Tile::Wall => {},
            _ => if !self.world.boxes.contains(&(new_x, new_y)) {
                // No box blocking the new position, so move there!
                self.world.player_pos = (new_x, new_y);
            } else {
                // Box in the way, so try to push it.
                let (new_box_x, new_box_y) = ((new_x as isize + dx) as usize,
                                              (new_y as isize + dy) as usize);
                match self.world.get_tile(new_box_x, new_box_y) {
                    // Box blocked by a wall, so don't do anything.
                    Tile::Wall => {},
                    // Box can be pushed, so push it and move!
                    _ => if !self.world.boxes.contains(&(new_box_x, new_box_y)) {
                        self.world.boxes.remove(&(new_x, new_y));
                        self.world.boxes.insert((new_box_x, new_box_y));
                        self.world.player_pos = (new_x, new_y);
                    },
                }
            },
        }
    }

    fn update(&mut self) {
        match self.state {
            GameState::Play => self.play_update(),
            GameState::Win => self.win_update(),
            GameState::Quit => {},
        }
    }

    fn play_update(&mut self) {
        match self.rb.poll_event(false).unwrap() {
            Event::KeyEvent(key) => {
                match key {
                    Key::Up => self.try_move_player(0, -1),
                    Key::Down => self.try_move_player(0, 1),
                    Key::Left => self.try_move_player(-1, 0),
                    Key::Right => self.try_move_player(1, 0),
                    //Key::Char('n') => self.state = GameState::New,
                    //Key::Char('r') => self.reset_level(),
                    Key::Char('q') => self.state = GameState::Quit,
                    _ => {},
                }
            },
            _ => {},
        };

        if self.world.boxes.iter().all(|&(x, y)| self.world.get_tile(x, y) == Tile::Goal) {
            self.state = GameState::Win;
        }
    }

    fn win_update(&mut self) {
        match self.rb.poll_event(false).unwrap() {
            Event::KeyEvent(key) => {
                match key {
                    //Key::Char('n') => self.state = GameState::New,
                    Key::Char('q') => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn display(&self) {
        self.rb.clear();

        // Title
        self.rb.print(0, 0, rustbox::RB_BOLD, Color::Default, Color::Default, "Rusty Sokoban");

        //self.draw_actions();

        // Mine counter
        //self.rb.print(self.mines_pos.0, self.mines_pos.1,
        //              rustbox::RB_BOLD, Color::Red, Color::White,
        //              &format!("{:02}", self.grid.mines_left()));

        self.draw_map();

        self.draw_status();

        self.rb.present();
    }

    fn draw_map(&self) {
        // Draw tiles.
        for (tile, x, y) in self.world.iter_tiles() {
            let (draw_x, draw_y) = (x + self.map_pos.0, y + self.map_pos.1);
            match tile {
                // Floor tiles are left empty.
                Tile::Floor => {},
                Tile::Wall => self.rb.print_char(draw_x, draw_y, WALL_CELL.style, WALL_CELL.fg, WALL_CELL.bg, WALL_CELL.ch),
                Tile::Goal => self.rb.print_char(draw_x, draw_y, GOAL_CELL.style, GOAL_CELL.fg, GOAL_CELL.bg, GOAL_CELL.ch),
            }
        }

        // Draw boxes.
        for &(x, y) in &self.world.boxes {
            let (draw_x, draw_y) = (x + self.map_pos.0, y + self.map_pos.1);
            match self.world.get_tile(x, y) {
                Tile::Goal => self.rb.print_char(draw_x, draw_y, BOX_ON_GOAL_CELL.style, BOX_ON_GOAL_CELL.fg, BOX_ON_GOAL_CELL.bg, BOX_ON_GOAL_CELL.ch),
                _ => self.rb.print_char(draw_x, draw_y, BOX_CELL.style, BOX_CELL.fg, BOX_CELL.bg, BOX_CELL.ch),
            }
        }

        // Draw player.
        {
            let (x, y) = self.world.player_pos;
            let (draw_x, draw_y) = (x + self.map_pos.0, y + self.map_pos.1);
            match self.world.get_tile(x, y) {
                Tile::Goal => self.rb.print_char(draw_x, draw_y, PLAYER_ON_GOAL_CELL.style, PLAYER_ON_GOAL_CELL.fg, PLAYER_ON_GOAL_CELL.bg, PLAYER_ON_GOAL_CELL.ch),
                _ => self.rb.print_char(draw_x, draw_y, PLAYER_CELL.style, PLAYER_CELL.fg, PLAYER_CELL.bg, PLAYER_CELL.ch),
            }
        }
    }

    //fn draw_actions(&self) {
    //    for (i, text) in ACTION_STRINGS[self.state as usize].iter().enumerate() {
    //        self.rb.print(self.actions_pos.0, self.actions_pos.1 + i,
    //                      rustbox::RB_NORMAL, Color::Default, Color::Default, text);
    //    }
    //}

    fn draw_status(&self) {
        let status = match self.state {
            GameState::Play => "Play!",
            //GameState::Lose => "You lose...",
            GameState::Win => "You win!",
            //GameState::New => "Choose a difficulty",
            _ => "",
        };
        self.rb.print(self.status_pos.0, self.status_pos.1,
                      rustbox::RB_NORMAL, Color::Default, Color::Default, &status);
    }
}

fn main() {
    let rb = RustBox::init(Default::default())
        .expect("Failed to init rustbox");
    // Hide the cursor.
    rb.set_cursor(-1, -1);

    let mut game = Game::new(rb);

    while game.state != GameState::Quit {
        game.display();
        game.update();
    }
}
