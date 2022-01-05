use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsPalette {
    internal: Palette,
}

#[wasm_bindgen]
impl JsPalette {
    #[wasm_bindgen(constructor)]
    pub fn build(backgrounds: &JsValue, min_ratio: f32) -> JsPalette {
        let backgrounds: Vec<[u8; 3]> = backgrounds.into_serde().unwrap();
        JsPalette {
            internal: Palette::build(backgrounds, min_ratio),
        }
    }

    pub fn nearest(&self, color: &JsValue, distance_method: &str) -> JsValue {
        let color: [u8; 3] = color.into_serde().unwrap();
        let distance_method = match distance_method {
            "cie94" => ColorDistance::CIE94,
            "cie76" => ColorDistance::CIE76,
            _ => panic!("invalid color distance"),
        };

        let (closest, _) = self.internal.find_closest(color, distance_method);
        JsValue::from_serde(&closest).unwrap()
    }

    pub fn nearest_colors(&self, n: usize, color: &JsValue, distance_method: &str) -> JsValue {
        let color: [u8; 3] = color.into_serde().unwrap();
        let distance_method = match distance_method {
            "cie94" => ColorDistance::CIE94,
            "cie76" => ColorDistance::CIE76,
            _ => panic!("invalid color distance"),
        };

        let closest = self.internal.find_closest_n(color, distance_method, n);
        JsValue::from_serde(&closest).unwrap()
    }
}
