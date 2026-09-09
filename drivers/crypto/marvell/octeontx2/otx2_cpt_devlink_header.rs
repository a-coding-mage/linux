/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2021 Marvell.
 */

// Dependencies supplied by the corresponding C headers:
// "otx2_cpt_common.h"
// "otx2_cptpf.h"

#[repr(C)]
pub struct devlink {
    _private: [u8; 0],
}

#[repr(C)]
pub struct otx2_cptpf_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct otx2_cpt_devlink {
    pub dl: *mut devlink,
    pub cptpf: *mut otx2_cptpf_dev,
}

/* Devlink APIs */
extern "C" {
    pub fn otx2_cpt_register_dl(cptpf: *mut otx2_cptpf_dev) -> ::std::os::raw::c_int;
    pub fn otx2_cpt_unregister_dl(cptpf: *mut otx2_cptpf_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
