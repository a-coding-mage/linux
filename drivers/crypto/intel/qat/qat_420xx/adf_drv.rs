// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Linux kernel and local driver dependencies are supplied by the surrounding
// Rust translation environment.

use core::ptr;

extern "C" {
    static mut THIS_MODULE: module;
    static adf_err_handler: pci_error_handlers;
    static adf_sriov_configure: unsafe extern "C" fn(*mut pci_dev, i32) -> i32;

    fn adf_clean_hw_data_420xx(hw_device: *mut adf_hw_device_data);
    fn adf_dbgfs_exit(accel_dev: *mut adf_accel_dev);
    fn adf_cfg_dev_remove(accel_dev: *mut adf_accel_dev);
    fn adf_devmgr_rm_dev(accel_dev: *mut adf_accel_dev, dev: *mut core::ffi::c_void);
    fn adf_devmgr_add_dev(accel_dev: *mut adf_accel_dev, dev: *mut core::ffi::c_void) -> i32;
    fn adf_init_hw_data_420xx(hw_device: *mut adf_hw_device_data, device: u16);
    fn adf_cfg_dev_add(accel_dev: *mut adf_accel_dev) -> i32;
    fn adf_gen4_cfg_dev_init(accel_dev: *mut adf_accel_dev) -> i32;
    fn adf_dbgfs_init(accel_dev: *mut adf_accel_dev);
    fn adf_dev_up(accel_dev: *mut adf_accel_dev, sync: bool) -> i32;
    fn adf_dev_down(accel_dev: *mut adf_accel_dev);
    fn adf_sysfs_init(accel_dev: *mut adf_accel_dev) -> i32;
    fn adf_devmgr_pci_to_accel_dev(pdev: *mut pci_dev) -> *mut adf_accel_dev;
}

#[repr(C)]
struct module;
#[repr(C)]
struct pci_dev { _private: [u8; 0] }
#[repr(C)]
struct pci_error_handlers { _private: [u8; 0] }
#[repr(C)]
struct adf_accel_dev { _private: [u8; 0] }
#[repr(C)]
struct adf_hw_device_data { _private: [u8; 0] }

#[repr(C)]
struct pci_device_id {
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
    class: u32,
    class_mask: u32,
    driver_data: usize,
}

#[repr(C)]
struct pci_driver {
    id_table: *const pci_device_id,
    name: *const core::ffi::c_char,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    shutdown: Option<unsafe extern "C" fn(*mut pci_dev)>,
    sriov_configure: Option<unsafe extern "C" fn(*mut pci_dev, i32) -> i32>,
    err_handler: *const pci_error_handlers,
}

// PCI/device constants and structure layouts are provided by the kernel
// bindings and are represented here by their source-level names.
extern "C" {
    fn num_possible_nodes() -> i32;
    fn dev_to_node(dev: *mut core::ffi::c_void) -> i32;
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn pci_read_config_byte(pdev: *mut pci_dev, where_: u32, val: *mut u8) -> i32;
    fn pci_read_config_dword(pdev: *mut pci_dev, where_: u32, val: *mut u32) -> i32;
    fn pci_select_bars(pdev: *mut pci_dev, mask: u32) -> u64;
    fn pcim_enable_device(pdev: *mut pci_dev) -> i32;
    fn dma_set_mask_and_coherent(dev: *mut core::ffi::c_void, mask: u64) -> i32;
    fn pcim_request_all_regions(pdev: *mut pci_dev, name: *const core::ffi::c_char) -> i32;
    fn pci_name(pdev: *mut pci_dev) -> *const core::ffi::c_char;
    fn pcim_iomap(pdev: *mut pci_dev, bar: u32, offset: usize) -> *mut core::ffi::c_void;
    fn pci_save_state(pdev: *mut pci_dev) -> i32;
}

static adf_pci_tbl: [pci_device_id; 2] = [
    pci_device_id { vendor: 0x8086, device: PCI_DEVICE_ID_INTEL_QAT_420XX, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

unsafe fn adf_cleanup_accel(accel_dev: *mut adf_accel_dev) {
    // The pointed-to fields are accessed through the supplied C-compatible
    // adf_accel_dev layout.
    if !(*accel_dev).hw_device.is_null() {
        adf_clean_hw_data_420xx((*accel_dev).hw_device);
        (*accel_dev).hw_device = ptr::null_mut();
    }
    adf_dbgfs_exit(accel_dev);
    adf_cfg_dev_remove(accel_dev);
    adf_devmgr_rm_dev(accel_dev, ptr::null_mut());
}

unsafe extern "C" fn adf_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    if num_possible_nodes() > 1 && dev_to_node(pdev as *mut core::ffi::c_void) < 0 {
        return -22;
    }

    let accel_dev = devm_kzalloc(pdev as *mut core::ffi::c_void, core::mem::size_of::<adf_accel_dev>(), 0) as *mut adf_accel_dev;
    if accel_dev.is_null() { return -12; }

    if adf_devmgr_add_dev(accel_dev, ptr::null_mut()) != 0 {
        return -14;
    }

    let hw_data = devm_kzalloc(pdev as *mut core::ffi::c_void, core::mem::size_of::<adf_hw_device_data>(), 0) as *mut adf_hw_device_data;
    if hw_data.is_null() {
        adf_cleanup_accel(accel_dev);
        return -12;
    }
    (*accel_dev).hw_device = hw_data;
    adf_init_hw_data_420xx(hw_data, (*ent).device as u16);

    let mut ret = adf_cfg_dev_add(accel_dev);
    if ret != 0 { adf_cleanup_accel(accel_dev); return ret; }
    ret = pcim_enable_device(pdev);
    if ret != 0 { adf_cleanup_accel(accel_dev); return ret; }
    ret = dma_set_mask_and_coherent(pdev as *mut core::ffi::c_void, u64::MAX);
    if ret != 0 { adf_cleanup_accel(accel_dev); return ret; }
    ret = adf_gen4_cfg_dev_init(accel_dev);
    if ret != 0 { adf_cleanup_accel(accel_dev); return ret; }
    adf_dbgfs_init(accel_dev);
    ret = adf_dev_up(accel_dev, true);
    if ret != 0 { adf_cleanup_accel(accel_dev); return ret; }
    ret = adf_sysfs_init(accel_dev);
    if ret != 0 { adf_dev_down(accel_dev); adf_cleanup_accel(accel_dev); return ret; }
    ret
}

unsafe extern "C" fn adf_remove(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() { return; }
    adf_dev_down(accel_dev);
    adf_cleanup_accel(accel_dev);
}

unsafe extern "C" fn adf_shutdown(pdev: *mut pci_dev) {
    adf_dev_down(adf_devmgr_pci_to_accel_dev(pdev));
}

static mut adf_driver: pci_driver = pci_driver {
    id_table: adf_pci_tbl.as_ptr(),
    name: b"qat_420xx\0".as_ptr() as *const core::ffi::c_char,
    probe: Some(adf_probe),
    remove: Some(adf_remove),
    shutdown: Some(adf_shutdown),
    sriov_configure: Some(adf_sriov_configure),
    err_handler: &adf_err_handler,
};

// module_pci_driver(adf_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Intel");
// MODULE_FIRMWARE(ADF_420XX_FW);
// MODULE_FIRMWARE(ADF_420XX_MMP);
// MODULE_DESCRIPTION("Intel(R) QuickAssist Technology");
// MODULE_SOFTDEP("pre: crypto-intel_qat");
// MODULE_IMPORT_NS("CRYPTO_QAT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
