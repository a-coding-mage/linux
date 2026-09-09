/* SPDX-License-Identifier: GPL-2.0 */
// Dependency: `u16` corresponds to Linux's `u16` type.

#[repr(C)]
pub struct mmiowb_state {
    pub nesting_count: u16,
    pub mmiowb_pending: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
