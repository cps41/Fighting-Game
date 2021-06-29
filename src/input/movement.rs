use crate::characters; // used to get Fighter
use crate::animation; // used to get States

use serde_derive::{Serialize, Deserialize}; // NOT YET OFFICIALLY AUTHORIZED

// direction enum
#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn walk(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::Walk);

    match &f.char_state.direction {
        Direction::Left =>  { f.char_state.position = f.char_state.position.offset(-f.speed, 0); },
        Direction::Right => { f.char_state.position = f.char_state.position.offset(f.speed, 0); },
        Direction::Up => (),
        Direction::Down => (),
    }
}

pub fn jump(f: &mut characters::characterAbstract::Fighter) {

    match &f.char_state.direction {
        Direction::Left => { f.char_state.set_state(animation::sprites::State::Jump); },
        Direction::Right => { f.char_state.set_state(animation::sprites::State::FJump); },
        Direction::Up => { f.char_state.set_state(animation::sprites::State::Jump); },
        Direction::Down => (),
    }
    println!("Jump Input");

} // close jump fn

pub fn block(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::Block);
} // close block fn

pub fn lkick(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::LKick);
    println!("Low Kick Input");
} // close lkick fn

pub fn hkick(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::HKick);
    println!("High Kick Input");
} // close hkick fn

pub fn lpunch(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::LPunch);
    println!("Left Punch Input");
} // close lpunch fn


// EDIT: make functions public
// EDIT: remove "player_"
// fn player_run(f: &characters::characterAbstract::Fighter) -> bool {
// //     // this action should be looped until the key is unpressed
// //     // determine which direction the player is facing
// //  IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
// //     if &Fighter.direction == Left {
// //         while KeyDown::A | KeyDown::Left {
// //             if collision {
// //                 break;
// //             }
// //             &Fighter.x_cord--;
// //             // *update player sprite*
// //
// //             // if the player is falling/jumping,
// //             // avoid the walk animation
// //         }
// //     }
// //     else if &Fighter.direction == Right {
// //         while KeyDown::D | KeyDown::Right {
// //             if collision {
// //                 break;
// //             }
// //             &Fighter.x_cord++;
// //             // *update player sprite*
// //
// //             // if the player is falling/jumping,
// //             // avoid the walk animation
// //         }
// //     }
// //     return true;
// 		return false;
// }
// fn player_jump(f: &characters::characterAbstract::Fighter) -> bool {
//     // // if the player jumps
//     // // this 'if' may not be needed
//     //  IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
//     // if KeyDown::W | KeyDown::Space | KeyDown::Up {
//     //
//     //     while &Fighter is not grounded {
//     //
//     //         if &Fighter.y_cord >= &Fighter.jump_height {
//     //
//     //             // begin descent from apex of jump
//     //             bool falling = true;
//     //             break;
//     //         }
//     //         // else increase the Y cord, still jumping
//     //         else {
//     //             &Fighter.y_cord ++;
//     //         }
//     //         // *update player sprite*
//     //         // we can check here if a certain frame of jumping
//     //         // should cause a sprite change, i.e. animations
//     //     }
//     // }
//     // if falling = true {
//     //     while not grounded {
//     //         &Fighter.y_cord --;
//     //         // *update player sprite*
//     //     }
//     // }
//     // return true;
// 		return false;
// }
// fn player_punch(f: &characters::characterAbstract::Fighter) -> bool {
//     // // *update player sprite*
//     // IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
//     // if there is an object within some variable 'range_of_punch' {
//     //     if object is a player {
//     //         if that player is blocking {
//     //             // essentially do nothing
//     //         }
//     //         // somehow we need to recognize the other fighter object
//     //         // apply appropriate damage
//     //     }
//     //     else {
//     //         // it is nothing or part of the stage
//     //         // do nothing
//     //     }
//     // }
// 		return false;
// }
// fn player_kick(f: &characters::characterAbstract::Fighter) -> bool {
//     // similar to the punch fn
//     // *update player sprite*
//     // IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
//     // if there is an object within some variable 'range_of_kick' {
//     //     if object is a player {
//     //         if that player is blocking {
//     //             // essentially do nothing
//     //         }
//     //         // somehow we need to recognize the other fighter object
//     //         // apply appropriate damage
//     //     }
//     //     else {
//     //         // it is nothing or part of the stage
//     //         // do nothing
//     //     }
//     // }
// 	return false;
// }
// fn player_crouch(f: &characters::characterAbstract::Fighter) -> bool {
//     // *update player sprite*
//     // reduce sprite hitbox to appropriate ratio
//     // this action should be looped similar with running
//     // while KeyDown::CrouchKey {
//     //     // player cannot move
//     //     // resize &Fighter hitbox
//     //     // crouched = true;
//     // }
//     // return true;
// 	return false;
// }
// fn player_block(f: &characters::characterAbstract::Fighter) -> bool {
//     // // *update player sprite*
//     // // this action should be looped similar with running/crouching
//     // while KeyDown::BlockKey {
//     //     // player cannot take damage
//     //     // player cannot move, similar to crouching
//     //     // blocking == true;
//     // }
//     // return true;
// 	return false;
// }
