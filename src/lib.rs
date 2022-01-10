use kiddo::KdTree;
use kasi_kule::*;

mod color_calc;

pub use color_calc::*;

fn jab_distance(lhs: &[f32; 3], rhs: &[f32; 3]) -> f32 {
    Jab::from(*lhs).squared_difference(&Jab::from(*rhs))
}

#[cfg(target = "wasm32-unknown-unknown")]
mod js;
#[cfg(target = "wasm32-unknown-unknown")]
pub use js::*;

pub struct Palette {
    min_ratio: f32,
    backgrounds: Vec<[u8; 3]>,
    colors: KdTree<f32, [u8; 3], 3>,
}

impl Palette {
    pub fn build(backgrounds: Vec<[u8; 3]>, min_ratio: f32) -> Palette {
        let background_luminances: Vec<f32> =
            backgrounds.iter().map(|c| to_luminance(*c)).collect();
        let mut colors: KdTree<f32, [u8; 3], 3> = KdTree::new();

        for r in 0..255 {
            for g in 0..255 {
                for b in 0..255 {
                    let color = [r, g, b];
                    let luminance = to_luminance(color);
                    if background_luminances
                        .iter()
                        .all(|&bg| contrast_luminance(luminance, bg) > min_ratio)
                    {
                        let jab = Jab::from(color);
                        colors.add(&[jab.J, jab.a, jab.b], color).unwrap();
                    }
                }
            }
        }

        Palette {
            min_ratio,
            backgrounds,
            colors,
        }
    }

    pub fn find_closest(&self, color: [u8; 3]) -> ([u8; 3], bool) {
        if self
            .backgrounds
            .iter()
            .all(|&bg| contrast_rgb(color, bg) > self.min_ratio)
        {
            (color, true)
        } else {
            let jab = Jab::from(color);
            (
                *self
                    .colors
                    .nearest_one(&[jab.J, jab.a, jab.b], &jab_distance)
                    .unwrap()
                    .1,
                false,
            )
        }
    }

    pub fn find_closest_n(
        &self,
        color: [u8; 3],
        n: usize,
    ) -> Vec<(f32, [u8; 3])> {
        let jab = Jab::from(color);
        
        self.colors
            .nearest(&[jab.J, jab.a, jab.b], n, &jab_distance)
            .unwrap()
            .into_iter()
            .map(|(a, b)| (a, *b))
            .collect()
    }
}
