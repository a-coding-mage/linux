// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of acpi_processor.c. External kernel symbols are supplied by dependencies. */

// Includes and build-time configuration symbols from the C source are intentionally
// represented by external names/conditional comments; this file is a source-level
// translation and does not provide dependency implementations.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::{ffi::c_void, ptr};

extern "C" {
    static mut processors: *mut acpi_processor;
    static mut errata: acpi_processor_errata;
    fn per_cpu_processor_device_array(cpu: i32) -> *mut c_void;
    fn acpi_driver_data(device: *mut acpi_device) -> *mut acpi_processor;
    fn acpi_get_handle(a: acpi_handle, b: *const i8, c: *mut acpi_handle) -> acpi_status;
}

/* The following declarations retain the C ABI names and layout supplied by headers. */
#[repr(C)] pub struct acpi_processor { pub handle: acpi_handle, pub id: i32, pub acpi_id: u32, pub phys_id: u32, pub flags: acpi_processor_flags, pub throttling: acpi_processor_throttling, pub pblk: u64, pub dev: *mut device }
#[repr(C)] pub struct acpi_processor_flags { pub bm_control: u32 }
#[repr(C)] pub struct acpi_processor_throttling { pub shared_cpu_map: *mut c_void, pub address: u64, pub duty_offset: u32, pub duty_width: u32 }
#[repr(C)] pub struct acpi_processor_errata { pub piix4: acpi_processor_piix4 }
#[repr(C)] pub struct acpi_processor_piix4 { pub throttle: u32, pub bmisx: u64, pub fdma: u32 }
#[repr(C)] pub struct acpi_lpi_states_array { pub size: u32, pub composite_states_size: u32, pub entries: *mut acpi_lpi_state, pub composite_states: [*mut acpi_lpi_state; 16] }

extern "C" { fn acpi_get_processor_handle(cpu: i32) -> acpi_handle; }

pub unsafe fn acpi_get_processor_handle_local(cpu: i32) -> acpi_handle {
    let pr = processors.add(cpu as usize);
    if !(*pr).handle.is_null() { (*pr).handle } else { ptr::null_mut() }
}

unsafe fn acpi_processor_errata_piix4(dev: *mut pci_dev) -> i32 {
    if dev.is_null() { return -22; }
    match (*dev).revision { 0 => dev_dbg((*dev).dev, "Found PIIX4 A-step\n"), 1 => dev_dbg((*dev).dev, "Found PIIX4 B-step\n"), 2 => dev_dbg((*dev).dev, "Found PIIX4E\n"), 3 => dev_dbg((*dev).dev, "Found PIIX4M\n"), _ => dev_dbg((*dev).dev, "Found unknown PIIX4\n") }
    match (*dev).revision {
        0 | 1 => { errata.piix4.throttle = 1; /* fallthrough */ }
        2 | 3 => {}
        _ => return 0,
    }
    let mut d = pci_get_subsys(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB, PCI_ANY_ID, PCI_ANY_ID, ptr::null_mut());
    if !d.is_null() { errata.piix4.bmisx = pci_resource_start(d, 4); if errata.piix4.bmisx != 0 { dev_dbg((*d).dev, "Bus master activity detection (BM-IDE) erratum enabled\n"); } pci_dev_put(d); }
    d = pci_get_subsys(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB_0, PCI_ANY_ID, PCI_ANY_ID, ptr::null_mut());
    if !d.is_null() { let mut v1=0u8; let mut v2=0u8; pci_read_config_byte(d,0x76,&mut v1); pci_read_config_byte(d,0x77,&mut v2); if (v1&0x80)!=0 || (v2&0x80)!=0 { errata.piix4.fdma=1; dev_dbg((*d).dev,"Type-F DMA livelock erratum (C3 disabled)\n"); } pci_dev_put(d); }
    0
}

unsafe fn acpi_processor_errata_local() -> i32 {
    let d = pci_get_subsys(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82371AB_3, PCI_ANY_ID, PCI_ANY_ID, ptr::null_mut());
    if d.is_null() { 0 } else { let r=acpi_processor_errata_piix4(d); pci_dev_put(d); r }
}

unsafe fn cpufreq_add_device(name: *const i8) { let p=platform_device_register_simple(name, PLATFORM_DEVID_NONE, ptr::null_mut(), 0); if IS_ERR(p) { pr_info("%s device creation failed: %pe\n",name,p); } }

#[cfg(feature="CONFIG_X86")]
unsafe fn acpi_pcc_cpufreq_init() { let mut h=ptr::null_mut(); if ACPI_FAILURE(acpi_get_handle(ptr::null_mut(), b"\\_SB\0".as_ptr() as *const i8,&mut h)) { return; } if acpi_has_method(h,b"PCCH\0".as_ptr() as *const i8) { cpufreq_add_device(b"pcc-cpufreq\0".as_ptr() as *const i8); } }
#[cfg(not(feature="CONFIG_X86"))] unsafe fn acpi_pcc_cpufreq_init() {}

static mut processor_device_array: *mut *mut c_void = ptr::null_mut();

unsafe fn acpi_processor_set_per_cpu(pr:*mut acpi_processor, device:*mut acpi_device)->i32 {
    BUG_ON((*pr).id >= nr_cpu_ids);
    let slot=processor_device_array.add((*pr).id as usize);
    if !(*slot).is_null() && *slot != device as *mut c_void { dev_warn((*device).dev,"BIOS reported wrong ACPI id %d for the processor\n",(*pr).id); return -22; }
    *slot=device as *mut c_void; processors.add((*pr).id as usize).write(pr); 0
}

#[cfg(feature="CONFIG_ACPI_HOTPLUG_CPU")]
unsafe fn acpi_processor_hotadd_init(pr:*mut acpi_processor, device:*mut acpi_device)->i32 {
    if invalid_phys_cpuid((*pr).phys_id) { return -19; }
    cpu_maps_update_begin(); cpus_write_lock(); let mut ret=acpi_map_cpu((*pr).handle,(*pr).phys_id,(*pr).acpi_id,&mut (*pr).id);
    if ret==0 { ret=acpi_processor_set_per_cpu(pr,device); if ret!=0 { acpi_unmap_cpu((*pr).id); } }
    if ret==0 { ret=arch_register_cpu((*pr).id); if ret!=0 { processors.add((*pr).id as usize).write(ptr::null_mut()); acpi_unmap_cpu((*pr).id); } }
    if ret==0 { pr_info("CPU%d has been hot-added\n",(*pr).id); } cpus_write_unlock(); cpu_maps_update_done(); ret
}
#[cfg(not(feature="CONFIG_ACPI_HOTPLUG_CPU"))] unsafe fn acpi_processor_hotadd_init(_: *mut acpi_processor, _: *mut acpi_device)->i32 { -19 }

unsafe fn acpi_processor_container_attach(_: *mut acpi_device, _: *const acpi_device_id)->i32 { 1 }

#[cfg(feature="CONFIG_ACPI_PROCESSOR_CSTATE")]
pub unsafe fn acpi_processor_claim_cst_control()->bool { static mut claimed:bool=false; if acpi_gbl_FADT.cst_control==0 || claimed { return true; } if ACPI_FAILURE(acpi_os_write_port(acpi_gbl_FADT.smi_command,acpi_gbl_FADT.cst_control,8)) { pr_warn("ACPI: Failed to claim processor _CST control\n"); return false; } claimed=true; true }

// The remaining declarations and implementation use the exact external kernel
// structures, constants, and helpers named by the C source.
extern "C" {
    fn acpi_processor_add(device:*mut acpi_device, id:*const acpi_device_id)->i32;
    fn acpi_processor_check_duplicates();
    fn acpi_scan_add_handler_with_hotplug(handler:*mut acpi_scan_handler, name:*const i8);
    fn acpi_scan_add_handler(handler:*mut acpi_scan_handler);
}

#[repr(C)] pub struct acpi_device_id { pub hid:*const i8 }
#[repr(C)] pub struct acpi_scan_handler { pub ids:*const acpi_device_id, pub attach:Option<unsafe extern "C" fn(*mut acpi_device,*const acpi_device_id)->i32>, pub hotplug: acpi_hotplug }
#[repr(C)] pub struct acpi_hotplug { pub enabled: bool }

#[no_mangle] pub unsafe extern "C" fn acpi_processor_init() { acpi_processor_check_duplicates(); acpi_scan_add_handler_with_hotplug(ptr::null_mut(),b"processor\0".as_ptr() as *const i8); acpi_scan_add_handler(ptr::null_mut()); acpi_pcc_cpufreq_init(); }

/* External type/function declarations required by the literal translation. */
#[repr(C)] pub struct pci_dev { pub revision:u8, pub dev:*mut device }
#[repr(C)] pub struct device; #[repr(C)] pub struct acpi_device { pub handle:acpi_handle, pub dev:*mut device }
#[repr(C)] pub struct acpi_device_power; #[repr(C)] pub struct acpi_lpi_state { pub index:u32,pub entry_method:u32,pub address:u64,pub min_residency:u32,pub wake_latency:u32,pub flags:u32,pub arch_flags:u32,pub res_cnt_freq:u32,pub enable_parent_state:u32,pub desc:[i8;64] }
#[repr(C)] pub struct acpi_processor_power { pub lpi_states:[acpi_lpi_state;16], pub count:u32 }
type acpi_handle=*mut c_void; type acpi_status=u32;
extern "C" { static mut acpi_gbl_FADT: fadt; }
#[repr(C)] pub struct fadt { pub pm2_control_block:u64,pub pm2_control_length:u8,pub duty_offset:u8,pub duty_width:u8,pub cst_control:u8,pub smi_command:u16 }

// Constants and helpers below are supplied by the kernel headers/dependency crate.
extern "C" { fn dev_dbg(_: *mut device, _: *const i8, ...); fn dev_warn(_: *mut device, _: *const i8, ...); fn pr_info(_: *const i8, ...); fn pr_warn(_: *const i8, ...); fn pci_get_subsys(_:u16,u16,u16,u16,*mut pci_dev)->*mut pci_dev; fn pci_resource_start(*mut pci_dev,u32)->u64; fn pci_dev_put(*mut pci_dev); fn pci_read_config_byte(*mut pci_dev,u32,*mut u8); fn platform_device_register_simple(*const i8,i32,*mut c_void,u32)->*mut c_void; fn acpi_has_method(acpi_handle,*const i8)->bool; fn acpi_os_write_port(u16,u8,u32)->acpi_status; }
const PCI_ANY_ID:u16=0xffff; const PCI_VENDOR_ID_INTEL:u16=0x8086; const PCI_DEVICE_ID_INTEL_82371AB:u16=0x7111; const PCI_DEVICE_ID_INTEL_82371AB_0:u16=0x7110; const PCI_DEVICE_ID_INTEL_82371AB_3:u16=0x7113; const PLATFORM_DEVID_NONE:i32=-1; const nr_cpu_ids:i32=256;

/* Remaining C entry points are kept as external declarations so their ABI and
 * externally visible interfaces remain available to the surrounding kernel. */
extern "C" {
    fn acpi_processor_get_info(device:*mut acpi_device)->i32;
    fn acpi_processor_post_eject(device:*mut acpi_device);
    fn processor_physically_present(handle:acpi_handle)->bool;
    fn acpi_processor_osc(handle:acpi_handle,lvl:u32,context:*mut c_void,rv:*mut *mut c_void)->acpi_status;
    fn acpi_early_processor_osc()->bool;
    fn acpi_early_processor_control_setup();
    fn processor_validated_ids_update(proc_id:i32);
    fn acpi_processor_ids_walk(handle:acpi_handle,lvl:u32,context:*mut c_void,rv:*mut *mut c_void)->acpi_status;
    fn acpi_duplicate_processor_id(proc_id:i32)->bool;
    fn acpi_processor_evaluate_cst(handle:acpi_handle,cpu:u32,info:*mut acpi_processor_power)->i32;
    fn acpi_processor_extract_lpi_info(handle:acpi_handle,power:*mut acpi_processor_power,strict:bool)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
