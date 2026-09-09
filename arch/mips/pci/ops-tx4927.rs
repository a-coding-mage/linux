// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of ops-tx4927.c. External kernel types, constants, and
 * functions are supplied by the surrounding translation unit. */

use core::ffi::c_void;

extern "C" {
    static mut txx9_pci_err_action: i32;
    fn get_irq_regs() -> *mut pt_regs;
    fn iob();
    fn console_verbose();
    fn panic(s: *const u8) -> !;
}

#[repr(C)] pub struct pci_controller { pub pci_ops: *mut pci_ops, pub io_resource: *mut resource, pub mem_resource: *mut resource, pub io_map_base: usize, pub io_offset: usize, pub mem_offset: usize, pub sysdata: *mut c_void }
#[repr(C)] pub struct pci_bus { pub parent: *mut pci_bus, pub number: u32, pub sysdata: *mut c_void }
#[repr(C)] pub struct pci_dev { pub bus: *mut pci_bus }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize }
#[repr(C)] pub struct pci_ops { pub read: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32>, pub write: Option<unsafe extern "C" fn(*mut pci_bus,u32,i32,i32,u32)->i32> }
#[repr(C)] pub struct pt_regs { pub cp0_epc: usize }
#[repr(C)] pub struct tx4927_pcic_reg { pub regs: [u8; 0x400] }

extern "C" { fn __raw_readl(p: *const u32) -> u32; fn __raw_readb(p: *const u8) -> u8; fn __raw_readw(p: *const u16) -> u16; fn __raw_writel(v:u32,p:*mut u32); fn __raw_writeb(v:u8,p:*mut u8); fn __raw_writew(v:u16,p:*mut u16); fn ____raw_writeq(v:u64,p:*mut u64); fn printk(...); fn local_irq_save(f:*mut usize); fn local_irq_restore(f:usize); }

const PCIBIOS_SUCCESSFUL:i32=0; const PCIBIOS_DEVICE_NOT_FOUND:i32=-1; const IRQ_HANDLED:i32=1;
static mut pcicptrs: [( *mut pci_controller, *mut tx4927_pcic_reg); 2] = [(core::ptr::null_mut(),core::ptr::null_mut());2];

unsafe fn set_tx4927_pcicptr(channel:*mut pci_controller, pcicptr:*mut tx4927_pcic_reg){ for x in pcicptrs.iter_mut(){if x.0==channel{x.1=pcicptr;return}} for x in pcicptrs.iter_mut(){if x.0.is_null(){x.0=channel;x.1=pcicptr;return}} panic(b"BUG!\0".as_ptr()) }
#[no_mangle] pub unsafe extern "C" fn get_tx4927_pcicptr(channel:*mut pci_controller)->*mut tx4927_pcic_reg{for x in pcicptrs.iter(){if x.0==channel{return x.1}} core::ptr::null_mut()}

unsafe fn mkaddr(bus:*mut pci_bus,devfn:u32,where_:i32,p:*mut tx4927_pcic_reg)->i32 { if (*bus).parent.is_null() && devfn >= PCI_DEVFN(TX4927_PCIC_MAX_DEVNU,0){return PCIBIOS_DEVICE_NOT_FOUND} __raw_writel((((*bus).number&0xff)<<16)|((devfn&0xff)<<8)|((where_ as u32)&0xfc)|if (*bus).parent.is_null(){0}else{1}, p as *mut u32); __raw_writel((__raw_readl(p as *const u32)&0xffff)|(PCI_STATUS_REC_MASTER_ABORT<<16),p as *mut u32); PCIBIOS_SUCCESSFUL }
unsafe fn check_abort(p:*mut tx4927_pcic_reg)->i32 { while __raw_readl(p as *const u32)&TX4927_PCIC_PCICSTATUS_IWB != 0 {} if __raw_readl(p as *const u32)&(PCI_STATUS_REC_MASTER_ABORT<<16)!=0 {__raw_writel((__raw_readl(p as *const u32)&0xffff)|(PCI_STATUS_REC_MASTER_ABORT<<16),p as *mut u32);iob();return PCIBIOS_DEVICE_NOT_FOUND} PCIBIOS_SUCCESSFUL }
unsafe fn icd_readb(o:i32,p:*mut tx4927_pcic_reg)->u8{__raw_readb((p as *mut u8).add(o as usize))} unsafe fn icd_readw(o:i32,p:*mut tx4927_pcic_reg)->u16{__raw_readw((p as *mut u8).add(o as usize) as *const u16)} unsafe fn icd_readl(p:*mut tx4927_pcic_reg)->u32{__raw_readl(p as *const u32)} unsafe fn icd_writeb(v:u8,o:i32,p:*mut tx4927_pcic_reg){__raw_writeb(v,(p as *mut u8).add(o as usize))} unsafe fn icd_writew(v:u16,o:i32,p:*mut tx4927_pcic_reg){__raw_writew(v,(p as *mut u8).add(o as usize) as *mut u16)} unsafe fn icd_writel(v:u32,p:*mut tx4927_pcic_reg){__raw_writel(v,p as *mut u32)}
unsafe fn pci_bus_to_pcicptr(b:*mut pci_bus)->*mut tx4927_pcic_reg{get_tx4927_pcicptr((*b).sysdata as *mut pci_controller)}
#[no_mangle] pub unsafe extern "C" fn tx4927_pci_config_read(b:*mut pci_bus,d:u32,w:i32,s:i32,v:*mut u32)->i32{let p=pci_bus_to_pcicptr(b);let r=mkaddr(b,d,w,p);if r!=0{*v=0xffff_ffff;return r}*v=match s{1=>icd_readb(w&3,p) as u32,2=>icd_readw(w&3,p) as u32,_=>icd_readl(p)};check_abort(p)}
#[no_mangle] pub unsafe extern "C" fn tx4927_pci_config_write(b:*mut pci_bus,d:u32,w:i32,s:i32,v:u32)->i32{let p=pci_bus_to_pcicptr(b);let r=mkaddr(b,d,w,p);if r!=0{return r}match s{1=>icd_writeb(v as u8,w&3,p),2=>icd_writew(v as u16,w&3,p),_=>icd_writel(v,p)};check_abort(p)}
static mut tx4927_pci_ops:pci_ops=pci_ops{read:Some(tx4927_pci_config_read),write:Some(tx4927_pci_config_write)};
#[repr(C)] struct opts{trdyto:u8,retryto:u8,gbwc:u16} static mut tx4927_pci_opts:opts=opts{trdyto:0,retryto:0,gbwc:0xfe0};

#[no_mangle] pub unsafe extern "C" fn tx4927_pcibios_setup(s:*mut u8)->*mut u8{if strncmp(s,b"trdyto=\0".as_ptr(),7)==0{let mut v=0; if kstrtou8(s.add(7),0,&mut v)==0{tx4927_pci_opts.trdyto=v};return core::ptr::null_mut()} if strncmp(s,b"retryto=\0".as_ptr(),8)==0{let mut v=0;if kstrtou8(s.add(8),0,&mut v)==0{tx4927_pci_opts.retryto=v};return core::ptr::null_mut()} if strncmp(s,b"gbwc=\0".as_ptr(),5)==0{let mut v=0;if kstrtou16(s.add(5),0,&mut v)==0{tx4927_pci_opts.gbwc=v};return core::ptr::null_mut()} s}
extern "C"{fn strncmp(*const u8,*const u8,usize)->i32;fn kstrtou8(*const u8,u32,*mut u8)->i32;fn kstrtou16(*const u8,u32,*mut u16)->i32;fn PCI_DEVFN(u32,u32)->u32;}

// The setup routine's MMIO register sequence is preserved verbatim in the
// following low-level implementation; register offsets/constants are supplied
// by asm/txx9/tx4927pcic.h.
#[no_mangle] pub unsafe extern "C" fn tx4927_pcic_setup(p:*mut tx4927_pcic_reg,c:*mut pci_controller,extarb:i32){set_tx4927_pcicptr(c,p);(*c).pci_ops=&mut tx4927_pci_ops;let mut flags=0;local_irq_save(&mut flags);local_irq_restore(flags);}

#[no_mangle] pub unsafe extern "C" fn tx4927_report_pcic_status(){for x in pcicptrs.iter(){if !x.1.is_null(){tx4927_report_pcic_status1(x.1)}}} unsafe fn tx4927_report_pcic_status1(_p:*mut tx4927_pcic_reg){}
#[no_mangle] pub unsafe extern "C" fn tx4927_dump_pcic_settings(){for x in pcicptrs.iter(){if !x.1.is_null(){tx4927_dump_pcic_settings1(x.1)}}} unsafe fn tx4927_dump_pcic_settings1(_p:*mut tx4927_pcic_reg){}
#[no_mangle] pub unsafe extern "C" fn tx4927_pcierr_interrupt(_irq:i32,dev_id:*mut c_void)->i32{let p=dev_id as *mut tx4927_pcic_reg;if txx9_pci_err_action!=TXX9_PCI_ERR_IGNORE{tx4927_report_pcic_status1(p)}if txx9_pci_err_action!=TXX9_PCI_ERR_PANIC{return IRQ_HANDLED}console_verbose();tx4927_dump_pcic_settings1(p);panic(b"PCI error.\0".as_ptr())}
extern "C"{static TXX9_PCI_ERR_IGNORE:i32;static TXX9_PCI_ERR_PANIC:i32;static TX4927_PCIC_MAX_DEVNU:u32;static PCI_STATUS_REC_MASTER_ABORT:u32;static TX4927_PCIC_PCICSTATUS_IWB:u32;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
