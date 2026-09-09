/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __MACH_SSI_H

#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut imx_ssi_fiq_start: u8;
    pub static mut imx_ssi_fiq_end: u8;
    pub static mut imx_ssi_fiq_base: usize;
    pub static mut imx_ssi_fiq_tx_buffer: usize;
    pub static mut imx_ssi_fiq_rx_buffer: usize;
}

#[repr(C)]
pub struct imx_ssi_platform_data {
    pub flags: u32,
    pub ac97_reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,
    pub ac97_warm_reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,
}

pub const IMX_SSI_DMA: u32 = 1 << 0;
pub const IMX_SSI_USE_AC97: u32 = 1 << 1;
pub const IMX_SSI_NET: u32 = 1 << 2;
pub const IMX_SSI_SYN: u32 = 1 << 3;
pub const IMX_SSI_USE_I2S_SLAVE: u32 = 1 << 4;

unsafe extern "C" {
    pub fn mxc_set_irq_fiq(irq: u32, r#type: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
