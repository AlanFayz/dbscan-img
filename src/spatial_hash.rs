use std::{array, collections::BTreeMap};

pub trait Spatial<const N: usize> {
    fn position(self: &Self) -> [f32; N];
}

pub struct SpatialHash<const N: usize, T: Spatial<N> + Clone> {
    grid: BTreeMap<[i64; N], Vec<(T, usize)>>,
    cell_size: i64,
}

impl<const N: usize, T: Spatial<N> + Clone> SpatialHash<N, T> {
    pub fn new(cell_size: i64) -> Self {
        Self {
            grid: BTreeMap::new(),
            cell_size,
        }
    }

    pub fn insert(self: &mut Self, item: T, idx: usize) {
        let position = item.position().map(|x| x as i64 / self.cell_size);
        let list = &mut self.grid.entry(position).or_insert(Vec::new());
        list.push((item, idx));
    }

    pub fn remove(self: &mut Self, item: &T) {
        let position = item.position().map(|x| x as i64 / self.cell_size);
        let list = &mut self.grid.entry(position).or_insert(Vec::new());
        let idx = list
            .iter()
            .enumerate()
            .find(|(_, (elem, _))| elem.position() == item.position());

        match idx {
            Some((i, _)) => _ = list.swap_remove(i),
            _ => {}
        };
    }

    pub fn query(self: &Self, item: &T) -> Vec<(T, usize)> {
        return self.query_recursive(item, [0; N], 0);
    }

    fn query_recursive(
        self: &Self,
        item: &T,
        mut offsets: [i64; N],
        depth: usize,
    ) -> Vec<(T, usize)> {
        if depth >= N {
            let base = item.position();
            let position: [i64; N] =
                array::from_fn(|i| (base[i] as i64 / self.cell_size) + offsets[i]);

            return match self.grid.get(&position) {
                Some(val) => val.iter().cloned().collect(),
                None => Vec::new(),
            };
        }

        let mut result = Vec::new();
        for i in -1..2 {
            offsets[depth] = i;
            result.extend(self.query_recursive(item, offsets, depth + 1));
        }

        return result;
    }
}
