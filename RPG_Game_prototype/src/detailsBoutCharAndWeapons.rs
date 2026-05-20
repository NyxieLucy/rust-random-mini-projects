#[derive(Debug)]
pub enum Race {
    Human,
    Dwarf,
    Elf,
    Demon,
    HalfHuman,
    Soul,
    Ogre,

}
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Speciality {
    Swordsman = 1 ,
    Mage= 2,
    Catalyst = 3,
    Archer = 4,
    Enchanter = 5,
    Necromancer= 6,
}
#[derive(Debug)]
pub enum WeaponsEn {
    Swords = 1,
    Bows = 2,
    Artifacts = 3,
    Staffs = 4,
    Words = 5,
    The_pen = 6,

}
