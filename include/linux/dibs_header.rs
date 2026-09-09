/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Internal Buffer Sharing definitions. */

/* External kernel types are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct dibs_dmb {
    pub dmb_tok: u64,
    pub rgid: uuid_t,
    pub cpu_addr: *mut core::ffi::c_void,
    pub dmb_len: u32,
    pub idx: u32,
    pub vlan_valid: u32,
    pub vlan_id: u32,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dibs_event_type {
    DIBS_BUF_EVENT,
    DIBS_DEV_EVENT,
    DIBS_SW_EVENT,
    DIBS_OTHER_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dibs_event_subtype {
    DIBS_BUF_UNREGISTERED,
    DIBS_DEV_DISABLED,
    DIBS_DEV_ERR_STATE,
    DIBS_OTHER_SUBTYPE,
}

#[repr(C)]
pub struct dibs_event {
    pub type_: u32,
    pub subtype: u32,
    pub gid: uuid_t,
    pub buffer_tok: u64,
    pub time: u64,
    pub data: u64,
}

pub const MAX_DIBS_CLIENTS: usize = 8;
pub const NO_DIBS_CLIENT: u8 = 0xff;

pub struct dibs_dev;
pub struct dibs_client;

#[repr(C)]
pub struct dibs_client_ops {
    pub add_dev: Option<unsafe extern "C" fn(dev: *mut dibs_dev)>,
    pub del_dev: Option<unsafe extern "C" fn(dev: *mut dibs_dev)>,
    pub handle_irq: Option<unsafe extern "C" fn(dev: *mut dibs_dev, idx: core::ffi::c_uint, dmbemask: u16)>,
    pub handle_event: Option<unsafe extern "C" fn(dev: *mut dibs_dev, event: *const dibs_event)>,
}

#[repr(C)]
pub struct dibs_client {
    pub name: *const core::ffi::c_char,
    pub ops: *const dibs_client_ops,
    pub id: u8,
}

extern "C" {
    pub fn dibs_register_client(client: *mut dibs_client) -> core::ffi::c_int;
    pub fn dibs_unregister_client(client: *mut dibs_client) -> core::ffi::c_int;
}

pub const DIBS_LOOPBACK_FABRIC: u16 = 0xffff;

#[repr(C)]
pub struct dibs_dev_ops {
    pub get_fabric_id: Option<unsafe extern "C" fn(dev: *mut dibs_dev) -> u16>,
    pub query_remote_gid: Option<unsafe extern "C" fn(dev: *mut dibs_dev, rgid: *const uuid_t, vid_valid: u32, vid: u32) -> core::ffi::c_int>,
    pub max_dmbs: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub register_dmb: Option<unsafe extern "C" fn(dev: *mut dibs_dev, dmb: *mut dibs_dmb, client: *mut dibs_client) -> core::ffi::c_int>,
    pub unregister_dmb: Option<unsafe extern "C" fn(dev: *mut dibs_dev, dmb: *mut dibs_dmb) -> core::ffi::c_int>,
    pub move_data: Option<unsafe extern "C" fn(dev: *mut dibs_dev, dmb_tok: u64, idx: core::ffi::c_uint, sf: bool, offset: core::ffi::c_uint, data: *mut core::ffi::c_void, size: core::ffi::c_uint) -> core::ffi::c_int>,
    pub add_vlan_id: Option<unsafe extern "C" fn(dev: *mut dibs_dev, vlan_id: u64) -> core::ffi::c_int>,
    pub del_vlan_id: Option<unsafe extern "C" fn(dev: *mut dibs_dev, vlan_id: u64) -> core::ffi::c_int>,
    pub signal_event: Option<unsafe extern "C" fn(dev: *mut dibs_dev, rgid: *const uuid_t, trigger_irq: u32, event_code: u32, info: u64) -> core::ffi::c_int>,
    pub support_mmapped_rdmb: Option<unsafe extern "C" fn(dev: *mut dibs_dev) -> core::ffi::c_int>,
    pub attach_dmb: Option<unsafe extern "C" fn(dev: *mut dibs_dev, dmb: *mut dibs_dmb) -> core::ffi::c_int>,
    pub detach_dmb: Option<unsafe extern "C" fn(dev: *mut dibs_dev, token: u64) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct dibs_dev {
    pub list: list_head,
    pub dev: device,
    pub ops: *const dibs_dev_ops,
    pub gid: uuid_t,
    pub drv_priv: *mut core::ffi::c_void,
    pub priv_: [*mut core::ffi::c_void; MAX_DIBS_CLIENTS],
    pub lock: spinlock_t,
    pub dmb_clientid_arr: *mut u8,
    pub subs: [*mut dibs_client; MAX_DIBS_CLIENTS],
}

#[inline]
pub unsafe fn dibs_set_priv(dev: *mut dibs_dev, client: *mut dibs_client, priv_: *mut core::ffi::c_void) {
    (*dev).priv_[(*client).id as usize] = priv_;
}

#[inline]
pub unsafe fn dibs_get_priv(dev: *mut dibs_dev, client: *mut dibs_client) -> *mut core::ffi::c_void {
    (*dev).priv_[(*client).id as usize]
}

extern "C" {
    pub fn dibs_dev_alloc() -> *mut dibs_dev;
    pub fn dibs_dev_add(dibs: *mut dibs_dev) -> core::ffi::c_int;
    pub fn dibs_dev_del(dibs: *mut dibs_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
