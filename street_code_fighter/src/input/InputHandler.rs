//
// extend structure to include these mods in main
// should also include Movement.rs
//


// this file could be turned into a method and passed the keyboard input from main, or
// could be included in main.rs itself
//
// we could potentially also just include this as a mod and find another way to
// have it recognized by main

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

let mut event_pump = sdl_context.event_pump()?;

// SDL2 allows for 'running loop', essentially the same as a while
'running: loop {

    // Handle the user input during combat
    for event in event_pump.poll_iter() {
        match event {

            // these events account for the player using either the arrow keys (Right, Left, etc.)
            // OR ASDW and spacebar

            Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } |
            Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                // player moved left
                // call some code to alter momentum, player speed, and player position
                // handle this in Movement.rs
            },
            Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } |
            Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                // player moved right
                // call some code to alter momentum, player speed, and player position
                // handle this in Movement.rs
            },
            Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } |
            Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                // player moved down
                // call some code to alter momentum, player speed, and player position
                // handle this in Movement.rs
            },
            Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } |
            Event::KeyDown {keycode: Some(Keycode::Space), repeat: false, .. } => {
                // player moved up / jumped
                // acounts for 'W' or spacebar
                // call some code to alter momentum, player speed, and player position
                // handle this in Movement.rs
            },
            _ => {} // not really sure what to do for default key press yet


            // additional keyboard presses or press combinations for combat
            // moves should be included *here*
        }
    }

    // additional method of handling player movement would be to alter
    // the values in the player enum within the above loop, and calulate
    // the new player position here

}
// end loop
Ok(())

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
