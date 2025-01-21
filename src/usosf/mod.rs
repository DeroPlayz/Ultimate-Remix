// USOSF originally stood for Up-Special-Only-Special-Fall, but now I'm planning to make every special fall move act like Snake's up special; you can act out of it, but you can't do it again until you flinch/hit the ground.

use smash::{lib::lua_const::{FIGHTER_STATUS_KIND_SPECIAL_N, FIGHTER_STATUS_KIND_FALL, FIGHTER_STATUS_KIND_SPECIAL_S, FIGHTER_STATUS_KIND_SPECIAL_LW, FIGHTER_STATUS_KIND_SPECIAL_HI, FIGHTER_STATUS_KIND_FALL_SPECIAL, SITUATION_KIND_GROUND, SITUATION_KIND_AIR}, app::lua_bind::StatusModule};
use smashline::{L2CFighterCommon, Agent, Main};

unsafe extern "C" fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        // static mut CAN_B: bool = true;
        // static mut CAN_SB: bool = true;
        // static mut CAN_UB: bool = true;
        // static mut CAN_DB: bool = true;        
        
        let mut i: u32 = 0;        
        while i < 5{
            // if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N && CAN_B == false && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR{
            //     StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            // }
            // if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S && CAN_SB == false && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR{
            //     StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            // }
            // if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI && CAN_UB == false && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR{
            //     StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            // }
            // if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW && CAN_DB == false && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR{
            //     StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            // }
            // if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND{CAN_B = true;   CAN_SB = true;  CAN_UB = true;  CAN_DB = true;}

            if StatusModule::prev_status_kind(fighter.module_accessor, i) == *FIGHTER_STATUS_KIND_SPECIAL_N{
                if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL{
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    // CAN_B = false;
                }
            }
            if StatusModule::prev_status_kind(fighter.module_accessor, i) == *FIGHTER_STATUS_KIND_SPECIAL_S{
                if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL{
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    // CAN_SB = false;
                }
            }
            if StatusModule::prev_status_kind(fighter.module_accessor, i) == *FIGHTER_STATUS_KIND_SPECIAL_HI{
                if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL{
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    // CAN_UB = false;
                }
            }
            if StatusModule::prev_status_kind(fighter.module_accessor, i) == *FIGHTER_STATUS_KIND_SPECIAL_LW{
                if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL{
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    // CAN_DB = false;
                }
            }
            i = i + 1;
        }
    }
}

pub fn install() {
    Agent::new("fighter")
        .on_line(Main, global_fighter_frame)
        .install();
}
