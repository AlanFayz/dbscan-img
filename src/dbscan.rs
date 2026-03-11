use std::{collections::{BTreeMap, HashSet, VecDeque}, time::Instant};

use macroquad::prelude::*;

use crate::spatial_hash::{Spatial, SpatialHash};

pub trait Sample {
    fn distance_squared(self: &Self, right: &Self) -> f32;
}

#[derive(Default, Debug, Copy, Clone)]
pub struct DbPoint {
    pub cluster_id: Option<i32>,
    pub is_core: bool,
}

pub fn scan<const N: usize, T: Sample + Spatial<N> + Clone>(
    sample_points: &Vec<T>,
    min_pts: i32,
    eps: f32,
) -> Vec<DbPoint> {
    let mut points: Vec<DbPoint> = sample_points.iter().map(|_| Default::default()).collect();
    let mut grid = SpatialHash::new(eps as i64);
    for (i, point) in sample_points.iter().enumerate() {
        grid.insert(point.clone(), i);
    }

    //println!("Starting core query");
    //let start = Instant::now();
    for i in 0..sample_points.len() {
        //println!("{}%", i as f32 * 100.0 / points.len() as f32);
        let nearby_points = grid.query(&sample_points[i]);
        if nearby_points.len() >= min_pts as usize {
            points[i].is_core = true;
        }
    }
    //println!("Finished core query, took {}s", start.elapsed().as_secs_f64());


    //println!("Starting breadth first search");
    //let start = Instant::now();
    let mut cluster_id = 1;
    //let mut sm = 0;
    for i in 0..points.len() {
        match points[i].cluster_id {
            Some(_) => continue,
            _ => {}
        };

        if !points[i].is_core {
            continue;
        }

        let mut q: VecDeque<usize> = VecDeque::new();
        let mut seen: HashSet<usize> = HashSet::new();
        q.push_front(i);

        while !q.is_empty() {
            let front = q.pop_back().unwrap();
            seen.insert(front);

            //println!("{}%", sm as f32 * 100.0 / points.len() as f32);
            //sm += 1;

            if !points[front].is_core {
                continue;
            }

            points[front].cluster_id = Some(cluster_id);

            let nearby_points = grid.query(&sample_points[front]);
            for (_, idx) in &nearby_points {
                match points[*idx].cluster_id {
                    Some(_) => continue,
                    _ => {}
                };

                if !seen.contains(idx)
                    && sample_points[*idx].distance_squared(&sample_points[front]) <= (eps * eps)
                {
                    seen.insert(*idx);

                    if points[*idx].is_core {
                        q.push_front(*idx);
                    }
                }
            }
        }

        cluster_id += 1;
    }

    //println!("Finished breadth first search, took {}s", start.elapsed().as_secs_f64());

    return points;
}
