use crate::prelude::*;

//START: spawn_player
pub fn spawn_player(ecs : &mut World, pos : Point) {
    ecs.push(
        (Player,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('@')
            },
            Health{ current: 10, max: 10 },
            //START_HIGHLIGHT
            FieldOfView::new(8)
            //END_HIGHLIGHT
        )
    );
}
//END: spawn_player

pub fn spawn_monster(
    ecs: &mut World, 
    rng: &mut RandomNumberGenerator, 
    pos : Point
) {
    let (hp, name, glyph) = match rng.roll_dice(1,10) {
        1..=8 => goblin(),
        _ => orc()
    };

    //START: spawn_mob
    ecs.push(
        (Enemy,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            ChasingPlayer{},
            Health{current: hp, max: hp},
            Name(name),
            FieldOfView::new(6)
        )
    );
    //END: spawn_mob
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

//START: amulet
pub fn spawn_amulet_of_yala(ecs : &mut World, pos : Point) {
    ecs.push(
        (Item, AmuletOfYala,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('|')
            },
            Name("Amulet of Yala".to_string())
        )
    );
}
//END: amulet