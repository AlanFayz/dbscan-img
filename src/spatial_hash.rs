use std::collections::BTreeMap;

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

    pub fn _remove(self: &mut Self, item: &T) {
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

    pub fn query<'a>(&'a self, item: &T) -> impl Iterator<Item = &'a (T, usize)> + 'a {
        let base = item.position();
        let cell_size = self.cell_size as i64;

        let center: [i64; N] = std::array::from_fn(|i| base[i] as i64 / cell_size);

        (0..3i64.pow(N as u32))
            .filter_map(move |i| {
                let mut target = center;
                let mut k = i;

                for d in 0..N {
                    target[d] += (k % 3) - 1;
                    k /= 3;
                }

                self.grid.get(&target)
            })
            .flatten() 
    }
}
