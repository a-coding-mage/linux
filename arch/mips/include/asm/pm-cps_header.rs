/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2014 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

/*
 * The CM & CPC can only handle coherence & power control on a per-core basis,
 * thus in an MT system the VP(E)s within each core are coupled and can only
 * enter or exit states requiring CM or CPC assistance in unison.
 *
 * Build-time mapping from the original header:
 * CONFIG_CPU_MIPSR6 => coupled_coherence = cpu_has_vp
 * CONFIG_MIPS_MT   => coupled_coherence = cpu_has_mipsmt
 * otherwise        => coupled_coherence = 0
 */

/* Enumeration of possible PM states */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum cps_pm_state {
    CPS_PM_NC_WAIT = 0,      /* MIPS wait instruction, non-coherent */
    CPS_PM_CLOCK_GATED = 1,  /* Core clock gated */
    CPS_PM_POWER_GATED = 2,  /* Core power gated */
    CPS_PM_STATE_COUNT = 3,
}

/**
 * cps_pm_support_state - determine whether the system supports a PM state
 * @state: the state to test for support
 *
 * Returns true if the system supports the given state, otherwise false.
 */
extern "C" {
    pub fn cps_pm_support_state(state: cps_pm_state) -> bool;
}

/**
 * cps_pm_enter_state - enter a PM state
 * @state: the state to enter
 *
 * Enter the given PM state. If coupled_coherence is non-zero then it is
 * expected that this function be called at approximately the same time on
 * each coupled CPU. Returns 0 on successful entry & exit, otherwise -errno.
 */
extern "C" {
    pub fn cps_pm_enter_state(state: cps_pm_state) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
