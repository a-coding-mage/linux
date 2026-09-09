/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common defines for v7m cpus
 */

// IOMEM() resolves to the memory-mapped address in the target environment.
pub const V7M_SCS_ICTR: usize = 0xe000e004;
pub const V7M_SCS_ICTR_INTLINESNUM_MASK: u32 = 0x0000000f;

pub const BASEADDR_V7M_SCB: usize = 0xe000ed00;

pub const V7M_SCB_CPUID: u32 = 0x00;

pub const V7M_SCB_ICSR: u32 = 0x04;
pub const V7M_SCB_ICSR_PENDSVSET: u32 = 1 << 28;
pub const V7M_SCB_ICSR_PENDSVCLR: u32 = 1 << 27;
pub const V7M_SCB_ICSR_RETTOBASE: u32 = 1 << 11;
pub const V7M_SCB_ICSR_VECTACTIVE: u32 = 0x000001ff;

pub const V7M_SCB_VTOR: u32 = 0x08;

pub const V7M_SCB_AIRCR: u32 = 0x0c;
pub const V7M_SCB_AIRCR_VECTKEY: u32 = 0x05fa << 16;
pub const V7M_SCB_AIRCR_SYSRESETREQ: u32 = 1 << 2;

pub const V7M_SCB_SCR: u32 = 0x10;
pub const V7M_SCB_SCR_SLEEPDEEP: u32 = 1 << 2;

pub const V7M_SCB_CCR: u32 = 0x14;
pub const V7M_SCB_CCR_STKALIGN: u32 = 1 << 9;
pub const V7M_SCB_CCR_DC: u32 = 1 << 16;
pub const V7M_SCB_CCR_IC: u32 = 1 << 17;
pub const V7M_SCB_CCR_BP: u32 = 1 << 18;

pub const V7M_SCB_SHPR2: u32 = 0x1c;
pub const V7M_SCB_SHPR3: u32 = 0x20;

pub const V7M_SCB_SHCSR: u32 = 0x24;
pub const V7M_SCB_SHCSR_USGFAULTENA: u32 = 1 << 18;
pub const V7M_SCB_SHCSR_BUSFAULTENA: u32 = 1 << 17;
pub const V7M_SCB_SHCSR_MEMFAULTENA: u32 = 1 << 16;

pub const V7M_xPSR_FRAMEPTRALIGN: u32 = 0x00000200;
pub const V7M_xPSR_EXCEPTIONNO: u32 = V7M_SCB_ICSR_VECTACTIVE;

/*
 * When branching to an address that has bits [31:28] == 0xf an exception return
 * occurs. Bits [27:5] are reserved (SBOP). If the processor implements the FP
 * extension Bit [4] defines if the exception frame has space allocated for FP
 * state information, SBOP otherwise. Bit [3] defines the mode that is returned
 * to (0 -> handler mode; 1 -> thread mode). Bit [2] defines which sp is used
 * (0 -> msp; 1 -> psp). Bits [1:0] are fixed to 0b01.
 */
pub const EXC_RET_STACK_MASK: u32 = 0x00000004;
pub const EXC_RET_THREADMODE_PROCESSSTACK: u32 = 3 << 2;

/* Cache related definitions */
pub const V7M_SCB_CLIDR: u32 = 0x78; /* Cache Level ID register */
pub const V7M_SCB_CTR: u32 = 0x7c; /* Cache Type register */
pub const V7M_SCB_CCSIDR: u32 = 0x80; /* Cache size ID register */
pub const V7M_SCB_CSSELR: u32 = 0x84; /* Cache size selection register */

/* Memory-mapped MPU registers for M-class */
pub const MPU_TYPE: u32 = 0x90;
pub const MPU_CTRL: u32 = 0x94;
pub const MPU_CTRL_ENABLE: u32 = 1;
pub const MPU_CTRL_PRIVDEFENA: u32 = 1 << 2;

pub const PMSAv7_RNR: u32 = 0x98;
pub const PMSAv7_RBAR: u32 = 0x9c;
pub const PMSAv7_RASR: u32 = 0xa0;

pub const PMSAv8_RNR: u32 = 0x98;
pub const PMSAv8_RBAR: u32 = 0x9c;
pub const PMSAv8_RLAR: u32 = 0xa0;
pub const PMSAv8_RBAR_A: unsafe fn(u32) -> u32 = |n| PMSAv8_RBAR + 8 * n;
pub const PMSAv8_RLAR_A: unsafe fn(u32) -> u32 = |n| PMSAv8_RLAR + 8 * n;
pub const PMSAv8_MAIR0: u32 = 0xc0;
pub const PMSAv8_MAIR1: u32 = 0xc4;

/* Cache opeartions */
pub const V7M_SCB_ICIALLU: u32 = 0x250; /* I-cache invalidate all to PoU */
pub const V7M_SCB_ICIMVAU: u32 = 0x258; /* I-cache invalidate by MVA to PoU */
pub const V7M_SCB_DCIMVAC: u32 = 0x25c; /* D-cache invalidate by MVA to PoC */
pub const V7M_SCB_DCISW: u32 = 0x260; /* D-cache invalidate by set-way */
pub const V7M_SCB_DCCMVAU: u32 = 0x264; /* D-cache clean by MVA to PoU */
pub const V7M_SCB_DCCMVAC: u32 = 0x268; /* D-cache clean by MVA to PoC */
pub const V7M_SCB_DCCSW: u32 = 0x26c; /* D-cache clean by set-way */
pub const V7M_SCB_DCCIMVAC: u32 = 0x270; /* D-cache clean and invalidate by MVA to PoC */
pub const V7M_SCB_DCCISW: u32 = 0x274; /* D-cache clean and invalidate by set-way */
pub const V7M_SCB_BPIALL: u32 = 0x278; /* D-cache clean and invalidate by set-way */

#[repr(C)]
pub enum reboot_mode {}

extern "C" {
    pub fn armv7m_restart(mode: reboot_mode, cmd: *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
