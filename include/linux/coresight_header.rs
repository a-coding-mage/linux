/* SPDX-License-Identifier: GPL-2.0 */
/* C header translation: linux/coresight.h */

/* Dependencies supplied by the surrounding kernel translation. */

pub const CORESIGHT_PERIPHIDR4: u32 = 0xfd0;
pub const CORESIGHT_PERIPHIDR5: u32 = 0xfd4;
pub const CORESIGHT_PERIPHIDR6: u32 = 0xfd8;
pub const CORESIGHT_PERIPHIDR7: u32 = 0xfdc;
pub const CORESIGHT_PERIPHIDR0: u32 = 0xfe0;
pub const CORESIGHT_PERIPHIDR1: u32 = 0xfe4;
pub const CORESIGHT_PERIPHIDR2: u32 = 0xfe8;
pub const CORESIGHT_PERIPHIDR3: u32 = 0xfec;
pub const CORESIGHT_COMPIDR0: u32 = 0xff0;
pub const CORESIGHT_COMPIDR1: u32 = 0xff4;
pub const CORESIGHT_COMPIDR2: u32 = 0xff8;
pub const CORESIGHT_COMPIDR3: u32 = 0xffc;
pub const ETM_ARCH_V3_3: u32 = 0x23;
pub const ETM_ARCH_V3_5: u32 = 0x25;
pub const PFT_ARCH_V1_0: u32 = 0x30;
pub const PFT_ARCH_V1_1: u32 = 0x31;
pub const CORESIGHT_UNLOCK: u32 = 0xc5acce55;
pub const CORESIGHT_DESC_CPU_BOUND: u32 = 1 << 0;
pub const CORESIGHT_TRACE_IDS_MAX: usize = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum coresight_dev_type { CORESIGHT_DEV_TYPE_SINK, CORESIGHT_DEV_TYPE_LINK, CORESIGHT_DEV_TYPE_LINKSINK, CORESIGHT_DEV_TYPE_SOURCE, CORESIGHT_DEV_TYPE_HELPER, CORESIGHT_DEV_TYPE_MAX }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum coresight_dev_subtype_sink { CORESIGHT_DEV_SUBTYPE_SINK_DUMMY, CORESIGHT_DEV_SUBTYPE_SINK_PORT, CORESIGHT_DEV_SUBTYPE_SINK_BUFFER, CORESIGHT_DEV_SUBTYPE_SINK_SYSMEM, CORESIGHT_DEV_SUBTYPE_SINK_PERCPU_SYSMEM }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum coresight_dev_subtype_link { CORESIGHT_DEV_SUBTYPE_LINK_MERG, CORESIGHT_DEV_SUBTYPE_LINK_SPLIT, CORESIGHT_DEV_SUBTYPE_LINK_FIFO }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum coresight_dev_subtype_source { CORESIGHT_DEV_SUBTYPE_SOURCE_PROC, CORESIGHT_DEV_SUBTYPE_SOURCE_BUS, CORESIGHT_DEV_SUBTYPE_SOURCE_SOFTWARE, CORESIGHT_DEV_SUBTYPE_SOURCE_TPDM, CORESIGHT_DEV_SUBTYPE_SOURCE_OTHERS }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum coresight_dev_subtype_helper { CORESIGHT_DEV_SUBTYPE_HELPER_CATU, CORESIGHT_DEV_SUBTYPE_HELPER_ECT_CTI, CORESIGHT_DEV_SUBTYPE_HELPER_CTCU }

#[repr(C)]
pub union coresight_dev_subtype {
    pub sink_subtype: (coresight_dev_subtype_sink, coresight_dev_subtype_link),
    pub source_subtype: coresight_dev_subtype_source,
    pub helper_subtype: coresight_dev_subtype_helper,
}

#[repr(C)]
pub struct coresight_platform_data { pub nr_inconns: i32, pub nr_outconns: i32, pub out_conns: *mut *mut coresight_connection, pub in_conns: *mut *mut coresight_connection }
#[repr(C)]
pub union csdev_access_data {
    pub base: *mut core::ffi::c_void,
    pub callbacks: csdev_access_callbacks,
}
#[repr(C)]
pub struct csdev_access_callbacks { pub read: Option<unsafe extern "C" fn(u32, bool, bool) -> u64>, pub write: Option<unsafe extern "C" fn(u64, u32, bool, bool)> }
#[repr(C)]
pub struct csdev_access { pub io_mem: bool, pub data: csdev_access_data }
#[repr(C)]
pub struct coresight_desc { pub type_: coresight_dev_type, pub subtype: coresight_dev_subtype, pub ops: *const coresight_ops, pub pdata: *mut coresight_platform_data, pub dev: *mut device, pub groups: *const *const attribute_group, pub name: *const core::ffi::c_char, pub access: csdev_access, pub flags: u32, pub cpu: i32 }
#[repr(C)]
pub struct coresight_connection { pub src_port: i32, pub dest_port: i32, pub dest_fwnode: *mut fwnode_handle, pub dest_dev: *mut coresight_device, pub link: *mut coresight_sysfs_link, pub src_dev: *mut coresight_device, pub filter_src_fwnode: *mut fwnode_handle, pub filter_src_dev: *mut coresight_device, pub src_refcnt: i32, pub dest_refcnt: i32 }
#[repr(C)]
pub struct coresight_sysfs_link { pub orig: *mut coresight_device, pub orig_name: *const core::ffi::c_char, pub target: *mut coresight_device, pub target_name: *const core::ffi::c_char }
#[repr(C)]
pub struct coresight_trace_id_map { pub used_ids: [u8; 16], pub cpu_map: *mut atomic_t, pub perf_cs_etm_session_active: atomic_t, pub lock: raw_spinlock_t }
#[repr(C)]
pub struct coresight_device { pub pdata: *mut coresight_platform_data, pub type_: coresight_dev_type, pub subtype: coresight_dev_subtype, pub ops: *const coresight_ops, pub access: csdev_access, pub dev: device, pub path: *mut coresight_path, pub mode: atomic_t, pub refcnt: i32, pub cpu: i32, pub orphan: bool, pub sysfs_sink_activated: bool, pub ea: *mut dev_ext_attribute, pub def_sink: *mut coresight_device, pub perf_sink_id_map: coresight_trace_id_map, pub nr_links: i32, pub has_conns_grp: bool, pub feature_csdev_list: list_head, pub config_csdev_list: list_head, pub cscfg_csdev_lock: raw_spinlock_t, pub active_cscfg_ctxt: *mut core::ffi::c_void }
#[repr(C)]
pub struct coresight_dev_list { pub node: list_head, pub nr_idx: i32, pub pfx: *mut core::ffi::c_char, pub fwnode_list: *mut *mut fwnode_handle }
#[repr(C)]
pub struct coresight_path { pub path_list: list_head, pub trace_id: u8, pub handle: *mut perf_output_handle }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum cs_mode { CS_MODE_DISABLED = 0, CS_MODE_SYSFS = 1, CS_MODE_PERF = 2 }

#[repr(C)] pub struct coresight_ops_sink { pub enable: Option<unsafe extern "C" fn(*mut coresight_device, cs_mode, *mut coresight_path) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut coresight_device)>, pub alloc_buffer: Option<unsafe extern "C" fn(*mut coresight_device, *mut perf_event, *mut *mut core::ffi::c_void, i32, bool) -> *mut core::ffi::c_void>, pub free_buffer: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub update_buffer: Option<unsafe extern "C" fn(*mut coresight_device, *mut perf_output_handle, *mut core::ffi::c_void) -> usize> }
#[repr(C)] pub struct coresight_ops_link { pub enable: Option<unsafe extern "C" fn(*mut coresight_device, *mut coresight_connection, *mut coresight_connection) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut coresight_device, *mut coresight_connection, *mut coresight_connection)> }
#[repr(C)] pub struct coresight_ops_source { pub enable: Option<unsafe extern "C" fn(*mut coresight_device, *mut perf_event, cs_mode, *mut coresight_path) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut coresight_device, *mut perf_event)>, pub resume_perf: Option<unsafe extern "C" fn(*mut coresight_device) -> i32>, pub pause_perf: Option<unsafe extern "C" fn(*mut coresight_device)> }
#[repr(C)] pub struct coresight_ops_helper { pub enable: Option<unsafe extern "C" fn(*mut coresight_device, cs_mode, *mut coresight_path) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut coresight_device, *mut coresight_path) -> i32> }
#[repr(C)] pub struct coresight_ops_panic { pub sync: Option<unsafe extern "C" fn(*mut coresight_device) -> i32> }
#[repr(C)] pub struct coresight_ops { pub trace_id: Option<unsafe extern "C" fn(*mut coresight_device, cs_mode, *mut coresight_device) -> i32>, pub pm_save_disable: Option<unsafe extern "C" fn(*mut coresight_device) -> i32>, pub pm_restore_enable: Option<unsafe extern "C" fn(*mut coresight_device)>, pub sink_ops: *const coresight_ops_sink, pub link_ops: *const coresight_ops_link, pub source_ops: *const coresight_ops_source, pub helper_ops: *const coresight_ops_helper, pub panic_ops: *const coresight_ops_panic }

pub const fn CORESIGHT_CIDRn(i: u32) -> u32 { 0xff0 + i * 4 }
pub const fn CORESIGHT_PIDRn(i: u32) -> u32 { 0xfe0 + i * 4 }

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dev_ext_attribute { _private: [u8; 0] }
#[repr(C)] pub struct perf_output_handle { _private: [u8; 0] }
#[repr(C)] pub struct perf_event { _private: [u8; 0] }
#[repr(C)] pub struct amba_driver { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }

pub type coresight_timeout_cb_t = unsafe extern "C" fn(*mut csdev_access, u32, i32, i32);

extern "C" {
    pub static coresight_bustype: bus_type;
    pub fn coresight_register(desc: *mut coresight_desc) -> *mut coresight_device;
    pub fn coresight_unregister(csdev: *mut coresight_device);
    pub fn coresight_enable_sysfs(csdev: *mut coresight_device) -> i32;
    pub fn coresight_disable_sysfs(csdev: *mut coresight_device);
    pub fn coresight_timeout(csa: *mut csdev_access, offset: u32, position: i32, value: i32) -> i32;
    pub fn coresight_timeout_action(csa: *mut csdev_access, offset: u32, position: i32, value: i32, cb: coresight_timeout_cb_t) -> i32;
    pub fn coresight_claim_device(csdev: *mut coresight_device) -> i32;
    pub fn coresight_claim_device_unlocked(csdev: *mut coresight_device) -> i32;
    pub fn coresight_clear_self_claim_tag(csa: *mut csdev_access);
    pub fn coresight_clear_self_claim_tag_unlocked(csa: *mut csdev_access);
    pub fn coresight_disclaim_device(csdev: *mut coresight_device);
    pub fn coresight_disclaim_device_unlocked(csdev: *mut coresight_device);
    pub fn coresight_alloc_device_name(prefix: *const core::ffi::c_char, dev: *mut device) -> *mut core::ffi::c_char;
    pub fn coresight_loses_context_with_cpu(dev: *mut device) -> bool;
    pub fn coresight_relaxed_read32(csdev: *mut coresight_device, offset: u32) -> u32;
    pub fn coresight_read32(csdev: *mut coresight_device, offset: u32) -> u32;
    pub fn coresight_write32(csdev: *mut coresight_device, val: u32, offset: u32);
    pub fn coresight_relaxed_write32(csdev: *mut coresight_device, val: u32, offset: u32);
    pub fn coresight_relaxed_read64(csdev: *mut coresight_device, offset: u32) -> u64;
    pub fn coresight_read64(csdev: *mut coresight_device, offset: u32) -> u64;
    pub fn coresight_relaxed_write64(csdev: *mut coresight_device, val: u64, offset: u32);
    pub fn coresight_write64(csdev: *mut coresight_device, val: u64, offset: u32);
    pub fn coresight_get_cpu(dev: *mut device) -> i32;
    pub fn coresight_get_static_trace_id(dev: *mut device, id: *mut u32) -> i32;
    pub fn coresight_get_platform_data(dev: *mut device) -> *mut coresight_platform_data;
    pub fn coresight_add_out_conn(dev: *mut device, pdata: *mut coresight_platform_data, new_conn: *const coresight_connection) -> *mut coresight_connection;
    pub fn coresight_add_in_conn(conn: *mut coresight_connection) -> i32;
    pub fn coresight_find_input_type(pdata: *mut coresight_platform_data, type_: coresight_dev_type, subtype: coresight_dev_subtype) -> *mut coresight_device;
    pub fn coresight_find_output_type(pdata: *mut coresight_platform_data, type_: coresight_dev_type, subtype: coresight_dev_subtype) -> *mut coresight_device;
    pub fn coresight_init_driver_with_owner(drv: *const core::ffi::c_char, amba_drv: *mut amba_driver, pdev_drv: *mut platform_driver, owner: *mut module, mod_name: *const core::ffi::c_char) -> i32;
    pub fn coresight_remove_driver(amba_drv: *mut amba_driver, pdev_drv: *mut platform_driver);
    pub fn coresight_etm_get_trace_id(csdev: *mut coresight_device, mode: cs_mode, sink: *mut coresight_device) -> i32;
    pub fn coresight_get_enable_clocks(dev: *mut device, pclk: *mut *mut clk, atclk: *mut *mut clk) -> i32;
}

#[repr(C)] pub struct bus_type { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
