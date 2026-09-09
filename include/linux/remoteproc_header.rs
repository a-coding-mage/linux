/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Copyright(c) 2011 Texas Instruments, Inc.
 * Copyright(c) 2011 Google, Inc.
 * All rights reserved.
 */

use core::ffi::{c_char, c_ulong, c_void};

/* Dependencies supplied by the corresponding Linux headers. */

pub struct firmware;

#[repr(C)]
pub struct rproc_mem_entry {
    pub va: *mut c_void,
    pub is_iomem: bool,
    pub dma: dma_addr_t,
    pub len: usize,
    pub da: u32,
    pub priv_: *mut c_void,
    pub name: [c_char; 32],
    pub node: list_head,
    pub rsc_offset: u32,
    pub flags: u32,
    pub of_resm_idx: u32,
    pub alloc: Option<unsafe extern "C" fn(*mut rproc, *mut rproc_mem_entry) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut rproc, *mut rproc_mem_entry) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rsc_handling_status {
    RSC_HANDLED = 0,
    RSC_IGNORED = 1,
}

#[repr(C)]
pub struct rproc_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut rproc) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut rproc) -> i32>,
    pub start: Option<unsafe extern "C" fn(*mut rproc) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut rproc) -> i32>,
    pub attach: Option<unsafe extern "C" fn(*mut rproc) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut rproc) -> i32>,
    pub kick: Option<unsafe extern "C" fn(*mut rproc, i32)>,
    pub da_to_va: Option<unsafe extern "C" fn(*mut rproc, u64, usize, *mut bool) -> *mut c_void>,
    pub parse_fw: Option<unsafe extern "C" fn(*mut rproc, *const firmware) -> i32>,
    pub handle_rsc: Option<unsafe extern "C" fn(*mut rproc, u32, *mut c_void, i32, i32) -> i32>,
    pub find_loaded_rsc_table: Option<unsafe extern "C" fn(*mut rproc, *const firmware) -> *mut resource_table>,
    pub get_loaded_rsc_table: Option<unsafe extern "C" fn(*mut rproc, *mut usize) -> *mut resource_table>,
    pub load: Option<unsafe extern "C" fn(*mut rproc, *const firmware) -> i32>,
    pub sanity_check: Option<unsafe extern "C" fn(*mut rproc, *const firmware) -> i32>,
    pub get_boot_addr: Option<unsafe extern "C" fn(*mut rproc, *const firmware) -> u64>,
    pub panic: Option<unsafe extern "C" fn(*mut rproc) -> c_ulong>,
    pub coredump: Option<unsafe extern "C" fn(*mut rproc)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rproc_state {
    RPROC_OFFLINE = 0,
    RPROC_SUSPENDED = 1,
    RPROC_RUNNING = 2,
    RPROC_CRASHED = 3,
    RPROC_ATTACHED = 4,
    RPROC_DETACHED = 5,
    RPROC_LAST = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rproc_crash_type {
    RPROC_MMUFAULT,
    RPROC_WATCHDOG,
    RPROC_FATAL_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rproc_dump_mechanism {
    RPROC_COREDUMP_DISABLED,
    RPROC_COREDUMP_ENABLED,
    RPROC_COREDUMP_INLINE,
}

#[repr(C)]
pub struct rproc_dump_segment {
    pub node: list_head,
    pub da: dma_addr_t,
    pub size: usize,
    pub priv_: *mut c_void,
    pub dump: Option<unsafe extern "C" fn(*mut rproc, *mut rproc_dump_segment, *mut c_void, usize, usize)>,
    pub offset: loff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rproc_features {
    RPROC_FEAT_ATTACH_ON_RECOVERY,
    RPROC_MAX_FEATURES,
}

#[repr(C)]
pub struct rproc {
    pub node: list_head,
    pub domain: *mut iommu_domain,
    pub name: *const c_char,
    pub firmware: *const c_char,
    pub priv_: *mut c_void,
    pub ops: *mut rproc_ops,
    pub dev: device,
    pub power: atomic_t,
    pub state: u32,
    pub dump_conf: rproc_dump_mechanism,
    pub lock: mutex,
    pub dbg_dir: *mut dentry,
    pub traces: list_head,
    pub num_traces: i32,
    pub carveouts: list_head,
    pub mappings: list_head,
    pub bootaddr: u64,
    pub rvdevs: list_head,
    pub subdevs: list_head,
    pub notifyids: idr,
    pub index: i32,
    pub attach_work: work_struct,
    pub crash_handler: work_struct,
    pub crash_handler_lock: spinlock_t,
    pub deleting: bool,
    pub crash_cnt: u32,
    pub recovery_disabled: bool,
    pub max_notifyid: i32,
    pub table_ptr: *mut resource_table,
    pub clean_table: *mut resource_table,
    pub cached_table: *mut resource_table,
    pub table_sz: usize,
    pub has_iommu: bool,
    pub auto_boot: bool,
    pub sysfs_read_only: bool,
    pub subdevs_started: bool,
    pub dump_segments: list_head,
    pub nb_vdev: i32,
    pub elf_class: u8,
    pub elf_machine: u16,
    pub cdev: cdev,
    pub cdev_put_on_release: bool,
    /* DECLARE_BITMAP(features, RPROC_MAX_FEATURES); */
    pub features: [c_ulong; 1],
}

#[repr(C)]
pub struct rproc_subdev {
    pub node: list_head,
    pub prepare: Option<unsafe extern "C" fn(*mut rproc_subdev) -> i32>,
    pub start: Option<unsafe extern "C" fn(*mut rproc_subdev) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut rproc_subdev, bool)>,
    pub unprepare: Option<unsafe extern "C" fn(*mut rproc_subdev)>,
}

/* We currently support only two vrings per rvdev. */
pub const RVDEV_NUM_VRINGS: usize = 2;

#[repr(C)]
pub struct rproc_vring {
    pub va: *mut c_void,
    pub num: i32,
    pub da: u32,
    pub align: u32,
    pub notifyid: i32,
    pub rvdev: *mut rproc_vdev,
    pub vq: *mut virtqueue,
}

#[repr(C)]
pub struct rproc_vdev {
    pub subdev: rproc_subdev,
    pub pdev: *mut platform_device,
    pub id: u32,
    pub node: list_head,
    pub rproc: *mut rproc,
    pub vring: [rproc_vring; RVDEV_NUM_VRINGS],
    pub rsc_offset: u32,
    pub index: u32,
}

extern "C" {
    pub fn rproc_get_by_phandle(phandle: phandle) -> *mut rproc;
    pub fn rproc_get_by_child(dev: *mut device) -> *mut rproc;
    pub fn rproc_alloc(dev: *mut device, name: *const c_char, ops: *const rproc_ops, firmware: *const c_char, len: i32) -> *mut rproc;
    pub fn rproc_put(rproc: *mut rproc);
    pub fn rproc_add(rproc: *mut rproc) -> i32;
    pub fn rproc_del(rproc: *mut rproc) -> i32;
    pub fn rproc_free(rproc: *mut rproc);
    pub fn rproc_resource_cleanup(rproc: *mut rproc);
    pub fn devm_rproc_alloc(dev: *mut device, name: *const c_char, ops: *const rproc_ops, firmware: *const c_char, len: i32) -> *mut rproc;
    pub fn devm_rproc_add(dev: *mut device, rproc: *mut rproc) -> i32;
    pub fn rproc_add_carveout(rproc: *mut rproc, mem: *mut rproc_mem_entry);
    pub fn rproc_mem_entry_init(dev: *mut device, va: *mut c_void, dma: dma_addr_t, len: usize, da: u32, alloc: Option<unsafe extern "C" fn(*mut rproc, *mut rproc_mem_entry) -> i32>, release: Option<unsafe extern "C" fn(*mut rproc, *mut rproc_mem_entry) -> i32>, name: *const c_char, ...) -> *mut rproc_mem_entry;
    pub fn rproc_of_resm_mem_entry_init(dev: *mut device, of_resm_idx: u32, len: usize, da: u32, name: *const c_char, ...) -> *mut rproc_mem_entry;
    pub fn rproc_boot(rproc: *mut rproc) -> i32;
    pub fn rproc_shutdown(rproc: *mut rproc) -> i32;
    pub fn rproc_detach(rproc: *mut rproc) -> i32;
    pub fn rproc_set_firmware(rproc: *mut rproc, fw_name: *const c_char) -> i32;
    pub fn rproc_report_crash(rproc: *mut rproc, type_: rproc_crash_type);
    pub fn rproc_da_to_va(rproc: *mut rproc, da: u64, len: usize, is_iomem: *mut bool) -> *mut c_void;
    pub fn rproc_coredump_cleanup(rproc: *mut rproc);
    pub fn rproc_coredump(rproc: *mut rproc);
    pub fn rproc_coredump_using_sections(rproc: *mut rproc);
    pub fn rproc_coredump_add_segment(rproc: *mut rproc, da: dma_addr_t, size: usize) -> i32;
    pub fn rproc_coredump_add_custom_segment(rproc: *mut rproc, da: dma_addr_t, size: usize, dumpfn: Option<unsafe extern "C" fn(*mut rproc, *mut rproc_dump_segment, *mut c_void, usize, usize)>, priv_: *mut c_void) -> i32;
    pub fn rproc_coredump_set_elf_info(rproc: *mut rproc, class: u8, machine: u16) -> i32;
    pub fn rproc_add_subdev(rproc: *mut rproc, subdev: *mut rproc_subdev);
    pub fn rproc_remove_subdev(rproc: *mut rproc, subdev: *mut rproc_subdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
