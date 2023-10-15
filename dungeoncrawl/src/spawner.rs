use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

/**
 * ********
 * MONSTERS
 * ********
 */
fn goblin() -> (i32, String, FontCharType) {
    (1, "Globin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

fn troll() -> (i32, String, FontCharType) {
    (3, "Troll".to_string(), to_cp437('E'))
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point,
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=6 => goblin(),
        7..=9 => orc(),
        _ => troll(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MovingRandomly {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}
