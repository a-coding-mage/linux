/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

// Dependency declarations are supplied by the translated qed interface.

/* Structs used by PF to control and manipulate child VFs */
#[repr(C)]
pub struct qed_iov_hv_ops {
    pub configure:
        Option<unsafe extern "C" fn(cdev: *mut qed_dev, num_vfs_param: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,

    pub set_mac:
        Option<unsafe extern "C" fn(cdev: *mut qed_dev, mac: *mut u8, vfid: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,

    pub set_vlan:
        Option<unsafe extern "C" fn(cdev: *mut qed_dev, vid: u16, vfid: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,

    pub get_config:
        Option<unsafe extern "C" fn(
            cdev: *mut qed_dev,
            vf_id: ::std::os::raw::c_int,
            ivi: *mut ifla_vf_info,
        ) -> ::std::os::raw::c_int>,

    pub set_link_state:
        Option<unsafe extern "C" fn(
            cdev: *mut qed_dev,
            vf_id: ::std::os::raw::c_int,
            link_state: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int>,

    pub set_spoof: Option<unsafe extern "C" fn(
        cdev: *mut qed_dev,
        vfid: ::std::os::raw::c_int,
        val: bool,
    ) -> ::std::os::raw::c_int>,

    pub set_rate: Option<unsafe extern "C" fn(
        cdev: *mut qed_dev,
        vfid: ::std::os::raw::c_int,
        min_rate: u32,
        max_rate: u32,
    ) -> ::std::os::raw::c_int>,

    pub set_trust: Option<unsafe extern "C" fn(
        cdev: *mut qed_dev,
        vfid: ::std::os::raw::c_int,
        trust: bool,
    ) -> ::std::os::raw::c_int>,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
