#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub size: String,
    pub mtype: String,
    pub alignment: String,
    pub armor_class: i32,
    pub hit_points: i32,

    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,

    pub challenge_rating: f32,
}

pub fn new_from_drive(row: &std::vec::Vec<String>) -> Monster {
    Monster{
        name: row[0].to_string(),
        size: row[1].to_string(),
        mtype: row[2].to_string(),
        alignment: row[3].to_string(),
        armor_class: row[4].parse::<i32>().unwrap(),
        hit_points: row[5].parse::<i32>().unwrap(),
        strength: row[7].parse::<i32>().unwrap(),
        dexterity: row[8].parse::<i32>().unwrap(),
        constitution: row[9].parse::<i32>().unwrap(),
        intelligence: row[10].parse::<i32>().unwrap(),
        wisdom: row[11].parse::<i32>().unwrap(),
        charisma: row[12].parse::<i32>().unwrap(),
        challenge_rating: row[18].parse::<f32>().unwrap(),
    }
}