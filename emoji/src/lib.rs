#[derive(Debug, PartialEq)]
pub enum Status {
    Component,
    FullyQualified,
    MinimallyQualified,
    Unqualified,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Status::*;
        write!(
            f,
            "{}",
            match self {
                Component => "Component",
                FullyQualified => "FullyQualified",
                MinimallyQualified => "MinimallyQualified",
                Unqualified => "Unqualified",
            }
        )
    }
}
#[derive(Debug, PartialEq)]
pub struct Emoji {
    pub codepoint: &'static str,
    pub status: Status,
    pub glyph: &'static str,
    pub introduction_version: f32,
    pub name: &'static str,
    pub group: &'static str,
    pub subgroup: &'static str,
    pub variants: &'static [Emoji],
    pub annotations: &'static [Annotation],
    pub is_variant: bool,
}
#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub lang: &'static str,
    pub tts: Option<&'static str>,
    pub keywords: &'static [&'static str],
}
#[doc = " \" 🦀\" goes in, `emoji::food_and_drink::food_marine::CRAB` goes out  "]
#[doc = " Also defines several other utility functions"]
pub mod lookup_by_glyph;
#[doc = " Search for an emoji::Emoji by name. Yields exact matches only, but is extremely fast"]
pub mod lookup_by_name;
#[doc = " Fuzzy search algorithms for general purpose"]
pub mod search;
#[doc = r" All annotation languages"]
pub const ANNOTATION_LANGS_TOTAL: &'static [&'static str] = &["en", "fi"];
#[doc = r" Enabled annotation languages (feature dependent)  "]
#[doc = r#" Defaults to `["en"]`. Enable the `XX` features for each language to include annotations for another language. For example, to include Finnish annotations, use the `fi` feature."#]
pub const ANNOTATION_LANGS: &'static [&'static str] = &[
    #[cfg(feature = "en")]
    "en",
    #[cfg(feature = "fi")]
    "fi",
];
#[doc = r" The unicode release version that this crate is compiled against"]
pub const UNICODE_VERSION: f32 = 13.1f32;
#[doc = r" The rfc3339 formatted time of the unicode release that this crate is compiled against"]
pub const UNICODE_RELEASE_TIME: &'static str = "2020-08-28T05:24:13+00:00";
pub mod smileys_and_emotion {
    pub mod cat_face;
    pub mod emotion;
    pub mod face_affection;
    pub mod face_concerned;
    pub mod face_costume;
    pub mod face_glasses;
    pub mod face_hand;
    pub mod face_hat;
    pub mod face_negative;
    pub mod face_neutral_skeptical;
    pub mod face_sleepy;
    pub mod face_smiling;
    pub mod face_tongue;
    pub mod face_unwell;
    pub mod monkey_face;
}
pub mod people_and_body {
    pub mod body_parts;
    pub mod family;
    pub mod hand_fingers_closed;
    pub mod hand_fingers_open;
    pub mod hand_fingers_partial;
    pub mod hand_prop;
    pub mod hand_single_finger;
    pub mod hands;
    pub mod person;
    pub mod person_activity;
    pub mod person_fantasy;
    pub mod person_gesture;
    pub mod person_resting;
    pub mod person_role;
    pub mod person_sport;
    pub mod person_symbol;
}
pub mod component {
    pub mod hair_style;
    pub mod skin_tone;
}
pub mod animals_and_nature {
    pub mod animal_amphibian;
    pub mod animal_bird;
    pub mod animal_bug;
    pub mod animal_mammal;
    pub mod animal_marine;
    pub mod animal_reptile;
    pub mod plant_flower;
    pub mod plant_other;
}
pub mod food_and_drink {
    pub mod dishware;
    pub mod drink;
    pub mod food_asian;
    pub mod food_fruit;
    pub mod food_marine;
    pub mod food_prepared;
    pub mod food_sweet;
    pub mod food_vegetable;
}
pub mod travel_and_places {
    pub mod hotel;
    pub mod place_building;
    pub mod place_geographic;
    pub mod place_map;
    pub mod place_other;
    pub mod place_religious;
    pub mod sky_and_weather;
    pub mod time;
    pub mod transport_air;
    pub mod transport_ground;
    pub mod transport_water;
}
pub mod activities {
    pub mod arts_and_crafts;
    pub mod award_medal;
    pub mod event;
    pub mod game;
    pub mod sport;
}
pub mod objects {
    pub mod book_paper;
    pub mod clothing;
    pub mod computer;
    pub mod household;
    pub mod light_and_video;
    pub mod lock;
    pub mod mail;
    pub mod medical;
    pub mod money;
    pub mod music;
    pub mod musical_instrument;
    pub mod office;
    pub mod other_object;
    pub mod phone;
    pub mod science;
    pub mod sound;
    pub mod tool;
    pub mod writing;
}
pub mod symbols {
    pub mod alphanum;
    pub mod arrow;
    pub mod av_symbol;
    pub mod currency;
    pub mod gender;
    pub mod geometric;
    pub mod keycap;
    pub mod math;
    pub mod other_symbol;
    pub mod punctuation;
    pub mod religion;
    pub mod transport_sign;
    pub mod warning;
    pub mod zodiac;
}
pub mod flags {
    pub mod country_flag;
    pub mod flag;
    pub mod subdivision_flag;
}
