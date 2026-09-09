// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding C/Rust translation units:
// dcn42_soc_and_ip_translator.h, dcn42b_soc_and_ip_translator.h,
// dcn401_soc_and_ip_translator.h, and bounding_boxes/dcn42b_soc_bb.h.

/* soc_and_ip_translator component used to get up-to-date values for bounding box.
 * Bounding box values are stored in several locations and locations can vary with DCN revision.
 * This component provides an interface to get DCN-specific bounding box values.
 */

unsafe fn get_default_soc_bb(soc_bb: *mut dml2_soc_bb) {
    core::ptr::copy_nonoverlapping(
        &dml2_socbb_dcn42b as *const dml2_soc_bb,
        soc_bb,
        1,
    );
    core::ptr::copy_nonoverlapping(
        &dml_dcn42b_variant_a_soc_qos_params as *const dml2_soc_qos_parameters,
        core::ptr::addr_of_mut!((*soc_bb).qos_parameters),
        1,
    );
}

pub unsafe fn dcn42b_get_soc_bb(
    soc_bb: *mut dml2_soc_bb,
    dc: *const dc,
    config: *const dml2_configuration_options,
) {
    //get default soc_bb with static values
    get_default_soc_bb(soc_bb);
    //update soc_bb values with more accurate values
    dcn42_apply_soc_bb_updates(soc_bb, dc, config);
}

unsafe fn dcn42b_get_ip_caps(ip_caps: *mut dml2_ip_capabilities) {
    core::ptr::copy_nonoverlapping(
        &dml2_dcn42b_max_ip_caps as *const dml2_ip_capabilities,
        ip_caps,
        1,
    );
}

static mut dcn42b_translator_funcs: soc_and_ip_translator_funcs = soc_and_ip_translator_funcs {
    get_soc_bb: Some(dcn42b_get_soc_bb),
    get_ip_caps: Some(dcn42b_get_ip_caps),
};

pub unsafe fn dcn42b_construct_soc_and_ip_translator(
    soc_and_ip_translator: *mut soc_and_ip_translator,
) {
    (*soc_and_ip_translator).translator_funcs =
        core::ptr::addr_of_mut!(dcn42b_translator_funcs);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
