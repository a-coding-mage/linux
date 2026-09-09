/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from acpi/internal.h. */

// Dependency-provided C types and constants are intentionally referenced here.

unsafe extern "C" {
    pub static mut acpi_root: *mut acpi_device;

    pub fn early_acpi_osi_init() -> ::core::ffi::c_int;
    pub fn acpi_osi_init() -> ::core::ffi::c_int;
    pub fn acpi_os_initialize1() -> acpi_status;
    pub fn acpi_scan_init();
    #[cfg(feature = "CONFIG_PCI")]
    pub fn acpi_pci_root_init();
    #[cfg(feature = "CONFIG_PCI")]
    pub fn acpi_pci_link_init();
    pub fn acpi_processor_init();
    pub fn acpi_platform_init();
    pub fn acpi_pnp_init();
    pub fn acpi_sysfs_init() -> ::core::ffi::c_int;
    pub fn acpi_gpe_apply_masked_gpes();
    pub fn acpi_container_init();
    pub fn acpi_memory_hotplug_init();
    #[cfg(feature = "CONFIG_ACPI_HOTPLUG_IOAPIC")]
    pub fn pci_ioapic_remove(root: *mut acpi_pci_root);
    #[cfg(feature = "CONFIG_ACPI_HOTPLUG_IOAPIC")]
    pub fn acpi_ioapic_remove(root: *mut acpi_pci_root) -> ::core::ffi::c_int;
    #[cfg(feature = "CONFIG_ACPI_DOCK")]
    pub fn register_dock_dependent_device(adev: *mut acpi_device, dshandle: acpi_handle);
    #[cfg(feature = "CONFIG_ACPI_DOCK")]
    pub fn dock_notify(adev: *mut acpi_device, event: u32) -> ::core::ffi::c_int;
    #[cfg(feature = "CONFIG_ACPI_DOCK")]
    pub fn acpi_dock_add(adev: *mut acpi_device);
    #[cfg(feature = "CONFIG_X86")]
    pub fn acpi_cmos_rtc_init();
    pub fn acpi_rev_override_setup(str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;

    pub fn acpi_sysfs_add_hotplug_profile(hotplug: *mut acpi_hotplug_profile, name: *const ::core::ffi::c_char);
    pub fn acpi_scan_add_handler_with_hotplug(handler: *mut acpi_scan_handler, hotplug_profile_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn acpi_scan_hotplug_enabled(hotplug: *mut acpi_hotplug_profile, val: bool);
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub static mut acpi_debugfs_dir: *mut dentry;
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub fn acpi_debugfs_init();
    pub fn acpi_lpss_init();
    pub fn acpi_apd_init();
    pub fn acpi_hotplug_schedule(adev: *mut acpi_device, src: u32) -> acpi_status;
    pub fn acpi_queue_hotplug_work(work: *mut work_struct) -> bool;
    pub fn acpi_device_hotplug(adev: *mut acpi_device, src: u32);
    pub fn acpi_scan_is_offline(adev: *mut acpi_device, uevent: bool) -> bool;
    pub fn acpi_sysfs_table_handler(event: u32, table: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> acpi_status;
    pub fn acpi_scan_table_notify();
    pub fn acpi_active_trip_temp(adev: *mut acpi_device, id: ::core::ffi::c_int, ret_temp: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_passive_trip_temp(adev: *mut acpi_device, ret_temp: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_hot_trip_temp(adev: *mut acpi_device, ret_temp: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_critical_trip_temp(adev: *mut acpi_device, ret_temp: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_init_device_object(device: *mut acpi_device, handle: acpi_handle, type_: ::core::ffi::c_int, release: Option<unsafe extern "C" fn(*mut device)>);
    pub fn acpi_tie_acpi_dev(adev: *mut acpi_device) -> ::core::ffi::c_int;
    pub fn acpi_device_add(device: *mut acpi_device) -> ::core::ffi::c_int;
    pub fn acpi_device_setup_files(dev: *mut acpi_device);
    pub fn acpi_device_remove_files(dev: *mut acpi_device);
    pub static mut acpi_groups: *const *const attribute_group;
    pub fn acpi_device_add_finalize(device: *mut acpi_device);
    pub fn acpi_free_pnp_ids(pnp: *mut acpi_device_pnp);
    pub fn acpi_device_is_enabled(adev: *const acpi_device) -> bool;
    pub fn acpi_device_is_present(adev: *const acpi_device) -> bool;
    pub fn acpi_device_is_battery(adev: *mut acpi_device) -> bool;
    pub fn acpi_device_is_first_physical_node(adev: *mut acpi_device, dev: *const device) -> bool;
    pub fn acpi_bus_register_early_device(type_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_companion_match(dev: *const device) -> *const acpi_device;
    pub fn __acpi_device_uevent_modalias(adev: *const acpi_device, env: *mut kobj_uevent_env) -> ::core::ffi::c_int;
    pub fn acpi_power_resources_init();
    pub fn acpi_power_resources_list_free(list: *mut list_head);
    pub fn acpi_extract_power_resources(package: *mut acpi_object, start: ::core::ffi::c_uint, list: *mut list_head) -> ::core::ffi::c_int;
    pub fn acpi_add_power_resource(handle: acpi_handle) -> *mut acpi_device;
    pub fn acpi_power_add_remove_device(adev: *mut acpi_device, add: bool);
    pub fn acpi_power_wakeup_list_init(list: *mut list_head, system_level: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_device_sleep_wake(dev: *mut acpi_device, enable: ::core::ffi::c_int, sleep_state: ::core::ffi::c_int, dev_state: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_power_get_inferred_state(device: *mut acpi_device, state: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_power_on_resources(device: *mut acpi_device, state: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_power_transition(device: *mut acpi_device, state: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_turn_off_unused_power_resources();
    pub fn acpi_device_get_power(device: *mut acpi_device, state: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn acpi_wakeup_device_init() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline] pub unsafe fn acpi_pci_root_init() {}
#[cfg(not(feature = "CONFIG_PCI"))]
#[inline] pub unsafe fn acpi_pci_link_init() {}
#[cfg(not(feature = "CONFIG_ACPI_HOTPLUG_IOAPIC"))]
#[inline] pub unsafe fn pci_ioapic_remove(_: *mut acpi_pci_root) {}
#[cfg(not(feature = "CONFIG_ACPI_HOTPLUG_IOAPIC"))]
#[inline] pub unsafe fn acpi_ioapic_remove(_: *mut acpi_pci_root) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_X86"))]
#[inline] pub unsafe fn acpi_cmos_rtc_init() {}

pub const ACPI_STA_DEFAULT: u32 = ACPI_STA_DEVICE_PRESENT | ACPI_STA_DEVICE_ENABLED | ACPI_STA_DEVICE_UI | ACPI_STA_DEVICE_FUNCTIONING;

#[repr(C)]
pub struct acpi_device_bus_id { pub bus_id: *const ::core::ffi::c_char, pub instance_ida: ida, pub node: list_head }

#[repr(C)]
pub enum acpi_ec_event_state { EC_EVENT_READY = 0, EC_EVENT_IN_PROGRESS, EC_EVENT_COMPLETE }

#[repr(C)]
pub struct acpi_ec {
    pub handle: acpi_handle, pub gpe: ::core::ffi::c_int, pub irq: ::core::ffi::c_int,
    pub command_addr: usize, pub data_addr: usize, pub global_lock: bool, pub flags: usize,
    pub reference_count: usize, pub mutex: mutex, pub wait: wait_queue_head_t, pub list: list_head,
    pub curr: *mut transaction, pub lock: spinlock_t, pub work: work_struct, pub timestamp: usize,
    pub event_state: acpi_ec_event_state, pub events_to_process: u32, pub events_in_progress: u32,
    pub queries_in_progress: u32, pub busy_polling: bool, pub polling_guard: u32,
}

pub type acpi_ec_query_func = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;

unsafe extern "C" { pub static mut first_ec: *mut acpi_ec; }

#[cfg(not(feature = "CONFIG_ARM64"))]
#[inline] pub unsafe fn acpi_arch_thermal_cpufreq_pctg() -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_X86"))]
#[inline] pub unsafe fn force_storage_d3() -> bool { false }

pub const ACPI_DT_NAMESPACE_HID: &[u8] = b"PRP0001\0";

unsafe extern "C" {
    pub fn acpi_init_properties(adev: *mut acpi_device);
    pub fn acpi_free_properties(adev: *mut acpi_device);
    pub fn acpi_watchdog_init();
    pub fn acpi_init_lpit();
    pub fn acpi_mipi_check_crs_csi2(handle: acpi_handle);
    pub fn acpi_mipi_scan_crs_csi2();
    pub fn acpi_mipi_init_crs_csi2_swnodes();
    pub fn acpi_mipi_crs_csi2_cleanup();
    pub fn acpi_ec_init();
    pub fn acpi_ec_ecdt_probe();
    pub fn acpi_ec_dsdt_probe();
    pub fn acpi_ec_block_transactions();
    pub fn acpi_ec_unblock_transactions();
    pub fn acpi_ec_add_query_handler(ec: *mut acpi_ec, query_bit: u8, handle: acpi_handle, func: acpi_ec_query_func, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn acpi_ec_remove_query_handler(ec: *mut acpi_ec, query_bit: u8);
    pub fn acpi_ec_register_opregions(adev: *mut acpi_device);
    pub fn acpi_ec_flush_work();
    pub fn acpi_ec_dispatch_gpe() -> bool;
    pub fn acpi_sleep_init() -> ::core::ffi::c_int;
    pub fn acpi_sleep_proc_init();
    pub fn suspend_nvs_alloc() -> ::core::ffi::c_int;
    pub fn suspend_nvs_free();
    pub fn suspend_nvs_save() -> ::core::ffi::c_int;
    pub fn suspend_nvs_restore();
    pub fn acpi_extract_apple_properties(adev: *mut acpi_device);
}

#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_init() {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_ecdt_probe() {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_dsdt_probe() {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_block_transactions() {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_unblock_transactions() {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_add_query_handler(_: *mut acpi_ec, _: u8, _: acpi_handle, _: acpi_ec_query_func, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { -ENXIO }
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_remove_query_handler(_: *mut acpi_ec, _: u8) {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_register_opregions(_: *mut acpi_device) {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_flush_work() {}
#[cfg(not(feature = "CONFIG_ACPI_EC"))]
#[inline] pub unsafe fn acpi_ec_dispatch_gpe() -> bool { false }
#[cfg(not(feature = "CONFIG_ACPI_SYSTEM_POWER_STATES_SUPPORT"))]
#[inline] pub unsafe fn acpi_s2idle_wakeup() -> bool { false }
#[cfg(not(feature = "CONFIG_ACPI_SYSTEM_POWER_STATES_SUPPORT"))]
#[inline] pub unsafe fn acpi_sleep_init() -> ::core::ffi::c_int { -ENXIO }
#[cfg(not(feature = "CONFIG_ACPI_SLEEP"))]
#[inline] pub unsafe fn acpi_sleep_proc_init() {}
#[cfg(not(feature = "CONFIG_ACPI_SLEEP"))]
#[inline] pub unsafe fn suspend_nvs_alloc() -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_ACPI_SLEEP"))]
#[inline] pub unsafe fn suspend_nvs_free() {}
#[cfg(not(feature = "CONFIG_ACPI_SLEEP"))]
#[inline] pub unsafe fn suspend_nvs_save() -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_ACPI_SLEEP"))]
#[inline] pub unsafe fn suspend_nvs_restore() {}
#[cfg(not(feature = "CONFIG_ACPI_WATCHDOG"))]
#[inline] pub unsafe fn acpi_watchdog_init() {}
#[cfg(not(feature = "CONFIG_ACPI_LPIT"))]
#[inline] pub unsafe fn acpi_init_lpit() {}
#[cfg(not(feature = "CONFIG_X86"))]
#[inline] pub unsafe fn acpi_extract_apple_properties(_: *mut acpi_device) {}
#[cfg(not(feature = "CONFIG_X86"))]
#[inline] pub unsafe fn acpi_graph_ignore_port(_: acpi_handle) -> bool { false }

#[cfg(not(feature = "CONFIG_ARCH_MIGHT_HAVE_ACPI_PDC"))]
#[inline] pub unsafe fn acpi_early_processor_control_setup() {}
#[cfg(not(feature = "CONFIG_ACPI_PROCESSOR_CSTATE"))]
#[inline] pub unsafe fn acpi_idle_rescan_dead_smt_siblings() {}
#[cfg(not(feature = "CONFIG_X86"))]
#[inline] pub unsafe fn acpi_proc_quirk_mwait_check() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
