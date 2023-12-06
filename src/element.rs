#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Element {
    Salt = 1,
    Air,
    Fire,
    Water,
    Earth,
    Vitae,
    Mors,
    Quicksilver,
    Lead,
    Tin,
    Iron,
    Copper,
    Silver,
    Gold,
}

impl Element {
    pub(crate) fn to_index(element: Option<Element>) -> u8 {
        match element {
            Some(element) => element as u8,
            None => 0,
        }
    }

    pub(crate) fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => None,
            1 => Some(Self::Salt),
            2 => Some(Self::Air),
            3 => Some(Self::Fire),
            4 => Some(Self::Water),
            5 => Some(Self::Earth),
            6 => Some(Self::Vitae),
            7 => Some(Self::Mors),
            8 => Some(Self::Quicksilver),
            9 => Some(Self::Lead),
            10 => Some(Self::Tin),
            11 => Some(Self::Iron),
            12 => Some(Self::Copper),
            13 => Some(Self::Silver),
            14 => Some(Self::Gold),
            _ => panic!("invalid element index"),
        }
    }
}
