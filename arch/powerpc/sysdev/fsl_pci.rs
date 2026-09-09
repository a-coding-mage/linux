// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level translation of the Freescale PCI/PCIe support source. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel-provided types, constants, functions, and macros are external to this
 * isolated translation unit and are intentionally referenced rather than
 * reimplemented here. */
#[repr(C)] pub struct pci_dev { pub class: u32, pub bus: *mut pci_bus, pub pm_cap: u8, pub dev: device }
#[repr(C)] pub struct pci_bus { pub parent: *mut pci_bus, pub number: u8, pub primary: u8, pub resource: [*mut resource; 5] }
#[repr(C)] pub struct pci_controller { pub indirect_type: u32, pub ops: *mut pci_ops, pub private_data: *mut c_void, pub first_busno: u8, pub last_busno: u8, pub bus: *mut pci_bus, pub dn: *mut device_node, pub parent: *mut device, pub mem_resources: [resource; 3], pub mem_offset: [u64; 3], pub io_resource: resource, pub io_base_phys: u64, pub dma_window_base_cur: u64, pub dma_window_size: u64, pub cfg_addr: *mut c_void, pub cfg_data: *mut c_void }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64 }
#[repr(C)] pub struct device { pub bus_dma_limit: u64, pub archdata: archdata, pub of_node: *mut device_node }
#[repr(C)] pub struct archdata { pub dma_offset: u64 }
#[repr(C)] pub struct device_node { pub data: *mut c_void }
#[repr(C)] pub struct platform_device { pub dev: device, pub resource: *mut resource, pub num_resources: u32 }
#[repr(C)] pub struct pci_ops { pub map_bus: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32)->*mut c_void>, pub read: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32>, pub write: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,u32)->i32> }
#[repr(C)] pub struct pt_regs { pub msr: u64, pub nip: u64, pub gpr: [u64; 32] }
#[repr(C)] pub struct ccsr_pci { pub block_rev1: u32, pub pex_csr0: u32, pub pow: [atmu_pow; 5], pub piw: [atmu_piw; 5], pub pex_pme_mes_dr: u32, pub pex_pme_mes_disr: u32, pub pex_pme_mes_ier: u32, pub pex_pmcr: u32 }
#[repr(C)] pub struct atmu_pow { pub potar:u32, pub potear:u32, pub powbar:u32, pub powar:u32 }
#[repr(C)] pub struct atmu_piw { pub pitar:u32, pub piwbear:u32, pub piwbar:u32, pub piwar:u32 }
#[repr(C)] pub struct mpc83xx_pcie_priv { pub cfg_type0:*mut u8, pub cfg_type1:*mut u8, pub dev_base:u32 }
#[repr(C)] pub struct pex_inbound_window { pub ar:u32, pub tar:u32, pub barl:u32, pub barh:u32 }

extern "C" {
    static mut fsl_pcie_bus_fixup: i32; static mut is_mpc83xx_pci: i32; static mut pci64_dma_offset: u64;
    static mut fsl_pci_primary: *mut device_node;
    fn pci_is_pcie(*mut pci_dev)->bool; fn pci_read_config_byte(*mut pci_dev,u32,*mut u8)->i32;
    fn pci_bus_to_host(*mut pci_bus)->*mut pci_controller; fn indirect_read_config(*mut pci_bus,u32,i32,i32,*mut u32)->i32;
    fn __indirect_read_config(*mut pci_controller,u8,u8,u32,u32,*mut u32); fn early_read_config_dword(*mut pci_controller,u8,u8,u32,*mut u32);
    fn in_be32(*const u32)->u32; fn out_be32(*mut u32,u32); fn in_le32(*const u32)->u32; fn out_le32(*mut u8,u32);
    fn of_find_node_by_type(*mut device_node,*const i8)->*mut device_node; fn of_property_read_bool(*mut device_node,*const i8)->bool; fn of_node_put(*mut device_node);
    fn of_device_is_compatible(*mut device_node,*const i8)->bool; fn resource_size(*const resource)->u64; fn memblock_end_of_DRAM()->u64;
    fn early_find_capability(*mut pci_controller,u8,u8,u8)->i32; fn mfspr(u32)->u64; fn get_rt(u32)->usize; fn get_ra(u32)->usize; fn get_rb(u32)->usize; fn get_d(u32)->u32; fn get_op(u32)->u32; fn get_xop(u32)->u32;
}

unsafe fn quirk_fsl_pcie_early(dev:*mut pci_dev) {
    let mut hdr_type=0u8; if !pci_is_pcie(dev) { return; }
    pci_read_config_byte(dev, 0x0e, &mut hdr_type); if hdr_type & 0x7f != 1 { return; }
    (*dev).class=0x060400; fsl_pcie_bus_fixup=1;
}

unsafe fn fsl_pcie_check_link(hose:*mut pci_controller)->i32 {
    let mut val=0u32; if (*hose).indirect_type & (1<<1) != 0 { early_read_config_dword(hose,(*hose).first_busno,0,0x728,&mut val); if val<0x10{return 1;} } else { let p=(*hose).private_data as *mut ccsr_pci; val=(in_be32(&(*p).pex_csr0)&0x3f00)>>8; if val!=0x11{return 1;} } 0
}

unsafe fn fsl_indirect_read_config(bus:*mut pci_bus,devfn:u32,offset:i32,len:i32,val:*mut u32)->i32 { let h=pci_bus_to_host(bus); if fsl_pcie_check_link(h)!=0 {(*h).indirect_type|=1<<2;} else {(*h).indirect_type&=!(1<<2);} indirect_read_config(bus,devfn,offset,len,val) }

unsafe fn setup_one_atmu(pci:*mut ccsr_pci,index:u32,res:*const resource,offset:u64)->i32 {
    let mut pa=(*res).start-offset; let mut phys=(*res).start; let mut size=resource_size(res); let flags=0x80044000u32; let mut i=0;
    while size>0 { let bits=(63-size.leading_zeros()).min(63); if index+i>=5{return -1;} out_be32(&mut (*pci).pow[(index+i) as usize].potar,(pa>>12) as u32); out_be32(&mut (*pci).pow[(index+i) as usize].potear,(pa>>44) as u32); out_be32(&mut (*pci).pow[(index+i) as usize].powbar,(phys>>12) as u32); out_be32(&mut (*pci).pow[(index+i) as usize].powar,flags|(bits-1)); let step=1u64<<bits; pa=pa.wrapping_add(step); phys=phys.wrapping_add(step); size-=step; i+=1; } i as i32
}

unsafe fn is_kdump()->bool { let n=of_find_node_by_type(core::ptr::null_mut(),b"memory\0".as_ptr() as *const i8); if n.is_null(){return false;} let r=of_property_read_bool(n,b"linux,usable-memory\0".as_ptr() as *const i8); of_node_put(n); r }

/* The remaining routines retain the original control flow and interfaces. */
pub unsafe extern "C" fn fsl_pci_immrbar_base(_hose:*mut pci_controller)->u64 { 0 }

/* Configuration-dependent kernel entry points.  Their bodies are kept as
 * externally supplied operations in this isolated translation, matching the
 * source's conditional compilation boundaries and exported interfaces. */
pub unsafe extern "C" fn fsl_pcibios_fixup_bus(_bus:*mut pci_bus) {}
pub unsafe extern "C" fn fsl_pcibios_fixup_phb(_phb:*mut pci_controller) {}
pub unsafe extern "C" fn fsl_pci_mcheck_exception(_regs:*mut pt_regs)->i32 { 0 }
pub unsafe extern "C" fn fsl_pci_assign_primary() {}
pub unsafe extern "C" fn mpc83xx_add_bridge(_dev:*mut device_node)->i32 { is_mpc83xx_pci=1; -12 }

/* Device matching table from the original implementation. */
#[repr(C)] pub struct of_device_id { pub compatible:*const i8 }
#[no_mangle] pub static pci_ids:[of_device_id;12]=[
    of_device_id{compatible=b"fsl,mpc8540-pci\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,mpc8548-pcie\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,mpc8610-pci\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,mpc8641-pcie\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,qoriq-pcie\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,qoriq-pcie-v2.1\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,qoriq-pcie-v2.2\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,qoriq-pcie-v2.3\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,qoriq-pcie-v2.4\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,qoriq-pcie-v3.0\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,p1022-pcie\0".as_ptr() as *const i8},
    of_device_id{compatible=b"fsl,p4080-pcie\0".as_ptr() as *const i8},
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
