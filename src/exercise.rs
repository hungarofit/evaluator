#![allow(dead_code)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

macro_rules! define_exercises {
    (
        $($variant:ident {
            name: $name:literal,
            aerob: $is_aerob:literal,
            motor4: $is_motor4:literal,
            motor6: $is_motor6:literal,
            lower_better: $is_lower_better:literal,
        }),+ $(,)?
    ) => {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
        #[non_exhaustive]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
        pub enum Exercise {
            $($variant),+
        }

        impl Exercise {
            pub const ALL: &'static [Exercise] = &[$(Exercise::$variant),+];

            pub const fn table_name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $name),+
                }
            }

            pub const fn is_lower_better(&self) -> bool {
                match self {
                    $(Self::$variant => $is_lower_better),+
                }
            }

            pub const fn is_aerob(&self) -> bool {
                match self {
                    $(Self::$variant => $is_aerob),+
                }
            }

            pub const fn is_motor4(&self) -> bool {
                match self {
                    $(Self::$variant => $is_motor4),+
                }
            }

            pub const fn is_motor6(&self) -> bool {
                match self {
                    $(Self::$variant => $is_motor6),+
                }
            }

            pub const fn index(&self) -> usize {
                *self as usize
            }
        }

        impl std::fmt::Display for Exercise {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, stringify!($variant))),+
                }
            }
        }

        pub fn is_lower_better(e: Exercise) -> bool {
            e.is_lower_better()
        }

        pub static LIST: &[(&str, Exercise)] = &[$(($name, Exercise::$variant)),+];
        
        #[allow(dead_code)]
        pub fn from_string(s: &str) -> Option<Exercise> {
            LIST.iter()
                .find(|(name, _)| *name == s)
                .map(|(_, exercise)| *exercise)
        }
    };
}

define_exercises! {
    Motor4Jump {
        name: "motor4-jump",
        aerob: false,
        motor4: true,
        motor6: false,
        lower_better: false,
    },
    Motor4Pushup {
        name: "motor4-pushup",
        aerob: false,
        motor4: true,
        motor6: false,
        lower_better: false,
    },
    Motor4Situp {
        name: "motor4-situp",
        aerob: false,
        motor4: true,
        motor6: false,
        lower_better: false,
    },
    Motor4Torso {
        name: "motor4-torso",
        aerob: false,
        motor4: true,
        motor6: false,
        lower_better: false,
    },

    Motor6Jump {
        name: "motor6-jump",
        aerob: false,
        motor4: false,
        motor6: true,
        lower_better: false,
    },
    Motor6Pushup {
        name: "motor6-pushup",
        aerob: false,
        motor4: false,
        motor6: true,
        lower_better: false,
    },
    Motor6Situp {
        name: "motor6-situp",
        aerob: false,
        motor4: false,
        motor6: true,
        lower_better: false,
    },
    Motor6Torso {
        name: "motor6-torso",
        aerob: false,
        motor4: false,
        motor6: true,
        lower_better: false,
    },
    Motor6ThrowDouble {
        name: "motor6-throwdouble",
        aerob: false,
        motor4: false,
        motor6: true,
        lower_better: false,
    },
    Motor6ThrowSingle {
        name: "motor6-throwsingle",
        aerob: false,
        motor4: false,
        motor6: true,
        lower_better: false,
    },

    AerobBike12Min {
        name: "aerob-bike-12min",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: false,
    },
    AerobRun12Min {
        name: "aerob-run-12min",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: false,
    },
    AerobRun1Mile {
        name: "aerob-run-1mile",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: true,
    },
    AerobRun1Mile5 {
        name: "aerob-run-1mile5",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: true,
    },
    AerobRun2Km {
        name: "aerob-run-2km",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: true,
    },
    AerobRun2Mile {
        name: "aerob-run-2mile",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: true,
    },
    AerobRun3Km {
        name: "aerob-run-3km",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: true,
    },
    AerobRun6Min {
        name: "aerob-run-6min",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: false,
    },
    AerobSwim12Min {
        name: "aerob-swim-12min",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: false,
    },
    AerobSwim500M {
        name: "aerob-swim-500m",
        aerob: true,
        motor4: false,
        motor6: false,
        lower_better: true,
    },
}

// WASM-specific helper functions
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "aerobExerciseList")]
pub fn aerob_exercise_list() -> Vec<String> {
    Exercise::ALL
        .iter()
        .filter(|e| e.is_aerob())
        .map(|e| e.table_name().to_string())
        .collect()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "exerciseFromString")]
pub fn exercise_from_string(s: &str) -> Result<Exercise, JsError> {
    from_string(s).ok_or_else(|| JsError::new(&format!("Unknown exercise: {}", s)))
}
