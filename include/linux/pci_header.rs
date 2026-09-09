/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/pci.h.  Included kernel dependencies remain external. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

/* Types supplied by the included kernel headers. */
pub type u8 = core::primitive::u8; pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32; pub type u64 = core::primitive::u64;
pub type ssize_t = isize; pub type loff_t = i64; pub type phys_addr_t = u64;
pub type resource_size_t = u64; pub type kernel_ulong_t = usize;
pub type pm_message_t = c_int; pub type irq_handler_t = usize; pub type resource_alignf = usize;
pub type pci_power_t = c_int; pub type pci_channel_state_t = u32;
pub type pcie_reset_state_t = u32; pub type pci_dev_flags_t = u16; pub type pci_bus_flags_t = u16;
pub type pci_ers_result_t = u32;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub start: resource_size_t, pub end: resource_size_t, pub flags: u64, _private: [u8; 0] }
#[repr(C)] pub struct device_dma_parameters { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct irq_affinity { _private: [u8; 0] }
#[repr(C)] pub struct irq_affinity_desc { _private: [u8; 0] }
#[repr(C)] pub struct msi_map { pub index: c_int, pub virq: c_int }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct hotplug_slot { _private: [u8; 0] }
#[repr(C)] pub struct pci_sriov { _private: [u8; 0] }
#[repr(C)] pub struct pci_p2pdma { _private: [u8; 0] }
#[repr(C)] pub struct pcie_bwctrl_data { _private: [u8; 0] }
#[repr(C)] pub struct pcie_link_state { _private: [u8; 0] }
#[repr(C)] pub struct rcec_ea { _private: [u8; 0] }
#[repr(C)] pub struct npem { _private: [u8; 0] }
#[repr(C)] pub struct pci_tsm { _private: [u8; 0] }
#[repr(C)] pub struct pci_saved_state { _private: [u8; 0] }

pub const PCI_NUM_RESET_METHODS: usize = 8;
pub const PCI_RESET_PROBE: bool = true; pub const PCI_RESET_DO_RESET: bool = false;
pub const PCI_SLOT_ALL_DEVICES: u16 = 0xfeff; pub const PCI_SLOT_PLACEHOLDER: u16 = 0xffff;
pub const PCI_NUM_INTX: usize = 4; pub const PCI_ERROR_RESPONSE: u64 = !0;
pub const PCI_D0: pci_power_t=0; pub const PCI_D1:pci_power_t=1; pub const PCI_D2:pci_power_t=2;
pub const PCI_D3hot:pci_power_t=3; pub const PCI_D3cold:pci_power_t=4; pub const PCI_UNKNOWN:pci_power_t=5; pub const PCI_POWER_ERROR:pci_power_t=-1;
pub const PCI_DOMAIN_NR_NOT_SET: c_int = -1;

#[repr(C)] pub struct pci_slot { pub bus:*mut pci_bus, pub list:list_head, pub hotplug:*mut hotplug_slot, pub number:u16, pub per_func_slot:u32, pub kobj:kobject }
#[repr(C)] pub struct pci_vpd { pub lock:mutex, pub len:u32, pub cap:u8 }
#[repr(C)] pub struct pci_bus_region { pub start: u64, pub end: u64 }
#[repr(C)] pub struct pci_dynids { pub lock:spinlock_t, pub list:list_head }

#[repr(C)] pub struct pci_ops { pub add_bus:Option<unsafe extern "C" fn(*mut pci_bus)->c_int>, pub remove_bus:Option<unsafe extern "C" fn(*mut pci_bus)>, pub map_bus:Option<unsafe extern "C" fn(*mut pci_bus,u32,c_int)->*mut c_void>, pub read:Option<unsafe extern "C" fn(*mut pci_bus,u32,c_int,c_int,*mut u32)->c_int>, pub write:Option<unsafe extern "C" fn(*mut pci_bus,u32,c_int,c_int,u32)->c_int> }
#[repr(C)] pub struct pci_error_handlers { pub error_detected:Option<unsafe extern "C" fn(*mut pci_dev,pci_channel_state_t)->pci_ers_result_t>, pub mmio_enabled:Option<unsafe extern "C" fn(*mut pci_dev)->pci_ers_result_t>, pub slot_reset:Option<unsafe extern "C" fn(*mut pci_dev)->pci_ers_result_t>, pub reset_prepare:Option<unsafe extern "C" fn(*mut pci_dev)>, pub reset_done:Option<unsafe extern "C" fn(*mut pci_dev)>, pub resume:Option<unsafe extern "C" fn(*mut pci_dev)>, pub cor_error_detected:Option<unsafe extern "C" fn(*mut pci_dev)> }

#[repr(C)] pub struct pci_dev {
 pub bus_list:list_head, pub bus:*mut pci_bus, pub subordinate:*mut pci_bus, pub sysdata:*mut c_void, pub procent:*mut proc_dir_entry, pub slot:*mut pci_slot,
 pub devfn:u32, pub vendor:u16, pub device:u16, pub subsystem_vendor:u16, pub subsystem_device:u16, pub class:u32, pub revision:u8, pub hdr_type:u8,
 pub devcap:u32, pub rebar_cap:u16, pub pcie_cap:u8, pub msi_cap:u8, pub msix_cap:u8, pub pcie_mpss:u8, pub rom_base_reg:u8, pub pin:u8, pub pcie_flags_reg:u16, pub dma_alias_mask:*mut usize,
 pub driver:*mut pci_driver, pub dma_mask:u64, pub msi_addr_mask:u64, pub dma_parms:device_dma_parameters, pub current_state:pci_power_t, pub pm_cap:u8,
 pub error_state:pci_channel_state_t, pub dev:device, pub cfg_size:c_int, pub irq:u32, pub resource:*mut resource, pub driver_exclusive_resource:resource,
 pub dev_flags:pci_dev_flags_t, pub enable_cnt:atomic_t, pub pcie_cap_lock:spinlock_t, pub saved_config_space:[u32;16], pub saved_cap_space:hlist_head,
 pub vpd:pci_vpd, pub link_bwctrl:*mut pcie_bwctrl_data, pub acs_cap:u16, pub acs_capabilities:u16, pub supported_speeds:u8, pub rom:phys_addr_t, pub romlen:usize, pub priv_flags:usize, pub reset_methods:[u8;8], pub wake:*mut gpio_desc,
 pub is_virtfn:u32, pub is_physfn:u32, pub is_hotplug_bridge:u32, pub is_pciehp:u32, pub is_cxl:u32, pub msi_enabled:u32, pub msix_enabled:u32, pub broken_intx_masking:u32,
}
#[repr(C)] pub struct pci_host_bridge { pub dev:device, pub bus:*mut pci_bus, pub ops:*mut pci_ops, pub child_ops:*mut pci_ops, pub sysdata:*mut c_void, pub busnr:c_int, pub domain_nr:c_int, pub windows:list_head, pub dma_ranges:list_head, pub ports:list_head, pub private_: [usize;0] }
#[repr(C)] pub struct pci_bus { pub node:list_head, pub parent:*mut pci_bus, pub children:list_head, pub devices:list_head, pub self_:*mut pci_dev, pub slots:list_head, pub resource:[*mut resource;4], pub resources:list_head, pub busn_res:resource, pub ops:*mut pci_ops, pub sysdata:*mut c_void, pub procdir:*mut proc_dir_entry, pub number:u8, pub primary:u8, pub max_bus_speed:u8, pub cur_bus_speed:u8, pub name:[c_char;48], pub bridge_ctl:u16, pub bus_flags:pci_bus_flags_t, pub bridge:*mut device, pub dev:device }
#[repr(C)] pub struct pci_driver { pub name:*const c_char, pub id_table:*const pci_device_id, pub probe:Option<unsafe extern "C" fn(*mut pci_dev,*const pci_device_id)->c_int>, pub remove:Option<unsafe extern "C" fn(*mut pci_dev)>, pub suspend:Option<unsafe extern "C" fn(*mut pci_dev,pm_message_t)->c_int>, pub resume:Option<unsafe extern "C" fn(*mut pci_dev)->c_int>, pub shutdown:Option<unsafe extern "C" fn(*mut pci_dev)>, pub err_handler:*const pci_error_handlers, pub driver:device_driver, pub dynids:pci_dynids, pub driver_managed_dma:bool }

pub const PCI_INTERRUPT_UNKNOWN:u32=0; pub const PCI_INTERRUPT_INTA:u32=1; pub const PCI_INTERRUPT_INTB:u32=2; pub const PCI_INTERRUPT_INTC:u32=3; pub const PCI_INTERRUPT_INTD:u32=4;
pub const PCI_IRQ_INTX:u32=1; pub const PCI_IRQ_MSI:u32=2; pub const PCI_IRQ_MSIX:u32=4; pub const PCI_IRQ_AFFINITY:u32=8; pub const PCI_IRQ_VIRTUAL:u32=16; pub const PCI_IRQ_ALL_TYPES:u32=7;
pub const PCIBIOS_SUCCESSFUL:c_int=0; pub const PCIBIOS_FUNC_NOT_SUPPORTED:c_int=0x81; pub const PCIBIOS_BAD_VENDOR_ID:c_int=0x83; pub const PCIBIOS_DEVICE_NOT_FOUND:c_int=0x86; pub const PCIBIOS_BAD_REGISTER_NUMBER:c_int=0x87; pub const PCIBIOS_SET_FAILED:c_int=0x88; pub const PCIBIOS_BUFFER_TOO_SMALL:c_int=0x89;

#[repr(C)] pub struct msix_entry { pub vector:u32, pub entry:u16 }
#[repr(C)] pub struct pcie_ptm_ops { pub check_capability:Option<unsafe extern "C" fn(*mut c_void)->c_int>, pub context_update_write:Option<unsafe extern "C" fn(*mut c_void,u8)->c_int>, pub context_valid_write:Option<unsafe extern "C" fn(*mut c_void,bool)->c_int>, pub local_clock_read:Option<unsafe extern "C" fn(*mut c_void,*mut u64)->c_int> }
#[repr(C)] pub struct pci_ptm_debugfs { pub debugfs:*mut dentry, pub ops:*const pcie_ptm_ops, pub lock:mutex, pub pdata:*mut c_void }

#[inline] pub unsafe fn pci_devid(bus:u16,devfn:u32)->u16 { ((bus<<8) | devfn as u16) }
#[inline] pub unsafe fn pci_bus_num(x:u16)->u16 { (x>>8)&0xff }
#[inline] pub unsafe fn pci_physfn(dev:*mut pci_dev)->*mut pci_dev { if !dev.is_null() && (*dev).is_virtfn != 0 { dev } else { dev } }
#[inline] pub unsafe fn pci_is_root_bus(bus:*mut pci_bus)->bool { !(*bus).parent.is_null() == false }
#[inline] pub unsafe fn pci_is_bridge(dev:*mut pci_dev)->bool { (*dev).hdr_type == 1 || (*dev).hdr_type == 2 }
#[inline] pub unsafe fn pci_is_vga(dev:*mut pci_dev)->bool { ((*dev).class>>8)==0x0300 || ((*dev).class>>8)==0 }
#[inline] pub unsafe fn pci_is_display(dev:*mut pci_dev)->bool { ((*dev).class>>16)==3 }
#[inline] pub unsafe fn pci_channel_offline(dev:*mut pci_dev)->bool { (*dev).error_state != 1 }
#[inline] pub unsafe fn pci_dev_msi_enabled(dev:*mut pci_dev)->bool { (*dev).msi_enabled!=0 || (*dev).msix_enabled!=0 }
#[inline] pub unsafe fn pci_irq_type(dev:*mut pci_dev)->u32 { if (*dev).msix_enabled!=0 {4} else if (*dev).msi_enabled!=0 {2} else {1} }

extern "C" {
 pub static mut pci_flags:u32;
 pub fn pci_alloc_dev(bus:*mut pci_bus)->*mut pci_dev;
 pub fn pci_get_device(vendor:u32,device:u32,from:*mut pci_dev)->*mut pci_dev;
 pub fn pci_get_device_reverse(vendor:u32,device:u32,from:*mut pci_dev)->*mut pci_dev;
 pub fn pci_register_driver(driver:*mut pci_driver)->c_int; pub fn pci_unregister_driver(driver:*mut pci_driver);
 pub fn pci_enable_device(dev:*mut pci_dev)->c_int; pub fn pci_disable_device(dev:*mut pci_dev);
 pub fn pci_set_master(dev:*mut pci_dev); pub fn pci_clear_master(dev:*mut pci_dev);
 pub fn pci_read_config_byte(dev:*const pci_dev,where_:c_int,val:*mut u8)->c_int;
 pub fn pci_read_config_word(dev:*const pci_dev,where_:c_int,val:*mut u16)->c_int;
 pub fn pci_read_config_dword(dev:*const pci_dev,where_:c_int,val:*mut u32)->c_int;
 pub fn pci_write_config_byte(dev:*const pci_dev,where_:c_int,val:u8)->c_int;
 pub fn pci_write_config_word(dev:*const pci_dev,where_:c_int,val:u16)->c_int;
 pub fn pci_write_config_dword(dev:*const pci_dev,where_:c_int,val:u32)->c_int;
 pub fn pci_reset_function(dev:*mut pci_dev)->c_int; pub fn pci_assign_resource(dev:*mut pci_dev,i:c_int)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
