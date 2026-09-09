/* SPDX-License-Identifier: GPL-2.0 */

/* Default "unsigned long" context */
pub type mm_context_t = usize;

/* mm/srmmu.c */
extern "C" {
    pub static mut srmmu_ctx_table_phys: *mut ctxd_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
