/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The types and constants referenced here are
// supplied by the corresponding architecture headers.

/// Equivalent of the C `pa_to_nid` macro.
macro_rules! pa_to_nid {
    ($addr:expr) => {
        NASID_GET($addr)
    };
}

#[repr(C)]
pub struct hub_data {
    pub kern_vars: kern_vars_t,
    pub h_bigwin_used: [usize; (HUB_NUM_BIG_WINDOW + (usize::BITS as usize) - 1)
        / (usize::BITS as usize)],
    pub h_cpus: cpumask_t,
}

#[repr(C)]
pub struct node_data {
    pub pglist: pglist_data,
    pub hub: hub_data,
}

extern "C" {
    pub static mut __node_data: [*mut node_data; 0];
}

macro_rules! hub_data {
    ($n:expr) => {
        unsafe { &mut (*__node_data[$n]).hub }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
