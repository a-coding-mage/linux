// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */
// Linux and local header dependencies are supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut adf_driver: pci_driver;

    fn pci_release_regions(dev: *mut pci_dev);
    fn pci_disable_device(dev: *mut pci_dev);
    fn pci_iounmap(dev: *mut pci_dev, addr: *mut c_void);
    fn adf_clean_hw_data_dh895xcciov(hw: *mut adf_hw_device_data);
    fn kfree(ptr: *mut c_void);
    fn adf_dbgfs_exit(dev: *mut adf_accel_dev);
    fn adf_cfg_dev_remove(dev: *mut adf_accel_dev);
    fn adf_devmgr_pci_to_accel_dev(dev: *mut pci_dev) -> *mut adf_accel_dev;
    fn adf_devmgr_rm_dev(dev: *mut adf_accel_dev, pf: *mut adf_accel_dev);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc_node(size: usize, flags: c_ulong, node: c_int) -> *mut c_void;
    fn dev_to_node(dev: *mut device) -> c_int;
    fn adf_devmgr_add_dev(dev: *mut adf_accel_dev, pf: *mut adf_accel_dev) -> c_int;
    fn init_list_head(list: *mut list_head);
    fn adf_init_hw_data_dh895xcciov(hw: *mut adf_hw_device_data);
    fn adf_cfg_dev_add(dev: *mut adf_accel_dev) -> c_int;
    fn pci_enable_device(dev: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pci_request_regions(dev: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_select_bars(dev: *mut pci_dev, flags: c_ulong) -> c_ulong;
    fn pci_resource_start(dev: *mut pci_dev, bar: c_uint) -> u64;
    fn pci_resource_len(dev: *mut pci_dev, bar: c_uint) -> u64;
    fn pci_iomap(dev: *mut pci_dev, bar: c_uint, offset: u64) -> *mut c_void;
    fn init_completion(completion: *mut completion);
    fn adf_dbgfs_init(dev: *mut adf_accel_dev);
    fn adf_dev_up(dev: *mut adf_accel_dev, resume: bool) -> c_int;
    fn adf_dev_down(dev: *mut adf_accel_dev);
    fn adf_flush_vf_wq(dev: *mut adf_accel_dev);
    fn request_module(name: *const c_char) -> c_int;
    fn pci_register_driver(driver: *mut pci_driver) -> c_int;
    fn pci_unregister_driver(driver: *mut pci_driver);
    fn adf_clean_vf_map(force: bool);
    fn pr_err(fmt: *const c_char, ...);
}

// Types, constants, and callbacks below are provided by the included kernel and driver headers.
#[repr(C)] pub struct pci_device_id { pub _opaque: [u8; 32] }
#[repr(C)] pub struct pci_driver { pub id_table: *const pci_device_id, pub name: *const c_char, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut pci_dev)> }
#[repr(C)] pub struct pci_dev { pub device: u16, pub physfn: *mut pci_dev, pub dev: device }
#[repr(C)] pub struct device { pub _opaque: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct completion { pub _opaque: [u8; 0] }
#[repr(C)] pub struct adf_bar { pub base_addr: u64, pub size: u64, pub virt_addr: *mut c_void }
#[repr(C)] pub struct adf_accel_pci { pub pci_dev: *mut pci_dev, pub pci_bars: [adf_bar; ADF_PCI_MAX_BARS], pub sku: c_uint }
#[repr(C)] pub struct adf_vf { pub msg_received: completion }
#[repr(C)] pub struct adf_accel_dev { pub is_vf: bool, pub accel_pci_dev: adf_accel_pci, pub hw_device: *mut adf_hw_device_data, pub crypto_list: list_head, pub owner: *mut c_void, pub vf: adf_vf }
#[repr(C)] pub struct adf_hw_device_data { pub accel_mask: u32, pub ae_mask: u32, pub get_accel_mask: Option<unsafe extern "C" fn(*mut adf_hw_device_data) -> u32>, pub get_ae_mask: Option<unsafe extern "C" fn(*mut adf_hw_device_data) -> u32>, pub get_sku: Option<unsafe extern "C" fn(*mut adf_hw_device_data) -> c_uint> }

const ADF_PCI_MAX_BARS: usize = 8;
const PCI_DEVICE_ID_INTEL_QAT_DH895XCC_VF: u16 = 0x443;
const IORESOURCE_MEM: c_ulong = 0x200;
const DMA_BIT_MASK_48: u64 = (1u64 << 48) - 1;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const GFP_KERNEL: c_ulong = 0xCC0;
const ADF_DH895XCCVF_DEVICE_NAME: &[u8] = b"qat_dh895xccvf\0";

#[no_mangle]
pub static adf_pci_tbl: [pci_device_id; 2] = [pci_device_id { _opaque: [0; 32] }, pci_device_id { _opaque: [0; 32] }];
#[no_mangle]
pub static mut adf_driver: pci_driver = pci_driver { id_table: adf_pci_tbl.as_ptr(), name: ADF_DH895XCCVF_DEVICE_NAME.as_ptr() as *const c_char, probe: Some(adf_probe), remove: Some(adf_remove) };

unsafe extern "C" fn adf_cleanup_pci_dev(accel_dev: *mut adf_accel_dev) {
    pci_release_regions((*accel_dev).accel_pci_dev.pci_dev);
    pci_disable_device((*accel_dev).accel_pci_dev.pci_dev);
}

unsafe extern "C" fn adf_cleanup_accel(accel_dev: *mut adf_accel_dev) {
    let accel_pci_dev = &mut (*accel_dev).accel_pci_dev;
    for bar in accel_pci_dev.pci_bars.iter_mut() {
        if !bar.virt_addr.is_null() { pci_iounmap(accel_pci_dev.pci_dev, bar.virt_addr); }
    }
    if !(*accel_dev).hw_device.is_null() {
        if (*accel_pci_dev.pci_dev).device == PCI_DEVICE_ID_INTEL_QAT_DH895XCC_VF { adf_clean_hw_data_dh895xcciov((*accel_dev).hw_device); }
        kfree((*accel_dev).hw_device as *mut c_void);
        (*accel_dev).hw_device = core::ptr::null_mut();
    }
    adf_dbgfs_exit(accel_dev);
    adf_cfg_dev_remove(accel_dev);
    let pf = adf_devmgr_pci_to_accel_dev((*accel_pci_dev.pci_dev).physfn);
    adf_devmgr_rm_dev(accel_dev, pf);
}

unsafe extern "C" fn adf_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    let _ = ent;
    if (*pdev).device != PCI_DEVICE_ID_INTEL_QAT_DH895XCC_VF { return -ENODEV; }
    let accel_dev = kzalloc_node(core::mem::size_of::<adf_accel_dev>(), GFP_KERNEL, dev_to_node(&mut (*pdev).dev)) as *mut adf_accel_dev;
    if accel_dev.is_null() { return -ENOMEM; }
    (*accel_dev).is_vf = true;
    (*accel_dev).owner = core::ptr::null_mut();
    let pf = adf_devmgr_pci_to_accel_dev((*pdev).physfn);
    (*accel_dev).accel_pci_dev.pci_dev = pdev;
    if adf_devmgr_add_dev(accel_dev, pf) != 0 { kfree(accel_dev as *mut c_void); return -EFAULT; }
    init_list_head(&mut (*accel_dev).crypto_list);
    let hw_data = kzalloc_node(core::mem::size_of::<adf_hw_device_data>(), GFP_KERNEL, dev_to_node(&mut (*pdev).dev)) as *mut adf_hw_device_data;
    if hw_data.is_null() { adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut c_void); return -ENOMEM; }
    (*accel_dev).hw_device = hw_data;
    adf_init_hw_data_dh895xcciov(hw_data);
    (*hw_data).accel_mask = ((*hw_data).get_accel_mask.unwrap())(hw_data);
    (*hw_data).ae_mask = ((*hw_data).get_ae_mask.unwrap())(hw_data);
    (*accel_dev).accel_pci_dev.sku = ((*hw_data).get_sku.unwrap())(hw_data);
    let mut ret = adf_cfg_dev_add(accel_dev);
    if ret != 0 { adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut c_void); return ret; }
    if pci_enable_device(pdev) != 0 { ret = -EFAULT; } else { ret = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK_48); }
    if ret != 0 { adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut c_void); return ret; }
    if pci_request_regions(pdev, ADF_DH895XCCVF_DEVICE_NAME.as_ptr() as *const c_char) != 0 { ret = -EFAULT; adf_cleanup_accel(accel_dev); pci_disable_device(pdev); kfree(accel_dev as *mut c_void); return ret; }
    let mut i = 0usize;
    let mut bar_mask = pci_select_bars(pdev, IORESOURCE_MEM);
    while bar_mask != 0 && i < ADF_PCI_MAX_BARS {
        let bar_nr = bar_mask.trailing_zeros();
        bar_mask &= !(1u64 << bar_nr);
        let bar = &mut (*accel_dev).accel_pci_dev.pci_bars[i];
        bar.base_addr = pci_resource_start(pdev, bar_nr);
        if bar.base_addr == 0 { break; }
        bar.size = pci_resource_len(pdev, bar_nr);
        bar.virt_addr = pci_iomap(pdev, bar_nr, 0);
        if bar.virt_addr.is_null() { ret = -EFAULT; pci_release_regions(pdev); pci_disable_device(pdev); adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut c_void); return ret; }
        i += 1;
    }
    init_completion(&mut (*accel_dev).vf.msg_received);
    adf_dbgfs_init(accel_dev);
    ret = adf_dev_up(accel_dev, false);
    if ret != 0 { adf_dev_down(accel_dev); pci_release_regions(pdev); pci_disable_device(pdev); adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut c_void); }
    ret
}

unsafe extern "C" fn adf_remove(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() { return; }
    adf_flush_vf_wq(accel_dev); adf_dev_down(accel_dev); adf_cleanup_accel(accel_dev); adf_cleanup_pci_dev(accel_dev); kfree(accel_dev as *mut c_void);
}

unsafe extern "C" fn adfdrv_init() -> c_int { request_module(b"intel_qat\0".as_ptr() as *const c_char); if pci_register_driver(&mut adf_driver) != 0 { return -EFAULT; } 0 }
unsafe extern "C" fn adfdrv_release() { pci_unregister_driver(&mut adf_driver); adf_clean_vf_map(true); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
