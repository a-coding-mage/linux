/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Subset of bpf.h declarations, split out so files that need only these
 * declarations can avoid bpf.h's full include cost.
 */

// C header guard: _LINUX_BPF_DEFS_H

// CONFIG_BPF_SYSCALL selects the externally provided implementation.
#[cfg(feature = "CONFIG_BPF_SYSCALL")]
extern "C" {
    pub fn bpf_arena_handle_page_fault(
        addr: libc::c_ulong,
        is_write: bool,
        fault_ip: libc::c_ulong,
    ) -> bool;
}

// When CONFIG_BPF_SYSCALL is not enabled, the C static inline implementation
// returns false.
#[cfg(not(feature = "CONFIG_BPF_SYSCALL"))]
#[inline]
pub fn bpf_arena_handle_page_fault(
    _addr: libc::c_ulong,
    _is_write: bool,
    _fault_ip: libc::c_ulong,
) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
