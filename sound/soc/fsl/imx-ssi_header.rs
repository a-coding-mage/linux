/* SPDX-License-Identifier: GPL-2.0 */

pub const SSI_STX0: u32 = 0x00;
pub const SSI_STX1: u32 = 0x04;
pub const SSI_SRX0: u32 = 0x08;
pub const SSI_SRX1: u32 = 0x0c;

pub const SSI_SCR: u32 = 0x10;
pub const SSI_SCR_CLK_IST: u32 = 1 << 9;
pub const SSI_SCR_CLK_IST_SHIFT: u32 = 9;
pub const SSI_SCR_TCH_EN: u32 = 1 << 8;
pub const SSI_SCR_SYS_CLK_EN: u32 = 1 << 7;
pub const SSI_SCR_I2S_MODE_NORM: u32 = 0 << 5;
pub const SSI_SCR_I2S_MODE_MSTR: u32 = 1 << 5;
pub const SSI_SCR_I2S_MODE_SLAVE: u32 = 2 << 5;
pub const SSI_I2S_MODE_MASK: u32 = 3 << 5;
pub const SSI_SCR_SYN: u32 = 1 << 4;
pub const SSI_SCR_NET: u32 = 1 << 3;
pub const SSI_SCR_RE: u32 = 1 << 2;
pub const SSI_SCR_TE: u32 = 1 << 1;
pub const SSI_SCR_SSIEN: u32 = 1 << 0;

pub const SSI_SISR: u32 = 0x14;
pub const SSI_SISR_MASK: u32 = (1 << 19) - 1;
pub const SSI_SISR_CMDAU: u32 = 1 << 18;
pub const SSI_SISR_CMDDU: u32 = 1 << 17;
pub const SSI_SISR_RXT: u32 = 1 << 16;
pub const SSI_SISR_RDR1: u32 = 1 << 15;
pub const SSI_SISR_RDR0: u32 = 1 << 14;
pub const SSI_SISR_TDE1: u32 = 1 << 13;
pub const SSI_SISR_TDE0: u32 = 1 << 12;
pub const SSI_SISR_ROE1: u32 = 1 << 11;
pub const SSI_SISR_ROE0: u32 = 1 << 10;
pub const SSI_SISR_TUE1: u32 = 1 << 9;
pub const SSI_SISR_TUE0: u32 = 1 << 8;
pub const SSI_SISR_TFS: u32 = 1 << 7;
pub const SSI_SISR_RFS: u32 = 1 << 6;
pub const SSI_SISR_TLS: u32 = 1 << 5;
pub const SSI_SISR_RLS: u32 = 1 << 4;
pub const SSI_SISR_RFF1: u32 = 1 << 3;
pub const SSI_SISR_RFF0: u32 = 1 << 2;
pub const SSI_SISR_TFE1: u32 = 1 << 1;
pub const SSI_SISR_TFE0: u32 = 1 << 0;

pub const SSI_SIER: u32 = 0x18;
pub const SSI_SIER_RDMAE: u32 = 1 << 22;
pub const SSI_SIER_RIE: u32 = 1 << 21;
pub const SSI_SIER_TDMAE: u32 = 1 << 20;
pub const SSI_SIER_TIE: u32 = 1 << 19;
pub const SSI_SIER_CMDAU_EN: u32 = 1 << 18;
pub const SSI_SIER_CMDDU_EN: u32 = 1 << 17;
pub const SSI_SIER_RXT_EN: u32 = 1 << 16;
pub const SSI_SIER_RDR1_EN: u32 = 1 << 15;
pub const SSI_SIER_RDR0_EN: u32 = 1 << 14;
pub const SSI_SIER_TDE1_EN: u32 = 1 << 13;
pub const SSI_SIER_TDE0_EN: u32 = 1 << 12;
pub const SSI_SIER_ROE1_EN: u32 = 1 << 11;
pub const SSI_SIER_ROE0_EN: u32 = 1 << 10;
pub const SSI_SIER_TUE1_EN: u32 = 1 << 9;
pub const SSI_SIER_TUE0_EN: u32 = 1 << 8;
pub const SSI_SIER_TFS_EN: u32 = 1 << 7;
pub const SSI_SIER_RFS_EN: u32 = 1 << 6;
pub const SSI_SIER_TLS_EN: u32 = 1 << 5;
pub const SSI_SIER_RLS_EN: u32 = 1 << 4;
pub const SSI_SIER_RFF1_EN: u32 = 1 << 3;
pub const SSI_SIER_RFF0_EN: u32 = 1 << 2;
pub const SSI_SIER_TFE1_EN: u32 = 1 << 1;
pub const SSI_SIER_TFE0_EN: u32 = 1 << 0;

pub const SSI_STCR: u32 = 0x1c;
pub const SSI_STCR_TXBIT0: u32 = 1 << 9;
pub const SSI_STCR_TFEN1: u32 = 1 << 8;
pub const SSI_STCR_TFEN0: u32 = 1 << 7;
pub const SSI_FIFO_ENABLE_0_SHIFT: u32 = 7;
pub const SSI_STCR_TFDIR: u32 = 1 << 6;
pub const SSI_STCR_TXDIR: u32 = 1 << 5;
pub const SSI_STCR_TSHFD: u32 = 1 << 4;
pub const SSI_STCR_TSCKP: u32 = 1 << 3;
pub const SSI_STCR_TFSI: u32 = 1 << 2;
pub const SSI_STCR_TFSL: u32 = 1 << 1;
pub const SSI_STCR_TEFS: u32 = 1 << 0;

pub const SSI_SRCR: u32 = 0x20;
pub const SSI_SRCR_RXBIT0: u32 = 1 << 9;
pub const SSI_SRCR_RFEN1: u32 = 1 << 8;
pub const SSI_SRCR_RFEN0: u32 = 1 << 7;
/* SSI_FIFO_ENABLE_0_SHIFT is also defined here in C with the same value. */
pub const SSI_SRCR_RFDIR: u32 = 1 << 6;
pub const SSI_SRCR_RXDIR: u32 = 1 << 5;
pub const SSI_SRCR_RSHFD: u32 = 1 << 4;
pub const SSI_SRCR_RSCKP: u32 = 1 << 3;
pub const SSI_SRCR_RFSI: u32 = 1 << 2;
pub const SSI_SRCR_RFSL: u32 = 1 << 1;
pub const SSI_SRCR_REFS: u32 = 1 << 0;

pub const SSI_SRCCR: u32 = 0x28;
pub const SSI_SRCCR_DIV2: u32 = 1 << 18;
pub const SSI_SRCCR_PSR: u32 = 1 << 17;
pub const fn SSI_SRCCR_WL(x: u32) -> u32 {
    (((x).wrapping_sub(2)) >> 1) << 13
}
pub const fn SSI_SRCCR_DC(x: u32) -> u32 {
    ((x) & 0x1f) << 8
}
pub const fn SSI_SRCCR_PM(x: u32) -> u32 {
    ((x) & 0xff) << 0
}
pub const SSI_SRCCR_WL_MASK: u32 = 0xf << 13;
pub const SSI_SRCCR_DC_MASK: u32 = 0x1f << 8;
pub const SSI_SRCCR_PM_MASK: u32 = 0xff << 0;

pub const SSI_STCCR: u32 = 0x24;
pub const SSI_STCCR_DIV2: u32 = 1 << 18;
pub const SSI_STCCR_PSR: u32 = 1 << 17;
pub const fn SSI_STCCR_WL(x: u32) -> u32 {
    (((x).wrapping_sub(2)) >> 1) << 13
}
pub const fn SSI_STCCR_DC(x: u32) -> u32 {
    ((x) & 0x1f) << 8
}
pub const fn SSI_STCCR_PM(x: u32) -> u32 {
    ((x) & 0xff) << 0
}
pub const SSI_STCCR_WL_MASK: u32 = 0xf << 13;
pub const SSI_STCCR_DC_MASK: u32 = 0x1f << 8;
pub const SSI_STCCR_PM_MASK: u32 = 0xff << 0;

pub const SSI_SFCSR: u32 = 0x2c;
pub const fn SSI_SFCSR_RFCNT1(x: u32) -> u32 {
    ((x) & 0xf) << 28
}
pub const SSI_RX_FIFO_1_COUNT_SHIFT: u32 = 28;
pub const fn SSI_SFCSR_TFCNT1(x: u32) -> u32 {
    ((x) & 0xf) << 24
}
pub const SSI_TX_FIFO_1_COUNT_SHIFT: u32 = 24;
pub const fn SSI_SFCSR_RFWM1(x: u32) -> u32 {
    ((x) & 0xf) << 20
}
pub const fn SSI_SFCSR_TFWM1(x: u32) -> u32 {
    ((x) & 0xf) << 16
}
pub const fn SSI_SFCSR_RFCNT0(x: u32) -> u32 {
    ((x) & 0xf) << 12
}
pub const SSI_RX_FIFO_0_COUNT_SHIFT: u32 = 12;
pub const fn SSI_SFCSR_TFCNT0(x: u32) -> u32 {
    ((x) & 0xf) << 8
}
pub const SSI_TX_FIFO_0_COUNT_SHIFT: u32 = 8;
pub const fn SSI_SFCSR_RFWM0(x: u32) -> u32 {
    ((x) & 0xf) << 4
}
pub const fn SSI_SFCSR_TFWM0(x: u32) -> u32 {
    ((x) & 0xf) << 0
}
pub const SSI_SFCSR_RFWM0_MASK: u32 = 0xf << 4;
pub const SSI_SFCSR_TFWM0_MASK: u32 = 0xf << 0;

pub const SSI_STR: u32 = 0x30;
pub const SSI_STR_TEST: u32 = 1 << 15;
pub const SSI_STR_RCK2TCK: u32 = 1 << 14;
pub const SSI_STR_RFS2TFS: u32 = 1 << 13;
pub const fn SSI_STR_RXSTATE(x: u32) -> u32 {
    ((x) & 0xf) << 8
}
pub const SSI_STR_TXD2RXD: u32 = 1 << 7;
pub const SSI_STR_TCK2RCK: u32 = 1 << 6;
pub const SSI_STR_TFS2RFS: u32 = 1 << 5;
pub const fn SSI_STR_TXSTATE(x: u32) -> u32 {
    ((x) & 0xf) << 0
}

pub const SSI_SOR: u32 = 0x34;
pub const SSI_SOR_CLKOFF: u32 = 1 << 6;
pub const SSI_SOR_RX_CLR: u32 = 1 << 5;
pub const SSI_SOR_TX_CLR: u32 = 1 << 4;
pub const SSI_SOR_INIT: u32 = 1 << 3;
pub const fn SSI_SOR_WAIT(x: u32) -> u32 {
    ((x) & 0x3) << 1
}
pub const SSI_SOR_WAIT_MASK: u32 = 0x3 << 1;
pub const SSI_SOR_SYNRST: u32 = 1 << 0;

pub const SSI_SACNT: u32 = 0x38;
pub const fn SSI_SACNT_FRDIV(x: u32) -> u32 {
    ((x) & 0x3f) << 5
}
pub const SSI_SACNT_WR: u32 = 1 << 4;
pub const SSI_SACNT_RD: u32 = 1 << 3;
pub const SSI_SACNT_TIF: u32 = 1 << 2;
pub const SSI_SACNT_FV: u32 = 1 << 1;
pub const SSI_SACNT_AC97EN: u32 = 1 << 0;

pub const SSI_SACADD: u32 = 0x3c;
pub const SSI_SACDAT: u32 = 0x40;
pub const SSI_SATAG: u32 = 0x44;
pub const SSI_STMSK: u32 = 0x48;
pub const SSI_SRMSK: u32 = 0x4c;
pub const SSI_SACCST: u32 = 0x50;
pub const SSI_SACCEN: u32 = 0x54;
pub const SSI_SACCDIS: u32 = 0x58;

/* SSI clock sources */
pub const IMX_SSP_SYS_CLK: u32 = 0;

/* SSI audio dividers */
pub const IMX_SSI_TX_DIV_2: u32 = 0;
pub const IMX_SSI_TX_DIV_PSR: u32 = 1;
pub const IMX_SSI_TX_DIV_PM: u32 = 2;
pub const IMX_SSI_RX_DIV_2: u32 = 3;
pub const IMX_SSI_RX_DIV_PSR: u32 = 4;
pub const IMX_SSI_RX_DIV_PM: u32 = 5;

pub const DRV_NAME: &str = "imx-ssi";

/* C includes omitted: linux/dmaengine.h, linux/dma/imx-dma.h,
 * sound/dmaengine_pcm.h, and imx-pcm.h provide the external types below.
 */

#[repr(C)]
pub struct imx_ssi {
    pub ac97_dev: *mut platform_device,

    pub imx_ac97: *mut snd_soc_dai,
    pub clk: *mut clk,
    pub base: *mut core::ffi::c_void,
    pub irq: core::ffi::c_int,
    pub fiq_enable: core::ffi::c_int,
    pub offset: core::ffi::c_uint,

    pub flags: core::ffi::c_uint,

    pub ac97_reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,
    pub ac97_warm_reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,

    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub filter_data_tx: imx_dma_data,
    pub filter_data_rx: imx_dma_data,
    pub fiq_params: imx_pcm_fiq_params,

    pub fiq_init: core::ffi::c_int,
    pub dma_init: core::ffi::c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
