/*
 * Retu/Tahvo MFD driver interface
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License. See the file "COPYING" in the main directory of this
 * archive for more details.
 */

// C header guard: __LINUX_MFD_RETU_H

#[repr(C)]
pub struct retu_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn retu_read(dev: *mut retu_dev, reg: u8) -> i32;
    pub fn retu_write(dev: *mut retu_dev, reg: u8, value: u16) -> i32;
}

/* Registers */
pub const RETU_REG_WATCHDOG: u8 = 0x17; /* Watchdog */
pub const RETU_REG_CC1: u8 = 0x0d; /* Common control register 1 */
pub const RETU_REG_STATUS: u8 = 0x16; /* Status register */

/* Interrupt sources */
pub const TAHVO_INT_VBUS: i32 = 0; /* VBUS state */

/* Interrupt status */
pub const TAHVO_STAT_VBUS: i32 = 1 << TAHVO_INT_VBUS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
