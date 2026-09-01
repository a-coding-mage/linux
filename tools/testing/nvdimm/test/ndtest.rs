// SPDX-License-Identifier: GPL-2.0-only
// pr_fmt(fmt) was KBUILD_MODNAME ": " fmt in the C source.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type u64 = u64;
type dma_addr_t = u64;
type resource_size_t = u64;
type umode_t = u16;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const GFP_KERNEL: c_int = 0;
const NUMA_NO_NODE: c_int = -1;
const PAGE_SIZE: usize = 4096;

const SZ_128K: usize = 128 * 1024;
const SZ_16M: usize = 16 * 1024 * 1024;
const SZ_32M: usize = 32 * 1024 * 1024;
const SZ_128M: usize = 128 * 1024 * 1024;
const SZ_4M: usize = 4 * 1024 * 1024;
const SZ_4G: u64 = 4 * 1024 * 1024 * 1024;

const DIMM_SIZE: usize = SZ_32M;
const LABEL_SIZE: usize = SZ_128K;
const NUM_INSTANCES: usize = 2;
const NUM_DCR: usize = 4;
const NDTEST_MAX_MAPPING: usize = 6;

const ND_CMD_GET_CONFIG_SIZE: u32 = 0;
const ND_CMD_GET_CONFIG_DATA: u32 = 1;
const ND_CMD_SET_CONFIG_DATA: u32 = 2;
const ND_CMD_CALL: u32 = 10;

const NDTEST_SCM_DIMM_CMD_MASK: c_ulong = (1u64 << ND_CMD_GET_CONFIG_SIZE
    | 1u64 << ND_CMD_GET_CONFIG_DATA
    | 1u64 << ND_CMD_SET_CONFIG_DATA
    | 1u64 << ND_CMD_CALL) as c_ulong;

const ND_DEVICE_NAMESPACE_PMEM: c_int = 0;
const ND_DEVICE_NAMESPACE_IO: c_int = 1;
const NDD_LABELING: c_int = 0;
const NDD_UNARMED: c_int = 1;

const PAPR_PMEM_UNARMED: u64 = 1 << 0;
const PAPR_PMEM_EMPTY: u64 = 1 << 1;
const PAPR_PMEM_SAVE_FAILED: u64 = 1 << 2;
const PAPR_PMEM_SHUTDOWN_DIRTY: u64 = 1 << 3;
const PAPR_PMEM_HEALTH_FATAL: u64 = 1 << 4;
const PAPR_PMEM_UNARMED_MASK: u64 = PAPR_PMEM_UNARMED;
const PAPR_PMEM_BAD_SHUTDOWN_MASK: u64 = PAPR_PMEM_SHUTDOWN_DIRTY;
const PAPR_PMEM_BAD_RESTORE_MASK: u64 = PAPR_PMEM_EMPTY;
const PAPR_PMEM_SAVE_MASK: u64 = PAPR_PMEM_SAVE_FAILED;
const PAPR_PMEM_SMART_EVENT_MASK: u64 = PAPR_PMEM_HEALTH_FATAL;

const fn NFIT_DIMM_HANDLE(node: u32, socket: u32, imc: u32, chan: u32, dimm: u32) -> u32 {
    ((node & 0xfff) << 16)
        | ((socket & 0xf) << 12)
        | ((imc & 0xf) << 8)
        | ((chan & 0xf) << 4)
        | (dimm & 0xf)
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    raw: c_int,
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    name: *const c_char,
    mode: umode_t,
}

#[repr(C)]
pub struct device_attribute {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *mut c_char) -> ssize_t>,
    store: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *const c_char, size_t) -> ssize_t>,
}

#[repr(C)]
pub struct attribute_group {
    name: *const c_char,
    attrs: *mut *mut attribute,
    is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> umode_t>,
}

#[repr(C)]
pub struct class {
    name: *const c_char,
}

#[repr(C)]
pub struct device {
    kobj: kobject,
    release: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct platform_device {
    name: *const c_char,
    id: c_int,
    dev: device,
}

#[repr(C)]
pub struct platform_device_id {
    name: *const c_char,
}

#[repr(C)]
pub struct driver {
    name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: driver,
    id_table: *const platform_device_id,
}

#[repr(C)]
pub struct resource {
    start: resource_size_t,
    end: resource_size_t,
    name: *const c_char,
}

#[repr(C)]
pub struct gen_pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genpool_data_align {
    align: usize,
}

#[repr(C)]
pub struct nfit_test_resource {
    list: list_head,
    dev: *mut device,
    buf: *mut c_void,
    res: resource,
    lock: spinlock_t,
    requests: list_head,
}

#[repr(C)]
pub struct nvdimm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nvdimm_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nvdimm_bus_descriptor {
    ndctl: Option<
        unsafe extern "C" fn(
            *mut nvdimm_bus_descriptor,
            *mut nvdimm,
            u32,
            *mut c_void,
            u32,
            *mut c_int,
        ) -> c_int,
    >,
    module: *mut c_void,
    provider_name: *const c_char,
    attr_groups: *const *const attribute_group,
}

#[repr(C)]
pub struct nd_region {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nd_mapping_desc {
    start: u64,
    size: u64,
    position: c_int,
    nvdimm: *mut nvdimm,
}

#[repr(C)]
pub struct nd_interleave_set {
    cookie1: u64,
    cookie2: u64,
    altcookie: u64,
}

#[repr(C)]
pub struct nd_region_desc {
    mapping: *mut nd_mapping_desc,
    res: *mut resource,
    provider_data: *mut c_void,
    attr_groups: *const *const attribute_group,
    nd_set: *mut nd_interleave_set,
    num_mappings: c_int,
}

#[repr(C)]
pub struct nd_cmd_get_config_data_hdr {
    status: u32,
    in_offset: u32,
    in_length: u32,
    out_buf: [u8; 0],
}

#[repr(C)]
pub struct nd_cmd_set_config_hdr {
    in_offset: u32,
    in_length: u32,
    in_buf: [u8; 0],
}

#[repr(C)]
pub struct nd_cmd_get_config_size {
    status: u32,
    max_xfer: u32,
    config_size: u32,
}

#[repr(C)]
pub struct seq_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ndtest_dimm {
    size: usize,
    handle: u32,
    uuid_str: *const c_char,
    physical_id: u32,
    num_formats: c_int,
    flags: u64,
    nvdimm: *mut nvdimm,
    dev: *mut device,
    id: c_int,
    label_area: *mut u8,
    config_size: u32,
    fail_cmd: c_ulong,
    fail_cmd_code: c_int,
    address: dma_addr_t,
}

#[repr(C)]
pub struct ndtest_mapping {
    dimm: c_int,
    position: c_int,
    start: u64,
    size: u64,
}

#[repr(C)]
pub struct ndtest_region {
    type_: c_int,
    num_mappings: c_int,
    mapping: *mut ndtest_mapping,
    size: usize,
    range_index: c_int,
    region: *mut nd_region,
}

#[repr(C)]
pub struct ndtest_config {
    dimm_start: c_int,
    dimm_count: c_int,
    dimms: *mut ndtest_dimm,
    regions: *mut ndtest_region,
    num_regions: c_int,
}

#[repr(C)]
pub struct ndtest_priv {
    pdev: platform_device,
    resources: list_head,
    config: *mut ndtest_config,
    bus_desc: nvdimm_bus_descriptor,
    bus: *mut nvdimm_bus,
    dcr_dma: *mut dma_addr_t,
    label_dma: *mut dma_addr_t,
    dimm_dma: *mut dma_addr_t,
    dn: *mut c_void,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn kstrtol(buf: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_int) -> *mut c_void;
    fn devm_add_action(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn gen_pool_create(min_alloc_order: c_int, nid: c_int) -> *mut gen_pool;
    fn gen_pool_add(pool: *mut gen_pool, addr: u64, size: u64, nid: c_int) -> c_int;
    fn gen_pool_destroy(pool: *mut gen_pool);
    fn gen_pool_alloc_algo(pool: *mut gen_pool, size: size_t, algo: *mut c_void, data: *mut c_void) -> dma_addr_t;
    fn gen_pool_free(pool: *mut gen_pool, addr: dma_addr_t, size: size_t);
    static mut gen_pool_first_fit_align: *mut c_void;
    fn resource_size(res: *const resource) -> resource_size_t;
    fn class_register(class: *const class) -> c_int;
    fn class_unregister(class: *const class);
    fn device_create_with_groups(class: *const class, parent: *mut device, devt: u64, drvdata: *mut c_void, groups: *const *const attribute_group, fmt: *const c_char, ...) -> *mut device;
    fn device_unregister(dev: *mut device);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_device_register(pdev: *mut platform_device) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn put_device(dev: *mut device);
    fn get_device(dev: *mut device) -> *mut device;
    fn nvdimm_bus_register(dev: *mut device, desc: *mut nvdimm_bus_descriptor) -> *mut nvdimm_bus;
    fn nvdimm_bus_unregister(bus: *mut nvdimm_bus);
    fn nvdimm_create(bus: *mut nvdimm_bus, provider_data: *mut c_void, groups: *const *const attribute_group, flags: c_ulong, cmd_mask: c_ulong, num_flush: c_int, flush: *mut c_void) -> *mut nvdimm;
    fn nvdimm_provider_data(nvdimm: *mut nvdimm) -> *mut c_void;
    fn nvdimm_pmem_region_create(bus: *mut nvdimm_bus, desc: *mut nd_region_desc) -> *mut nd_region;
    fn nd_region_provider_data(region: *mut nd_region) -> *mut c_void;
    fn to_nvdimm(dev: *mut device) -> *mut nvdimm;
    fn to_nd_region(dev: *mut device) -> *mut nd_region;
    fn uuid_parse(src: *const c_char, dst: *mut c_void) -> c_int;
    fn cpu_to_le64(v: u64) -> u64;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn seq_buf_init(s: *mut seq_buf, buf: *mut c_char, size: size_t);
    fn seq_buf_printf(s: *mut seq_buf, fmt: *const c_char, ...);
    fn seq_buf_used(s: *const seq_buf) -> size_t;
    fn pmem_test();
    fn libnvdimm_test();
    fn device_dax_test();
    fn dax_pmem_test();
    fn nfit_test_setup(lookup: unsafe extern "C" fn(resource_size_t) -> *mut nfit_test_resource, data: *mut c_void);
    fn nfit_test_teardown();
    fn ilog2(n: u64) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

type c_uint = u32;

static mut ndtest_lock: spinlock_t = spinlock_t { raw: 0 };
static mut instances: [*mut ndtest_priv; NUM_INSTANCES] = [ptr::null_mut(); NUM_INSTANCES];

static ndtest_dimm_class: class = class {
    name: b"nfit_test_dimm\0".as_ptr() as *const c_char,
};

static mut ndtest_pool: *mut gen_pool = ptr::null_mut();

static mut dimm_group1: [ndtest_dimm; 5] = [
    ndtest_dimm { size: DIMM_SIZE, handle: NFIT_DIMM_HANDLE(0, 0, 0, 0, 0), uuid_str: b"1e5c75d2-b618-11ea-9aa3-507b9ddc0f72\0".as_ptr() as *const c_char, physical_id: 0, num_formats: 2, flags: 0, nvdimm: ptr::null_mut(), dev: ptr::null_mut(), id: 0, label_area: ptr::null_mut(), config_size: 0, fail_cmd: 0, fail_cmd_code: 0, address: 0 },
    ndtest_dimm { size: DIMM_SIZE, handle: NFIT_DIMM_HANDLE(0, 0, 0, 0, 1), uuid_str: b"1c4d43ac-b618-11ea-be80-507b9ddc0f72\0".as_ptr() as *const c_char, physical_id: 1, num_formats: 2, flags: 0, nvdimm: ptr::null_mut(), dev: ptr::null_mut(), id: 0, label_area: ptr::null_mut(), config_size: 0, fail_cmd: 0, fail_cmd_code: 0, address: 0 },
    ndtest_dimm { size: DIMM_SIZE, handle: NFIT_DIMM_HANDLE(0, 0, 1, 0, 0), uuid_str: b"a9f17ffc-b618-11ea-b36d-507b9ddc0f72\0".as_ptr() as *const c_char, physical_id: 2, num_formats: 2, flags: 0, nvdimm: ptr::null_mut(), dev: ptr::null_mut(), id: 0, label_area: ptr::null_mut(), config_size: 0, fail_cmd: 0, fail_cmd_code: 0, address: 0 },
    ndtest_dimm { size: DIMM_SIZE, handle: NFIT_DIMM_HANDLE(0, 0, 1, 0, 1), uuid_str: b"b6b83b22-b618-11ea-8aae-507b9ddc0f72\0".as_ptr() as *const c_char, physical_id: 3, num_formats: 2, flags: 0, nvdimm: ptr::null_mut(), dev: ptr::null_mut(), id: 0, label_area: ptr::null_mut(), config_size: 0, fail_cmd: 0, fail_cmd_code: 0, address: 0 },
    ndtest_dimm { size: DIMM_SIZE, handle: NFIT_DIMM_HANDLE(0, 1, 0, 0, 0), uuid_str: b"bf9baaee-b618-11ea-b181-507b9ddc0f72\0".as_ptr() as *const c_char, physical_id: 4, num_formats: 2, flags: 0, nvdimm: ptr::null_mut(), dev: ptr::null_mut(), id: 0, label_area: ptr::null_mut(), config_size: 0, fail_cmd: 0, fail_cmd_code: 0, address: 0 },
];

static mut dimm_group2: [ndtest_dimm; 1] = [
    ndtest_dimm { size: DIMM_SIZE, handle: NFIT_DIMM_HANDLE(1, 0, 0, 0, 0), uuid_str: b"ca0817e2-b618-11ea-9db3-507b9ddc0f72\0".as_ptr() as *const c_char, physical_id: 0, num_formats: 1, flags: PAPR_PMEM_UNARMED | PAPR_PMEM_EMPTY | PAPR_PMEM_SAVE_FAILED | PAPR_PMEM_SHUTDOWN_DIRTY | PAPR_PMEM_HEALTH_FATAL, nvdimm: ptr::null_mut(), dev: ptr::null_mut(), id: 0, label_area: ptr::null_mut(), config_size: 0, fail_cmd: 0, fail_cmd_code: 0, address: 0 },
];

static mut region0_mapping: [ndtest_mapping; 2] = [
    ndtest_mapping { dimm: 0, position: 0, start: 0, size: SZ_16M as u64 },
    ndtest_mapping { dimm: 1, position: 1, start: 0, size: SZ_16M as u64 },
];

static mut region1_mapping: [ndtest_mapping; 4] = [
    ndtest_mapping { dimm: 0, position: 0, start: SZ_16M as u64, size: SZ_16M as u64 },
    ndtest_mapping { dimm: 1, position: 1, start: SZ_16M as u64, size: SZ_16M as u64 },
    ndtest_mapping { dimm: 2, position: 2, start: SZ_16M as u64, size: SZ_16M as u64 },
    ndtest_mapping { dimm: 3, position: 3, start: SZ_16M as u64, size: SZ_16M as u64 },
];

static mut bus0_regions: [ndtest_region; 2] = [
    ndtest_region { type_: ND_DEVICE_NAMESPACE_PMEM, num_mappings: 2, mapping: unsafe { region0_mapping.as_mut_ptr() }, size: DIMM_SIZE, range_index: 1, region: ptr::null_mut() },
    ndtest_region { type_: ND_DEVICE_NAMESPACE_PMEM, num_mappings: 4, mapping: unsafe { region1_mapping.as_mut_ptr() }, size: DIMM_SIZE * 2, range_index: 2, region: ptr::null_mut() },
];

static mut region6_mapping: [ndtest_mapping; 1] = [
    ndtest_mapping { dimm: 0, position: 0, start: 0, size: DIMM_SIZE as u64 },
];

static mut bus1_regions: [ndtest_region; 1] = [
    ndtest_region { type_: ND_DEVICE_NAMESPACE_IO, num_mappings: 1, mapping: unsafe { region6_mapping.as_mut_ptr() }, size: DIMM_SIZE, range_index: 1, region: ptr::null_mut() },
];

static mut bus_configs: [ndtest_config; NUM_INSTANCES] = [
    ndtest_config { dimm_start: 0, dimm_count: 5, dimms: unsafe { dimm_group1.as_mut_ptr() }, regions: unsafe { bus0_regions.as_mut_ptr() }, num_regions: 2 },
    ndtest_config { dimm_start: 5, dimm_count: 1, dimms: unsafe { dimm_group2.as_mut_ptr() }, regions: unsafe { bus1_regions.as_mut_ptr() }, num_regions: 1 },
];

unsafe fn to_ndtest_priv(dev: *mut device) -> *mut ndtest_priv {
    let pdev = dev as *mut platform_device;
    pdev as *mut ndtest_priv
}

unsafe extern "C" fn ndtest_config_get(p: *mut ndtest_dimm, buf_len: u32, hdr: *mut nd_cmd_get_config_data_hdr) -> c_int {
    if (*hdr).in_offset.wrapping_add((*hdr).in_length) > LABEL_SIZE as u32 {
        return -EINVAL;
    }
    (*hdr).status = 0;
    let len = core::cmp::min((*hdr).in_length, LABEL_SIZE as u32 - (*hdr).in_offset);
    memcpy((*hdr).out_buf.as_mut_ptr() as *mut c_void, (*p).label_area.add((*hdr).in_offset as usize) as *const c_void, len as size_t);
    buf_len.wrapping_sub(len) as c_int
}

unsafe extern "C" fn ndtest_config_set(p: *mut ndtest_dimm, buf_len: u32, hdr: *mut nd_cmd_set_config_hdr) -> c_int {
    if (*hdr).in_offset.wrapping_add((*hdr).in_length) > LABEL_SIZE as u32 {
        return -EINVAL;
    }
    let len = core::cmp::min((*hdr).in_length, LABEL_SIZE as u32 - (*hdr).in_offset);
    memcpy((*p).label_area.add((*hdr).in_offset as usize) as *mut c_void, (*hdr).in_buf.as_ptr() as *const c_void, len as size_t);
    buf_len.wrapping_sub(len) as c_int
}

unsafe extern "C" fn ndtest_get_config_size(dimm: *mut ndtest_dimm, _buf_len: u32, size: *mut nd_cmd_get_config_size) -> c_int {
    (*size).status = 0;
    (*size).max_xfer = 8;
    (*size).config_size = (*dimm).config_size;
    0
}

unsafe extern "C" fn ndtest_ctl(
    _nd_desc: *mut nvdimm_bus_descriptor,
    nvdimm: *mut nvdimm,
    cmd: u32,
    buf: *mut c_void,
    buf_len: u32,
    mut cmd_rc: *mut c_int,
) -> c_int {
    let mut _cmd_rc: c_int = 0;
    if cmd_rc.is_null() {
        cmd_rc = &mut _cmd_rc;
    }
    *cmd_rc = 0;
    if nvdimm.is_null() {
        return -EINVAL;
    }
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    if dimm.is_null() {
        return -EINVAL;
    }
    match cmd {
        ND_CMD_GET_CONFIG_SIZE => *cmd_rc = ndtest_get_config_size(dimm, buf_len, buf as *mut nd_cmd_get_config_size),
        ND_CMD_GET_CONFIG_DATA => *cmd_rc = ndtest_config_get(dimm, buf_len, buf as *mut nd_cmd_get_config_data_hdr),
        ND_CMD_SET_CONFIG_DATA => *cmd_rc = ndtest_config_set(dimm, buf_len, buf as *mut nd_cmd_set_config_hdr),
        _ => return -EINVAL,
    }
    /*
     * Failures for a DIMM can be injected using fail_cmd and
     * fail_cmd_code, see the device attributes below
     */
    if ((1u64 << cmd) as c_ulong & (*dimm).fail_cmd) != 0 {
        return if (*dimm).fail_cmd_code != 0 { (*dimm).fail_cmd_code } else { -EIO };
    }
    0
}

unsafe extern "C" fn ndtest_resource_lookup(addr: resource_size_t) -> *mut nfit_test_resource {
    let mut i = 0;
    while i < NUM_INSTANCES {
        let mut nfit_res: *mut nfit_test_resource = ptr::null_mut();
        let t = instances[i];
        if t.is_null() {
            i += 1;
            continue;
        }
        let mut n = (*t).resources.next as *mut nfit_test_resource;
        while !n.is_null() && (n as *mut list_head) != &mut (*t).resources {
            if addr >= (*n).res.start && addr < (*n).res.start + resource_size(&(*n).res) {
                nfit_res = n;
                break;
            } else if addr >= (*n).buf as c_ulong as u64
                && addr < (*n).buf as c_ulong as u64 + resource_size(&(*n).res)
            {
                nfit_res = n;
                break;
            }
            n = (*n).list.next as *mut nfit_test_resource;
        }
        if !nfit_res.is_null() {
            return nfit_res;
        }
        i += 1;
    }
    pr_warn(b"Failed to get resource\n\0".as_ptr() as *const c_char);
    ptr::null_mut()
}

unsafe extern "C" fn ndtest_release_resource(data: *mut c_void) {
    let res = data as *mut nfit_test_resource;
    if resource_size(&(*res).res) >= DIMM_SIZE as u64 {
        gen_pool_free(ndtest_pool, (*res).res.start, resource_size(&(*res).res) as size_t);
    }
    vfree((*res).buf);
    kfree(res as *mut c_void);
}

unsafe fn ndtest_alloc_resource(p: *mut ndtest_priv, size: size_t, dma: *mut dma_addr_t) -> *mut c_void {
    let mut __dma: dma_addr_t;
    let data = genpool_data_align { align: SZ_128M };
    let res = kzalloc(size_of::<nfit_test_resource>(), GFP_KERNEL) as *mut nfit_test_resource;
    if res.is_null() {
        return ptr::null_mut();
    }
    let buf = vmalloc(size);
    if size >= DIMM_SIZE {
        let mut data_mut = data;
        __dma = gen_pool_alloc_algo(ndtest_pool, size, gen_pool_first_fit_align, &mut data_mut as *mut _ as *mut c_void);
    } else {
        __dma = buf as c_ulong as dma_addr_t;
    }
    if __dma == 0 {
        if __dma != 0 && size >= DIMM_SIZE {
            gen_pool_free(ndtest_pool, __dma, size);
        }
        vfree(buf);
        kfree(res as *mut c_void);
        return ptr::null_mut();
    }
    (*res).dev = &mut (*p).pdev.dev;
    (*res).buf = buf;
    (*res).res.start = __dma;
    (*res).res.end = __dma + size as u64 - 1;
    (*res).res.name = b"NFIT\0".as_ptr() as *const c_char;
    if !dma.is_null() {
        *dma = __dma;
    }
    if devm_add_action(&mut (*p).pdev.dev, ndtest_release_resource, res as *mut c_void) == 0 {
        return (*res).buf;
    }
    if __dma != 0 && size >= DIMM_SIZE {
        gen_pool_free(ndtest_pool, __dma, size);
    }
    vfree(buf);
    kfree(res as *mut c_void);
    ptr::null_mut()
}

unsafe extern "C" fn range_index_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let nd_region = to_nd_region(dev);
    let region = nd_region_provider_data(nd_region) as *mut ndtest_region;
    sprintf(buf, b"%d\n\0".as_ptr() as *const c_char, (*region).range_index) as ssize_t
}

static mut dev_attr_range_index: device_attribute = device_attribute {
    attr: attribute { name: b"range_index\0".as_ptr() as *const c_char, mode: 0o444 },
    show: Some(range_index_show),
    store: None,
};

static mut ndtest_region_attributes: [*mut attribute; 2] = unsafe { [&mut dev_attr_range_index.attr, ptr::null_mut()] };
static ndtest_region_attribute_group: attribute_group = attribute_group {
    name: b"papr\0".as_ptr() as *const c_char,
    attrs: unsafe { ndtest_region_attributes.as_ptr() as *mut *mut attribute },
    is_visible: None,
};
static ndtest_region_attribute_groups: [*const attribute_group; 2] = [&ndtest_region_attribute_group, ptr::null()];

unsafe fn ndtest_create_region(p: *mut ndtest_priv, region: *mut ndtest_region) -> c_int {
    let mut mappings: [nd_mapping_desc; NDTEST_MAX_MAPPING] = core::mem::zeroed();
    let mut _ndr_desc: nd_region_desc = core::mem::zeroed();
    let ndr_desc = &mut _ndr_desc as *mut nd_region_desc;
    let mut res: resource = core::mem::zeroed();
    let mut ndimm = (*(*region).mapping).dimm;
    let mut uuid: [u64; 2] = [0; 2];
    if ndtest_alloc_resource(p, (*region).size, &mut res.start).is_null() {
        return -ENOMEM;
    }
    res.end = res.start + (*region).size as u64 - 1;
    (*ndr_desc).mapping = mappings.as_mut_ptr();
    (*ndr_desc).res = &mut res;
    (*ndr_desc).provider_data = region as *mut c_void;
    (*ndr_desc).attr_groups = ndtest_region_attribute_groups.as_ptr();
    if uuid_parse((*(*(*p).config).dimms.add(ndimm as usize)).uuid_str, uuid.as_mut_ptr() as *mut c_void) != 0 {
        pr_err(b"failed to parse UUID\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    let nd_set = devm_kzalloc(&mut (*p).pdev.dev, size_of::<nd_interleave_set>(), GFP_KERNEL) as *mut nd_interleave_set;
    if nd_set.is_null() {
        return -ENOMEM;
    }
    (*nd_set).cookie1 = cpu_to_le64(uuid[0]);
    (*nd_set).cookie2 = cpu_to_le64(uuid[1]);
    (*nd_set).altcookie = (*nd_set).cookie1;
    (*ndr_desc).nd_set = nd_set;
    let mut i = 0;
    while i < (*region).num_mappings {
        ndimm = (*(*region).mapping.add(i as usize)).dimm;
        mappings[i as usize].start = (*(*region).mapping.add(i as usize)).start;
        mappings[i as usize].size = (*(*region).mapping.add(i as usize)).size;
        mappings[i as usize].position = (*(*region).mapping.add(i as usize)).position;
        mappings[i as usize].nvdimm = (*(*(*p).config).dimms.add(ndimm as usize)).nvdimm;
        i += 1;
    }
    (*ndr_desc).num_mappings = (*region).num_mappings;
    (*region).region = nvdimm_pmem_region_create((*p).bus, ndr_desc);
    if (*region).region.is_null() {
        dev_err(&mut (*p).pdev.dev, b"Error registering region %pR\n\0".as_ptr() as *const c_char, (*ndr_desc).res);
        return -ENXIO;
    }
    0
}

unsafe fn ndtest_init_regions(p: *mut ndtest_priv) -> c_int {
    let mut i = 0;
    while i < (*(*p).config).num_regions {
        let ret = ndtest_create_region(p, (*(*p).config).regions.add(i as usize));
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn put_dimms(data: *mut c_void) {
    let p = data as *mut ndtest_priv;
    let mut i = 0;
    while i < (*(*p).config).dimm_count {
        let dimm = (*(*p).config).dimms.add(i as usize);
        if !(*dimm).dev.is_null() {
            device_unregister((*dimm).dev);
            (*dimm).dev = ptr::null_mut();
        }
        i += 1;
    }
}

unsafe extern "C" fn handle_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dimm = dev_get_drvdata(dev) as *mut ndtest_dimm;
    sprintf(buf, b"%#x\n\0".as_ptr() as *const c_char, (*dimm).handle) as ssize_t
}

unsafe extern "C" fn fail_cmd_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dimm = dev_get_drvdata(dev) as *mut ndtest_dimm;
    sprintf(buf, b"%#x\n\0".as_ptr() as *const c_char, (*dimm).fail_cmd) as ssize_t
}

unsafe extern "C" fn fail_cmd_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, size: size_t) -> ssize_t {
    let dimm = dev_get_drvdata(dev) as *mut ndtest_dimm;
    let mut val: c_ulong = 0;
    let rc = kstrtol(buf, 0, &mut val);
    if rc != 0 {
        return rc as ssize_t;
    }
    (*dimm).fail_cmd = val;
    size as ssize_t
}

unsafe extern "C" fn fail_cmd_code_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let dimm = dev_get_drvdata(dev) as *mut ndtest_dimm;
    sprintf(buf, b"%d\n\0".as_ptr() as *const c_char, (*dimm).fail_cmd_code) as ssize_t
}

unsafe extern "C" fn fail_cmd_code_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, size: size_t) -> ssize_t {
    let dimm = dev_get_drvdata(dev) as *mut ndtest_dimm;
    let mut val: c_ulong = 0;
    let rc = kstrtol(buf, 0, &mut val);
    if rc != 0 {
        return rc as ssize_t;
    }
    (*dimm).fail_cmd_code = val as c_int;
    size as ssize_t
}

static mut dev_attr_handle: device_attribute = device_attribute { attr: attribute { name: b"handle\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(handle_show), store: None };
static mut dev_attr_fail_cmd: device_attribute = device_attribute { attr: attribute { name: b"fail_cmd\0".as_ptr() as *const c_char, mode: 0o644 }, show: Some(fail_cmd_show), store: Some(fail_cmd_store) };
static mut dev_attr_fail_cmd_code: device_attribute = device_attribute { attr: attribute { name: b"fail_cmd_code\0".as_ptr() as *const c_char, mode: 0o644 }, show: Some(fail_cmd_code_show), store: Some(fail_cmd_code_store) };
static mut dimm_attributes: [*mut attribute; 4] = unsafe { [&mut dev_attr_handle.attr, &mut dev_attr_fail_cmd.attr, &mut dev_attr_fail_cmd_code.attr, ptr::null_mut()] };
static mut dimm_attribute_group: attribute_group = attribute_group { name: ptr::null(), attrs: unsafe { dimm_attributes.as_ptr() as *mut *mut attribute }, is_visible: None };
static mut dimm_attribute_groups: [*const attribute_group; 2] = unsafe { [&mut dimm_attribute_group, ptr::null()] };

unsafe extern "C" fn phys_id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let nvdimm = to_nvdimm(dev);
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    sprintf(buf, b"%#x\n\0".as_ptr() as *const c_char, (*dimm).physical_id) as ssize_t
}

unsafe extern "C" fn vendor_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"0x1234567\n\0".as_ptr() as *const c_char) as ssize_t
}

unsafe extern "C" fn id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let nvdimm = to_nvdimm(dev);
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    sprintf(buf, b"%04x-%02x-%04x-%08x\0".as_ptr() as *const c_char, 0xabcd, 0xa, 2016, !(*dimm).handle) as ssize_t
}

unsafe extern "C" fn nvdimm_handle_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let nvdimm = to_nvdimm(dev);
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    sprintf(buf, b"%#x\n\0".as_ptr() as *const c_char, (*dimm).handle) as ssize_t
}

static mut dev_attr_nvdimm_show_handle: device_attribute = device_attribute {
    attr: attribute { name: b"handle\0".as_ptr() as *const c_char, mode: 0o444 },
    show: Some(nvdimm_handle_show),
    store: None,
};

unsafe extern "C" fn subsystem_vendor_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"0x%04x\n\0".as_ptr() as *const c_char, 0) as ssize_t
}

unsafe extern "C" fn dirty_shutdown_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"%d\n\0".as_ptr() as *const c_char, 42) as ssize_t
}

unsafe extern "C" fn formats_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let nvdimm = to_nvdimm(dev);
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    sprintf(buf, b"%d\n\0".as_ptr() as *const c_char, (*dimm).num_formats) as ssize_t
}

unsafe extern "C" fn format_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let nvdimm = to_nvdimm(dev);
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    if (*dimm).num_formats > 1 {
        return sprintf(buf, b"0x201\n\0".as_ptr() as *const c_char) as ssize_t;
    }
    sprintf(buf, b"0x101\n\0".as_ptr() as *const c_char) as ssize_t
}

unsafe extern "C" fn format1_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"0x301\n\0".as_ptr() as *const c_char) as ssize_t
}

static mut dev_attr_vendor: device_attribute = device_attribute { attr: attribute { name: b"vendor\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(vendor_show), store: None };
static mut dev_attr_id: device_attribute = device_attribute { attr: attribute { name: b"id\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(id_show), store: None };
static mut dev_attr_phys_id: device_attribute = device_attribute { attr: attribute { name: b"phys_id\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(phys_id_show), store: None };
static mut dev_attr_subsystem_vendor: device_attribute = device_attribute { attr: attribute { name: b"subsystem_vendor\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(subsystem_vendor_show), store: None };
static mut dev_attr_dirty_shutdown: device_attribute = device_attribute { attr: attribute { name: b"dirty_shutdown\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(dirty_shutdown_show), store: None };
static mut dev_attr_formats: device_attribute = device_attribute { attr: attribute { name: b"formats\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(formats_show), store: None };
static mut dev_attr_format: device_attribute = device_attribute { attr: attribute { name: b"format\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(format_show), store: None };
static mut dev_attr_format1: device_attribute = device_attribute { attr: attribute { name: b"format1\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(format1_show), store: None };

unsafe extern "C" fn ndtest_nvdimm_attr_visible(kobj: *mut kobject, a: *mut attribute, _n: c_int) -> umode_t {
    let dev = kobj as *mut device;
    let nvdimm = to_nvdimm(dev);
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    if a == &mut dev_attr_format1.attr && (*dimm).num_formats <= 1 {
        return 0;
    }
    (*a).mode
}

unsafe extern "C" fn flags_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let nvdimm = to_nvdimm(dev);
    let dimm = nvdimm_provider_data(nvdimm) as *mut ndtest_dimm;
    let mut s: seq_buf = core::mem::zeroed();
    let flags = (*dimm).flags;
    seq_buf_init(&mut s, buf, PAGE_SIZE);
    if flags & PAPR_PMEM_UNARMED_MASK != 0 { seq_buf_printf(&mut s, b"not_armed \0".as_ptr() as *const c_char); }
    if flags & PAPR_PMEM_BAD_SHUTDOWN_MASK != 0 { seq_buf_printf(&mut s, b"flush_fail \0".as_ptr() as *const c_char); }
    if flags & PAPR_PMEM_BAD_RESTORE_MASK != 0 { seq_buf_printf(&mut s, b"restore_fail \0".as_ptr() as *const c_char); }
    if flags & PAPR_PMEM_SAVE_MASK != 0 { seq_buf_printf(&mut s, b"save_fail \0".as_ptr() as *const c_char); }
    if flags & PAPR_PMEM_SMART_EVENT_MASK != 0 { seq_buf_printf(&mut s, b"smart_notify \0".as_ptr() as *const c_char); }
    if seq_buf_used(&s) != 0 { seq_buf_printf(&mut s, b"\n\0".as_ptr() as *const c_char); }
    seq_buf_used(&s) as ssize_t
}

static mut dev_attr_flags: device_attribute = device_attribute { attr: attribute { name: b"flags\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(flags_show), store: None };
static mut ndtest_nvdimm_attributes: [*mut attribute; 11] = unsafe {
    [
        &mut dev_attr_nvdimm_show_handle.attr,
        &mut dev_attr_vendor.attr,
        &mut dev_attr_id.attr,
        &mut dev_attr_phys_id.attr,
        &mut dev_attr_subsystem_vendor.attr,
        &mut dev_attr_dirty_shutdown.attr,
        &mut dev_attr_formats.attr,
        &mut dev_attr_format.attr,
        &mut dev_attr_format1.attr,
        &mut dev_attr_flags.attr,
        ptr::null_mut(),
    ]
};
static ndtest_nvdimm_attribute_group: attribute_group = attribute_group {
    name: b"papr\0".as_ptr() as *const c_char,
    attrs: unsafe { ndtest_nvdimm_attributes.as_ptr() as *mut *mut attribute },
    is_visible: Some(ndtest_nvdimm_attr_visible),
};
static ndtest_nvdimm_attribute_groups: [*const attribute_group; 2] = [&ndtest_nvdimm_attribute_group, ptr::null()];

unsafe fn ndtest_dimm_register(priv_: *mut ndtest_priv, dimm: *mut ndtest_dimm, id: c_int) -> c_int {
    let dev = &mut (*priv_).pdev.dev as *mut device;
    let mut dimm_flags = (*dimm).flags as c_ulong;
    if (*dimm).num_formats > 1 {
        set_bit(NDD_LABELING, &mut dimm_flags);
    }
    if (*dimm).flags & PAPR_PMEM_UNARMED_MASK != 0 {
        set_bit(NDD_UNARMED, &mut dimm_flags);
    }
    (*dimm).nvdimm = nvdimm_create((*priv_).bus, dimm as *mut c_void, ndtest_nvdimm_attribute_groups.as_ptr(), dimm_flags, NDTEST_SCM_DIMM_CMD_MASK, 0, ptr::null_mut());
    if (*dimm).nvdimm.is_null() {
        dev_err(dev, b"Error creating DIMM object for %pOF\n\0".as_ptr() as *const c_char, (*priv_).dn);
        return -ENXIO;
    }
    (*dimm).dev = device_create_with_groups(&ndtest_dimm_class, &mut (*priv_).pdev.dev, 0, dimm as *mut c_void, dimm_attribute_groups.as_ptr(), b"test_dimm%d\0".as_ptr() as *const c_char, id);
    if (*dimm).dev.is_null() {
        pr_err(b"Could not create dimm device attributes\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    0
}

unsafe fn ndtest_nvdimm_init(p: *mut ndtest_priv) -> c_int {
    let mut i = 0;
    while i < (*(*p).config).dimm_count {
        let d = (*(*p).config).dimms.add(i as usize);
        let id = (*(*p).config).dimm_start + i;
        (*d).id = id;
        let res = ndtest_alloc_resource(p, LABEL_SIZE, ptr::null_mut());
        if res.is_null() { return -ENOMEM; }
        (*d).label_area = res as *mut u8;
        sprintf((*d).label_area as *mut c_char, b"label%d\0".as_ptr() as *const c_char, id);
        (*d).config_size = LABEL_SIZE as u32;
        if ndtest_alloc_resource(p, (*d).size, (*p).dimm_dma.add(id as usize)).is_null() { return -ENOMEM; }
        if ndtest_alloc_resource(p, LABEL_SIZE, (*p).label_dma.add(id as usize)).is_null() { return -ENOMEM; }
        if ndtest_alloc_resource(p, LABEL_SIZE, (*p).dcr_dma.add(id as usize)).is_null() { return -ENOMEM; }
        (*d).address = *(*p).dimm_dma.add(id as usize);
        ndtest_dimm_register(p, d, id);
        i += 1;
    }
    0
}

unsafe extern "C" fn compatible_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"nvdimm_test\0".as_ptr() as *const c_char) as ssize_t
}

static mut dev_attr_compatible: device_attribute = device_attribute { attr: attribute { name: b"compatible\0".as_ptr() as *const c_char, mode: 0o444 }, show: Some(compatible_show), store: None };
static mut of_node_attributes: [*mut attribute; 2] = unsafe { [&mut dev_attr_compatible.attr, ptr::null_mut()] };
static of_node_attribute_group: attribute_group = attribute_group { name: b"of_node\0".as_ptr() as *const c_char, attrs: unsafe { of_node_attributes.as_ptr() as *mut *mut attribute }, is_visible: None };
static ndtest_attribute_groups: [*const attribute_group; 2] = [&of_node_attribute_group, ptr::null()];

unsafe fn ndtest_bus_register(p: *mut ndtest_priv) -> c_int {
    (*p).config = &mut bus_configs[(*p).pdev.id as usize];
    (*p).bus_desc.ndctl = Some(ndtest_ctl);
    (*p).bus_desc.module = THIS_MODULE;
    (*p).bus_desc.provider_name = ptr::null();
    (*p).bus_desc.attr_groups = ndtest_attribute_groups.as_ptr();
    (*p).bus = nvdimm_bus_register(&mut (*p).pdev.dev, &mut (*p).bus_desc);
    if (*p).bus.is_null() {
        dev_err(&mut (*p).pdev.dev, b"Error creating nvdimm bus %pOF\n\0".as_ptr() as *const c_char, (*p).dn);
        return -ENOMEM;
    }
    0
}

unsafe extern "C" fn ndtest_remove(pdev: *mut platform_device) {
    let p = to_ndtest_priv(&mut (*pdev).dev);
    nvdimm_bus_unregister((*p).bus);
}

unsafe extern "C" fn ndtest_probe(pdev: *mut platform_device) -> c_int {
    let p = to_ndtest_priv(&mut (*pdev).dev);
    if ndtest_bus_register(p) != 0 {
        return -ENOMEM;
    }
    (*p).dcr_dma = devm_kcalloc(&mut (*p).pdev.dev, NUM_DCR, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
    if (*p).dcr_dma.is_null() { pr_err(b"%s:%d Failed nvdimm init\n\0".as_ptr() as *const c_char, b"ndtest_probe\0".as_ptr() as *const c_char, line!()); return -ENOMEM; }
    (*p).label_dma = devm_kcalloc(&mut (*p).pdev.dev, NUM_DCR, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
    if (*p).label_dma.is_null() { pr_err(b"%s:%d Failed nvdimm init\n\0".as_ptr() as *const c_char, b"ndtest_probe\0".as_ptr() as *const c_char, line!()); return -ENOMEM; }
    (*p).dimm_dma = devm_kcalloc(&mut (*p).pdev.dev, NUM_DCR, size_of::<dma_addr_t>(), GFP_KERNEL) as *mut dma_addr_t;
    if (*p).dimm_dma.is_null() { pr_err(b"%s:%d Failed nvdimm init\n\0".as_ptr() as *const c_char, b"ndtest_probe\0".as_ptr() as *const c_char, line!()); return -ENOMEM; }
    let mut rc = ndtest_nvdimm_init(p);
    if rc != 0 { pr_err(b"%s:%d Failed nvdimm init\n\0".as_ptr() as *const c_char, b"ndtest_probe\0".as_ptr() as *const c_char, line!()); return rc; }
    rc = ndtest_init_regions(p);
    if rc != 0 { pr_err(b"%s:%d Failed nvdimm init\n\0".as_ptr() as *const c_char, b"ndtest_probe\0".as_ptr() as *const c_char, line!()); return rc; }
    rc = devm_add_action_or_reset(&mut (*pdev).dev, put_dimms, p as *mut c_void);
    if rc != 0 { pr_err(b"%s:%d Failed nvdimm init\n\0".as_ptr() as *const c_char, b"ndtest_probe\0".as_ptr() as *const c_char, line!()); return rc; }
    platform_set_drvdata(pdev, p as *mut c_void);
    0
}

static ndtest_id: [platform_device_id; 2] = [
    platform_device_id { name: b"ndtest\0".as_ptr() as *const c_char },
    platform_device_id { name: ptr::null() },
];

static mut ndtest_driver: platform_driver = platform_driver {
    probe: Some(ndtest_probe),
    remove: Some(ndtest_remove),
    driver: driver { name: b"ndtest\0".as_ptr() as *const c_char },
    id_table: ndtest_id.as_ptr(),
};

unsafe extern "C" fn ndtest_release(dev: *mut device) {
    let p = to_ndtest_priv(dev);
    kfree(p as *mut c_void);
}

unsafe fn cleanup_devices() {
    let mut i = 0;
    while i < NUM_INSTANCES {
        if !instances[i].is_null() {
            platform_device_unregister(&mut (*instances[i]).pdev);
        }
        i += 1;
    }
    nfit_test_teardown();
    if !ndtest_pool.is_null() {
        gen_pool_destroy(ndtest_pool);
    }
    class_unregister(&ndtest_dimm_class);
}

unsafe extern "C" fn ndtest_init() -> c_int {
    pmem_test();
    libnvdimm_test();
    device_dax_test();
    dax_pmem_test();
    nfit_test_setup(ndtest_resource_lookup, ptr::null_mut());
    let mut rc = class_register(&ndtest_dimm_class);
    if rc != 0 {
        pr_err(b"Error registering platform device\n\0".as_ptr() as *const c_char);
        cleanup_devices();
        return rc;
    }
    ndtest_pool = gen_pool_create(ilog2(SZ_4M as u64), NUMA_NO_NODE);
    if ndtest_pool.is_null() {
        rc = -ENOMEM;
        pr_err(b"Error registering platform device\n\0".as_ptr() as *const c_char);
        cleanup_devices();
        return rc;
    }
    if gen_pool_add(ndtest_pool, SZ_4G, SZ_4G, NUMA_NO_NODE) != 0 {
        rc = -ENOMEM;
        pr_err(b"Error registering platform device\n\0".as_ptr() as *const c_char);
        cleanup_devices();
        return rc;
    }
    /* Each instance can be taken as a bus, which can have multiple dimms */
    let mut i = 0;
    while i < NUM_INSTANCES {
        let priv_ = kzalloc(size_of::<ndtest_priv>(), GFP_KERNEL) as *mut ndtest_priv;
        if priv_.is_null() {
            rc = -ENOMEM;
            pr_err(b"Error registering platform device\n\0".as_ptr() as *const c_char);
            cleanup_devices();
            return rc;
        }
        let pdev = &mut (*priv_).pdev as *mut platform_device;
        (*pdev).name = b"ndtest\0".as_ptr() as *const c_char;
        (*pdev).id = i as c_int;
        (*pdev).dev.release = Some(ndtest_release);
        rc = platform_device_register(pdev);
        if rc != 0 {
            put_device(&mut (*pdev).dev);
            pr_err(b"Error registering platform device\n\0".as_ptr() as *const c_char);
            cleanup_devices();
            return rc;
        }
        get_device(&mut (*pdev).dev);
        instances[i] = priv_;
        i += 1;
    }
    rc = platform_driver_register(&mut ndtest_driver);
    if rc != 0 {
        pr_err(b"Error registering platform device\n\0".as_ptr() as *const c_char);
        cleanup_devices();
        return rc;
    }
    0
}

unsafe extern "C" fn ndtest_exit() {
    cleanup_devices();
    platform_driver_unregister(&mut ndtest_driver);
}

// module_init(ndtest_init);
// module_exit(ndtest_exit);
// MODULE_DESCRIPTION("Test non-NFIT devices");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("IBM Corporation");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
