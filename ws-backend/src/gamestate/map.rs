use crate::config::{weapon_location, HEIGHT, LENGTH, VISIBILITY, WEAPON_VISIBILITY, WIDTH};

use super::player::Player;

pub struct Map {
    pub map: Vec<Vec<u8>>,
}

pub fn helper_weapon(map: &mut Vec<Vec<u8>>) {
    let (x, y) = weapon_location();
    map[y as usize][x as usize] = 2;
}

impl Map {
    pub fn new() -> Map {
        let mut map: Vec<Vec<u8>> = vec![
            vec![1; WIDTH as usize],
            vec![
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
            ],
            vec![
                1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
                1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1,
                0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1,
                0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1,
            ],
            vec![
                1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1,
                0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1,
            ],
            vec![
                1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1,
                1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1,
                0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1,
                0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1,
            ],
            vec![
                1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![1; WIDTH as usize],
        ];
        helper_weapon(&mut map);
        Map { map }
    }

    pub fn player_exits(&mut self, xx: i32, yy: i32) -> Result<((i32, i32), Vec<Vec<u8>>), String> {
        // just check the first column, if there is any 0, return that coordinate and set it to 3
        let x = xx / LENGTH as i32;
        let y = yy / LENGTH as i32;

        if self.map[y as usize][x as usize] == 3 {
            let rmap = self.get_player_map(x as i32, y as i32, false);
            return Ok(((x * (LENGTH as i32), (y * LENGTH as i32) as i32), rmap));
        }
        Err("No free space".to_string())
    }

    pub fn assign_player(&mut self) -> Result<((i32, i32), Vec<Vec<u8>>), String> {
        // just check the first column, if there is any 0, return that coordinate and set it to 3

        for y in 0..HEIGHT {
            if self.map[y as usize][1] == 0 {
                self.map[y as usize][1] = 3;
                let rmap = self.get_player_map(1, y as i32, false);
                return Ok(((1 * (LENGTH as i32), (y * LENGTH) as i32), rmap));
            }
        }
        Err("No free space".to_string())
    }

    pub fn update_player(
        &mut self,
        ox: i32,
        oy: i32,
        nx: i32,
        ny: i32,
        p: &mut Player,
        weapon_reset: bool,
    ) -> Result<(Vec<Vec<u8>>, bool), String> {
        // check if the new coordinates is 0, if yes then change old x and y on map to new x and y on map; if no then return error. Also check whether distance between new and old x and y is 1 of not.

        let ox = ox / LENGTH as i32;
        let nx = nx / LENGTH as i32;
        let ny = ny / LENGTH as i32;
        let oy = oy / LENGTH as i32;

        if weapon_reset {
            helper_weapon(self.map.as_mut());
        }

        if nx == ox && ny == oy {
            return Err("Player is already at that location".to_string());
        }

        if self.map[ny as usize][nx as usize] == 3 && p.weapon {
            self.map[oy as usize][ox as usize] = 0;
            self.map[ny as usize][nx as usize] = 3;
            return Ok((self.get_player_map(nx, ny, true), true));
        }

        if self.map[ny as usize][nx as usize] == 2 {
            p.weapon = true;
            self.map[oy as usize][ox as usize] = 0;
            self.map[ny as usize][nx as usize] = 3;
            return Ok((self.get_player_map(nx, ny, true), false));
        }

        if nx >= WIDTH as i32
            || nx < 0
            || ny >= HEIGHT as i32
            || ny < 0
            || self.map[ny as usize][nx as usize] != 0
        {
            return Err("Position is out of map".to_string());
        }

        if (self.map[oy as usize][ox as usize] == 3 || p.weapon)
            && self.map[ny as usize][nx as usize] == 0
            && (((nx - ox).abs() == 1 && (ny - oy) == 0)
                || ((nx - ox) == 0 && (ny - oy).abs() == 1))
        {
            self.map[oy as usize][ox as usize] = 0;
            self.map[ny as usize][nx as usize] = 3;
            let rmap = self.get_player_map(nx, ny, p.weapon);
            Ok((rmap, false))
        } else {
            Err("Cannot move to the designated location".to_string())
        }
    }

    pub fn remove_player(&mut self, x: i32, y: i32, isweapon: bool) {
        self.map[y as usize / LENGTH][x as usize / LENGTH] = 0;
        if isweapon {
            helper_weapon(self.map.as_mut());
        }
    }

    pub fn get_player_map(&mut self, x: i32, y: i32, weapon: bool) -> Vec<Vec<u8>> {
        // We will get player position and need to return map with visible parts ( only arround player; 3 unit each side), rest make 5
        let mut rmap: Vec<Vec<u8>> = vec![vec![5; WIDTH]; HEIGHT];

        let vis = if weapon == true {
            WEAPON_VISIBILITY
        } else {
            VISIBILITY
        };

        let vis = vis as i32;

        for j in (x - vis)..=(x + vis) {
            if j >= WIDTH as i32 || j < 0 {
                continue;
            }

            for i in (y - vis)..=(y + vis) {
                if i >= HEIGHT as i32 || i < 0 {
                    continue;
                }
                rmap[i as usize][j as usize] = self.map[i as usize][j as usize];
            }
        }

        rmap
    }
}
