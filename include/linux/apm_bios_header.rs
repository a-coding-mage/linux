/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Include file for the interface to an APM BIOS
 * Copyright 1994-2001 Stephen Rothwell (sfr@canb.auug.org.au)
 */

/* C dependency: <uapi/linux/apm_bios.h> */

pub const APM_CS: u32 = GDT_ENTRY_APMBIOS_BASE * 8;
pub const APM_CS_16: u32 = APM_CS + 8;
pub const APM_DS: u32 = APM_CS_16 + 8;

/* Results of APM Installation Check */
pub const APM_16_BIT_SUPPORT: u16 = 0x0001;
pub const APM_32_BIT_SUPPORT: u16 = 0x0002;
pub const APM_IDLE_SLOWS_CLOCK: u16 = 0x0004;
pub const APM_BIOS_DISABLED: u16 = 0x0008;
pub const APM_BIOS_DISENGAGED: u16 = 0x0010;

/*
 * Data for APM that is persistent across module unload/load
 */
#[repr(C)]
pub struct apm_info {
    pub bios: apm_bios_info,
    pub connection_version: u16,
    pub get_power_status_broken: i32,
    pub get_power_status_swabinminutes: i32,
    pub allow_ints: i32,
    pub forbid_idle: i32,
    pub realmode_power_off: i32,
    pub disabled: i32,
}

/*
 * The APM function codes
 */
pub const APM_FUNC_INST_CHECK: u16 = 0x5300;
pub const APM_FUNC_REAL_CONN: u16 = 0x5301;
pub const APM_FUNC_16BIT_CONN: u16 = 0x5302;
pub const APM_FUNC_32BIT_CONN: u16 = 0x5303;
pub const APM_FUNC_DISCONN: u16 = 0x5304;
pub const APM_FUNC_IDLE: u16 = 0x5305;
pub const APM_FUNC_BUSY: u16 = 0x5306;
pub const APM_FUNC_SET_STATE: u16 = 0x5307;
pub const APM_FUNC_ENABLE_PM: u16 = 0x5308;
pub const APM_FUNC_RESTORE_BIOS: u16 = 0x5309;
pub const APM_FUNC_GET_STATUS: u16 = 0x530a;
pub const APM_FUNC_GET_EVENT: u16 = 0x530b;
pub const APM_FUNC_GET_STATE: u16 = 0x530c;
pub const APM_FUNC_ENABLE_DEV_PM: u16 = 0x530d;
pub const APM_FUNC_VERSION: u16 = 0x530e;
pub const APM_FUNC_ENGAGE_PM: u16 = 0x530f;
pub const APM_FUNC_GET_CAP: u16 = 0x5310;
pub const APM_FUNC_RESUME_TIMER: u16 = 0x5311;
pub const APM_FUNC_RESUME_ON_RING: u16 = 0x5312;
pub const APM_FUNC_TIMER: u16 = 0x5313;

/* Function code for APM_FUNC_RESUME_TIMER */
pub const APM_FUNC_DISABLE_TIMER: u32 = 0;
pub const APM_FUNC_GET_TIMER: u32 = 1;
pub const APM_FUNC_SET_TIMER: u32 = 2;

/* Function code for APM_FUNC_RESUME_ON_RING */
pub const APM_FUNC_DISABLE_RING: u32 = 0;
pub const APM_FUNC_ENABLE_RING: u32 = 1;
pub const APM_FUNC_GET_RING: u32 = 2;

/* Function code for APM_FUNC_TIMER_STATUS */
pub const APM_FUNC_TIMER_DISABLE: u32 = 0;
pub const APM_FUNC_TIMER_ENABLE: u32 = 1;
pub const APM_FUNC_TIMER_GET: u32 = 2;

/* in arch/i386/kernel/setup.c */
extern "C" {
    pub static mut apm_info: apm_info;
}

/* This is the "All Devices" ID communicated to the BIOS */
pub unsafe fn APM_DEVICE_BALL() -> u32 {
    if apm_info.connection_version > 0x0100 {
        APM_DEVICE_ALL
    } else {
        APM_DEVICE_OLD_ALL
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
