/*
 * hecubafb.h - definitions for the hecuba framebuffer driver
 *
 * Copyright (C) 2008 by Jaya Kumar
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file COPYING in the main directory of this archive for
 * more details.
 */

/* Apollo controller specific defines */
pub const APOLLO_START_NEW_IMG: u8 = 0xA0;
pub const APOLLO_STOP_IMG_DATA: u8 = 0xA1;
pub const APOLLO_DISPLAY_IMG: u8 = 0xA2;
pub const APOLLO_ERASE_DISPLAY: u8 = 0xA3;
pub const APOLLO_INIT_DISPLAY: u8 = 0xA4;

/* Hecuba interface specific defines */
pub const HCB_WUP_BIT: u8 = 0x01;
pub const HCB_DS_BIT: u8 = 0x02;
pub const HCB_RW_BIT: u8 = 0x04;
pub const HCB_CD_BIT: u8 = 0x08;
pub const HCB_ACK_BIT: u8 = 0x80;

/* External types supplied by the framebuffer and module subsystems. */
#[repr(C)]
pub struct fb_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hecubafb_par {
    pub info: *mut fb_info,
    pub board: *mut hecuba_board,
    pub send_command: Option<unsafe extern "C" fn(*mut hecubafb_par, u8)>,
    pub send_data: Option<unsafe extern "C" fn(*mut hecubafb_par, u8)>,
}

/*
 * Board specific routines.
 * Board drivers can implement wait_for_ack with interrupts if desired. If
 * wait_for_ack is called with clear=0, then go to sleep and return when ack
 * goes high; with clear=1, return when ack goes low.
 */
#[repr(C)]
pub struct hecuba_board {
    pub owner: *mut module,
    pub remove: Option<unsafe extern "C" fn(*mut hecubafb_par)>,
    pub set_ctl: Option<unsafe extern "C" fn(*mut hecubafb_par, u8, u8)>,
    pub set_data: Option<unsafe extern "C" fn(*mut hecubafb_par, u8)>,
    pub wait_for_ack: Option<unsafe extern "C" fn(*mut hecubafb_par, i32)>,
    pub init: Option<unsafe extern "C" fn(*mut hecubafb_par) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
