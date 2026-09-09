// SPDX-License-Identifier: MIT
/* Copyright 2021 Advanced Micro Devices, Inc.
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
 *
 * Authors: David Nieto
 *          Roy Sun
 */

// Dependencies supplied by the surrounding kernel/amdgpu translation unit.

static AMDGPU_IP_NAME: [&'static core::ffi::c_char; AMDGPU_HW_IP_NUM] = [
    c"gfx".as_ptr(),
    c"compute".as_ptr(),
    c"dma".as_ptr(),
    c"dec".as_ptr(),
    c"enc".as_ptr(),
    c"enc_1".as_ptr(),
    c"dec".as_ptr(),
    c"enc".as_ptr(),
    c"jpeg".as_ptr(),
    c"vpe".as_ptr(),
];

pub unsafe fn amdgpu_show_fdinfo(p: *mut drm_printer, file: *mut drm_file) {
    let fpriv: *mut amdgpu_fpriv = (*file).driver_priv as *mut amdgpu_fpriv;
    let vm: *mut amdgpu_vm = &mut (*fpriv).vm;

    let mut stats: [amdgpu_mem_stats; __AMDGPU_PL_NUM] = core::mem::zeroed();
    let mut usage: [ktime_t; AMDGPU_HW_IP_NUM] = core::mem::zeroed();
    let pl_name: [*const core::ffi::c_char; 8] = [
        c"vram".as_ptr(),
        c"gtt".as_ptr(),
        c"cpu".as_ptr(),
        c"gds".as_ptr(),
        c"gws".as_ptr(),
        c"oa".as_ptr(),
        c"doorbell".as_ptr(),
        c"mmioremap".as_ptr(),
    ];
    let mut hw_ip: core::ffi::c_uint;
    let mut i: core::ffi::c_uint;

    amdgpu_vm_get_memory(vm, stats.as_mut_ptr());
    amdgpu_ctx_mgr_usage(&mut (*fpriv).ctx_mgr, usage.as_mut_ptr());

    /*
     * ******************************************************************
     * For text output format description please see drm-usage-stats.rst!
     * ******************************************************************
     */

    drm_printf(p, c"pasid:\t%u\n".as_ptr(), (*fpriv).vm.pasid);

    i = 0;
    while i < pl_name.len() as core::ffi::c_uint {
        if pl_name[i as usize].is_null() {
            i += 1;
            continue;
        }

        drm_print_memory_stats(
            p,
            &mut stats[i as usize].drm,
            DRM_GEM_OBJECT_RESIDENT | DRM_GEM_OBJECT_PURGEABLE,
            pl_name[i as usize],
        );
        i += 1;
    }

    /* Legacy amdgpu keys, alias to drm-resident-memory-: */
    drm_printf(p, c"drm-memory-vram:\t%llu KiB\n".as_ptr(), stats[TTM_PL_VRAM].drm.resident / 1024u64);
    drm_printf(p, c"drm-memory-gtt: \t%llu KiB\n".as_ptr(), stats[TTM_PL_TT].drm.resident / 1024u64);
    drm_printf(p, c"drm-memory-cpu: \t%llu KiB\n".as_ptr(), stats[TTM_PL_SYSTEM].drm.resident / 1024u64);

    /* Amdgpu specific memory accounting keys: */
    drm_printf(p, c"amd-evicted-vram:\t%llu KiB\n".as_ptr(), stats[TTM_PL_VRAM].evicted / 1024u64);
    drm_printf(p, c"amd-requested-vram:\t%llu KiB\n".as_ptr(), (stats[TTM_PL_VRAM].drm.shared + stats[TTM_PL_VRAM].drm.private) / 1024u64);
    drm_printf(p, c"amd-requested-gtt:\t%llu KiB\n".as_ptr(), (stats[TTM_PL_TT].drm.shared + stats[TTM_PL_TT].drm.private) / 1024u64);

    hw_ip = 0;
    while hw_ip < AMDGPU_HW_IP_NUM as core::ffi::c_uint {
        if usage[hw_ip as usize] == 0 {
            hw_ip += 1;
            continue;
        }

        drm_printf(p, c"drm-engine-%s:\t%lld ns\n".as_ptr(), AMDGPU_IP_NAME[hw_ip as usize], ktime_to_ns(usage[hw_ip as usize]));
        hw_ip += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
