use kiddo::KdTree;
use lab::Lab;
use serde::{Deserialize, Serialize};

mod color_calc;

pub use color_calc::*;

#[cfg(target = "wasm32-unknown-unknown")]
mod js;
#[cfg(target = "wasm32-unknown-unknown")]
pub use js::*;

#[derive(Serialize, Deserialize)]
pub enum ColorDistance {
    CIE76,
    CIE94,
    CIEDE2000,
}

#[derive(Serialize, Deserialize)]
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
                        let lab = Lab::from_rgb(&color);
                        colors.add(&[lab.l, lab.a, lab.b], color).unwrap();
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

    pub fn find_closest(&self, color: [u8; 3], distance_method: ColorDistance) -> ([u8; 3], bool) {
        if self
            .backgrounds
            .iter()
            .all(|&bg| contrast_rgb(color, bg) > self.min_ratio)
        {
            (color, true)
        } else {
            let lab = Lab::from_rgb(&color);
            let dist = &match distance_method {
                ColorDistance::CIE76 => cie76_distance,
                ColorDistance::CIE94 => cie94_distance,
                ColorDistance::CIEDE2000 => {
                    |lhs: &[f32; 3], rhs: &[f32; 3]| empfindung::cie00::diff(lhs, rhs)
                }
            };

            (
                *self
                    .colors
                    .nearest_one(&[lab.l, lab.a, lab.b], dist)
                    .unwrap()
                    .1,
                false,
            )
        }
    }

    pub fn find_closest_n(
        &self,
        color: [u8; 3],
        distance_method: ColorDistance,
        n: usize,
    ) -> Vec<(f32, [u8; 3])> {
        let lab = Lab::from_rgb(&color);
        let dist = &match distance_method {
            ColorDistance::CIE76 => cie76_distance,
            ColorDistance::CIE94 => cie94_distance,
            ColorDistance::CIEDE2000 => {
                |lhs: &[f32; 3], rhs: &[f32; 3]| empfindung::cie00::diff(lhs, rhs)
            }
        };

        self.colors
            .nearest(&[lab.l, lab.a, lab.b], n, dist)
            .unwrap()
            .into_iter()
            .map(|(a, b)| (a, *b))
            .collect()
    }
}
