// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2018-2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit:
// kfd_device_queue_manager.h, navi10_enum.h, gc_10_1_0_offset.h,
// gc_10_1_0_sh_mask.h

use core::ffi::c_void;

unsafe extern "C" {
    fn qpd_to_pdd(qpd: *mut qcm_process_device) -> *mut kfd_process_device;
    fn pr_debug(format: *const u8, ...);
    fn mqd_manager_init_v10(asic_ops: *mut device_queue_manager_asic_ops);
}

unsafe fn compute_sh_mem_bases_64bit(pdd: *mut kfd_process_device) -> u32 {
    let shared_base = (*pdd).lds_base >> 48;
    let private_base = (*pdd).scratch_base >> 48;

    (shared_base << SH_MEM_BASES__SHARED_BASE__SHIFT) | private_base
}

unsafe fn set_cache_memory_policy_v10(
    _dqm: *mut device_queue_manager,
    qpd: *mut qcm_process_device,
    _default_policy: cache_policy,
    _alternate_policy: cache_policy,
    _alternate_aperture_base: *mut c_void,
    _alternate_aperture_size: u64,
    _misc_process_properties: u32,
) -> bool {
    (*qpd).sh_mem_config =
        (SH_MEM_ALIGNMENT_MODE_UNALIGNED << SH_MEM_CONFIG__ALIGNMENT_MODE__SHIFT)
            | (3 << SH_MEM_CONFIG__INITIAL_INST_PREFETCH__SHIFT);
    (*qpd).sh_mem_ape1_limit = 0;
    (*qpd).sh_mem_ape1_base = 0;
    (*qpd).sh_mem_bases = compute_sh_mem_bases_64bit(qpd_to_pdd(qpd));

    pr_debug(b"sh_mem_bases 0x%X\n\0".as_ptr(), (*qpd).sh_mem_bases);
    true
}

unsafe fn update_qpd_v10(
    _dqm: *mut device_queue_manager,
    _qpd: *mut qcm_process_device,
) -> i32 {
    0
}

unsafe fn init_sdma_vm_v10(
    _dqm: *mut device_queue_manager,
    q: *mut queue,
    _qpd: *mut qcm_process_device,
) {
    // Not needed on SDMAv4 onwards any more
    (*q).properties.sdma_vm_addr = 0;
}

unsafe fn device_queue_manager_init_v10(
    asic_ops: *mut device_queue_manager_asic_ops,
) {
    (*asic_ops).set_cache_memory_policy = Some(set_cache_memory_policy_v10);
    (*asic_ops).update_qpd = Some(update_qpd_v10);
    (*asic_ops).init_sdma_vm = Some(init_sdma_vm_v10);
    (*asic_ops).mqd_manager_init = Some(mqd_manager_init_v10);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
