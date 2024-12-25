use std::sync::Arc;

use futures::lock::Mutex;
use tokio::sync::RwLock;

use crate::config::WEAPON_MOVES;

use super::{map::Map, player::Player};

pub struct Game {
    pub players: Arc<Mutex<Vec<Player>>>,
    pub count: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Arc::new(Mutex::new(Vec::new())),
            count: 0,
        }
    }
    pub async fn add_player(
        &self,
        id: String,
        map: &Arc<RwLock<Map>>,
    ) -> Result<(Vec<Player>, Vec<Vec<u8>>), String> {
        let play = Arc::clone(&self.players);

        let mut players = play.lock().await;

        let mut player_exist = 0;

        let pdata = players.iter().find(|x| {
            if x.id == id && x.die == false {
                player_exist = 1;
                true
            } else if x.id == id {
                player_exist = 2;
                false
            } else {
                false
            }
        });

        if players.len() > 4 {
            return Err("Game is full".to_string());
        }

        if player_exist == 0 {
            match map.write().await.assign_player() {
                Ok(cord) => {
                    players.push(Player::new(id, cord.0 .0, cord.0 .1, false, false));
                    let mut pp = players.clone();
                    pp.retain(|f| f.die == false);
                    Ok((pp, cord.1))
                }
                Err(err) => Err(err),
            }
        } else if player_exist == 1 && Option::is_some(&pdata) {
            match map
                .write()
                .await
                .player_exits(pdata.unwrap().x, pdata.unwrap().y)
            {
                Ok(cord) => {
                    players.retain(|x| x.id != id);
                    players.push(Player::new(id, cord.0 .0, cord.0 .1, false, false));
                    let mut pp = players.clone();
                    pp.retain(|f| f.die == false);
                    Ok((pp, cord.1))
                }
                Err(err) => Err(err),
            }
        } else {
            return Err("You are already dead!".to_string());
        }
    }
    pub async fn remove_player(&self, id: String, map: &Arc<RwLock<Map>>) -> Result<(), String> {
        let play = Arc::clone(&self.players);
        let mut check: bool = false;
        let (mut x_c, mut y_c): (i32, i32) = (-1, -1);
        let mut isweapon = false;

        for p in play.lock().await.iter_mut() {
            if p.id == id && p.die == true {
                check = true;
                x_c = p.x;
                y_c = p.y;
                isweapon = p.weapon;
                break;
            }
        }

        // play.lock().await.retain(|x| {
        //     if x.id == id {
        //         check = true;
        //         x_c = x.x;
        //         y_c = x.y;
        //         false
        //     } else {
        //         true
        //     }
        // });

        if check {
            map.write().await.remove_player(x_c, y_c, isweapon);
            Ok(())
        } else {
            Err("Cannot find Player to remove".to_string())
        }
    }
    pub async fn update_player(
        &mut self,
        id: String,
        ox: i32,
        oy: i32,
        nx: i32,
        ny: i32,
        map: &Arc<RwLock<Map>>,
    ) -> Result<(Vec<Vec<u8>>, bool, (bool, Option<String>)), String> {
        let play = Arc::clone(&self.players);
        let mut py = play.lock().await;
        let pc = py.clone();
        for p in py.iter_mut() {
            if p.id == id && p.die == false {
                // Check with map if that element is 0 or not, and if it is, set it to 3
                match map.write().await.update_player(
                    ox,
                    oy,
                    nx,
                    ny,
                    p,
                    if let WEAPON_MOVES = self.count {
                        true
                    } else {
                        false
                    },
                ) {
                    Ok(m) => {
                        p.x = nx;
                        p.y = ny;
                        if m.1 || p.weapon {
                            if self.count >= WEAPON_MOVES {
                                self.count = 0;
                                p.weapon = false;
                            } else {
                                self.count += 1;
                            }
                        }
                        let mut out_id: Option<String> = None;
                        for pat in pc {
                            if pat.x == nx && pat.y == ny && pat.id != p.id {
                                out_id = Some(pat.id);
                            }
                        }
                        return Ok((m.0, p.weapon, (m.1, out_id)));
                    }
                    Err(err) => return Err(err),
                }
            }
        }
        Err("Cannot find Player to update".to_string())
    }
}
