use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn clown_cannon_shield_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD {
        if frame > 15.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

// Bowser Jr. Clown Cannon Dash Cancel
unsafe fn clown_cannon_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_SHOOT {
        if frame > 13.0 {
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

// Bowser Jr. Kart Jump Waveland
unsafe fn kart_jump_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP && situation_kind == *SITUATION_KIND_AIR {
        if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
    }
}

unsafe fn upB_kart_respawn(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) && WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) == 0.0 && WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, false);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    clown_cannon_shield_cancel(boma, status_kind, situation_kind, frame);
    clown_cannon_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    kart_jump_waveland(boma, status_kind, situation_kind, cat[0]);
    upB_kart_respawn(boma, status_kind);
}

pub unsafe fn weapon_moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    
}

#[utils::opff(FIGHTER_KIND_KOOPAJR )]
pub fn koopajr_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		koopajr_frame(fighter)
    }
}

pub unsafe fn koopajr_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame( agent = WEAPON_KIND_KOOPAJR_CANNONBALL )]
pub fn koopajr_weapon_frame_wrapper(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        koopajr_weapon_frame(weapon)
    }
}

pub unsafe fn koopajr_weapon_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    if let Some(info) = crate::hooks::sys_line::WeaponFrameInfo::weapon_update_and_get(weapon) {
        weapon_moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}