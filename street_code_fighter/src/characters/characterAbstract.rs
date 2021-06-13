#[derive(Default)]
struct Fighter {
    x_pos: f32,
    y_pos: f32,
    weight: u16,
    gravity: f32,
    max_fall_speed: u16,
    max_ground_speed: u16,
    run_speed: u16,
    max_air_speed: u16,
    aerial_transition_speed: u16,
    crawl_speed: u16,
    dodge_speed: u16,
    friction: f32,
    static_grip: u16,
    pivot_grip: u16,
    air_resistance: f32,
    air_control: u16,
    jumps: u16,
    jump_height: u16,
    short_hop_height: u16,
    air_jump_height: u16,
    heavy_land_lag: u16,
    wavedash_lag: u16,
    fastfall_multiplier: f32,
    hitstun_elasticity: f32,
    shield_size: u16,
}

impl Person {
    // Getters
    fn x_pos(&self) -> &String {
        &self.x_pos
    }
    fn y_pos(&self) -> &String {
        &self.y_pos
    }
    fn weight(&self) -> &String {
        &self.weight
    }
    fn gravity(&self) -> &String {
        &self.gravity
    }
    fn max_fall_speed(&self) -> &String {
        &self.max_fall_speed
    }
    fn max_ground_speed(&self) -> &String {
        &self.max_ground_speed
    }
    fn run_speed(&self) -> &String {
        &self.run_speed
    }
    fn max_air_speed(&self) -> &String {
        &self.max_air_speed
    }
    fn aerial_transition_speed(&self) -> &String {
        &self.aerial_transition_speed
    }
    fn crawl_speed(&self) -> &String {
        &self.crawl_speed
    }
    fn dodge_speed(&self) -> &String {
        &self.dodge_speed
    }
    fn friction(&self) -> &String {
        &self.friction
    }
    fn static_grip(&self) -> &String {
        &self.static_grip
    }
    fn pivot_grip(&self) -> &String {
        &self.pivot_grip
    }
    fn air_resistance(&self) -> &String {
        &self.air_resistance
    }
    fn air_control(&self) -> &String {
        &self.air_control
    }
    fn jumps(&self) -> &String {
        &self.jumps
    }
    fn jump_height(&self) -> &String {
        &self.jump_height
    }
    fn short_hop_height(&self) -> &String {
        &self.short_hop_height
    }
    fn air_jump_height(&self) -> &String {
        &self.air_jump_height
    }
    fn heavy_land_lag(&self) -> &String {
        &self.heavy_land_lag
    }
    fn wavedash_lag(&self) -> &String {
        &self.wavedash_lag
    }
    fn fastfall_multiplier(&self) -> &String {
        &self.fastfall_multiplier
    }
    fn hitstun_elasticity(&self) -> &String {
        &self.hitstun_elasticity
    }
    fn shield_size(&self) -> &String {
        &self.shield_size
    }

    // Setters
    fn x_pos_mut(&mut self) -> &mut String {
        &mut self.x_pos
    }
    fn y_pos_mut(&mut self) -> &mut String {
        &mut self.y_pos
    }
    fn weight_mut(&mut self) -> &mut String {
        &mut self.weight
    }
    fn gravity_mut(&mut self) -> &mut String {
        &mut self.gravity
    }
    fn max_fall_speed_mut(&mut self) -> &mut String {
        &mut self.max_fall_speed
    }
    fn max_ground_speed_mut(&mut self) -> &mut String {
        &mut self.max_ground_speed
    }
    fn run_speed_mut(&mut self) -> &mut String {
        &mut self.run_speed
    }
    fn max_air_speed_mut(&mut self) -> &mut String {
        &mut self.max_air_speed
    }
    fn aerial_transition_speed_mut(&mut self) -> &mut String {
        &mut self.aerial_transition_speed
    }
    fn crawl_speed_mut(&mut self) -> &mut String {
        &mut self.crawl_speed
    }
    fn dodge_speed_mut(&mut self) -> &mut String {
        &mut self.dodge_speed
    }
    fn friction_mut(&mut self) -> &mut String {
        &mut self.friction
    }
    fn static_grip_mut(&mut self) -> &mut String {
        &mut self.static_grip
    }
    fn pivot_grip_mut(&mut self) -> &mut String {
        &mut self.pivot_grip
    }
    fn air_resistance_mut(&mut self) -> &mut String {
        &mut self.air_resistance
    }
    fn air_control_mut(&mut self) -> &mut String {
        &mut self.air_control
    }
    fn jumps_mut(&mut self) -> &mut String {
        &mut self.jumps
    }
    fn jump_height_mut(&mut self) -> &mut String {
        &mut self.jump_height
    }
    fn short_hop_height_mut(&mut self) -> &mut String {
        &mut self.short_hop_height
    }
    fn air_jump_height_mut(&mut self) -> &mut String {
        &mut self.air_jump_height
    }
    fn heavy_land_lag_mut(&mut self) -> &mut String {
        &mut self.heavy_land_lag
    }
    fn wavedash_lag_mut(&mut self) -> &mut String {
        &mut self.wavedash_lag
    }
    fn fastfall_multiplier_mut(&mut self) -> &mut String {
        &mut self.fastfall_multiplier
    }
    fn hitstun_elasticity_mut(&mut self) -> &mut String {
        &mut self.hitstun_elasticity
    }
    fn shield_size_mut(&mut self) -> &mut String {
        &mut self.shield_size
    }
}
