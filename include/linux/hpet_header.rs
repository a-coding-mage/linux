/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/linux/hpet.h> are supplied elsewhere.

/*
 * Offsets into HPET Registers
 */

#[repr(C)]
pub struct Hpet {
    pub hpet_cap: u64, // capabilities
    pub res0: u64, // reserved
    pub hpet_config: u64, // configuration
    pub res1: u64, // reserved
    pub hpet_isr: u64, // interrupt status reg
    pub res2: [u64; 25], // reserved
    pub _u0: HpetMainCounter,
    pub res3: u64, // reserved
    pub hpet_timers: [HpetTimer; 0],
}

#[repr(C)]
pub union HpetMainCounter {
    pub _hpet_mc64: u64,
    pub _hpet_mc32: u32,
    pub _hpet_mc: usize,
}

#[repr(C)]
pub struct HpetTimer {
    pub hpet_config: u64, // configuration/cap
    pub _u1: HpetTimerCompare,
    pub hpet_fsb: [u64; 2], // FSB route
}

#[repr(C)]
pub union HpetTimerCompare {
    pub _hpet_hc64: u64,
    pub _hpet_hc32: u32,
    pub _hpet_compare: usize,
}

// #define hpet_mc _u0._hpet_mc
// #define hpet_compare _u1._hpet_compare

pub const HPET_MAX_TIMERS: usize = 32;
pub const HPET_MAX_IRQ: usize = 32;

/*
 * HPET general capabilities register
 */

pub const HPET_COUNTER_CLK_PERIOD_MASK: u64 = 0xffffffff00000000u64;
pub const HPET_COUNTER_CLK_PERIOD_SHIFT: u64 = 32u64;
pub const HPET_VENDOR_ID_MASK: u64 = 0x00000000ffff0000u64;
pub const HPET_VENDOR_ID_SHIFT: u64 = 16u64;
pub const HPET_LEG_RT_CAP_MASK: u64 = 0x8000u64;
pub const HPET_COUNTER_SIZE_MASK: u64 = 0x2000u64;
pub const HPET_NUM_TIM_CAP_MASK: u64 = 0x1f00u64;
pub const HPET_NUM_TIM_CAP_SHIFT: u64 = 8u64;

/*
 * HPET general configuration register
 */

pub const HPET_LEG_RT_CNF_MASK: u64 = 2u64;
pub const HPET_ENABLE_CNF_MASK: u64 = 1u64;

/*
 * Timer configuration register
 */

pub const Tn_INT_ROUTE_CAP_MASK: u64 = 0xffffffff00000000u64;
pub const Tn_INT_ROUTE_CAP_SHIFT: u64 = 32u64;
pub const Tn_FSB_INT_DELCAP_MASK: u64 = 0x8000u64;
pub const Tn_FSB_INT_DELCAP_SHIFT: u64 = 15u64;
pub const Tn_FSB_EN_CNF_MASK: u64 = 0x4000u64;
pub const Tn_FSB_EN_CNF_SHIFT: u64 = 14u64;
pub const Tn_INT_ROUTE_CNF_MASK: u64 = 0x3e00u64;
pub const Tn_INT_ROUTE_CNF_SHIFT: u64 = 9u64;
pub const Tn_32MODE_CNF_MASK: u64 = 0x0100u64;
pub const Tn_VAL_SET_CNF_MASK: u64 = 0x0040u64;
pub const Tn_SIZE_CAP_MASK: u64 = 0x0020u64;
pub const Tn_PER_INT_CAP_MASK: u64 = 0x0010u64;
pub const Tn_TYPE_CNF_MASK: u64 = 0x0008u64;
pub const Tn_INT_ENB_CNF_MASK: u64 = 0x0004u64;
pub const Tn_INT_TYPE_CNF_MASK: u64 = 0x0002u64;

/*
 * Timer FSB Interrupt Route Register
 */

pub const Tn_FSB_INT_ADDR_MASK: u64 = 0xffffffff00000000u64;
pub const Tn_FSB_INT_ADDR_SHIFT: u64 = 32u64;
pub const Tn_FSB_INT_VAL_MASK: u64 = 0x00000000ffffffffu64;

/*
 * exported interfaces
 */

#[repr(C)]
pub struct HpetData {
    pub hd_phys_address: usize,
    pub hd_address: *mut core::ffi::c_void,
    pub hd_nirqs: u16,
    pub hd_state: u32, // timer allocated
    pub hd_irq: [u32; HPET_MAX_TIMERS],
}

pub unsafe fn hpet_reserve_timer(hd: *mut HpetData, timer: i32) {
    (*hd).hd_state |= 1u32.wrapping_shl(timer as u32);
    return;
}

unsafe extern "C" {
    pub fn hpet_alloc(hd: *mut HpetData) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
