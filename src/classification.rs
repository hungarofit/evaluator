#[derive(Copy, Clone)]
#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub enum Classification {
    Concerning,
    Weak,
    Mediocre,
    Average,
    Good,
    Excellent,
    Outstanding,
}

impl Classification {
    pub fn from_score(score: f32) -> Self {
        let score_x2 = (score * 2.0) as u32;
        match score_x2 {
            0..=40 => Self::Concerning,
            41..=80 => Self::Weak,
            81..=120 => Self::Mediocre,
            121..=160 => Self::Average,
            161..=200 => Self::Good,
            201..=240 => Self::Excellent,
            _ => Self::Outstanding,
        }
    }

    #[allow(dead_code)]
    pub fn score_range(&self) -> (f32, f32) {
        match self {
            Self::Concerning => (0.0, 20.499),
            Self::Weak => (20.5, 40.499),
            Self::Mediocre => (40.5, 60.499),
            Self::Average => (60.5, 80.499),
            Self::Good => (80.5, 100.499),
            Self::Excellent => (100.5, 120.499),
            Self::Outstanding => (120.5, 140.0),
        }
    }
}
