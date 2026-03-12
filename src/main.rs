use std::{
    collections::HashMap,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use macroquad::prelude::*;
use palette::{FromColor, Lab, Srgb};

use crate::{
    dbscan::{Sample, scan},
    spatial_hash::Spatial,
};

mod dbscan;
mod spatial_hash;

#[derive(Default, Debug, Copy, Clone)]
struct SamplePoint {
    pub position: Vec2,
}

#[derive(Default, Debug, Copy, Clone)]
struct ImagePoint {
    pub position: Vec2,
    pub color: Color,
}

impl Sample for SamplePoint {
    fn distance_squared(self: &Self, right: &Self) -> f32 {
        self.position.distance_squared(right.position)
    }
}

const COLOR_FACTOR: f32 = 500.0;

fn rgb_to_pixel_lab(color: Color) -> [f32; 3] {
    let rgb = Srgb::new(color.r, color.g, color.b);
    let lab = Lab::from_color(rgb);

    // l: 0.0 -> 1.0
    // a, b: 0.0 -> 2.0
    [
        lab.l / 100.0,
        (lab.a + 128.0) / 128.0,
        (lab.b + 128.0) / 128.0,
    ]
}

impl Sample for ImagePoint {
    fn distance_squared(&self, right: &Self) -> f32 {
        let dx = self.position.x - right.position.x;
        let dy = self.position.y - right.position.y;

        let (col0, col1) = (rgb_to_pixel_lab(self.color), rgb_to_pixel_lab(right.color));

        let dr = (col0[0] - col1[0]) * COLOR_FACTOR;
        let dg = (col0[1] - col1[1]) * COLOR_FACTOR;
        let db = (col0[2] - col1[2]) * COLOR_FACTOR;

        return (dx * dx) + (dy * dy) + (dr * dr) + (dg * dg) + (db * db);
    }
}

impl Spatial<2> for SamplePoint {
    fn position(self: &Self) -> [f32; 2] {
        self.position.into()
    }
}

impl Spatial<5> for ImagePoint {
    fn position(self: &Self) -> [f32; 5] {
        let col = rgb_to_pixel_lab(self.color);
        [
            self.position.x,
            self.position.y,
            col[0] * COLOR_FACTOR,
            col[1] * COLOR_FACTOR,
            col[2] * COLOR_FACTOR,
        ]
    }
}

async fn generate_image_data() -> Vec<ImagePoint> {
    let tex = load_texture("image.png").await.unwrap();
    let img = tex.get_texture_data();

    let mut points = Vec::new();
    for y in 0..tex.height() as u32 {
        for x in 0..tex.width() as u32 {
            points.push(ImagePoint {
                position: vec2(x as f32, y as f32),
                color: img.get_pixel(x, y),
            })
        }
    }

    return points;
}

fn _generate_test_data() -> Vec<SamplePoint> {
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

fn window_conf() -> Conf {
    Conf {
        window_title: "dbscan".to_owned(),
        window_width: 1600,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let sample_points = generate_image_data().await;

    let time = Instant::now();
    let points = scan(&sample_points, 12, 10.0);
    println!("{}s", time.elapsed().as_secs_f64());

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

            //draw_circle(sample_point.position.x, sample_point.position.y, 5.0, color);
            draw_rectangle(
                sample_point.position.x,
                sample_point.position.y,
                1.0,
                1.0,
                color,
            );
        }

        next_frame().await
    }
}
