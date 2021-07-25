use crate::characters; // used to get Fighter
use crate::animation; // used to get States
use crate::physics::vecmath::PhysVec;
use crate::physics::particle;
use crate::view::globals::*;

use serde_derive::{Serialize, Deserialize};

// direction enum
#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

//moves character on first frame a sprite is loaded
pub fn move_char(f: &mut characters::characterAbstract::Fighter){
    match f.char_state.state{
        //walk right or left, depending
        animation::sprites::State::Walk => {
            if f.char_state.frame_count == 1  ||
               f.char_state.frame_count == 6  ||
               f.char_state.frame_count == 11 ||
               f.char_state.frame_count == 16 ||
               f.char_state.frame_count == 21 ||
               f.char_state.frame_count == 26 {
                if f.char_state.direction == Direction::Right {
                    f.char_state.particle.borrow_mut().velocity.add_vec(&PhysVec::new(200.0, 0.0));
                    // f.update_position(&PhysVec::new(0.0, 0.0));
                }
                else{
                    f.char_state.particle.borrow_mut().velocity.add_vec(&PhysVec::new(-200.0, 0.0));
                    // f.update_position(&PhysVec::new(0.0, 0.0 ));
                }
            }
        },
        
        //jump or or left, depending on input
        animation::sprites::State::Jump => {
            if f.char_state.frame_count == 1 || 
               f.char_state.frame_count == 6 ||
               f.char_state.frame_count == 11{
                f.char_state.particle.borrow_mut().velocity.replace(&PhysVec::new(0.0, -800.0));
                // f.update_position(&PhysVec::new(0.0, 0.0));     
            }else if f.char_state.frame_count == 16 ||
            f.char_state.frame_count == 21 {
                f.char_state.particle.borrow_mut().velocity.replace(&PhysVec::new(0.0, -500.0));
                // f.update_position(&PhysVec::new(0.0, 0.0));            
            }else if f.char_state.frame_count == 24{
            
                if f.char_state.direction == Direction::Left{
                    // f.update_position(&PhysVec::new(-f.speed as f32, 0.0));
                }else{
                    // f.update_position(&PhysVec::new(f.speed as f32, 0.0));
                }
            }
        },
        
        //jump forward
        animation::sprites::State::FJump => {
            if f.char_state.frame_count == 1  ||
               f.char_state.frame_count == 7  ||
               f.char_state.frame_count == 13 ||
               f.char_state.frame_count == 19 {
                f.char_state.particle.borrow_mut().velocity.replace(&PhysVec::new(500.0, -500.0));
                // f.update_position(&PhysVec::new(0.0, 0.0));   
            }else if f.char_state.frame_count == 25 ||
                     f.char_state.frame_count == 31 {
                        f.char_state.particle.borrow_mut().velocity.replace(&PhysVec::new(500.0, 500.0));
                        // f.update_position(&PhysVec::new(0.0, 0.0));   
            }else{
                // f.update_position(&PhysVec::new(0f32, 0f32));
            }
        },
        
        animation::sprites::State::Idle => {
            let force = PhysVec::new(0.0, GRAVITY);
            let (x, _y) = f.char_state.velocity();
            match x {
                x if x != 0.0 => f.char_state.particle.borrow_mut().velocity.x = 0.0,
                _ => {}
            }
            f.update_position(&force);
        },

        _ => (),
    }
}

/*//Jumps
if fighter.char_state.state == animation::sprites::State::Jump ||
   fighter.char_state.state == animation::sprites::State::FJump {
    match &fighter.char_state.direction {
        input::movement::Direction::Left => {
                                if fighter.char_state.current_frame < 3 { // Note: only works since there are 6x states in Jump.
                                    fighter.char_state.position = fighter.char_state.position.offset(-fighter.speed, -fighter.speed);
                                } else if fighter.char_state.current_frame < 5 { // account for starting at 0
                                    fighter.char_state.position = fighter.char_state.position.offset(-fighter.speed, 28);
                                } else if fighter.char_state.current_frame == 5 {
                                    fighter.char_state.position = fighter.char_state.position.offset(-fighter.speed, 0);
                                    fighter.char_state.set_state(animation::sprites::State::Idle); 
                                    fighter.char_state.set_current_frame(0);
                                }
                            },
        input::movement::Direction::Right => {
                                if fighter.char_state.current_frame < 4 {
                                    fighter.char_state.position = fighter.char_state.position.offset(fighter.speed, -fighter.speed);
                                } else if fighter.char_state.current_frame < 6 {
                                    fighter.char_state.position = fighter.char_state.position.offset(fighter.speed, 28);
                                } else if fighter.char_state.current_frame == 6 {
                                    fighter.char_state.position = fighter.char_state.position.offset(fighter.speed, 28);
                                    fighter.char_state.set_state(animation::sprites::State::Idle); 
                                    fighter.char_state.set_current_frame(0);
                                }
                            },
        input::movement::Direction::Up => {
                                if fighter.char_state.current_frame < 3 {
                                    fighter.char_state.position = fighter.char_state.position.offset(0, -fighter.speed);
                                } else if fighter.char_state.current_frame < 5 { // Note: works b/c there are 6x states in jump
                                    fighter.char_state.position = fighter.char_state.position.offset(0, 28);

                                } else if fighter.char_state.current_frame == 5{
                                    fighter.char_state.position = fighter.char_state.position.offset(0, 0);
                                    //not sure the purpose of these, they set it so they are considered idle while still jumping
                                    // fighter.char_state.state = animation::sprites::State::Idle;                                            
                                    // fighter.char_state.current_frame = 0;
                                } 

                            },

        input::movement::Direction::Down => (),
     } // end direction jump match
}  // end jump if
*/

/*
pub fn walk(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::Walk);
    /*
    match &f.char_state.direction {
        Direction::Left =>  { f.char_state.update_position(vec![-f.speed, -10]); },
        Direction::Right => { f.char_state.update_position(vec![f.speed, -10]); },
        Direction::Up => (),
        Direction::Down => (),
    }
    */
}

pub fn jump(f: &mut characters::characterAbstract::Fighter) {

    match &f.char_state.direction {
        Direction::Left => { f.char_state.set_state(animation::sprites::State::Jump); },
        Direction::Right => { f.char_state.set_state(animation::sprites::State::FJump); },
        Direction::Up => { f.char_state.set_state(animation::sprites::State::Jump); },
        Direction::Down => (),
    }

} // close jump fn

pub fn block(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::Block);
} // close block fn

pub fn lkick(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::LKick);
} // close lkick fn

pub fn hkick(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::HKick);
} // close hkick fn

pub fn lpunch(f: &mut characters::characterAbstract::Fighter) {
    f.char_state.set_state(animation::sprites::State::LPunch);
} // close lpunch fn
*/

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
