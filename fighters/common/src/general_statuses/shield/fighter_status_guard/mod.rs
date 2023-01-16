// status imports
use super::*;
use globals::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon37FighterStatusGuard__calc_shield_scaleEN3lib8L2CValueE")]
pub unsafe fn calc_shield_scale(fighter: &mut L2CFighterCommon, shield_level: L2CValue) -> L2CValue {
    let shield_level = shield_level.get_f32();
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    
    let x = shield_level / shield_max;
    let k = -0.95 as f32;
    let interpolated = (x - x * k) / (k - x.abs() * 2.0 * k + 1.0) as f32;

    println!("shield health: {}", x);
    println!("shield scale: {}", interpolated);

    let shield_size = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_size"));
    let shield_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_scale"), 0);
    let shield_scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_scale_min"));
    let shield_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);

    L2CValue::F32((interpolated * shield_size * shield_scale * (1.0 - shield_scale_min) + shield_scale_min) * shield_radius)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon40FighterStatusGuard__check_hit_stop_delayEN3lib8L2CValueE")]
pub unsafe fn check_hit_stop_delay(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if !arg.get_bool() { return false.into(); }
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor).abs();
    let hit_stop_delay_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_stick"));
    if hit_stop_delay_stick <= stick_x {
        let mut pos = *PostureModule::pos(fighter.module_accessor);
        let auto_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_stick_auto_mul"));
        let delay_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_DELAY_MUL);
        pos.x += stick_x * auto_mul * delay_mul;
        PostureModule::set_pos(fighter.module_accessor, &pos);
        true.into()
    } else {
        false.into()
    }
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon46FighterStatusGuard__check_hit_stop_delay_flickEN3lib8L2CValueE")]
pub unsafe fn check_hit_stop_delay_flick(fighter: &mut L2CFighterCommon, user_mul: L2CValue) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor).abs();
    let sub_x = ControlModule::get_flick_sub_x(fighter.module_accessor) as f32;
    let hit_stop_delay_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_stick"));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_DISABLE_HIT_STOP_DELAY_STICK)
        && StopModule::is_hit(fighter.module_accessor) 
        && sub_x < hit_stop_delay_stick
        && hit_stop_delay_stick <= stick_x {
        let mut pos = *PostureModule::pos(fighter.module_accessor);
        let flick_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_flick_mul"));
        let guard_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_guard_mul"));
        let user_mul = WorkModule::get_float(fighter.module_accessor, user_mul.get_i32());
        pos.x += stick_x * flick_mul * guard_mul * user_mul;
        PostureModule::set_pos(fighter.module_accessor, &pos);
        ControlModule::reset_flick_sub_x(fighter.module_accessor);
        true.into()
    } else {
        false.into()
    }

}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon49FighterStatusGuard__is_continue_just_shield_countEv")]
pub unsafe fn is_continue_just_shield_count(fighter: &mut L2CFighterCommon) -> L2CValue {
    let just_shield_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_SHEILD_COUNT);
    let max_count = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("continue_just_shield_count"));
    L2CValue::Bool(just_shield_count <= max_count)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusGuard__landing_effect_controlEv")]
pub unsafe fn landing_effect_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_LANDING_EFFECT_FRAME);
    let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_LANDING_EFFECT_FRAME);
    if frame <= 0 {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_guardlandingeffect"), -1);
        WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_STATUS_GUARD_ON_WORK_INT_LANDING_EFFECT_FRAME);
    }
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusGuard__set_guard_blend_motionEN3lib8L2CValueES2_S2_S2_S2_")]
pub unsafe fn set_guard_blend_motion(fighter: &mut L2CFighterCommon, delta_x: L2CValue, delta_y: L2CValue, stick_x: L2CValue, stick_y: L2CValue, flag: L2CValue) -> L2CValue {
    let magnitude = stick_x.get_f32().powi(2) + stick_y.get_f32().powi(2);
    let magnitude = magnitude.sqrt().min(1.0);
    if flag.get_bool() {
        MotionModule::set_weight(fighter.module_accessor, 1.0 - magnitude, true);
    } else {
        let prev_weight = MotionModule::prev_weight(fighter.module_accessor);
        MotionModule::set_weight_rate(fighter.module_accessor, 1.0 - magnitude - prev_weight);
    }
    set_guard_blend_motion_angle(fighter, delta_x, delta_y);
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon48FighterStatusGuard__set_guard_blend_motion_angleEN3lib8L2CValueES2_")]
pub unsafe fn set_guard_blend_motion_angle(fighter: &mut L2CFighterCommon, delta_x: L2CValue, delta_y: L2CValue) -> L2CValue {
    let delta_x = delta_x.get_f32();
    let delta_y = delta_y.get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let angle = delta_y.atan2(delta_x * lr).to_degrees();
    let angle = if angle >= 360.0 {
        angle - 360.0
    } else if angle < 0.0 {
        angle + 360.0
    } else {
        angle
    };
    MotionModule::set_frame_2nd(fighter.module_accessor, angle, true);
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon41FighterStatusGuard__set_just_shield_scaleEv")]
pub unsafe fn set_just_shield_scale(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield_size = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_size"));
    let shield_scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_scale_min"));
    let shield_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);

    let scale = ((1.0 - shield_scale_min) * shield_size + shield_scale_min) * shield_radius;
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f { x: scale, y: scale, z: scale});
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon36FighterStatusGuard__set_shield_scaleEN3lib8L2CValueE")]
pub unsafe fn set_shield_scale(fighter: &mut L2CFighterCommon, scale: L2CValue) -> L2CValue {
    let scale = scale.get_f32();
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f { x: scale, y: scale, z: scale});
    L2CValue::I32(0)
}

pub fn install() {
    install_hooks!(
        calc_shield_scale
    );
}