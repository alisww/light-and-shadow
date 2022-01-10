static K_L: f32 = 1.0;
static K_ONE: f32 = 0.045;
static K_TWO: f32 = 0.015;

pub fn linear_channel(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c > 0.03928 {
        ((c + 0.055) / 1.055).powf(2.4)
    } else {
        c / 12.92
    }
}

pub fn to_luminance(c: [u8; 3]) -> f32 {
    linear_channel(c[0]) * 0.2126 + linear_channel(c[1]) * 0.7152 + linear_channel(c[2]) * 0.0722
}

pub fn contrast_luminance(lhs: f32, rhs: f32) -> f32 {
    let (lhs, rhs) = (lhs + 0.05, rhs + 0.05);
    if lhs > rhs {
        lhs / rhs
    } else {
        rhs / lhs
    }
}

pub fn contrast_rgb(lhs: [u8; 3], rhs: [u8; 3]) -> f32 {
    contrast_luminance(to_luminance(lhs), to_luminance(rhs))
}

pub fn cie76_distance(lhs: &[f32; 3], rhs: &[f32; 3]) -> f32 {
    (lhs[0] - rhs[0]).powi(2) + (lhs[1] - rhs[1]).powi(2) + (lhs[2] - rhs[2]).powi(2)
}

pub fn cie94_distance(lhs: &[f32; 3], rhs: &[f32; 3]) -> f32 {
    let delta_l = lhs[0] - rhs[0];

    let lhs_c = (lhs[1].powi(2) + lhs[2].powi(2)).sqrt();
    let rhs_c = (rhs[1].powi(2) + rhs[2].powi(2)).sqrt();

    let delta_c = lhs_c - rhs_c;
    let delta_h = (lhs[1] - rhs[1]).powi(2) + (lhs[2] - rhs[2]).powi(2) - delta_c.powi(2);

    let s_l = 1.0;
    let s_c = 1.0 + K_ONE * lhs_c;
    let s_h = 1.0 + K_TWO * lhs_c;

    (delta_l / (K_L * s_l)).powi(2)
        + (delta_c / (K_L * s_c)).powi(2)
        + (delta_h / (K_L * s_h)).powi(2)
}

