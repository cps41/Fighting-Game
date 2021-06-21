use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use crate::characters; // use to get get acces to Fighter struct
use crate::input; // add to use stuff in movement

pub fn keyboard_input(event: &Event, fighter: &mut characters::characterAbstract::Fighter) {
            match event {
                    Event::KeyDown{keycode: Some(k), ..} => {
                        match k {
                            Keycode::W => { input::movement::jump(fighter); }, // jump
                            Keycode::A => { fighter.char_state.direction = input::movement::Direction::Left; // update direction left
                                            input::movement::walk(fighter); // character walks left
                                           },
                            Keycode::S => (), // crouch (stretch goal)
                            Keycode::D => { fighter.char_state.direction = input::movement::Direction::Right; // update direction right 
                                            input::movement::walk(fighter); // character walks right
                                           },
                            Keycode::Space => (), 
                            // TODO: expand keyboard commands
                            _ => {},
                        } // close match k
                    } // close KeyDown
                    _ => {},
            } // close match event
} // close fn


// EDIT: delete below, move comments up (as desired)
// let mut event_pump = sdl_context.event_pump()?;

// // SDL2 allows for 'running loop', essentially the same as a while
// 'running: loop {

//     // Handle the user input during combat
//     for event in event_pump.poll_iter() {
//         match event {

//             // these events account for the player using either the arrow keys (Right, Left, etc.)
//             // OR ASDW and spacebar
// 			// EDIT: Add in Quit event for Escape
// 			// EDIT: Roll into "match" switch-like statement per example: https://github.com/nfarnan/cs1666_examples/blob/main/sdl/examples/sdl05_key_events.rs
//             Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } |
//             Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
//                 // player moved left
//                 // call some code to alter momentum, player speed, and player position
//                 // handle this in Movement.rs
//             },
//             Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } |
//             Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
//                 // player moved right
//                 // call some code to alter momentum, player speed, and player position
//                 // handle this in Movement.rs
//             },
//             Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } |
//             Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
//                 // player moved down
//                 // call some code to alter momentum, player speed, and player position
//                 // handle this in Movement.rs
//             },
//             Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } |
//             Event::KeyDown {keycode: Some(Keycode::Space), repeat: false, .. } => {
//                 // player moved up / jumped
//                 // acounts for 'W' or spacebar
//                 // call some code to alter momentum, player speed, and player position
//                 // handle this in Movement.rs
//             },
// 			// EDIT: Add in all fight key codes, based on Appendix > controls: https://docs.google.com/document/d/1k_R2QGC2Lmlz-AfOTmTTM9RsKEg4kmrS/edit# (no preference here)

// 			// additional keyboard presses or press combinations for combat
//             // moves should be included *here*
			
//             _ => {} // not really sure what to do for default key press yet



//         }
//     }

//     // additional method of handling player movement would be to alter
//     // the values in the player enum within the above loop, and calulate
//     // the new player position here

// }
// // end loop
// Ok(())

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