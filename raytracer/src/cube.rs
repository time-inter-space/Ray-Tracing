use crate::*;

pub struct Cube {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}
impl Cube {
    pub fn new(p0: Point3, p1: Point3, ptr: Rc<dyn Material>) -> Cube {
        let mut sides = HittableList::new();

        sides.add(Rc::new(XYRect::new(
            p0.e0,
            p1.e0,
            p0.e1,
            p1.e1,
            p1.e2,
            ptr.clone(),
        )));
        sides.add(Rc::new(XYRect::new(
            p0.e0,
            p1.e0,
            p0.e1,
            p1.e1,
            p0.e2,
            ptr.clone(),
        )));

        sides.add(Rc::new(XZRect::new(
            p0.e0,
            p1.e0,
            p0.e2,
            p1.e2,
            p1.e1,
            ptr.clone(),
        )));
        sides.add(Rc::new(XZRect::new(
            p0.e0,
            p1.e0,
            p0.e2,
            p1.e2,
            p0.e1,
            ptr.clone(),
        )));

        sides.add(Rc::new(YZRect::new(
            p0.e1,
            p1.e1,
            p0.e2,
            p1.e2,
            p1.e0,
            ptr.clone(),
        )));
        sides.add(Rc::new(YZRect::new(
            p0.e1,
            p1.e1,
            p0.e2,
            p1.e2,
            p0.e0,
            ptr.clone(),
        )));

        Cube {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let ret = Aabb::new(self.box_min, self.box_max);
        Some(ret)
    }
}
