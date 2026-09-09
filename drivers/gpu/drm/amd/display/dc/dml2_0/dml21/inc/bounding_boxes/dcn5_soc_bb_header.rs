// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dcn5_soc_bb.h.
// Dependencies supplied by the original C includes are expected to provide:
// dml2_soc_bb, dml2_ip_capabilities, utm_qos_model, and
// utm_qos_model_dchub_v1.

#[inline]
pub unsafe fn dcn5_initialize_soc_bb(soc_bb: *mut dml2_soc_bb) {
    core::ptr::write_bytes(soc_bb as *mut u8, 0, core::mem::size_of::<dml2_soc_bb>());
}

#[inline]
pub unsafe fn dcn5_initialize_ip_caps(ip_caps: *mut dml2_ip_capabilities) {
    core::ptr::write_bytes(
        ip_caps as *mut u8,
        0,
        core::mem::size_of::<dml2_ip_capabilities>(),
    );
}

#[inline]
pub unsafe fn dcn5_initialize_utm_qos_model(
    qos_model: *mut utm_qos_model,
    dchub: *mut utm_qos_model_dchub_v1,
) {
    core::ptr::write_bytes(
        qos_model as *mut u8,
        0,
        core::mem::size_of::<utm_qos_model>(),
    );
    core::ptr::write_bytes(
        dchub as *mut u8,
        0,
        core::mem::size_of::<utm_qos_model_dchub_v1>(),
    );
    (*qos_model).dchub_v1 = dchub;
}

#[inline]
pub unsafe fn dcn5or_initialize_utm_qos_model(
    qos_model: *mut utm_qos_model,
    dchub: *mut utm_qos_model_dchub_v1,
) {
    core::ptr::write_bytes(
        qos_model as *mut u8,
        0,
        core::mem::size_of::<utm_qos_model>(),
    );
    core::ptr::write_bytes(
        dchub as *mut u8,
        0,
        core::mem::size_of::<utm_qos_model_dchub_v1>(),
    );
    (*qos_model).dchub_v1 = dchub;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
