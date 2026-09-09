/* SPDX-License-Identifier: GPL-2.0-only */

/* Dependencies supplied by <uapi/linux/ethtool.h> and "netlink.h". */

use core::ffi::c_char;

#[repr(C)]
pub struct ethnl_sock_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdevice_tracker {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

/**
 * struct ethnl_module_fw_flash_ntf_params - module firmware flashing
 *                                             notifications parameters
 * @portid: Netlink portid of sender.
 * @seq: Sequence number of sender.
 * @closed_sock: Indicates whether the socket was closed from user space.
 */
#[repr(C)]
pub struct ethnl_module_fw_flash_ntf_params {
    pub portid: u32,
    pub seq: u32,
    pub closed_sock: bool,
}

/**
 * struct ethtool_module_fw_flash_params - module firmware flashing parameters
 * @password: Module password. Only valid when @pass_valid is set.
 * @password_valid: Whether the module password is valid or not.
 */
#[repr(C)]
pub struct ethtool_module_fw_flash_params {
    pub password: u32,
    /* C bit-field: password_valid:1. */
    pub password_valid: u8,
}

/**
 * struct ethtool_cmis_fw_update_params - CMIS firmware update specific
 *                                         parameters
 * @dev: Pointer to the net_device to be flashed.
 * @params: Module firmware flashing parameters.
 * @ntf_params: Module firmware flashing notification parameters.
 * @fw: Firmware to flash.
 */
#[repr(C)]
pub struct ethtool_cmis_fw_update_params {
    pub dev: *mut net_device,
    pub params: ethtool_module_fw_flash_params,
    pub ntf_params: ethnl_module_fw_flash_ntf_params,
    pub fw: *const firmware,
}

/**
 * struct ethtool_module_fw_flash - module firmware flashing
 * @list: List node for &module_fw_flash_work_list.
 * @dev_tracker: Refcount tracker for @dev.
 * @work: The flashing firmware work.
 * @fw_update: CMIS firmware update specific parameters.
 */
#[repr(C)]
pub struct ethtool_module_fw_flash {
    pub list: list_head,
    pub dev_tracker: netdevice_tracker,
    pub work: work_struct,
    pub fw_update: ethtool_cmis_fw_update_params,
}

extern "C" {
    pub fn ethnl_module_fw_flash_sock_destroy(sk_priv: *mut ethnl_sock_priv);

    pub fn ethnl_module_fw_flash_ntf_err(
        dev: *mut net_device,
        params: *mut ethnl_module_fw_flash_ntf_params,
        err_msg: *mut c_char,
        sub_err_msg: *mut c_char,
    );

    pub fn ethnl_module_fw_flash_ntf_start(
        dev: *mut net_device,
        params: *mut ethnl_module_fw_flash_ntf_params,
    );

    pub fn ethnl_module_fw_flash_ntf_complete(
        dev: *mut net_device,
        params: *mut ethnl_module_fw_flash_ntf_params,
    );

    pub fn ethnl_module_fw_flash_ntf_in_progress(
        dev: *mut net_device,
        params: *mut ethnl_module_fw_flash_ntf_params,
        done: u64,
        total: u64,
    );

    pub fn ethtool_cmis_fw_update(params: *mut ethtool_cmis_fw_update_params);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
