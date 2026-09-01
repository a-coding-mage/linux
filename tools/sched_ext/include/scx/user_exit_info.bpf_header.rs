/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Define struct user_exit_info which is shared between BPF and userspace parts
 * to communicate exit status and other information.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// C header dependencies:
// - "vmlinux.h" when LSP is not defined
// - <bpf/bpf_core_read.h>
// - "user_exit_info_common.h"

// Original C macro:
// #define UEI_DEFINE(__name)                                           \
//      char RESIZABLE_ARRAY(data, __name##_dump);                      \
//      const volatile u32 __name##_dump_len;                           \
//      struct user_exit_info __name SEC(".data")
//
// Rust macro_rules! cannot form new identifiers such as `name_dump` from a
// single `name` identifier without external token-pasting support. Callers pass
// the pasted identifiers explicitly while preserving the generated declarations.
macro_rules! UEI_DEFINE {
    ($name:ident, $name_dump:ident, $name_dump_len:ident) => {
        RESIZABLE_ARRAY!(data, $name_dump);
        static $name_dump_len: u32;
        #[link_section = ".data"]
        static mut $name: user_exit_info;
    };
}

macro_rules! UEI_RECORD {
    ($uei_name:ident, $uei_name_dump:ident, $uei_name_dump_len:ident, $ei:expr) => {{
        bpf_probe_read_kernel_str(
            $uei_name.reason.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&$uei_name.reason),
            (*($ei)).reason,
        );
        bpf_probe_read_kernel_str(
            $uei_name.msg.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&$uei_name.msg),
            (*($ei)).msg,
        );
        bpf_probe_read_kernel_str(
            $uei_name_dump.as_mut_ptr() as *mut _,
            $uei_name_dump_len,
            (*($ei)).dump,
        );
        if bpf_core_field_exists!((*($ei)).exit_code) {
            $uei_name.exit_code = (*($ei)).exit_code;
        }
        $uei_name.exit_cpu = -1;
        if bpf_core_field_exists!((*($ei)).exit_cpu) {
            $uei_name.exit_cpu = (*($ei)).exit_cpu;
        }
        /* use __sync to force memory barrier */
        __sync_val_compare_and_swap(
            &mut $uei_name.kind as *mut _,
            $uei_name.kind,
            (*($ei)).kind,
        );
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
