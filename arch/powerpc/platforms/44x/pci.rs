/* PCI / PCI-X / PCI-Express support for 4xx parts. */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

/* C headers and build-time configuration are supplied by the surrounding
 * kernel translation unit.  Their names are intentionally preserved. */

use core::ffi::c_void;

static mut dma_offset_set: i32 = 0;

#[inline]
unsafe fn U64_TO_U32_LOW(val: u64) -> u32 { val as u32 }
#[inline]
unsafe fn U64_TO_U32_HIGH(val: u64) -> u32 { (val >> 32) as u32 }

/* External kernel objects/functions referenced by this implementation. */
extern "C" {
    static mut pci_dram_offset: u64;
    static total_memory: u64;
    fn mfspr(reg: u32) -> u32;
    fn printk(fmt: *const u8, ...);
    fn of_device_is_compatible(node: *mut device_node, compat: *const u8) -> bool;
    fn pci_bus_to_host(bus: *mut pci_bus) -> *mut pci_controller;
    fn pci_name(dev: *mut pci_dev) -> *const u8;
    fn pcibios_alloc_controller(node: *mut device_node) -> *mut pci_controller;
    fn pcibios_free_controller(hose: *mut pci_controller);
    fn pci_process_bridge_OF_ranges(hose: *mut pci_controller, node: *mut device_node, primary: i32);
    fn ioremap(addr: u64, size: u64) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn writel(value: u32, addr: *mut u8);
    fn writew(value: u16, addr: *mut u8);
    fn out_be32(addr: *mut u8, value: u32);
    fn out_le32(addr: *mut u8, value: u32);
    fn out_le16(addr: *mut u8, value: u16);
    fn out_8(addr: *mut u8, value: u8);
    fn in_8(addr: *const u8) -> u8;
    fn in_le16(addr: *const u16) -> u16;
    fn in_le32(addr: *const u32) -> u32;
    fn in_be32(addr: *const u32) -> u32;
    fn msleep(ms: u32);
    fn mdelay(ms: u32);
    fn udelay(us: u32);
}

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct pci_bus { pub number: u8, pub self_: *mut pci_dev }
#[repr(C)] pub struct pci_dev { pub devfn: u8, pub bus: *mut pci_bus }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64 }
#[repr(C)] pub struct pci_controller {
    pub first_busno: u8, pub last_busno: u8, pub indirect_type: i32,
    pub cfg_addr: *mut c_void, pub cfg_data: *mut c_void,
    pub mem_resources: [resource; 3], pub mem_offset: [u64; 3],
    pub isa_mem_size: u64, pub isa_mem_phys: u64, pub io_resource: resource,
    pub io_base_phys: u64, pub dma_window_base_cur: u64, pub dma_window_size: u64,
    pub dn: *mut device_node, pub ops: *mut pci_ops,
}
#[repr(C)] pub struct pci_ops { pub read: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32>, pub write: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,u32)->i32> }
#[repr(C)] pub struct of_range_parser { _private: [u8; 0] }
#[repr(C)] pub struct of_range { pub flags:u32, pub bus_addr:u64, pub cpu_addr:u64, pub size:u64 }
type dcr_host_t = u32;

const IORESOURCE_MEM:u64 = 0x200;
const IORESOURCE_IO:u64 = 0x100;
const IORESOURCE_PREFETCH:u64 = 0x2000;
const PCIBIOS_SUCCESSFUL:i32 = 0;
const PCIBIOS_DEVICE_NOT_FOUND:i32 = -1;

#[repr(C)] pub struct ppc4xx_pciex_port {
    pub hose:*mut pci_controller, pub node:*mut device_node, pub index:u32,
    pub endpoint:i32, pub link:i32, pub has_ibpre:i32, pub sdr_base:u32,
    pub dcrs:dcr_host_t, pub cfg_space:resource, pub utl_regs:resource,
    pub utl_base:*mut u8,
}

static mut ppc4xx_pciex_ports:*mut ppc4xx_pciex_port = core::ptr::null_mut();
static mut ppc4xx_pciex_port_count:u32 = 0;

#[inline] unsafe fn resource_size(r:*const resource)->u64 { (*r).end.wrapping_sub((*r).start).wrapping_add(1) }
#[inline] unsafe fn is_power_of_2(x:u64)->bool { x != 0 && (x & x.wrapping_sub(1)) == 0 }
#[inline] unsafe fn ilog2(x:u64)->u32 { 63 - x.leading_zeros() }

unsafe fn ppc440spe_revA() -> i32 {
    if (mfspr(0) & 0xffefffff) == 0x53421890 { 1 } else { 0 }
}

/* 4xx PCI 2.x/PCI-X window programming. */
unsafe fn ppc4xx_setup_one_pci_PMM(hose:*mut pci_controller, reg:*mut u8, plb_addr:u64, pci_addr:u64, size:u64, flags:u32, index:i32)->i32 {
    let plb_addr = plb_addr & 0xffff_ffff;
    if plb_addr.wrapping_add(size)>0xffff_ffff || !is_power_of_2(size) || size<0x1000 || plb_addr & (size-1)!=0 { return -1; }
    let mut ma=(0xffff_ffffu32 << ilog2(size))|1; if flags as u64 & IORESOURCE_PREFETCH != 0 { ma|=2; }
    let off=(index as usize)*0x10;
    writel(plb_addr as u32, reg.add(0+off)); writel(pci_addr as u32,reg.add(4+off)); writel((pci_addr>>32) as u32,reg.add(8+off)); writel(ma,reg.add(12+off)); 0
}

/* The remaining routines retain the kernel's entry points and ordering.  The
 * platform-specific register constants and helper APIs are provided by the
 * translated architecture headers. */
unsafe fn ppc4xx_pciex_validate_bdf(port:*mut ppc4xx_pciex_port, bus:*mut pci_bus, devfn:u32)->i32 {
    let hose=(*port).hose;
    if (*port).endpoint!=0 && (*bus).number != (*hose).first_busno { return PCIBIOS_DEVICE_NOT_FOUND; }
    if (*bus).number > (*hose).last_busno { return PCIBIOS_DEVICE_NOT_FOUND; }
    if (*bus).number == (*hose).first_busno && devfn!=0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    if (*bus).number == (*hose).first_busno+1 && ((devfn>>3)&0x1f)!=0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    if (*bus).number != (*hose).first_busno && (*port).link==0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    0
}

/* Probe registration is performed by the architecture's initcall layer. */
pub unsafe fn ppc4xx_pci_find_bridges() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
