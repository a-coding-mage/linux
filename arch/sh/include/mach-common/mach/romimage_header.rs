// SPDX-License-Identifier: GPL-2.0

// C preprocessor condition: __ASSEMBLER__.
// Do nothing here by default when assembling.

// C translation of the non-assembler branch.
#[inline]
fn mmcif_update_progress(nr: i32) {
    let _ = nr;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
