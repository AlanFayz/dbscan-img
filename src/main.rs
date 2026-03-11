use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    time::{SystemTime, UNIX_EPOCH},
};

use macroquad::prelude::*;

#[derive(Default, Debug, Copy, Clone)]
struct Point {
    pub position: Vec2,
    pub cluster_id: Option<i32>,
    pub is_core: bool,
}

trait Sample {
    fn distance_squared(self: &Self, right: &Self) -> f32;
}

#[derive(Default, Debug, Copy, Clone)]
struct SamplePoint {
    pub position: Vec2,
}

impl Sample for SamplePoint {
    fn distance_squared(self: &Self, right: &Self) -> f32 {
        self.position.distance_squared(right.position)
    }
}

fn generate_test_data() -> Vec<SamplePoint> {
    let mut points = Vec::new();
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    let gap = 100.0;

    for _ in 0..50 {
        points.push(SamplePoint {
            position: vec2(
                rand::gen_range(10.0, center_x - gap),
                rand::gen_range(10.0, center_y - gap),
            ),
        });
    }

    for _ in 0..50 {
        points.push(SamplePoint {
            position: vec2(
                rand::gen_range(center_x + gap, screen_width() - 10.0),
                rand::gen_range(center_y + gap, screen_height() - 10.0),
            ),
        });
    }

    points.push(SamplePoint {
        position: vec2(center_x, center_y),
    });

    return points;
}

#[macroquad::main("MyGame")]
async fn main() {
    rand::srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let (min_pts, eps) = (5, 50);

    let sample_points = generate_test_data();
    let mut points: Vec<Point> = sample_points
        .iter()
        .map(|p| Point {
            position: p.position,
            ..Default::default()
        })
        .collect();

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
    let mut colors = Vec::new();
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

                let (l, r) = points.split_at_mut(j.max(front));

                let p0 = &mut l[j.min(front)];
                let p1 = &mut r[0];

                if !q.contains(&j)
                    && p0.position.distance_squared(p1.position) <= (eps * eps) as f32
                {
                    q.push_front(j);
                }
            }
        }

        println!("Finished with {}", cluster_id);
        cluster_id += 1;
        colors.push(Color::from_hex(rand::rand()));
    }

    loop {
        clear_background(WHITE);

        for (_, point) in sample_points.iter().zip(points.iter()) {
            let color = match point.cluster_id {
                Some(i) => colors[i as usize - 1],
                _ => BLACK,
            };

            draw_circle(point.position.x, point.position.y, 10.0, color);
        }

        next_frame().await
    }
}
