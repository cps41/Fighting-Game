// extern crate street_code_fighter;
//
// use std::collections::HashSet;
//
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
//
// use street_code_fighter::SDLCore;
// use street_code_fighter::Demo;
//
// pub fn check_collision(a: &CollisionObject, b: &CollisionObject) -> bool {
// 	match a.obj_type {
// 		CollisionObjectType::Character => {
// 			match b.obj_type {
// 				CollisionObjectType::Character => false,
// 				_ => reg_collision(a.rect, b.rect),
// 			}
// 		},
// 		_ => reg_collision(a.rect, b.rect),
// 	}
// 	reg_collision(a.rect, b.rect)
// }
//
// pub fn reg_collision(a: &Rect, b: &Rect) -> bool {
// 	if a.bottom() < b.top()
// 			|| a.top() > b.bottom()
// 			|| a.right() < b.left()
// 			|| a.left() > b.right()
// 		{
// 			false;
// 		}
// 		else {
// 			true;
// 		}
// }
//
// pub fn resist(vel: i32, deltav: i32) -> i32 {
// 	if deltav == 0 {
// 		if vel > 0 {
// 			-1
// 		}
// 		else if vel < 0 {
// 			1
// 		}
// 		else {
// 			deltav
// 		}
// 	}
// 	else {
// 		deltav
// 	}
// }
//
// enum COLLISION_OBJECT_TYPE {
//     Character,
//     Hazard,
//     Platform,
//     Wall
// }
//
// pub struct COLLISION_OBJECT {
//     obj_type: COLLISION_OBJECT_TYPE,
//     size: i16,
//     rect: Rect,
// }
//
// impl COLLISION_OBJECT {
//     fn new(obj_type: COLLISION_OBJECT_TYPE, x: i32, y: i32, width: i32, height: i32) -> COLLISION_OBJECT {
//         let r = Rect::new(x, y, width, height);
//         COLLISION_OBJECT {
//             obj_type,
//             r,
//         }
//     }
// }
