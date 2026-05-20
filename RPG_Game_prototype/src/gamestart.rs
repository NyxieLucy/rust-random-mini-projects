
use crate::{detailsBoutCharAndWeapons::{Race, Speciality, WeaponsEn}, player::{self, Player, Weapon}, shortcuts::input};

pub fn character_create() -> Player {
    let name = input("enter your character's name: ").unwrap_or("unknown".to_string());
    //------name done--------//
    let race_str = input("Enter your character race \n 1.Human | 2.Dwarf | 3.Elf | 4.Demon | 5. Half-Human | 6.souls | 7.Oger (prolly u irl): ").unwrap_or("unknown".to_string());
    let race:u32 = race_str.parse().unwrap_or(0);
    let race_str = match race {
        1 => Race::Human,
        2 => Race::Dwarf,
        3 => Race::Elf,
        4 => Race::Demon,
        5 => Race::HalfHuman,
        6 => Race::Soul,
        7 => Race::Ogre,
        _ => Race::Human,
    };
    //------done with the race i guess? idk i'll see where the wind takes me ---------//
    let age_str = input("please insert your character's age: ").unwrap_or("unvalid".to_string());
    let age:u32 = age_str.parse().unwrap_or(0);
    //------done with the age--------//
    let speciality_str = input("your speciality: \n 1.Swordsman | 2.Mage | 3.Catalyst | 4.Archer | 5.Enchanter | 6.Necromancer | 7.Else: ").unwrap_or("0".to_string());

    let speciality = match speciality_str.parse().unwrap_or(0){
        1 => Speciality::Swordsman,
        2 => Speciality::Mage,
        3 => Speciality::Catalyst,
        4 => Speciality::Archer,
        5 => Speciality::Enchanter,
        6 => Speciality::Necromancer,
        _ => Speciality::Swordsman
    };
    let personality = input("personality treats?: ").unwrap_or("0".to_string());

    Player { name, race_str, age, speciality, personality }
    //oh boy that'll be so fun to tweak, anyways i'm done with the speciality i guess for now//
}

pub fn weapon(player:&Player) -> Weapon {
    let weeapon = input("what's your weapon of choice? (the default one u can choose another one later) : \n 1.swords | 2.bows | 3.artifacts | 4.staffs | 5.words | 6.the pen(pen stronger than the sword lol) | 7.Other idk:\n ").unwrap_or("prob".to_string());
    let damege_default_str:f32 = 5.0;
    let weight_str = input("how much do you think it weights?: ").unwrap_or("0".to_string());
    let weight:f32 = weight_str.parse().unwrap_or(0.0);
    let weapon_num:u32 = weeapon.parse().unwrap_or(0);
    let weapon = match weapon_num {
        1 => WeaponsEn::Swords,
        2 => WeaponsEn::Bows,
        3 => WeaponsEn::Artifacts,
        4 => WeaponsEn::Staffs,
        5 => WeaponsEn::Words,
        6 => WeaponsEn::The_pen,
        _ => WeaponsEn::Swords
    };
    let buff = (player.speciality as u32) as f32 * 1.5;
    Weapon {
        tool:weapon, damage:damege_default_str,  weight, buff_with_char:buff
    }
}