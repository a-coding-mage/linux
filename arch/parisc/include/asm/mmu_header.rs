/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct mm_context_t {
    pub space_id: usize,
    pub vdso_base: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
