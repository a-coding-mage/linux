/* SPDX-License-Identifier: GPL-2.0 */

/*
 * MAX floating point unit state size (FSAVE/FRESTORE)
 *
 * The cfg names below preserve the original build-time CONFIG_* conditions.
 */

#[cfg(any(CONFIG_M68020, CONFIG_M68030))]
pub const FPSTATESIZE: usize = 216;

#[cfg(all(
    not(any(CONFIG_M68020, CONFIG_M68030)),
    CONFIG_M68040
))]
pub const FPSTATESIZE: usize = 96;

#[cfg(all(
    not(any(CONFIG_M68020, CONFIG_M68030)),
    not(CONFIG_M68040),
    CONFIG_M68KFPU_EMU
))]
pub const FPSTATESIZE: usize = 28;

#[cfg(all(
    not(any(CONFIG_M68020, CONFIG_M68030)),
    not(CONFIG_M68040),
    not(CONFIG_M68KFPU_EMU),
    CONFIG_COLDFIRE,
    CONFIG_MMU
))]
pub const FPSTATESIZE: usize = 16;

#[cfg(all(
    not(any(CONFIG_M68020, CONFIG_M68030)),
    not(CONFIG_M68040),
    not(CONFIG_M68KFPU_EMU),
    not(all(CONFIG_COLDFIRE, CONFIG_MMU)),
    CONFIG_M68060
))]
pub const FPSTATESIZE: usize = 12;

#[cfg(all(
    not(any(CONFIG_M68020, CONFIG_M68030)),
    not(CONFIG_M68040),
    not(CONFIG_M68KFPU_EMU),
    not(all(CONFIG_COLDFIRE, CONFIG_MMU)),
    not(CONFIG_M68060)
))]
pub const FPSTATESIZE: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
