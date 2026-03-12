
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
    min_pts: i64,
    eps: f32,
) -> Vec<DbPoint> {
    let mut points: Vec<DbPoint> = sample_points.iter().map(|_| Default::default()).collect();
    let mut grid = SpatialHash::new(eps as i64);
    for (i, point) in sample_points.iter().enumerate() {
        grid.insert(point.clone(), i);
    }

    for i in 0..sample_points.len() {
        let nearby_points = grid.query(&sample_points[i]);
        let cnt = nearby_points.count();
        if cnt >= min_pts as usize {
            points[i].is_core = true;
        }
    }

    let mut cluster_id = 1;

    let mut q: Vec<usize> = Vec::with_capacity(points.len());
    let mut seen: Vec<bool> = vec![false; points.len()];

    for i in 0..points.len() {
        if seen[i] {
            continue;
        }

        seen[i] = true;

        if !points[i].is_core {
            continue;
        }

        q.push(i);
        seen[i] = true;

        while let Some(front) = q.pop() {
            if !points[front].is_core {
                continue;
            }

            points[front].cluster_id = Some(cluster_id);

            let nearby_points = grid.query(&sample_points[front]);
            for (_, idx) in nearby_points {
                if !seen[*idx]
                    && sample_points[*idx].distance_squared(&sample_points[front]) <= (eps * eps)
                {
                    seen[*idx] = true;

                    if points[*idx].is_core {
                        q.push(*idx);
                    }
                }
            }
        }

        cluster_id += 1;
    }

    return points;
}
