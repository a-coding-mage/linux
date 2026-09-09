// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of powerpc/platforms/powermac/pci.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Kernel and architecture types/functions are supplied by the surrounding tree. */
#[repr(C)] pub struct device_node { pub sibling: *mut device_node, pub child: *mut device_node, pub parent: *mut device_node, pub full_name: *const c_char }
#[repr(C)] pub struct property { pub length: c_int, pub value: *mut c_void }
#[repr(C)] pub struct resource { pub start: c_ulong, pub end: c_ulong, pub flags: c_ulong, pub name: *const c_char }
#[repr(C)] pub struct pci_bus { pub number: u8, pub self_: *mut pci_dev, pub dev_of_node: *mut device_node }
#[repr(C)] pub struct pci_dev { pub vendor: u16, pub device: u16, pub class: u32, pub irq: c_int, pub devfn: u8, pub bus: *mut pci_bus, pub of_node: *mut device_node, pub resource: [resource; 6] }
#[repr(C)] pub struct pci_controller { pub first_busno: u8, pub last_busno: u8, pub dn: *mut device_node, pub cfg_addr: *mut u8, pub cfg_data: *mut u8, pub ops: *const pci_ops, pub controller_ops: pci_controller_ops, pub mem_resources: [resource; 3], pub mem_offset: [c_ulong; 3], pub io_base_phys: c_ulong, pub pci_io_size: c_ulong, pub io_resource: resource }
#[repr(C)] pub struct pci_host_bridge { pub bus: *mut pci_bus }
#[repr(C)] pub struct pci_ops { pub map_bus: Option<unsafe extern "C" fn(*mut pci_bus,c_uint,c_int)->*mut u8>, pub read: Option<unsafe extern "C" fn() -> c_int>, pub write: Option<unsafe extern "C" fn() -> c_int> }
#[repr(C)] pub struct pci_controller_ops { pub probe_mode: Option<unsafe extern "C" fn(*mut pci_bus)->c_int>, pub enable_device_hook: Option<unsafe extern "C" fn(*mut pci_dev)->bool> }

extern "C" {
    static mut pcibios_assign_bus_offset: c_int; static mut k2_skiplist: [*mut device_node; 2];
    fn of_get_property(*mut device_node,*const c_char,*mut c_int)->*const u32; fn of_find_property(*mut device_node,*const c_char,*mut c_int)->*mut property;
    fn of_pci_find_child_device(*mut device_node,c_uint)->*mut device_node; fn pci_bus_to_host(*mut pci_bus)->*mut pci_controller;
    fn ioremap(c_ulong,c_ulong)->*mut u8; fn in_le32(*mut u8)->u32; fn out_le32(*mut u8,u32); fn in_be32(*mut u8)->u32; fn in_8(*mut u8)->u8; fn in_le16(*mut u8)->u16; fn in_be16(*mut u8)->u16; fn out_8(*mut u8,u32); fn out_le16(*mut u8,u32); fn out_be16(*mut u8,u32); fn out_le32(*mut u8,u32); fn out_be32(*mut u8,u32); fn udelay(c_uint);
    fn pcibios_alloc_controller(*mut device_node)->*mut pci_controller; fn pci_process_bridge_OF_ranges(*mut pci_controller,*mut device_node,c_int); fn pci_devs_phb_init_dynamic(*mut pci_controller); fn pcibios_get_phb_of_node(*mut pci_bus)->*mut device_node;
    fn pci_generic_config_read()->c_int; fn pci_generic_config_write()->c_int; fn irq_create_mapping(*mut c_void,c_uint)->c_int; fn irq_set_irq_type(c_int,c_uint);
    fn printk(*const c_char,...); fn machine_is_powermac()->bool; fn pci_set_flags(c_uint); fn pci_has_flag(c_uint)->bool;
    fn pci_read_config_word(*mut pci_dev,c_uint,*mut u16)->c_int; fn pci_write_config_word(*mut pci_dev,c_uint,u16); fn pci_read_config_byte(*mut pci_dev,c_uint,*mut u8)->c_int; fn pci_write_config_byte(*mut pci_dev,c_uint,u8); fn pci_write_config_dword(*mut pci_dev,c_uint,u32);
    fn pci_device_to_OF_node(*mut pci_dev)->*mut device_node; fn pmac_call_feature(c_uint,*mut device_node,c_int,c_int); fn resource_size(*mut resource)->c_ulong;
}

static mut has_uninorth: c_int = 0;
#[cfg(target_pointer_width="64")] static mut u3_agp: *mut pci_controller = core::ptr::null_mut();
#[cfg(target_pointer_width="32")] static mut has_second_ohare: c_int = 0;

const BANDIT_DEVID_2:u32=8; const BANDIT_REVID:u32=3; const BANDIT_DEVNUM:u32=11; const BANDIT_MAGIC:u32=0x50; const BANDIT_COHERENT:u32=0x40;

#[inline] unsafe fn macrisc_cfa0(devfn:u32,off:u32)->u32 { (1u32 << (devfn >> 3)) | ((devfn & 7) << 8) | (off & 0xfc) }
#[inline] unsafe fn macrisc_cfa1(bus:u32,devfn:u32,off:u32)->u32 { (bus<<16)|(devfn<<8)|(off&0xfc)|1 }

unsafe extern "C" fn macrisc_cfg_map_bus(bus:*mut pci_bus,devfn:c_uint,mut offset:c_int)->*mut u8 { let hose=pci_bus_to_host(bus); if hose.is_null(){return core::ptr::null_mut()} let c=if (*bus).number==(*hose).first_busno {if devfn < (11<<3){return core::ptr::null_mut()} macrisc_cfa0(devfn,offset as u32)} else {macrisc_cfa1((*bus).number as u32,devfn,offset as u32)}; loop {out_le32((*hose).cfg_addr,c); if in_le32((*hose).cfg_addr)==c{break}}; offset &= if has_uninorth!=0 {7}else{3}; (*hose).cfg_data.add(offset as usize) }
static MACRISC_OPS:pci_ops=pci_ops{map_bus:Some(macrisc_cfg_map_bus),read:Some(pci_generic_config_read),write:Some(pci_generic_config_write)};

unsafe fn fixup_one_level_bus_range(mut node:*mut device_node,mut higher:c_int)->c_int { while !node.is_null(){let mut len=0; let class=of_get_property(node,b"class-code\0".as_ptr() as _,core::ptr::null_mut()); if class.is_null() || ((*class>>8)!=0x0604 && (*class>>8)!=0x0607){node=(*node).sibling;continue} let br=of_get_property(node,b"bus-range\0".as_ptr() as _,&mut len); if !br.is_null()&&len>2*core::mem::size_of::<c_int>() as i32&&*(br.add(1) as *const c_int)>higher{higher=*(br.add(1) as *const c_int)} higher=fixup_one_level_bus_range((*node).child,higher); node=(*node).sibling} higher }
unsafe fn fixup_bus_range(bridge:*mut device_node){let mut len=0;let p=of_find_property(bridge,b"bus-range\0".as_ptr() as _,&mut len);if p.is_null()||(*p).length<2*core::mem::size_of::<c_int>() as i32{return}let v=(*p).value as *mut c_int;*v.add(1)=fixup_one_level_bus_range((*bridge).child,*v.add(1));}

#[cfg(target_pointer_width="64")]
unsafe fn u3_ht_cfg_access(hose:*mut pci_controller,bus:u8,devfn:u8,offset:u8,swap:*mut c_int)->*mut u8{*swap=1;if bus==(*hose).first_busno{if devfn!=0{return (*hose).cfg_data.add(((devfn as usize)<<8)+offset as usize)}*swap=0;return (*hose).cfg_addr.add((offset as usize)<<2)}(*hose).cfg_data.add(((bus as usize)<<16)+((devfn as usize)<<8)+offset as usize)}

#[cfg(target_pointer_width="64")]
unsafe extern "C" fn pmac_pci_probe_mode(bus:*mut pci_bus)->c_int { let n=(*bus).dev_of_node; if (*bus).self_.is_null() && (!n.is_null()){return 0} 1 }

#[cfg(target_pointer_width="32")]
unsafe extern "C" fn pmac_pci_enable_device_hook(dev:*mut pci_dev)->bool { let node=pci_device_to_OF_node(dev); if (*dev).vendor==0x106b && (*dev).class==0x0c0310 && node.is_null(){return false} true }

#[no_mangle] pub static mut pmac_pci_controller_ops:pci_controller_ops=pci_controller_ops{
    #[cfg(target_pointer_width="64")] probe_mode:Some(pmac_pci_probe_mode), #[cfg(target_pointer_width="32")] probe_mode:None,
    #[cfg(target_pointer_width="32")] enable_device_hook:Some(pmac_pci_enable_device_hook), #[cfg(target_pointer_width="64")] enable_device_hook:None,
};

#[no_mangle] pub unsafe extern "C" fn pmac_pci_irq_fixup(_dev:*mut pci_dev) {}
#[no_mangle] pub unsafe extern "C" fn pmac_pci_init() { pci_set_flags(1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
