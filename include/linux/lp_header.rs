/* SPDX-License-Identifier: GPL-2.0 */
/*
 * usr/include/linux/lp.h c.1991-1992 James Wiegand
 * many modifications copyright (C) 1992 Michael K. Johnson
 * Interrupt support added 1993 Nigel Gamble
 * Removed 8255 status defines from inside __KERNEL__ Marcelo Tosatti
 */

// Dependencies supplied by the surrounding kernel translation.

/* Magic numbers for defining port-device mappings */
pub const LP_PARPORT_UNSPEC: i32 = -4;
pub const LP_PARPORT_AUTO: i32 = -3;
pub const LP_PARPORT_OFF: i32 = -2;
pub const LP_PARPORT_NONE: i32 = -1;

macro_rules! LP_F {
    ($minor:expr) => { unsafe { lp_table[($minor)].flags } };
}
macro_rules! LP_CHAR {
    ($minor:expr) => { unsafe { lp_table[($minor)].chars } };
}
macro_rules! LP_TIME {
    ($minor:expr) => { unsafe { lp_table[($minor)].time } };
}
macro_rules! LP_WAIT {
    ($minor:expr) => { unsafe { lp_table[($minor)].wait } };
}
macro_rules! LP_IRQ {
    ($minor:expr) => { unsafe { (*(*lp_table[($minor)].dev).port).irq } };
}
/* PARPORT_IRQ_NONE means polled */
#[cfg(feature = "LP_STATS")]
macro_rules! LP_STAT {
    ($minor:expr) => { unsafe { lp_table[($minor)].stats } };
}

pub const LP_BUFFER_SIZE: usize = PAGE_SIZE;

macro_rules! LP_BASE {
    ($x:expr) => { unsafe { (*(*lp_table[($x)].dev).port).base } };
}

#[cfg(feature = "LP_STATS")]
#[repr(C)]
pub struct lp_stats {
    pub chars: ::core::ffi::c_ulong,
    pub sleeps: ::core::ffi::c_ulong,
    pub maxrun: ::core::ffi::c_uint,
    pub maxwait: ::core::ffi::c_uint,
    pub meanwait: ::core::ffi::c_uint,
    pub mdev: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct lp_struct {
    pub dev: *mut pardevice,
    pub flags: ::core::ffi::c_ulong,
    pub chars: ::core::ffi::c_uint,
    pub time: ::core::ffi::c_uint,
    pub wait: ::core::ffi::c_uint,
    pub lp_buffer: *mut ::core::ffi::c_char,
    #[cfg(feature = "LP_STATS")]
    pub lastcall: ::core::ffi::c_uint,
    #[cfg(feature = "LP_STATS")]
    pub runchars: ::core::ffi::c_uint,
    #[cfg(feature = "LP_STATS")]
    pub stats: lp_stats,
    pub waitq: wait_queue_head_t,
    pub last_error: ::core::ffi::c_uint,
    pub port_mutex: mutex,
    pub dataq: wait_queue_head_t,
    pub timeout: ::core::ffi::c_long,
    pub best_mode: ::core::ffi::c_uint,
    pub current_mode: ::core::ffi::c_uint,
    pub bits: ::core::ffi::c_ulong,
}

/*
 * The following constants describe the various signals of the printer port
 * hardware.  Note that the hardware inverts some signals and that some
 * signals are active low.  An example is LP_STROBE, which must be programmed
 * with 1 for being active and 0 for being inactive, because the strobe signal
 * gets inverted, but it is also active low.
 */

/*
 * defines for 8255 control port
 * base + 2
 * accessed with LP_C(minor)
 */
pub const LP_PINTEN: u32 = 0x10;
pub const LP_PSELECP: u32 = 0x08;
pub const LP_PINITP: u32 = 0x04;
pub const LP_PAUTOLF: u32 = 0x02;
pub const LP_PSTROBE: u32 = 0x01;

/*
 * the value written to ports to test existence. PC-style ports will
 * return the value written. AT-style ports will return 0. so why not
 * make them the same ?
 */
pub const LP_DUMMY: u32 = 0x00;

/*
 * This is the port delay time, in microseconds.
 * It is used only in the lp_init() and lp_reset() routine.
 */
pub const LP_DELAY: u32 = 50;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
