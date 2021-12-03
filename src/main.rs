mod monster_data;
use monster_data::monster::Monster;

fn average_monster_scores<T>(monsters: &Vec<&Monster>, func: T) -> i32 where T: Fn(&Monster) -> i32 {
    let avg_val = monsters.iter().fold(0, |acc, m| acc + func(m)) / monsters.len() as i32;
    avg_val
}

fn get_monsters_by_cr(monsters: &Vec<Monster>, cr_min: f32, cr_max: f32) -> Vec<&Monster> {
    let monsters_by_cr = monsters.iter()
        .filter(|x| x.challenge_rating >= cr_min && x.challenge_rating <= cr_max)
        .collect::<Vec<&Monster>>();
    monsters_by_cr
}

#[derive(Debug)]
struct MonsterTier {
    min: f32,
    max: f32
}

fn main() {
    let monsters = monster_data::get_monster_data();

    let crs: Vec<f32> = vec![0.125, 0.25, 0.5, 1.0, 2.0, 3.0, 4.0, 5.0,
        6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0,
        28.0, 29.0, 30.0];

    let mut cr_tiers: Vec<MonsterTier> = vec![];

    for i in 1..=crs.len()-2 {
        cr_tiers.push(MonsterTier{min: crs[i - 1], max: crs[i + 1]});
    }

    for tier in cr_tiers {
        let tier_monsters = get_monsters_by_cr(&monsters, tier.min, tier.max);

        if tier_monsters.len() == 0 {
            continue
        }

        let avg_hp = average_monster_scores(&tier_monsters, |m| m.hit_points);
        let avg_ac = average_monster_scores(&tier_monsters, |m| m.armor_class);

        println!("CR: {} to {}, HP: {}, AC: {}", tier.min, tier.max, avg_hp, avg_ac);
    }
}