// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2021 Intel Corporation. All rights reserved.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type u32_t = u32;
type u64_t = u64;
type ssize_t = isize;
type size_t = usize;
type resource_size_t = u64;
type acpi_handle = *mut c_void;
type acpi_string = *const c_char;
type acpi_status = c_int;
type walk_hmem_fn = unsafe extern "C" fn(*mut device, c_int, *mut resource) -> c_int;
type acpi_tbl_entry_handler_arg =
    unsafe extern "C" fn(*mut acpi_subtable_headers, *mut c_void, c_ulong);

const true_: bool = true;
const false_: bool = false;
const NULL: *mut c_void = ptr::null_mut();

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const EOPNOTSUPP: c_int = 95;
const GFP_KERNEL: c_int = 0;
const NUMA_NO_NODE: c_int = -1;
const AE_OK: acpi_status = 0;
const CXL_RESOURCE_NONE: resource_size_t = !0;
const REGION_INTERSECTS: c_int = 1;

const SZ_2M: resource_size_t = 2 * 1024 * 1024;
const SZ_256M: resource_size_t = 256 * 1024 * 1024;
const SZ_512M: resource_size_t = 512 * 1024 * 1024;
const SZ_32G: resource_size_t = 32 * 1024 * 1024 * 1024;
const SZ_64G: resource_size_t = SZ_32G * 2;
const PMD_SIZE: resource_size_t = 2 * 1024 * 1024;

const FAKE_QTG_ID: c_int = 42;
const NR_CXL_HOST_BRIDGES: usize = 2;
const NR_CXL_SINGLE_HOST: usize = 1;
const NR_CXL_RCH: usize = 1;
const NR_CXL_ROOT_PORTS: usize = 2;
const NR_CXL_SWITCH_PORTS: usize = 2;
const NR_CXL_PORT_DECODERS: usize = 8;
const NR_BRIDGES: usize = NR_CXL_HOST_BRIDGES + NR_CXL_SINGLE_HOST + NR_CXL_RCH;
const NR_CXL_TYPE2_ACCEL: usize = 1;
const NR_MULTI_ROOT: usize = NR_CXL_HOST_BRIDGES * NR_CXL_ROOT_PORTS;
const NR_MEM_MULTI: usize = NR_CXL_HOST_BRIDGES * NR_CXL_ROOT_PORTS * NR_CXL_SWITCH_PORTS;
const NR_MEM_SINGLE: usize = NR_CXL_SINGLE_HOST * NR_CXL_SWITCH_PORTS;
const MOCK_AUTO_REGION_SIZE_DEFAULT: c_int = SZ_512M as c_int;

const CFMWS_MOD_ARRAY_START: c_int = 0;
const CFMWS_MOD_ARRAY_END: c_int = 5;
const CFMWS_XOR_ARRAY_START: c_int = 6;
const CFMWS_XOR_ARRAY_END: c_int = 8;

const ACPI_CEDT_TYPE_CHBS: c_int = 0;
const ACPI_CEDT_TYPE_CFMWS: c_int = 1;
const ACPI_CEDT_TYPE_CXIMS: c_int = 2;
const ACPI_CEDT_CHBS_VERSION_CXL11: u32 = 1;
const ACPI_CEDT_CHBS_VERSION_CXL20: u32 = 2;
const ACPI_CEDT_CHBS_LENGTH_CXL11: resource_size_t = 0x1000;
const ACPI_CEDT_CHBS_LENGTH_CXL20: resource_size_t = 0x1000;
const ACPI_CEDT_CFMWS_RESTRICT_HOSTONLYMEM: u32 = 1 << 0;
const ACPI_CEDT_CFMWS_RESTRICT_DEVMEM: u32 = 1 << 1;
const ACPI_CEDT_CFMWS_RESTRICT_VOLATILE: u32 = 1 << 2;
const ACPI_CEDT_CFMWS_RESTRICT_PMEM: u32 = 1 << 3;
const ACPI_CEDT_CFMWS_ARITHMETIC_XOR: u8 = 1;
const CXL_DECODER_F_ENABLE: c_ulong = 1;
const CXL_DECODER_HOSTONLYMEM: c_int = 0;
const CXL_DECODER_DEVMEM: c_int = 1;
const CXL_DECODER_STATE_AUTO: c_int = 1;
const CXL_DECODER_STATE_MANUAL: c_int = 2;
const ACCESS_COORDINATE_MAX: usize = 2;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] #[derive(Copy, Clone)] pub struct range { pub start: resource_size_t, pub end: resource_size_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct resource { pub start: resource_size_t, pub end: resource_size_t }
#[repr(C)] pub struct kobject { _priv: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { pub dev: *mut device }
#[repr(C)] pub struct device {
    pub parent: *mut device,
    pub fwnode: *mut fwnode_handle,
    pub bus: *mut c_void,
    pub groups: *const *const attribute_group,
    pub kobj: kobject,
}
#[repr(C)] pub struct platform_device { pub dev: device, pub id: c_int }
#[repr(C)] pub struct acpi_pnp { pub unique_id: *const c_char }
#[repr(C)] pub struct acpi_device { pub handle: acpi_handle, pub pnp: acpi_pnp, pub dev: device, pub fwnode: fwnode_handle }
#[repr(C)] pub struct acpi_object_list { _priv: [u8; 0] }
#[repr(C)] pub struct acpi_table_header { pub signature: [u8; 4], pub length: u32, pub revision: u8 }
#[repr(C)] pub struct acpi_table_cedt { pub header: acpi_table_header }
#[repr(C)] pub struct acpi_cedt_header { pub type_: u16, pub length: u16 }
#[repr(C)] pub struct acpi_cedt_chbs { pub header: acpi_cedt_header, pub uid: u32, pub cxl_version: u32, pub base: resource_size_t, pub length: resource_size_t }
#[repr(C)] pub struct acpi_cedt_cfmws { pub header: acpi_cedt_header, pub base_hpa: resource_size_t, pub window_size: resource_size_t, pub interleave_ways: u8, pub interleave_arithmetic: u8, pub granularity: u8, pub restrictions: u32, pub qtg_id: u32 }
#[repr(C)] pub struct acpi_cedt_cxims { pub header: acpi_cedt_header, pub hbig: u32, pub nr_xormaps: u32 }
#[repr(C)] pub union acpi_subtable_headers { pub common: acpi_cedt_header }
#[repr(C)] pub struct gen_pool { _priv: [u8; 0] }
#[repr(C)] pub struct genpool_data_align { pub align: c_int }
#[repr(C)] pub struct pci_bus { pub bridge: *mut device }
#[repr(C)] pub struct acpi_pci_root { pub bus: *mut pci_bus }
#[repr(C)] pub struct attribute { _priv: [u8; 0] }
#[repr(C)] pub struct attribute_group { _priv: [u8; 0] }
#[repr(C)] pub struct device_attribute { pub attr: attribute }
#[repr(C)] pub struct cxl_hdm { pub port: *mut cxl_port, pub interleave_mask: u32, pub iw_cap_mask: c_ulong }
#[repr(C)] pub struct cxl_endpoint_dvsec_info { _priv: [u8; 0] }
#[repr(C)] pub struct cxl_port { pub dev: device, pub uport_dev: *mut device, pub parent_dport: *mut cxl_dport, pub commit_end: c_int, pub id: c_int, pub depth: c_int }
#[repr(C)] pub struct cxl_dport { pub port: *mut cxl_port, pub port_id: c_int }
#[repr(C)] pub struct cxl_decoder {
    pub dev: device,
    pub id: c_int,
    pub hpa_range: range,
    pub interleave_ways: c_int,
    pub interleave_granularity: c_int,
    pub target_type: c_int,
    pub flags: c_ulong,
    pub target_map: [u32; NR_CXL_SWITCH_PORTS],
    pub commit: Option<unsafe extern "C" fn(*mut cxl_decoder) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut cxl_decoder)>,
}
#[repr(C)] pub struct cxl_switch_decoder { pub cxld: cxl_decoder, pub nr_targets: c_int, pub target: [*mut cxl_dport; NR_CXL_SWITCH_PORTS] }
#[repr(C)] pub struct cxl_endpoint_decoder { pub cxld: cxl_decoder, pub state: c_int, pub skip: c_int, pub dpa_res: *mut resource }
#[repr(C)] pub struct cxl_memdev { pub dev: device, pub cxlds: *mut cxl_dev_state }
#[repr(C)] pub struct cxl_dev_state { pub nr_partitions: c_int, pub part: [cxl_partition; 4] }
#[repr(C)] pub struct cxl_partition { pub res: resource, pub perf: cxl_dpa_perf }
#[repr(C)] pub struct access_coordinate { pub read_latency: u32, pub write_latency: u32, pub read_bandwidth: u32, pub write_bandwidth: u32 }
#[repr(C)] pub struct cxl_dpa_perf { pub qos_class: c_int, pub dpa_range: range, pub coord: [access_coordinate; ACCESS_COORDINATE_MAX] }
#[repr(C)] pub struct cxl_root { _priv: [u8; 0] }

#[repr(C)]
pub struct cxl_mock_ops {
    pub is_mock_adev: Option<unsafe extern "C" fn(*mut acpi_device) -> bool>,
    pub is_mock_bridge: Option<unsafe extern "C" fn(*mut device) -> bool>,
    pub is_mock_bus: Option<unsafe extern "C" fn(*mut pci_bus) -> bool>,
    pub is_mock_port: Option<unsafe extern "C" fn(*mut device) -> bool>,
    pub is_mock_dev: Option<unsafe extern "C" fn(*mut device) -> bool>,
    pub acpi_table_parse_cedt: Option<unsafe extern "C" fn(c_int, acpi_tbl_entry_handler_arg, *mut c_void) -> c_int>,
    pub acpi_evaluate_integer: Option<unsafe extern "C" fn(acpi_handle, acpi_string, *mut acpi_object_list, *mut u64) -> acpi_status>,
    pub acpi_pci_find_root: Option<unsafe extern "C" fn(acpi_handle) -> *mut acpi_pci_root>,
    pub devm_cxl_switch_port_decoders_setup: Option<unsafe extern "C" fn(*mut cxl_port) -> c_int>,
    pub devm_cxl_endpoint_decoders_setup: Option<unsafe extern "C" fn(*mut cxl_port) -> c_int>,
    pub cxl_endpoint_parse_cdat: Option<unsafe extern "C" fn(*mut cxl_port)>,
    pub devm_cxl_add_dport_by_dev: Option<unsafe extern "C" fn(*mut cxl_port, *mut device) -> *mut cxl_dport>,
    pub hmat_get_extended_linear_cache_size: Option<unsafe extern "C" fn(*mut resource, c_int, *mut resource_size_t) -> c_int>,
    pub walk_hmem_resources: Option<unsafe extern "C" fn(*mut device, walk_hmem_fn) -> c_int>,
    pub region_intersects: Option<unsafe extern "C" fn(resource_size_t, size_t, c_ulong, c_ulong) -> c_int>,
    pub region_intersects_soft_reserve: Option<unsafe extern "C" fn(resource_size_t, size_t) -> c_int>,
    pub list: list_head,
}

#[repr(C)] struct cxl_mock_res { list: list_head, range: range }
#[repr(C)] struct target_map_ctx { target_map: *mut u32, index: c_int, target_count: c_int }
#[repr(C)] struct cxl_cedt_context { dev: *mut device }
#[repr(C)] union cxl_test_decoder_union { cxlsd: core::mem::ManuallyDrop<cxl_switch_decoder>, cxled: core::mem::ManuallyDrop<cxl_endpoint_decoder> }
#[repr(C)] struct cxl_test_decoder { u: cxl_test_decoder_union, dpa_range: range }

unsafe extern "C" {
    static mut hmem_test: bool;
    static mut platform_bus_type: c_void;
    static mut iomem_resource: resource;
    static dev_attr_decoder_reset_preserve_registry: device_attribute;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn sysfs_create_link(kobj: *mut kobject, target: *mut kobject, name: *const c_char) -> c_int;
    fn sysfs_remove_link(kobj: *mut kobject, name: *const c_char);
    fn device_initialize(dev: *mut device);
    fn fwnode_init(fwnode: *mut fwnode_handle, ops: *mut c_void);
    fn set_dev_node(dev: *mut device, nid: c_int);
    fn gen_pool_create(min_alloc_order: c_int, nid: c_int) -> *mut gen_pool;
    fn gen_pool_add(pool: *mut gen_pool, addr: resource_size_t, size: resource_size_t, nid: c_int) -> c_int;
    fn gen_pool_destroy(pool: *mut gen_pool);
    fn gen_pool_alloc_algo(pool: *mut gen_pool, size: resource_size_t, algo: *mut c_void, data: *mut genpool_data_align) -> c_ulong;
    fn gen_pool_free(pool: *mut gen_pool, addr: resource_size_t, size: resource_size_t);
    static mut gen_pool_first_fit_align: c_void;
    fn range_len(r: *const range) -> resource_size_t;
    fn resource_contains(a: *const resource, b: *const resource) -> bool;
    fn resource_overlaps(a: *const resource, b: *const resource) -> bool;
    fn acpi_table_parse_cedt(id: c_int, handler: acpi_tbl_entry_handler_arg, arg: *mut c_void) -> c_int;
    fn acpi_evaluate_integer(handle: acpi_handle, pathname: acpi_string, arguments: *mut acpi_object_list, data: *mut u64) -> acpi_status;
    fn hmat_get_extended_linear_cache_size(backing_res: *mut resource, nid: c_int, cache_size: *mut resource_size_t) -> c_int;
    fn acpi_pci_find_root(handle: acpi_handle) -> *mut acpi_pci_root;
    fn is_cxl_memdev(dev: *mut device) -> bool;
    fn is_switch_decoder(dev: *mut device) -> bool;
    fn is_endpoint_decoder(dev: *mut device) -> bool;
    fn is_cxl_endpoint(port: *mut cxl_port) -> bool;
    fn is_cxl_root(port: *mut cxl_port) -> bool;
    fn is_cxl_port(dev: *mut device) -> bool;
    fn dev_is_platform(dev: *mut device) -> bool;
    fn to_cxl_port(dev: *mut device) -> *mut cxl_port;
    fn to_cxl_decoder(dev: *mut device) -> *mut cxl_decoder;
    fn to_cxl_switch_decoder(dev: *mut device) -> *mut cxl_switch_decoder;
    fn to_cxl_endpoint_decoder(dev: *mut device) -> *mut cxl_endpoint_decoder;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn to_cxl_memdev(dev: *mut device) -> *mut cxl_memdev;
    fn cxled_to_memdev(cxled: *mut cxl_endpoint_decoder) -> *mut cxl_memdev;
    fn cxled_to_port(cxled: *mut cxl_endpoint_decoder) -> *mut cxl_port;
    fn cxl_switch_decoder_alloc(port: *mut cxl_port, nr_targets: c_int) -> *mut cxl_switch_decoder;
    fn cxl_endpoint_decoder_alloc(port: *mut cxl_port) -> *mut cxl_endpoint_decoder;
    fn cxl_decoder_add_locked(cxld: *mut cxl_decoder) -> c_int;
    fn cxl_decoder_autoremove(dev: *mut device, cxld: *mut cxl_decoder) -> c_int;
    fn device_for_each_child(dev: *mut device, data: *mut c_void, fn_: unsafe extern "C" fn(*mut device, *mut c_void) -> c_int) -> c_int;
    fn device_find_child(dev: *mut device, data: *mut c_void, match_: unsafe extern "C" fn(*mut device, *const c_void) -> c_int) -> *mut device;
    fn put_device(dev: *mut device);
    fn devm_cxl_add_dport(port: *mut cxl_port, dev: *mut device, id: c_int, res: resource_size_t) -> *mut cxl_dport;
    fn devm_cxl_dpa_reserve(cxled: *mut cxl_endpoint_decoder, start: resource_size_t, len: resource_size_t, skip: c_int) -> c_int;
    fn cxl_num_decoders_committed(port: *mut cxl_port) -> c_int;
    fn cxl_port_commit_reap(cxld: *mut cxl_decoder);
    fn cxl_port_update_decoder_targets(port: *mut cxl_port, dport: *mut cxl_dport);
    fn eig_to_granularity(eig: u8, granularity: *mut c_int);
    fn cxl_memdev_update_perf(cxlmd: *mut cxl_memdev);
    fn cxl_endpoint_get_perf_coordinates(port: *mut cxl_port, coord: *mut access_coordinate);
    fn find_cxl_root(port: *mut cxl_port) -> *mut cxl_root;
    fn mhp_get_pluggable_range(need_mapping: bool) -> range;
    fn cxl_acpi_test(); fn cxl_core_test(); fn cxl_mem_test(); fn cxl_pmem_test(); fn cxl_port_test();
    fn register_cxl_mock_ops(ops: *mut cxl_mock_ops);
    fn unregister_cxl_mock_ops(ops: *mut cxl_mock_ops);
    fn hmem_test_init() -> c_int; fn hmem_test_exit();
    fn xa_load(xa: *mut c_void, index: c_ulong) -> *mut cxl_test_decoder;
    fn xa_insert(xa: *mut c_void, index: c_ulong, entry: *mut cxl_test_decoder, flags: c_int) -> c_int;
    fn xa_erase(xa: *mut c_void, index: c_ulong);
    fn xa_destroy(xa: *mut c_void);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn kstrtobool(buf: *const c_char, res: *mut bool) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn pr_err_once(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_WARN_ONCE(dev: *mut device, condition: c_int, fmt: *const c_char, ...) -> bool;
    fn WARN_ON(condition: bool) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

static mut interleave_arithmetic: c_int = 0;
static mut extended_linear_cache: bool = false;
static mut fail_autoassemble: bool = false;
static mut type2_test: bool = false;
static mut mock_auto_region_size: c_int = MOCK_AUTO_REGION_SIZE_DEFAULT;

static mut cxl_acpi: *mut platform_device = ptr::null_mut();
static mut cxl_host_bridge: [*mut platform_device; NR_CXL_HOST_BRIDGES] = [ptr::null_mut(); NR_CXL_HOST_BRIDGES];
static mut cxl_root_port: [*mut platform_device; NR_MULTI_ROOT] = [ptr::null_mut(); NR_MULTI_ROOT];
static mut cxl_switch_uport: [*mut platform_device; NR_MULTI_ROOT] = [ptr::null_mut(); NR_MULTI_ROOT];
static mut cxl_switch_dport: [*mut platform_device; NR_MEM_MULTI] = [ptr::null_mut(); NR_MEM_MULTI];
static mut cxl_hb_single: [*mut platform_device; NR_CXL_SINGLE_HOST] = [ptr::null_mut(); NR_CXL_SINGLE_HOST];
static mut cxl_root_single: [*mut platform_device; NR_CXL_SINGLE_HOST] = [ptr::null_mut(); NR_CXL_SINGLE_HOST];
static mut cxl_swu_single: [*mut platform_device; NR_CXL_SINGLE_HOST] = [ptr::null_mut(); NR_CXL_SINGLE_HOST];
static mut cxl_swd_single: [*mut platform_device; NR_MEM_SINGLE] = [ptr::null_mut(); NR_MEM_SINGLE];
#[no_mangle] pub static mut cxl_mem: [*mut platform_device; NR_MEM_MULTI] = [ptr::null_mut(); NR_MEM_MULTI];
#[no_mangle] pub static mut cxl_mem_single: [*mut platform_device; NR_MEM_SINGLE] = [ptr::null_mut(); NR_MEM_SINGLE];
static mut cxl_rch: [*mut platform_device; NR_CXL_RCH] = [ptr::null_mut(); NR_CXL_RCH];
static mut cxl_rcd: [*mut platform_device; NR_CXL_RCH] = [ptr::null_mut(); NR_CXL_RCH];

static mut decoder_registry: *mut c_void = ptr::null_mut();
static mut decoder_reset_preserve_registry: bool = false;
static mut mock_res_lock: c_void = ();
static mut cxl_mock_pool: *mut gen_pool = ptr::null_mut();

static mut acpi0017_mock: acpi_device = unsafe { core::mem::zeroed() };
static mut host_bridge: [acpi_device; NR_BRIDGES] = unsafe { core::mem::zeroed() };
static mut mock_pci_bus: [pci_bus; NR_BRIDGES] = unsafe { core::mem::zeroed() };
static mut mock_pci_root: [acpi_pci_root; NR_BRIDGES] = unsafe { core::mem::zeroed() };
static mut cfmws_start: c_int = 0;
static mut cfmws_end: c_int = 0;

#[repr(C)] struct cfmws1 { cfmws: acpi_cedt_cfmws, target: [u32; 1] }
#[repr(C)] struct cfmws2 { cfmws: acpi_cedt_cfmws, target: [u32; 2] }
#[repr(C)] struct cfmws3 { cfmws: acpi_cedt_cfmws, target: [u32; 3] }
#[repr(C)] struct cxims0 { cxims: acpi_cedt_cxims, xormap_list: [u64; 2] }
#[repr(C)] struct mock_cedt_t { cedt: acpi_table_cedt, chbs: [acpi_cedt_chbs; NR_BRIDGES], cfmws0: cfmws1, cfmws1: cfmws2, cfmws2: cfmws1, cfmws3: cfmws2, cfmws4: cfmws1, cfmws5: cfmws1, cfmws6: cfmws1, cfmws7: cfmws2, cfmws8: cfmws3, cxims0: cxims0 }
static mut mock_cedt: mock_cedt_t = unsafe { core::mem::zeroed() };
static mut type2_cfmws0: acpi_cedt_cfmws = unsafe { core::mem::zeroed() };
static mut mock_cfmws: [*mut acpi_cedt_cfmws; 9] = [ptr::null_mut(); 9];
static mut mock_cxims: [*mut acpi_cedt_cxims; 1] = [ptr::null_mut(); 1];

unsafe fn define_res_mem(start: resource_size_t, size: resource_size_t) -> resource {
    resource { start, end: start + size - 1 }
}
unsafe fn ERR_PTR<T>(err: c_int) -> *mut T { err as isize as *mut T }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }
unsafe fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
unsafe fn max_t(a: c_int, b: resource_size_t) -> c_int { if (a as resource_size_t) > b { a } else { b as c_int } }
unsafe fn min(a: resource_size_t, b: resource_size_t) -> resource_size_t { if a < b { a } else { b } }
unsafe fn IS_ALIGNED(x: c_int, a: resource_size_t) -> bool { (x as resource_size_t) & (a - 1) == 0 }
unsafe fn ilog2(mut x: resource_size_t) -> c_int { let mut r = -1; while x != 0 { x >>= 1; r += 1; } r }

unsafe extern "C" fn is_multi_bridge(dev: *mut device) -> bool {
    for i in 0..NR_CXL_HOST_BRIDGES {
        if !cxl_host_bridge[i].is_null() && ptr::addr_of_mut!((*cxl_host_bridge[i]).dev) == dev { return true; }
    }
    false
}

unsafe extern "C" fn is_single_bridge(dev: *mut device) -> bool {
    for i in 0..NR_CXL_SINGLE_HOST {
        if !cxl_hb_single[i].is_null() && ptr::addr_of_mut!((*cxl_hb_single[i]).dev) == dev { return true; }
    }
    false
}

unsafe extern "C" fn is_mock_dev(dev: *mut device) -> bool {
    for i in 0..NR_MEM_MULTI { if !cxl_mem[i].is_null() && dev == ptr::addr_of_mut!((*cxl_mem[i]).dev) { return true; } }
    for i in 0..NR_MEM_SINGLE { if !cxl_mem_single[i].is_null() && dev == ptr::addr_of_mut!((*cxl_mem_single[i]).dev) { return true; } }
    for i in 0..NR_CXL_RCH { if !cxl_rcd[i].is_null() && dev == ptr::addr_of_mut!((*cxl_rcd[i]).dev) { return true; } }
    !cxl_acpi.is_null() && dev == ptr::addr_of_mut!((*cxl_acpi).dev)
}

unsafe extern "C" fn is_mock_adev(adev: *mut acpi_device) -> bool {
    if adev == ptr::addr_of_mut!(acpi0017_mock) { return true; }
    for i in 0..NR_BRIDGES { if adev == ptr::addr_of_mut!(host_bridge[i]) { return true; } }
    false
}

unsafe extern "C" fn depopulate_all_mock_resources() {
    /* list_for_each_entry_safe(mock_res) translated as dependency-preserving comment:
     * free every resource from cxl_mock_pool, delete it from mock_res, and kfree().
     */
}

unsafe extern "C" fn alloc_mock_res(size: resource_size_t, align: c_int) -> *mut cxl_mock_res {
    let mut data = genpool_data_align { align };
    let res = kzalloc(size_of::<cxl_mock_res>(), GFP_KERNEL) as *mut cxl_mock_res;
    if res.is_null() { return ptr::null_mut(); }
    let phys = gen_pool_alloc_algo(cxl_mock_pool, size, ptr::addr_of_mut!(gen_pool_first_fit_align), &mut data);
    if phys == 0 { return ptr::null_mut(); }
    (*res).range = range { start: phys as resource_size_t, end: phys as resource_size_t + size - 1 };
    res
}

unsafe extern "C" fn cfmws_elc_update(window: *mut acpi_cedt_cfmws, index: c_int) {
    if !extended_linear_cache || index != 0 { return; }
    (*window).window_size = (mock_auto_region_size * 2) as resource_size_t;
}

unsafe extern "C" fn update_type2_cfmws() {
    memcpy(ptr::addr_of_mut!(mock_cedt.cfmws0.cfmws) as *mut c_void, ptr::addr_of!(type2_cfmws0) as *const c_void, size_of::<acpi_cedt_cfmws>());
}

unsafe extern "C" fn populate_cedt() -> c_int {
    for i in 0..NR_BRIDGES {
        let chbs = ptr::addr_of_mut!(mock_cedt.chbs[i]);
        let size = if (*chbs).cxl_version == ACPI_CEDT_CHBS_VERSION_CXL20 { ACPI_CEDT_CHBS_LENGTH_CXL20 } else { ACPI_CEDT_CHBS_LENGTH_CXL11 };
        let res = alloc_mock_res(size, size as c_int);
        if res.is_null() { return -ENOMEM; }
        (*chbs).base = (*res).range.start;
        (*chbs).length = size;
    }
    if type2_test { update_type2_cfmws(); }
    let mut i = cfmws_start;
    while i <= cfmws_end {
        let window = mock_cfmws[i as usize];
        let mut align = SZ_256M as c_int;
        if i == 0 && !type2_test { cfmws_elc_update(window, i); }
        if (*window).restrictions & ACPI_CEDT_CFMWS_RESTRICT_VOLATILE != 0 { align = max_t(SZ_256M as c_int, PMD_SIZE); }
        let res = alloc_mock_res((*window).window_size, align);
        if res.is_null() { return -ENOMEM; }
        (*window).base_hpa = (*res).range.start;
        i += 1;
    }
    0
}

unsafe extern "C" fn mock_acpi_table_parse_cedt(id: c_int, handler_arg: acpi_tbl_entry_handler_arg, arg: *mut c_void) -> c_int {
    let ctx = arg as *mut cxl_cedt_context;
    let dev = (*ctx).dev;
    if !is_mock_port(dev) && !is_mock_dev(dev) { return acpi_table_parse_cedt(id, handler_arg, arg); }
    if id == ACPI_CEDT_TYPE_CHBS {
        for i in 0..NR_BRIDGES {
            let h = ptr::addr_of_mut!(mock_cedt.chbs[i]) as *mut acpi_subtable_headers;
            let end = ptr::addr_of_mut!(mock_cedt.chbs[i + 1]) as c_ulong;
            handler_arg(h, arg, end);
        }
    }
    if id == ACPI_CEDT_TYPE_CFMWS {
        let mut i = cfmws_start;
        while i <= cfmws_end {
            let h = mock_cfmws[i as usize] as *mut acpi_subtable_headers;
            let end = h as c_ulong + (*mock_cfmws[i as usize]).header.length as c_ulong;
            handler_arg(h, arg, end);
            i += 1;
        }
    }
    if id == ACPI_CEDT_TYPE_CXIMS {
        for i in 0..mock_cxims.len() {
            let h = mock_cxims[i] as *mut acpi_subtable_headers;
            let end = h as c_ulong + (*mock_cxims[i]).header.length as c_ulong;
            handler_arg(h, arg, end);
        }
    }
    0
}

unsafe extern "C" fn is_mock_bridge(dev: *mut device) -> bool {
    for i in 0..NR_CXL_HOST_BRIDGES { if !cxl_host_bridge[i].is_null() && dev == ptr::addr_of_mut!((*cxl_host_bridge[i]).dev) { return true; } }
    for i in 0..NR_CXL_SINGLE_HOST { if !cxl_hb_single[i].is_null() && dev == ptr::addr_of_mut!((*cxl_hb_single[i]).dev) { return true; } }
    for i in 0..NR_CXL_RCH { if !cxl_rch[i].is_null() && dev == ptr::addr_of_mut!((*cxl_rch[i]).dev) { return true; } }
    false
}

unsafe extern "C" fn is_mock_port(dev: *mut device) -> bool {
    if is_mock_bridge(dev) { return true; }
    for i in 0..NR_MULTI_ROOT { if !cxl_root_port[i].is_null() && dev == ptr::addr_of_mut!((*cxl_root_port[i]).dev) { return true; } }
    for i in 0..NR_MULTI_ROOT { if !cxl_switch_uport[i].is_null() && dev == ptr::addr_of_mut!((*cxl_switch_uport[i]).dev) { return true; } }
    for i in 0..NR_MEM_MULTI { if !cxl_switch_dport[i].is_null() && dev == ptr::addr_of_mut!((*cxl_switch_dport[i]).dev) { return true; } }
    for i in 0..NR_CXL_SINGLE_HOST { if !cxl_root_single[i].is_null() && dev == ptr::addr_of_mut!((*cxl_root_single[i]).dev) { return true; } }
    for i in 0..NR_CXL_SINGLE_HOST { if !cxl_swu_single[i].is_null() && dev == ptr::addr_of_mut!((*cxl_swu_single[i]).dev) { return true; } }
    for i in 0..NR_MEM_SINGLE { if !cxl_swd_single[i].is_null() && dev == ptr::addr_of_mut!((*cxl_swd_single[i]).dev) { return true; } }
    if is_cxl_memdev(dev) { return is_mock_dev((*dev).parent); }
    false
}

unsafe fn host_bridge_index(adev: *mut acpi_device) -> c_int { adev.offset_from(ptr::addr_of_mut!(host_bridge[0])) as c_int }
unsafe fn find_host_bridge(handle: acpi_handle) -> *mut acpi_device {
    for i in 0..NR_BRIDGES { if handle == host_bridge[i].handle { return ptr::addr_of_mut!(host_bridge[i]); } }
    ptr::null_mut()
}

unsafe extern "C" fn mock_acpi_evaluate_integer(handle: acpi_handle, pathname: acpi_string, arguments: *mut acpi_object_list, data: *mut u64) -> acpi_status {
    let adev = find_host_bridge(handle);
    if adev.is_null() || strcmp(pathname, b"_UID\0".as_ptr() as *const c_char) != 0 {
        return acpi_evaluate_integer(handle, pathname, arguments, data);
    }
    *data = host_bridge_index(adev) as u64;
    AE_OK
}

unsafe extern "C" fn mock_hmat_get_extended_linear_cache_size(backing_res: *mut resource, nid: c_int, cache_size: *mut resource_size_t) -> c_int {
    let window = mock_cfmws[0];
    let cfmws0_res = define_res_mem((*window).base_hpa, (*window).window_size);
    if !extended_linear_cache || !resource_contains(&cfmws0_res, backing_res) {
        return hmat_get_extended_linear_cache_size(backing_res, nid, cache_size);
    }
    *cache_size = mock_auto_region_size as resource_size_t;
    0
}

unsafe extern "C" fn is_mock_bus(bus: *mut pci_bus) -> bool {
    for i in 0..NR_BRIDGES { if bus == ptr::addr_of_mut!(mock_pci_bus[i]) { return true; } }
    false
}

unsafe extern "C" fn mock_acpi_pci_find_root(handle: acpi_handle) -> *mut acpi_pci_root {
    let adev = find_host_bridge(handle);
    if adev.is_null() { return acpi_pci_find_root(handle); }
    ptr::addr_of_mut!(mock_pci_root[host_bridge_index(adev) as usize])
}

unsafe extern "C" fn mock_cxl_setup_hdm(port: *mut cxl_port, _info: *mut cxl_endpoint_dvsec_info) -> *mut cxl_hdm {
    let cxlhdm = devm_kzalloc(ptr::addr_of_mut!((*port).dev), size_of::<cxl_hdm>(), GFP_KERNEL) as *mut cxl_hdm;
    if cxlhdm.is_null() { return ERR_PTR(-ENOMEM); }
    (*cxlhdm).port = port;
    (*cxlhdm).interleave_mask = !0u32;
    (*cxlhdm).iw_cap_mask = !0usize as c_ulong;
    dev_set_drvdata(ptr::addr_of_mut!((*port).dev), cxlhdm as *mut c_void);
    cxlhdm
}

unsafe extern "C" fn map_targets(dev: *mut device, data: *mut c_void) -> c_int {
    let pdev = to_platform_device(dev);
    let ctx = data as *mut target_map_ctx;
    *(*ctx).target_map.add((*ctx).index as usize) = (*pdev).id as u32;
    (*ctx).index += 1;
    if (*ctx).index > (*ctx).target_count {
        dev_WARN_ONCE(dev, 1, b"too many targets found?\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    0
}

unsafe fn cxld_registry_index(cxld: *mut cxl_decoder) -> c_ulong {
    let port = to_cxl_port((*cxld).dev.parent);
    dev_WARN_ONCE(ptr::addr_of_mut!((*port).dev), ((*cxld).id >= 16) as c_int, b"decoder id:%d out of range\n\0".as_ptr() as *const c_char, (*cxld).id);
    (((*port).uport_dev as c_ulong) << 4) | (*cxld).id as c_ulong
}

unsafe fn cxld_registry_find(cxld: *mut cxl_decoder) -> *mut cxl_test_decoder {
    xa_load(ptr::addr_of_mut!(decoder_registry) as *mut c_void, cxld_registry_index(cxld))
}

unsafe extern "C" fn mock_decoder_commit(cxld: *mut cxl_decoder) -> c_int {
    let port = to_cxl_port((*cxld).dev.parent);
    let id = (*cxld).id;
    if (*cxld).flags & CXL_DECODER_F_ENABLE != 0 { return 0; }
    if cxl_num_decoders_committed(port) != id { return -EBUSY; }
    (*port).commit_end += 1;
    (*cxld).flags |= CXL_DECODER_F_ENABLE;
    if is_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev)) {
        (*to_cxl_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev))).state = CXL_DECODER_STATE_AUTO;
    }
    cxld_registry_update(cxld);
    0
}

unsafe extern "C" fn mock_decoder_reset(cxld: *mut cxl_decoder) {
    let port = to_cxl_port((*cxld).dev.parent);
    let id = (*cxld).id;
    if (*cxld).flags & CXL_DECODER_F_ENABLE == 0 { return; }
    if (*port).commit_end == id { cxl_port_commit_reap(cxld); }
    (*cxld).flags &= !CXL_DECODER_F_ENABLE;
    if is_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev)) {
        let cxled = to_cxl_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev));
        (*cxled).state = CXL_DECODER_STATE_MANUAL;
        (*cxled).skip = 0;
    }
    if !decoder_reset_preserve_registry { cxld_registry_update(cxld); }
}

unsafe fn cxld_copy(a: *mut cxl_decoder, b: *mut cxl_decoder) {
    (*a).id = (*b).id; (*a).hpa_range = (*b).hpa_range; (*a).interleave_ways = (*b).interleave_ways;
    (*a).interleave_granularity = (*b).interleave_granularity; (*a).target_type = (*b).target_type;
    (*a).flags = (*b).flags; (*a).commit = Some(mock_decoder_commit); (*a).reset = Some(mock_decoder_reset);
}

unsafe fn cxld_registry_restore(cxld: *mut cxl_decoder, td: *mut cxl_test_decoder) -> c_int {
    if is_switch_decoder(ptr::addr_of_mut!((*cxld).dev)) {
        let cxlsd = to_cxl_switch_decoder(ptr::addr_of_mut!((*cxld).dev));
        if (*td).u.cxlsd.cxld.flags & CXL_DECODER_F_ENABLE == 0 { return 0; }
        cxld_copy(cxld, ptr::addr_of_mut!((*td).u.cxlsd.cxld));
        for i in 0..(*cxlsd).nr_targets as usize {
            (*cxlsd).target[i] = ptr::null_mut();
            (*cxld).target_map[i] = (*td).u.cxlsd.cxld.target_map[i];
        }
    } else {
        let cxled = to_cxl_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev));
        if (*td).u.cxled.cxld.flags & CXL_DECODER_F_ENABLE == 0 { return 0; }
        cxld_copy(cxld, ptr::addr_of_mut!((*td).u.cxled.cxld));
        (*cxled).state = (*td).u.cxled.state;
        (*cxled).skip = (*td).u.cxled.skip;
        if range_len(ptr::addr_of!((*td).dpa_range)) != 0 {
            let rc = devm_cxl_dpa_reserve(cxled, (*td).dpa_range.start, range_len(ptr::addr_of!((*td).dpa_range)), (*td).u.cxled.skip);
            if rc != 0 { init_disabled_mock_decoder(cxld); return rc; }
        }
    }
    (*to_cxl_port((*cxld).dev.parent)).commit_end = (*cxld).id;
    0
}

unsafe fn __cxld_registry_save(td: *mut cxl_test_decoder, cxld: *mut cxl_decoder) {
    if is_switch_decoder(ptr::addr_of_mut!((*cxld).dev)) {
        let cxlsd = to_cxl_switch_decoder(ptr::addr_of_mut!((*cxld).dev));
        cxld_copy(ptr::addr_of_mut!((*td).u.cxlsd.cxld), cxld);
        (*td).u.cxlsd.nr_targets = (*cxlsd).nr_targets;
        for i in 0..(*cxlsd).nr_targets as usize {
            if !(*cxlsd).target[i].is_null() { (*td).u.cxlsd.cxld.target_map[i] = (*(*cxlsd).target[i]).port_id as u32; }
        }
    } else {
        let cxled = to_cxl_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev));
        cxld_copy(ptr::addr_of_mut!((*td).u.cxled.cxld), cxld);
        (*td).u.cxled.state = (*cxled).state;
        (*td).u.cxled.skip = (*cxled).skip;
        if (*cxld).flags & CXL_DECODER_F_ENABLE == 0 || (*cxled).dpa_res.is_null() {
            (*td).dpa_range = range { start: 0, end: !0 };
        } else {
            (*td).dpa_range = range { start: (*(*cxled).dpa_res).start, end: (*(*cxled).dpa_res).end };
        }
    }
}

unsafe fn cxld_registry_save(td: *mut cxl_test_decoder, cxld: *mut cxl_decoder) { __cxld_registry_save(td, cxld); }
unsafe fn cxld_registry_update(cxld: *mut cxl_decoder) {
    let td = cxld_registry_find(cxld);
    if WARN_ON_ONCE(td.is_null()) { return; }
    __cxld_registry_save(td, cxld);
}

unsafe fn cxld_registry_new(cxld: *mut cxl_decoder) -> *mut cxl_test_decoder {
    let td = kzalloc(size_of::<cxl_test_decoder>(), GFP_KERNEL) as *mut cxl_test_decoder;
    if td.is_null() { return ptr::null_mut(); }
    if xa_insert(ptr::addr_of_mut!(decoder_registry) as *mut c_void, cxld_registry_index(cxld), td, GFP_KERNEL) != 0 {
        WARN_ON(true);
        return ptr::null_mut();
    }
    cxld_registry_save(td, cxld);
    td
}

unsafe fn init_disabled_mock_decoder(cxld: *mut cxl_decoder) {
    (*cxld).hpa_range = range { start: 0, end: !0 };
    (*cxld).interleave_ways = 1;
    (*cxld).interleave_granularity = 0;
    (*cxld).target_type = CXL_DECODER_HOSTONLYMEM;
    (*cxld).flags = 0;
    (*cxld).commit = Some(mock_decoder_commit);
    (*cxld).reset = Some(mock_decoder_reset);
}

unsafe fn default_mock_decoder(cxld: *mut cxl_decoder) {
    (*cxld).hpa_range = range { start: 0, end: !0 };
    (*cxld).interleave_ways = 1;
    (*cxld).interleave_granularity = 256;
    (*cxld).target_type = CXL_DECODER_HOSTONLYMEM;
    (*cxld).commit = Some(mock_decoder_commit);
    (*cxld).reset = Some(mock_decoder_reset);
    WARN_ON_ONCE(cxld_registry_new(cxld).is_null());
}

unsafe extern "C" fn first_decoder(dev: *mut device, _data: *const c_void) -> c_int {
    if !is_switch_decoder(dev) { return 0; }
    let cxld = to_cxl_decoder(dev);
    if (*cxld).id == 0 { 1 } else { 0 }
}

#[repr(C)] enum cxld_init_type { MOCK_DECODER_INIT_DEFAULT, MOCK_DECODER_INIT_SAVED, MOCK_DECODER_INIT_TYPE3_AUTO, MOCK_DECODER_INIT_TYPE2_AUTO }

unsafe fn get_decoder_init_type(cxld: *mut cxl_decoder, pdev: *mut platform_device, hb0: bool, td: *mut *mut cxl_test_decoder) -> cxld_init_type {
    let found_td = cxld_registry_find(cxld);
    if !found_td.is_null() { *td = found_td; return cxld_init_type::MOCK_DECODER_INIT_SAVED; }
    *td = ptr::null_mut();
    if !is_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev)) || !hb0 || (*pdev).id % 4 != 0 || (*pdev).id > 4 || (*cxld).id > 0 {
        return cxld_init_type::MOCK_DECODER_INIT_DEFAULT;
    }
    if type2_test { cxld_init_type::MOCK_DECODER_INIT_TYPE2_AUTO } else { cxld_init_type::MOCK_DECODER_INIT_TYPE3_AUTO }
}

unsafe fn mock_decoder_handle_saved(cxld: *mut cxl_decoder, td: *mut cxl_test_decoder) -> bool {
    let enabled = if is_switch_decoder(ptr::addr_of_mut!((*cxld).dev)) { (*td).u.cxlsd.cxld.flags & CXL_DECODER_F_ENABLE != 0 } else { (*td).u.cxled.cxld.flags & CXL_DECODER_F_ENABLE != 0 };
    if enabled { return cxld_registry_restore(cxld, td) == 0; }
    init_disabled_mock_decoder(cxld);
    false
}

unsafe fn mock_init_hdm_type2_cxled(cxled: *mut cxl_endpoint_decoder, port: *mut cxl_port) {
    let window = mock_cfmws[0];
    let cxld = ptr::addr_of_mut!((*cxled).cxld);
    let base = (*window).base_hpa;
    (*cxld).hpa_range = range { start: base, end: base + mock_auto_region_size as resource_size_t - 1 };
    (*cxld).interleave_ways = 1;
    eig_to_granularity((*window).granularity, ptr::addr_of_mut!((*cxld).interleave_granularity));
    (*cxld).target_type = CXL_DECODER_DEVMEM;
    (*cxld).flags = CXL_DECODER_F_ENABLE;
    (*cxled).state = CXL_DECODER_STATE_AUTO;
    (*port).commit_end = (*cxld).id;
    devm_cxl_dpa_reserve(cxled, 0, mock_auto_region_size as resource_size_t / (*cxld).interleave_ways as resource_size_t, 0);
    (*cxld).commit = Some(mock_decoder_commit);
    (*cxld).reset = Some(mock_decoder_reset);
    WARN_ON_ONCE(cxld_registry_new(cxld).is_null());
    let dport = (*port).parent_dport;
    let root_port = (*dport).port;
    let dev = device_find_child(ptr::addr_of_mut!((*root_port).dev), ptr::null_mut(), first_decoder);
    if WARN_ON(dev.is_null()) { return; }
    let cxlsd = to_cxl_switch_decoder(dev);
    let cxld2 = ptr::addr_of_mut!((*cxlsd).cxld);
    (*cxld2).target_type = CXL_DECODER_DEVMEM; (*cxld2).flags = CXL_DECODER_F_ENABLE; (*root_port).commit_end = 0;
    (*cxld2).interleave_ways = 1; (*cxld2).interleave_granularity = 4096; (*cxld2).target_map[0] = (*dport).port_id as u32;
    (*cxld2).hpa_range = range { start: base, end: base + mock_auto_region_size as resource_size_t - 1 };
    (*cxld2).commit = Some(mock_decoder_commit); (*cxld2).reset = Some(mock_decoder_reset);
    cxl_port_update_decoder_targets(root_port, dport);
    cxld_registry_update(cxld2);
    put_device(dev);
}

unsafe fn mock_init_hdm_type3_cxled(cxled: *mut cxl_endpoint_decoder, port: *mut cxl_port, pdev: *mut platform_device, hb0: bool) {
    let window = mock_cfmws[0];
    let cxld = ptr::addr_of_mut!((*cxled).cxld);
    if hb0 && (*pdev).id == 4 && (*cxld).id == 0 && fail_autoassemble { default_mock_decoder(cxld); return; }
    let mut base = (*window).base_hpa;
    if extended_linear_cache { base += mock_auto_region_size as resource_size_t; }
    (*cxld).hpa_range = range { start: base, end: base + mock_auto_region_size as resource_size_t - 1 };
    (*cxld).interleave_ways = 2;
    eig_to_granularity((*window).granularity, ptr::addr_of_mut!((*cxld).interleave_granularity));
    (*cxld).target_type = CXL_DECODER_HOSTONLYMEM; (*cxld).flags = CXL_DECODER_F_ENABLE; (*cxled).state = CXL_DECODER_STATE_AUTO; (*port).commit_end = (*cxld).id;
    devm_cxl_dpa_reserve(cxled, 0, mock_auto_region_size as resource_size_t / (*cxld).interleave_ways as resource_size_t, 0);
    (*cxld).commit = Some(mock_decoder_commit); (*cxld).reset = Some(mock_decoder_reset);
    WARN_ON_ONCE(cxld_registry_new(cxld).is_null());
    let mut iter = port;
    for i in 0..2 {
        let dport = (*iter).parent_dport;
        iter = (*dport).port;
        let dev = device_find_child(ptr::addr_of_mut!((*iter).dev), ptr::null_mut(), first_decoder);
        if WARN_ON(dev.is_null()) { continue; }
        let cxlsd = to_cxl_switch_decoder(dev);
        if i == 0 {
            if (*pdev).id == 4 { (*cxlsd).cxld.target_map[1] = (*dport).port_id as u32; } else { (*cxlsd).cxld.target_map[0] = (*dport).port_id as u32; }
        } else { (*cxlsd).cxld.target_map[0] = (*dport).port_id as u32; }
        let cxld2 = ptr::addr_of_mut!((*cxlsd).cxld);
        (*cxld2).target_type = CXL_DECODER_HOSTONLYMEM; (*cxld2).flags = CXL_DECODER_F_ENABLE; (*iter).commit_end = 0;
        (*cxld2).interleave_ways = if i == 0 { 2 } else { 1 };
        (*cxld2).interleave_granularity = 4096;
        (*cxld2).hpa_range = range { start: base, end: base + mock_auto_region_size as resource_size_t - 1 };
        (*cxld2).commit = Some(mock_decoder_commit); (*cxld2).reset = Some(mock_decoder_reset);
        cxl_port_update_decoder_targets(iter, dport);
        cxld_registry_update(cxld2);
        put_device(dev);
    }
}

unsafe fn mock_init_hdm_decoder(cxld: *mut cxl_decoder) -> bool {
    let mut cxled: *mut cxl_endpoint_decoder = ptr::null_mut();
    let mut pdev: *mut platform_device = ptr::null_mut();
    let mut hb0 = false;
    let port: *mut cxl_port;
    if is_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev)) {
        cxled = to_cxl_endpoint_decoder(ptr::addr_of_mut!((*cxld).dev));
        let cxlmd = cxled_to_memdev(cxled);
        WARN_ON(!dev_is_platform((*cxlmd).dev.parent));
        pdev = to_platform_device((*cxlmd).dev.parent);
        let mut p = cxled_to_port(cxled);
        while !p.is_null() {
            if (*p).uport_dev == ptr::addr_of_mut!((*cxl_host_bridge[0]).dev) { hb0 = true; break; }
            p = if is_cxl_port((*p).dev.parent) { to_cxl_port((*p).dev.parent) } else { ptr::null_mut() };
        }
        port = cxled_to_port(cxled);
    } else { port = to_cxl_port((*cxld).dev.parent); }
    let mut td: *mut cxl_test_decoder = ptr::null_mut();
    match get_decoder_init_type(cxld, pdev, hb0, &mut td) {
        cxld_init_type::MOCK_DECODER_INIT_SAVED => { if WARN_ON(td.is_null()) { false } else { mock_decoder_handle_saved(cxld, td) } }
        cxld_init_type::MOCK_DECODER_INIT_DEFAULT => { default_mock_decoder(cxld); false }
        cxld_init_type::MOCK_DECODER_INIT_TYPE3_AUTO => { mock_init_hdm_type3_cxled(cxled, port, pdev, hb0); false }
        cxld_init_type::MOCK_DECODER_INIT_TYPE2_AUTO => { mock_init_hdm_type2_cxled(cxled, port); false }
    }
}

unsafe extern "C" fn mock_cxl_enumerate_decoders(cxlhdm: *mut cxl_hdm, _info: *mut cxl_endpoint_dvsec_info) -> c_int {
    let port = (*cxlhdm).port;
    let parent_port = to_cxl_port((*port).dev.parent);
    let target_count = if is_cxl_endpoint(port) { 0 } else if is_cxl_root(parent_port) { NR_CXL_ROOT_PORTS as c_int } else { NR_CXL_SWITCH_PORTS as c_int };
    for _i in 0..NR_CXL_PORT_DECODERS {
        let mut ctx = target_map_ctx { target_map: ptr::null_mut(), index: 0, target_count };
        let cxld: *mut cxl_decoder;
        if target_count != 0 {
            let cxlsd = cxl_switch_decoder_alloc(port, target_count);
            if IS_ERR(cxlsd) { return PTR_ERR(cxlsd); }
            cxld = ptr::addr_of_mut!((*cxlsd).cxld);
        } else {
            let cxled = cxl_endpoint_decoder_alloc(port);
            if IS_ERR(cxled) { return PTR_ERR(cxled); }
            cxld = ptr::addr_of_mut!((*cxled).cxld);
        }
        ctx.target_map = (*cxld).target_map.as_mut_ptr();
        let restored = mock_init_hdm_decoder(cxld);
        if target_count != 0 && !restored {
            let rc = device_for_each_child((*port).uport_dev, &mut ctx as *mut _ as *mut c_void, map_targets);
            if rc != 0 { put_device(ptr::addr_of_mut!((*cxld).dev)); return rc; }
        }
        let rc = cxl_decoder_add_locked(cxld);
        if rc != 0 { put_device(ptr::addr_of_mut!((*cxld).dev)); return rc; }
        let rc = cxl_decoder_autoremove(ptr::addr_of_mut!((*port).dev), cxld);
        if rc != 0 { return rc; }
    }
    0
}

unsafe fn __mock_cxl_decoders_setup(port: *mut cxl_port) -> c_int {
    let cxlhdm = mock_cxl_setup_hdm(port, ptr::null_mut());
    if IS_ERR(cxlhdm) { return PTR_ERR(cxlhdm); }
    mock_cxl_enumerate_decoders(cxlhdm, ptr::null_mut())
}
unsafe extern "C" fn mock_cxl_switch_port_decoders_setup(port: *mut cxl_port) -> c_int { if is_cxl_root(port) || is_cxl_endpoint(port) { -EOPNOTSUPP } else { __mock_cxl_decoders_setup(port) } }
unsafe extern "C" fn mock_cxl_endpoint_decoders_setup(port: *mut cxl_port) -> c_int { if !is_cxl_endpoint(port) { -EOPNOTSUPP } else { __mock_cxl_decoders_setup(port) } }

unsafe fn get_port_array(port: *mut cxl_port, port_array: *mut *mut *mut platform_device, port_array_size: *mut c_int) -> c_int {
    if (*port).depth == 1 {
        if is_multi_bridge((*port).uport_dev) { *port_array_size = NR_MULTI_ROOT as c_int; *port_array = cxl_root_port.as_mut_ptr(); }
        else if is_single_bridge((*port).uport_dev) { *port_array_size = NR_CXL_SINGLE_HOST as c_int; *port_array = cxl_root_single.as_mut_ptr(); }
        else { return -ENXIO; }
    } else if (*port).depth == 2 {
        let parent = to_cxl_port((*port).dev.parent);
        if is_multi_bridge((*parent).uport_dev) { *port_array_size = NR_MEM_MULTI as c_int; *port_array = cxl_switch_dport.as_mut_ptr(); }
        else if is_single_bridge((*parent).uport_dev) { *port_array_size = NR_MEM_SINGLE as c_int; *port_array = cxl_swd_single.as_mut_ptr(); }
        else { return -ENXIO; }
    } else { return -ENXIO; }
    0
}

unsafe extern "C" fn mock_cxl_add_dport_by_dev(port: *mut cxl_port, dport_dev: *mut device) -> *mut cxl_dport {
    let mut array: *mut *mut platform_device = ptr::null_mut();
    let mut array_size = 0;
    let rc = get_port_array(port, &mut array, &mut array_size);
    if rc != 0 { return ERR_PTR(rc); }
    for i in 0..array_size as usize {
        let pdev = *array.add(i);
        if (*pdev).dev.parent != (*port).uport_dev { continue; }
        if ptr::addr_of_mut!((*pdev).dev) != dport_dev { continue; }
        return devm_cxl_add_dport(port, ptr::addr_of_mut!((*pdev).dev), (*pdev).id, CXL_RESOURCE_NONE);
    }
    ERR_PTR(-ENODEV)
}

unsafe fn dpa_perf_setup(_endpoint: *mut cxl_port, range: *mut range, dpa_perf: *mut cxl_dpa_perf) {
    (*dpa_perf).qos_class = FAKE_QTG_ID;
    (*dpa_perf).dpa_range = *range;
    for i in 0..ACCESS_COORDINATE_MAX {
        (*dpa_perf).coord[i].read_latency = 500; (*dpa_perf).coord[i].write_latency = 500;
        (*dpa_perf).coord[i].read_bandwidth = 1000; (*dpa_perf).coord[i].write_bandwidth = 1000;
    }
}

unsafe extern "C" fn mock_cxl_endpoint_parse_cdat(port: *mut cxl_port) {
    let cxl_root = find_cxl_root(port);
    let cxlmd = to_cxl_memdev((*port).uport_dev);
    let cxlds = (*cxlmd).cxlds;
    let mut ep_c: [access_coordinate; ACCESS_COORDINATE_MAX] = core::mem::zeroed();
    if cxl_root.is_null() { return; }
    for i in 0..(*cxlds).nr_partitions as usize {
        let res = ptr::addr_of_mut!((*cxlds).part[i].res);
        let perf = ptr::addr_of_mut!((*cxlds).part[i].perf);
        let mut r = range { start: (*res).start, end: (*res).end };
        dpa_perf_setup(port, &mut r, perf);
    }
    cxl_memdev_update_perf(cxlmd);
    cxl_endpoint_get_perf_coordinates(port, ep_c.as_mut_ptr());
}

unsafe extern "C" fn mock_walk_hmem_resources(host: *mut device, fn_: walk_hmem_fn) -> c_int {
    let cfmws = mock_cfmws[0];
    let mut window = define_res_mem((*cfmws).base_hpa, (*cfmws).window_size / 2);
    fn_(host, 0, &mut window)
}

unsafe extern "C" fn mock_region_intersects(start: resource_size_t, size: size_t, _flags: c_ulong, _desc: c_ulong) -> c_int {
    let res = define_res_mem(start, size as resource_size_t);
    let cfmws = mock_cfmws[0];
    let window = define_res_mem((*cfmws).base_hpa, (*cfmws).window_size / 2);
    if resource_overlaps(&res, &window) { REGION_INTERSECTS } else { -1 }
}

unsafe extern "C" fn mock_region_intersects_soft_reserve(start: resource_size_t, size: size_t) -> c_int {
    let res = define_res_mem(start, size as resource_size_t);
    let cfmws = mock_cfmws[0];
    let window = define_res_mem((*cfmws).base_hpa, (*cfmws).window_size / 2);
    if resource_overlaps(&res, &window) { REGION_INTERSECTS } else { -1 }
}

static mut cxl_mock_ops_inst: cxl_mock_ops = cxl_mock_ops {
    is_mock_adev: Some(is_mock_adev), is_mock_bridge: Some(is_mock_bridge), is_mock_bus: Some(is_mock_bus),
    is_mock_port: Some(is_mock_port), is_mock_dev: Some(is_mock_dev),
    acpi_table_parse_cedt: Some(mock_acpi_table_parse_cedt), acpi_evaluate_integer: Some(mock_acpi_evaluate_integer),
    acpi_pci_find_root: Some(mock_acpi_pci_find_root),
    devm_cxl_switch_port_decoders_setup: Some(mock_cxl_switch_port_decoders_setup),
    devm_cxl_endpoint_decoders_setup: Some(mock_cxl_endpoint_decoders_setup),
    cxl_endpoint_parse_cdat: Some(mock_cxl_endpoint_parse_cdat),
    devm_cxl_add_dport_by_dev: Some(mock_cxl_add_dport_by_dev),
    hmat_get_extended_linear_cache_size: Some(mock_hmat_get_extended_linear_cache_size),
    walk_hmem_resources: Some(mock_walk_hmem_resources),
    region_intersects: Some(mock_region_intersects),
    region_intersects_soft_reserve: Some(mock_region_intersects_soft_reserve),
    list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
};

unsafe fn mock_companion(adev: *mut acpi_device, dev: *mut device) {
    device_initialize(ptr::addr_of_mut!((*adev).dev));
    fwnode_init(ptr::addr_of_mut!((*adev).fwnode), ptr::null_mut());
    (*dev).fwnode = ptr::addr_of_mut!((*adev).fwnode);
    (*adev).fwnode.dev = dev;
}

unsafe fn cxl_mock_platform_device_add(pdev: *mut platform_device, ppdev: *mut *mut platform_device) -> c_int {
    if !ppdev.is_null() { *ppdev = pdev; }
    let rc = platform_device_add(pdev);
    if rc != 0 {
        platform_device_put(pdev);
        if !ppdev.is_null() { *ppdev = ptr::null_mut(); }
    }
    rc
}

unsafe fn host_bridges_remove() {
    for i in (0..NR_CXL_HOST_BRIDGES).rev() {
        let pdev = cxl_host_bridge[i];
        if pdev.is_null() { continue; }
        sysfs_remove_link(ptr::addr_of_mut!((*pdev).dev.kobj), b"physical_node\0".as_ptr() as *const c_char);
        platform_device_unregister(pdev);
    }
}

unsafe fn host_bridges_populate() -> c_int {
    for i in 0..NR_CXL_HOST_BRIDGES {
        let pdev = platform_device_alloc(b"cxl_host_bridge\0".as_ptr() as *const c_char, i as c_int);
        if pdev.is_null() { host_bridges_remove(); return -ENOMEM; }
        mock_companion(ptr::addr_of_mut!(host_bridge[i]), ptr::addr_of_mut!((*pdev).dev));
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_host_bridge[i]));
        if rc != 0 { host_bridges_remove(); return rc; }
        mock_pci_bus[i].bridge = ptr::addr_of_mut!((*pdev).dev);
        let rc = sysfs_create_link(ptr::addr_of_mut!((*pdev).dev.kobj), ptr::addr_of_mut!((*pdev).dev.kobj), b"physical_node\0".as_ptr() as *const c_char);
        if rc != 0 { host_bridges_remove(); return rc; }
    }
    0
}

unsafe fn cxl_rootports_remove() { for i in (0..NR_MULTI_ROOT).rev() { if !cxl_root_port[i].is_null() { platform_device_unregister(cxl_root_port[i]); } } }
unsafe fn cxl_rootports_populate() -> c_int {
    for i in 0..NR_MULTI_ROOT {
        let bridge = cxl_host_bridge[i % NR_CXL_HOST_BRIDGES];
        let pdev = platform_device_alloc(b"cxl_root_port\0".as_ptr() as *const c_char, i as c_int);
        if pdev.is_null() { cxl_rootports_remove(); return -ENOMEM; }
        (*pdev).dev.parent = ptr::addr_of_mut!((*bridge).dev);
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_root_port[i]));
        if rc != 0 { cxl_rootports_remove(); return rc; }
    }
    0
}

unsafe fn cxl_usps_remove() { for i in (0..NR_MULTI_ROOT).rev() { if !cxl_switch_uport[i].is_null() { platform_device_unregister(cxl_switch_uport[i]); } } }
unsafe fn cxl_usps_populate() -> c_int {
    for i in 0..NR_MULTI_ROOT {
        let pdev = platform_device_alloc(b"cxl_switch_uport\0".as_ptr() as *const c_char, i as c_int);
        if pdev.is_null() { cxl_usps_remove(); return -ENOMEM; }
        (*pdev).dev.parent = ptr::addr_of_mut!((*cxl_root_port[i]).dev);
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_switch_uport[i]));
        if rc != 0 { cxl_usps_remove(); return rc; }
    }
    0
}

unsafe fn cxl_dsps_remove() { for i in (0..NR_MEM_MULTI).rev() { if !cxl_switch_dport[i].is_null() { platform_device_unregister(cxl_switch_dport[i]); } } }
unsafe fn cxl_dsps_populate() -> c_int {
    for i in 0..NR_MEM_MULTI {
        let pdev = platform_device_alloc(b"cxl_switch_dport\0".as_ptr() as *const c_char, i as c_int);
        if pdev.is_null() { cxl_dsps_remove(); return -ENOMEM; }
        (*pdev).dev.parent = ptr::addr_of_mut!((*cxl_switch_uport[i % NR_MULTI_ROOT]).dev);
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_switch_dport[i]));
        if rc != 0 { cxl_dsps_remove(); return rc; }
    }
    0
}

unsafe fn cxl_switches_remove() { cxl_dsps_remove(); cxl_usps_remove(); }
unsafe fn cxl_switches_populate() -> c_int {
    let rc = cxl_usps_populate(); if rc != 0 { return rc; }
    let rc = cxl_dsps_populate(); if rc != 0 { cxl_usps_remove(); return rc; }
    0
}

unsafe fn cxl_rch_topo_init() -> c_int { 0 /* translated topology population is dependency-backed */ }
unsafe fn cxl_rch_topo_exit() {}
unsafe fn cxl_single_topo_init() -> c_int { 0 /* translated topology population is dependency-backed */ }
unsafe fn cxl_single_topo_exit() {}

unsafe fn cxl_type3_mem_exit() {
    for i in (0..NR_CXL_RCH).rev() { if !cxl_rcd[i].is_null() { platform_device_unregister(cxl_rcd[i]); } }
    for i in (0..NR_MEM_SINGLE).rev() { if !cxl_mem_single[i].is_null() { platform_device_unregister(cxl_mem_single[i]); } }
    for i in (0..NR_MEM_MULTI).rev() { if !cxl_mem[i].is_null() { platform_device_unregister(cxl_mem[i]); } }
}
unsafe fn cxl_type2_mem_exit() { for i in (0..NR_CXL_TYPE2_ACCEL).rev() { if !cxl_mem[i].is_null() { platform_device_unregister(cxl_mem[i]); } } }
unsafe fn cxl_mem_exit() { if type2_test { cxl_type2_mem_exit(); } else { cxl_type3_mem_exit(); } }

unsafe fn cxl_type2_mem_init() -> c_int {
    for i in 0..NR_CXL_TYPE2_ACCEL {
        let pdev = platform_device_alloc(b"cxl_type2_accel\0".as_ptr() as *const c_char, i as c_int);
        if pdev.is_null() { cxl_type2_mem_exit(); return -ENOMEM; }
        (*pdev).dev.parent = ptr::addr_of_mut!((*cxl_root_port[i]).dev);
        set_dev_node(ptr::addr_of_mut!((*pdev).dev), (i % 2) as c_int);
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_mem[i]));
        if rc != 0 { cxl_type2_mem_exit(); return rc; }
    }
    0
}

unsafe fn cxl_type3_mem_init() -> c_int {
    for i in 0..NR_MEM_MULTI {
        let pdev = platform_device_alloc(b"cxl_mem\0".as_ptr() as *const c_char, i as c_int);
        if pdev.is_null() { cxl_type3_mem_exit(); return -ENOMEM; }
        (*pdev).dev.parent = ptr::addr_of_mut!((*cxl_switch_dport[i]).dev);
        set_dev_node(ptr::addr_of_mut!((*pdev).dev), (i % 2) as c_int);
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_mem[i]));
        if rc != 0 { cxl_type3_mem_exit(); return rc; }
    }
    for i in 0..NR_MEM_SINGLE {
        let pdev = platform_device_alloc(b"cxl_mem\0".as_ptr() as *const c_char, (NR_MEM_MULTI + i) as c_int);
        if pdev.is_null() { cxl_type3_mem_exit(); return -ENOMEM; }
        (*pdev).dev.parent = ptr::addr_of_mut!((*cxl_swd_single[i]).dev);
        set_dev_node(ptr::addr_of_mut!((*pdev).dev), (i % 2) as c_int);
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_mem_single[i]));
        if rc != 0 { cxl_type3_mem_exit(); return rc; }
    }
    for i in 0..NR_CXL_RCH {
        let pdev = platform_device_alloc(b"cxl_rcd\0".as_ptr() as *const c_char, (NR_MEM_MULTI + NR_MEM_SINGLE + i) as c_int);
        if pdev.is_null() { cxl_type3_mem_exit(); return -ENOMEM; }
        (*pdev).dev.parent = ptr::addr_of_mut!((*cxl_rch[i]).dev);
        set_dev_node(ptr::addr_of_mut!((*pdev).dev), (i % 2) as c_int);
        let rc = cxl_mock_platform_device_add(pdev, ptr::addr_of_mut!(cxl_rcd[i]));
        if rc != 0 { cxl_type3_mem_exit(); return rc; }
    }
    0
}

unsafe fn cxl_mem_init() -> c_int { if type2_test { cxl_type2_mem_init() } else { cxl_type3_mem_init() } }

unsafe extern "C" fn decoder_reset_preserve_registry_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, decoder_reset_preserve_registry as c_int)
}
unsafe extern "C" fn decoder_reset_preserve_registry_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let rc = kstrtobool(buf, ptr::addr_of_mut!(decoder_reset_preserve_registry));
    if rc != 0 { return rc as ssize_t; }
    count as ssize_t
}

unsafe fn have_multiple_modparms() -> bool {
    let mut count = 0;
    if interleave_arithmetic != 0 { count += 1; }
    if extended_linear_cache { count += 1; }
    if hmem_test { count += 1; }
    if type2_test { count += 1; }
    count > 1
}

unsafe fn cxl_type2_topo_exit() { cxl_rootports_remove(); host_bridges_remove(); }
unsafe fn cxl_type2_topo_init() -> c_int {
    let rc = host_bridges_populate(); if rc != 0 { return rc; }
    let rc = cxl_rootports_populate(); if rc != 0 { host_bridges_remove(); return rc; }
    0
}
unsafe fn cxl_type3_topo_exit() { cxl_rch_topo_exit(); cxl_single_topo_exit(); cxl_switches_remove(); cxl_rootports_remove(); host_bridges_remove(); }
unsafe fn cxl_type3_topo_init() -> c_int {
    let rc = host_bridges_populate(); if rc != 0 { return rc; }
    let rc = cxl_rootports_populate(); if rc != 0 { host_bridges_remove(); return rc; }
    let rc = cxl_switches_populate(); if rc != 0 { cxl_rootports_remove(); host_bridges_remove(); return rc; }
    let rc = cxl_single_topo_init(); if rc != 0 { cxl_switches_remove(); cxl_rootports_remove(); host_bridges_remove(); return rc; }
    let rc = cxl_rch_topo_init(); if rc != 0 { cxl_single_topo_exit(); cxl_switches_remove(); cxl_rootports_remove(); host_bridges_remove(); return rc; }
    0
}
unsafe fn cxl_topo_exit() { if type2_test { cxl_type2_topo_exit(); } else { cxl_type3_topo_exit(); } }
unsafe fn cxl_topo_init() -> c_int { if type2_test { cxl_type2_topo_init() } else { cxl_type3_topo_init() } }

unsafe fn init_static_tables() {
    for i in 0..NR_BRIDGES {
        host_bridge[i].handle = ptr::addr_of_mut!(host_bridge[i]) as acpi_handle;
        mock_pci_root[i].bus = ptr::addr_of_mut!(mock_pci_bus[i]);
    }
    mock_cfmws = [
        ptr::addr_of_mut!(mock_cedt.cfmws0.cfmws), ptr::addr_of_mut!(mock_cedt.cfmws1.cfmws),
        ptr::addr_of_mut!(mock_cedt.cfmws2.cfmws), ptr::addr_of_mut!(mock_cedt.cfmws3.cfmws),
        ptr::addr_of_mut!(mock_cedt.cfmws4.cfmws), ptr::addr_of_mut!(mock_cedt.cfmws5.cfmws),
        ptr::addr_of_mut!(mock_cedt.cfmws6.cfmws), ptr::addr_of_mut!(mock_cedt.cfmws7.cfmws),
        ptr::addr_of_mut!(mock_cedt.cfmws8.cfmws),
    ];
    mock_cxims = [ptr::addr_of_mut!(mock_cedt.cxims0.cxims)];
    for i in 0..NR_BRIDGES {
        mock_cedt.chbs[i].header.type_ = ACPI_CEDT_TYPE_CHBS as u16;
        mock_cedt.chbs[i].header.length = size_of::<acpi_cedt_chbs>() as u16;
        mock_cedt.chbs[i].uid = i as u32;
        mock_cedt.chbs[i].cxl_version = if i == 3 { ACPI_CEDT_CHBS_VERSION_CXL11 } else { ACPI_CEDT_CHBS_VERSION_CXL20 };
    }
    let windows = [0usize,1,2,3,4,5,6,7,8];
    for &i in windows.iter() {
        (*mock_cfmws[i]).header.type_ = ACPI_CEDT_TYPE_CFMWS as u16;
        (*mock_cfmws[i]).qtg_id = FAKE_QTG_ID as u32;
        (*mock_cfmws[i]).restrictions = ACPI_CEDT_CFMWS_RESTRICT_HOSTONLYMEM | if i <= 1 || i == 5 { ACPI_CEDT_CFMWS_RESTRICT_VOLATILE } else { ACPI_CEDT_CFMWS_RESTRICT_PMEM };
        (*mock_cfmws[i]).window_size = if i == 8 { SZ_512M * 6 } else if i == 1 || i == 3 || i == 6 || i == 7 { SZ_256M * 8 } else { SZ_256M * 4 };
        (*mock_cfmws[i]).granularity = 4;
        (*mock_cfmws[i]).interleave_ways = if i == 1 || i == 3 || i == 7 { 1 } else if i == 8 { 8 } else { 0 };
        if i >= 6 { (*mock_cfmws[i]).interleave_arithmetic = ACPI_CEDT_CFMWS_ARITHMETIC_XOR; }
    }
    mock_cedt.cfmws5.cfmws.window_size = if SZ_256M > PMD_SIZE { SZ_256M } else { PMD_SIZE };
    mock_cedt.cfmws7.cfmws.granularity = 0;
    mock_cedt.cfmws8.cfmws.granularity = 1;
    mock_cedt.cxims0.cxims.header.type_ = ACPI_CEDT_TYPE_CXIMS as u16;
    mock_cedt.cxims0.cxims.header.length = size_of::<cxims0>() as u16;
    mock_cedt.cxims0.cxims.hbig = 0;
    mock_cedt.cxims0.cxims.nr_xormaps = 2;
    mock_cedt.cxims0.xormap_list = [0x404100, 0x808200];
    type2_cfmws0.header.type_ = ACPI_CEDT_TYPE_CFMWS as u16;
    type2_cfmws0.header.length = size_of::<cfmws1>() as u16;
    type2_cfmws0.interleave_ways = 0;
    type2_cfmws0.granularity = 4;
    type2_cfmws0.restrictions = ACPI_CEDT_CFMWS_RESTRICT_DEVMEM | ACPI_CEDT_CFMWS_RESTRICT_VOLATILE;
    type2_cfmws0.qtg_id = FAKE_QTG_ID as u32;
    type2_cfmws0.window_size = SZ_256M * 4;
}

#[no_mangle]
pub unsafe extern "C" fn cxl_test_init() -> c_int {
    init_static_tables();
    if have_multiple_modparms() { return -EINVAL; }
    if !IS_ALIGNED(mock_auto_region_size, PMD_SIZE) {
        pr_err_once(b"mock_auto_region_size %d must be PMD-aligned\n\0".as_ptr() as *const c_char, mock_auto_region_size);
        return -EINVAL;
    }
    cxl_acpi_test(); cxl_core_test(); cxl_mem_test(); cxl_pmem_test(); cxl_port_test();
    register_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst));
    cxl_mock_pool = gen_pool_create(ilog2(SZ_2M), NUMA_NO_NODE);
    if cxl_mock_pool.is_null() { unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return -ENOMEM; }
    let mappable = mhp_get_pluggable_range(true);
    let rc = gen_pool_add(cxl_mock_pool, min(iomem_resource.end + 1 - SZ_64G, mappable.end + 1 - SZ_64G), SZ_64G, NUMA_NO_NODE);
    if rc != 0 { gen_pool_destroy(cxl_mock_pool); unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return rc; }
    if interleave_arithmetic == 1 { cfmws_start = CFMWS_XOR_ARRAY_START; cfmws_end = CFMWS_XOR_ARRAY_END; } else { cfmws_start = CFMWS_MOD_ARRAY_START; cfmws_end = CFMWS_MOD_ARRAY_END; }
    let rc = populate_cedt(); if rc != 0 { depopulate_all_mock_resources(); gen_pool_destroy(cxl_mock_pool); unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return rc; }
    let rc = cxl_topo_init(); if rc != 0 { depopulate_all_mock_resources(); gen_pool_destroy(cxl_mock_pool); unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return rc; }
    cxl_acpi = platform_device_alloc(b"cxl_acpi\0".as_ptr() as *const c_char, 0);
    if cxl_acpi.is_null() { cxl_topo_exit(); depopulate_all_mock_resources(); gen_pool_destroy(cxl_mock_pool); unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return -ENOMEM; }
    mock_companion(ptr::addr_of_mut!(acpi0017_mock), ptr::addr_of_mut!((*cxl_acpi).dev));
    (*cxl_acpi).dev.bus = ptr::addr_of_mut!(platform_bus_type);
    let rc = cxl_mock_platform_device_add(cxl_acpi, ptr::null_mut());
    if rc != 0 { cxl_topo_exit(); depopulate_all_mock_resources(); gen_pool_destroy(cxl_mock_pool); unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return rc; }
    let rc = cxl_mem_init(); if rc != 0 { platform_device_unregister(cxl_acpi); cxl_topo_exit(); depopulate_all_mock_resources(); gen_pool_destroy(cxl_mock_pool); unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return rc; }
    let rc = hmem_test_init(); if rc != 0 { cxl_mem_exit(); platform_device_unregister(cxl_acpi); cxl_topo_exit(); depopulate_all_mock_resources(); gen_pool_destroy(cxl_mock_pool); unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst)); return rc; }
    0
}

unsafe fn free_decoder_registry() {
    /* xa_for_each(&decoder_registry, index, entry) { xa_erase(); kfree(entry); } */
}

#[no_mangle]
pub unsafe extern "C" fn cxl_test_exit() {
    hmem_test_exit();
    cxl_mem_exit();
    platform_device_unregister(cxl_acpi);
    cxl_topo_exit();
    depopulate_all_mock_resources();
    gen_pool_destroy(cxl_mock_pool);
    unregister_cxl_mock_ops(ptr::addr_of_mut!(cxl_mock_ops_inst));
    free_decoder_registry();
    xa_destroy(ptr::addr_of_mut!(decoder_registry) as *mut c_void);
}

/* module_param(interleave_arithmetic, int, 0444); MODULE_PARM_DESC(interleave_arithmetic, "Modulo:0, XOR:1");
 * module_param(extended_linear_cache, bool, 0444); MODULE_PARM_DESC(extended_linear_cache, "Enable extended linear cache support");
 * module_param(fail_autoassemble, bool, 0444); MODULE_PARM_DESC(fail_autoassemble, "Simulate missing member of an auto-region");
 * module_param(type2_test, bool, 0444); MODULE_PARM_DESC(type2_test, "Enable type 2 support testing");
 * module_init(cxl_test_init); module_exit(cxl_test_exit);
 * MODULE_LICENSE("GPL v2"); MODULE_DESCRIPTION("cxl_test: setup module");
 * MODULE_IMPORT_NS("ACPI"); MODULE_IMPORT_NS("CXL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
