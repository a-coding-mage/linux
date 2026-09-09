// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// soc_and_ip_translator.h
// soc_and_ip_translator/dcn401/dcn401_soc_and_ip_translator.h
// soc_and_ip_translator/dcn42/dcn42_soc_and_ip_translator.h
// soc_and_ip_translator/dcn42b/dcn42b_soc_and_ip_translator.h
// soc_and_ip_translator/dcn60/dcn60_soc_and_ip_translator.h

unsafe extern "C" {
    fn dcn401_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );
    fn dcn42_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );
    fn dcn42b_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );
    fn dcn60_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );
}

unsafe fn dc_construct_soc_and_ip_translator(
    soc_and_ip_translator: *mut soc_and_ip_translator,
    dc_version: dce_version,
) {
    match dc_version {
        dce_version::DCN_VERSION_4_01 => {
            dcn401_construct_soc_and_ip_translator(soc_and_ip_translator);
        }
        dce_version::DCN_VERSION_4_2 => {
            dcn42_construct_soc_and_ip_translator(soc_and_ip_translator);
        }
        dce_version::DCN_VERSION_4_2B => {
            dcn42b_construct_soc_and_ip_translator(soc_and_ip_translator);
        }
        dce_version::DCN_VERSION_6_0 => {
            dcn60_construct_soc_and_ip_translator(soc_and_ip_translator);
        }
        _ => {}
    }
}

pub unsafe fn dc_create_soc_and_ip_translator(
    dc_version: dce_version,
) -> *mut soc_and_ip_translator {
    let mut soc_and_ip_translator: *mut soc_and_ip_translator;

    // C macro dependency: kzalloc_obj(*soc_and_ip_translator).
    soc_and_ip_translator = kzalloc_obj!(soc_and_ip_translator);
    if soc_and_ip_translator.is_null() {
        return core::ptr::null_mut();
    }

    dc_construct_soc_and_ip_translator(soc_and_ip_translator, dc_version);

    soc_and_ip_translator
}

pub unsafe fn dc_destroy_soc_and_ip_translator(
    soc_and_ip_translator: *mut *mut soc_and_ip_translator,
) {
    kfree!(*soc_and_ip_translator);
    *soc_and_ip_translator = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
