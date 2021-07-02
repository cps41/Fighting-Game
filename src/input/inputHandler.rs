use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use crate::characters; // use to get get acces to Fighter struct
use crate::input; // add to use stuff in movement
use crate::animation; // used to get States

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
                                    if fighter.char_state.state != animation::sprites::State::FJump || 
                                    fighter.char_state.state != animation::sprites::State::Jump { // if we're idle, then walk
                                        input::movement::walk(fighter); // character walks right
                                    }
                                },
                                Keycode::Return => { input::movement::block(fighter); },
                                _ => {},
                            } // end match
                        },
                        Event::KeyDown{keycode: Some(k), repeat:false, ..} => {
                            fighter.char_state.reset_current_frame(); // reset frames to 0 every click
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
                                    if fighter.char_state.state != animation::sprites::State::FJump || 
                                    fighter.char_state.state != animation::sprites::State::Jump { // if we're idle, then walk
                                        input::movement::walk(fighter); // character walks right
                                    }
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