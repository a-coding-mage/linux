/* SPDX-License-Identifier: GPL-2.0 */

// The following items are present only when CONFIG_MMIOWB is enabled in the
// source build. The corresponding Rust build configuration is external to
// this header.
#[cfg(feature = "CONFIG_MMIOWB")]
#[macro_export]
macro_rules! arch_mmiowb_state {
    () => {
        &local_paca.mmiowb_state
    };
}

#[cfg(feature = "CONFIG_MMIOWB")]
#[macro_export]
macro_rules! mmiowb {
    () => {
        mb!()
    };
}

// Equivalent to the source's <asm-generic/mmiowb.h> inclusion; declarations
// supplied by that dependency remain external to this translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
