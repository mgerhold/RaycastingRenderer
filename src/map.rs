use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub enum TileType {
    Occupied,
    Free,
}

pub struct Map {
    width: usize,
    tiles: Vec<TileType>,
}

impl Map {

    pub fn new((width, height): (usize, usize)) -> Map {
        let tiles = vec![TileType::Free; width * height];
        Map {
            width,
            tiles,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.tiles.len() / self.width)
    }

}

impl Index<(usize, usize)> for Map {
    type Output = TileType;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[x + y * self.width]
    }
}