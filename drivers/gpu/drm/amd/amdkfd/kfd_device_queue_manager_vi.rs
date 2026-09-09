// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.

const APE1_FIXED_BITS_MASK: u64 = 0xFFFF80000000FFFF;
const APE1_LIMIT_ALIGNMENT: u64 = 0xFFFF;

extern "C" {
    fn get_sh_mem_bases_nybble_64(pdd: *mut qcm_process_device) -> u32;
    fn qpd_to_pdd(qpd: *mut qcm_process_device) -> *mut qcm_process_device;
    fn mqd_manager_init_vi(asic_ops: *mut device_queue_manager_asic_ops);
}

#[repr(C)]
pub struct device_queue_manager {
    _private: [u8; 0],
}
#[repr(C)]
pub struct qcm_process_device {
    pub sh_mem_ape1_base: u64,
    pub sh_mem_ape1_limit: u64,
    pub sh_mem_config: u32,
    pub sh_mem_bases: u32,
}
#[repr(C)]
pub struct queue_properties {
    pub sdma_vm_addr: u32,
}
#[repr(C)]
pub struct queue {
    pub properties: queue_properties,
}
#[repr(C)]
pub struct device_queue_manager_asic_ops {
    pub set_cache_memory_policy: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut qcm_process_device, cache_policy, cache_policy, *mut core::ffi::c_void, u64, u32) -> bool>,
    pub update_qpd: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut qcm_process_device) -> i32>,
    pub init_sdma_vm: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut queue, *mut qcm_process_device)>,
    pub mqd_manager_init: Option<unsafe extern "C" fn(*mut device_queue_manager_asic_ops)>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cache_policy {
    cache_policy_coherent,
}

const MTYPE_UC: u32 = 0;
const MTYPE_NC: u32 = 1;
const SH_MEM_ALIGNMENT_MODE_UNALIGNED: u32 = 0;
const SH_MEM_CONFIG__ALIGNMENT_MODE__SHIFT: u32 = 0;
const SH_MEM_CONFIG__DEFAULT_MTYPE__SHIFT: u32 = 2;
const SH_MEM_CONFIG__APE1_MTYPE__SHIFT: u32 = 4;
const SH_MEM_BASES__SHARED_BASE__SHIFT: u32 = 16;
const SDMA0_RLC0_VIRTUAL_ADDR__SHARED_BASE__SHIFT: u32 = 0;
const SDMA0_RLC0_VIRTUAL_ADDR__SHARED_BASE_MASK: u32 = 0xFFFFF000;

unsafe fn compute_sh_mem_bases_64bit(top_address_nybble: u32) -> u32 {
    // In 64-bit mode, hardware supplies the remaining aperture address bits.
    // WARN_ON((top_address_nybble & 1) || top_address_nybble > 0xE ||
    //         top_address_nybble == 0);
    (top_address_nybble << 12)
        | ((top_address_nybble << 12) << SH_MEM_BASES__SHARED_BASE__SHIFT)
}

unsafe extern "C" fn set_cache_memory_policy_vi(
    _dqm: *mut device_queue_manager,
    qpd: *mut qcm_process_device,
    default_policy: cache_policy,
    alternate_policy: cache_policy,
    alternate_aperture_base: *mut core::ffi::c_void,
    alternate_aperture_size: u64,
    _misc_process_properties: u32,
) -> bool {
    let mut retval = true;

    if alternate_aperture_size == 0 {
        (*qpd).sh_mem_ape1_base = 1;
        (*qpd).sh_mem_ape1_limit = 0;
    } else {
        let base = alternate_aperture_base as usize as u64;
        let limit = base.wrapping_add(alternate_aperture_size).wrapping_sub(1);

        if limit <= base
            || (base & APE1_FIXED_BITS_MASK) != 0
            || (limit & APE1_FIXED_BITS_MASK) != APE1_LIMIT_ALIGNMENT
        {
            retval = false;
            return retval;
        }

        (*qpd).sh_mem_ape1_base = base >> 16;
        (*qpd).sh_mem_ape1_limit = limit >> 16;
    }

    let default_mtype = if default_policy == cache_policy::cache_policy_coherent { MTYPE_UC } else { MTYPE_NC };
    let ape1_mtype = if alternate_policy == cache_policy::cache_policy_coherent { MTYPE_UC } else { MTYPE_NC };

    (*qpd).sh_mem_config =
        (SH_MEM_ALIGNMENT_MODE_UNALIGNED << SH_MEM_CONFIG__ALIGNMENT_MODE__SHIFT)
        | (default_mtype << SH_MEM_CONFIG__DEFAULT_MTYPE__SHIFT)
        | (ape1_mtype << SH_MEM_CONFIG__APE1_MTYPE__SHIFT);

    let temp = get_sh_mem_bases_nybble_64(qpd_to_pdd(qpd));
    (*qpd).sh_mem_bases = compute_sh_mem_bases_64bit(temp);
    retval
}

unsafe extern "C" fn update_qpd_vi(
    _dqm: *mut device_queue_manager,
    _qpd: *mut qcm_process_device,
) -> i32 {
    0
}

unsafe extern "C" fn init_sdma_vm(
    _dqm: *mut device_queue_manager,
    q: *mut queue,
    qpd: *mut qcm_process_device,
) {
    (*q).properties.sdma_vm_addr =
        (get_sh_mem_bases_nybble_64(qpd_to_pdd(qpd)) << SDMA0_RLC0_VIRTUAL_ADDR__SHARED_BASE__SHIFT)
            & SDMA0_RLC0_VIRTUAL_ADDR__SHARED_BASE_MASK;
}

pub unsafe extern "C" fn device_queue_manager_init_vi(
    asic_ops: *mut device_queue_manager_asic_ops,
) {
    (*asic_ops).set_cache_memory_policy = Some(set_cache_memory_policy_vi);
    (*asic_ops).update_qpd = Some(update_qpd_vi);
    (*asic_ops).init_sdma_vm = Some(init_sdma_vm);
    (*asic_ops).mqd_manager_init = Some(mqd_manager_init_vi);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
