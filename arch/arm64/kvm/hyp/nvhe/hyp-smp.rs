// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 - Google LLC
 * Author: David Brazdil <dbrazdil@google.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/kvm_asm.h, asm/kvm_hyp.h, and asm/kvm_mmu.h.

/*
 * nVHE copy of data structures tracking available CPU cores.
 * Only entries for CPUs that were online at KVM init are populated.
 * Other CPUs should not be allowed to boot because their features were
 * not checked against the finalized system capabilities.
 */
// `NR_CPUS` and `INVALID_HWID` are supplied by the surrounding translation unit.
pub static mut hyp_cpu_logical_map: [u64; NR_CPUS] = [INVALID_HWID; NR_CPUS];

pub unsafe fn cpu_logical_map(cpu: u32) -> u64 {
    assert!((cpu as usize) < hyp_cpu_logical_map.len());

    hyp_cpu_logical_map[cpu as usize]
}

// `__ro_after_init` is a kernel annotation; the storage remains mutable here
// so that its initialization semantics can be supplied by the surrounding code.
pub static mut kvm_arm_hyp_percpu_base: [usize; NR_CPUS] = [0; NR_CPUS];

// External symbols supplied by the surrounding kernel translation unit.
extern "C" {
    static __per_cpu_start: u8;
    fn kern_hyp_va(addr: usize) -> usize;
}

pub unsafe fn __hyp_per_cpu_offset(cpu: u32) -> usize {
    let cpu_base_array: *mut usize = kvm_arm_hyp_percpu_base.as_mut_ptr();
    let this_cpu_base: usize = kern_hyp_va(*cpu_base_array.add(cpu as usize));
    let elf_base: usize = &__per_cpu_start as *const u8 as usize;
    this_cpu_base.wrapping_sub(elf_base)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
