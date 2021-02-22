use std::str::FromStr;

pub enum CreatureKind {
    Applezard,
    Appleguana,
    AppleRex,

    Melocalf,
    Melocattle,
    Melobull,

    Blowblow,
    Durowfish,
    Blowrian,

    Axolapple,
    Pineolotl,
    Pineaxolapple,

    Clameye,
    Clamscular,

    Storcot,
    Storcarrot,
    Storkoneer,

    Strawchick,
    Strawhawk,
    Stralcon,

    Banabug,
    PrayingBanantis,

    Bluebug,
    Bluecoon,
    Blueknight,

    Dragonling,
    Dragonion,
    Wygoon,

    Castork,
    Castorkoo,
    Banjot,

    Breadclops,

    Chillwhal,

    FlyingLimezard,

    Orambutan,

    SirToast,
}

impl CreatureKind {
    pub fn evolves_into(&self) -> Option<CreatureKind> {
        use CreatureKind::*;

        match self {
            Applezard => Some(Appleguana),
            Appleguana => Some(AppleRex),

            Melocalf => Some(Melocattle),
            Melocattle => Some(Melobull),

            Blowblow => Some(Durowfish),
            Durowfish => Some(Blowrian),

            Axolapple => Some(Pineolotl),
            Pineolotl => Some(Pineaxolapple),

            Clameye => Some(Clamscular),

            Storcot => Some(Storcarrot),
            Storcarrot => Some(Storkoneer),

            Strawchick => Some(Strawhawk),
            Strawhawk => Some(Stralcon),

            Banabug => Some(PrayingBanantis),

            Bluebug => Some(Bluecoon),
            Bluecoon => Some(Blueknight),

            Dragonling => Some(Dragonion),
            Dragonion => Some(Wygoon),

            Castork => Some(Castorkoo),
            Castorkoo => Some(Banjot),

            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        use CreatureKind::*;

        match self {
            Applezard => "Applezard",
            Appleguana => "Appleguana",
            AppleRex => "Apple Rex",
            Melocalf => "Melocalf",
            Melocattle => "Melocattle",
            Melobull => "Melobull",
            Blowblow => "Blowblow",
            Durowfish => "Durowfish",
            Blowrian => "Blowrian",
            Axolapple => "Axolapple",
            Pineolotl => "Pineolotl",
            Pineaxolapple => "Pineaxolapple",
            Clameye => "Clameye",
            Clamscular => "Clamscular",
            Storcot => "Storcot",
            Storcarrot => "Storcarrot",
            Storkoneer => "Storkoneer",
            Strawchick => "Strawchick",
            Strawhawk => "Strawhawk",
            Stralcon => "Stralcon",
            Banabug => "Banabug",
            PrayingBanantis => "Praying Banantis",
            Bluebug => "Bluebug",
            Bluecoon => "Bluecoon",
            Blueknight => "Blueknight",
            Dragonling => "Dragonling",
            Dragonion => "Dragonion",
            Wygoon => "Wygoon",
            Castork => "Castork",
            Castorkoo => "Castorkoo",
            Banjot => "Banjot",
            Breadclops => "Breadclops",
            Chillwhal => "Chillwhal",
            FlyingLimezard => "Flying Limezard",
            Orambutan => "Orambutan",
            SirToast => "Sir Toast",
        }
    }
}

impl FromStr for CreatureKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatureKind::*;

        match s {
            "applezard" => Ok(Applezard),
            "appleguana" => Ok(Appleguana),
            "apple_rex" => Ok(AppleRex),
            "melocalf" => Ok(Melocalf),
            "melocattle" => Ok(Melocattle),
            "melobull" => Ok(Melobull),
            "blowblow" => Ok(Blowblow),
            "durowfish" => Ok(Durowfish),
            "blowrian" => Ok(Blowrian),
            "axolapple" => Ok(Axolapple),
            "pineolotl" => Ok(Pineolotl),
            "pineaxolapple" => Ok(Pineaxolapple),
            "clameye" => Ok(Clameye),
            "clamscular" => Ok(Clamscular),
            "storcot" => Ok(Storcot),
            "storcarrot" => Ok(Storcarrot),
            "storkoneer" => Ok(Storkoneer),
            "strawchick" => Ok(Strawchick),
            "strawhawk" => Ok(Strawhawk),
            "stralcon" => Ok(Stralcon),
            "banabug" => Ok(Banabug),
            "praying_banantis" => Ok(PrayingBanantis),
            "bluebug" => Ok(Bluebug),
            "bluecoon" => Ok(Bluecoon),
            "blueknight" => Ok(Blueknight),
            "dragonling" => Ok(Dragonling),
            "dragonion" => Ok(Dragonion),
            "wygoon" => Ok(Wygoon),
            "castork" => Ok(Castork),
            "castorkoo" => Ok(Castorkoo),
            "banjot" => Ok(Banjot),
            "breadclops" => Ok(Breadclops),
            "chillwhal" => Ok(Chillwhal),
            "flying_limezard" => Ok(FlyingLimezard),
            "orambutan" => Ok(Orambutan),
            "sir_toast" => Ok(SirToast),

            _ => Err(()),
        }
    }
}
