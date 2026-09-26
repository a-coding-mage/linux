// SPDX-License-Identifier: GPL-2.0-only
//! The built-in initcall_debug core parameter and its actual bool owner.

#![cfg(not(MODULE))]

use super::{bindings, main_globals};

#[repr(transparent)]
pub(super) struct CoreParameter(pub(super) bindings::kernel_param);

// SAFETY: immutable descriptor fields point to permanent strings and the
// original parameter operations. The argument points to the sole mutable bool
// owner, whose access is serialized by boot and kernel parameter machinery.
unsafe impl Sync for CoreParameter {}

static NAME: [u8; b"initcall_debug\0".len()] = *b"initcall_debug\0";

const RECORD: bindings::kernel_param = bindings::kernel_param {
    name: NAME.as_ptr().cast(),
    mod_: core::ptr::null_mut(),
    ops: core::ptr::addr_of!(bindings::param_ops_bool),
    perm: 0o644,
    level: -1,
    flags: 0,
    __bindgen_anon_1: bindings::kernel_param__bindgen_ty_1 {
        arg: core::ptr::addr_of_mut!(main_globals::initcall_debug).cast(),
    },
};

// These two architectures require writable data relocations, as specified by
// __moduleparam_const. The natural canonical type alignment supplies the stride.
#[cfg(any(CONFIG_ALPHA, CONFIG_PPC64))]
#[used]
#[link_section = "__param"]
#[linkage = "internal"]
pub(super) static mut INITCALL_DEBUG_PARAMETER: CoreParameter = CoreParameter(RECORD);

#[cfg(not(any(CONFIG_ALPHA, CONFIG_PPC64)))]
#[used]
#[link_section = "__param"]
#[linkage = "internal"]
pub(super) static INITCALL_DEBUG_PARAMETER: CoreParameter = CoreParameter(RECORD);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
