/* SPDX-License-Identifier: GPL-2.0 */
/*
 * access the core module control register.
 */
extern "C" {
    pub fn cm_get() -> u32;
    pub fn cm_control(_: u32, _: u32);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn cm_init();
    pub fn cm_clear_irqs();
}

pub const CM_CTRL_LED: u32 = 1 << 0;
pub const CM_CTRL_nMBDET: u32 = 1 << 1;
pub const CM_CTRL_REMAP: u32 = 1 << 2;

/*
 * Integrator/AP,PP2 specific
 */
pub const CM_CTRL_HIGHVECTORS: u32 = 1 << 4;
pub const CM_CTRL_BIGENDIAN: u32 = 1 << 5;
pub const CM_CTRL_FASTBUS: u32 = 1 << 6;
pub const CM_CTRL_SYNC: u32 = 1 << 7;

/*
 * ARM926/946/966 Integrator/CP specific
 */
pub const CM_CTRL_LCDBIASEN: u32 = 1 << 8;
pub const CM_CTRL_LCDBIASUP: u32 = 1 << 9;
pub const CM_CTRL_LCDBIASDN: u32 = 1 << 10;
pub const CM_CTRL_LCDMUXSEL_MASK: u32 = 7 << 11;
pub const CM_CTRL_LCDMUXSEL_GENLCD: u32 = 1 << 11;
pub const CM_CTRL_LCDMUXSEL_VGA565_TFT555: u32 = 2 << 11;
pub const CM_CTRL_LCDMUXSEL_SHARPLCD: u32 = 3 << 11;
pub const CM_CTRL_LCDMUXSEL_VGA555_TFT555: u32 = 4 << 11;
pub const CM_CTRL_LCDEN0: u32 = 1 << 14;
pub const CM_CTRL_LCDEN1: u32 = 1 << 15;
pub const CM_CTRL_STATIC1: u32 = 1 << 16;
pub const CM_CTRL_STATIC2: u32 = 1 << 17;
pub const CM_CTRL_STATIC: u32 = 1 << 18;
pub const CM_CTRL_n24BITEN: u32 = 1 << 19;
pub const CM_CTRL_EBIWP: u32 = 1 << 20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
