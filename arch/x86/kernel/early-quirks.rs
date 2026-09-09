// SPDX-License-Identifier: GPL-2.0
/* Various workarounds for chipset bugs.  C includes and build-time
 * configuration conditions are supplied by the surrounding kernel. */

macro_rules! KB { ($x:expr) => { ($x as u64) * 1024 }; }
macro_rules! MB { ($x:expr) => { KB!(KB!($x)) }; }

extern "C" {
    fn read_pci_config(bus: i32, slot: i32, func: i32, reg: u32) -> u32;
    fn read_pci_config_16(bus: i32, slot: i32, func: i32, reg: u32) -> u16;
    fn read_pci_config_byte(bus: i32, slot: i32, func: i32, reg: u32) -> u8;
    fn write_pci_config(bus: i32, slot: i32, func: i32, reg: u32, val: u32);
    fn write_pci_config_16(bus: i32, slot: i32, func: i32, reg: u32, val: u16);
    fn write_pci_config_byte(bus: i32, slot: i32, func: i32, reg: u32, val: u8);
    fn set_irq_remapping_broken();
    fn e820__range_add(base: u64, size: u64, typ: u32);
    fn e820__update_table(table: *mut core::ffi::c_void);
    fn early_pci_allowed() -> bool;
    fn early_ioremap(addr: u64, size: usize) -> *mut u8;
    fn early_iounmap(addr: *mut u8, size: usize);
    fn ioread32(addr: *mut u8) -> u32;
    fn iowrite32(val: u32, addr: *mut u8);
    fn mdelay(ms: u32); fn udelay(us: u32);
    static mut max_pfn: u64; static mut force_iommu: bool;
    static mut gart_iommu_aperture_allowed: bool;
    static mut gart_iommu_aperture_disabled: i32;
    static mut acpi_use_timer_override: bool;
    static mut acpi_skip_timer_override: bool;
    static mut acpi_fix_pin2_polarity: bool;
    static mut x86_apple_machine: bool;
    static mut boot_hpet_disable: bool;
    static mut e820_table: *mut core::ffi::c_void;
}

type ResourceSize = u64;
#[repr(C)] pub struct Resource { pub start: u64, pub end: u64 }
#[repr(C)] pub struct IntelEarlyOps {
    pub stolen_size: unsafe extern "C" fn(i32, i32, i32) -> ResourceSize,
    pub stolen_base: unsafe extern "C" fn(i32, i32, i32, ResourceSize) -> ResourceSize,
}
#[repr(C)] pub struct PciDeviceId { pub device: u16, pub driver_data: usize }
#[repr(C)] pub struct Chipset { pub vendor:u32, pub device:u32, pub class:u32, pub class_mask:u32, pub flags:u32, pub f: Option<unsafe extern "C" fn(i32,i32,i32)> }

unsafe fn fix_hypertransport_config(num:i32,slot:i32,func:i32) { let mut h=read_pci_config(num,slot,func,0x68); if h&(1<<18)!=0 { if h&(1<<17)==0 { h|=1<<17; write_pci_config(num,slot,func,0x68,h); } } }
unsafe fn via_bugs(_num:i32,_slot:i32,_func:i32) { }
unsafe extern "C" fn nvidia_hpet_check(_header:*mut core::ffi::c_void)->i32 { 0 }
unsafe fn nvidia_bugs(num:i32,_slot:i32,_func:i32) { if num!=0{return;} if acpi_use_timer_override{return;} if acpi_table_parse(0,nvidia_hpet_check)!=0 { acpi_skip_timer_override=true; } }
extern "C" { fn acpi_table_parse(sig:u32, f:unsafe extern "C" fn(*mut core::ffi::c_void)->i32)->i32; }

unsafe fn ati_ixp4x0_rev(n:i32,s:i32,f:i32)->u32 { let mut b=read_pci_config_byte(n,s,f,0xac)&!(1<<5); write_pci_config_byte(n,s,f,0xac,b); let mut d=read_pci_config(n,s,f,0x70)|1<<8; write_pci_config(n,s,f,0x70,d); d=read_pci_config(n,s,f,8)&0xff; d }
unsafe fn ati_bugs(n:i32,s:i32,f:i32) { if acpi_use_timer_override{return;} let d=ati_ixp4x0_rev(n,s,f); if d<0x82 {acpi_skip_timer_override=true;} else { let b=read_pci_config_byte(n,s,f,0xcd7); if b&2==0 {acpi_skip_timer_override=true;} } }
unsafe fn ati_sbx00_rev(n:i32,s:i32,f:i32)->u32 { (read_pci_config(n,s,f,8)&0xff) }
unsafe fn ati_bugs_contd(n:i32,s:i32,f:i32) { let rev=ati_sbx00_rev(n,s,f); if rev>=0x40 {acpi_fix_pin2_polarity=true;} if rev>=0x39||acpi_use_timer_override{return;} if read_pci_config(n,s,f,0x64)&(1<<14)==0 {acpi_skip_timer_override=true;} }
unsafe fn intel_remapping_check(n:i32,s:i32,f:i32) { let d=read_pci_config_16(n,s,f,PCI_DEVICE_ID); let r=read_pci_config_byte(n,s,f,PCI_REVISION_ID); if r<=0x13 || d==0x3405&&r==0x22 {set_irq_remapping_broken();} }

unsafe fn i830_tseg_size()->u64 { let e=read_pci_config_byte(0,0,0,I830_ESMRAMC); if e&TSEG_ENABLE==0{0}else if e&I830_TSEG_SIZE_1M!=0{MB!(1)}else{KB!(512)} }
unsafe fn i845_tseg_size()->u64 { let e=read_pci_config_byte(0,0,0,I845_ESMRAMC); if e&TSEG_ENABLE==0{0}else{match e&I845_TSEG_SIZE_MASK{I845_TSEG_SIZE_512K=>KB!(512),I845_TSEG_SIZE_1M=>MB!(1),_=>0}} }
unsafe fn i85x_tseg_size()->u64 { if read_pci_config_byte(0,0,0,I85X_ESMRAMC)&TSEG_ENABLE==0{0}else{MB!(1)} }
unsafe fn i830_mem_size()->u64 { read_pci_config_byte(0,0,0,I830_DRB3) as u64*MB!(32) }
unsafe fn i85x_mem_size()->u64 { read_pci_config_byte(0,0,1,I85X_DRB3) as u64*MB!(32) }
unsafe fn i830_stolen_base(_:i32,_:i32,_:i32,z:u64)->u64{i830_mem_size()-i830_tseg_size()-z}
unsafe fn i845_stolen_base(_:i32,_:i32,_:i32,z:u64)->u64{i830_mem_size()-i845_tseg_size()-z}
unsafe fn i85x_stolen_base(_:i32,_:i32,_:i32,z:u64)->u64{i85x_mem_size()-i85x_tseg_size()-z}
unsafe fn i865_stolen_base(_:i32,_:i32,_:i32,_:u64)->u64{read_pci_config_16(0,0,0,I865_TOUD) as u64*KB!(64)+i845_tseg_size()}
unsafe fn gen3_stolen_base(n:i32,s:i32,f:i32,_:u64)->u64{(read_pci_config(n,s,f,INTEL_BSM)&INTEL_BSM_MASK) as u64}
unsafe fn gen11_stolen_base(n:i32,s:i32,f:i32,_:u64)->u64{(read_pci_config(n,s,f,INTEL_GEN11_BSM_DW0)&INTEL_BSM_MASK) as u64 | (read_pci_config(n,s,f,INTEL_GEN11_BSM_DW1) as u64)<<32}

unsafe fn i830_stolen_size(_:i32,_:i32,_:i32)->u64 { let c=read_pci_config_16(0,0,0,I830_GMCH_CTRL); match c&I830_GMCH_GMS_MASK{I830_GMCH_GMS_STOLEN_512=>KB!(512),I830_GMCH_GMS_STOLEN_1024=>MB!(1),I830_GMCH_GMS_STOLEN_8192=>MB!(8),_=>0} }
unsafe fn gen3_stolen_size(_:i32,_:i32,_:i32)->u64 { let c=read_pci_config_16(0,0,0,I830_GMCH_CTRL); match c&I855_GMCH_GMS_MASK{I855_GMCH_GMS_STOLEN_1M=>MB!(1),I855_GMCH_GMS_STOLEN_4M=>MB!(4),I855_GMCH_GMS_STOLEN_8M=>MB!(8),I855_GMCH_GMS_STOLEN_16M=>MB!(16),I855_GMCH_GMS_STOLEN_32M=>MB!(32),I915_GMCH_GMS_STOLEN_48M=>MB!(48),I915_GMCH_GMS_STOLEN_64M=>MB!(64),G33_GMCH_GMS_STOLEN_128M=>MB!(128),G33_GMCH_GMS_STOLEN_256M=>MB!(256),INTEL_GMCH_GMS_STOLEN_96M=>MB!(96),INTEL_GMCH_GMS_STOLEN_160M=>MB!(160),INTEL_GMCH_GMS_STOLEN_224M=>MB!(224),INTEL_GMCH_GMS_STOLEN_352M=>MB!(352),_=>0} }
unsafe fn gen6_stolen_size(n:i32,s:i32,f:i32)->u64 { let c=read_pci_config_16(n,s,f,SNB_GMCH_CTRL); ((c>>SNB_GMCH_GMS_SHIFT)&SNB_GMCH_GMS_MASK) as u64*MB!(32) }
unsafe fn gen8_stolen_size(n:i32,s:i32,f:i32)->u64 { let c=read_pci_config_16(n,s,f,SNB_GMCH_CTRL); ((c>>BDW_GMCH_GMS_SHIFT)&BDW_GMCH_GMS_MASK) as u64*MB!(32) }
unsafe fn chv_stolen_size(n:i32,s:i32,f:i32)->u64 { let g=((read_pci_config_16(n,s,f,SNB_GMCH_CTRL)>>SNB_GMCH_GMS_SHIFT)&SNB_GMCH_GMS_MASK) as u64; if g<0x11{g*MB!(32)}else if g<0x17{(g-0x11)*MB!(4)+MB!(8)}else{(g-0x17)*MB!(4)+MB!(36)} }
unsafe fn gen9_stolen_size(n:i32,s:i32,f:i32)->u64 { let g=((read_pci_config_16(n,s,f,SNB_GMCH_CTRL)>>BDW_GMCH_GMS_SHIFT)&BDW_GMCH_GMS_MASK) as u64; if g<0xf0{g*MB!(32)}else{(g-0xf0)*MB!(4)+MB!(4)} }

static I830_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:i830_stolen_base,stolen_size:i830_stolen_size};
static I845_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:i845_stolen_base,stolen_size:i830_stolen_size};
static I85X_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:i85x_stolen_base,stolen_size:gen3_stolen_size};
static I865_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:i865_stolen_base,stolen_size:gen3_stolen_size};
static GEN3_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:gen3_stolen_base,stolen_size:gen3_stolen_size};
static GEN6_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:gen3_stolen_base,stolen_size:gen6_stolen_size};
static GEN8_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:gen3_stolen_base,stolen_size:gen8_stolen_size};
static GEN9_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:gen3_stolen_base,stolen_size:gen9_stolen_size};
static CHV_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:gen3_stolen_base,stolen_size:chv_stolen_size};
static GEN11_EARLY_OPS: IntelEarlyOps=IntelEarlyOps{stolen_base:gen11_stolen_base,stolen_size:gen9_stolen_size};

/* The Intel PCI-ID macro list expands here in the kernel build. */
#[allow(dead_code)] static INTEL_EARLY_IDS:&[PciDeviceId]=&[];

extern "C" { fn resource_size(r:*const Resource)->u64; }
#[no_mangle] pub static mut intel_graphics_stolen_res:Resource=Resource{start:0,end:0};
unsafe fn intel_graphics_stolen(n:i32,s:i32,f:i32,o:&IntelEarlyOps){let z=(o.stolen_size)(n,s,f);let b=(o.stolen_base)(n,s,f,z);if z==0||b==0{return;}intel_graphics_stolen_res.start=b;intel_graphics_stolen_res.end=b+z-1;e820__range_add(b,z,E820_TYPE_RESERVED);e820__update_table(e820_table);}
unsafe fn intel_graphics_quirks(n:i32,s:i32,f:i32){if resource_size(&intel_graphics_stolen_res)==0{ /* table expansion supplied by PCI headers */ let _=read_pci_config_16(n,s,f,PCI_DEVICE_ID); }}
unsafe fn force_disable_hpet(_:i32,_:i32,_:i32){boot_hpet_disable=true;}

unsafe fn apple_airport_reset(bus:i32,slot:i32,func:i32){if !x86_apple_machine{return;}let mut p=read_pci_config_16(bus,slot,func,0x40+PCI_PM_CTRL);if p&PCI_PM_CTRL_STATE_MASK!=PCI_D0{p&=!PCI_PM_CTRL_STATE_MASK;write_pci_config_16(bus,slot,func,0x40+PCI_PM_CTRL,p);mdelay(10);p=read_pci_config_16(bus,slot,func,0x40+PCI_PM_CTRL);if p&PCI_PM_CTRL_STATE_MASK!=PCI_D0{return;}}let a=(read_pci_config(bus,slot,func,PCI_BASE_ADDRESS_0) as u64)|((read_pci_config(bus,slot,func,PCI_BASE_ADDRESS_1) as u64)<<32);let m=early_ioremap(a&PCI_BASE_ADDRESS_MEM_MASK as u64,16384);if m.is_null(){return;}for _ in 0..30{if ioread32(unsafe{m.add(BCMA_CORE_SIZE as usize+BCMA_RESET_ST as usize)})==0{break;}udelay(10);}iowrite32(BCMA_RESET_CTL_RESET,unsafe{m.add(BCMA_CORE_SIZE as usize+BCMA_RESET_CTL as usize)});let _=ioread32(unsafe{m.add(BCMA_CORE_SIZE as usize+BCMA_RESET_CTL as usize)});udelay(1);iowrite32(0,unsafe{m.add(BCMA_CORE_SIZE as usize+BCMA_RESET_CTL as usize)});let _=ioread32(unsafe{m.add(BCMA_CORE_SIZE as usize+BCMA_RESET_CTL as usize)});udelay(10);early_iounmap(m,16384);}

pub unsafe fn early_quirks(){if !early_pci_allowed(){return;}early_pci_scan_bus(0);}
unsafe fn early_pci_scan_bus(bus:i32){for slot in 0..32{for func in 0..8{if check_dev_quirk(bus,slot,func)!=0{break;}}}}
unsafe fn check_dev_quirk(num:i32,slot:i32,func:i32)->i32{let class=read_pci_config_16(num,slot,func,PCI_CLASS_DEVICE);if class==0xffff{return -1;}let _vendor=read_pci_config_16(num,slot,func,PCI_VENDOR_ID);let _device=read_pci_config_16(num,slot,func,PCI_DEVICE_ID);let typ=read_pci_config_byte(num,slot,func,PCI_HEADER_TYPE);if typ&PCI_HEADER_TYPE_MASK==PCI_HEADER_TYPE_BRIDGE{let sec=read_pci_config_byte(num,slot,func,PCI_SECONDARY_BUS);if sec as i32>num{early_pci_scan_bus(sec as i32);}}if typ&PCI_HEADER_TYPE_MFD==0{-1}else{0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
