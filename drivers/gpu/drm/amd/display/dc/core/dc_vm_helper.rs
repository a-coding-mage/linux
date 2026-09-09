/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// C dependencies: vm_helper.h and dc.h.

pub unsafe fn vm_helper_mark_vmid_used(
    vm_helper: *mut vm_helper,
    pos: ::core::ffi::c_uint,
    hubp_idx: u8,
) {
    let mut vmids = (*vm_helper).hubp_vmid_usage[hubp_idx as usize];

    vmids.vmid_usage[0] = vmids.vmid_usage[1];
    vmids.vmid_usage[1] = 1 << pos;
}

pub unsafe fn dc_setup_system_context(
    dc: *mut dc,
    pa_config: *mut dc_phy_addr_space_config,
) -> ::core::ffi::c_uint {
    let mut num_vmids: ::core::ffi::c_uint = 0;

    /* Call HWSS to setup HUBBUB for address config */
    if (*dc).hwss.init_sys_ctx.is_some() {
        num_vmids = ((*dc).hwss.init_sys_ctx.unwrap())((*dc).hwseq, dc, pa_config);

        /* Pre-init system aperture start/end for all HUBP instances (if not gating?)
         * or cache system aperture if using power gating
         */
        ::core::ptr::copy_nonoverlapping(
            pa_config,
            &mut (*dc).vm_pa_config,
            1,
        );
        (*dc).vm_pa_config.valid = true;
        (*dc).dml2_options.gpuvm_enable = true;
        dc_z10_save_init(dc);
    }

    num_vmids
}

pub unsafe fn dc_setup_vm_context(
    dc: *mut dc,
    va_config: *mut dc_virtual_addr_space_config,
    vmid: ::core::ffi::c_int,
) {
    ((*dc).hwss.init_vm_ctx.unwrap())((*dc).hwseq, dc, va_config, vmid);
}

pub unsafe fn dc_get_vmid_use_vector(dc: *mut dc) -> ::core::ffi::c_int {
    let mut in_use: ::core::ffi::c_int = 0;

    for i in 0..MAX_HUBP {
        in_use |= (*dc).vm_helper.hubp_vmid_usage[i].vmid_usage[0]
            | (*dc).vm_helper.hubp_vmid_usage[i].vmid_usage[1];
    }
    in_use
}

pub unsafe fn vm_helper_init(vm_helper: *mut vm_helper, num_vmid: ::core::ffi::c_uint) {
    (*vm_helper).num_vmid = num_vmid;

    ::core::ptr::write_bytes(
        (*vm_helper).hubp_vmid_usage.as_mut_ptr(),
        0,
        MAX_HUBP,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
