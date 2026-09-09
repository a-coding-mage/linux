// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::*;

pub unsafe fn device_queue_manager_init_v12_1(
    asic_ops: *mut device_queue_manager_asic_ops,
) {
    (*asic_ops).update_qpd = Some(update_qpd_v12_1);
    (*asic_ops).init_sdma_vm = Some(init_sdma_vm_v12_1);
    (*asic_ops).mqd_manager_init = Some(mqd_manager_init_v12_1);
}

unsafe fn compute_sh_mem_bases_64bit(pdd: *mut kfd_process_device) -> u32 {
    let shared_base: u32 = ((*pdd).lds_base >> 48) as u32;
    let private_base: u32 = ((*pdd).scratch_base >> 58) as u32;

    (shared_base << SH_MEM_BASES__SHARED_BASE__SHIFT)
        | (private_base << SH_MEM_BASES__PRIVATE_BASE__SHIFT)
}

unsafe fn update_qpd_v12_1(
    dqm: *mut device_queue_manager,
    qpd: *mut qcm_process_device,
) -> i32 {
    let pdd: *mut kfd_process_device;
    let adev: *mut amdgpu_device = (*(*dqm).dev).adev;
    let hub: *mut amdgpu_vmhub = &mut (*adev).vmhub[AMDGPU_GFXHUB(0) as usize];
    let xnack_enabled: bool;

    pdd = qpd_to_pdd(qpd);
    (*qpd).vm_cntx_cntl = (*hub).vm_cntx_cntl;

    /* check if sh_mem_config register already configured */
    if (*qpd).sh_mem_config == 0 {
        (*qpd).sh_mem_config =
            (SH_MEM_ALIGNMENT_MODE_UNALIGNED << SH_MEM_CONFIG__ALIGNMENT_MODE__SHIFT)
                | (3 << SH_MEM_CONFIG__INITIAL_INST_PREFETCH__SHIFT);

        (*qpd).sh_mem_config |= 1 << SH_MEM_CONFIG__F8_MODE__SHIFT;
        (*qpd).sh_mem_ape1_limit = 0;
        (*qpd).sh_mem_ape1_base = 0;
    }

    xnack_enabled = if KFD_SUPPORT_XNACK_PER_PROCESS!((*dqm).dev) {
        (*(*pdd).process).xnack_enabled
    } else {
        !(*(*pdd).dev).kfd.noretry
    };

    if !xnack_enabled {
        (*qpd).sh_mem_config |= 1 << SH_MEM_CONFIG__RETRY_DISABLE__SHIFT;
        (*qpd).vm_cntx_cntl &=
            !(1 << GCVM_CONTEXT0_CNTL__RETRY_PERMISSION_OR_INVALID_PAGE_FAULT__SHIFT);
    } else {
        (*qpd).sh_mem_config &= !(1 << SH_MEM_CONFIG__RETRY_DISABLE__SHIFT);
        (*qpd).vm_cntx_cntl |=
            1 << GCVM_CONTEXT0_CNTL__RETRY_PERMISSION_OR_INVALID_PAGE_FAULT__SHIFT;
    }

    (*qpd).sh_mem_bases = compute_sh_mem_bases_64bit(pdd);

    pr_debug!("sh_mem_bases 0x%X\n", (*qpd).sh_mem_bases);

    0
}

unsafe fn init_sdma_vm_v12_1(
    _dqm: *mut device_queue_manager,
    q: *mut queue,
    _qpd: *mut qcm_process_device,
) {
    /* Not needed on SDMAv4 onwards any more */
    (*q).properties.sdma_vm_addr = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
