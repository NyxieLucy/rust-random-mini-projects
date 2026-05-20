use std::{thread, time};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crossterm::event::{self,Event,KeyCode};

//we need a player module
mod shortcuts;
use shortcuts::*;
mod detailsBoutCharAndWeapons;
use detailsBoutCharAndWeapons::*;
mod player;
use player::*;
mod gamestart;
use gamestart::*;
fn main() {
    let mut character:Vec<Player> = Vec::new();
    let mut weapons:Vec<Weapon> = Vec::new();
    let start = input("do ya wanna start the game?(y/n): ").unwrap_or("0".to_string());
    if start.trim() == "y" {
    let player = character_create();
    character.push(player);
    println!("alright solid hehe, now for the weapon!");
    let player_weapon= weapon(&character[0]);
    println!("dayuuum you have a {:?} with {:?} damage", player_weapon.tool, player_weapon.damage);
    weapons.push(player_weapon);
    println!("okay so as a recape your name is {}, your age is {} year old, and you're a {:?} and you're also a {:?}, personality treats: {}", &character[0].name, &character[0].age, &character[0].race_str, &character[0].speciality, &character[0].personality);
    let processing = time::Duration::from_secs(3);
    thread::sleep(processing);
    }
    else if start.trim() =="n" {

        println!("fuck you man, why bother opening then damn!");
        let delay = time::Duration::from_secs(2);
        thread::sleep(delay);
        println!("matter of fact, i aint letting that slide, time for punishment");
        thread::sleep(delay);
        println!("brace for impact twin");
        for _ in 0 .. 3 {
            println!(".");
            thread::sleep(delay);
        }
        let mut miku = 0;
        while miku < 10000000 {
            println!("miku miku beeeeeeeeeeeeeeeeeeeeeeeeeaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaam");
            miku += 1;
        }


    }
    else {
        println!("man is u stupid?");
    }
}
