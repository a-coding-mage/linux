// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(C) 2020 Linaro Limited. All rights reserved.
 * Author: Mike Leach <mike.leach@linaro.org>
 */

// C dependencies: "coresight-config.h" and "coresight-syscfg.h".

/* create an alternate autofdo configuration */

/* we will provide 4 sets of preset parameter values */
const AFDO2_NR_PRESETS: usize = 4;
/* the total number of parameters in used features - strobing has 2 */
const AFDO2_NR_PARAM_SUM: usize = 2;

static AFDO2_REF_NAMES: [&'static [u8]; 1] = [b"strobing\0"];

/*
 * set of presets leaves strobing window constant while varying period to allow
 * experimentation with mark / space ratios for various workloads
 */
static mut AFDO2_PRESETS: [[u64; AFDO2_NR_PARAM_SUM]; AFDO2_NR_PRESETS] = [
    [1000, 100],
    [1000, 1000],
    [1000, 5000],
    [1000, 10000],
];

static mut AFDO2: crate::cscfg_config_desc = crate::cscfg_config_desc {
    name: b"autofdo2\0".as_ptr(),
    description: b"Setup ETMs with strobing for autofdo\nSupplied presets allow experimentation with mark-space ratio for various loads\n".as_ptr(),
    nr_feat_refs: AFDO2_REF_NAMES.len(),
    feat_ref_names: AFDO2_REF_NAMES.as_ptr(),
    nr_presets: AFDO2_NR_PRESETS,
    nr_total_params: AFDO2_NR_PARAM_SUM,
    presets: unsafe { AFDO2_PRESETS.as_ptr() as *const u64 },
};

static mut SAMPLE_FEATS: [*mut crate::cscfg_feature_desc; 1] = [core::ptr::null_mut()];

static mut SAMPLE_CFGS: [*mut crate::cscfg_config_desc; 2] = [
    core::ptr::addr_of_mut!(AFDO2),
    core::ptr::null_mut(),
];

static mut MOD_OWNER: crate::cscfg_load_owner_info = crate::cscfg_load_owner_info {
    type_: crate::CSCFG_OWNER_MODULE,
    owner_handle: crate::THIS_MODULE,
};

extern "C" {
    fn cscfg_load_config_sets(
        configs: *mut *mut crate::cscfg_config_desc,
        features: *mut *mut crate::cscfg_feature_desc,
        owner: *mut crate::cscfg_load_owner_info,
    ) -> core::ffi::c_int;
    fn cscfg_unload_config_sets(owner: *mut crate::cscfg_load_owner_info);
}

/* module init and exit - just load and unload configs */
#[no_mangle]
pub unsafe extern "C" fn cscfg_sample_init() -> core::ffi::c_int {
    cscfg_load_config_sets(
        SAMPLE_CFGS.as_mut_ptr(),
        SAMPLE_FEATS.as_mut_ptr(),
        core::ptr::addr_of_mut!(MOD_OWNER),
    )
}

#[no_mangle]
pub unsafe extern "C" fn cscfg_sample_exit() {
    cscfg_unload_config_sets(core::ptr::addr_of_mut!(MOD_OWNER));
}

// C module metadata:
// module_init(cscfg_sample_init);
// module_exit(cscfg_sample_exit);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Mike Leach <mike.leach@linaro.org>");
// MODULE_DESCRIPTION("CoreSight Syscfg Example");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
