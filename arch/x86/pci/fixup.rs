// SPDX-License-Identifier: GPL-2.0
/* Exceptions for specific devices. Usually work-arounds for fatal design flaws. */

// Kernel dependencies supplied by the surrounding tree.
use core::ptr;

extern "C" {
    fn pci_read_config_byte(d: *mut pci_dev, where_: u32, val: *mut u8) -> i32;
    fn pci_read_config_word(d: *mut pci_dev, where_: u32, val: *mut u16) -> i32;
    fn pci_read_config_dword(d: *mut pci_dev, where_: u32, val: *mut u32) -> i32;
    fn pci_write_config_byte(d: *mut pci_dev, where_: u32, val: u8) -> i32;
    fn pci_write_config_word(d: *mut pci_dev, where_: u32, val: u16) -> i32;
    fn pci_write_config_dword(d: *mut pci_dev, where_: u32, val: u32) -> i32;
    fn pcibios_scan_root(bus: u8);
    fn raw_pci_read(domain: i32, bus: u8, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32;
    fn raw_pci_write(domain: i32, bus: u8, devfn: u32, where_: i32, size: i32, value: u32) -> i32;
    fn pci_domain_nr(bus: *mut pci_bus) -> i32;
    fn pci_bus_set_ops(bus: *mut pci_bus, ops: *const pci_ops);
    fn pci_find_host_bridge(bus: *mut pci_bus) -> *mut pci_host_bridge;
    fn pcie_get_readrq(dev: *mut pci_dev) -> i32;
    fn pcie_set_readrq(dev: *mut pci_dev, rq: i32);
    fn pci_is_bridge(dev: *mut pci_dev) -> bool;
    fn pci_disable_rom(dev: *mut pci_dev);
    fn release_resource(res: *mut resource);
    fn request_mem_region(start: u64, size: u64, name: *const i8) -> *mut resource;
    fn request_resource_conflict(root: *mut resource, res: *mut resource) -> *mut resource;
    fn pci_bus_add_resource(bus: *mut pci_bus, res: *mut resource);
    fn dmi_check_system(table: *const dmi_system_id) -> i32;
    fn dmi_match(which: u32, value: *const i8) -> bool;
    fn vga_default_device() -> *mut pci_dev;
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> u64;
    fn is_vmd(bus: *mut pci_bus) -> bool;
    fn pci_d3cold_disable(dev: *mut pci_dev);
    fn pcie_find_root_port(dev: *mut pci_dev) -> *mut pci_dev;
    fn pm_suspend_in_progress() -> bool;
    fn amd_smn_read(node: u32, address: u32, data: *mut u32) -> i32;
    fn amd_smn_write(node: u32, address: u32, data: u32) -> i32;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub name: *const i8, pub flags: u64, pub parent: *mut resource }
#[repr(C)] pub struct pci_bus { pub number: u8, pub self_: *mut pci_dev, pub parent: *mut pci_bus, pub ops: *const pci_ops, pub devices: list_head, pub dev: device, pub bus_flags: u32 }
#[repr(C)] pub struct pci_dev { pub dev: device, pub device: u16, pub vendor: u16, pub revision: u8, pub irq: i32, pub resource: [resource; 6], pub subordinate: *mut pci_bus, pub bus: *mut pci_bus, pub devfn: u32, pub pcie_cap: u16, pub transparent: u8, pub non_compliant_bars: u8, pub pme_support: u32, pub current_state: u32, pub subsystem_vendor: u16, pub subsystem_device: u16, pub dev_flags: u32, pub pm_cap: u8 }
#[repr(C)] pub struct pci_host_bridge { pub no_ext_tags: u8, pub enable_device: Option<unsafe extern "C" fn(*mut pci_host_bridge, *mut pci_dev) -> i32> }
#[repr(C)] pub struct pci_ops { pub read: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32>, pub write: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,u32)->i32> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dmi_system_id { pub ident: *const i8, pub matches: [u8; 0] }

extern "C" { static mut pcibios_last_bus: i32; static mut pcibios_max_latency: u8; static mut hpet_address: u64; static mut pci_probe: u32; static mut high_memory: *mut u8; static mut iomem_resource: resource; }

const PCI_BASE_ADDRESS_SPACE_IO: u64 = 1;
const VIA_8363_KL133_REVISION_ID: u8 = 0x81;
const VIA_8363_KM133_REVISION_ID: u8 = 0x84;
const MAX_PCIEROOT: usize = 6;
static mut quirk_aspm_offset: [u8; MAX_PCIEROOT << 3] = [0; MAX_PCIEROOT << 3];
const fn get_index(a: u16, b: u32) -> usize { (((a - PCI_DEVICE_ID_INTEL_MCH_PA) << 3) as usize) + ((b & 7) as usize) }

unsafe fn pci_fixup_i450nx(d: *mut pci_dev) { let mut reg = 0xd0u32; for pxb in 0..2 { let mut busno=0; let mut suba=0; let mut subb=0; pci_read_config_byte(d,reg,&mut busno); reg+=1; pci_read_config_byte(d,reg,&mut suba); reg+=1; pci_read_config_byte(d,reg,&mut subb); reg+=1; if busno != 0 { pcibios_scan_root(busno); } if suba < subb { pcibios_scan_root(suba + 1); } let _ = pxb; } pcibios_last_bus = -1; }
unsafe fn pci_fixup_i450gx(d: *mut pci_dev) { let mut busno=0; pci_read_config_byte(d,0x4a,&mut busno); pcibios_scan_root(busno); pcibios_last_bus=-1; }
unsafe fn pci_fixup_umc_ide(d: *mut pci_dev) { for i in 0..4 { (*d).resource[i].flags |= PCI_BASE_ADDRESS_SPACE_IO; } }
unsafe fn pci_fixup_latency(_: *mut pci_dev) { pcibios_max_latency=32; }
unsafe fn pci_fixup_piix4_acpi(d: *mut pci_dev) { (*d).irq=9; }

unsafe fn pci_fixup_via_northbridge_bug(d: *mut pci_dev) { let mut where_=0x55u32; let mut mask=0x1fu8; if (*d).device==PCI_DEVICE_ID_VIA_8367_0 { pci_write_config_byte(d,PCI_LATENCY_TIMER,0); where_=0x95; } else if (*d).device==PCI_DEVICE_ID_VIA_8363_0 && ((*d).revision==VIA_8363_KL133_REVISION_ID || (*d).revision==VIA_8363_KM133_REVISION_ID) { mask=0x3f; } let mut v=0; pci_read_config_byte(d,where_,&mut v); if v & !mask != 0 { v &= mask; pci_write_config_byte(d,where_,v); } }
unsafe fn pci_fixup_transparent_bridge(dev: *mut pci_dev) { if (*dev).device & 0xff00 == 0x2400 { (*dev).transparent=1; } }
unsafe fn pci_fixup_nforce2(dev: *mut pci_dev) { let mut val=0; pci_read_config_dword(dev,0x6c,&mut val); if val & 0x00ff0000 != 0x00010000 { pci_write_config_dword(dev,0x6c,(val & 0xff00ffff)|0x00010000); } }

unsafe extern "C" fn quirk_pcie_aspm_read(bus:*mut pci_bus,devfn:u32,where_:i32,size:i32,value:*mut u32)->i32 { raw_pci_read(pci_domain_nr(bus),(*bus).number,devfn,where_,size,value) }
unsafe extern "C" fn quirk_pcie_aspm_write(bus:*mut pci_bus,devfn:u32,where_:i32,size:i32,mut value:u32)->i32 { let offset=quirk_aspm_offset[get_index((*(*bus).self_).device,devfn)]; if offset!=0 && where_ as u8==offset { value &= !PCI_EXP_LNKCTL_ASPMC; } raw_pci_write(pci_domain_nr(bus),(*bus).number,devfn,where_,size,value) }
static mut quirk_pcie_aspm_ops: pci_ops = pci_ops { read:Some(quirk_pcie_aspm_read), write:Some(quirk_pcie_aspm_write) };
unsafe fn pcie_rootport_aspm_quirk(pdev:*mut pci_dev) { let pbus=(*pdev).subordinate; if pbus.is_null() || (*pdev).device<PCI_DEVICE_ID_INTEL_MCH_PA || (*pdev).device>PCI_DEVICE_ID_INTEL_MCH_PC1 { return; } if (*pbus).devices.next==ptr::null_mut() { for i in get_index((*pdev).device,0)..=get_index((*pdev).device,7) { quirk_aspm_offset[i]=0; } pci_bus_set_ops(pbus,(*pbus).parent.cast::<pci_bus>().as_ref().map_or(ptr::null(),|b|b.ops)); } else { pci_bus_set_ops(pbus,&quirk_pcie_aspm_ops); } }

unsafe extern "C" fn limit_mrrs_to_128(_: *mut pci_host_bridge,pdev:*mut pci_dev)->i32 { if pcie_get_readrq(pdev)>128 { pcie_set_readrq(pdev,128); } 0 }
unsafe fn pci_xeon_x2_bifurc_quirk(pdev:*mut pci_dev) { let bridge=pci_find_host_bridge((*pdev).bus); let mut linkcap=0; pci_read_config_dword(pdev,PCI_EXP_LNKCAP,&mut linkcap); if ((linkcap & PCI_EXP_LNKCAP_MLW)>>4)!=2 { return; } (*bridge).no_ext_tags=1; (*bridge).enable_device=Some(limit_mrrs_to_128); }

unsafe fn pci_fixup_video(pdev:*mut pci_dev) { let mut bus=(*pdev).bus; while !bus.is_null() { let bridge=(*bus).self_; if !bridge.is_null() && pci_is_bridge(bridge) { let mut config=0; pci_read_config_word(bridge,PCI_BRIDGE_CONTROL,&mut config); if config & PCI_BRIDGE_CTL_VGA==0 { return; } } bus=(*bus).parent; } if vga_default_device().is_null() || pdev==vga_default_device() { let mut config=0; pci_read_config_word(pdev,PCI_COMMAND,&mut config); if config & (PCI_COMMAND_IO|PCI_COMMAND_MEMORY)!=0 { let res=&mut (*pdev).resource[PCI_ROM_RESOURCE as usize]; pci_disable_rom(pdev); if !(*res).parent.is_null() { release_resource(res); } res.start=0xc0000; res.end=res.start+0x20000-1; res.flags=IORESOURCE_MEM|IORESOURCE_ROM_SHADOW|IORESOURCE_PCI_FIXED; } } }

static mut toshiba_line_size:u16=0;
unsafe fn pci_pre_fixup_toshiba_ohci1394(dev:*mut pci_dev) { (*dev).current_state=PCI_D3cold; pci_read_config_word(dev,PCI_CACHE_LINE_SIZE,&mut toshiba_line_size); }
unsafe fn pci_post_fixup_toshiba_ohci1394(dev:*mut pci_dev) { pci_write_config_word(dev,PCI_CACHE_LINE_SIZE,toshiba_line_size); let irq=&mut (*dev).irq as *mut i32 as *mut u8; pci_read_config_byte(dev,PCI_INTERRUPT_LINE,irq); pci_write_config_dword(dev,PCI_BASE_ADDRESS_0,pci_resource_start(dev,0) as u32); pci_write_config_dword(dev,PCI_BASE_ADDRESS_1,pci_resource_start(dev,1) as u32); }
unsafe fn pci_early_fixup_cyrix_5530(dev:*mut pci_dev) { let mut r=0; pci_read_config_byte(dev,0x42,&mut r); pci_write_config_byte(dev,0x42,r&0xfd); }
unsafe fn pci_siemens_interrupt_controller(dev:*mut pci_dev) { (*dev).resource[0].flags|=IORESOURCE_PCI_FIXED; }
unsafe fn sb600_disable_hpet_bar(dev:*mut pci_dev) { let mut val=0; pci_read_config_byte(dev,0x08,&mut val); if val<0x2f { outb(0x55,0xcd6); val=inb(0xcd7); outb(0x55,0xcd6); outb(val|0x80,0xcd7); } }
unsafe fn pci_fixup_msi_k8t_onboard_sound(dev:*mut pci_dev) { let mut val=0; pci_read_config_byte(dev,0x50,&mut val); if val&0x40!=0 { pci_write_config_byte(dev,0x50,val&!0x40); pci_read_config_byte(dev,0x50,&mut val); } }
unsafe fn twinhead_reserve_killing_zone(dev:*mut pci_dev) { if (*dev).subsystem_vendor==0x14ff && (*dev).subsystem_device==0xa003 { let _=request_mem_region(0xffb00000,0x100000, b"twinhead\0".as_ptr() as *const i8); } }
unsafe fn pci_invalid_bar(dev:*mut pci_dev) { (*dev).non_compliant_bars=1; }
unsafe fn pci_fixup_amd_ehci_pme(dev:*mut pci_dev) { (*dev).pme_support &= !((PCI_PM_CAP_PME_D3hot|PCI_PM_CAP_PME_D3cold)>>PCI_PM_CAP_PME_SHIFT); }
unsafe fn pci_fixup_amd_fch_xhci_pme(dev:*mut pci_dev) { (*dev).pme_support &= !(PCI_PM_CAP_PME_D0>>PCI_PM_CAP_PME_SHIFT); }
unsafe fn quirk_intel_th_dnv(dev:*mut pci_dev) { let r=&mut (*dev).resource[4]; if r.end==r.start+0x7ff { r.start=0; r.end=0x3fffff; r.flags|=IORESOURCE_UNSET; } }
unsafe fn quirk_apple_mbp_poweroff(pdev:*mut pci_dev) { if (*pdev).bus.is_null() || (*pdev).bus.as_ref().unwrap().number!=0 || (*pdev).devfn!=PCI_DEVFN(0x1c,0) { return; } let _=request_mem_region(0x7fa00000,0x200000,b"MacBook Pro poweroff workaround\0".as_ptr() as *const i8); }
unsafe fn quirk_no_aersid(pdev:*mut pci_dev) { if is_vmd((*pdev).bus) && pci_is_root_bus((*pdev).bus) { (*(*pdev).bus).bus_flags |= PCI_BUS_FLAGS_NO_AERSID; } }
unsafe fn quirk_clear_strap_no_soft_reset_dev2_f0(dev:*mut pci_dev) { let mut data=0; if amd_smn_read(0,0x10136008,&mut data)==0 { data &= !0x80; let _=amd_smn_write(0,0x10136008,data); } else { let _=dev; } }
unsafe fn pci_amd_enable_64bit_bar(dev:*mut pci_dev) { if pci_probe & PCI_BIG_ROOT_WINDOW==0{return;} let mut base=0; let mut high=0; let mut i=0; while i<8 { pci_read_config_dword(dev,0x80+i*8,&mut base); pci_read_config_dword(dev,0x180+i*4,&mut high); if base & 3==0 {break;} base=(base>>8)|(high<<24); if base>0x10000{return;} i+=1; } if i==8{return;} base=((0xbd00000000u64>>8)&0xffffff00) as u32|3; let limit=((0xfd00000000u64>>8)&0xffffff00) as u32; high=((0xbd00000000u64>>40)&0xff) as u32|(((0xfd00000000u64>>40)<<16)&0xff0000) as u32; pci_write_config_dword(dev,0x180+i*4,high); pci_write_config_dword(dev,0x84+i*8,limit); pci_write_config_dword(dev,0x80+i*8,base); }
unsafe fn rs690_fix_64bit_dma(pdev:*mut pci_dev) { let mut val=0; pci_write_config_dword(pdev,0xa8,0x30); pci_read_config_dword(pdev,0xac,&mut val); if val==0 { pci_write_config_dword(pdev,0xa8,0x31|0x100); pci_write_config_dword(pdev,0xac,1); pci_write_config_dword(pdev,0xa8,0x30|0x100); pci_write_config_dword(pdev,0xac,1); } }

static mut prev_cap:u16=0; static mut l1ss_cap:u16=0; static mut prev_header:u32=0; static mut l1ss_header:u32=0;
unsafe fn chromeos_save_apl_pci_l1ss_capability(dev:*mut pci_dev) { let mut pos=PCI_CFG_SPACE_SIZE; let mut prev=0u16; let mut pheader=0; while pos!=0 { let mut header=0; pci_read_config_dword(dev,pos as u32,&mut header); if PCI_EXT_CAP_ID(header)==PCI_EXT_CAP_ID_L1SS { prev_cap=prev; prev_header=pheader; l1ss_cap=pos as u16; l1ss_header=header; return; } prev=pos as u16; pheader=header; pos=PCI_EXT_CAP_NEXT(header) as usize; } }
unsafe fn chromeos_fixup_apl_pci_l1ss_capability(dev:*mut pci_dev) { if prev_cap==0||prev_header==0||l1ss_cap==0||l1ss_header==0{return;} let mut header=0; pci_read_config_dword(dev,l1ss_cap as u32,&mut header); if header!=l1ss_header { pci_write_config_dword(dev,l1ss_cap as u32,l1ss_header); } pci_read_config_dword(dev,prev_cap as u32,&mut header); if header!=prev_header { pci_write_config_dword(dev,prev_cap as u32,prev_header); } }

unsafe fn asus_disable_nvme_d3cold(pdev:*mut pci_dev) { if dmi_check_system(ptr::null())>0 { pci_d3cold_disable(pdev); } }
unsafe fn amd_rp_pme_suspend(dev:*mut pci_dev) { if !pm_suspend_in_progress(){return;} let rp=pcie_find_root_port(dev); if rp.is_null()||(*rp).pm_cap==0{return;} (*rp).pme_support &= !((PCI_PM_CAP_PME_D3hot|PCI_PM_CAP_PME_D3cold)>>PCI_PM_CAP_PME_SHIFT); }
unsafe fn amd_rp_pme_resume(dev:*mut pci_dev) { let rp=pcie_find_root_port(dev); if rp.is_null()||(*rp).pm_cap==0{return;} let mut pmc=0; pci_read_config_word(rp,(*rp).pm_cap as u32+PCI_PM_PMC,&mut pmc); (*rp).pme_support=(pmc as u32 & PCI_PM_CAP_PME_MASK)>>PCI_PM_CAP_PME_SHIFT; }
unsafe fn quirk_tuxeo_rp_d3(pdev:*mut pci_dev) { let root=pcie_find_root_port(pdev); if !root.is_null(){(*root).dev_flags|=PCI_DEV_FLAGS_NO_D3;} }

// DECLARE_PCI_FIXUP_* registrations from fixup.c are retained by the build integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
