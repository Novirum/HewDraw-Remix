use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn bow_ff_lc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if [*FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if hdr::compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Dark Pit Guardian Orbitar Jump Cancels
unsafe fn guardian_orbitar_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD {
        if frame > 3.0 {
            if moveset_utils::jump_checker_buffer(boma, cat1) {
                if situation_kind == *SITUATION_KIND_AIR {
                    if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    if facing * stick_x < 0.0 {
                        PostureModule::reverse_lr(boma);
                    }
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bow_ff_lc(boma, status_kind, situation_kind, cat[1], stick_y);
    guardian_orbitar_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    pits::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

#[utils::opff(FIGHTER_KIND_PITB )]
pub fn pitb_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		pitb_frame(fighter)
    }
}

pub unsafe fn pitb_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}