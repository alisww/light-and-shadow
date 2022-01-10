use light_and_shadow::*;

fn main() {
    let p = Palette::build(vec![[54, 57, 64], [255, 255, 255]], 3.0);
    println!("{:?}",p.find_closest([0,0,255]));
    // println!("{}", p.colors.size());
}
