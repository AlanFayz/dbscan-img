use std::{collections::HashMap, 
    time::{SystemTime, UNIX_EPOCH}}
;

use macroquad::prelude::*;

use crate::{dbscan::{Sample, scan}, spatial_hash::Spatial};

mod dbscan;
mod spatial_hash;

#[derive(Default, Debug, Copy, Clone)]
struct SamplePoint {
    pub position: Vec2,
}

impl Sample for SamplePoint {
    fn distance_squared(self: &Self, right: &Self) -> f32 {
        self.position.distance_squared(right.position)
    }
}

impl Spatial<2> for SamplePoint {
    fn position(self: &Self) -> [f32; 2] {
        self.position.into()
    }
}

fn generate_test_data() -> Vec<SamplePoint> {
    let mut points = Vec::new();
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    let gap = 100.0;

    for _ in 0..500 {
        points.push(SamplePoint {
            position: vec2(
                rand::gen_range(10.0, center_x - gap),
                rand::gen_range(10.0, center_y - gap),
            ),
        });
    }

    for _ in 0..500 {
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

    let sample_points = generate_test_data();

    let points = scan(&sample_points, 5, 12.0);

    let mut colors: HashMap<_, Color> = HashMap::new();
    for i in 0..points.len() {
        match points[i].cluster_id {
            Some(i) => _ = colors.entry(i).or_insert(Color::from_hex(rand::rand())),
            None => {}
        };
    }

    loop {
        clear_background(WHITE);

        for (sample_point, point) in sample_points.iter().zip(points.iter()) {
            let color = point
                .cluster_id
                .map(|i| colors.get(&i).unwrap().clone())
                .unwrap_or(BLACK);

            draw_circle(sample_point.position.x, sample_point.position.y, 5.0, color);
        }

        next_frame().await
    }
}
