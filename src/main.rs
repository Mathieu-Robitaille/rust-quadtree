mod tree;

use crate::tree::{HasPosition, QuadTree, Rect};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use glam::Vec2;
use rand::Rng;
use std::fmt::Debug;

const W: f32 = 800.0;
const H: f32 = 800.0;
const A: f32 = H * W;
const DOTS: usize = 1000;

const OFFSET_VEC: Vec2 = Vec2 {
    x: W / 2.0,
    y: H / 2.0,
};

fn main() {
    /* QuadTree init */

    /* Window init */
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let t = quadtree_init();
    let mut bounds: Vec<Rect> = t.get_bounds();
    let points: Vec<Vec2> = t.get_positions();

    bounds.sort_by(|a, b| a.area().partial_cmp(&b.area()).unwrap());
    bounds.reverse();

    for bound in bounds {
        let shape = shapes::Rectangle {
            extents: bound.size,
            origin: RectangleOrigin::BottomLeft,
        };

        // let norm = normalize_bound(bound.area());
        let transform_vec = Vec3 {
            x: bound.pos.x - OFFSET_VEC.x,
            y: bound.pos.y - OFFSET_VEC.y,
            z: 0.0,
        };

        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLACK),
                outline_mode: StrokeMode::new(Color::WHITE, 1.0),
            },
            Transform::from_translation(transform_vec),
        ));
    }

    for spot in points {
        let shape = shapes::Circle {
            center: spot - OFFSET_VEC,
            radius: 1.0,
        };

        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Stroke(StrokeMode {
                options: StrokeOptions::default(),
                color: Color::RED,
            }),
            Transform::default(),
        ));
    }

    let shape = shapes::Circle {
        center: Vec2 { x: 0.0, y: 0.0 },
        radius: 2.0,
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Stroke(StrokeMode {
            options: StrokeOptions::default(),
            color: Color::GREEN,
        }),
        Transform::default(),
    ));

    commands.spawn_bundle(Camera2dBundle::default());
}

fn quadtree_init() -> QuadTree<Vec2> {
    let x: Rect = Rect {
        pos: Vec2 { x: 0.0, y: 0.0 },
        size: Vec2 { x: W, y: H },
    };
    let mut t: QuadTree<Vec2> = QuadTree::new_empty(x);

    let mut rng = rand::thread_rng();

    /* generate DOTS random points */
    for _ in 1..DOTS {
        let (x, y) = (rng.gen_range(0.0..W), rng.gen_range(0.0..H));
        t.insert(Vec2 { x, y });
    }
    t
}

#[allow(unused)]
fn redraw<T>(_t: QuadTree<T>)
where
    T: HasPosition + Copy + Debug,
{
    unimplemented!();
}

#[allow(unused)]
fn normalize_bound(m: f32) -> f32 {
    let (r_min, r_max) = (0.0, A);
    let (t_min, t_max) = (0.0, 255.0);
    ((m - r_min) / (r_max - r_min)) * (t_max - t_min) + t_min
}
