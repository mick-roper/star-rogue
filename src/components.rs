use rltk::{RGB, Point};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32, 
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub foreground: RGB,
    pub background: RGB,
}

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct ViewShed {
    pub visible_tiles : Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String
}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub current_hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl CombatStats {
    pub fn new(hp: i32, defense: i32, power: i32) -> CombatStats {
        CombatStats {
            max_hp: hp,
            current_hp: hp,
            defense: defense,
            power: power,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("unable to insert damage");
        }
    }
}