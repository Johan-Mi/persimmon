use crate::{
    kind::CreatureKind,
    types::{Exp, Health},
};

pub struct Creature {
    kind: CreatureKind,
    health: Health,
    exp: Exp,
}
