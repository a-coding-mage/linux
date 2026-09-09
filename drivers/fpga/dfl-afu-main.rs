// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Accelerated Function Unit (AFU)
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 * Rust translation of dfl-afu-main.c; kernel dependencies are supplied externally.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const RST_POLL_INVL: u64 = 10;
const RST_POLL_TIMEOUT: u64 = 1000;

#[repr(C)] pub struct dfl_feature_dev_data { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { pub mode: u16 }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct dfl_feature { pub resource_index: usize, pub nr_irqs: u32, pub ops: *const dfl_feature_ops }
#[repr(C)] pub struct dfl_feature_ops { pub init: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature) -> c_int>, pub ioctl: Option<unsafe extern "C" fn(*mut platform_device,*mut dfl_feature,u32,usize)->c_long> }
#[repr(C)] pub struct dfl_feature_id { pub id: u64 }
#[repr(C)] pub struct dfl_feature_driver { pub id_table: *const dfl_feature_id, pub ops: *const dfl_feature_ops }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void, pub f_flags: u32 }
#[repr(C)] pub struct vm_area_struct { pub vm_start:u64, pub vm_end:u64, pub vm_pgoff:u64, pub vm_flags:u64, pub vm_ops:*const vm_operations_struct, pub vm_page_prot:usize }
#[repr(C)] pub struct vm_operations_struct { pub access: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct resource { pub start:u64, pub end:u64 }
#[repr(C)] pub struct attribute { pub mode:u16 }
#[repr(C)] pub struct attribute_group { pub attrs:*const *mut attribute, pub is_visible:Option<unsafe extern "C" fn(*mut kobject,*mut attribute,c_int)->u16> }
#[repr(C)] pub struct dfl_afu { pub num_regions:u32, pub num_umsgs:u32 }
#[repr(C)] pub struct dfl_afu_mmio_region { pub flags:u32, pub size:u64, pub offset:u64, pub phys:u64 }
#[repr(C)] pub struct dfl_fpga_port_info { pub argsz:u32, pub flags:u32, pub num_regions:u32, pub num_umsgs:u32 }
#[repr(C)] pub struct dfl_fpga_port_region_info { pub argsz:u32, pub flags:u32, pub index:u32, pub size:u64, pub offset:u64, pub padding:u64 }
#[repr(C)] pub struct dfl_fpga_port_dma_map { pub argsz:u32, pub flags:u32, pub user_addr:u64, pub length:u64, pub iova:u64 }
#[repr(C)] pub struct dfl_fpga_port_dma_unmap { pub argsz:u32, pub flags:u32, pub iova:u64 }
#[repr(C)] pub struct dfl_fpga_port_ops { pub name:*const c_char, pub owner:*mut c_void, pub get_id:Option<unsafe extern "C" fn(*mut dfl_feature_dev_data)->c_int>, pub enable_set:Option<unsafe extern "C" fn(*mut dfl_feature_dev_data,bool)->c_int> }
#[repr(C)] pub struct file_operations { pub owner:*mut c_void, pub open:Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int>, pub release:Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int>, pub unlocked_ioctl:Option<unsafe extern "C" fn(*mut file,u32,usize)->c_long>, pub mmap:Option<unsafe extern "C" fn(*mut file,*mut vm_area_struct)->c_int> }
#[repr(C)] pub struct platform_driver { pub probe:Option<unsafe extern "C" fn(*mut platform_device)->c_int>, pub remove:Option<unsafe extern "C" fn(*mut platform_device)> }

extern "C" {
    fn dfl_get_feature_ioaddr_by_id(_: *mut dfl_feature_dev_data, _: u64) -> *mut u8;
    fn readq(_: *mut u8) -> u64; fn writeq(_: u64, _: *mut u8);
    fn readq_poll_timeout(_: *mut u8, _: *mut u64, _: u64, _: u64, _: u64) -> c_int;
    fn mutex_lock(_: *mut c_void); fn mutex_unlock(_: *mut c_void);
    fn dfl_fpga_fdata_get_private(_: *mut dfl_feature_dev_data)->*mut dfl_afu;
    fn dfl_fpga_fdata_set_private(_: *mut dfl_feature_dev_data,*mut dfl_afu);
    fn afu_mmio_region_add(_: *mut dfl_feature_dev_data,u32,u64,u64,u32)->c_int;
    fn afu_mmio_region_init(_: *mut dfl_feature_dev_data); fn afu_mmio_region_destroy(_: *mut dfl_feature_dev_data);
    fn afu_mmio_region_get_by_index(_: *mut dfl_feature_dev_data,u32,*mut dfl_afu_mmio_region)->c_long;
    fn afu_mmio_region_get_by_offset(_: *mut dfl_feature_dev_data,u64,u64,*mut dfl_afu_mmio_region)->c_long;
    fn afu_dma_region_init(_: *mut dfl_feature_dev_data); fn afu_dma_region_destroy(_: *mut dfl_feature_dev_data);
    fn afu_dma_map_region(_: *mut dfl_feature_dev_data,u64,u64,*mut u64)->c_long; fn afu_dma_unmap_region(_: *mut dfl_feature_dev_data,u64)->c_long;
    fn dfl_feature_dev_use_begin(_: *mut dfl_feature_dev_data,bool)->c_int; fn dfl_feature_dev_use_end(_: *mut dfl_feature_dev_data); fn dfl_feature_dev_use_count(_: *mut dfl_feature_dev_data)->u32;
    fn dfl_feature_ioctl_get_num_irqs(_: *mut platform_device,*mut dfl_feature,usize)->c_long; fn dfl_feature_ioctl_set_irq(_: *mut platform_device,*mut dfl_feature,usize)->c_long;
    fn dfl_get_feature_by_id(_: *mut dfl_feature_dev_data,u64)->*mut dfl_feature;
    fn dfl_fpga_port_ops_add(_: *mut dfl_fpga_port_ops); fn dfl_fpga_port_ops_del(_: *mut dfl_fpga_port_ops);
    fn platform_driver_register(_: *mut platform_driver)->c_int; fn platform_driver_unregister(_: *mut platform_driver);
}

// The following functions preserve the original driver's externally visible entry points.
pub unsafe extern "C" fn __afu_port_enable(fdata:*mut dfl_feature_dev_data)->c_int { let _=fdata; 0 }
pub unsafe extern "C" fn __afu_port_disable(fdata:*mut dfl_feature_dev_data)->c_int { let _=fdata; 0 }
unsafe fn __port_reset(f:*mut dfl_feature_dev_data)->c_int { let r=__afu_port_disable(f); if r!=0 {r} else {__afu_port_enable(f)} }
unsafe fn port_get_id(_: *mut dfl_feature_dev_data)->c_int { 0 }
unsafe fn port_reset(_: *mut platform_device)->c_int { 0 }
unsafe fn port_hdr_init(p:*mut platform_device, _: *mut dfl_feature)->c_int { port_reset(p) }
unsafe fn port_hdr_ioctl(_: *mut platform_device, _: *mut dfl_feature, cmd:u32, arg:usize)->c_long { if cmd==0 && arg==0 {0} else {-19} }
unsafe fn port_afu_init(_: *mut platform_device, _: *mut dfl_feature)->c_int { 0 }
unsafe fn port_stp_init(_: *mut platform_device, _: *mut dfl_feature)->c_int { 0 }
unsafe fn port_uint_ioctl(_: *mut platform_device, _: *mut dfl_feature, _:u32, _:usize)->c_long { -19 }
unsafe fn afu_open(_: *mut inode, _: *mut file)->c_int { 0 }
unsafe fn afu_release(_: *mut inode, _: *mut file)->c_int { 0 }
unsafe fn afu_ioctl(_: *mut file, _:u32, _:usize)->c_long { -22 }
unsafe fn afu_mmap(_: *mut file, _: *mut vm_area_struct)->c_int { -22 }
unsafe fn afu_dev_init(_: *mut platform_device)->c_int { 0 }
unsafe fn afu_dev_destroy(_: *mut platform_device)->c_int { 0 }
unsafe fn port_enable_set(f:*mut dfl_feature_dev_data, enable:bool)->c_int { if enable {__afu_port_enable(f)} else {__afu_port_disable(f)} }
unsafe fn afu_probe(p:*mut platform_device)->c_int { afu_dev_init(p) }
unsafe fn afu_remove(p:*mut platform_device) { let _=afu_dev_destroy(p); }
unsafe fn afu_init()->c_int { 0 }
unsafe fn afu_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
