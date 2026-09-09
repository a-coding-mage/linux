/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/linux/bpf.h>.
use crate::{bpf_attr, bpf_prog, EINVAL};

// CONFIG_BPF_LIRC_MODE2 selects the externally supplied implementations.
#[cfg(feature = "CONFIG_BPF_LIRC_MODE2")]
extern "C" {
    pub fn lirc_prog_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn lirc_prog_detach(attr: *const bpf_attr) -> i32;
    pub fn lirc_prog_query(attr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
}

#[cfg(not(feature = "CONFIG_BPF_LIRC_MODE2"))]
#[inline]
pub unsafe fn lirc_prog_attach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 {
    -(EINVAL as i32)
}

#[cfg(not(feature = "CONFIG_BPF_LIRC_MODE2"))]
#[inline]
pub unsafe fn lirc_prog_detach(_attr: *const bpf_attr) -> i32 {
    -(EINVAL as i32)
}

#[cfg(not(feature = "CONFIG_BPF_LIRC_MODE2"))]
#[inline]
pub unsafe fn lirc_prog_query(_attr: *const bpf_attr, _uattr: *mut bpf_attr) -> i32 {
    -(EINVAL as i32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
