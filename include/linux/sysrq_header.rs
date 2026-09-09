/* SPDX-License-Identifier: GPL-2.0 */
/* -*- linux-c -*-
 *
 *	$Id: sysrq.h,v 1.3 1997/07/17 11:54:33 mj Exp $
 *
 *	Linux Magic System Request Key Hacks
 *
 *	(c) 1997 Martin Mares <mj@atrey.karlin.mff.cuni.cz>
 *
 *	(c) 2000 Crutcher Dunnavant <crutcher+kernel@datastacks.com>
 *	overhauled to use key registration
 *	based upon discusions in irc://irc.openprojects.net/#kernelnewbies
 */

// Dependencies supplied by other translated files: errno, u8, and bool.

/* Possible values of bitmask for enabling sysrq functions */
/* 0x0001 is reserved for enable everything */
pub const SYSRQ_ENABLE_LOG: i32 = 0x0002;
pub const SYSRQ_ENABLE_KEYBOARD: i32 = 0x0004;
pub const SYSRQ_ENABLE_DUMP: i32 = 0x0008;
pub const SYSRQ_ENABLE_SYNC: i32 = 0x0010;
pub const SYSRQ_ENABLE_REMOUNT: i32 = 0x0020;
pub const SYSRQ_ENABLE_SIGNAL: i32 = 0x0040;
pub const SYSRQ_ENABLE_BOOT: i32 = 0x0080;
pub const SYSRQ_ENABLE_RTNICE: i32 = 0x0100;

#[repr(C)]
pub struct sysrq_key_op {
    pub handler: Option<unsafe extern "C" fn(u8)>,
    pub help_msg: *const core::ffi::c_char,
    pub action_msg: *const core::ffi::c_char,
    pub enable_mask: i32,
}

/* Generic SysRq interface -- you may call it from any device driver, supplying
 * ASCII code of the key, pointer to registers and kbd/tty structs (if they
 * are available -- else NULL's).
 */

#[cfg(CONFIG_MAGIC_SYSRQ)]
extern "C" {
    pub fn handle_sysrq(key: u8);
    pub fn __handle_sysrq(key: u8, check_mask: bool);
    pub fn register_sysrq_key(key: u8, op: *const sysrq_key_op) -> i32;
    pub fn unregister_sysrq_key(key: u8, op: *const sysrq_key_op) -> i32;
    pub static __sysrq_reboot_op: *const sysrq_key_op;

    pub fn sysrq_toggle_support(enable_mask: i32) -> i32;
    pub fn sysrq_mask() -> i32;
}

#[cfg(not(CONFIG_MAGIC_SYSRQ))]
#[inline]
pub unsafe fn handle_sysrq(_key: u8) {}

#[cfg(not(CONFIG_MAGIC_SYSRQ))]
#[inline]
pub unsafe fn __handle_sysrq(_key: u8, _check_mask: bool) {}

#[cfg(not(CONFIG_MAGIC_SYSRQ))]
#[inline]
pub unsafe fn register_sysrq_key(_key: u8, _op: *const sysrq_key_op) -> i32 {
    -EINVAL
}

#[cfg(not(CONFIG_MAGIC_SYSRQ))]
#[inline]
pub unsafe fn unregister_sysrq_key(_key: u8, _op: *const sysrq_key_op) -> i32 {
    -EINVAL
}

#[cfg(not(CONFIG_MAGIC_SYSRQ))]
#[inline]
pub unsafe fn sysrq_mask() -> i32 {
    /* Magic SysRq disabled mask */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
