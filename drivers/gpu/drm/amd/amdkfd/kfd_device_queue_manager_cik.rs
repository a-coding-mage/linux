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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the corresponding kernel headers and modules.

const APE1_FIXED_BITS_MASK: u64 = 0xFFFF80000000FFFF;
const APE1_LIMIT_ALIGNMENT: u64 = 0xFFFF;

pub unsafe fn device_queue_manager_init_cik(asic_ops: *mut device_queue_manager_asic_ops) {
    (*asic_ops).set_cache_memory_policy = Some(set_cache_memory_policy_cik);
    (*asic_ops).update_qpd = Some(update_qpd_cik);
    (*asic_ops).init_sdma_vm = Some(init_sdma_vm);
    (*asic_ops).mqd_manager_init = Some(mqd_manager_init_cik);
}

unsafe fn compute_sh_mem_bases_64bit(top_address_nybble: u32) -> u32 {
    /* In 64-bit mode, we can only control the top 3 bits of the LDS,
     * scratch and GPUVM apertures.
     * The hardware fills in the remaining 59 bits according to the
     * following pattern:
     * LDS:       X0000000'00000000 - X0000001'00000000 (4GB)
     * Scratch:   X0000001'00000000 - X0000002'00000000 (4GB)
     * GPUVM:     Y0010000'00000000 - Y0020000'00000000 (1TB)
     *
     * (where X/Y is the configurable nybble with the low-bit 0)
     *
     * LDS and scratch will have the same top nybble programmed in the
     * top 3 bits of SH_MEM_BASES.PRIVATE_BASE.
     * GPUVM can have a different top nybble programmed in the
     * top 3 bits of SH_MEM_BASES.SHARED_BASE.
     * We don't bother to support different top nybbles
     * for LDS/Scratch and GPUVM.
     */
    WARN_ON((top_address_nybble & 1) != 0
        || top_address_nybble > 0xE
        || top_address_nybble == 0);

    PRIVATE_BASE(top_address_nybble << 12) | SHARED_BASE(top_address_nybble << 12)
}

unsafe fn set_cache_memory_policy_cik(
    _dqm: *mut device_queue_manager,
    qpd: *mut qcm_process_device,
    default_policy: cache_policy,
    alternate_policy: cache_policy,
    alternate_aperture_base: *mut core::ffi::c_void,
    alternate_aperture_size: u64,
    _misc_process_properties: u32,
) -> bool {
    let default_mtype: u32;
    let ape1_mtype: u32;
    let temp: u32;
    let mut retval = true;

    if alternate_aperture_size == 0 {
        // base > limit disables APE1
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

    default_mtype = if default_policy == cache_policy_coherent {
        MTYPE_NONCACHED
    } else {
        MTYPE_CACHED
    };
    ape1_mtype = if alternate_policy == cache_policy_coherent {
        MTYPE_NONCACHED
    } else {
        MTYPE_CACHED
    };

    (*qpd).sh_mem_config = ((*qpd).sh_mem_config & PTR32)
        | ALIGNMENT_MODE(SH_MEM_ALIGNMENT_MODE_UNALIGNED)
        | DEFAULT_MTYPE(default_mtype)
        | APE1_MTYPE(ape1_mtype);
    // On dGPU we're always in GPUVM64 addressing mode with 64-bit aperture addresses.
    temp = get_sh_mem_bases_nybble_64(qpd_to_pdd(qpd));
    (*qpd).sh_mem_bases = compute_sh_mem_bases_64bit(temp);

    pr_debug(
        "is32bit process: %d sh_mem_bases nybble: 0x%X and register 0x%X\n",
        (*(*qpd).pqm).process.is_32bit_user_mode,
        temp,
        (*qpd).sh_mem_bases,
    );

    retval
}

unsafe fn update_qpd_cik(
    _dqm: *mut device_queue_manager,
    _qpd: *mut qcm_process_device,
) -> i32 {
    0
}

unsafe fn init_sdma_vm(
    _dqm: *mut device_queue_manager,
    q: *mut queue,
    qpd: *mut qcm_process_device,
) {
    // On dGPU we're always in GPUVM64 addressing mode with 64-bit aperture addresses.
    (*q).properties.sdma_vm_addr = (get_sh_mem_bases_nybble_64(qpd_to_pdd(qpd))
        << SDMA0_RLC0_VIRTUAL_ADDR__SHARED_BASE__SHIFT)
        & SDMA0_RLC0_VIRTUAL_ADDR__SHARED_BASE_MASK;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
