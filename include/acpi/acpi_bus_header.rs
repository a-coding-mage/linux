/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Direct Rust translation of acpi_bus.h. Linux dependencies are external. */

use core::ffi::{c_char, c_int, c_void};

/* External C types supplied by the surrounding kernel translation. */
pub type AcpiHandle = *mut c_void;
pub type AcpiString = *mut c_char;
pub type AcpiStatus = u32;
pub type Guid = [u8; 16];
pub type PhysAddr = usize;

#[repr(C)] pub struct AcpiObject { pub object_type: u32, pub data: [u8; 0] }
#[repr(C)] pub struct AcpiBuffer { pub length: usize, pub pointer: *mut c_void }
#[repr(C)] pub struct AcpiObjectList { pub count: u32, pub pointer: *mut AcpiObject }
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct Kobject { pub _private: [u8; 0] }
#[repr(C)] pub struct Device { pub parent: *mut Device, pub _private: [u8; 0] }
#[repr(C)] pub struct FwnodeHandle { pub ops: *const FwnodeOperations, pub _private: [u8; 0] }
#[repr(C)] pub struct FwnodeOperations { pub _private: [u8; 0] }
#[repr(C)] pub struct Mutex { pub _private: [u8; 0] }
#[repr(C)] pub struct Completion { pub _private: [u8; 0] }
#[repr(C)] pub struct PropertyEntry { pub _private: [u8; 0] }
#[repr(C)] pub struct SoftwareNode { pub _private: [u8; 0] }
#[repr(C)] pub struct SoftwareNodeRefArgs { pub _private: [u8; 0] }
#[repr(C)] pub struct WakeupSource { pub _private: [u8; 0] }
#[repr(C)] pub struct Resource { pub _private: [u8; 0] }
#[repr(C)] pub struct PciBus { pub _private: [u8; 0] }
#[repr(C)] pub struct BusDmaRegion { pub _private: [u8; 0] }
#[repr(C)] pub struct ProcDirEntry { pub _private: [u8; 0] }
#[repr(C)] pub struct AcpiDeviceId { pub _private: [u8; 0] }
#[repr(C)] pub struct AcpiPldInfo { pub _private: [u8; 0] }
#[repr(C)] pub struct AcpiGblFadt { pub header: AcpiFadtHeader }
#[repr(C)] pub struct AcpiFadtHeader { pub revision: u8, pub _pad: [u8; 0] }
#[repr(C)] pub struct NotifierBlock { pub _private: [u8; 0] }
#[repr(C)] pub struct BusType { pub _private: [u8; 0] }
#[repr(C)] pub struct IommuOps { pub _private: [u8; 0] }
pub type AcpiNotifyHandler = Option<unsafe extern "C" fn(AcpiHandle, u32, *mut c_void)>;
pub type AcpiObjectType = u32;
pub type DevDmaAttr = u32;

pub const ACPI_TYPE_PACKAGE: u32 = 4;
pub const ACPI_STATE_D0: i32 = 0;
pub const ACPI_STATE_D3_HOT: usize = 3;
pub const ACPI_STATE_D3_COLD: usize = 4;
pub const ACPI_D_STATE_COUNT: usize = 5;
pub const AE_SUPPORT: AcpiStatus = 0x0009;
pub const ENODEV: i32 = 19;

#[repr(C)] pub struct AcpiHandleList { pub count: u32, pub handles: *mut AcpiHandle }

extern "C" {
    pub fn acpi_extract_package(package: *mut AcpiObject, format: *mut AcpiBuffer, buffer: *mut AcpiBuffer) -> AcpiStatus;
    pub fn acpi_evaluate_integer(handle: AcpiHandle, pathname: AcpiString, arguments: *mut AcpiObjectList, data: *mut u64) -> AcpiStatus;
    pub fn acpi_evaluate_reference(handle: AcpiHandle, pathname: AcpiString, arguments: *mut AcpiObjectList, list: *mut AcpiHandleList) -> bool;
    pub fn acpi_handle_list_equal(list1: *mut AcpiHandleList, list2: *mut AcpiHandleList) -> bool;
    pub fn acpi_handle_list_replace(dst: *mut AcpiHandleList, src: *mut AcpiHandleList);
    pub fn acpi_handle_list_free(list: *mut AcpiHandleList);
    pub fn acpi_device_dep(target: AcpiHandle, r#match: AcpiHandle) -> bool;
    pub fn acpi_evaluate_ost(handle: AcpiHandle, source_event: u32, status_code: u32, status_buf: *mut AcpiBuffer) -> AcpiStatus;
    pub fn acpi_has_method(handle: AcpiHandle, name: *mut c_char) -> bool;
    pub fn acpi_execute_simple_method(handle: AcpiHandle, method: *mut c_char, arg: u64) -> AcpiStatus;
    pub fn acpi_evaluate_ej0(handle: AcpiHandle) -> AcpiStatus;
    pub fn acpi_evaluate_lck(handle: AcpiHandle, lock: c_int) -> AcpiStatus;
    pub fn acpi_evaluate_reg(handle: AcpiHandle, space_id: u8, function: u32) -> AcpiStatus;
    pub fn acpi_ata_match(handle: AcpiHandle) -> bool;
    pub fn acpi_bay_match(handle: AcpiHandle) -> bool;
    pub fn acpi_dock_match(handle: AcpiHandle) -> bool;
    pub fn acpi_check_dsm(handle: AcpiHandle, guid: *const Guid, rev: u64, funcs: u64) -> bool;
    pub fn acpi_evaluate_dsm(handle: AcpiHandle, guid: *const Guid, rev: u64, func: u64, argv4: *mut AcpiObject) -> *mut AcpiObject;
    pub fn acpi_get_physical_device_location(handle: AcpiHandle, pld: *mut *mut AcpiPldInfo) -> bool;
}

#[inline] pub unsafe fn acpi_evaluate_dsm_typed(handle: AcpiHandle, guid: *const Guid, rev: u64, func: u64, argv4: *mut AcpiObject, r#type: AcpiObjectType) -> *mut AcpiObject {
    let obj = acpi_evaluate_dsm(handle, guid, rev, func, argv4);
    if !obj.is_null() && (*obj).object_type != r#type { acpi_free(obj); core::ptr::null_mut() } else { obj }
}
extern "C" { pub fn acpi_free(obj: *mut AcpiObject); }

#[repr(C)] pub struct AcpiDeviceStatus { pub present: u32, pub enabled: u32, pub show_in_ui: u32, pub functional: u32, pub battery_present: u32, pub reserved: u32 }
#[repr(C)] pub struct AcpiDeviceFlags { pub dynamic_status: u32, pub removable: u32, pub ejectable: u32, pub power_manageable: u32, pub initialized: u32, pub visited: u32, pub hotplug_notify: u32, pub is_dock_station: u32, pub of_compatible_ok: u32, pub coherent_dma: u32, pub cca_seen: u32, pub enumeration_by_parent: u32, pub honor_deps: u32, pub reserved: u32 }
#[repr(C)] pub struct AcpiDeviceDir { pub entry: *mut ProcDirEntry }
pub type AcpiBusId = [c_char; 8];
pub type AcpiBusAddress = u64;
pub type AcpiDeviceName = [c_char; 40];
pub type AcpiDeviceClass = [c_char; 20];
#[repr(C)] pub struct AcpiHardwareId { pub list: ListHead, pub id: *const c_char }
#[repr(C)] pub struct AcpiPnpType { pub hardware_id: u32, pub bus_address: u32, pub platform_id: u32, pub backlight: u32, pub reserved: u32 }
#[repr(C)] pub struct AcpiDevicePnp { pub bus_id: AcpiBusId, pub instance_no: c_int, pub r#type: AcpiPnpType, pub bus_address: AcpiBusAddress, pub unique_id: *mut c_char, pub ids: ListHead, pub device_name: AcpiDeviceName, pub device_class: AcpiDeviceClass }
#[repr(C)] pub struct AcpiDevicePowerFlags { pub explicit_get: u32, pub power_resources: u32, pub inrush_current: u32, pub power_removed: u32, pub ignore_parent: u32, pub dsw_present: u32, pub reserved: u32 }
#[repr(C)] pub struct AcpiDevicePowerStateFlags { pub valid: u8, pub explicit_set: u8, pub reserved: u8 }
#[repr(C)] pub struct AcpiDevicePowerState { pub resources: ListHead, pub flags: AcpiDevicePowerStateFlags, pub power: c_int, pub latency: c_int }
#[repr(C)] pub struct AcpiDevicePower { pub state: c_int, pub flags: AcpiDevicePowerFlags, pub states: [AcpiDevicePowerState; ACPI_D_STATE_COUNT], pub state_for_enumeration: u8 }
#[repr(C)] pub struct AcpiDepData { pub node: ListHead, pub supplier: AcpiHandle, pub consumer: AcpiHandle, pub honor_dep: bool, pub met: bool, pub free_when_met: bool }
#[repr(C)] pub struct AcpiDevicePerfFlags { pub reserved: u8 }
#[repr(C)] pub struct AcpiDevicePerfStateFlags { pub valid: u8, pub reserved: u8 }
#[repr(C)] pub struct AcpiDevicePerfState { pub flags: AcpiDevicePerfStateFlags, pub power: u8, pub performance: u8, pub latency: c_int }
#[repr(C)] pub struct AcpiDevicePerf { pub state: c_int, pub flags: AcpiDevicePerfFlags, pub state_count: c_int, pub states: *mut AcpiDevicePerfState }
#[repr(C)] pub struct AcpiDeviceWakeupFlags { pub valid: u8, pub notifier_present: u8 }
pub type AcpiWakeupFunc = Option<unsafe extern "C" fn(*mut AcpiDeviceWakeupContext)>;
#[repr(C)] pub struct AcpiDeviceWakeupContext { pub func: AcpiWakeupFunc, pub dev: *mut Device }
#[repr(C)] pub struct AcpiDeviceWakeup { pub gpe_device: AcpiHandle, pub gpe_number: u64, pub sleep_state: u64, pub resources: ListHead, pub flags: AcpiDeviceWakeupFlags, pub context: AcpiDeviceWakeupContext, pub ws: *mut WakeupSource, pub prepare_count: c_int, pub enable_count: c_int }
#[repr(C)] pub struct AcpiDevicePhysicalNode { pub node: ListHead, pub dev: *mut Device, pub node_id: u32, pub put_online: bool }
#[repr(C)] pub struct AcpiDeviceProperties { pub list: ListHead, pub guid: *const Guid, pub properties: *mut AcpiObject, pub bufs: *mut *mut c_void }
#[repr(C)] pub struct AcpiDeviceData { pub pointer: *const AcpiObject, pub properties: ListHead, pub of_compatible: *const AcpiObject, pub subnodes: ListHead }
#[repr(C)] pub struct AcpiHotplugProfile { pub kobj: Kobject, pub scan_dependent: Option<unsafe extern "C" fn(*mut AcpiDevice) -> c_int>, pub notify_online: Option<unsafe extern "C" fn(*mut AcpiDevice)>, pub enabled: bool, pub demand_offline: bool }
pub type AcpiHpNotify = Option<unsafe extern "C" fn(*mut AcpiDevice, u32) -> c_int>;
pub type AcpiHpUevent = Option<unsafe extern "C" fn(*mut AcpiDevice, u32)>;
pub type AcpiHpFixup = Option<unsafe extern "C" fn(*mut AcpiDevice)>;
#[repr(C)] pub struct AcpiHotplugContext { pub self_: *mut AcpiDevice, pub notify: AcpiHpNotify, pub uevent: AcpiHpUevent, pub fixup: AcpiHpFixup }
#[repr(C)] pub struct AcpiScanHandler { pub list_node: ListHead, pub ids: *const AcpiDeviceId, pub r#match: Option<unsafe extern "C" fn(*const c_char, *mut *const AcpiDeviceId) -> bool>, pub attach: Option<unsafe extern "C" fn(*mut AcpiDevice, *const AcpiDeviceId) -> c_int>, pub detach: Option<unsafe extern "C" fn(*mut AcpiDevice)>, pub post_eject: Option<unsafe extern "C" fn(*mut AcpiDevice)>, pub bind: Option<unsafe extern "C" fn(*mut Device)>, pub unbind: Option<unsafe extern "C" fn(*mut Device)>, pub hotplug: AcpiHotplugProfile }
#[repr(C)] pub struct AcpiDeviceSoftwareNodePort { pub port_name: [c_char; 9], pub data_lanes: [u32; 8], pub lane_polarities: [u32; 9], pub link_frequencies: [u64; 8], pub port_nr: u32, pub crs_csi2_local: bool, pub port_props: [PropertyEntry; 3], pub ep_props: [PropertyEntry; 9], pub remote_ep: [SoftwareNodeRefArgs; 1] }
#[repr(C)] pub struct AcpiDeviceSoftwareNodes { pub dev_props: [PropertyEntry; 6], pub nodes: *mut SoftwareNode, pub nodeptrs: *const *const SoftwareNode, pub ports: *mut AcpiDeviceSoftwareNodePort, pub num_ports: u32 }
#[repr(C)] pub struct AcpiDevice { pub pld_crc: u32, pub device_type: c_int, pub handle: AcpiHandle, pub fwnode: FwnodeHandle, pub wakeup_list: ListHead, pub del_list: ListHead, pub status: AcpiDeviceStatus, pub flags: AcpiDeviceFlags, pub pnp: AcpiDevicePnp, pub power: AcpiDevicePower, pub wakeup: AcpiDeviceWakeup, pub performance: AcpiDevicePerf, pub dir: AcpiDeviceDir, pub data: AcpiDeviceData, pub handler: *mut AcpiScanHandler, pub hp: *mut AcpiHotplugContext, pub swnodes: *mut AcpiDeviceSoftwareNodes, pub driver_gpios: *const c_void, pub driver_data: *mut c_void, pub dev: Device, pub physical_node_count: u32, pub dep_unmet: u32, pub physical_node_list: ListHead, pub physical_node_lock: Mutex, pub remove: Option<unsafe extern "C" fn(*mut AcpiDevice)> }
#[repr(C)] pub struct AcpiDataNode { pub sibling: ListHead, pub name: *const c_char, pub handle: AcpiHandle, pub fwnode: FwnodeHandle, pub parent: *mut FwnodeHandle, pub data: AcpiDeviceData, pub kobj: Kobject, pub kobj_done: Completion }

pub const ACPI_DEVICE_SWNODE_ROOT: usize = 0;
pub const ACPI_DEVICE_CSI2_DATA_LANES: usize = 8;
pub const ACPI_DEVICE_SWNODE_PORT_NAME_LENGTH: usize = 8;
#[inline] pub const fn acpi_device_swnode_port(port: usize) -> usize { 2 * port + 1 }
#[inline] pub const fn acpi_device_swnode_ep(endpoint: usize) -> usize { acpi_device_swnode_port(endpoint) + 1 }

extern "C" {
    pub static acpi_device_fwnode_ops: FwnodeOperations;
    pub static acpi_data_fwnode_ops: FwnodeOperations;
    pub static acpi_static_fwnode_ops: FwnodeOperations;
    pub fn is_acpi_device_node(fwnode: *const FwnodeHandle) -> bool;
    pub fn is_acpi_data_node(fwnode: *const FwnodeHandle) -> bool;
    pub fn acpi_initialize_hp_context(adev: *mut AcpiDevice, hp: *mut AcpiHotplugContext, notify: AcpiHpNotify, uevent: AcpiHpUevent);
    pub static acpi_bus_type: BusType;
    pub fn acpi_device_hid(device: *mut AcpiDevice) -> *const c_char;
    pub fn acpi_bus_get_status_handle(handle: AcpiHandle, sta: *mut u64) -> AcpiStatus;
    pub fn acpi_bus_get_status(device: *mut AcpiDevice) -> c_int;
    pub fn acpi_bus_set_power(handle: AcpiHandle, state: c_int) -> c_int;
    pub fn acpi_power_state_string(state: c_int) -> *const c_char;
    pub fn acpi_device_set_power(device: *mut AcpiDevice, state: c_int) -> c_int;
    pub fn acpi_bus_init_power(device: *mut AcpiDevice) -> c_int;
    pub fn acpi_device_fix_up_power(device: *mut AcpiDevice) -> c_int;
    pub fn acpi_device_fix_up_power_extended(adev: *mut AcpiDevice);
    pub fn acpi_device_fix_up_power_children(adev: *mut AcpiDevice);
    pub fn acpi_bus_update_power(handle: AcpiHandle, state_p: *mut c_int) -> c_int;
    pub fn acpi_device_update_power(device: *mut AcpiDevice, state_p: *mut c_int) -> c_int;
    pub fn acpi_bus_power_manageable(handle: AcpiHandle) -> bool;
    pub fn acpi_dev_power_up_children_with_adr(adev: *mut AcpiDevice);
    pub fn acpi_dev_power_state_for_wake(adev: *mut AcpiDevice) -> u8;
    pub fn acpi_device_enumerated(adev: *mut AcpiDevice) -> bool;
    pub fn acpi_dev_uid_to_integer(adev: *mut AcpiDevice, integer: *mut u64) -> c_int;
    pub fn acpi_dev_clear_dependencies(supplier: *mut AcpiDevice);
    pub fn acpi_dev_ready_for_enumeration(device: *const AcpiDevice) -> bool;
    pub fn acpi_dev_get_next_consumer_dev(supplier: *mut AcpiDevice, start: *mut AcpiDevice) -> *mut AcpiDevice;
    pub fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: i64) -> *mut AcpiDevice;
    pub fn acpi_dev_get_next_match_dev(adev: *mut AcpiDevice, hid: *const c_char, uid: *const c_char, hrv: i64) -> *mut AcpiDevice;
    pub fn acpi_fetch_acpi_dev(handle: AcpiHandle) -> *mut AcpiDevice;
    pub fn acpi_get_acpi_dev(handle: AcpiHandle) -> *mut AcpiDevice;
}

#[inline] pub unsafe fn is_acpi_node(fwnode: *const FwnodeHandle) -> bool { is_acpi_device_node(fwnode) || is_acpi_data_node(fwnode) }
#[inline] pub unsafe fn acpi_fwnode_handle(adev: *mut AcpiDevice) -> *mut FwnodeHandle { &mut (*adev).fwnode }
#[inline] pub unsafe fn acpi_driver_data(d: *mut AcpiDevice) -> *mut c_void { (*d).driver_data }
#[inline] pub unsafe fn acpi_device_enumerated_inline(adev: *mut AcpiDevice) -> bool { !adev.is_null() && (*adev).flags.initialized != 0 && (*adev).flags.visited != 0 }
#[inline] pub unsafe fn acpi_set_device_status(adev: *mut AcpiDevice, sta: u32) { core::ptr::write(adev.cast::<u32>().add(0), sta); }
#[inline] pub unsafe fn acpi_set_hp_context(adev: *mut AcpiDevice, hp: *mut AcpiHotplugContext) { (*hp).self_ = adev; (*adev).hp = hp; }
#[inline] pub unsafe fn acpi_device_power_manageable(adev: *mut AcpiDevice) -> bool { (*adev).flags.power_manageable != 0 }
#[inline] pub unsafe fn acpi_device_can_wakeup(adev: *mut AcpiDevice) -> bool { (*adev).wakeup.flags.valid != 0 }
#[inline] pub unsafe fn acpi_device_can_poweroff(adev: *mut AcpiDevice) -> bool { (*adev).power.states[ACPI_STATE_D3_COLD].flags.valid != 0 || ((*acpi_gbl_FADT).header.revision < 6 && (*adev).power.states[ACPI_STATE_D3_HOT].flags.explicit_set != 0) }
extern "C" { pub static acpi_gbl_FADT: *const AcpiGblFadt; }

#[repr(C)] pub struct AcpiBusEvent { pub node: ListHead, pub device_class: AcpiDeviceClass, pub bus_id: AcpiBusId, pub r#type: u32, pub data: u32 }
pub const ACPI_AC_CLASS: &[u8] = b"ac_adapter\0";
#[repr(C)] pub struct AcpiBusType { pub list: ListHead, pub name: *const c_char, pub r#match: Option<unsafe extern "C" fn(*mut Device) -> bool>, pub find_companion: Option<unsafe extern "C" fn(*mut Device) -> *mut AcpiDevice>, pub setup: Option<unsafe extern "C" fn(*mut Device)> }
#[repr(C)] pub struct AcpiPciRoot { pub device: *mut AcpiDevice, pub bus: *mut PciBus, pub segment: u16, pub bridge_type: c_int, pub secondary: Resource, pub osc_support_set: u32, pub osc_control_set: u32, pub osc_ext_support_set: u32, pub osc_ext_control_set: u32, pub mcfg_addr: PhysAddr }
#[repr(C)] pub struct AcpiBridgeType(pub c_int);
pub const ACPI_BRIDGE_TYPE_PCIE: c_int = 1;
pub const ACPI_BRIDGE_TYPE_CXL: c_int = 2;

/* Remaining declarations are external kernel interfaces from the header. */
extern "C" {
    pub fn acpi_bus_for_each_dev(fn_: Option<unsafe extern "C" fn(*mut Device, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn acpi_dev_for_each_child(adev: *mut AcpiDevice, fn_: Option<unsafe extern "C" fn(*mut AcpiDevice, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn acpi_dev_for_each_child_reverse(adev: *mut AcpiDevice, fn_: Option<unsafe extern "C" fn(*mut AcpiDevice, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn acpi_dev_install_notify_handler(adev: *mut AcpiDevice, handler_type: u32, handler: AcpiNotifyHandler, context: *mut c_void) -> c_int;
    pub fn acpi_dev_remove_notify_handler(adev: *mut AcpiDevice, handler_type: u32, handler: AcpiNotifyHandler);
    pub fn acpi_bus_scan(handle: AcpiHandle) -> c_int;
    pub fn acpi_bus_trim(start: *mut AcpiDevice);
    pub fn acpi_find_child_device(parent: *mut AcpiDevice, address: u64, check_children: bool) -> *mut AcpiDevice;
    pub fn acpi_find_child_by_adr(adev: *mut AcpiDevice, adr: AcpiBusAddress) -> *mut AcpiDevice;
    pub fn acpi_is_root_bridge(handle: AcpiHandle) -> c_int;
    pub fn acpi_pci_find_root(handle: AcpiHandle) -> *mut AcpiPciRoot;
    pub fn acpi_dma_supported(adev: *const AcpiDevice) -> bool;
    pub fn acpi_get_dma_attr(adev: *mut AcpiDevice) -> DevDmaAttr;
    pub fn acpi_dev_power_state_for_wake(adev: *mut AcpiDevice) -> u8;
    pub fn acpi_dev_put(adev: *mut AcpiDevice);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
