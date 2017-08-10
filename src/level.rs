use std::iter::Enumerate;
use std::collections::HashSet;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Floor,
    Goal,
    Wall,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn iter_tiles(&self) -> TileIterator {
        TileIterator {
            inner: self.tiles.iter().enumerate(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        let i = (y * self.width) + x;
        self.tiles[i]
    }
}

pub struct TileIterator<'a> {
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
pub struct World {
    pub map: Map,
    pub boxes: HashSet<(usize, usize)>,
    pub player_pos: (usize, usize),
}

impl World {
    pub fn new() -> Self {
        use self::Tile::*;

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

    pub fn iter_tiles(&self) -> TileIterator {
        self.map.iter_tiles()
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.map.get_tile(x, y)
    }
}
