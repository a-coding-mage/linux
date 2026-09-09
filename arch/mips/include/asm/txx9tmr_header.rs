/*
 * include/asm-mips/txx9tmr.h
 * TX39/TX49 timer controller definitions.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

use core::ffi::{c_int, c_uint, c_ulong};

#[repr(C)]
pub struct txx9_tmr_reg {
    pub tcr: u32,
    pub tisr: u32,
    pub cpra: u32,
    pub cprb: u32,
    pub itmr: u32,
    pub unused0: [u32; 3],
    pub ccdr: u32,
    pub unused1: [u32; 3],
    pub pgmr: u32,
    pub unused2: [u32; 3],
    pub wtmr: u32,
    pub unused3: [u32; 43],
    pub trr: u32,
}

/* TMTCR : Timer Control */
pub const TXx9_TMTCR_TCE: u32 = 0x00000080;
pub const TXx9_TMTCR_CCDE: u32 = 0x00000040;
pub const TXx9_TMTCR_CRE: u32 = 0x00000020;
pub const TXx9_TMTCR_ECES: u32 = 0x00000008;
pub const TXx9_TMTCR_CCS: u32 = 0x00000004;
pub const TXx9_TMTCR_TMODE_MASK: u32 = 0x00000003;
pub const TXx9_TMTCR_TMODE_ITVL: u32 = 0x00000000;
pub const TXx9_TMTCR_TMODE_PGEN: u32 = 0x00000001;
pub const TXx9_TMTCR_TMODE_WDOG: u32 = 0x00000002;

/* TMTISR : Timer Int. Status */
pub const TXx9_TMTISR_TPIBS: u32 = 0x00000004;
pub const TXx9_TMTISR_TPIAS: u32 = 0x00000002;
pub const TXx9_TMTISR_TIIS: u32 = 0x00000001;

/* TMITMR : Interval Timer Mode */
pub const TXx9_TMITMR_TIIE: u32 = 0x00008000;
pub const TXx9_TMITMR_TZCE: u32 = 0x00000001;

/* TMWTMR : Watchdog Timer Mode */
pub const TXx9_TMWTMR_TWIE: u32 = 0x00008000;
pub const TXx9_TMWTMR_WDIS: u32 = 0x00000080;
pub const TXx9_TMWTMR_TWC: u32 = 0x00000001;

extern "C" {
    pub fn txx9_clocksource_init(baseaddr: c_ulong, imbusclk: c_uint);
    pub fn txx9_clockevent_init(baseaddr: c_ulong, irq: c_int, imbusclk: c_uint);
    pub fn txx9_tmr_init(baseaddr: c_ulong);
}

pub const TXX9_TIMER_BITS: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
