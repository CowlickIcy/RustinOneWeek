use super::*;

pub struct RotateY {
    pub hptr: Box<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub rbox: Aabb,
}

// impl RotatyY {
//     pub fn new(hptr: Box<dyn Hittable>, angle: f64) -> RotatyY {
//         let radians = degress_to_radians(angle);
//         let curr_ab = Aabb::new()
//         let hasbox = hptr.bounding_box(0, 1, output_box)

//         // Cal aabb box
//         let min = Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
//         let max = Point::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

//         for i in 0..=2 {
//             for j in 0..=2 {
//                 for k in 0..=2 {
//                     let x = i * rbox
//                 }
//             }
//         }
//     }
// }
