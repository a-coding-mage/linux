// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-mvebu/dove.c
 *
 * Marvell Dove 88AP510 System On Chip FDT Board
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn tauros2_init(value: c_int);
    fn mvebu_mbus_dt_init(disable: bool) -> c_int;
    fn dove_init_pmu();
    fn mvebu_restart(mode: c_int, cmd: *const c_char);
}

/// Initialize the Marvell Dove board.
unsafe fn dove_init() {
    // pr_info("Dove 88AP510 SoC\\n");
    // The kernel logging macro is provided by the surrounding environment.

    // CONFIG_CACHE_TAUROS2 conditionally includes this call in the C source.
    #[cfg(CONFIG_CACHE_TAUROS2)]
    {
        tauros2_init(0);
    }

    // BUG_ON(mvebu_mbus_dt_init(false));
    if mvebu_mbus_dt_init(false) != 0 {
        unsafe { core::intrinsics::abort() };
    }
    dove_init_pmu();
}

static DOVE_DT_COMPAT: [*const c_char; 2] = [
    c"marvell,dove".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(DOVE_DT, "Marvell Dove")
//     .init_machine = dove_init,
//     .restart = mvebu_restart,
//     .dt_compat = dove_dt_compat,
// MACHINE_END
// The machine descriptor is emitted by the platform's DT registration macros.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
