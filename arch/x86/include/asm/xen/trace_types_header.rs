/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub enum xen_mc_flush_reason {
    XEN_MC_FL_NONE,     /* explicit flush */
    XEN_MC_FL_BATCH,    /* out of hypercall space */
    XEN_MC_FL_ARGS,     /* out of argument space */
    XEN_MC_FL_CALLBACK, /* out of callback space */
}

#[repr(C)]
pub enum xen_mc_extend_args {
    XEN_MC_XE_OK,
    XEN_MC_XE_BAD_OP,
    XEN_MC_XE_NO_SPACE,
}

pub type xen_mc_callback_fn_t = unsafe extern "C" fn(*mut core::ffi::c_void);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
