/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Rivos, Inc.
 */

use core::ffi::c_void;

extern "C" {
    pub fn __riscv_copy_words_unaligned(dst: *mut c_void, src: *const c_void, size: usize);
    pub fn __riscv_copy_bytes_unaligned(dst: *mut c_void, src: *const c_void, size: usize);

    // Preserves the C build-time condition CONFIG_RISCV_PROBE_VECTOR_UNALIGNED_ACCESS.
    #[cfg(feature = "CONFIG_RISCV_PROBE_VECTOR_UNALIGNED_ACCESS")]
    pub fn __riscv_copy_vec_words_unaligned(dst: *mut c_void, src: *const c_void, size: usize);

    #[cfg(feature = "CONFIG_RISCV_PROBE_VECTOR_UNALIGNED_ACCESS")]
    pub fn __riscv_copy_vec_bytes_unaligned(dst: *mut c_void, src: *const c_void, size: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
