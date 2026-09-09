/* SPDX-License-Identifier: GPL-2.0 */

/* The C header includes <asm/irq.h>; IRQ symbols are supplied by the target. */

/* PIT structure */

pub const BVME_PIT_BASE: usize = 0xffa00000;

#[repr(C)]
pub struct PitRegs_t {
    pub pad_a: [u8; 3], pub pgcr: u8,
    pub pad_b: [u8; 3], pub psrr: u8,
    pub pad_c: [u8; 3], pub paddr: u8,
    pub pad_d: [u8; 3], pub pbddr: u8,
    pub pad_e: [u8; 3], pub pcddr: u8,
    pub pad_f: [u8; 3], pub pivr: u8,
    pub pad_g: [u8; 3], pub pacr: u8,
    pub pad_h: [u8; 3], pub pbcr: u8,
    pub pad_i: [u8; 3], pub padr: u8,
    pub pad_j: [u8; 3], pub pbdr: u8,
    pub pad_k: [u8; 3], pub paar: u8,
    pub pad_l: [u8; 3], pub pbar: u8,
    pub pad_m: [u8; 3], pub pcdr: u8,
    pub pad_n: [u8; 3], pub psr: u8,
    pub pad_o: [u8; 3], pub res1: u8,
    pub pad_p: [u8; 3], pub res2: u8,
    pub pad_q: [u8; 3], pub tcr: u8,
    pub pad_r: [u8; 3], pub tivr: u8,
    pub pad_s: [u8; 3], pub res3: u8,
    pub pad_t: [u8; 3], pub cprh: u8,
    pub pad_u: [u8; 3], pub cprm: u8,
    pub pad_v: [u8; 3], pub cprl: u8,
    pub pad_w: [u8; 3], pub res4: u8,
    pub pad_x: [u8; 3], pub crh: u8,
    pub pad_y: [u8; 3], pub crm: u8,
    pub pad_z: [u8; 3], pub crl: u8,
    pub pad_A: [u8; 3], pub tsr: u8,
    pub pad_B: [u8; 3], pub res5: u8,
}
pub type PitRegsPtr = *mut PitRegs_t;
pub const fn bvmepit() -> PitRegsPtr { BVME_PIT_BASE as PitRegsPtr }

pub const BVME_RTC_BASE: usize = 0xff900000;

#[repr(C)]
pub struct RtcRegs_t {
    pub pad_a: [u8; 3], pub msr: u8,
    pub pad_b: [u8; 3], pub t0cr_rtmr: u8,
    pub pad_c: [u8; 3], pub t1cr_omr: u8,
    pub pad_d: [u8; 3], pub pfr_icr0: u8,
    pub pad_e: [u8; 3], pub irr_icr1: u8,
    pub pad_f: [u8; 3], pub bcd_tenms: u8,
    pub pad_g: [u8; 3], pub bcd_sec: u8,
    pub pad_h: [u8; 3], pub bcd_min: u8,
    pub pad_i: [u8; 3], pub bcd_hr: u8,
    pub pad_j: [u8; 3], pub bcd_dom: u8,
    pub pad_k: [u8; 3], pub bcd_mth: u8,
    pub pad_l: [u8; 3], pub bcd_year: u8,
    pub pad_m: [u8; 3], pub bcd_ujcc: u8,
    pub pad_n: [u8; 3], pub bcd_hjcc: u8,
    pub pad_o: [u8; 3], pub bcd_dow: u8,
    pub pad_p: [u8; 3], pub t0lsb: u8,
    pub pad_q: [u8; 3], pub t0msb: u8,
    pub pad_r: [u8; 3], pub t1lsb: u8,
    pub pad_s: [u8; 3], pub t1msb: u8,
    pub pad_t: [u8; 3], pub cmp_sec: u8,
    pub pad_u: [u8; 3], pub cmp_min: u8,
    pub pad_v: [u8; 3], pub cmp_hr: u8,
    pub pad_w: [u8; 3], pub cmp_dom: u8,
    pub pad_x: [u8; 3], pub cmp_mth: u8,
    pub pad_y: [u8; 3], pub cmp_dow: u8,
    pub pad_z: [u8; 3], pub sav_sec: u8,
    pub pad_A: [u8; 3], pub sav_min: u8,
    pub pad_B: [u8; 3], pub sav_hr: u8,
    pub pad_C: [u8; 3], pub sav_dom: u8,
    pub pad_D: [u8; 3], pub sav_mth: u8,
    pub pad_E: [u8; 3], pub ram: u8,
    pub pad_F: [u8; 3], pub test: u8,
}
pub type RtcPtr_t = *mut RtcRegs_t;

pub const BVME_I596_BASE: usize = 0xff100000;
pub const BVME_ETHIRQ_REG: usize = 0xff20000b;
pub const BVME_LOCAL_IRQ_STAT: usize = 0xff20000f;
pub const BVME_ETHERR: u8 = 0x02;
pub const BVME_ABORT_STATUS: u8 = 0x08;
pub const BVME_NCR53C710_BASE: usize = 0xff000000;
pub const BVME_SCC_A_ADDR: usize = 0xffb0000b;
pub const BVME_SCC_B_ADDR: usize = 0xffb00003;
pub const BVME_SCC_RTxC: u32 = 7372800;
pub const BVME_CONFIG_REG: usize = 0xff500003;
pub const BVME_CONFIG_SW1: u8 = 0x08;
pub const BVME_CONFIG_SW2: u8 = 0x04;
pub const BVME_CONFIG_SW3: u8 = 0x02;
pub const BVME_CONFIG_SW4: u8 = 0x01;

pub const BVME_IRQ_TYPE_PRIO: i32 = 0;
pub const BVME_IRQ_PRN: i32 = IRQ_USER + 20;
pub const BVME_IRQ_TIMER: i32 = IRQ_USER + 25;
pub const BVME_IRQ_I596: i32 = IRQ_AUTO_2;
pub const BVME_IRQ_SCSI: i32 = IRQ_AUTO_3;
pub const BVME_IRQ_RTC: i32 = IRQ_AUTO_6;
pub const BVME_IRQ_ABORT: i32 = IRQ_AUTO_7;

/* SCC interrupts */
pub const BVME_IRQ_SCC_BASE: i32 = IRQ_USER;
pub const BVME_IRQ_SCCB_TX: i32 = IRQ_USER;
pub const BVME_IRQ_SCCB_STAT: i32 = IRQ_USER + 2;
pub const BVME_IRQ_SCCB_RX: i32 = IRQ_USER + 4;
pub const BVME_IRQ_SCCB_SPCOND: i32 = IRQ_USER + 6;
pub const BVME_IRQ_SCCA_TX: i32 = IRQ_USER + 8;
pub const BVME_IRQ_SCCA_STAT: i32 = IRQ_USER + 10;
pub const BVME_IRQ_SCCA_RX: i32 = IRQ_USER + 12;
pub const BVME_IRQ_SCCA_SPCOND: i32 = IRQ_USER + 14;

/* Address control registers */
pub const BVME_ACR_A32VBA: usize = 0xff400003;
pub const BVME_ACR_A32MSK: usize = 0xff410003;
pub const BVME_ACR_A24VBA: usize = 0xff420003;
pub const BVME_ACR_A24MSK: usize = 0xff430003;
pub const BVME_ACR_A16VBA: usize = 0xff440003;
pub const BVME_ACR_A32LBA: usize = 0xff450003;
pub const BVME_ACR_A24LBA: usize = 0xff460003;
pub const BVME_ACR_ADDRCTL: usize = 0xff470003;

pub const fn config_reg_ptr() -> *mut u8 { BVME_CONFIG_REG as *mut u8 }
pub const fn bvme_acr_a32vba() -> *mut u8 { BVME_ACR_A32VBA as *mut u8 }
pub const fn bvme_acr_a32msk() -> *mut u8 { BVME_ACR_A32MSK as *mut u8 }
pub const fn bvme_acr_a24vba() -> *mut u8 { BVME_ACR_A24VBA as *mut u8 }
pub const fn bvme_acr_a24msk() -> *mut u8 { BVME_ACR_A24MSK as *mut u8 }
pub const fn bvme_acr_a16vba() -> *mut u8 { BVME_ACR_A16VBA as *mut u8 }
pub const fn bvme_acr_a32lba() -> *mut u8 { BVME_ACR_A32LBA as *mut u8 }
pub const fn bvme_acr_a24lba() -> *mut u8 { BVME_ACR_A24LBA as *mut u8 }
pub const fn bvme_acr_addrctl() -> *mut u8 { BVME_ACR_ADDRCTL as *mut u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
