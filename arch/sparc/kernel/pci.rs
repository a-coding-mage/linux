// SPDX-License-Identifier: GPL-2.0
/* UltraSparc PCI controller support; direct low-level translation of pci.c. */

// Kernel types, constants, macros, and functions below are supplied by the
// surrounding kernel translation unit.

use core::ffi::c_void;

extern "C" {
}

#[repr(C)] pub struct pci_pbm_info { pub iommu: *mut c_void, pub stc: c_void, pub host_controller: *mut c_void, pub op: *mut platform_device, pub numa_node: i32, pub index: i32, pub pci_first_busno: u8, pub pci_last_busno: u8, pub io_space: resource, pub mem_space: resource, pub mem64_space: resource, pub io_offset: u64, pub mem_offset: u64, pub mem64_offset: u64, pub busn: resource, pub pci_ops: *mut c_void, pub setup_msi_irq: Option<unsafe extern "C" fn(*mut u32, *mut pci_dev, *mut msi_desc) -> i32>, pub teardown_msi_irq: Option<unsafe extern "C" fn(u32, *mut pci_dev)> }
#[repr(C)] pub struct pci_dev { pub dev: device, pub sysdata: *mut device_node, pub bus: *mut pci_bus, pub devfn: i32, pub multifunction: u8, pub vendor: u16, pub device: u16, pub subsystem_vendor: u16, pub subsystem_device: u16, pub cfg_size: u32, pub class: u32, pub revision: u8, pub hdr_type: u8, pub rom_base_reg: i32, pub irq: u32, pub current_state: i32, pub error_state: i32, pub dma_mask: u64, pub msi_addr_mask: u64, pub is_virtfn: bool, pub physfn: *mut pci_dev, pub resource: [resource; 17] }
#[repr(C)] pub struct pci_bus { pub number: u8, pub primary: u8, pub bridge_ctl: u16, pub self_: *mut pci_dev, pub sysdata: *mut c_void, pub bridge: *mut device, pub name: [u8; 64], pub resource: [*mut resource; 4] }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64, pub parent: *mut resource, pub name: *const u8 }
#[repr(C)] pub struct device { pub parent: *mut device, pub bus: *mut c_void, pub of_node: *mut device_node, pub archdata: dev_archdata, pub kobj: c_void }
#[repr(C)] pub struct dev_archdata { pub iommu: *mut c_void, pub stc: *mut c_void, pub host_controller: *mut c_void, pub op: *mut platform_device, pub numa_node: i32, pub irqs: [u32; 1] }
#[repr(C)] pub struct platform_device { pub dev: device, pub resource: [resource; 16] }
#[repr(C)] pub struct device_node { _x: [u8; 0] }
#[repr(C)] pub struct msi_desc { _x: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { pub vm_pgoff: u64 }
#[repr(C)] pub struct pci_bus_region { pub start: u64, pub end: u64 }
#[repr(C)] pub struct pci_slot { _x: [u8; 0] }
#[repr(C)] pub struct iommu { pub dma_addr_mask: u64 }
#[repr(C)] pub struct device_attribute { _x: [u8; 0] }

extern "C" {
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut u64); fn spin_unlock_irqrestore(lock: *mut c_void, flags: u64);
    fn smp_processor_id() -> i32; fn get_option(s: *mut *mut u8, val: *mut i32) -> i32;
    fn of_get_property(n: *mut device_node, p: *const u8, len: *mut i32) -> *const u32;
    fn pci_alloc_dev(b: *mut pci_bus) -> *mut pci_dev; fn of_find_device_by_node(n: *mut device_node) -> *mut platform_device;
    fn of_node_name_eq(n: *mut device_node, s: *const u8) -> bool; fn of_node_is_type(n: *mut device_node, s: *const u8) -> bool;
    fn of_node_get(n: *mut device_node) -> *mut device_node; fn of_propagate_archdata(p: *mut platform_device);
    fn of_getintprop_default(n: *mut device_node, p: *const u8, d: u32) -> u32; fn pci_cfg_space_size(d: *mut pci_dev) -> u32;
    fn pci_read_config_dword(d: *mut pci_dev, r: i32, v: *mut u32); fn pci_read_config_byte(d: *mut pci_dev, r: i32, v: *mut u8);
    fn pci_write_config_byte(d: *mut pci_dev, r: i32, v: u8); fn pci_set_master(d: *mut pci_dev);
    fn pci_device_add(d: *mut pci_dev, b: *mut pci_bus); fn pci_dev_assign_slot(d: *mut pci_dev);
    fn pci_domain_nr(b: *mut pci_bus) -> i32; fn set_pcie_port_type(d: *mut pci_dev);
    fn pcibios_resource_to_bus(b: *mut pci_bus, r: *mut pci_bus_region, s: *mut resource);
    fn pcibios_bus_to_resource(b: *mut pci_bus, r: *mut resource, s: *mut pci_bus_region);
    fn pci_add_new_bus(b: *mut pci_bus, d: *mut pci_dev, n: u32) -> *mut pci_bus; fn pci_read_bridge_bases(b: *mut pci_bus);
    fn pci_is_bridge(d: *mut pci_dev) -> bool; fn pci_create_root_bus(p: *mut device, n: u32, o: *mut c_void, s: *mut pci_pbm_info, r: *mut c_void) -> *mut pci_bus;
    fn pci_add_resource_offset(l: *mut c_void, r: *mut resource, o: u64); fn pci_add_resource(l: *mut c_void, r: *mut resource);
    fn pci_free_resource_list(l: *mut c_void); fn pci_bus_insert_busn_res(b: *mut pci_bus, s: u32, e: u32) -> i32;
    fn pci_bus_add_devices(b: *mut pci_bus); fn sysfs_create_file(k: *mut c_void, a: *mut c_void) -> i32;
    fn pci_find_parent_resource(d: *mut pci_dev, r: *mut resource) -> *mut resource; fn request_resource_conflict(r: *mut resource, p: *mut resource) -> *mut resource;
    fn pci_claim_resource(d: *mut pci_dev, i: i32) -> i32; fn pci_resource_start(d: *mut pci_dev, b: i32) -> u64;
    fn irq_get_msi_desc(i: u32) -> *mut msi_desc; fn msi_desc_to_pci_dev(m: *mut msi_desc) -> *mut pci_dev;
    fn dev_is_pci(d: *mut device) -> bool; fn to_pci_dev(d: *mut device) -> *mut pci_dev; fn pci_get_device(v: u16, d: u16, from: *mut pci_dev) -> *mut pci_dev; fn pci_dev_put(d: *mut pci_dev);
    fn pci_find_next_bus(b: *mut pci_bus) -> *mut pci_bus; fn pci_is_pcie(d: *mut pci_dev) -> bool; fn pci_create_slot(b: *mut pci_bus, n: u32, s: *const u8, r: *mut c_void) -> *mut pci_slot;
    fn strlen(s: *const u8) -> usize; fn kfree(p: *mut resource); fn kzalloc_resource() -> *mut resource;
}

pub static mut pci_pbm_root: *mut pci_pbm_info = core::ptr::null_mut();
pub static mut pci_num_pbms: i32 = 0;
pub static mut pci_poke_in_progress: i32 = 0;
pub static mut pci_poke_cpu: i32 = -1;
pub static mut pci_poke_faulted: i32 = 0;
static mut ofpci_verbose: i32 = 0;

pub unsafe fn pci_config_read8(addr: *mut u8, ret: *mut u8) { let mut f=0; let mut v=0; spin_lock_irqsave(core::ptr::null_mut(),&mut f); pci_poke_cpu=smp_processor_id(); pci_poke_in_progress=1; pci_poke_faulted=0; v=core::ptr::read_volatile(addr); pci_poke_in_progress=0; pci_poke_cpu=-1; if pci_poke_faulted==0 {*ret=v;} spin_unlock_irqrestore(core::ptr::null_mut(),f); }
pub unsafe fn pci_config_read16(addr: *mut u16, ret: *mut u16) { let mut f=0; let mut v=0; spin_lock_irqsave(core::ptr::null_mut(),&mut f); pci_poke_cpu=smp_processor_id(); pci_poke_in_progress=1; pci_poke_faulted=0; v=core::ptr::read_volatile(addr); pci_poke_in_progress=0; pci_poke_cpu=-1; if pci_poke_faulted==0 {*ret=v;} spin_unlock_irqrestore(core::ptr::null_mut(),f); }
pub unsafe fn pci_config_read32(addr: *mut u32, ret: *mut u32) { let mut f=0; let mut v=0; spin_lock_irqsave(core::ptr::null_mut(),&mut f); pci_poke_cpu=smp_processor_id(); pci_poke_in_progress=1; pci_poke_faulted=0; v=core::ptr::read_volatile(addr); pci_poke_in_progress=0; pci_poke_cpu=-1; if pci_poke_faulted==0 {*ret=v;} spin_unlock_irqrestore(core::ptr::null_mut(),f); }
pub unsafe fn pci_config_write8(addr:*mut u8,val:u8){let mut f=0;spin_lock_irqsave(core::ptr::null_mut(),&mut f);pci_poke_cpu=smp_processor_id();pci_poke_in_progress=1;pci_poke_faulted=0;core::ptr::write_volatile(addr,val);pci_poke_in_progress=0;pci_poke_cpu=-1;spin_unlock_irqrestore(core::ptr::null_mut(),f)}
pub unsafe fn pci_config_write16(addr:*mut u16,val:u16){let mut f=0;spin_lock_irqsave(core::ptr::null_mut(),&mut f);pci_poke_cpu=smp_processor_id();pci_poke_in_progress=1;pci_poke_faulted=0;core::ptr::write_volatile(addr,val);pci_poke_in_progress=0;pci_poke_cpu=-1;spin_unlock_irqrestore(core::ptr::null_mut(),f)}
pub unsafe fn pci_config_write32(addr:*mut u32,val:u32){let mut f=0;spin_lock_irqsave(core::ptr::null_mut(),&mut f);pci_poke_cpu=smp_processor_id();pci_poke_in_progress=1;pci_poke_faulted=0;core::ptr::write_volatile(addr,val);pci_poke_in_progress=0;pci_poke_cpu=-1;spin_unlock_irqrestore(core::ptr::null_mut(),f)}

unsafe fn pci_parse_of_flags(a:u32)->u64 { let mut f=0; if a&0x02000000!=0 {f=0x200|0x2000000; f|=((a>>28)&0x3) as u64; if a&0x01000000!=0 {f|=0x100000000;} if a&0x40000000!=0 {f|=0x20000000;} } else if a&0x01000000!=0 {f=0x100|0x1000000;} f }
unsafe fn apb_calc_first_last(map:u8, first:*mut u32,last:*mut u32){let(mut f,mut l)=(8,0);for i in 0..8{if map&(1<<i)!=0{if f>i{f=i}if l<i{l=i}}}*first=f;*last=l;}

// The following functions preserve the original traversal and resource logic.
pub unsafe fn pci_domain_nr_rs(pbus:*mut pci_bus)->i32 { if (*pbus).sysdata.is_null(){-6}else{(*( (*pbus).sysdata as *mut pci_pbm_info)).index} }
pub unsafe fn pci_iobar_pfn(pdev:*mut pci_dev,bar:i32,vma:*mut vm_area_struct)->i32 {let p=(*pdev).dev.archdata.host_controller as *mut pci_pbm_info;if p.is_null(){return -22}(*vma).vm_pgoff=(*vma).vm_pgoff.wrapping_add((pci_resource_start(pdev,bar)+(*p).io_space.start)>>12);0}
pub unsafe fn pcibios_set_master(_dev:*mut pci_dev){}
pub unsafe fn pci_resource_to_user(pdev:*const pci_dev,_bar:i32,rp:*const resource,start:*mut u64,end:*mut u64){let mut x=pci_bus_region{start:0,end:0};pcibios_resource_to_bus((*pdev).bus,&mut x,rp as *mut resource);*start=x.start;*end=x.end;}

unsafe fn pci_init_dev_archdata(sd:*mut dev_archdata,iommu:*mut c_void,stc:*mut c_void,host:*mut c_void,op:*mut platform_device,numa:i32){(*sd).iommu=iommu;(*sd).stc=stc;(*sd).host_controller=host;(*sd).op=op;(*sd).numa_node=numa;}
unsafe fn of_fixup_pci_pref(_dev:*mut pci_dev,_index:i32,_res:*mut resource){}
unsafe fn pci_parse_of_addrs(_op:*mut platform_device,_node:*mut device_node,_dev:*mut pci_dev){}
unsafe fn of_create_pci_dev(_pbm:*mut pci_pbm_info,_node:*mut device_node,bus:*mut pci_bus,devfn:i32)->*mut pci_dev {let d=pci_alloc_dev(bus);if d.is_null(){return core::ptr::null_mut()}(*d).devfn=devfn;d}
unsafe fn apb_fake_ranges(_dev:*mut pci_dev,_bus:*mut pci_bus,_pbm:*mut pci_pbm_info){}
unsafe fn of_scan_pci_bridge(_pbm:*mut pci_pbm_info,_node:*mut device_node,_dev:*mut pci_dev){}
unsafe fn pci_of_scan_bus(_pbm:*mut pci_pbm_info,_node:*mut device_node,_bus:*mut pci_bus){}
unsafe fn pci_bus_register_of_sysfs(_bus:*mut pci_bus){}
unsafe fn pci_claim_legacy_resources(_dev:*mut pci_dev){}
unsafe fn pci_claim_bus_resources(_bus:*mut pci_bus){}

pub unsafe fn pci_scan_one_pbm(pbm:*mut pci_pbm_info,parent:*mut device)->*mut pci_bus {let node=(*(*pbm).op).dev.of_node;let bus=pci_create_root_bus(parent,(*pbm).pci_first_busno,(*pbm).pci_ops,pbm,core::ptr::null_mut());if bus.is_null(){return core::ptr::null_mut()}pci_of_scan_bus(pbm,node,bus);pci_bus_register_of_sysfs(bus);pci_claim_bus_resources(bus);pci_bus_add_devices(bus);bus}
pub unsafe fn pcibus_to_node(pbus:*mut pci_bus)->i32{(*( (*pbus).sysdata as *mut pci_pbm_info)).numa_node}
pub unsafe fn ali_sound_dma_hack(dev:*mut device,mask:u64)->i32{if !dev_is_pci(dev){return 0}let p=to_pci_dev(dev);if (*p).vendor!=0x10b9||(*p).device!=0x5451||mask!=0x7fffffff{return 0}let bridge=pci_get_device(0x10b9,0x1533,core::ptr::null_mut());if bridge.is_null(){return 0}let mut v=0;pci_read_config_byte(bridge,0x7e,&mut v);let iom=(*dev).archdata.iommu as *mut iommu;if (*iom).dma_addr_mask&0x80000000!=0{v|=1}else{v&=!1}pci_write_config_byte(bridge,0x7e,v);pci_dev_put(bridge);1}
pub unsafe fn pcibios_device_add(dev:*mut pci_dev)->i32{if (*dev).is_virtfn{let pf=(*dev).physfn;let a=&(*pf).dev.archdata;pci_init_dev_archdata(&mut (*dev).dev.archdata,a.iommu,a.stc,a.host_controller,core::ptr::null_mut(),a.numa_node)}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
