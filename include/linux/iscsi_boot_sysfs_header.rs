/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Export the iSCSI boot info to userland via sysfs.
 *
 * Copyright (C) 2010 Red Hat, Inc.  All rights reserved.
 * Copyright (C) 2010 Mike Christie
 */

/* Forward declarations supplied by kernel dependencies. */
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}
#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kset {
    _private: [u8; 0],
}

pub type ssize_t = isize;
pub type umode_t = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iscsi_boot_eth_properties_enum {
    ISCSI_BOOT_ETH_INDEX,
    ISCSI_BOOT_ETH_FLAGS,
    ISCSI_BOOT_ETH_IP_ADDR,
    ISCSI_BOOT_ETH_PREFIX_LEN,
    ISCSI_BOOT_ETH_SUBNET_MASK,
    ISCSI_BOOT_ETH_ORIGIN,
    ISCSI_BOOT_ETH_GATEWAY,
    ISCSI_BOOT_ETH_PRIMARY_DNS,
    ISCSI_BOOT_ETH_SECONDARY_DNS,
    ISCSI_BOOT_ETH_DHCP,
    ISCSI_BOOT_ETH_VLAN,
    ISCSI_BOOT_ETH_MAC,
    /* eth_pci_bdf - this is replaced by link to the device itself. */
    ISCSI_BOOT_ETH_HOSTNAME,
    ISCSI_BOOT_ETH_END_MARKER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iscsi_boot_tgt_properties_enum {
    ISCSI_BOOT_TGT_INDEX,
    ISCSI_BOOT_TGT_FLAGS,
    ISCSI_BOOT_TGT_IP_ADDR,
    ISCSI_BOOT_TGT_PORT,
    ISCSI_BOOT_TGT_LUN,
    ISCSI_BOOT_TGT_CHAP_TYPE,
    ISCSI_BOOT_TGT_NIC_ASSOC,
    ISCSI_BOOT_TGT_NAME,
    ISCSI_BOOT_TGT_CHAP_NAME,
    ISCSI_BOOT_TGT_CHAP_SECRET,
    ISCSI_BOOT_TGT_REV_CHAP_NAME,
    ISCSI_BOOT_TGT_REV_CHAP_SECRET,
    ISCSI_BOOT_TGT_END_MARKER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iscsi_boot_initiator_properties_enum {
    ISCSI_BOOT_INI_INDEX,
    ISCSI_BOOT_INI_FLAGS,
    ISCSI_BOOT_INI_ISNS_SERVER,
    ISCSI_BOOT_INI_SLP_SERVER,
    ISCSI_BOOT_INI_PRI_RADIUS_SERVER,
    ISCSI_BOOT_INI_SEC_RADIUS_SERVER,
    ISCSI_BOOT_INI_INITIATOR_NAME,
    ISCSI_BOOT_INI_END_MARKER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iscsi_boot_acpitbl_properties_enum {
    ISCSI_BOOT_ACPITBL_SIGNATURE,
    ISCSI_BOOT_ACPITBL_OEM_ID,
    ISCSI_BOOT_ACPITBL_OEM_TABLE_ID,
}

#[repr(C)]
pub struct iscsi_boot_kobj {
    pub kobj: kobject,
    pub attr_group: *mut attribute_group,
    pub list: list_head,
    /*
     * Pointer to store driver specific info. If set this will
     * be freed for the LLD when the kobj release function is called.
     */
    pub data: *mut core::ffi::c_void,
    /* Driver specific show function. */
    pub show: Option<unsafe extern "C" fn(
        data: *mut core::ffi::c_void,
        type_: i32,
        buf: *mut core::ffi::c_char,
    ) -> ssize_t>,
    /* Drivers specific visibility function. */
    pub is_visible: Option<unsafe extern "C" fn(
        data: *mut core::ffi::c_void,
        type_: i32,
    ) -> umode_t>,
    /* Driver specific release function. */
    pub release: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct iscsi_boot_kset {
    pub kobj_list: list_head,
    pub kset: *mut kset,
}

extern "C" {
    pub fn iscsi_boot_create_initiator(
        boot_kset: *mut iscsi_boot_kset,
        index: i32,
        data: *mut core::ffi::c_void,
        show: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_char) -> ssize_t>,
        is_visible: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> umode_t>,
        release: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    ) -> *mut iscsi_boot_kobj;
    pub fn iscsi_boot_create_ethernet(
        boot_kset: *mut iscsi_boot_kset, index: i32, data: *mut core::ffi::c_void,
        show: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_char) -> ssize_t>,
        is_visible: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> umode_t>,
        release: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    ) -> *mut iscsi_boot_kobj;
    pub fn iscsi_boot_create_target(
        boot_kset: *mut iscsi_boot_kset, index: i32, data: *mut core::ffi::c_void,
        show: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_char) -> ssize_t>,
        is_visible: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> umode_t>,
        release: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    ) -> *mut iscsi_boot_kobj;
    pub fn iscsi_boot_create_acpitbl(
        boot_kset: *mut iscsi_boot_kset, index: i32, data: *mut core::ffi::c_void,
        show: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_char) -> ssize_t>,
        is_visible: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> umode_t>,
        release: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    ) -> *mut iscsi_boot_kobj;
    pub fn iscsi_boot_create_kset(set_name: *const core::ffi::c_char) -> *mut iscsi_boot_kset;
    pub fn iscsi_boot_create_host_kset(hostno: u32) -> *mut iscsi_boot_kset;
    pub fn iscsi_boot_destroy_kset(boot_kset: *mut iscsi_boot_kset);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
