use std::iter::Iterator;

#[rustfmt::skip]
const NEIGHBORING_OFFSETS: [(i16, i16); 4] = [
              (0, -1),
    (-1,  0),          (1,  0),
              (0,  1),
];

#[rustfmt::skip]
const ALL_NEIGHBORING_OFFSETS: [(i16, i16); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub struct NeighboringCoords<'a> {
    origin: (u16, u16),
    index: usize,
    neighbors_map: &'a [(i16, i16)],
}

impl<'a> NeighboringCoords<'a> {
    pub fn neighbors(origin: (u16, u16)) -> Self {
        Self {
            origin,
            index: 0,
            neighbors_map: &NEIGHBORING_OFFSETS[..],
        }
    }

    pub fn all_neighbors(origin: (u16, u16)) -> Self {
        Self {
            origin,
            index: 0,
            neighbors_map: &ALL_NEIGHBORING_OFFSETS[..],
        }
    }
}

impl<'a> Iterator for NeighboringCoords<'a> {
    type Item = (u16, u16);
    fn next(&mut self) -> Option<Self::Item> {
        let (origin_x, origin_y) = self.origin;

        while self.index < self.neighbors_map.len() {
            let (x, y) = self.neighbors_map[self.index];
            self.index += 1;

            if origin_x as i16 + x < 0 || origin_y as i16 + y < 0 {
                continue;
            }

            return Some(((origin_x as i16 + x) as u16, (origin_y as i16 + y) as u16));
        }

        None
    }
}
