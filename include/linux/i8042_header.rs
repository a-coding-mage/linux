/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding translation unit are intentionally
// left external, matching the original header's included kernel definitions.

/* Standard commands. */
pub const I8042_CMD_CTL_RCTR: u16 = 0x0120;
pub const I8042_CMD_CTL_WCTR: u16 = 0x1060;
pub const I8042_CMD_CTL_TEST: u16 = 0x01aa;

pub const I8042_CMD_KBD_DISABLE: u16 = 0x00ad;
pub const I8042_CMD_KBD_ENABLE: u16 = 0x00ae;
pub const I8042_CMD_KBD_TEST: u16 = 0x01ab;
pub const I8042_CMD_KBD_LOOP: u16 = 0x11d2;

pub const I8042_CMD_AUX_DISABLE: u16 = 0x00a7;
pub const I8042_CMD_AUX_ENABLE: u16 = 0x00a8;
pub const I8042_CMD_AUX_TEST: u16 = 0x01a9;
pub const I8042_CMD_AUX_SEND: u16 = 0x10d4;
pub const I8042_CMD_AUX_LOOP: u16 = 0x11d3;

pub const I8042_CMD_MUX_PFX: u16 = 0x0090;
pub const I8042_CMD_MUX_SEND: u16 = 0x1090;

/* Status register bits. */
pub const I8042_STR_PARITY: u8 = 0x80;
pub const I8042_STR_TIMEOUT: u8 = 0x40;
pub const I8042_STR_AUXDATA: u8 = 0x20;
pub const I8042_STR_KEYLOCK: u8 = 0x10;
pub const I8042_STR_CMDDAT: u8 = 0x08;
pub const I8042_STR_MUXERR: u8 = 0x04;
pub const I8042_STR_IBF: u8 = 0x02;
pub const I8042_STR_OBF: u8 = 0x01;

/* Control register bits. */
pub const I8042_CTR_KBDINT: u8 = 0x01;
pub const I8042_CTR_AUXINT: u8 = 0x02;
pub const I8042_CTR_IGNKEYLOCK: u8 = 0x08;
pub const I8042_CTR_KBDDIS: u8 = 0x10;
pub const I8042_CTR_AUXDIS: u8 = 0x20;
pub const I8042_CTR_XLATE: u8 = 0x40;

#[repr(C)]
pub struct serio {
    _private: [u8; 0],
}

/**
 * i8042 filter callback.
 *
 * Data and status are received from the i8042 controller; `serio` identifies
 * the controller and `context` is the callback's associated context pointer.
 * The callback returns true when the data should be filtered out.
 */
pub type i8042_filter_t = unsafe extern "C" fn(
    data: u8,
    str_: u8,
    serio: *mut serio,
    context: *mut core::ffi::c_void,
) -> bool;

// CONFIG_SERIO_I8042 or CONFIG_SERIO_I8042_MODULE selects the external
// controller implementation. Otherwise the original header supplies the
// inline ENODEV stubs below.
#[cfg(any(CONFIG_SERIO_I8042, CONFIG_SERIO_I8042_MODULE))]
extern "C" {
    pub fn i8042_lock_chip();
    pub fn i8042_unlock_chip();
    pub fn i8042_command(param: *mut u8, command: core::ffi::c_int) -> core::ffi::c_int;
    pub fn i8042_install_filter(
        filter: i8042_filter_t,
        context: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn i8042_remove_filter(filter: i8042_filter_t) -> core::ffi::c_int;
}

#[cfg(not(any(CONFIG_SERIO_I8042, CONFIG_SERIO_I8042_MODULE)))]
pub unsafe fn i8042_lock_chip() {}

#[cfg(not(any(CONFIG_SERIO_I8042, CONFIG_SERIO_I8042_MODULE)))]
pub unsafe fn i8042_unlock_chip() {}

#[cfg(not(any(CONFIG_SERIO_I8042, CONFIG_SERIO_I8042_MODULE)))]
pub unsafe fn i8042_command(
    _param: *mut u8,
    _command: core::ffi::c_int,
) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(any(CONFIG_SERIO_I8042, CONFIG_SERIO_I8042_MODULE)))]
pub unsafe fn i8042_install_filter(
    _filter: i8042_filter_t,
    _context: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(any(CONFIG_SERIO_I8042, CONFIG_SERIO_I8042_MODULE)))]
pub unsafe fn i8042_remove_filter(_filter: i8042_filter_t) -> core::ffi::c_int {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
