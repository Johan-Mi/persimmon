pub(crate) enum CreatureKind {
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
    fn evolves_into(&self) -> Option<CreatureKind> {
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
}
