use macroquad::prelude::*;
use ::rand::{rng, RngExt};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

type Polygon = Vec<Point>;

fn sub(a: Point, b: Point) -> Point {
    Point { x: a.x - b.x, y: a.y - b.y }
}

fn dot(a: Point, b: Point) -> f32 {
    a.x * b.x + a.y * b.y
}

fn clip_by_bisector(poly: &Polygon, p: Point, q: Point) -> Polygon {
    if poly.is_empty() {
        return vec![];
    }

    let mid = Point { x: (p.x + q.x) / 2.0, y: (p.y + q.y) / 2.0 };
    let normal = sub(q, p);

    let inside = |pt: Point| dot(sub(pt, mid), normal) <= 0.0;

    let intersect = |a: Point, b: Point| -> Point {
        let da = dot(sub(a, mid), normal);
        let db = dot(sub(b, mid), normal);
        let t = da / (da - db);
        Point {
            x: a.x + t * (b.x - a.x),
            y: a.y + t * (b.y - a.y),
        }
    };

    let mut result = Vec::new();
    let n = poly.len();

    for i in 0..n {
        let curr = poly[i];
        let next = poly[(i + 1) % n];
        match (inside(curr), inside(next)) {
            (true,  true)  => result.push(curr),
            (true,  false) => { result.push(curr); result.push(intersect(curr, next)); }
            (false, true)  => result.push(intersect(curr, next)),
            (false, false) => {}
        }
    }

    result
}

fn voronoi_cell(site: Point, all_sites: &[Point], bbox: &Polygon) -> Polygon {
    let mut cell = bbox.clone();

    for &other in all_sites {
        if (other.x - site.x).abs() < 1e-4 && (other.y - site.y).abs() < 1e-4 {
            continue;
        }
        cell = clip_by_bisector(&cell, site, other);
        if cell.is_empty() {
            break;
        }
    }

    cell
}

fn draw_polygon_filled(poly: &Polygon, color: Color) {
    if poly.len() < 3 {
        return;
    }
    let origin = poly[0];
    for i in 1..poly.len() - 1 {
        draw_triangle(
            vec2(origin.x,    origin.y),
            vec2(poly[i].x,   poly[i].y),
            vec2(poly[i+1].x, poly[i+1].y),
            color,
        );
    }
}

fn draw_polygon_outline(poly: &Polygon, color: Color) {
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        draw_line(a.x, a.y, b.x, b.y, 1.5, color);
    }
}

fn random_color(rng: &mut impl ::rand::Rng) -> Color {
    Color::new(
        rng.random_range(0.3..1.0),
        rng.random_range(0.3..1.0),
        rng.random_range(0.3..1.0),
        1.0,
    )
}

#[macroquad::main("Voronoi")]
async fn main() {
    let mut rng = rng();
    let num_sites = 20;

    let w = screen_width();
    let h = screen_height();

    let sites: Vec<Point> = (0..num_sites)
        .map(|_| Point::new(
            rng.random_range(50.0..w - 50.0),
            rng.random_range(50.0..h - 50.0),
        ))
        .collect();

    let colors: Vec<Color> = (0..num_sites).map(|_| random_color(&mut rng)).collect();

    loop {
        let w = screen_width();
        let h = screen_height();

        let bbox = vec![
            Point::new(0.0, 0.0),
            Point::new(w,   0.0),
            Point::new(w,   h),
            Point::new(0.0, h),
        ];

        clear_background(BLACK);

        for (i, &site) in sites.iter().enumerate() {
            let cell = voronoi_cell(site, &sites, &bbox);
            draw_polygon_filled(&cell, colors[i]);
            draw_polygon_outline(&cell, BLACK);
        }

        for &site in &sites {
            draw_circle(site.x, site.y, 4.0, WHITE);
            draw_circle_lines(site.x, site.y, 4.0, 1.0, BLACK);
        }

        next_frame().await
    }
}