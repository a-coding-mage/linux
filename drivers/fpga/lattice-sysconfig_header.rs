/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: __LATTICE_SYSCONFIG_H */

pub const SYSCONFIG_ISC_ENABLE: [u8; 4] = [0xC6, 0x00, 0x00, 0x00];
pub const SYSCONFIG_ISC_DISABLE: [u8; 4] = [0x26, 0x00, 0x00, 0x00];
pub const SYSCONFIG_ISC_ERASE: [u8; 4] = [0x0E, 0x01, 0x00, 0x00];
pub const SYSCONFIG_LSC_READ_STATUS: [u8; 4] = [0x3C, 0x00, 0x00, 0x00];
pub const SYSCONFIG_LSC_CHECK_BUSY: [u8; 4] = [0xF0, 0x00, 0x00, 0x00];
pub const SYSCONFIG_LSC_REFRESH: [u8; 4] = [0x79, 0x00, 0x00, 0x00];
pub const SYSCONFIG_LSC_INIT_ADDR: [u8; 4] = [0x46, 0x00, 0x00, 0x00];
pub const SYSCONFIG_LSC_BITSTREAM_BURST: [u8; 4] = [0x7a, 0x00, 0x00, 0x00];

/* BIT and GENMASK are supplied by the surrounding dependencies. */
pub const SYSCONFIG_STATUS_DONE: u32 = BIT(8);
pub const SYSCONFIG_STATUS_BUSY: u32 = BIT(12);
pub const SYSCONFIG_STATUS_FAIL: u32 = BIT(13);
pub const SYSCONFIG_STATUS_ERR: u32 = GENMASK(25, 23);

pub const SYSCONFIG_POLL_INTERVAL_US: u32 = 30;
pub const SYSCONFIG_POLL_BUSY_TIMEOUT_US: u32 = 1000000;
pub const SYSCONFIG_POLL_GPIO_TIMEOUT_US: u32 = 100000;

#[repr(C)]
pub struct sysconfig_priv {
    pub program: *mut gpio_desc,
    pub init: *mut gpio_desc,
    pub done: *mut gpio_desc,
    pub dev: *mut device,
    pub command_transfer: Option<unsafe extern "C" fn(
        priv_: *mut sysconfig_priv,
        tx_buf: *const core::ffi::c_void,
        tx_len: usize,
        rx_buf: *mut core::ffi::c_void,
        rx_len: usize,
    ) -> i32>,
    pub bitstream_burst_write_init:
        Option<unsafe extern "C" fn(priv_: *mut sysconfig_priv) -> i32>,
    pub bitstream_burst_write: Option<unsafe extern "C" fn(
        priv_: *mut sysconfig_priv,
        tx_buf: *const core::ffi::c_char,
        tx_len: usize,
    ) -> i32>,
    pub bitstream_burst_write_complete:
        Option<unsafe extern "C" fn(priv_: *mut sysconfig_priv) -> i32>,
}

unsafe extern "C" {
    pub fn sysconfig_probe(priv_: *mut sysconfig_priv) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
