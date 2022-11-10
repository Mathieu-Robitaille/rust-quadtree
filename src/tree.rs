use std::fmt::Debug;
use glam::Vec2;
use bevy::prelude::Component;

use crate::structs::{HasPosition, Line, Rect};

const CAPACITY: usize = 4;

// https://www.topcoder.com/thrive/articles/Geometry%20Concepts%20part%202:%20%20Line%20Intersection%20and%20its%20Applications

#[derive(Component)]
pub struct QuadTree<T>
where
    T: HasPosition + Copy + Debug,
{
    pub bounds: Rect,
    pub inner_trees: Option<Box<[QuadTree<T>; 4]>>,
    pub points: Option<Vec<T>>,
}

impl<T> QuadTree<T>
where
    T: HasPosition + Copy + Debug,
{
    #[allow(unused)]
    pub fn new_empty(r: Rect) -> QuadTree<T> {
        QuadTree {
            bounds: r,
            points: None,
            inner_trees: None,
        }
    }

    #[allow(unused)]
    pub fn from(i: Vec<T>, r: Rect) -> QuadTree<T> {
        let mut q: QuadTree<T> = QuadTree {
            bounds: r,
            points: None,
            inner_trees: None,
        };
        for obj in i {
            q.insert(obj);
        }
        q
    }

    // Attempt to insert a point into the quadtree structure
    // return false if it cannot be inserted for any reason
    #[allow(unused)]
    pub fn insert(&mut self, p: T) -> Result<(), &'static str> {
        if !self.bounds.contains(&p) {
            return Err("The point does not exist in the bounds");
        }
        match (self.points.as_mut(), self.inner_trees.as_mut()) {
            // this tree itself contains points
            (Some(val), None) => {
                if val.len() == CAPACITY - 1 || val.len() >= CAPACITY {
                    val.push(p);
                    self.subdivide();
                    Ok(())
                } else {
                    val.push(p);
                    Ok(())
                }
            }
            // This tree contains subtrees but no points
            (None, Some(val)) => {
                let x = &mut **val;
                if x.iter_mut().any(|x| x.insert(p).is_ok()) {
                    return Ok(());
                }
                println!("Point: {:?}", p);
                for y in x.iter() {
                    println!("{:?}", y.bounds);
                }
                Err("This point doesnt belong in any subtrees.")
            }
            // This tree has nothing cool about it
            (None, None) => {
                self.points = Some(vec![p]);
                Ok(())
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
                let mut x: Vec<Vec2> = val.iter().map(|x| x.get_pos()).collect();
                res.append(&mut x);
            }

            // This tree contains subtrees but no points
            (None, Some(val)) => {
                let x = &**val;
                for i in x {
                    let mut cont = i.get_positions();
                    res.append(&mut cont);
                }
            }
            _ => (),
        }
        res
    }

    #[allow(unused)]
    pub fn get_bounds(&self) -> Vec<Rect> {
        let mut result: Vec<Rect> = vec![];
        if let Some(boxed_inner_trees) = &self.inner_trees {
            let inner_trees = &**boxed_inner_trees;
            for x in inner_trees.iter() {
                // For the bounds we intersect recur this function
                result.append(&mut QuadTree::get_bounds(x));
            }
        } else if let Some(internal_bounds) = &self.points {
            result.push(self.bounds);
        }
        result
    }

    // Get the quadtree leaves that a line intersects with
    #[allow(unused)]
    pub fn get_intersecting_bounds(&self, l: &Line) -> Vec<&QuadTree<T>> {
        let mut res: Vec<&QuadTree<T>> = vec![];

        // Unbox the trees so we can use them
        if let Some(boxed_inner_trees) = &self.inner_trees {
            let inner_trees = &**boxed_inner_trees;
            for x in inner_trees.iter() {
                // For the bounds we intersect recur this function
                if l.linerect_intersect(&x.bounds).is_ok() {
                    res.append(&mut QuadTree::get_intersecting_bounds(x, l));
                }
            }
        }
        // this should return the lowest level leaf colliding with the line
        else if l.linerect_intersect(&self.bounds).is_ok() {
            res.push(self)
        }
        res
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
            (Some(val), None) => {}

            // This tree contains subtrees but no points
            (None, Some(val)) => {}
            _ => (),
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
            pos: Vec2 {
                x: self.bounds.pos.x + half_w,
                y: self.bounds.pos.y,
            },
            size: Vec2 {
                x: half_w,
                y: half_h,
            },
        };
        let r2 = Rect {
            pos: Vec2 {
                x: self.bounds.pos.x,
                y: self.bounds.pos.y,
            },
            size: Vec2 {
                x: half_w,
                y: half_h,
            },
        };
        let r3 = Rect {
            pos: Vec2 {
                x: self.bounds.pos.x,
                y: self.bounds.pos.y + half_h,
            },
            size: Vec2 {
                x: half_w,
                y: half_h,
            },
        };
        let r4 = Rect {
            pos: Vec2 {
                x: self.bounds.pos.x + half_w,
                y: self.bounds.pos.y + half_h,
            },
            size: Vec2 {
                x: half_w,
                y: half_h,
            },
        };

        let ret: [QuadTree<T>; 4] = [
            QuadTree {
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
            },
        ];

        self.inner_trees = Some(Box::new(ret));
        self.points = None;
    }
}
