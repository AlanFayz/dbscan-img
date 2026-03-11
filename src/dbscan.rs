use std::collections::VecDeque;

use macroquad::prelude::*;

pub trait Sample {
    fn distance_squared(self: &Self, right: &Self) -> f32;
}

#[derive(Default, Debug, Copy, Clone)]
pub struct DbPoint {
    pub cluster_id: Option<i32>,
    pub is_core: bool,
}

pub fn scan<T: Sample>(sample_points: &Vec<T>, min_pts: i32, eps: f32) -> Vec<DbPoint> {
    let mut points: Vec<DbPoint> = sample_points.iter().map(|_| Default::default()).collect();

    for i in 0..sample_points.len() {
        let mut cnt = 0;
        for j in 0..sample_points.len() {
            if i == j {
                continue;
            }

            if sample_points[i].distance_squared(&sample_points[j]) <= (eps * eps) as f32 {
                cnt += 1;
            }
        }

        if cnt >= min_pts {
            points[i].is_core = true;
        }
    }

    let mut cluster_id = 1;
    for i in 0..points.len() {
        match points[i].cluster_id {
            Some(_) => continue,
            _ => {}
        };

        if !points[i].is_core {
            continue;
        }

        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_front(i);

        while !q.is_empty() {
            let front = q.pop_back().unwrap();
            points[front].cluster_id = Some(cluster_id);

            if !points[front].is_core {
                continue;
            }

            for j in 0..points.len() {
                match points[j].cluster_id {
                    Some(_) => continue,
                    _ => {}
                };

                let (l, r) = sample_points.split_at(j.max(front));

                let p0 = &l[j.min(front)];
                let p1 = &r[0];

                if !q.contains(&j) && p0.distance_squared(&p1) <= (eps * eps) as f32 {
                    q.push_front(j);
                }
            }
        }

        cluster_id += 1;
    }

    return points;
}
