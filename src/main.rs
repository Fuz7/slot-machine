mod json;

use json::{get_player,update_exp,update_highscore,update_revive,Player};

fn main(){

    // Write JSON into file
    let player: Player = get_player();
    let _ = update_exp(40);
    let _ = update_highscore(20);
    let _ = update_revive(30);
    println!("{:?}",player)
}