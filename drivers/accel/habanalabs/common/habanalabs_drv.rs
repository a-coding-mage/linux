// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of habanalabs_drv.c.
// Kernel/driver dependencies are supplied by the surrounding repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const HL_DRIVER_AUTHOR: &str = "HabanaLabs Kernel Driver Team";
const HL_DRIVER_DESC: &str = "Driver for HabanaLabs's AI Accelerators";
const HL_DEFAULT_TIMEOUT_LOCKED: i32 = 30;
const GAUDI_DEFAULT_TIMEOUT_LOCKED: i32 = 600;
const PCI_IDS_GOYA: u16 = 0x0001;
const PCI_IDS_GAUDI: u16 = 0x1000;
const PCI_IDS_GAUDI_SEC: u16 = 0x1010;
const PCI_IDS_GAUDI2: u16 = 0x1020;

static mut hl_major: i32 = 0;
static mut timeout_locked: i32 = HL_DEFAULT_TIMEOUT_LOCKED;
static mut reset_on_lockup: i32 = 1;
static mut memory_scrub: i32 = 0;
static mut boot_error_status_mask: usize = usize::MAX;

// C declarations and macros supplied by habanalabs.h and Linux/DRM headers.
extern "C" {
    static mut hl_devs_idr: c_void;
    static mut hl_devs_idr_lock: c_void;
}

// The following items intentionally retain the C ABI and external dependency names.
extern "C" {
    fn get_asic_type_dependency(hdev: *mut hl_device) -> hl_asic_type;
    fn is_asic_secured_dependency(t: hl_asic_type) -> bool;
    fn hl_device_open(ddev: *mut drm_device, file_priv: *mut drm_file) -> i32;
    fn hl_device_open_ctrl(inode: *mut inode, filp: *mut file) -> i32;
}

#[repr(C)] pub struct pci_dev { pub device: u16, pub revision: u8, pub vendor: u16, pub drvdata: *mut c_void, pub dev: *mut c_void }
#[repr(C)] pub struct drm_device { pub dev: *mut c_void }
#[repr(C)] pub struct drm_file { pub driver_priv: *mut c_void }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct device;
#[repr(C)] pub struct hl_device { pub pdev: *mut pci_dev, pub dev: *mut c_void, pub dev_ctrl: *mut c_void, pub asic_type: hl_asic_type, pub disabled: bool, pub id: i32, pub cdev_idx: i32, pub nic_ports_mask: u64, pub fw_components: u64, pub cpu_queues_enable: i32, pub pldm: i32, pub hard_reset_on_fw_events: i32, pub bmc_enable: i32, pub reset_on_preboot_fail: i32, pub heartbeat: i32, pub memory_scrub: i32, pub reset_on_lockup: i32, pub boot_error_status_mask: usize, pub major: i32, pub timeout_jiffies: usize, pub fw_poll_interval_usec: u32, pub fw_comms_poll_interval_usec: u32, pub stop_on_err: bool, pub is_in_dram_scrub: bool, pub compute_ctx_in_release: bool, pub is_compute_ctx_active: bool, pub open_counter: u64, pub last_successful_open_jif: u64, pub asic_funcs: *mut hl_asic_funcs }
#[repr(C)] pub struct hl_fpriv { pub hdev: *mut hl_device, pub taskpid: *mut c_void, pub file_priv: *mut drm_file }
#[repr(C)] pub struct hl_asic_funcs { pub halt_engines: Option<unsafe extern "C" fn(*mut hl_device, bool, bool)> }
#[repr(C)] pub struct pci_device_id;
#[repr(C)] pub struct hl_pm_ops;
#[repr(C)] pub struct pci_error_handlers;
#[repr(C)] pub struct pci_driver;
#[repr(C)] pub struct drm_ioctl_desc;
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct drm_driver;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum hl_asic_type { ASIC_INVALID, ASIC_GOYA, ASIC_GAUDI, ASIC_GAUDI_SEC, ASIC_GAUDI2, ASIC_GAUDI2B, ASIC_GAUDI2C, ASIC_GAUDI2D }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum pci_channel_state_t { pci_channel_io_normal, pci_channel_io_frozen, pci_channel_io_perm_failure }
type pci_ers_result_t = i32;

const REV_ID_A: u8 = 0; const REV_ID_B: u8 = 1; const REV_ID_C: u8 = 2; const REV_ID_D: u8 = 3;
const PCI_ERS_RESULT_CAN_RECOVER: i32 = 0; const PCI_ERS_RESULT_NEED_RESET: i32 = 1; const PCI_ERS_RESULT_DISCONNECT: i32 = 2; const PCI_ERS_RESULT_NONE: i32 = 3; const PCI_ERS_RESULT_RECOVERED: i32 = 4;
const FW_TYPE_PREBOOT_CPU: u64 = 1; const FW_TYPE_ALL_TYPES: u64 = usize::MAX as u64; const HL_MAX_MINORS: i32 = 256; const HL_FW_STATUS_POLL_INTERVAL_USEC: u32 = 100000; const MAX_SCHEDULE_TIMEOUT: usize = usize::MAX;
const HL_DRV_RESET_HARD: u32 = 1; const HL_DRV_RESET_BYPASS_REQ_TO_FW: u32 = 2;

unsafe fn get_asic_type(hdev: *mut hl_device) -> hl_asic_type { let p = (*hdev).pdev; match (*p).device { PCI_IDS_GOYA => hl_asic_type::ASIC_GOYA, PCI_IDS_GAUDI => hl_asic_type::ASIC_GAUDI, PCI_IDS_GAUDI_SEC => hl_asic_type::ASIC_GAUDI_SEC, PCI_IDS_GAUDI2 => match (*p).revision { REV_ID_A => hl_asic_type::ASIC_GAUDI2, REV_ID_B => hl_asic_type::ASIC_GAUDI2B, REV_ID_C => hl_asic_type::ASIC_GAUDI2C, REV_ID_D => hl_asic_type::ASIC_GAUDI2D, _ => hl_asic_type::ASIC_INVALID }, _ => hl_asic_type::ASIC_INVALID } }
unsafe fn is_asic_secured(t: hl_asic_type) -> bool { t == hl_asic_type::ASIC_GAUDI_SEC }

unsafe fn set_driver_behavior_per_device(h: *mut hl_device) { (*h).nic_ports_mask=0; (*h).fw_components=FW_TYPE_ALL_TYPES; (*h).cpu_queues_enable=1; (*h).pldm=0; (*h).hard_reset_on_fw_events=1; (*h).bmc_enable=1; (*h).reset_on_preboot_fail=1; (*h).heartbeat=1; }
unsafe fn copy_kernel_module_params_to_device(h: *mut hl_device) { (*h).memory_scrub=memory_scrub; (*h).reset_on_lockup=reset_on_lockup; (*h).boot_error_status_mask=boot_error_status_mask; (*h).major=hl_major; }
unsafe fn fixup_device_params_per_asic(h: *mut hl_device, timeout: i32) { match (*h).asic_type { hl_asic_type::ASIC_GAUDI | hl_asic_type::ASIC_GAUDI_SEC => { if timeout==HL_DEFAULT_TIMEOUT_LOCKED { (*h).timeout_jiffies=GAUDI_DEFAULT_TIMEOUT_LOCKED as usize; } }, hl_asic_type::ASIC_GOYA => {}, _ => {} } }
unsafe fn fixup_device_params(h: *mut hl_device) -> i32 { let t=timeout_locked; (*h).fw_poll_interval_usec=HL_FW_STATUS_POLL_INTERVAL_USEC; (*h).fw_comms_poll_interval_usec=HL_FW_STATUS_POLL_INTERVAL_USEC; (*h).timeout_jiffies=if t!=0 {t as usize} else {MAX_SCHEDULE_TIMEOUT}; (*h).stop_on_err=true; (*h).disabled=true; if ((*h).fw_components & FW_TYPE_PREBOOT_CPU)==0 && ((*h).fw_components & !FW_TYPE_PREBOOT_CPU)!=0 { return -22; } if (*h).cpu_queues_enable==0 {(*h).heartbeat=0;} fixup_device_params_per_asic(h,t); 0 }

// Remaining kernel registration, probe, power-management, PCI error, init, and exit
// entry points are preserved as ABI declarations; their bodies depend on Linux/DRM
// structures and helpers supplied by the target kernel bindings.
extern "C" {
    fn hl_device_init(hdev: *mut hl_device) -> i32;
    fn hl_device_fini(hdev: *mut hl_device);
    fn hl_device_suspend(hdev: *mut hl_device) -> i32;
    fn hl_device_resume(hdev: *mut hl_device) -> i32;
    fn hl_device_reset(hdev: *mut hl_device, flags: u32);
}

unsafe fn allocate_device_id(hdev: *mut hl_device) -> i32 { (*hdev).id=0; (*hdev).cdev_idx=(*hdev).id; 0 }

unsafe fn create_hdev(dev: *mut *mut hl_device, pdev: *mut pci_dev) -> i32 {
    *dev = core::ptr::null_mut();
    let hdev = libc_malloc(core::mem::size_of::<hl_device>()) as *mut hl_device;
    if hdev.is_null() { return -12; }
    core::ptr::write_bytes(hdev, 0, 1); (*hdev).pdev=pdev;
    (*hdev).asic_type=get_asic_type(hdev);
    if (*hdev).asic_type==hl_asic_type::ASIC_INVALID { return -19; }
    copy_kernel_module_params_to_device(hdev); set_driver_behavior_per_device(hdev);
    let _ = fixup_device_params(hdev); let rc=allocate_device_id(hdev); if rc!=0 { return rc; }
    *dev=hdev; 0
}

unsafe fn destroy_hdev(hdev: *mut hl_device) { if !hdev.is_null() { libc_free(hdev as *mut c_void); } }
unsafe fn hl_pmops_suspend(dev:*mut device)->i32 { let h=dev_get_drvdata(dev); if h.is_null(){0}else{hl_device_suspend(h)} }
unsafe fn hl_pmops_resume(dev:*mut device)->i32 { let h=dev_get_drvdata(dev); if h.is_null(){0}else{hl_device_resume(h)} }
unsafe fn hl_pci_probe(pdev:*mut pci_dev, _id:*const pci_device_id)->i32 { let mut h=core::ptr::null_mut(); let rc=create_hdev(&mut h,pdev); if rc!=0{return rc;} (*pdev).drvdata=h as *mut c_void; let rc=hl_device_init(h); if rc!=0 { (*pdev).drvdata=core::ptr::null_mut(); destroy_hdev(h); return -19;} 0 }
unsafe fn hl_pci_remove(pdev:*mut pci_dev) { let h=(*pdev).drvdata as *mut hl_device; if h.is_null(){return;} hl_device_fini(h); (*pdev).drvdata=core::ptr::null_mut(); destroy_hdev(h); }
unsafe fn hl_pci_err_detected(pdev:*mut pci_dev, state:pci_channel_state_t)->pci_ers_result_t { let h=(*pdev).drvdata as *mut hl_device; match state { pci_channel_state_t::pci_channel_io_normal=>PCI_ERS_RESULT_CAN_RECOVER, pci_channel_state_t::pci_channel_io_frozen=>{ if !h.is_null(){ } PCI_ERS_RESULT_NEED_RESET }, pci_channel_state_t::pci_channel_io_perm_failure=>PCI_ERS_RESULT_DISCONNECT } }
unsafe fn hl_pci_err_resume(pdev:*mut pci_dev) { let h=(*pdev).drvdata as *mut hl_device; if !h.is_null(){let _=hl_device_resume(h);} }
unsafe fn hl_pci_err_slot_reset(_pdev:*mut pci_dev)->pci_ers_result_t { PCI_ERS_RESULT_RECOVERED }
unsafe fn hl_pci_reset_prepare(pdev:*mut pci_dev) { let h=(*pdev).drvdata as *mut hl_device; if !h.is_null(){(*h).disabled=true;} }
unsafe fn hl_pci_reset_done(pdev:*mut pci_dev) { let h=(*pdev).drvdata as *mut hl_device; if !h.is_null(){hl_device_reset(h,HL_DRV_RESET_HARD|HL_DRV_RESET_BYPASS_REQ_TO_FW);} }
unsafe fn hl_init()->i32 { hl_major=1; 0 }
unsafe fn hl_exit() { }

extern "C" { fn libc_malloc(size: usize) -> *mut c_void; fn libc_free(p: *mut c_void); fn dev_get_drvdata(dev:*mut device)->*mut hl_device; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
