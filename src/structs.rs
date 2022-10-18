use bevy::prelude::Component;
use glam::{Mat2, Vec2};

pub trait HasPosition {
    fn get_pos(&self) -> Vec2;
}

/* RECT */

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

    pub fn lineline_intersect(&self, l2: &Line) -> Result<Vec2, &str> {
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
    pub fn linerect_intersect(&self, r: &Rect) -> Result<Vec<Vec2>, &str> {
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

    // Ua calc done by hand for validation
    fn ua2(&self, l2: &Line) -> f32 {
        let (x1, y1) = (self.origin.x, self.origin.y);
        let (x2, y2) = (self.end.x, self.end.y);
        let (x3, y3) = (l2.origin.x, l2.origin.y);
        let (x4, y4) = (l2.end.x, l2.end.y);

        let d1: f32 = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
            / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
        d1
    }

    fn ub(&self, l2: &Line) -> f32 {
        let d1: f32 =
            Mat2::from_cols(self.end - self.origin, self.origin - l2.origin).determinant();
        let d2: f32 = Mat2::from_cols(l2.end - l2.origin, self.end - self.origin).determinant();
        -d1 / d2
    }
}

/* Helpers */

impl HasPosition for Vec2 {
    fn get_pos(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
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
}
