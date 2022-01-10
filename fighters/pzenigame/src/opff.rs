use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
    }
    if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Squirtle Withdraw Jump Cancels
unsafe fn withdraw_jc(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    /*
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END].contains(&status_kind)
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame > 10.0 {
    */
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        withdraw_frame[id] = 0.0;
    }
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP].contains(&status_kind) {
        // Increment the Withdraw frame every frame you're in the SPECIAL_S_LOOP status kind
        withdraw_frame[id] += 1.0;
        // JC Lockout: frame 30
        if withdraw_frame[id] > 15.0 {
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

    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) && frame < 10.0 {
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

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    withdraw_jc(boma, id, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    nspecial_cancels(boma, status_kind, situation_kind, cat[0]);
}

#[utils::opff(FIGHTER_KIND_PZENIGAME )]
pub fn pzenigame_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		pzenigame_frame(fighter)
    }
}

pub unsafe fn pzenigame_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}