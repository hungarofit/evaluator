#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub enum Gender {
    Male,
    Female,
}

// Separate impl block for methods not exposed to wasm_bindgen directly
impl Gender {
    #[allow(dead_code)]
    const ALL: [Gender; 2] = [Gender::Female, Gender::Male];

    #[inline]
    pub const fn sheet_index(&self) -> usize {
        match self {
            Self::Female => 0,
            Self::Male => 1,
        }
    }
}

impl Gender {
    #[allow(dead_code)]
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "male" => Some(Self::Male),
            "female" => Some(Self::Female),
            _ => None,
        }
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = "genderList")]
pub fn gender_list() -> Vec<String> {
    Gender::ALL
        .iter()
        .map(|g| g.to_string().to_lowercase())
        .collect()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = "genderFromString")]
pub fn gender_from_string(s: &str) -> Result<Gender, wasm_bindgen::JsError> {
    Gender::from_string(s).ok_or_else(|| wasm_bindgen::JsError::new(&format!("Unknown gender: {}", s)))
}
