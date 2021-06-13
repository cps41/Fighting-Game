fn create_python_fighter() {
    let weight = 50;
    let gravity = 50;
    let max_fall_speed = 50;
    let max_ground_speed = 50;
    let run_speed = 50;
    let max_air_speed = 50;
    let aerial_transition_speed = 50;
    let crawl_speed = 50;
    let dodge_speed = 50;
    let friction = 50;
    let static_grip = 50;
    let pivot_grip = 50;
    let air_resistance = 50;
    let air_control = 50;
    let jumps = 50;
    let jump_height = 50;
    let short_hop_height = 50;
    let air_jump_height = 50;
    let heavy_land_lag = 50;
    let wavedash_lag = 50;
    let fastfall_multiplier = 50;
    let hitstun_elasticity = 50;
    let shield_size = 50;

    let mut python_fighter = Fighter{ weight, gravity, max_fall_speed, max_ground_speed,
                                      run_speed, max_air_speed, aerial_transition_speed, crawl_speed, 
                                      dodge_speed, friction, static_grip, pivot_grip, 
                                      air_resistance, air_control, jumps, jump_height, 
                                      short_hop_height, air_jump_height, heavy_land_lag, wavedash_lag, 
                                      fastfall_multiplier, hitstun_elasticity, shield_size };
}
