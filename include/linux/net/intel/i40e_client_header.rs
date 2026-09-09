/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2013 - 2018 Intel Corporation. */

/* Translated from i40e_client.h.  linux/auxiliary_bus.h supplies the external
 * kernel types used below. */

pub const I40E_CLIENT_STR_LENGTH: usize = 10;
pub const I40E_CLIENT_VERSION_MAJOR: u8 = 0;
pub const I40E_CLIENT_VERSION_MINOR: u8 = 1;
pub const I40E_CLIENT_VERSION_BUILD: u8 = 0;
pub const I40E_CLIENT_VERSION_STR: &str = "0.1.0";

#[repr(C)]
pub struct i40e_client_version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
    pub rsvd: u8,
}

#[repr(C)]
pub enum i40e_client_instance_state {
    __I40E_CLIENT_INSTANCE_NONE,
    __I40E_CLIENT_INSTANCE_OPENED,
}

pub enum i40e_ops {}
pub enum i40e_client {}

pub const I40E_QUEUE_INVALID_IDX: u16 = 0xFFFF;

#[repr(C)]
pub struct i40e_qv_info {
    pub v_idx: u32, /* msix_vector */
    pub ceq_idx: u16,
    pub aeq_idx: u16,
    pub itr_idx: u8,
}

#[repr(C)]
pub struct i40e_qvlist_info {
    pub num_vectors: u32,
    pub qv_info: [i40e_qv_info; 0],
}

/* set of LAN parameters useful for clients managed by LAN */
/* Struct to hold per priority info */
#[repr(C)]
pub struct i40e_prio_qos_params {
    pub qs_handle: u16, /* qs handle for prio */
    pub tc: u8, /* TC mapped to prio */
    pub reserved: u8,
}

pub const I40E_CLIENT_MAX_USER_PRIORITY: usize = 8;
/* Struct to hold Client QoS */
#[repr(C)]
pub struct i40e_qos_params {
    pub prio_qos: [i40e_prio_qos_params; I40E_CLIENT_MAX_USER_PRIORITY],
}

#[repr(C)]
pub struct i40e_params {
    pub qos: i40e_qos_params,
    pub mtu: u16,
}

/* External kernel types. */
pub enum net_device {}
pub enum pci_dev {}
pub enum auxiliary_device {}
pub enum msix_entry {}
pub enum list_head {}
pub enum atomic_t {}

/* Structure to hold Lan device info for a client device */
#[repr(C)]
pub struct i40e_info {
    pub version: i40e_client_version,
    pub lanmac: [u8; 6],
    pub netdev: *mut net_device,
    pub pcidev: *mut pci_dev,
    pub aux_dev: *mut auxiliary_device,
    pub hw_addr: *mut u8,
    pub fid: u8, /* function id, PF id or VF id */
    pub ftype: u8, /* function type, PF or VF */
    pub pf: *mut core::ffi::c_void,
    pub qvlist_info: *mut i40e_qvlist_info,
    pub params: i40e_params,
    pub ops: *mut i40e_ops,
    pub msix_count: u16, /* number of msix vectors*/
    pub msix_entries: *mut msix_entry,
    pub itr_index: u16, /* Which ITR index the PE driver is suppose to use */
    pub fw_maj_ver: u16, /* firmware major version */
    pub fw_min_ver: u16, /* firmware minor version */
    pub fw_build: u32, /* firmware build number */
}

pub const I40E_CLIENT_FTYPE_PF: u8 = 0;

#[repr(C)]
pub struct i40e_auxiliary_device {
    pub aux_dev: auxiliary_device,
    pub ldev: *mut i40e_info,
}

pub const I40E_CLIENT_RESET_LEVEL_PF: u32 = 1;
pub const I40E_CLIENT_RESET_LEVEL_CORE: u32 = 2;
pub const I40E_CLIENT_VSI_FLAG_TCP_ENABLE: u32 = 1u32 << 1;

#[repr(C)]
pub struct i40e_ops {
    /* setup_q_vector_list enables queues with a particular vector */
    pub setup_qvlist: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, *mut i40e_qvlist_info) -> i32>,
    pub virtchnl_send: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, u32, *mut u8, u16) -> i32>,
    pub request_reset: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, u32)>,
    pub update_vsi_ctxt: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, bool, u32, u32, u32) -> i32>,
}

#[repr(C)]
pub struct i40e_client_ops {
    pub open: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, bool)>,
    pub l2_param_change: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, *mut i40e_params)>,
    pub virtchnl_receive: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, u32, *mut u8, u16) -> i32>,
    pub vf_reset: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, u32)>,
    pub vf_enable: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, u32)>,
    pub vf_capable: Option<unsafe extern "C" fn(*mut i40e_info, *mut i40e_client, u32) -> i32>,
}

#[repr(C)]
pub struct i40e_client_instance {
    pub list: list_head,
    pub lan_info: i40e_info,
    pub client: *mut i40e_client,
    pub state: usize,
}

#[repr(C)]
pub struct i40e_client {
    pub list: list_head,
    pub name: [core::ffi::c_char; I40E_CLIENT_STR_LENGTH],
    pub version: i40e_client_version,
    pub state: usize,
    pub ref_cnt: atomic_t,
    pub flags: u32,
    pub type_: u8,
    pub ops: *const i40e_client_ops,
}

pub const I40E_CLIENT_IWARP: u8 = 0;

extern "C" {
    pub fn i40e_client_device_register(ldev: *mut i40e_info, client: *mut i40e_client);
    pub fn i40e_client_device_unregister(ldev: *mut i40e_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
