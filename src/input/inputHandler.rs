use std::collections::HashSet;

use sdl2::keyboard::Keycode;

use crate::characters; // use to get get acces to Fighter struct
use crate::input; // add to use stuff in movement
use crate::animation; // used to get States


pub fn keyboard_input(player_input: &HashSet<u8>, fighter: &mut characters::characterAbstract::Fighter){
    //if character animation is over, reset to idle
    if fighter.char_state.frame_count == animation::sprites::get_frame_cnt(&fighter.char_state){
        fighter.char_state.set_state(animation::sprites::State::Idle);
        fighter.char_state.reset_current_frame();
        fighter.char_state.direction = input::movement::Direction::Up;        
    }

    //inputs accepted while idle
    if fighter.char_state.state ==  animation::sprites::State::Idle
       && !player_input.is_empty(){
        for pressed in player_input.iter(){
            match pressed{
                1 =>       {fighter.char_state.direction = input::movement::Direction::Left;
                                     fighter.char_state.set_state(animation::sprites::State::Walk);
                                     fighter.char_state.reset_current_frame();   
                                     break;},
                2 =>       {fighter.char_state.direction = input::movement::Direction::Right;
                                     fighter.char_state.set_state(animation::sprites::State::Walk);    
                                     fighter.char_state.reset_current_frame();   
                                     break;},
                3 =>  {fighter.char_state.set_state(animation::sprites::State::Block);    
                                     fighter.char_state.reset_current_frame();   
                                     break;},
                4 =>       {fighter.char_state.set_state(animation::sprites::State::Jump);    
                                     fighter.char_state.reset_current_frame();   
                                     break;},
                5 =>       {fighter.char_state.set_state(animation::sprites::State::LKick);    
                                     fighter.char_state.reset_current_frame();   
                                     break;},
                6 =>       {fighter.char_state.set_state(animation::sprites::State::HKick);    
                                     fighter.char_state.reset_current_frame();   
                                     break;},
                7 =>       {fighter.char_state.set_state(animation::sprites::State::LPunch);    
                                     fighter.char_state.reset_current_frame();   
                                     break;},
                _=> {},
            }
        }
    //inputs accepted while walking
    }else if fighter.char_state.state == animation::sprites::State::Walk{
        //if no longer holding down, stop walking
        if player_input.is_empty(){
            fighter.char_state.direction = input::movement::Direction::Up;
            fighter.char_state.set_state(animation::sprites::State::Idle); 
            fighter.char_state.reset_current_frame();
        }else{
            //inputs that intterupt walk
            for pressed in player_input.iter(){
                match pressed{
                    3 =>  {fighter.char_state.set_state(animation::sprites::State::Block);   
                                         fighter.char_state.reset_current_frame();   
                                         return;},
                    4 =>       {if fighter.char_state.direction == input::movement::Direction::Right{
                                            fighter.char_state.set_state(animation::sprites::State::FJump);
                                         }else{
                                             fighter.char_state.set_state(animation::sprites::State::Jump);
                                         }
                                         fighter.char_state.reset_current_frame();
                                         return;},
                    5 =>       {fighter.char_state.set_state(animation::sprites::State::LKick);   
                                         fighter.char_state.reset_current_frame();   
                                         return;},
                    6 =>       {fighter.char_state.set_state(animation::sprites::State::HKick);
                                         fighter.char_state.reset_current_frame();   
                                         return;},
                    7 =>       {fighter.char_state.set_state(animation::sprites::State::LPunch);
                                         fighter.char_state.reset_current_frame();   
                                         return;},
                    _=> {},            
                }
                //if not trying to interrupt, keep walking
                if fighter.char_state.direction == input::movement::Direction::Right {
                    if player_input.contains(&1){
                        fighter.char_state.direction = input::movement::Direction::Left;
                    }else{
                        fighter.char_state.direction = input::movement::Direction::Right;
                    }
                }else if fighter.char_state.direction == input::movement::Direction::Left {
                    if player_input.contains(&2){
                        fighter.char_state.direction = input::movement::Direction::Right;
                    }else{
                        fighter.char_state.direction = input::movement::Direction::Left;
                    }
                }

            }
        }
    //TODO: handle block intterupts
    }else if fighter.char_state.state == animation::sprites::State::Block{

    }
}



pub fn convert_input(player_input: &HashSet<Keycode>) -> HashSet<u8>{

    let mut set: HashSet<u8> = HashSet::new();

    for pressed in player_input.iter(){
        match pressed{
            Keycode::A =>       {set.insert(1);},
            Keycode::D =>       {set.insert(2);},
            Keycode::Return =>  {set.insert(3);},
            Keycode::W =>       {set.insert(4);},
            Keycode::J =>       {set.insert(5);},
            Keycode::I =>       {set.insert(6);},
            Keycode::K =>       {set.insert(7);},
            _=> {},
        }
    }
    
    set
}


/*
pub fn keyboard_input(event: &Event, fighter: &mut characters::characterAbstract::Fighter) {

            // if !fighter.char_state.isMoving() {
                match event {
                        Event::KeyDown{keycode: Some(k), repeat:true, ..} => {
                            match k {
                                Keycode::A => {
                                    fighter.char_state.direction = input::movement::Direction::Left; // update direction left
                                    if fighter.char_state.state != animation::sprites::State::FJump || 
                                       fighter.char_state.state != animation::sprites::State::Jump { // if we're idle, then walk
                                        input::movement::walk(fighter); // character walks left
                                    }
                                },
                                Keycode::D => {
                                    fighter.char_state.direction = input::movement::Direction::Right; // update direction right 
                                    input::movement::walk(fighter);
                                    /*
                                    if fighter.char_state.state != animation::sprites::State::FJump || 
                                    fighter.char_state.state != animation::sprites::State::Jump { // if we're idle, then walk
                                        input::movement::walk(fighter); // character walks right
                                    }
                                    */
                                },
                                Keycode::W => { 
                                    if (fighter.char_state.state != animation::sprites::State::FJump && 
                                        fighter.char_state.state != animation::sprites::State::Jump) {
                                        input::movement::jump(fighter); 
                                    }
                                }, // jump                                                                                     
                                Keycode::S => (), // crouch (stretch goal)
                                Keycode::Space => (),
                                Keycode::J => { input::movement::lkick(fighter); }, 
                                Keycode::I => { input::movement::hkick(fighter); }, 
                                Keycode::K => { input::movement::lpunch(fighter); }, 

                                // Stetch goal: expand keyboard commands, if expand moves
                                _ => {},
                            } // close match k
                        } // close KeyDown
                        _ => {},
                } // close match event
            // } // close if stmt
} // close fn
*/

// *** ALL POSSIBLE KEYCODES AVAILABLE FOR KeyEvent *** //
// https://docs.rs/sdl2/0.9.0/sdl2/keyboard/enum.Keycode.html //

// pub enum Keycode {
//     Backspace,
//     Tab,
//     Return,
//     Escape,
//     Space,
//     Exclaim,
//     Quotedbl,
//     Hash,
//     Dollar,
//     Percent,
//     Ampersand,
//     Quote,
//     LeftParen,
//     RightParen,
//     Asterisk,
//     Plus,
//     Comma,
//     Minus,
//     Period,
//     Slash,
//     Num0,
//     Num1,
//     Num2,
//     Num3,
//     Num4,
//     Num5,
//     Num6,
//     Num7,
//     Num8,
//     Num9,
//     Colon,
//     Semicolon,
//     Less,
//     Equals,
//     Greater,
//     Question,
//     At,
//     LeftBracket,
//     Backslash,
//     RightBracket,
//     Caret,
//     Underscore,
//     Backquote,
//     A,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G,
//     H,
//     I,
//     J,
//     K,
//     L,
//     M,
//     N,
//     O,
//     P,
//     Q,
//     R,
//     S,
//     T,
//     U,
//     V,
//     W,
//     X,
//     Y,
//     Z,
//     Delete,
//     CapsLock,
//     F1,
//     F2,
//     F3,
//     F4,
//     F5,
//     F6,
//     F7,
//     F8,
//     F9,
//     F10,
//     F11,
//     F12,
//     PrintScreen,
//     ScrollLock,
//     Pause,
//     Insert,
//     Home,
//     PageUp,
//     End,
//     PageDown,
//     Right,
//     Left,
//     Down,
//     Up,
//     NumLockClear,
//     KpDivide,
//     KpMultiply,
//     KpMinus,
//     KpPlus,
//     KpEnter,
//     Kp1,
//     Kp2,
//     Kp3,
//     Kp4,
//     Kp5,
//     Kp6,
//     Kp7,
//     Kp8,
//     Kp9,
//     Kp0,
//     KpPeriod,
//     Application,
//     Power,
//     KpEquals,
//     F13,
//     F14,
//     F15,
//     F16,
//     F17,
//     F18,
//     F19,
//     F20,
//     F21,
//     F22,
//     F23,
//     F24,
//     Execute,
//     Help,
//     Menu,
//     Select,
//     Stop,
//     Again,
//     Undo,
//     Cut,
//     Copy,
//     Paste,
//     Find,
//     Mute,
//     VolumeUp,
//     VolumeDown,
//     KpComma,
//     KpEqualsAS400,
//     AltErase,
//     Sysreq,
//     Cancel,
//     Clear,
//     Prior,
//     Return2,
//     Separator,
//     Out,
//     Oper,
//     ClearAgain,
//     CrSel,
//     ExSel,
//     Kp00,
//     Kp000,
//     ThousandsSeparator,
//     DecimalSeparator,
//     CurrencyUnit,
//     CurrencySubUnit,
//     KpLeftParen,
//     KpRightParen,
//     KpLeftBrace,
//     KpRightBrace,
//     KpTab,
//     KpBackspace,
//     KpA,
//     KpB,
//     KpC,
//     KpD,
//     KpE,
//     KpF,
//     KpXor,
//     KpPower,
//     KpPercent,
//     KpLess,
//     KpGreater,
//     KpAmpersand,
//     KpDblAmpersand,
//     KpVerticalBar,
//     KpDblVerticalBar,
//     KpColon,
//     KpHash,
//     KpSpace,
//     KpAt,
//     KpExclam,
//     KpMemStore,
//     KpMemRecall,
//     KpMemClear,
//     KpMemAdd,
//     KpMemSubtract,
//     KpMemMultiply,
//     KpMemDivide,
//     KpPlusMinus,
//     KpCear,
//     KpClearEntry,
//     KpBinary,
//     KpOctal,
//     KpDecimal,
//     KpHexadecimal,
//     LCtrl,
//     LShift,
//     LAlt,
//     LGui,
//     RCtrl,
//     RShift,
//     RAlt,
//     RGui,
//     Mode,
//     AudioNext,
//     AudioPrev,
//     AudioStop,
//     AudioPlay,
//     AudioMute,
//     MediaSelect,
//     Www,
//     Mail,
//     Calculator,
//     Computer,
//     AcSearch,
//     AcHome,
//     AcBack,
//     AcForward,
//     AcStop,
//     AcRefresh,
//     AcBookmarks,
//     BrightnessDown,
//     BrightnessUp,
//     DisplaySwitch,
//     KbdIllumToggle,
//     KbdIllumDown,
//     KbdIllumUp,
//     Eject,
//     Sleep,
// }