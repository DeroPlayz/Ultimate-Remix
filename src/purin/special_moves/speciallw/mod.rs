use smash::{app::{lua_bind::AttackModule, sv_animcmd::{frame, wait}}, lib::lua_const::{ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_NONE, ATTACK_REGION_NONE, COLLISION_SITUATION_MASK_GA}};
use smash_script::macros;
use smashline::{L2CAgentBase, Hash40, Agent};
use smashline::Priority::*;

unsafe extern "C" fn game_specialairlwl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 20.0, 270, 66, 0, 100, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn game_specialairlwr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 20.0, 270, 66, 0, 100, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

pub fn install() {
    Agent::new("purin")
        .game_acmd("game_specialhisub", game_specialhisub, Default)
        .game_acmd("game_specialhil", game_specialhil, Default)
        .game_acmd("game_specialairhil", game_specialairhil, Default)
        .game_acmd("game_specialhir", game_specialhir, Default)
        .game_acmd("game_specialairhir", game_specialairhir, Default)
        .install();
}