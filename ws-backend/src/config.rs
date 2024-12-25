use rand::seq::SliceRandom;

pub const WIDTH: usize = 43;
pub const HEIGHT: usize = 22;
pub const LENGTH: usize = 35;
pub const VISIBILITY: u8 = 2;
pub const WEAPON_VISIBILITY: u8 = 5;
pub const WEAPON_MOVES: i32 = 50;

pub fn weapon_location() -> (u8, u8) {
    let location: Vec<(u8, u8)> = vec![(37, 1), (34, 16), (22, 14)];
    // randomly choose one out of above location and send back

    *location.choose(&mut rand::thread_rng()).unwrap()
}
