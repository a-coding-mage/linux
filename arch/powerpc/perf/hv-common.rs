// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel translation:
// asm/io.h, asm/hvcall.h, hv-gpci.h, and hv-common.h.

#[repr(C, packed)]
struct P {
    params: hv_get_perf_counter_info_params,
    caps: hv_gpci_system_performance_capabilities,
}

#[repr(C)]
extern "C" {
    fn plpar_hcall_norets(token: ::core::ffi::c_ulong, arg: ::core::ffi::c_ulong, size: u64) -> ::core::ffi::c_ulong;
    fn virt_to_phys(addr: *const core::ffi::c_void) -> ::core::ffi::c_ulong;
}

pub unsafe fn hv_perf_caps_get(caps: *mut hv_perf_caps) -> ::core::ffi::c_ulong {
    let mut arg = P {
        params: hv_get_perf_counter_info_params {
            counter_request: cpu_to_be32(HV_GPCI_system_performance_capabilities),
            starting_index: cpu_to_be32((-1i32) as u32),
            counter_info_version_in: 0,
            ..core::mem::zeroed()
        },
        caps: core::mem::zeroed(),
    };

    let r = plpar_hcall_norets(
        H_GET_PERF_COUNTER_INFO,
        virt_to_phys((&mut arg as *mut P).cast::<core::ffi::c_void>()),
        core::mem::size_of::<P>() as u64,
    );

    if r != 0 {
        return r;
    }

    pr_devel!("capability_mask: 0x{:x}\n", arg.caps.capability_mask);

    (*caps).version = arg.params.counter_info_version_out;
    (*caps).collect_privileged = (arg.caps.perf_collect_privileged != 0) as _;
    (*caps).ga = ((arg.caps.capability_mask & HV_GPCI_CM_GA) != 0) as _;
    (*caps).expanded = ((arg.caps.capability_mask & HV_GPCI_CM_EXPANDED) != 0) as _;
    (*caps).lab = ((arg.caps.capability_mask & HV_GPCI_CM_LAB) != 0) as _;

    r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
