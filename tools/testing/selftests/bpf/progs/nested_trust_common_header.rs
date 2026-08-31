// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

// C dependency intent: #include <stdbool.h>

extern "C" {
    // __ksym
    pub fn bpf_cpumask_test_cpu(cpu: ::core::ffi::c_uint, cpumask: *const cpumask) -> bool;

    // __ksym
    pub fn bpf_cpumask_first_zero(cpumask: *const cpumask) -> u32;
}
