use glam::{Vec2, Mat2};

use std::fmt::Debug;
use bevy::prelude::Component;

const CAPACITY: usize = 4;

// https://www.topcoder.com/thrive/articles/Geometry%20Concepts%20part%202:%20%20Line%20Intersection%20and%20its%20Applications


pub struct QuadTree<T> 
where
    T: HasPosition + Copy + Debug
{
    pub bounds: Rect,
    pub points: Option<Vec<T>>,
    pub inner_trees: Option<Box::<[QuadTree<T>; 4]>>
}

impl<T> QuadTree<T> 
where
    T: HasPosition + Copy + Debug
{
    #[allow(unused)]
    pub fn new_empty(r: Rect) -> QuadTree<T> {
        QuadTree { bounds: r, points: None, inner_trees: None }
    }

    #[allow(unused)]
    pub fn from(i: Vec<T>, r: Rect) -> QuadTree<T> {
        let mut q: QuadTree<T> = QuadTree{ bounds: r, points: None, inner_trees: None };
        for obj in i {
            q.insert(obj);
        }
        q
    }

    // Attempt to insert a point into the quadtree structure
    // return false if it cannot be inserted for any reason
    #[allow(unused)]
    pub fn insert(&mut self, p: T) -> bool {
        if !self.bounds.contains(&p) { return false; }
        match (self.points.as_mut(), self.inner_trees.as_mut()) {
            // this tree itself contains points
            (Some(val), None) => {
                if val.len() == CAPACITY - 1 || val.len() >= CAPACITY {
                    val.push(p);
                    self.subdivide();
                    return true;
                } else {
                    val.push(p);
                    return true;
                }
            }
            // This tree contains subtrees but no points
            (None, Some(val)) => {
                let x = &mut **val;
                return x
                    .iter_mut()
                    .any(|x| x.insert(p))
            }
            // This tree has nothing cool about it
            (None, None) => {
                self.points = Some(vec![p]);
                return true;
            }
            _ => {
                panic!("HOW DID WE GET HERE")
            }
        }
    }


    // Returns a copy of each elements position
    // This is intended to be a read-only func
    #[allow(unused)]
    pub fn get_positions(&self) -> Vec<Vec2> {
        let mut res: Vec<Vec2> = vec![];

        match (&self.points, &self.inner_trees) {
            // this tree itself contains points
            // get all the points and send them back up
            (Some(val), None) => {
                let mut x: Vec<Vec2> = val.iter()
                    .map(|x| x.get_pos())
                    .collect();
                res.append(&mut x);
            },

            // This tree contains subtrees but no points
            (None, Some(val)) => {
                let x = &**val;
                for i in x {
                    let mut cont = i.get_positions();
                    res.append(&mut cont);
                }
            },
            _ => ()
        }
        res
    }

    #[allow(unused)]
    pub fn get_bounds(&self) -> Vec<Rect> {
        let mut result: Vec<Rect> = vec![];
        result.push(self.bounds);
        result.append(&mut self.get_bounds_internal());
        result
    }

    #[allow(unused)]
    fn get_bounds_internal(&self) -> Vec<Rect> {
        let mut result: Vec<Rect> = vec![self.bounds];
        if let Some(box_inner_trees) = &self.inner_trees {
            let inner_trees = &**box_inner_trees;
            for x in inner_trees.iter() {
                result.append(&mut x.get_bounds_internal());
                // println!("{:?}", result);
            }
        }
        result
    }

    #[allow(unused)]
    fn get_intersecting_bounds() -> Vec<QuadTree<T>> {
        todo!()
    }

    fn get_intersecting_bounds_internal() -> Vec<QuadTree<T>> {
        todo!()
    }

    // Get the objects inside this structure
    // I wonder if this really should be a thing
    // This would allow writing of these objects
    #[allow(unused)]
    pub fn get_objects(&self) -> Vec<T> {
        let res: Vec<T> = vec![];
        
        match (self.points.as_ref(), self.inner_trees.as_ref()) {
            // this tree itself contains points
            // get all the points and send them back up
            (Some(val), None) => {
                
            },

            // This tree contains subtrees but no points
            (None, Some(val)) => {

            },
            _ => ()
        }
        res
    }

    fn subdivide(&mut self) {
        let half_w = self.bounds.size.x / 2.0;
        let half_h = self.bounds.size.y / 2.0;

        // r1 is quartant 1
        // r2 is quadrant 2
        // r3 is quadtant 3
        // r4 is quadtant 4
        //  of the cartesian plane

        /*

      (0, 0)
        |
        V
         _______ _______
        |       |       |
        |   2   |   1   |
        |_______|_______|
        |       |       |
        |   3   |   4   |
        |_______|_______|

        */


        let r1 = Rect { 
            pos: Vec2 {x: self.bounds.pos.x + half_w, y: self.bounds.pos.y },
            size: Vec2 {x: half_w, y: half_h } 
        };
        let r2 = Rect { 
            pos: Vec2 { x: self.bounds.pos.x, y: self.bounds.pos.y }, 
            size: Vec2 { x: half_w, y: half_h }
        };
        let r3 = Rect { 
            pos: Vec2 { x: self.bounds.pos.x, y: self.bounds.pos.y + half_h }, 
            size: Vec2 { x: half_w, y: half_h}
        };
        let r4 = Rect { 
            pos: Vec2 { x: self.bounds.pos.x + half_w, y: self.bounds.pos.y + half_w }, 
            size: Vec2 { x: half_w, y: half_h }
        };

        let ret: [QuadTree<T>; 4] = [QuadTree {
            bounds: r1,
            points: Some(r1.points_inside(&self.points)),
            inner_trees: None,
        }, 
        QuadTree {
            bounds: r2,
            points: Some(r2.points_inside(&self.points)),
            inner_trees: None,
        }, 
        QuadTree {
            bounds: r3,
            points: Some(r3.points_inside(&self.points)),
            inner_trees: None,
        },
        QuadTree {
            bounds: r4,
            points: Some(r4.points_inside(&self.points)),
            inner_trees: None,
        }];

        self.inner_trees = Some(Box::new(ret));
        self.points = None;
    }

}


pub trait HasPosition {
    fn get_pos(&self) -> Vec2;
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn contains<T: HasPosition>(&self, v: &T) -> bool {
        if v.get_pos().x >= self.pos.x && v.get_pos().x < self.pos.x + self.size.x && v.get_pos().y >= self.pos.y && v.get_pos().y < self.pos.y + self.size.y {
            return true;
        }
        false
    }

    pub fn area(&self) -> f32 {
        self.size.x * self.size.y
    }
    // Return the points in the specified rectangle
    pub fn points_inside<T>(&self, points: &Option<Vec<T>>) -> Vec<T> 
    where
        T: HasPosition + Copy
    {
        if let Some(p) = points {
            return p.iter()
                .filter(|&x| self.contains(x))
                .copied()
                .collect::<Vec<T>>()
        } else {
            return vec![];
        }
    }  
}

impl HasPosition for Rect {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }
}

impl HasPosition for Vec2 {
    fn get_pos(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
}

struct Line {
    origin: Vec2,
    end: Vec2,
}

impl Line {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line { 
            origin: Vec2 { x: x1, y: y1 }, 
            end: Vec2 { x: x2, y: y2 },
         }
    }

    // Ax + Bx = C
    fn a(&self) -> f32 {
        self.end.y - self.origin.y
    }
    fn b(&self) -> f32 {
        self.end.x - self.origin.x
        // self.origin.x - self.end.x
    }
    fn c(&self) -> f32 {
        self.a() + self.b()
    }
    fn ab(&self) -> Vec2 {
        Vec2 { x: self.a(), y: self.b() }
    }

    fn ua(&self, l2: &Line) -> f32 {
        let d1: f32 = Mat2::from_cols(l2.end - l2.origin, self.origin - l2.origin).determinant();
        let d2: f32 = Mat2::from_cols(l2.end - self.origin, l2.end - l2.origin).determinant();
        d1 / d2
    }

    fn ub(&self, l2: &Line) -> f32 {
        let d1: f32 = Mat2::from_cols(self.end - self.origin, self.origin - l2.origin).determinant();
        let d2: f32 = Mat2::from_cols(self.end - self.origin, l2.end - l2.origin).determinant();
        d1 / d2
    }

    fn lineline_intersect(&self, l2: Line) -> Option<Vec2> {
        let ua = self.ua(&l2);
        let ub = self.ub(&l2);
        // lines are colliding if ua and ub are within [0, 1]
        if ua >= 0.0 && ua <= 1.0 && ub >= 0.0 && ub <= 1.0 {
            return Some(Vec2{
                x: self.origin.x + (ua * self.b()),
                y: self.origin.y + (ua * self.a())
            });
        }
        None
    }

    #[allow(unused)]
    fn linerect_intersect_points(&self, r: Rect) -> Option<Vec<Vec2>> {
        let left: Option<Vec2> = self.lineline_intersect(
            Line::new(r.pos.x, r.pos.y, r.pos.x, r.pos.y + r.size.y));
        let right: Option<Vec2> = self.lineline_intersect(
            Line::new(r.pos.x + r.size.x, r.pos.y, r.pos.x, r.pos.y + r.size.y));
        let top: Option<Vec2> = self.lineline_intersect(
            Line::new(r.pos.x, r.pos.y, r.pos.x + r.size.x, r.pos.y));
        let bottom: Option<Vec2> = self.lineline_intersect(
            Line::new(r.pos.x, r.pos.y + r.size.y, r.pos.x + r.size.x, r.pos.y + r.size.y));
        todo!();
        None
    }
}

