use light_and_shadow::*;

fn main() {
    // let p = Palette::build(vec![[54, 57, 64], [255, 255, 255]], 3.2);
}

// fn main() {
//     let mut colors: KdTree<f32, [u8; 3], 3> = KdTree::new();
//     let dark_bg = RGB8::from([54, 57, 64]);
//     let light_bg = RGB8::from([255, 255, 255]);

//     for r in 0..255 {
//         for g in 0..255 {
//             for b in 0..255 {
//                 let c = RGB8::from([r, g, b]);
//                 if contrast::<u8, f64>(c, dark_bg) > 3.2 && contrast::<u8, f64>(c, light_bg) > 3.2 {
//                     let c = [r, g, b];
//                     let l = Lab::from_rgb(&c);
//                     colors.add(&[l.l, l.a, l.b], c);
//                 }
//             }
//         }
//     }

//     println!("colors done");

//     while true {
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         let v = input.trim().splitn(3, ',').collect::<Vec<&str>>();

//         let color = [
//             v[0].parse::<u8>().unwrap(),
//             v[1].parse::<u8>().unwrap(),
//             v[2].parse::<u8>().unwrap(),
//         ];
//         if contrast::<u8, f64>(RGB8::from(color), dark_bg) > 3.2
//             && contrast::<u8, f64>(RGB8::from(color), light_bg) > 3.2
//         {
//             println!("already contrasts");
//         } else {
//             let lab = Lab::from_rgb(&color);
//             let nearest =
//                 colors.nearest_one(&[lab.l, lab.a, lab.b], &|lhs: &[f32; 3], rhs: &[f32; 3]| {
//                     (lhs[0] - rhs[0]).powi(2)
//                         + (lhs[1] - rhs[1]).powi(2)
//                         + (lhs[2] - rhs[2]).powi(2)
//                 });
//         }
//     }
// }
