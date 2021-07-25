fn create_python_fighter() {
    // These are just random values for now
    let x_pos = 0.0;
    let y_pos = 0.0;
    let weight = 180;
    let gravity = -9.8;
    let max_fall_speed = 20;
    let max_ground_speed = 10;
    let run_speed = 15;
    let max_air_speed = 5;
    let aerial_transition_speed = 3;
    let crawl_speed = 3;
    let dodge_speed = 5;
    let friction = -0.1;
    let static_grip = 20;
    let pivot_grip = 25;
    let air_resistance = -0.1;
    let air_control = 5;
    let jumps = 2;
    let jump_height = 10;
    let short_hop_height = 5;
    let air_jump_height = 7;
    let heavy_land_lag = 2;
    let wavedash_lag = 2;
    let fastfall_multiplier = 1.25;
    let hitstun_elasticity = 2.5;
    let shield_size = 3;

    let mut python_fighter = Fighter{ x_pos, y_pos, weight, gravity, max_fall_speed, max_ground_speed,
                                      run_speed, max_air_speed, aerial_transition_speed, crawl_speed, 
                                      dodge_speed, friction, static_grip, pivot_grip, 
                                      air_resistance, air_control, jumps, jump_height, 
                                      short_hop_height, air_jump_height, heavy_land_lag, wavedash_lag, 
                                      fastfall_multiplier, hitstun_elasticity, shield_size };
}

