/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _LINUX_OLPC_EC_H */
/* C dependency: <linux/bits.h> */

/* XO-1 EC commands */
pub const EC_FIRMWARE_REV: u8 = 0x08;
pub const EC_WRITE_SCI_MASK: u8 = 0x1b;
pub const EC_WAKE_UP_WLAN: u8 = 0x24;
pub const EC_WLAN_LEAVE_RESET: u8 = 0x25;
pub const EC_DCON_POWER_MODE: u8 = 0x26;
pub const EC_READ_EB_MODE: u8 = 0x2a;
pub const EC_SET_SCI_INHIBIT: u8 = 0x32;
pub const EC_SET_SCI_INHIBIT_RELEASE: u8 = 0x34;
pub const EC_WLAN_ENTER_RESET: u8 = 0x35;
pub const EC_WRITE_EXT_SCI_MASK: u8 = 0x38;
pub const EC_SCI_QUERY: u8 = 0x84;
pub const EC_EXT_SCI_QUERY: u8 = 0x85;

/* SCI source values */
pub const EC_SCI_SRC_GAME: u16 = 1u16 << 0;
pub const EC_SCI_SRC_BATTERY: u16 = 1u16 << 1;
pub const EC_SCI_SRC_BATSOC: u16 = 1u16 << 2;
pub const EC_SCI_SRC_BATERR: u16 = 1u16 << 3;
pub const EC_SCI_SRC_EBOOK: u16 = 1u16 << 4; // XO-1 only
pub const EC_SCI_SRC_WLAN: u16 = 1u16 << 5; // XO-1 only
pub const EC_SCI_SRC_ACPWR: u16 = 1u16 << 6;
pub const EC_SCI_SRC_BATCRIT: u16 = 1u16 << 7;
pub const EC_SCI_SRC_GPWAKE: u16 = 1u16 << 8; // XO-1.5 only
pub const EC_SCI_SRC_ALL: u16 = (1u16 << (8 + 1)) - 1;

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct olpc_ec_driver {
    pub suspend: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub ec_cmd: Option<
        unsafe extern "C" fn(u8, *mut u8, usize, *mut u8, usize, *mut core::ffi::c_void) -> i32,
    >,
    pub wakeup_available: bool,
}

/* CONFIG_OLPC_EC conditional declarations. */
#[cfg(feature = "CONFIG_OLPC_EC")]
extern "C" {
    pub fn olpc_ec_driver_register(drv: *mut olpc_ec_driver, arg: *mut core::ffi::c_void);

    pub fn olpc_ec_cmd(
        cmd: u8,
        inbuf: *mut u8,
        inlen: usize,
        outbuf: *mut u8,
        outlen: usize,
    ) -> i32;

    pub fn olpc_ec_wakeup_set(value: u16);
    pub fn olpc_ec_wakeup_clear(value: u16);

    pub fn olpc_ec_mask_write(bits: u16) -> i32;
    pub fn olpc_ec_sci_query(sci_value: *mut u16) -> i32;

    pub fn olpc_ec_wakeup_available() -> bool;

    /* C calling convention attribute: asmlinkage */
    pub fn xo1_do_sleep(sleep_state: u8) -> i32;
}

/* CONFIG_OLPC_EC disabled branch. ENODEV is supplied by the kernel environment. */
#[cfg(not(feature = "CONFIG_OLPC_EC"))]
pub unsafe fn olpc_ec_cmd(
    _cmd: u8,
    _inbuf: *mut u8,
    _inlen: usize,
    _outbuf: *mut u8,
    _outlen: usize,
) -> i32 {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_OLPC_EC"))]
pub unsafe fn olpc_ec_wakeup_set(_value: u16) {}

#[cfg(not(feature = "CONFIG_OLPC_EC"))]
pub unsafe fn olpc_ec_wakeup_clear(_value: u16) {}

#[cfg(not(feature = "CONFIG_OLPC_EC"))]
pub unsafe fn olpc_ec_wakeup_available() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
