// use std::collections::HashMap;
// use sdl2::image::LoadTexture; 
// use sdl2::render::Texture;

// use crate::characters;
// use crate::animation;

// #[derive(Hash, Eq, PartialEq, Debug)]
// struct CharacterMap {
//     name: character::characterAbstract::Character,
//     state: animation::spites::State,
//     texture: 
// }

// struct 



// pub fn load_textures(s: &mut self) -> 
// 	HashMap<characters::characterAbstract::Characters, 
// 			HashMap<animation::sprites::State, &'static Texture>> {//>> {
	
// 	// initialize Texture creator
// 	let texture_creator = s.core.wincan.texture_creator();

// 	// initialize HashMaps
// 	let mut all_characters = HashMap::new();
// 	let mut python = HashMap::new();

// 	// create textures
// 	let test_texture = texture_creator.load_texture("src/assets/images/characters/python/idle.png");



// 	// insert States and Textures
// 	python.insert(animation::sprites::State::Idle, test_texture);

// 	// insert Characters into all_characters
// 	all_characters.insert(characters::characterAbstract::Characters::Python, python);

// 	// return all_characters
// 	all_characters
// }


				// State::Idle => { return "src/assets/images/characters/python/idle.png"; },
				// State::Walk => { return "src/assets/images/characters/python/walk.png"; },
				// State::Jump => { return "src/assets/images/characters/python/jump.png"; },
				// State::FJump => { return "src/assets/images/characters/python/fjump.png"; },
				// State::LPunch => { return "src/assets/images/characters/python/lpunch.png"; },
				// State::LKick => { return "src/assets/images/characters/python/lkick.png"; },
				// State::HKick => { return "src/assets/images/characters/python/hkick.png"; },
				// State::Block => { return "src/assets/images/characters/python/block.png"; },



//////////

// pub struct TextureManager<'a> {
//     tc: &'a TextureCreator<WindowCanvas>,
//     root_path: String,
//     textures: HashMap<String, Texture<'a>>,
// }

// impl<'s> TextureManager<'s> {
//     pub fn new<'a>(texture_creator: &'a TextureCreator<WindowCanvas>, initial_path: String) -> TextureManager {
//         return TextureManager { tc: &texture_creator, 
//                                 root_path: initial_path,
//                                 textures: HashMap::new() };
//     }

//     pub fn load(&mut self, resource: &str) -> &Texture {
//         let full_resource_path = String::from(Path::new(self.root_path.as_str())
//             .join(resource).to_str().unwrap());

//         {
//             if self.textures.contains_key(&full_resource_path) {
//                 return self.textures.get(&full_resource_path).unwrap();
//             }
//         }

//         let new_texture = self.tc.load_texture(Path::new(self.root_path.as_str())
//             .join(resource).as_path()).unwrap();
//         self.textures.insert(full_resource_path.clone(), new_texture);

//         return self.textures.get(&full_resource_path).unwrap().clone();
//     }

//     fn unload_all(&mut self) {
//         self.textures.clear();
//     }
// }

/////////////


// // setting up
// pub struct loadedImages { 
//     character: characters::characterAbstract::Character,
//     texture: Texture<'a>,
// }

// impl loadedImages {
//     fn texture(&self) -> &Texture {
//         &self.texture
//     }
// }