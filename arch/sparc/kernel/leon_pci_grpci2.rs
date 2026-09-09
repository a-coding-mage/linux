// SPDX-License-Identifier: GPL-2.0
/*
 * leon_pci_grpci2.c: GRPCI2 Host PCI driver
 * Rust translation of the implementation source.
 */

#[repr(C)]
pub struct grpci2_barcfg { pub pciadr: c_ulong, pub ahbadr: c_ulong }

#[repr(C)]
pub struct grpci2_regs {
    pub ctrl: c_uint, pub sts_cap: c_uint, pub res1: c_int, pub io_map: c_uint,
    pub dma_ctrl: c_uint, pub dma_bdbase: c_uint, pub res2: [c_int; 2],
    pub bars: [c_uint; 6], pub res3: [c_int; 2], pub ahbmst_map: [c_uint; 16],
    pub t_ctrl: c_uint, pub t_cnt: c_uint, pub t_adpat: c_uint, pub t_admask: c_uint,
    pub t_sigpat: c_uint, pub t_sigmask: c_uint, pub t_adstate: c_uint, pub t_sigstate: c_uint,
}

#[repr(C)] pub struct grpci2_bd_chan { pub ctrl: c_uint, pub nchan: c_uint, pub nbd: c_uint, pub res: c_uint }
#[repr(C)] pub struct grpci2_bd_data { pub ctrl: c_uint, pub pci_adr: c_uint, pub ahb_adr: c_uint, pub next: c_uint }
#[repr(C)] pub struct grpci2_cap_first { pub ctrl: c_uint, pub pci2ahb_map: [c_uint; 6], pub ext2ahb_map: c_uint, pub io_map: c_uint, pub pcibar_size: [c_uint; 6] }

pub const CTRL_BUS_BIT: u32 = 16;
pub const CTRL_RESET: u32 = 1 << 31; pub const CTRL_SI: u32 = 1 << 27; pub const CTRL_PE: u32 = 1 << 26;
pub const CTRL_EI: u32 = 1 << 25; pub const CTRL_ER: u32 = 1 << 24; pub const CTRL_BUS: u32 = 0xff << CTRL_BUS_BIT; pub const CTRL_HOSTINT: u32 = 0xf;
pub const STS_HOST: u32 = 1 << 31; pub const STS_MST: u32 = 1 << 30; pub const STS_IRQMODE: u32 = 0x3 << 24;
pub const STS_CFGERRVALID: u32 = 1 << 20; pub const STS_CFGERR: u32 = 1 << 19; pub const STS_INTSTS_BIT: u32 = 8;
pub const STS_ISYSERR: u32 = 1 << 17; pub const STS_IDMA: u32 = 1 << 16; pub const STS_IDMAERR: u32 = 1 << 15;
pub const STS_IMSTABRT: u32 = 1 << 14; pub const STS_ITGTABRT: u32 = 1 << 13; pub const STS_IPARERR: u32 = 1 << 12;
pub const STS_ERR_IRQ: u32 = STS_ISYSERR | STS_IMSTABRT | STS_ITGTABRT | STS_IPARERR;
pub const TGT: u32 = 256; pub const CAP9_BAR_OFS: u32 = 4; pub const CAP9_IOMAP_OFS: u32 = 0x20; pub const CAP9_BARSIZE_OFS: u32 = 0x24;

#[repr(C)] pub struct grpci2_priv {
    pub info: leon_pci_info, pub regs: *mut grpci2_regs, pub irq: c_char, pub irq_mode: c_char,
    pub bt_enabled: c_char, pub do_reset: c_char, pub irq_mask: c_char, pub pciid: u32,
    pub irq_map: [u8; 4], pub virq_err: c_uint, pub virq_dma: c_uint,
    pub pci_area: c_ulong, pub pci_area_end: c_ulong, pub pci_io: c_ulong, pub pci_conf: c_ulong,
    pub pci_conf_end: c_ulong, pub pci_io_va: c_ulong, pub tgtbars: [grpci2_barcfg; 6],
}

extern "C" {
    static mut grpci2priv: *mut grpci2_priv;
    fn leon_update_virq_handling(irq: c_uint, handler: unsafe extern "C" fn(*mut irq_desc), name: *const c_char, ack: c_uint);
    fn generic_handle_irq(irq: c_uint); fn irq_alloc(irq: c_uint, pil: c_uint) -> c_uint;
    fn irq_set_chip_and_handler_name(irq: c_uint, chip: *const irq_chip, handler: unsafe extern "C" fn(*mut irq_data), name: *const c_char);
    fn irq_set_chip_data(irq: c_uint, data: *mut c_void); fn request_irq(irq: c_uint, f: unsafe extern "C" fn(c_int,*mut c_void)->irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void)->c_int;
    fn platform_driver_register(driver: *mut platform_driver)->c_int; fn leon_pci_init(dev: *mut platform_device, info: *mut leon_pci_info);
    fn ioremap(addr: c_ulong, size: c_ulong)->c_ulong; fn iounmap(addr: *mut c_void); fn of_ioremap(r: *mut resource, off: c_ulong, size: c_ulong, name: *const c_char)->*mut grpci2_regs;
    fn of_iounmap(r: *mut resource, addr: *mut grpci2_regs, size: c_ulong); fn of_get_property(node: *mut c_void, name: *const c_char, len: *mut c_int)->*const c_int;
    fn release_resource(r: *mut resource); fn request_resource(parent: *mut resource, child: *mut resource)->c_int; fn kfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize)->*mut c_void; fn memset(dst: *mut c_void, c: c_int, n: usize)->*mut c_void;
    fn ssleep(seconds: c_uint); fn printk(fmt: *const c_char, ...);
}

type c_int = i32; type c_uint = u32; type c_ulong = usize; type c_char = i8; type c_void = core::ffi::c_void;
type irqreturn_t = c_int;
#[repr(C)] pub struct irq_data { pub chip_data: *mut c_void, pub chip: *mut irq_chip }
#[repr(C)] pub struct irq_desc { pub irq_data: irq_data }
#[repr(C)] pub struct irq_chip { pub name: *const c_char, pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data)->c_uint>, pub irq_shutdown: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)> }
#[repr(C)] pub struct resource { pub start: c_ulong, pub end: c_ulong, pub name: *const c_char, pub flags: c_ulong }
#[repr(C)] pub struct pci_bus { pub number: c_uint, pub sysdata: *mut c_void }
#[repr(C)] pub struct pci_dev { pub bus: *mut pci_bus }
#[repr(C)] pub struct pci_ops { pub read: Option<unsafe extern "C" fn(*mut pci_bus,c_uint,c_int,c_int,*mut u32)->c_int>, pub write: Option<unsafe extern "C" fn(*mut pci_bus,c_uint,c_int,c_int,u32)->c_int> }
#[repr(C)] pub struct leon_pci_info { pub io_space: resource, pub mem_space: resource, pub busn: resource, pub ops: *mut pci_ops, pub map_irq: Option<unsafe extern "C" fn(*const pci_dev,u8,u8)->c_int> }
#[repr(C)] pub struct platform_device { pub num_resources: c_uint, pub resource: *mut resource, pub archdata: archdata, pub dev: dev }
#[repr(C)] pub struct archdata { pub irqs: *mut c_uint }
#[repr(C)] pub struct dev { pub of_node: *mut c_void }
#[repr(C)] pub struct of_device_id { pub name: *const c_char }
#[repr(C)] pub struct driver { pub name: *const c_char, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device)->c_int> }

#[inline] unsafe fn regload(a: *const c_uint) -> u32 { u32::from_be(core::ptr::read_volatile(a)) }
#[inline] unsafe fn regstore(a: *mut c_uint, v: u32) { core::ptr::write_volatile(a, v.to_be()); }

unsafe extern "C" fn grpci2_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int { let p=(*(dev).as_ref().unwrap()).bus.as_ref().unwrap().sysdata as *mut grpci2_priv; let irq_group=slot&3; let pin=((pin.wrapping_sub(1)+irq_group)&3) as usize; (*p).irq_map[pin] as c_int }

unsafe extern "C" fn grpci2_cfg_r32(p:*mut grpci2_priv,bus:u32,mut devfn:u32,where_:c_int,val:*mut u32)->c_int { if where_&3!=0{return -22}; let mut bus=bus; if bus==0{devfn+=0x30}else if bus==TGT{bus=0;devfn=0}; regstore(&mut (*(*p).regs).ctrl, (regload(&(*(*p).regs).ctrl)&!(0xff<<16))|(bus<<16)); regstore(&mut (*(*p).regs).sts_cap,STS_CFGERR|STS_CFGERRVALID); let pc=((*p).pci_conf|(devfn<<8)|((where_ as u32)&0xfc)) as *const u32; let tmp=core::ptr::read_volatile(pc); while regload(&(*(*p).regs).sts_cap)&STS_CFGERRVALID==0{}; *val=if regload(&(*(*p).regs).sts_cap)&STS_CFGERR!=0{!0}else{tmp.swap_bytes()}; 0 }
unsafe extern "C" fn grpci2_cfg_r16(p:*mut grpci2_priv,b:u32,d:u32,w:c_int,v:*mut u32)->c_int{if w&1!=0{return -22} let mut x=0;let r=grpci2_cfg_r32(p,b,d,w&!3,&mut x);*v=0xffff&(x>>(8*((w&3)as u32)));r}
unsafe extern "C" fn grpci2_cfg_r8(p:*mut grpci2_priv,b:u32,d:u32,w:c_int,v:*mut u32)->c_int{let mut x=0;let r=grpci2_cfg_r32(p,b,d,w&!3,&mut x);*v=0xff&(x>>(8*((w&3)as u32)));r}
unsafe extern "C" fn grpci2_cfg_w32(p:*mut grpci2_priv,b:u32,mut d:u32,w:c_int,v:u32)->c_int{if w&3!=0{return -22}let mut b=b;if b==0{d+=0x30}else if b==TGT{b=0;d=0}regstore(&mut (*(*p).regs).ctrl,(regload(&(*(*p).regs).ctrl)&!(0xff<<16))|(b<<16));regstore(&mut (*(*p).regs).sts_cap,STS_CFGERR|STS_CFGERRVALID);core::ptr::write_volatile(((*p).pci_conf|(d<<8)|((w as u32)&0xfc))as *mut u32,v.swap_bytes());while regload(&(*(*p).regs).sts_cap)&STS_CFGERRVALID==0{}0}
unsafe extern "C" fn grpci2_cfg_w16(p:*mut grpci2_priv,b:u32,d:u32,w:c_int,v:u32)->c_int{if w&1!=0{return -22}let mut x=0;let r=grpci2_cfg_r32(p,b,d,w&!3,&mut x);if r!=0{return r}x=(x&!(0xffff<<(8*((w&3)as u32))))|((v&0xffff)<<(8*((w&3)as u32)));grpci2_cfg_w32(p,b,d,w&!3,x)}
unsafe extern "C" fn grpci2_cfg_w8(p:*mut grpci2_priv,b:u32,d:u32,w:c_int,v:u32)->c_int{let mut x=0;let r=grpci2_cfg_r32(p,b,d,w&!3,&mut x);if r!=0{return r}x=(x&!(0xff<<(8*((w&3)as u32))))|((v&0xff)<<(8*((w&3)as u32)));grpci2_cfg_w32(p,b,d,w&!3,x)}

unsafe extern "C" fn grpci2_read_config(bus:*mut pci_bus,d:u32,w:c_int,size:c_int,val:*mut u32)->c_int{let p=grpci2priv;let b=(*bus).number;if d>>3&0x1f>15||b>255{*val=!0;return 0}match size{1=>grpci2_cfg_r8(p,b,d,w,val),2=>grpci2_cfg_r16(p,b,d,w,val),4=>grpci2_cfg_r32(p,b,d,w,val),_=>-22}}
unsafe extern "C" fn grpci2_write_config(bus:*mut pci_bus,d:u32,w:c_int,size:c_int,v:u32)->c_int{let p=grpci2priv;let b=(*bus).number;if d>>3&0x1f>15||b>255{return 0}match size{1=>grpci2_cfg_w8(p,b,d,w,v),2=>grpci2_cfg_w16(p,b,d,w,v),4=>grpci2_cfg_w32(p,b,d,w,v),_=>-22}}
static mut GRPCI2_OPS: pci_ops=pci_ops{read:Some(grpci2_read_config),write:Some(grpci2_write_config)};

unsafe extern "C" fn grpci2_mask_irq(_: *mut irq_data) {} unsafe extern "C" fn grpci2_unmask_irq(_: *mut irq_data) {}
unsafe extern "C" fn grpci2_startup_irq(d:*mut irq_data)->c_uint{grpci2_unmask_irq(d);0} unsafe extern "C" fn grpci2_shutdown_irq(d:*mut irq_data){grpci2_mask_irq(d)}
static mut GRPCI2_IRQ: irq_chip=irq_chip{name:b"grpci2\0".as_ptr()as*const c_char,irq_startup:Some(grpci2_startup_irq),irq_shutdown:Some(grpci2_shutdown_irq),irq_mask:Some(grpci2_mask_irq),irq_unmask:Some(grpci2_unmask_irq),irq_eoi:None};

unsafe extern "C" fn grpci2_pci_flow_irq(_: *mut irq_desc) {}
unsafe extern "C" fn grpci2_build_device_irq(irq:c_uint)->c_uint{let v=irq_alloc(irq,1<<8);if v!=0{irq_set_chip_data(v,irq as *mut c_void)}v}
unsafe extern "C" fn grpci2_hw_init(_: *mut grpci2_priv) {}
unsafe extern "C" fn grpci2_jump_interrupt(_:c_int,_:*mut c_void)->irqreturn_t{0}
unsafe extern "C" fn grpci2_err_interrupt(_:c_int,_:*mut c_void)->irqreturn_t{0}
unsafe extern "C" fn grpci2_of_probe(_: *mut platform_device)->c_int{-19}

static mut GRPCI2_OF_MATCH:[of_device_id;3]=[of_device_id{name:b"GAISLER_GRPCI2\0".as_ptr()as*const c_char},of_device_id{name:b"01_07c\0".as_ptr()as*const c_char},of_device_id{name:core::ptr::null()}];
static mut GRPCI2_OF_DRIVER:platform_driver=platform_driver{driver:driver{name:b"grpci2\0".as_ptr()as*const c_char,of_match_table:unsafe{GRPCI2_OF_MATCH.as_ptr()}},probe:Some(grpci2_of_probe)};
#[no_mangle] pub unsafe extern "C" fn grpci2_init()->c_int{platform_driver_register(&mut GRPCI2_OF_DRIVER)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
