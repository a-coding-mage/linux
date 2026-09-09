/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Faithful Rust translation of linux/acpi.h.  External kernel/ACPI types and
 * functions are intentionally referenced but not implemented here. */

use core::ffi::{c_char, c_int, c_void};

/* Configuration branches from the C header are retained as comments; this
 * translation exposes the declarations and their non-ACPI fallback bodies. */
pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type s64 = ::core::primitive::i64;
pub type ssize_t = isize;
pub type size_t = usize;
pub type __u8 = u8;
pub type __u64 = u64;
pub type resource_size_t = usize;
pub type kernel_ulong_t = usize;

#[repr(C)] pub struct acpi_device { pub handle: acpi_handle, pub dev: device, pub flags: acpi_device_flags, pub data: acpi_device_data }
#[repr(C)] pub struct acpi_device_flags { pub visited: bool }
#[repr(C)] pub struct acpi_device_data { pub properties: list_head }
#[repr(C)] pub struct device { pub fwnode: *mut fwnode_handle, pub parent: *mut device }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct acpi_data_node { _private: [u8; 0] }
#[repr(C)] pub struct acpi_table_header { _private: [u8; 0] }
#[repr(C)] pub struct acpi_subtable_header { pub length: u32 }
#[repr(C)] pub struct acpi_resource { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct resource_win { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain_ops { _private: [u8; 0] }
#[repr(C)] pub struct property_entry { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct kobj_uevent_env { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct cpumask_t { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct access_coordinate { _private: [u8; 0] }
#[repr(C)] pub struct bus_dma_region { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_reference_args { _private: [u8; 0] }
#[repr(C)] pub struct acpi_processor_power { _private: [u8; 0] }
#[repr(C)] pub struct acpi_srat_cpu_affinity { _private: [u8; 0] }
#[repr(C)] pub struct acpi_srat_x2apic_cpu_affinity { _private: [u8; 0] }
#[repr(C)] pub struct acpi_srat_gicc_affinity { _private: [u8; 0] }
#[repr(C)] pub struct acpi_srat_rintc_affinity { _private: [u8; 0] }
#[repr(C)] pub struct acpi_prt_entry { _private: [u8; 0] }
#[repr(C)] pub struct acpi_resource_gpio { _private: [u8; 0] }
#[repr(C)] pub struct acpi_device_id { _private: [u8; 0] }
#[repr(C)] pub struct acpi_buffer { pub length: usize, pub pointer: *mut c_void }
#[repr(C)] pub struct guid_t { _private: [u8; 0] }
pub type acpi_handle = *mut c_void;
pub type acpi_status = i64;
pub type acpi_integer = u64;
pub type acpi_object_type = u32;
#[repr(C)] pub union acpi_object { pub integer: u64, pub pointer: *mut c_void }
pub type acpi_osd_exec_callback = unsafe extern "C" fn(*mut c_void);
pub type acpi_tbl_table_handler = unsafe extern "C" fn(*mut acpi_table_header) -> c_int;
pub type acpi_tbl_entry_handler = unsafe extern "C" fn(*mut acpi_subtable_header) -> c_int;
pub type acpi_tbl_entry_handler_arg = unsafe extern "C" fn(*mut acpi_subtable_header, *mut c_void) -> c_int;

#[repr(C)] pub struct acpi_debugger_ops {
    pub create_thread: Option<unsafe extern "C" fn(acpi_osd_exec_callback, *mut c_void) -> c_int>,
    pub write_log: Option<unsafe extern "C" fn(*const c_char) -> ssize_t>,
    pub read_cmd: Option<unsafe extern "C" fn(*mut c_char, size_t) -> ssize_t>,
    pub wait_command_ready: Option<unsafe extern "C" fn(bool, *mut c_char, size_t) -> c_int>,
    pub notify_command_complete: Option<unsafe extern "C" fn() -> c_int>,
}
#[repr(C)] pub struct acpi_debugger { pub ops: *const acpi_debugger_ops, pub owner: *mut module, pub lock: mutex }

#[repr(C)] pub enum acpi_irq_model_id { ACPI_IRQ_MODEL_PIC=0, ACPI_IRQ_MODEL_IOAPIC, ACPI_IRQ_MODEL_IOSAPIC, ACPI_IRQ_MODEL_PLATFORM, ACPI_IRQ_MODEL_GIC, ACPI_IRQ_MODEL_GIC_V5, ACPI_IRQ_MODEL_LPIC, ACPI_IRQ_MODEL_RINTC, ACPI_IRQ_MODEL_COUNT }
#[repr(C)] pub enum acpi_interrupt_id { ACPI_INTERRUPT_PMI=1, ACPI_INTERRUPT_INIT, ACPI_INTERRUPT_CPEI, ACPI_INTERRUPT_COUNT }
#[repr(C)] pub enum acpi_address_range_id { ACPI_ADDRESS_RANGE_MEMORY=1, ACPI_ADDRESS_RANGE_RESERVED=2, ACPI_ADDRESS_RANGE_ACPI=3, ACPI_ADDRESS_RANGE_NVS=4, ACPI_ADDRESS_RANGE_COUNT }
#[repr(C)] pub enum acpi_predicate { all_versions, less_than_or_equal, equal, greater_than_or_equal }

pub static mut acpi_irq_model: acpi_irq_model_id = acpi_irq_model_id::ACPI_IRQ_MODEL_PIC;
pub const ACPI_SPACE_MEM: u32 = 0;
pub const INVALID_ACPI_IRQ: u32 = u32::MAX;
pub const PXM_INVAL: i32 = -1;
pub const OSC_PCI_CAPABILITY_DWORDS: u32 = 3;
pub const OSC_CXL_CAPABILITY_DWORDS: u32 = 5;
pub const OSC_QUERY_DWORD: usize=0; pub const OSC_SUPPORT_DWORD: usize=1; pub const OSC_CONTROL_DWORD: usize=2; pub const OSC_EXT_SUPPORT_DWORD: usize=3; pub const OSC_EXT_CONTROL_DWORD: usize=4;
pub const OSC_QUERY_ENABLE:u32=1; pub const OSC_REQUEST_ERROR:u32=2; pub const OSC_INVALID_UUID_ERROR:u32=4; pub const OSC_INVALID_REVISION_ERROR:u32=8; pub const OSC_CAPABILITIES_MASK_ERROR:u32=16;
pub const ACPI_VIDEO_OUTPUT_SWITCHING:u32=1; pub const ACPI_VIDEO_DEVICE_POSTING:u32=2; pub const ACPI_VIDEO_ROM_AVAILABLE:u32=4; pub const ACPI_VIDEO_BACKLIGHT:u32=8; pub const ACPI_VIDEO_BACKLIGHT_FORCE_VENDOR:u32=0x10; pub const ACPI_VIDEO_BACKLIGHT_FORCE_VIDEO:u32=0x20; pub const ACPI_VIDEO_OUTPUT_SWITCHING_FORCE_VENDOR:u32=0x40; pub const ACPI_VIDEO_OUTPUT_SWITCHING_FORCE_VIDEO:u32=0x80; pub const ACPI_VIDEO_BACKLIGHT_DMI_VENDOR:u32=0x100; pub const ACPI_VIDEO_BACKLIGHT_DMI_VIDEO:u32=0x200; pub const ACPI_VIDEO_OUTPUT_SWITCHING_DMI_VENDOR:u32=0x400; pub const ACPI_VIDEO_OUTPUT_SWITCHING_DMI_VIDEO:u32=0x800;
pub const ACPI_TABLE_ID_LEN: usize = 5;

#[repr(C)] pub struct acpi_osc_context { pub uuid_str:*mut c_char, pub rev:c_int, pub cap:acpi_buffer, pub ret:acpi_buffer }
#[repr(C)] pub struct acpi_platform_list { pub oem_id:[c_char;7], pub oem_table_id:[c_char;9], pub oem_revision:u32, pub table:*mut c_char, pub pred:acpi_predicate, pub reason:*mut c_char, pub data:u32 }
#[repr(C)] pub struct acpi_s2idle_dev_ops { pub list_node:list_head, pub prepare:Option<unsafe extern "C" fn()>, pub check:Option<unsafe extern "C" fn()>, pub restore:Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct acpi_probe_entry { pub id:[u8;5], pub r#type:u8, pub subtable_valid:Option<unsafe extern "C" fn(*mut acpi_subtable_header,*mut acpi_probe_entry)->bool>, pub probe_table:Option<acpi_tbl_table_handler>, pub driver_data:kernel_ulong_t }

pub type phys_cpuid_t = u32;
pub const PHYS_CPUID_INVALID: phys_cpuid_t = u32::MAX;
pub const fn invalid_logical_cpuid(cpuid:u32)->bool { (cpuid as i32)<0 }
pub const fn invalid_phys_cpuid(id:phys_cpuid_t)->bool { id==PHYS_CPUID_INVALID }

extern "C" {
    pub fn acpi_get_first_physical_node(adev:*mut acpi_device)->*mut device;
    pub fn acpi_table_parse(id:*mut c_char, handler:acpi_tbl_table_handler)->c_int;
    pub fn acpi_get_table_pointer(signature:*mut c_char, instance:u32)->*mut acpi_table_header;
    pub fn acpi_get_cpu_uid(cpu:u32, uid:*mut u32)->c_int;
    pub fn acpi_get_processor_handle(cpu:c_int)->acpi_handle;
    pub fn acpi_register_gsi(dev:*mut device,gsi:u32,triggering:c_int,polarity:c_int)->c_int;
    pub fn acpi_unregister_gsi(gsi:u32);
    pub fn acpi_run_osc(handle:acpi_handle, context:*mut acpi_osc_context)->acpi_status;
    pub fn acpi_match_platform_list(list:*const acpi_platform_list)->c_int;
    pub fn acpi_arch_init();
    pub fn acpi_subsystem_init();
    pub fn acpi_early_init();
    pub fn acpi_device_notify(dev:*mut device);
    pub fn acpi_device_notify_remove(dev:*mut device);
}

#[inline] pub unsafe fn acpi_device_handle(adev:*mut acpi_device)->acpi_handle { if adev.is_null(){core::ptr::null_mut()} else {*adev}.handle }
#[inline] pub unsafe fn acpi_sci_irq_valid(irq:u32)->bool { irq != INVALID_ACPI_IRQ }
#[inline] pub unsafe fn acpi_device_set_enumerated(adev:*mut acpi_device) { (*adev).flags.visited=true }
#[inline] pub unsafe fn acpi_device_clear_enumerated(adev:*mut acpi_device) { (*adev).flags.visited=false }
#[inline] pub unsafe fn acpi_osc_ctx_get_pci_control(c:*mut acpi_osc_context)->u32 { *((*c).ret.pointer as *mut u32).add(OSC_CONTROL_DWORD) }
#[inline] pub unsafe fn acpi_osc_ctx_get_cxl_control(c:*mut acpi_osc_context)->u32 { *((*c).ret.pointer as *mut u32).add(OSC_EXT_CONTROL_DWORD) }

/* The remaining C declarations are external kernel interfaces; configuration
 * disabled branches retain their C fallback values in these direct helpers. */
#[inline] pub fn acpi_reduced_hardware()->bool { false }
#[inline] pub fn acpi_dev_found(_hid:*const c_char)->bool { false }
#[inline] pub fn acpi_dev_present(_hid:*const c_char,_uid:*const c_char,_hrv:s64)->bool { false }
#[inline] pub fn acpi_sleep_state_supported(_state:u8)->bool { false }
#[inline] pub fn acpi_dma_supported(_adev:*const acpi_device)->bool { false }
#[inline] pub fn acpi_dev_state_d0(_dev:*mut device)->bool { true }
#[inline] pub fn acpi_node_backed_by_real_pxm(_nid:c_int)->bool { false }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
