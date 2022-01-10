use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn dair_mash_rise(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64, situation_kind: i32, frame: f32) {
    if motion_kind == hash40("attack_air_lw") {
        //let motion_vec = Vector3f{x:0.0, y: 2.5, z: 0.0};
        let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
        let cbm_vec2 = Vector4f{ /* Red */ x: 0.9907, /* Green */ y: 0.02, /* Blue */ z: 0.0251, /* Alpha */ w: 0.2};
        let rise_amount = 0.275;
        let max_drift_speed = 0.955;
        let max_rise_speed = 0.815;
        let mut motion_vec = Vector3f{x:0.0, y: rise_amount, z: 0.0};
        let x_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let facing = PostureModule::lr(boma);
        if frame <= 28.0 {
            if hdr::compare_cat(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
                // Tell the game that you've started rising
                aerial_command_rising[id] = true;
                // Add vertical speed for the dair rise if you've activated the rise and this isn't your second time attempting to initiate the rise during your current airtime
                if aerial_command_rising[id] && !aerial_command_risen[id] {
                    // Reset momentum on the first special button press press
                    if !aerial_command_momentum_reset[id]{
                        // Slow down the move to better facilitate recovering
                        MotionModule::set_rate(boma, 0.5);
                        // Have mario glow a bit to indicate that he's recovering
                        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.21, 1.0, 5, /* Display Color */ true);
                        // Reset momentum
                        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                        aerial_command_momentum_reset[id] = true;
                    }
                    //KineticModule::add_speed(boma, &motion_vec);
                    if y_speed + motion_vec.y > max_rise_speed {
                        motion_vec.y = max_rise_speed - y_speed;
                    }
                    KineticModule::add_speed(boma, &motion_vec);
                    motion_vec.y = rise_amount;

                    /*
                    if x_speed.abs() > max_drift_speed {
                        motion_vec.x = (max_drift_speed * facing) - x_speed;
                    }
                    KineticModule::add_speed(boma, &motion_vec);
                    motion_vec.x = 0.0;
                    */
                    //KineticModule::add_speed_outside(boma,*KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION,&motion_vec);
                }
            }
        }
    }

    if aerial_command_rising[id] {
        if motion_kind != hash40("attack_air_lw")
            || (motion_kind == hash40("attack_air_lw") && frame > 28.0) {
            ColorBlendModule::cancel_main_color(boma, 0);
            aerial_command_risen[id] = true;
            aerial_command_rising[id] = false;
            aerial_command_momentum_reset[id] = false;
        }
    }

    // If grounded, reset aerial rise and momentum reset flags
    if situation_kind == *SITUATION_KIND_GROUND && aerial_command_risen[id] {
        aerial_command_risen[id] = false;
        aerial_command_momentum_reset[id] = false;
    }
}

// Super Jump Punch Wall Jump
unsafe fn up_b_wall_jump(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if situation_kind == *SITUATION_KIND_AIR {
            if frame >= 23.0 && frame <= 25.0 {
                if !special_wall_jump[id] {
                    if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32) {
                        if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) {
                            special_wall_jump[id] = true;
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                        }
                    }
                    if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) {
                        if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) {
                            special_wall_jump[id] = true;
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                        }
                    }
                }
            }
        }
    }
}

// F.L.U.D.D. B-Reverse
unsafe fn fludd_b_reverse(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT].contains(&status_kind) {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 && !b_reversed[id] {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    b_reversed[id] = true;
                }
            }
        }
    }
}

unsafe fn dspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like down-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
    }
    if status_kind == *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Fireball double article fix
unsafe fn special_n_article_fix(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
        //if situation_kind == *SITUATION_KIND_GROUND {
            if frame <= 1.0 /*frame >= 13.0 && frame < 15.0*/ {
                //println!("Reset fireball projectile flag");
                special_projectile_spawned[id] = false;
            }
        //}
        /*
        else if situation_kind == *SITUATION_KIND_AIR {
            if frame >= 14.0 && frame < 15.0{
                special_projectile_spawned[id] = true;
                println!("=== PROJECTILE SPAWNED FROM AERIAL VERSION");
            }
        }
        */
    }
    /*
    else{
        if special_projectile_spawned[id]{
            special_projectile_spawned[id] = false;
        }
    }

    if !special_projectile_spawned[id]{
    }
    */
}

// NokNok Shell Timer Count
unsafe fn noknok_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 1801 {
        if gimmick_timerr > 1799 {
            noknok_shell[id] = false;
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// NokNok shell flag reset
unsafe fn noknok_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if noknok_shell[id] {
        if [*FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
                noknok_shell[id] = false;
        }
    }
}

// TRAINING MODE
// NokNok shell flag reset via taunt
unsafe fn noknok_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if hdr::is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            noknok_shell[id] = false;
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //dair_mash_rise(fighter, boma, id, motion_kind, situation_kind, frame);
    up_b_wall_jump(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
    fludd_b_reverse(fighter, boma, id, status_kind, stick_x, facing, frame);
    dspecial_cancels(boma, status_kind, situation_kind, cat[0]);
    special_n_article_fix(fighter, boma, id, status_kind, situation_kind, frame);
    noknok_timer(fighter, boma, id);
    noknok_reset(fighter, id, status_kind);
    noknok_training(fighter, id, status_kind);
}

#[utils::opff(FIGHTER_KIND_MARIO )]
pub fn mario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		mario_frame(fighter)
    }
}

pub unsafe fn mario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}