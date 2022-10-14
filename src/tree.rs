use bevy::prelude::Component;
use glam::{Mat2, Vec2};
use std::fmt::Debug;
const CAPACITY: usize = 4;

// https://www.topcoder.com/thrive/articles/Geometry%20Concepts%20part%202:%20%20Line%20Intersection%20and%20its%20Applications

pub struct QuadTree<T>
where
    T: HasPosition + Copy + Debug,
{
    pub bounds: Rect,
    pub points: Option<Vec<T>>,
    pub inner_trees: Option<Box<[QuadTree<T>; 4]>>,
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
        if v.get_pos().x >= self.pos.x
            && v.get_pos().x < self.pos.x + self.size.x
            && v.get_pos().y >= self.pos.y
            && v.get_pos().y < self.pos.y + self.size.y
        {
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
        T: HasPosition + Copy,
    {
        if let Some(p) = points {
            return p
                .iter()
                .filter(|&x| self.contains(x))
                .copied()
                .collect::<Vec<T>>();
        }
        vec![]
    }
}

impl HasPosition for Rect {
    fn get_pos(&self) -> Vec2 {
        self.pos
    }
}

impl HasPosition for Vec2 {
    fn get_pos(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

// Mostly an internal struct used to calculate casts from A to B
pub struct Line {
    pub origin: Vec2,
    pub end: Vec2,
}

#[allow(unused)]
impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
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
        Vec2 {
            x: self.a(),
            y: self.b(),
        }
    }

    fn ua(&self, l2: &Line) -> f32 {
        let d1: f32 = Mat2::from_cols(l2.end - l2.origin, self.origin - l2.origin).determinant();
        let d2: f32 = Mat2::from_cols(l2.end - l2.origin, self.end - self.origin).determinant();
        -d1 / d2
    }

    fn ua2(&self, l2: &Line) -> f32 {
        let (x1, y1) = (self.origin.x, self.origin.y);
        let (x2, y2) = (self.end.x, self.end.y);
        let (x3, y3) = (l2.origin.x, l2.origin.y);
        let (x4, y4) = (l2.end.x, l2.end.y);

        let d1: f32 = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
            / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
        // println!("{:?}", d1);
        d1
    }

    fn ub(&self, l2: &Line) -> f32 {
        let d1: f32 =
            Mat2::from_cols(self.end - self.origin, self.origin - l2.origin).determinant();
        let d2: f32 = Mat2::from_cols(l2.end - l2.origin, self.end - self.origin).determinant();
        -d1 / d2
    }

    fn lineline_intersect(&self, l2: &Line) -> Result<Vec2, &str> {
        let ua = self.ua(l2);
        let ub = self.ub(l2);
        // println!("{:?} {:?}", ua, ub);
        // lines are colliding if ua and ub are within [0, 1]
        if (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub) {
            let intersection_point = Vec2 {
                x: self.origin.x + (ua * self.b()),
                y: self.origin.y + (ua * self.a()),
            };
            return Ok(intersection_point);
        }
        Err("No intersection.")
    }

    #[allow(unused)]
    fn linerect_intersect(&self, r: &Rect) -> Result<Vec<Vec2>, &str> {
        let left: Result<Vec2, &str> =
            self.lineline_intersect(&Line::new(r.pos.x, r.pos.y, r.pos.x, r.pos.y + r.size.y));
        let right: Result<Vec2, &str> = self.lineline_intersect(&Line::new(
            r.pos.x + r.size.x,
            r.pos.y,
            r.pos.x,
            r.pos.y + r.size.y,
        ));
        let top: Result<Vec2, &str> =
            self.lineline_intersect(&Line::new(r.pos.x, r.pos.y, r.pos.x + r.size.x, r.pos.y));
        let bottom: Result<Vec2, &str> = self.lineline_intersect(&Line::new(
            r.pos.x,
            r.pos.y + r.size.y,
            r.pos.x + r.size.x,
            r.pos.y + r.size.y,
        ));
        let collection = [left, right, top, bottom];
        if collection.iter().any(|x| x.is_ok()) {
            return Ok(collection
                .iter()
                .filter(|&&x| x.is_ok())
                .map(|&x| x.unwrap())
                .collect::<Vec<Vec2>>());
        }
        Err("No collisions.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn lineline_intersect_1() {
        let l1 = Line {
            origin: Vec2 { x: 0.0, y: 0.0 },
            end: Vec2 { x: 10.0, y: 10.0 },
        };
        let l2 = Line {
            origin: Vec2 { x: 0.0, y: 10.0 },
            end: Vec2 { x: 10.0, y: 0.0 },
        };
        assert_eq!(
            Line::lineline_intersect(&l1, &l2).unwrap(),
            Vec2 { x: 5.0, y: 5.0 }
        );
    }

    #[test]
    fn ua_test_rng() {
        let mut rng = rand::thread_rng();
        for _ in 1..100 {
            let (x1, y1) = (rng.gen_range(1.0..10.0), rng.gen_range(1.0..10.0));
            let (x2, y2) = (rng.gen_range(1.0..10.0), rng.gen_range(1.0..10.0));
            let (x3, y3) = (rng.gen_range(1.0..10.0), rng.gen_range(1.0..10.0));
            let (x4, y4) = (rng.gen_range(1.0..10.0), rng.gen_range(1.0..10.0));
            let l1: Line = Line {
                origin: Vec2 { x: x1, y: y1 },
                end: Vec2 { x: x2, y: y2 },
            };
            let l2: Line = Line {
                origin: Vec2 { x: x3, y: y3 },
                end: Vec2 { x: x4, y: y4 },
            };
            assert_eq!(Line::ua(&l1, &l2), Line::ua2(&l1, &l2))
        }
    }

    #[test]
    fn ua_test_1() {
        let l1 = Line {
            origin: Vec2 { x: 0.0, y: 0.0 },
            end: Vec2 { x: 10.0, y: 10.0 },
        };
        let l2 = Line {
            origin: Vec2 { x: 0.0, y: 10.0 },
            end: Vec2 { x: 10.0, y: 0.0 },
        };
        assert_eq!(Line::ua(&l1, &l2), Line::ua2(&l1, &l2))
    }

    #[test]
    fn lineline_intersect_2() {
        let l1 = Line {
            origin: Vec2 { x: 10.0, y: 10.0 },
            end: Vec2 { x: 20.0, y: 20.0 },
        };
        let l2 = Line {
            origin: Vec2 { x: 10.0, y: 20.0 },
            end: Vec2 { x: 20.0, y: 10.0 },
        };
        assert_eq!(
            Line::lineline_intersect(&l1, &l2).unwrap(),
            Vec2 { x: 15.0, y: 15.0 }
        );
    }

    #[test]
    fn lineline_intersect_overlap_1() {
        let l1 = Line {
            origin: Vec2 { x: 10.0, y: 10.0 },
            end: Vec2 { x: 20.0, y: 20.0 },
        };
        assert_eq!(Line::lineline_intersect(&l1, &l1), Err("No intersection."));
    }

    #[test]
    fn lineline_intersect_negative_1() {
        let l1 = Line {
            origin: Vec2 { x: -10.0, y: 10.0 },
            end: Vec2 { x: 10.0, y: -10.0 },
        };
        let l2 = Line {
            origin: Vec2 { x: -15.0, y: -15.0 },
            end: Vec2 { x: 10.0, y: 15.0 },
        };
        assert_eq!(
            Line::lineline_intersect(&l1, &l2).unwrap(),
            Vec2 {
                x: -1.363636,
                y: 1.363636
            }
        );
    }

    #[test]
    fn rect_contains_1() {
        let p = Vec2 {
            x: 640.0831,
            y: 292.49387,
        };
        let r = Rect {
            pos: Vec2 { x: 400.0, y: 400.0 },
            size: Vec2 { x: 400.0, y: 200.0 },
        };
        assert!(r.contains(&p));
    }

    // #[test]
    // fn linerect_intersect_1() {
    //     let l: Line = Line {
    //         origin: Vec2 { x: 0.0, y: 0.0 },
    //         end: Vec2 { x: 10.0, y: 10.0 },
    //     };
    //     let r: Rect = Rect {
    //         pos: Vec2 { x: 4.1, y: 6.1 },
    //         size: Vec2 { x: 2.0, y: 2.0 },
    //     };
    //     assert_ne!(Line::linerect_intersect(&l, &r).unwrap(), RectIntersect {});
    // }
}
