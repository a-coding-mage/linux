/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the CONFIG_PPC_POWERNV conditional section. */

pub const PNV_THREAD_RUNNING: u32 = 0;
pub const PNV_THREAD_NAP: u32 = 1;
pub const PNV_THREAD_SLEEP: u32 = 2;
pub const PNV_THREAD_WINKLE: u32 = 3;

/*
 * Core state used in powernv idle for POWER8.
 *
 * The lock bit synchronizes updates to the state, as well as parts of the
 * sleep/wake code (see kernel/idle_book3s.S).
 *
 * Bottom 8 bits track the idle state of each thread. Bit is cleared before
 * the thread executes an idle instruction (nap/sleep/winkle).
 *
 * Then there is winkle tracking. A core does not lose complete state
 * until every thread is in winkle. So the winkle count field counts the
 * number of threads in winkle (small window of false positives is okay
 * around the sleep/wake, so long as there are no false negatives).
 *
 * When the winkle count reaches 8 (the COUNT_ALL_BIT becomes set), then the
 * THREAD_WINKLE_BITS are set, which indicate which threads have not
 * yet woken from the winkle state.
 */
pub const NR_PNV_CORE_IDLE_LOCK_BIT: u32 = 28;
pub const PNV_CORE_IDLE_LOCK_BIT: u64 = 1u64 << NR_PNV_CORE_IDLE_LOCK_BIT;

pub const PNV_CORE_IDLE_WINKLE_COUNT_SHIFT: u32 = 16;
pub const PNV_CORE_IDLE_WINKLE_COUNT: u32 = 0x0001_0000;
pub const PNV_CORE_IDLE_WINKLE_COUNT_BITS: u32 = 0x000F_0000;
pub const PNV_CORE_IDLE_THREAD_WINKLE_BITS_SHIFT: u32 = 8;
pub const PNV_CORE_IDLE_THREAD_WINKLE_BITS: u32 = 0x0000_FF00;

pub const PNV_CORE_IDLE_THREAD_BITS: u32 = 0x0000_00FF;

/*
 * ============================ NOTE =================================
 * The older firmware populates only the RL field in the psscr_val and
 * sets the psscr_mask to 0xf. On such a firmware, the kernel sets the
 * remaining PSSCR fields to default values as follows:
 *
 * - ESL and EC bits are to 1. So wakeup from any stop state will be
 *   at vector 0x100.
 *
 * - MTL and PSLL are set to the maximum allowed value as per the ISA,
 *    i.e. 15.
 *
 * - The Transition Rate, TR is set to the Maximum value 3.
 */
pub const PSSCR_HV_DEFAULT_VAL: u64 = PSSCR_ESL | PSSCR_EC |
    PSSCR_PSLL_MASK | PSSCR_TR_MASK | PSSCR_MTL_MASK;

pub const PSSCR_HV_DEFAULT_MASK: u64 = PSSCR_ESL | PSSCR_EC |
    PSSCR_PSLL_MASK | PSSCR_TR_MASK | PSSCR_MTL_MASK | PSSCR_RL_MASK;
pub const PSSCR_EC_SHIFT: u32 = 20;
pub const PSSCR_ESL_SHIFT: u32 = 21;

#[inline]
pub const fn GET_PSSCR_EC(x: u64) -> u64 {
    ((x & PSSCR_EC) >> PSSCR_EC_SHIFT)
}

#[inline]
pub const fn GET_PSSCR_ESL(x: u64) -> u64 {
    ((x & PSSCR_ESL) >> PSSCR_ESL_SHIFT)
}

#[inline]
pub const fn GET_PSSCR_RL(x: u64) -> u64 {
    x & PSSCR_RL_MASK
}

pub const ERR_EC_ESL_MISMATCH: i32 = -1;
pub const ERR_DEEP_STATE_ESL_MISMATCH: i32 = -2;

pub const PNV_IDLE_NAME_LEN: usize = 16;

#[repr(C)]
pub struct pnv_idle_states_t {
    pub name: [core::ffi::c_char; PNV_IDLE_NAME_LEN],
    pub latency_ns: u32,
    pub residency_ns: u32,
    pub psscr_val: u64,
    pub psscr_mask: u64,
    pub flags: u32,
    pub valid: bool,
}

unsafe extern "C" {
    pub static mut pnv_idle_states: *mut pnv_idle_states_t;
    pub static mut nr_pnv_idle_states: i32;

    pub fn pnv_cpu_offline(cpu: u32) -> core::ffi::c_ulong;
    pub fn validate_psscr_val_mask(
        psscr_val: *mut u64,
        psscr_mask: *mut u64,
        flags: u32,
    ) -> i32;
}

#[inline]
pub unsafe fn report_invalid_psscr_val(psscr_val: u64, err: i32) {
    match err {
        ERR_EC_ESL_MISMATCH => {
            pr_warn(
                "Invalid psscr 0x%016llx : ESL,EC bits unequal\0",
                psscr_val,
            );
        }
        ERR_DEEP_STATE_ESL_MISMATCH => {
            pr_warn(
                "Invalid psscr 0x%016llx : ESL cleared for deep stop-state\0",
                psscr_val,
            );
        }
        _ => {}
    }
}

unsafe extern "C" {
    fn pr_warn(format: *const core::ffi::c_char, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
