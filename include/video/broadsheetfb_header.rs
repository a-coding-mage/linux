/*
 * broadsheetfb.h - definitions for the broadsheet framebuffer driver
 *
 * Copyright (C) 2008 by Jaya Kumar
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file COPYING in the main directory of this archive for
 * more details.
 *
 */

/* Broadsheet command defines */
pub const BS_CMD_INIT_SYS_RUN: u8 = 0x06;
pub const BS_CMD_INIT_DSPE_CFG: u8 = 0x09;
pub const BS_CMD_INIT_DSPE_TMG: u8 = 0x0A;
pub const BS_CMD_INIT_ROTMODE: u8 = 0x0B;
pub const BS_CMD_RD_REG: u8 = 0x10;
pub const BS_CMD_WR_REG: u8 = 0x11;
pub const BS_CMD_LD_IMG: u8 = 0x20;
pub const BS_CMD_LD_IMG_AREA: u8 = 0x22;
pub const BS_CMD_LD_IMG_END: u8 = 0x23;
pub const BS_CMD_WAIT_DSPE_TRG: u8 = 0x28;
pub const BS_CMD_WAIT_DSPE_FREND: u8 = 0x29;
pub const BS_CMD_RD_WFM_INFO: u8 = 0x30;
pub const BS_CMD_UPD_INIT: u8 = 0x32;
pub const BS_CMD_UPD_FULL: u8 = 0x33;
pub const BS_CMD_UPD_GDRV_CLR: u8 = 0x37;

/* Broadsheet register interface defines */
pub const BS_REG_REV: u8 = 0x00;
pub const BS_REG_PRC: u8 = 0x02;

/* Broadsheet pin interface specific defines */
pub const BS_CS: u8 = 0x01;
pub const BS_DC: u8 = 0x02;
pub const BS_WR: u8 = 0x03;

/* Broadsheet IO interface specific defines */
pub const BS_MMIO_CMD: u8 = 0x01;
pub const BS_MMIO_DATA: u8 = 0x02;

/* External types supplied by the surrounding kernel code. */
pub enum fb_info {}
pub enum module {}
pub enum wait_queue_head_t {}
pub enum mutex {}

/* struct used by broadsheet. board specific stuff comes from *board */
#[repr(C)]
pub struct broadsheetfb_par {
    pub info: *mut fb_info,
    pub board: *mut broadsheet_board,
    pub write_reg: Option<unsafe extern "C" fn(*mut broadsheetfb_par, u16, u16)>,
    pub read_reg: Option<unsafe extern "C" fn(*mut broadsheetfb_par, u16) -> u16>,
    pub waitq: wait_queue_head_t,
    pub panel_index: ::core::ffi::c_int,
    pub io_lock: mutex,
}

/* board specific routines */
#[repr(C)]
pub struct broadsheet_board {
    pub owner: *mut module,
    pub init: Option<unsafe extern "C" fn(*mut broadsheetfb_par) -> ::core::ffi::c_int>,
    pub wait_for_rdy:
        Option<unsafe extern "C" fn(*mut broadsheetfb_par) -> ::core::ffi::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut broadsheetfb_par)>,
    pub get_panel_type: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub setup_irq: Option<unsafe extern "C" fn(*mut fb_info) -> ::core::ffi::c_int>,

    /* Functions for boards that use GPIO */
    pub set_ctl: Option<unsafe extern "C" fn(*mut broadsheetfb_par, u8, u8)>,
    pub set_hdb: Option<unsafe extern "C" fn(*mut broadsheetfb_par, u16)>,
    pub get_hdb: Option<unsafe extern "C" fn(*mut broadsheetfb_par) -> u16>,

    /* Functions for boards that have specialized MMIO */
    pub mmio_write: Option<unsafe extern "C" fn(*mut broadsheetfb_par, ::core::ffi::c_int, u16)>,
    pub mmio_read: Option<unsafe extern "C" fn(*mut broadsheetfb_par) -> u16>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
