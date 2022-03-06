// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

const FULL_UP: u32 = 100;

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health <= 0 {
            Some(Player {
                health: FULL_UP,
                mana: match self.mana {
                    Some(_) => Some(FULL_UP),
                    _ => None,
                },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(current_mana) => {
                if current_mana >= mana_cost {
                    self.mana = Some(current_mana - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            _ => {
                self.health -= self.health.min(mana_cost);
                0
            }
        }
    }
}
