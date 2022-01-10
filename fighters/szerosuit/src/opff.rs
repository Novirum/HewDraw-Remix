use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn paralyzer_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_N_SHOOT {
        if frame > 7.0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN_DASH, false);
                }
            }
        }
    }
}

// ZSS Flip Jump Jump Cancel
unsafe fn flip_jump_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW
        || motion_kind == hash40("special_lw_start")
        || motion_kind == hash40("special_air_lw_start") {
        if frame > 20.0 {
            if moveset_utils::jump_checker_buffer(boma, cat1) {
                if hdr::get_jump_count(boma) < hdr::get_jump_count_max(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    //paralyzer_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    flip_jump_jc(boma, status_kind, motion_kind, cat[0], frame);
}

#[utils::opff(FIGHTER_KIND_SZEROSUIT )]
pub fn szerosuit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		szerosuit_frame(fighter)
    }
}

pub unsafe fn szerosuit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}