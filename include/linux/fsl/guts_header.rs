/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/fsl/guts.h. */

#[repr(C, packed)]
pub struct CcsrGuts {
    pub porpllsr: u32, pub porbmsr: u32, pub porimpscr: u32, pub pordevsr: u32,
    pub pordbgmsr: u32, pub pordevsr2: u32, pub res018: [u8; 0x20 - 0x18],
    pub porcir: u32, pub res024: [u8; 0x30 - 0x24], pub gpiocr: u32,
    pub res034: [u8; 0x40 - 0x34], pub gpoutdr: u32, pub res044: [u8; 0x50 - 0x44],
    pub gpindr: u32, pub res054: [u8; 0x60 - 0x54], pub pmuxcr: u32, pub pmuxcr2: u32,
    pub dmuxcr: u32, pub res06c: [u8; 0x70 - 0x6c], pub devdisr: u32,
    pub devdisr2: u32, pub res078: [u8; 0x7c - 0x78], pub pmjcr: u32,
    pub powmgtcsr: u32, pub pmrccr: u32, pub pmpdccr: u32, pub pmcdr: u32,
    pub mcpsumr: u32, pub rstrscr: u32, pub ectrstcr: u32, pub autorstsr: u32,
    pub pvr: u32, pub svr: u32, pub res0a8: [u8; 0xb0 - 0xa8], pub rstcr: u32,
    pub res0b4: [u8; 0xc0 - 0xb4], pub iovselsr: u32, pub res0c4: [u8; 0x100 - 0xc4],
    pub rcw: CcsrGutsRcw, pub res180: [u8; 0x224 - 0x180], pub iodelay1: u32,
    pub iodelay2: u32, pub res22c: [u8; 0x604 - 0x22c], pub pamubypenr: u32,
    pub res608: [u8; 0x800 - 0x608], pub clkdvdr: u32, pub res804: [u8; 0x900 - 0x804],
    pub ircr: u32, pub res904: [u8; 0x908 - 0x904], pub dmacr: u32,
    pub res90c: [u8; 0x914 - 0x90c], pub elbccr: u32, pub res918: [u8; 0xb20 - 0x918],
    pub ddr1clkdr: u32, pub ddr2clkdr: u32, pub ddrclkdr: u32,
    pub resb2c: [u8; 0xe00 - 0xb2c], pub clkocr: u32, pub rese04: [u8; 0xe10 - 0xe04],
    pub ddrdllcr: u32, pub rese14: [u8; 0xe20 - 0xe14], pub lbcdllcr: u32,
    pub cpfor: u32, pub rese28: [u8; 0xf04 - 0xe28], pub srds1cr0: u32,
    pub srds1cr1: u32, pub resf0c: [u8; 0xf2c - 0xf0c], pub itcr: u32,
    pub resf30: [u8; 0xf40 - 0xf30], pub srds2cr0: u32, pub srds2cr1: u32,
}
#[repr(C)] pub union CcsrGutsRcw { pub rcwsr: [u32; 32], pub rcwcr: [u32; 32] }
pub const CCSR_GUTS_DEVDISR_TB1: u32 = 0x00001000;
pub const CCSR_GUTS_DEVDISR_TB0: u32 = 0x00004000;

extern "C" {
    pub fn fsl_guts_lane_validate(serdes_idx: i32, lane: i32, lane_mode: LynxLaneMode) -> i32;
    pub fn fsl_guts_lane_set_mode(serdes_idx: i32, lane: i32, lane_mode: LynxLaneMode) -> i32;
}
extern "Rust" { pub type LynxLaneMode; }

pub const fn mpc85xx_pmuxcr_qe(x: u32) -> u32 { 0x8000u32 >> x }

#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_DMACR_DEV_SSI: u32 = 0;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_DMACR_DEV_IR: u32 = 1;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub unsafe fn guts_set_dmacr(guts: *mut CcsrGuts, co: u32, ch: u32, device: u32) {
    let shift = 16 + (8 * (1 - co) + 2 * (3 - ch));
    let p = core::ptr::addr_of_mut!((*guts).dmacr);
    let old = u32::from_be(core::ptr::read_volatile(p));
    core::ptr::write_volatile(p, (old & !(3 << shift) | (device << shift)).to_be());
}
#[cfg(feature = "CONFIG_PPC_86xx")]
pub unsafe fn guts_set_pmuxcr_dma(guts: *mut CcsrGuts, co: u32, ch: u32, value: u32) {
    if ch == 0 || ch == 3 {
        let shift = 2 * (co + 1) - (ch & 1) - 1;
        let p = core::ptr::addr_of_mut!((*guts).pmuxcr);
        let old = u32::from_be(core::ptr::read_volatile(p));
        core::ptr::write_volatile(p, (old & !(1 << shift) | (value << shift)).to_be());
    }
}
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_LDPSEL: u32 = 0x00010000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI1_MASK: u32 = 0x0000C000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI1_LA: u32 = 0;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI1_HI: u32 = 0x00004000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI1_SSI: u32 = 0x00008000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI2_MASK: u32 = 0x00003000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI2_LA: u32 = 0;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI2_HI: u32 = 0x00001000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_SSI2_SSI: u32 = 0x00002000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_LA_22_25_LA: u32 = 0;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_LA_22_25_HI: u32 = 0x400;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_DBGDRV: u32 = 0x200;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_DMA2_0: u32 = 8;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_DMA2_3: u32 = 4;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_DMA1_0: u32 = 2;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_PMUXCR_DMA1_3: u32 = 1;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_PXCKEN: u32 = 0x80000000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_SSICKEN: u32 = 0x20000000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_PXCKINV: u32 = 0x10000000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_PXCKDLY_SHIFT: u32 = 25;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_PXCKDLY_MASK: u32 = 0x06000000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const fn CCSR_GUTS_CLKDVDR_PXCKDLY(x: u32) -> u32 { (x & 3) << 25 }
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_PXCLK_SHIFT: u32 = 16;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_PXCLK_MASK: u32 = 0x001F0000;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const fn CCSR_GUTS_CLKDVDR_PXCLK(x: u32) -> u32 { (x & 31) << 16 }
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const CCSR_GUTS_CLKDVDR_SSICLK_MASK: u32 = 0xff;
#[cfg(feature = "CONFIG_PPC_86xx")]
pub const fn CCSR_GUTS_CLKDVDR_SSICLK(x: u32) -> u32 { x & 0xff }

#[repr(C, packed)]
pub struct CcsrRcpmV1 {
    pub res0000: [u8;4], pub cdozsr: u32, pub res0008: [u8;4], pub cdozcr: u32,
    pub res0010: [u8;4], pub cnapsr: u32, pub res0018: [u8;4], pub cnapcr: u32,
    pub res0020: [u8;4], pub cdozpsr: u32, pub res0028: [u8;4], pub cnappsr: u32,
    pub res0030: [u8;4], pub cwaitsr: u32, pub res0038: [u8;4], pub cwdtdsr: u32,
    pub powmgtcsr: u32, pub res0044: [u8;12], pub ippdexpcr: u32,
    pub res0054: [u8;16], pub cpmimr: u32, pub res0068: [u8;4], pub cpmcimr: u32,
    pub res0070: [u8;4], pub cpmmcmr: u32, pub res0078: [u8;4], pub cpmnmimr: u32,
    pub res0080: [u8;4], pub ctbenr: u32, pub res0088: [u8;4], pub ctbckselr: u32,
    pub res0090: [u8;4], pub ctbhltcr: u32, pub res0098: [u8;4], pub cmcpmaskcr: u32,
}
pub const RCPM_POWMGTCSR_SLP: u32 = 0x00020000;

#[repr(C, packed)]
pub struct CcsrRcpmV2 {
    pub res_00: [u8;12], pub tph10sr0: u32, pub res_10: [u8;12], pub tph10setr0: u32,
    pub res_20: [u8;12], pub tph10clrr0: u32, pub res_30: [u8;12], pub tph10psr0: u32,
    pub res_40: [u8;12], pub twaitsr0: u32, pub res_50: [u8;96], pub pcph15sr: u32,
    pub pcph15setr: u32, pub pcph15clrr: u32, pub pcph15psr: u32, pub res_c0: [u8;16],
    pub pcph20sr: u32, pub pcph20setr: u32, pub pcph20clrr: u32, pub pcph20psr: u32,
    pub pcpw20sr: u32, pub res_e0: [u8;12], pub pcph30sr: u32, pub pcph30setr: u32,
    pub pcph30clrr: u32, pub pcph30psr: u32, pub res_100: [u8;32], pub ippwrgatecr: u32,
    pub res_124: [u8;12], pub powmgtcsr: u32, pub res_134: [u8;12], pub ippdexpcr: [u32;4],
    pub res_150: [u8;12], pub tpmimr0: u32, pub res_160: [u8;12], pub tpmcimr0: u32,
    pub res_170: [u8;12], pub tpmmcmr0: u32, pub res_180: [u8;12], pub tpmnmimr0: u32,
    pub res_190: [u8;12], pub tmcpmaskcr0: u32, pub pctbenr: u32, pub pctbclkselr: u32,
    pub tbclkdivr: u32, pub res_1ac: [u8;4], pub ttbhltcr: [u32;4], pub clpcl10sr: u32,
    pub clpcl10setr: u32, pub clpcl10clrr: u32, pub clpcl10psr: u32, pub cddslpsetr: u32,
    pub cddslpclrr: u32, pub cdpwroksetr: u32, pub cdpwrokclrr: u32, pub cdpwrensr: u32,
    pub cddslsr: u32, pub res_1e8: [u8;8], pub dslpcntcr: [u32;8], pub res_300: [u8;3568],
}
pub const RCPM_POWMGTCSR_LPM20_RQ: u32 = 0x00100000;
pub const RCPM_POWMGTCSR_LPM20_ST: u32 = 0x00000200;
pub const RCPM_POWMGTCSR_P_LPM20_ST: u32 = 0x00000100;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
