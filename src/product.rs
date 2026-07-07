#[derive(Copy, Clone)]
pub enum Product {
    MapleStory,
    Mabinogi,
    Vindictus,
    MapleStoryM,
}

impl Product {
    pub fn id(self) -> i32 {
        match self {
            Self::MapleStory => 10100,
            Self::Mabinogi => 10200,
            Self::Vindictus => 10300,
            // _ => 59749,
            // _ => 59750,
            // _ => 59759,
            Self::MapleStoryM => 59789,
            // _ => 59794,
        }
    }
}