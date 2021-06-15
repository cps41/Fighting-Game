use crate::characters; // used to get Fighter

//
// extend structure to include this in InputHandler.rs
//

// variables to help with jumping logic, needs fleshed out
// these variables may be more useful in the characterAbstract.rs
// we should also consider adding dimensions to characterAbstract
// to determine hitboxes

//TODO: resolve variables outside of function
/*let grounded: bool = true;
let falling: bool = false;
let blocking: bool = false;
let crouched: bool = false;*/

// direction enum
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn player_run(f: &characters::characterAbstract::Fighter) -> bool {
//     // this action should be looped until the key is unpressed
//     // determine which direction the player is facing
//  IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
//     if &Fighter.direction == Left {
//         while KeyDown::A | KeyDown::Left {
//             if collision {
//                 break;
//             }
//             &Fighter.x_cord--;
//             // *update player sprite*
//
//             // if the player is falling/jumping,
//             // avoid the walk animation
//         }
//     }
//     else if &Fighter.direction == Right {
//         while KeyDown::D | KeyDown::Right {
//             if collision {
//                 break;
//             }
//             &Fighter.x_cord++;
//             // *update player sprite*
//
//             // if the player is falling/jumping,
//             // avoid the walk animation
//         }
//     }
//     return true;
		return false;
}
fn player_jump(f: &characters::characterAbstract::Fighter) -> bool {
    // // if the player jumps
    // // this 'if' may not be needed
    //  IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
    // if KeyDown::W | KeyDown::Space | KeyDown::Up {
    //
    //     while &Fighter is not grounded {
    //
    //         if &Fighter.y_cord >= &Fighter.jump_height {
    //
    //             // begin descent from apex of jump
    //             bool falling = true;
    //             break;
    //         }
    //         // else increase the Y cord, still jumping
    //         else {
    //             &Fighter.y_cord ++;
    //         }
    //         // *update player sprite*
    //         // we can check here if a certain frame of jumping
    //         // should cause a sprite change, i.e. animations
    //     }
    // }
    // if falling = true {
    //     while not grounded {
    //         &Fighter.y_cord --;
    //         // *update player sprite*
    //     }
    // }
    // return true;
		return false;
}
fn player_punch(f: &characters::characterAbstract::Fighter) -> bool {
    // // *update player sprite*
    // IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
    // if there is an object within some variable 'range_of_punch' {
    //     if object is a player {
    //         if that player is blocking {
    //             // essentially do nothing
    //         }
    //         // somehow we need to recognize the other fighter object
    //         // apply appropriate damage
    //     }
    //     else {
    //         // it is nothing or part of the stage
    //         // do nothing
    //     }
    // }
		return false;
}
fn player_kick(f: &characters::characterAbstract::Fighter) -> bool {
    // similar to the punch fn
    // *update player sprite*
    // IF CROUCHED == False && BLOCK == False <-- cannot move while crouched/blocking
    // if there is an object within some variable 'range_of_kick' {
    //     if object is a player {
    //         if that player is blocking {
    //             // essentially do nothing
    //         }
    //         // somehow we need to recognize the other fighter object
    //         // apply appropriate damage
    //     }
    //     else {
    //         // it is nothing or part of the stage
    //         // do nothing
    //     }
    // }
	return false;
}
fn player_crouch(f: &characters::characterAbstract::Fighter) -> bool {
    // *update player sprite*
    // reduce sprite hitbox to appropriate ratio
    // this action should be looped similar with running
    // while KeyDown::CrouchKey {
    //     // player cannot move
    //     // resize &Fighter hitbox
    //     // crouched = true;
    // }
    // return true;
	return false;
}
fn player_block(f: &characters::characterAbstract::Fighter) -> bool {
    // // *update player sprite*
    // // this action should be looped similar with running/crouching
    // while KeyDown::BlockKey {
    //     // player cannot take damage
    //     // player cannot move, similar to crouching
    //     // blocking == true;
    // }
    // return true;
	return false;
}
