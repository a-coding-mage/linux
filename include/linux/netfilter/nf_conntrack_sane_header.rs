/* SPDX-License-Identifier: GPL-2.0 */
/* SANE tracking. */

#[repr(C)]
pub enum sane_state {
    SANE_STATE_NORMAL,
    SANE_STATE_START_REQUESTED,
}

/* This structure exists only once per master */
#[repr(C)]
pub struct nf_ct_sane_master {
    pub state: sane_state,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
