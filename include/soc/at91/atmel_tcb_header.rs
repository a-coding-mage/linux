/* Timer/Counter Unit (TC) registers. */

// The original header includes Linux compiler/list definitions.

use core::ffi::c_void;

pub struct clk;
pub struct platform_device;
pub struct list_head;

#[repr(C)]
pub struct atmel_tcb_config {
    pub counter_width: usize,
    pub has_gclk: bool,
    pub has_qdec: bool,
}

#[repr(C)]
pub struct atmel_tc {
    pub pdev: *mut platform_device,
    pub regs: *mut c_void,
    pub id: i32,
    pub tcb_config: *const atmel_tcb_config,
    pub irq: [i32; 3],
    pub clk: [*mut clk; 3],
    pub slow_clk: *mut clk,
    pub node: list_head,
    pub allocated: bool,
}

extern "C" {
    pub static atmel_tc_divisors: [u8; 5];
}

pub const ATMEL_TC_BCR: u32 = 0xc0;
pub const ATMEL_TC_SYNC: u32 = 1 << 0;
pub const ATMEL_TC_BMR: u32 = 0xc4;
pub const ATMEL_TC_TC0XC0S: u32 = 3 << 0;
pub const ATMEL_TC_TC0XC0S_TCLK0: u32 = 0 << 0;
pub const ATMEL_TC_TC0XC0S_NONE: u32 = 1 << 0;
pub const ATMEL_TC_TC0XC0S_TIOA1: u32 = 2 << 0;
pub const ATMEL_TC_TC0XC0S_TIOA2: u32 = 3 << 0;
pub const ATMEL_TC_TC1XC1S: u32 = 3 << 2;
pub const ATMEL_TC_TC1XC1S_TCLK1: u32 = 0 << 2;
pub const ATMEL_TC_TC1XC1S_NONE: u32 = 1 << 2;
pub const ATMEL_TC_TC1XC1S_TIOA0: u32 = 2 << 2;
pub const ATMEL_TC_TC1XC1S_TIOA2: u32 = 3 << 2;
pub const ATMEL_TC_TC2XC2S: u32 = 3 << 4;
pub const ATMEL_TC_TC2XC2S_TCLK2: u32 = 0 << 4;
pub const ATMEL_TC_TC2XC2S_NONE: u32 = 1 << 4;
pub const ATMEL_TC_TC2XC2S_TIOA0: u32 = 2 << 4;
pub const ATMEL_TC_TC2XC2S_TIOA1: u32 = 3 << 4;

#[inline]
pub const fn ATMEL_TC_CHAN(idx: u32) -> u32 { idx * 0x40 }
#[macro_export]
macro_rules! ATMEL_TC_REG { ($idx:expr, $reg:expr) => { $crate::ATMEL_TC_CHAN($idx) + $reg }; }

pub const ATMEL_TC_CCR: u32 = 0x00;
pub const ATMEL_TC_CLKEN: u32 = 1 << 0;
pub const ATMEL_TC_CLKDIS: u32 = 1 << 1;
pub const ATMEL_TC_SWTRG: u32 = 1 << 2;
pub const ATMEL_TC_CMR: u32 = 0x04;
pub const ATMEL_TC_TCCLKS: u32 = 7 << 0;
pub const ATMEL_TC_TIMER_CLOCK1: u32 = 0 << 0;
pub const ATMEL_TC_TIMER_CLOCK2: u32 = 1 << 0;
pub const ATMEL_TC_TIMER_CLOCK3: u32 = 2 << 0;
pub const ATMEL_TC_TIMER_CLOCK4: u32 = 3 << 0;
pub const ATMEL_TC_TIMER_CLOCK5: u32 = 4 << 0;
pub const ATMEL_TC_XC0: u32 = 5 << 0;
pub const ATMEL_TC_XC1: u32 = 6 << 0;
pub const ATMEL_TC_XC2: u32 = 7 << 0;
pub const ATMEL_TC_CLKI: u32 = 1 << 3;
pub const ATMEL_TC_BURST: u32 = 3 << 4;
pub const ATMEL_TC_GATE_NONE: u32 = 0 << 4;
pub const ATMEL_TC_GATE_XC0: u32 = 1 << 4;
pub const ATMEL_TC_GATE_XC1: u32 = 2 << 4;
pub const ATMEL_TC_GATE_XC2: u32 = 3 << 4;
pub const ATMEL_TC_WAVE: u32 = 1 << 15;

pub const ATMEL_TC_LDBSTOP: u32 = 1 << 6;
pub const ATMEL_TC_LDBDIS: u32 = 1 << 7;
pub const ATMEL_TC_ETRGEDG: u32 = 3 << 8;
pub const ATMEL_TC_ETRGEDG_NONE: u32 = 0 << 8;
pub const ATMEL_TC_ETRGEDG_RISING: u32 = 1 << 8;
pub const ATMEL_TC_ETRGEDG_FALLING: u32 = 2 << 8;
pub const ATMEL_TC_ETRGEDG_BOTH: u32 = 3 << 8;
pub const ATMEL_TC_ABETRG: u32 = 1 << 10;
pub const ATMEL_TC_CPCTRG: u32 = 1 << 14;
pub const ATMEL_TC_LDRA: u32 = 3 << 16;
pub const ATMEL_TC_LDRA_NONE: u32 = 0 << 16;
pub const ATMEL_TC_LDRA_RISING: u32 = 1 << 16;
pub const ATMEL_TC_LDRA_FALLING: u32 = 2 << 16;
pub const ATMEL_TC_LDRA_BOTH: u32 = 3 << 16;
pub const ATMEL_TC_LDRB: u32 = 3 << 18;
pub const ATMEL_TC_LDRB_NONE: u32 = 0 << 18;
pub const ATMEL_TC_LDRB_RISING: u32 = 1 << 18;
pub const ATMEL_TC_LDRB_FALLING: u32 = 2 << 18;
pub const ATMEL_TC_LDRB_BOTH: u32 = 3 << 18;

pub const ATMEL_TC_CPCSTOP: u32 = 1 << 6;
pub const ATMEL_TC_CPCDIS: u32 = 1 << 7;
pub const ATMEL_TC_EEVTEDG: u32 = 3 << 8;
pub const ATMEL_TC_EEVTEDG_NONE: u32 = 0 << 8;
pub const ATMEL_TC_EEVTEDG_RISING: u32 = 1 << 8;
pub const ATMEL_TC_EEVTEDG_FALLING: u32 = 2 << 8;
pub const ATMEL_TC_EEVTEDG_BOTH: u32 = 3 << 8;
pub const ATMEL_TC_EEVT: u32 = 3 << 10;
pub const ATMEL_TC_EEVT_TIOB: u32 = 0 << 10;
pub const ATMEL_TC_EEVT_XC0: u32 = 1 << 10;
pub const ATMEL_TC_EEVT_XC1: u32 = 2 << 10;
pub const ATMEL_TC_EEVT_XC2: u32 = 3 << 10;
pub const ATMEL_TC_ENETRG: u32 = 1 << 12;
pub const ATMEL_TC_WAVESEL: u32 = 3 << 13;
pub const ATMEL_TC_WAVESEL_UP: u32 = 0 << 13;
pub const ATMEL_TC_WAVESEL_UPDOWN: u32 = 1 << 13;
pub const ATMEL_TC_WAVESEL_UP_AUTO: u32 = 2 << 13;
pub const ATMEL_TC_WAVESEL_UPDOWN_AUTO: u32 = 3 << 13;

macro_rules! tc_actions { ($($n:ident: $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
tc_actions! {
    ATMEL_TC_ACPA: 3<<16, ATMEL_TC_ACPA_NONE:0<<16, ATMEL_TC_ACPA_SET:1<<16, ATMEL_TC_ACPA_CLEAR:2<<16, ATMEL_TC_ACPA_TOGGLE:3<<16,
    ATMEL_TC_ACPC: 3<<18, ATMEL_TC_ACPC_NONE:0<<18, ATMEL_TC_ACPC_SET:1<<18, ATMEL_TC_ACPC_CLEAR:2<<18, ATMEL_TC_ACPC_TOGGLE:3<<18,
    ATMEL_TC_AEEVT: 3<<20, ATMEL_TC_AEEVT_NONE:0<<20, ATMEL_TC_AEEVT_SET:1<<20, ATMEL_TC_AEEVT_CLEAR:2<<20, ATMEL_TC_AEEVT_TOGGLE:3<<20,
    ATMEL_TC_ASWTRG: 3<<22, ATMEL_TC_ASWTRG_NONE:0<<22, ATMEL_TC_ASWTRG_SET:1<<22, ATMEL_TC_ASWTRG_CLEAR:2<<22, ATMEL_TC_ASWTRG_TOGGLE:3<<22,
    ATMEL_TC_BCPB: 3<<24, ATMEL_TC_BCPB_NONE:0<<24, ATMEL_TC_BCPB_SET:1<<24, ATMEL_TC_BCPB_CLEAR:2<<24, ATMEL_TC_BCPB_TOGGLE:3<<24,
    ATMEL_TC_BCPC: 3<<26, ATMEL_TC_BCPC_NONE:0<<26, ATMEL_TC_BCPC_SET:1<<26, ATMEL_TC_BCPC_CLEAR:2<<26, ATMEL_TC_BCPC_TOGGLE:3<<26,
    ATMEL_TC_BEEVT: 3<<28, ATMEL_TC_BEEVT_NONE:0<<28, ATMEL_TC_BEEVT_SET:1<<28, ATMEL_TC_BEEVT_CLEAR:2<<28, ATMEL_TC_BEEVT_TOGGLE:3<<28,
    ATMEL_TC_BSWTRG: 3<<30, ATMEL_TC_BSWTRG_NONE:0<<30, ATMEL_TC_BSWTRG_SET:1<<30, ATMEL_TC_BSWTRG_CLEAR:2<<30, ATMEL_TC_BSWTRG_TOGGLE:3<<30,
}

pub const ATMEL_TC_CV: u32 = 0x10;
pub const ATMEL_TC_RA: u32 = 0x14;
pub const ATMEL_TC_RB: u32 = 0x18;
pub const ATMEL_TC_RC: u32 = 0x1c;
pub const ATMEL_TC_SR: u32 = 0x20;
pub const ATMEL_TC_CLKSTA: u32 = 1 << 16;
pub const ATMEL_TC_MTIOA: u32 = 1 << 17;
pub const ATMEL_TC_MTIOB: u32 = 1 << 18;
pub const ATMEL_TC_IER: u32 = 0x24;
pub const ATMEL_TC_IDR: u32 = 0x28;
pub const ATMEL_TC_IMR: u32 = 0x2c;
pub const ATMEL_TC_COVFS: u32 = 1 << 0;
pub const ATMEL_TC_LOVRS: u32 = 1 << 1;
pub const ATMEL_TC_CPAS: u32 = 1 << 2;
pub const ATMEL_TC_CPBS: u32 = 1 << 3;
pub const ATMEL_TC_CPCS: u32 = 1 << 4;
pub const ATMEL_TC_LDRAS: u32 = 1 << 5;
pub const ATMEL_TC_LDRBS: u32 = 1 << 6;
pub const ATMEL_TC_ETRGS: u32 = 1 << 7;
pub const ATMEL_TC_ALL_IRQ: u32 = ATMEL_TC_COVFS | ATMEL_TC_LOVRS | ATMEL_TC_CPAS | ATMEL_TC_CPBS | ATMEL_TC_CPCS | ATMEL_TC_LDRAS | ATMEL_TC_LDRBS | ATMEL_TC_ETRGS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
