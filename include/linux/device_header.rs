// SPDX-License-Identifier: GPL-2.0
/* Translated from linux/device.h. External kernel types and functions are supplied elsewhere. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct device;
#[repr(C)] pub struct device_private;
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub sync_state: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct driver_private;
#[repr(C)] pub struct module;
#[repr(C)] pub struct subsys_private;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct fwnode_handle;
#[repr(C)] pub struct iommu_group;
#[repr(C)] pub struct dev_pin_info;
#[repr(C)] pub struct dev_iommu;
#[repr(C)] pub struct msi_device_data;
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16, pub ignore_lockdep: bool }
#[repr(C)] pub struct attribute_group;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kobject { pub uevent_suppress: c_uint, pub state_in_sysfs: c_int }
#[repr(C)] pub struct bus_type { pub name: *const c_char, pub offline: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub online: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub num_vf: Option<unsafe extern "C" fn(*mut device) -> c_int> }
#[repr(C)] pub struct class { pub name: *const c_char }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct dev_pm_ops;
#[repr(C)] pub struct dev_pm_domain;
#[repr(C)] pub struct em_perf_domain;
#[repr(C)] pub struct dma_map_ops;
#[repr(C)] pub struct bus_dma_region;
#[repr(C)] pub struct dma_coherent_mem;
#[repr(C)] pub struct cma;
#[repr(C)] pub struct io_tlb_mem;
#[repr(C)] pub struct dev_archdata;
#[repr(C)] pub struct pm_subsys_data;
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct bin_attribute;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct kref;
pub type dev_t = u64; pub type umode_t = u16; pub type kuid_t = u32; pub type kgid_t = u32;
pub type ssize_t = isize; pub type size_t = usize; pub type refcount_t = u32;

#[repr(C)] pub struct subsys_interface { pub name: *const c_char, pub subsys: *const bus_type, pub node: list_head, pub add_dev: Option<unsafe extern "C" fn(*mut device,*mut subsys_interface)->c_int>, pub remove_dev: Option<unsafe extern "C" fn(*mut device,*mut subsys_interface)> }
#[repr(C)] pub struct device_type { pub name: *const c_char, pub groups: *const *const attribute_group, pub uevent: Option<unsafe extern "C" fn(*const device,*mut c_void)->c_int>, pub devnode: Option<unsafe extern "C" fn(*const device,*mut umode_t,*mut kuid_t,*mut kgid_t)->*mut c_char>, pub release: Option<unsafe extern "C" fn(*mut device)>, pub pm: *const dev_pm_ops }
#[repr(C)] pub struct device_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*mut c_char)->ssize_t>, pub show_const: Option<unsafe extern "C" fn(*mut device,*const device_attribute,*mut c_char)->ssize_t>, pub store: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*const c_char,size_t)->ssize_t>, pub store_const: Option<unsafe extern "C" fn(*mut device,*const device_attribute,*const c_char,size_t)->ssize_t> }
#[repr(C)] pub struct dev_ext_attribute { pub attr: device_attribute, pub var: *mut c_void }

extern "C" { pub fn subsys_interface_register(*mut subsys_interface)->c_int; pub fn subsys_interface_unregister(*mut subsys_interface); pub fn subsys_system_register(*const bus_type,*const *const attribute_group)->c_int; pub fn subsys_virtual_register(*const bus_type,*const *const attribute_group)->c_int; }
extern "C" { pub fn device_show_ulong(*mut device,*mut device_attribute,*mut c_char)->ssize_t; pub fn device_store_ulong(*mut device,*mut device_attribute,*const c_char,size_t)->ssize_t; pub fn device_show_int(*mut device,*mut device_attribute,*mut c_char)->ssize_t; pub fn device_store_int(*mut device,*mut device_attribute,*const c_char,size_t)->ssize_t; pub fn device_show_bool(*mut device,*mut device_attribute,*mut c_char)->ssize_t; pub fn device_store_bool(*mut device,*mut device_attribute,*const c_char,size_t)->ssize_t; pub fn device_show_string(*mut device,*mut device_attribute,*mut c_char)->ssize_t; }

#[repr(C)] pub struct device_dma_parameters { pub max_segment_size: c_uint, pub min_align_mask: c_uint, pub segment_boundary_mask: c_ulong }
#[repr(i32)] pub enum device_link_state { DL_STATE_NONE=-1, DL_STATE_DORMANT, DL_STATE_AVAILABLE, DL_STATE_CONSUMER_PROBE, DL_STATE_ACTIVE, DL_STATE_SUPPLIER_UNBIND }
pub const DL_FLAG_STATELESS:u32=1<<0; pub const DL_FLAG_AUTOREMOVE_CONSUMER:u32=1<<1; pub const DL_FLAG_PM_RUNTIME:u32=1<<2; pub const DL_FLAG_RPM_ACTIVE:u32=1<<3; pub const DL_FLAG_AUTOREMOVE_SUPPLIER:u32=1<<4; pub const DL_FLAG_AUTOPROBE_CONSUMER:u32=1<<5; pub const DL_FLAG_MANAGED:u32=1<<6; pub const DL_FLAG_SYNC_STATE_ONLY:u32=1<<7; pub const DL_FLAG_INFERRED:u32=1<<8; pub const DL_FLAG_CYCLE:u32=1<<9;
#[repr(i32)] pub enum dl_dev_state { DL_DEV_NO_DRIVER, DL_DEV_PROBING, DL_DEV_DRIVER_BOUND, DL_DEV_UNBINDING }
#[repr(i32)] pub enum device_removable { DEVICE_REMOVABLE_NOT_SUPPORTED, DEVICE_REMOVABLE_UNKNOWN, DEVICE_FIXED, DEVICE_REMOVABLE }
#[repr(C)] pub struct dev_links_info { pub suppliers:list_head, pub consumers:list_head, pub defer_sync:list_head, pub status:dl_dev_state }
#[repr(C)] pub struct dev_msi_info { pub domain:*mut irq_domain, pub data:*mut msi_device_data }
#[repr(i32)] pub enum device_physical_location_panel { DEVICE_PANEL_TOP, DEVICE_PANEL_BOTTOM, DEVICE_PANEL_LEFT, DEVICE_PANEL_RIGHT, DEVICE_PANEL_FRONT, DEVICE_PANEL_BACK, DEVICE_PANEL_UNKNOWN }
#[repr(i32)] pub enum device_physical_location_vertical_position { DEVICE_VERT_POS_UPPER, DEVICE_VERT_POS_CENTER, DEVICE_VERT_POS_LOWER }
#[repr(i32)] pub enum device_physical_location_horizontal_position { DEVICE_HORI_POS_LEFT, DEVICE_HORI_POS_CENTER, DEVICE_HORI_POS_RIGHT }
#[repr(C)] pub struct device_physical_location { pub panel:device_physical_location_panel, pub vertical_position:device_physical_location_vertical_position, pub horizontal_position:device_physical_location_horizontal_position, pub dock:bool, pub lid:bool }
#[repr(i32)] pub enum struct_device_flags { DEV_FLAG_READY_TO_PROBE, DEV_FLAG_CAN_MATCH, DEV_FLAG_DMA_IOMMU, DEV_FLAG_DMA_SKIP_SYNC, DEV_FLAG_DMA_OPS_BYPASS, DEV_FLAG_STATE_SYNCED, DEV_FLAG_DMA_COHERENT, DEV_FLAG_OF_NODE_REUSED, DEV_FLAG_OFFLINE_DISABLED, DEV_FLAG_OFFLINE, DEV_FLAG_COUNT }

#[repr(C)] pub struct driver_override { pub name:*const c_char, pub lock:spinlock_t }
#[repr(C)] pub struct dev_pm_info { pub subsys_data:*mut pm_subsys_data, pub is_prepared:bool, pub async_suspend:bool, pub no_pm:bool, pub no_callbacks:bool, pub syscore:bool, pub driver_flags:u32, pub smart_suspend:bool, pub strict_midlayer:bool }
#[repr(C)] pub struct device { pub kobj:kobject, pub parent:*mut device, pub p:*mut device_private, pub init_name:*const c_char, pub type_:*const device_type, pub bus:*const bus_type, pub driver:*mut device_driver, pub platform_data:*mut c_void, pub driver_data:*mut c_void, pub driver_override:driver_override, pub mutex:mutex, pub links:dev_links_info, pub power:dev_pm_info, pub pm_domain:*mut dev_pm_domain, pub em_pd:*mut em_perf_domain, pub pins:*mut dev_pin_info, pub msi:dev_msi_info, pub dma_ops:*const dma_map_ops, pub dma_mask:*mut u64, pub coherent_dma_mask:u64, pub bus_dma_limit:u64, pub dma_range_map:*const bus_dma_region, pub dma_parms:*mut device_dma_parameters, pub dma_pools:list_head, pub dma_mem:*mut dma_coherent_mem, pub cma_area:*mut cma, pub dma_io_tlb_mem:*mut io_tlb_mem, pub dma_io_tlb_pools:list_head, pub dma_io_tlb_lock:spinlock_t, pub dma_uses_io_tlb:bool, pub archdata:dev_archdata, pub of_node:*mut device_node, pub fwnode:*mut fwnode_handle, pub numa_node:c_int, pub devt:dev_t, pub id:u32, pub devres_lock:spinlock_t, pub devres_head:list_head, pub class:*const class, pub groups:*const *const attribute_group, pub release:Option<unsafe extern "C" fn(*mut device)>, pub iommu_group:*mut iommu_group, pub iommu:*mut dev_iommu, pub physical_location:*mut device_physical_location, pub removable:device_removable, pub flags:[usize; 1] }
#[repr(C)] pub struct device_link { pub supplier:*mut device, pub s_node:list_head, pub consumer:*mut device, pub c_node:list_head, pub link_dev:device, pub status:device_link_state, pub flags:u32, pub rpm_active:refcount_t, pub kref:kref, pub rm_work:work_struct, pub supplier_preactivated:bool }

extern "C" { pub fn __device_set_driver_override(*mut device,*const c_char,size_t)->c_int; pub fn device_create_file(*mut device,*const device_attribute)->c_int; pub fn device_remove_file(*mut device,*const device_attribute); pub fn device_remove_file_self(*mut device,*const device_attribute)->bool; pub fn device_create_bin_file(*mut device,*const bin_attribute)->c_int; pub fn device_remove_bin_file(*mut device,*const bin_attribute); pub fn device_register(*mut device)->c_int; pub fn device_unregister(*mut device); pub fn device_initialize(*mut device); pub fn device_add(*mut device)->c_int; pub fn device_del(*mut device); pub fn get_device(*mut device)->*mut device; pub fn put_device(*mut device); pub fn device_rename(*mut device,*const c_char)->c_int; pub fn device_move(*mut device,*mut device,c_int)->c_int; pub fn device_change_owner(*mut device,kuid_t,kgid_t)->c_int; pub fn device_shutdown(); pub fn dev_driver_string(*const device)->*const c_char; }

#[inline] pub unsafe fn device_iommu_mapped(dev:*mut device)->bool { !(*dev).iommu_group.is_null() }
#[inline] pub unsafe fn dev_get_drvdata(dev:*const device)->*mut c_void { (*dev).driver_data }
#[inline] pub unsafe fn dev_set_drvdata(dev:*mut device,data:*mut c_void) { (*dev).driver_data=data; }
#[inline] pub unsafe fn device_supports_offline(dev:*mut device)->bool { (*dev).bus.as_ref().map_or(false,|b| b.offline.is_some() && b.online.is_some()) }
#[inline] pub unsafe fn dev_set_removable(dev:*mut device,r:device_removable){(*dev).removable=r}
#[inline] pub unsafe fn dev_is_removable(dev:*mut device)->bool{(*dev).removable==device_removable::DEVICE_REMOVABLE}
#[inline] pub unsafe fn dev_removable_is_valid(dev:*mut device)->bool{(*dev).removable!=device_removable::DEVICE_REMOVABLE_NOT_SUPPORTED}

extern "C" { pub fn device_link_add(*mut device,*mut device,u32)->*mut device_link; pub fn device_link_del(*mut device_link); pub fn device_link_remove(*mut c_void,*mut device); pub fn device_links_supplier_sync_state_pause(); pub fn device_links_supplier_sync_state_resume(); pub fn device_link_wait_removal(); pub fn root_device_unregister(*mut device); }
#[inline] pub unsafe fn device_link_test(link:*const device_link,flags:u32)->bool{((*link).flags&flags)!=0}

// C declaration-only APIs retained as external dependencies.
extern "C" { pub fn set_primary_fwnode(*mut device,*mut fwnode_handle); pub fn set_secondary_fwnode(*mut device,*mut fwnode_handle); pub fn device_set_node(*mut device,*mut fwnode_handle); pub fn device_add_of_node(*mut device,*mut device_node)->c_int; pub fn device_remove_of_node(*mut device); pub fn get_dev_from_fwnode(*mut fwnode_handle)->*mut device; }

extern "C" {
 pub fn device_for_each_child(*mut device,*mut c_void,*mut c_void)->c_int;
 pub fn device_for_each_child_reverse(*mut device,*mut c_void,*mut c_void)->c_int;
 pub fn device_for_each_child_reverse_from(*mut device,*mut device,*mut c_void,*mut c_void)->c_int;
 pub fn device_find_child(*mut device,*const c_void,*mut c_void)->*mut device;
 pub fn device_attach(*mut device)->c_int; pub fn driver_attach(*const device_driver)->c_int;
 pub fn device_release_driver(*mut device); pub fn device_initial_probe(*mut device);
 pub fn device_reprobe(*mut device)->c_int; pub fn device_is_bound(*mut device)->bool;
 pub fn device_create(*const class,*mut device,dev_t,*mut c_void,*const c_char,...)->*mut device;
 pub fn device_destroy(*const class,dev_t);
 pub fn device_add_groups(*mut device,*const *const attribute_group)->c_int;
 pub fn device_remove_groups(*mut device,*const *const attribute_group);
 pub fn devm_device_add_group(*mut device,*const attribute_group)->c_int;
 pub fn kill_device(*mut device)->bool;
 pub fn lock_device_hotplug(); pub fn unlock_device_hotplug(); pub fn lock_device_hotplug_sysfs()->c_int;
 pub fn device_offline(*mut device)->c_int; pub fn device_online(*mut device)->c_int;
 pub fn dev_set_name(*mut device,*const c_char,...)->c_int;
 pub fn device_driver_attach(*const device_driver,*mut device)->c_int;
 pub fn device_bind_driver(*mut device)->c_int;
 pub fn __root_device_register(*const c_char,*mut module)->*mut device;
}

#[inline] pub unsafe fn dev_to_node(dev:*mut device)->c_int { (*dev).numa_node }
#[inline] pub unsafe fn set_dev_node(dev:*mut device,node:c_int){(*dev).numa_node=node}
#[inline] pub unsafe fn dev_get_msi_domain(dev:*const device)->*mut irq_domain{(*dev).msi.domain}
#[inline] pub unsafe fn dev_set_msi_domain(dev:*mut device,d:*mut irq_domain){(*dev).msi.domain=d}
#[inline] pub unsafe fn dev_to_psd(dev:*mut device)->*mut pm_subsys_data{if dev.is_null(){core::ptr::null_mut()}else{(*dev).power.subsys_data}}
#[inline] pub unsafe fn dev_get_uevent_suppress(dev:*const device)->c_uint{(*dev).kobj.uevent_suppress}
#[inline] pub unsafe fn dev_set_uevent_suppress(dev:*mut device,val:c_int){(*dev).kobj.uevent_suppress=val as c_uint}
#[inline] pub unsafe fn device_is_registered(dev:*mut device)->c_int{(*dev).kobj.state_in_sysfs}
#[inline] pub unsafe fn device_enable_async_suspend(dev:*mut device){if !(*dev).power.is_prepared{(*dev).power.async_suspend=true}}
#[inline] pub unsafe fn device_disable_async_suspend(dev:*mut device){if !(*dev).power.is_prepared{(*dev).power.async_suspend=false}}
#[inline] pub unsafe fn device_async_suspend_enabled(dev:*mut device)->bool{(*dev).power.async_suspend}
#[inline] pub unsafe fn device_pm_not_required(dev:*mut device)->bool{(*dev).power.no_pm}
#[inline] pub unsafe fn device_set_pm_not_required(dev:*mut device){(*dev).power.no_pm=true;(*dev).power.no_callbacks=true}
#[inline] pub unsafe fn dev_pm_set_driver_flags(dev:*mut device,flags:u32){(*dev).power.driver_flags=flags}
#[inline] pub unsafe fn dev_pm_test_driver_flags(dev:*mut device,flags:u32)->bool{((*dev).power.driver_flags&flags)!=0}
#[inline] pub unsafe fn dev_pm_smart_suspend(dev:*mut device)->bool{(*dev).power.smart_suspend}
#[inline] pub unsafe fn dev_pm_set_strict_midlayer(dev:*mut device,val:bool){(*dev).power.strict_midlayer=val}
#[inline] pub unsafe fn dev_pm_strict_midlayer_is_set(dev:*mut device)->bool{(*dev).power.strict_midlayer}

// Locally meaningful attribute macro forms; handler selection remains an external C/Rust integration concern.
#[macro_export] macro_rules! DEVICE_ATTR { ($name:ident,$mode:expr,$show:expr,$store:expr) => { pub static mut $name: $crate::device_attribute = $crate::device_attribute { attr:$crate::attribute{name:concat!(stringify!($name),"\0").as_ptr() as *const _,mode:$mode,ignore_lockdep:false}, show:$show,show_const:None,store:$store,store_const:None }; }; }
#[macro_export] macro_rules! DEVICE_ATTR_RO { ($name:ident) => { $crate::DEVICE_ATTR!($name,0o444,None,None) }; }
#[macro_export] macro_rules! DEVICE_ATTR_RW { ($name:ident) => { $crate::DEVICE_ATTR!($name,0o644,None,None) }; }
#[macro_export] macro_rules! DEVICE_ATTR_WO { ($name:ident) => { $crate::DEVICE_ATTR!($name,0o200,None,None) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
