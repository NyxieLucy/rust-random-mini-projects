use crate::detailsBoutCharAndWeapons::{Race, WeaponsEn, Speciality};
#[derive(Debug)]
pub struct Player{
    pub name: String,
    pub race_str: Race,
    pub age: u32,
    pub speciality: Speciality,
    pub personality: String,
}
//for now imma keep stuff simple before diving into the deep end

//now for the weapon of choice, i was going to add a new struct for that but i remembered that i literally have a speciality, but would't be fun to watch a mage swinging a sword lmao so yeah fuck it that's my game shush

pub struct Weapon {
    pub tool : WeaponsEn,
    pub damage : f32,
    pub weight : f32,
    pub buff_with_char : f32,
}