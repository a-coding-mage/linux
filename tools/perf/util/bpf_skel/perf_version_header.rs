/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */

/*
 * Original C dependencies:
 *   #include "vmlinux.h"
 *   #include <bpf/bpf_helpers.h>
 *
 * This is used by tests/shell/record_bpf_metadata.sh
 * to verify that BPF metadata generation works.
 *
 * PERF_VERSION is defined by a build rule at compile time.
 */

extern "C" {
    static PERF_VERSION: [core::ffi::c_char; 0];
}

#[link_section = ".rodata"]
#[no_mangle]
pub static bpf_metadata_perf_version: *const core::ffi::c_char =
    unsafe { PERF_VERSION.as_ptr() };
