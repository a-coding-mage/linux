/* SPDX-License-Identifier: MIT */
/* Hardware platform operations. Intended for use by domain-0 kernel. */

// C dependency: xen/interface/xen.h supplies xen_pfn_t and xen_ulong_t.

pub const XENPF_INTERFACE_VERSION: u32 = 0x03000001;
pub const XENPF_settime32: u32 = 17;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_settime32 { pub secs: u32, pub nsecs: u32, pub system_time: u64 }
pub type xenpf_settime32_t = *mut xenpf_settime32;
pub const XENPF_settime64: u32 = 62;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_settime64 { pub secs: u64, pub nsecs: u32, pub mbz: u32, pub system_time: u64 }
pub type xenpf_settime64_t = *mut xenpf_settime64;

pub const XENPF_add_memtype: u32 = 31;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_add_memtype { pub mfn: xen_pfn_t, pub nr_mfns: u64, pub type_: u32, pub handle: u32, pub reg: u32 }
pub type xenpf_add_memtype_t = *mut xenpf_add_memtype;
pub const XENPF_del_memtype: u32 = 32;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_del_memtype { pub handle: u32, pub reg: u32 }
pub type xenpf_del_memtype_t = *mut xenpf_del_memtype;
pub const XENPF_read_memtype: u32 = 33;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_read_memtype { pub reg: u32, pub mfn: xen_pfn_t, pub nr_mfns: u64, pub type_: u32 }
pub type xenpf_read_memtype_t = *mut xenpf_read_memtype;

pub const XENPF_microcode_update: u32 = 35;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_microcode_update { pub data: *mut core::ffi::c_void, pub length: u32 }
pub type xenpf_microcode_update_t = *mut xenpf_microcode_update;
pub const XENPF_platform_quirk: u32 = 39;
pub const QUIRK_NOIRQBALANCING: u32 = 1; pub const QUIRK_IOAPIC_BAD_REGSEL: u32 = 2; pub const QUIRK_IOAPIC_GOOD_REGSEL: u32 = 3;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_platform_quirk { pub quirk_id: u32 }
pub type xenpf_platform_quirk_t = *mut xenpf_platform_quirk;

pub const XENPF_efi_runtime_call: u32 = 49;
pub const XEN_EFI_get_time: u32 = 1; pub const XEN_EFI_set_time: u32 = 2; pub const XEN_EFI_get_wakeup_time: u32 = 3; pub const XEN_EFI_set_wakeup_time: u32 = 4; pub const XEN_EFI_get_next_high_monotonic_count: u32 = 5; pub const XEN_EFI_get_variable: u32 = 6; pub const XEN_EFI_set_variable: u32 = 7; pub const XEN_EFI_get_next_variable_name: u32 = 8; pub const XEN_EFI_query_variable_info: u32 = 9; pub const XEN_EFI_query_capsule_capabilities: u32 = 10; pub const XEN_EFI_update_capsule: u32 = 11;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_time { pub year:u16,pub month:u8,pub day:u8,pub hour:u8,pub min:u8,pub sec:u8,pub ns:u32,pub tz:i16,pub daylight:u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_guid { pub data1:u32,pub data2:u16,pub data3:u16,pub data4:[u8;8] }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_get_time { pub time:xenpf_efi_time,pub resolution:u32,pub accuracy:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_variable { pub name:*mut core::ffi::c_void,pub size:xen_ulong_t,pub data:*mut core::ffi::c_void,pub vendor_guid:xenpf_efi_guid }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_next_variable { pub size:xen_ulong_t,pub name:*mut core::ffi::c_void,pub vendor_guid:xenpf_efi_guid }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_query_variable_info { pub attr:u32,pub max_store_size:u64,pub remain_store_size:u64,pub max_size:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_capsule { pub capsule_header_array:*mut core::ffi::c_void,pub capsule_count:xen_ulong_t,pub max_capsule_size:u64,pub reset_type:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_update_capsule { pub capsule_header_array:*mut core::ffi::c_void,pub capsule_count:xen_ulong_t,pub sg_list:u64 }
#[repr(C)] pub union xenpf_efi_runtime_union { pub get_time:xenpf_efi_get_time,pub set_time:xenpf_efi_time,pub get_wakeup_time:xenpf_efi_time,pub set_wakeup_time:xenpf_efi_time,pub variable:xenpf_efi_variable,pub next_variable:xenpf_efi_next_variable,pub query_variable_info:xenpf_efi_query_variable_info,pub capsule:xenpf_efi_capsule,pub update_capsule:xenpf_efi_update_capsule }
pub const XEN_EFI_GET_TIME_SET_CLEARS_NS:u32=1; pub const XEN_EFI_GET_WAKEUP_TIME_ENABLED:u32=1; pub const XEN_EFI_GET_WAKEUP_TIME_PENDING:u32=2; pub const XEN_EFI_SET_WAKEUP_TIME_ENABLE:u32=1; pub const XEN_EFI_SET_WAKEUP_TIME_ENABLE_ONLY:u32=2; pub const XEN_EFI_VARIABLE_NON_VOLATILE:u32=1; pub const XEN_EFI_VARIABLE_BOOTSERVICE_ACCESS:u32=2; pub const XEN_EFI_VARIABLE_RUNTIME_ACCESS:u32=4;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_efi_runtime_call { pub function:u32,pub misc:u32,pub status:xen_ulong_t,pub u:xenpf_efi_runtime_union }
pub type xenpf_efi_runtime_call_t=*mut xenpf_efi_runtime_call;

pub const XEN_FW_EFI_VERSION:u32=0; pub const XEN_FW_EFI_CONFIG_TABLE:u32=1; pub const XEN_FW_EFI_VENDOR:u32=2; pub const XEN_FW_EFI_MEM_INFO:u32=3; pub const XEN_FW_EFI_RT_VERSION:u32=4;
pub const XENPF_firmware_info:u32=50; pub const XEN_FW_DISK_INFO:u32=1; pub const XEN_FW_DISK_MBR_SIGNATURE:u32=2; pub const XEN_FW_VBEDDC_INFO:u32=3; pub const XEN_FW_EFI_INFO:u32=4; pub const XEN_FW_KBD_SHIFT_FLAGS:u32=5;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_firmware_info { pub type_:u32,pub index:u32,pub u:[u8;32] }
pub type xenpf_firmware_info_t=*mut xenpf_firmware_info;

pub const XENPF_enter_acpi_sleep:u32=51; pub const XENPF_ACPI_SLEEP_EXTENDED:u32=1;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_enter_acpi_sleep { pub val_a:u16,pub val_b:u16,pub sleep_state:u32,pub flags:u32 }
pub type xenpf_enter_acpi_sleep_t=*mut xenpf_enter_acpi_sleep;
pub const XENPF_change_freq:u32=52;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_change_freq { pub flags:u32,pub cpu:u32,pub freq:u64 }
pub type xenpf_change_freq_t=*mut xenpf_change_freq;
pub const XENPF_getidletime:u32=53;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_getidletime { pub cpumap_bitmap:*mut u8,pub cpumap_nr_cpus:u32,pub idletime:*mut u64,pub now:u64 }
pub type xenpf_getidletime_t=*mut xenpf_getidletime;

pub const XENPF_set_processor_pminfo:u32=54; pub const XEN_PROCESSOR_PM_CX:u32=1; pub const XEN_PROCESSOR_PM_PX:u32=2; pub const XEN_PROCESSOR_PM_TX:u32=4; pub const XEN_PM_CX:u32=0; pub const XEN_PM_PX:u32=1; pub const XEN_PM_TX:u32=2; pub const XEN_PM_PDC:u32=3; pub const XEN_PX_PCT:u32=1; pub const XEN_PX_PSS:u32=2; pub const XEN_PX_PPC:u32=4; pub const XEN_PX_PSD:u32=8;
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_power_register { pub space_id:u32,pub bit_width:u32,pub bit_offset:u32,pub access_size:u32,pub address:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_processor_csd { pub domain:u32,pub coord_type:u32,pub num:u32 }
pub type xen_processor_csd_t=*mut xen_processor_csd;
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_processor_cx { pub reg:xen_power_register,pub type_:u8,pub latency:u32,pub power:u32,pub dpcnt:u32,pub dp:xen_processor_csd_t }
pub type xen_processor_cx_t=*mut xen_processor_cx;
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_processor_flags { pub bits:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_processor_power { pub count:u32,pub flags:xen_processor_flags,pub states:xen_processor_cx_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_pct_register { pub descriptor:u8,pub length:u16,pub space_id:u8,pub bit_width:u8,pub bit_offset:u8,pub reserved:u8,pub address:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_processor_px { pub core_frequency:u64,pub power:u64,pub transition_latency:u64,pub bus_master_latency:u64,pub control:u64,pub status:u64 }
pub type xen_processor_px_t=*mut xen_processor_px;
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_psd_package { pub num_entries:u64,pub revision:u64,pub domain:u64,pub coord_type:u64,pub num_processors:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_processor_performance { pub flags:u32,pub platform_limit:u32,pub control_register:xen_pct_register,pub status_register:xen_pct_register,pub state_count:u32,pub states:xen_processor_px_t,pub domain_info:xen_psd_package,pub shared_type:u32 }
#[repr(C)] pub union xenpf_pminfo_union { pub power:xen_processor_power,pub perf:xen_processor_performance,pub pdc:*mut u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_set_processor_pminfo { pub id:u32,pub type_:u32,pub u:xenpf_pminfo_union }
pub type xenpf_set_processor_pminfo_t=*mut xenpf_set_processor_pminfo;

pub const XENPF_get_cpuinfo:u32=55; pub const XEN_PCPU_FLAGS_ONLINE:u32=1; pub const XEN_PCPU_FLAGS_INVALID:u32=2;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_pcpuinfo { pub xen_cpuid:u32,pub max_present:u32,pub flags:u32,pub apic_id:u32,pub acpi_id:u32 }
pub type xenpf_pcpuinfo_t=*mut xenpf_pcpuinfo;
pub const XENPF_cpu_online:u32=56; pub const XENPF_cpu_offline:u32=57;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_cpu_ol { pub cpuid:u32 } pub type xenpf_cpu_ol_t=*mut xenpf_cpu_ol;
pub const XENPF_cpu_hotadd:u32=58; #[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_cpu_hotadd { pub apic_id:u32,pub acpi_id:u32,pub pxm:u32 }
pub const XENPF_mem_hotadd:u32=59; #[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_mem_hotadd { pub spfn:u64,pub epfn:u64,pub pxm:u32,pub flags:u32 }
pub const XENPF_core_parking:u32=60; pub const XEN_CORE_PARKING_SET:u32=1; pub const XEN_CORE_PARKING_GET:u32=2;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_core_parking { pub type_:u32,pub idle_nums:u32 } pub type xenpf_core_parking_t=*mut xenpf_core_parking;
pub const XENPF_get_symbol:u32=63;
#[repr(C)] #[derive(Copy, Clone)] pub struct xenpf_symdata { pub namelen:u32,pub symnum:u32,pub name:*mut i8,pub address:u64,pub type_:i8 }
pub type xenpf_symdata_t=*mut xenpf_symdata;
pub const XENPF_get_dom0_console:u32=64;

// The following union includes dom0_vga_console_info from xen/interface/xen.h.
#[repr(C)] pub union xen_platform_op_union { pub settime32:xenpf_settime32,pub settime64:xenpf_settime64,pub add_memtype:xenpf_add_memtype,pub del_memtype:xenpf_del_memtype,pub read_memtype:xenpf_read_memtype,pub microcode:xenpf_microcode_update,pub platform_quirk:xenpf_platform_quirk,pub efi_runtime_call:xenpf_efi_runtime_call,pub firmware_info:xenpf_firmware_info,pub enter_acpi_sleep:xenpf_enter_acpi_sleep,pub change_freq:xenpf_change_freq,pub getidletime:xenpf_getidletime,pub set_pminfo:xenpf_set_processor_pminfo,pub pcpu_info:xenpf_pcpuinfo,pub cpu_ol:xenpf_cpu_ol,pub cpu_add:xenpf_cpu_hotadd,pub mem_add:xenpf_mem_hotadd,pub core_parking:xenpf_core_parking,pub symdata:xenpf_symdata,pub dom0_console:dom0_vga_console_info,pub pad:[u8;128] }
#[repr(C)] pub struct xen_platform_op { pub cmd:u32,pub interface_version:u32,pub u:xen_platform_op_union }
pub type xen_platform_op_t=*mut xen_platform_op;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
