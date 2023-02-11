use crate::{player::Player, enemy::{Enemies, Enemy}, shot::ExplosionStatus};

pub struct GameLogic {
}

pub enum GameState {
    Progress,
    Win,
    Lose
}

impl GameLogic {
    pub fn new() -> Self {Self{}}
    pub fn check_game_state(
        self: & Self,
        player: &mut Player,
        enemies: &mut Enemies
    ) -> GameState {

        let mut list_of_enemy_indices_to_despawn: Vec<usize> = Vec::new();

        if enemies.enemy_list.is_empty() {
            return GameState::Win;
        }

        for e in enemies.enemy_list.iter() {
            if e.x == player.x && e.y == player.y {
                return GameState::Lose;
            }

            for shot in player.shots.iter_mut() {
                if 
                    (
                        (e.x == shot.x) 
                        && (e.y == shot.y)
                    ) 
                    || 
                    (
                        shot.particle_effects_coords.clone().iter().filter(
                            |&&p| {
                                e.x == p.0 && e.y == p.1
                            }
                        ).collect::<Vec<&(usize, usize)>>().len() > 0
                    ) {

                    list_of_enemy_indices_to_despawn.push(e.i);
                    
                    if matches!(shot.explosion_status, ExplosionStatus::None) {
                        shot.explode();
                    }
                }
            }
        }


        log::info!("fff {:?}", list_of_enemy_indices_to_despawn);

        for i in list_of_enemy_indices_to_despawn {
            enemies.get_shot(
                i
            );
        }

        GameState::Progress
    }
}


