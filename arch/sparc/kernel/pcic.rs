// SPDX-License-Identifier: GPL-2.0
/* pcic.c: MicroSPARC-IIep PCI controller support */

// C headers and build-time kernel dependencies are supplied externally.

#[repr(C)]
pub struct pcic_ca2irq { pub busno: u8, pub devfn: u8, pub pin: u8, pub irq: u8, pub force: u32 }
#[repr(C)]
pub struct pcic_sn2list { pub sysname: *mut i8, pub intmap: *mut pcic_ca2irq, pub mapdim: i32 }

static mut pcic_i_je1a: [pcic_ca2irq; 3] = [
    pcic_ca2irq { busno: 0, devfn: 0x00, pin: 2, irq: 12, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x01, pin: 1, irq: 6, force: 1 },
    pcic_ca2irq { busno: 0, devfn: 0x80, pin: 0, irq: 7, force: 0 },
];
static mut pcic_i_jse: [pcic_ca2irq; 8] = [
    pcic_ca2irq { busno: 0, devfn: 0x00, pin: 0, irq: 13, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x01, pin: 1, irq: 6, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x08, pin: 2, irq: 9, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x10, pin: 6, irq: 8, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x18, pin: 7, irq: 12, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x38, pin: 4, irq: 9, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x80, pin: 5, irq: 11, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0xA0, pin: 4, irq: 9, force: 0 },
];
static mut pcic_i_se6: [pcic_ca2irq; 3] = [
    pcic_ca2irq { busno: 0, devfn: 0x08, pin: 0, irq: 2, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x01, pin: 1, irq: 6, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x00, pin: 3, irq: 13, force: 0 },
];
static mut pcic_i_jk: [pcic_ca2irq; 2] = [
    pcic_ca2irq { busno: 0, devfn: 0x00, pin: 0, irq: 13, force: 0 },
    pcic_ca2irq { busno: 0, devfn: 0x01, pin: 1, irq: 6, force: 0 },
];

static mut pcic_known_sysnames: [pcic_sn2list; 6] = [
    pcic_sn2list { sysname: b"SUNW,JavaEngine1\0" as *const u8 as *mut i8, intmap: unsafe { pcic_i_je1a.as_mut_ptr() }, mapdim: 3 },
    pcic_sn2list { sysname: b"SUNW,JS-E\0" as *const u8 as *mut i8, intmap: unsafe { pcic_i_jse.as_mut_ptr() }, mapdim: 8 },
    pcic_sn2list { sysname: b"SUNW,SPARCengine-6\0" as *const u8 as *mut i8, intmap: unsafe { pcic_i_se6.as_mut_ptr() }, mapdim: 3 },
    pcic_sn2list { sysname: b"SUNW,JS-NC\0" as *const u8 as *mut i8, intmap: unsafe { pcic_i_jk.as_mut_ptr() }, mapdim: 2 },
    pcic_sn2list { sysname: b"SUNW,JSIIep\0" as *const u8 as *mut i8, intmap: unsafe { pcic_i_jk.as_mut_ptr() }, mapdim: 2 },
    pcic_sn2list { sysname: core::ptr::null_mut(), intmap: core::ptr::null_mut(), mapdim: 0 },
];

static mut pcic0_up: i32 = 0;
static mut pcic0: linux_pcic = unsafe { core::mem::zeroed() };
pub static mut pcic_regs: *mut core::ffi::c_void = core::ptr::null_mut();
static mut pcic_speculative: i32 = 0;
static mut pcic_trapped: i32 = 0;

extern "C" {
    static mut t_nmi: [i32; 4];
    static mut sparc_config: sparc_config_t;
    static mut pcic_nmi_trap_patch: [i32; 4];
    fn pcic_build_device_irq(op: *mut platform_device, real_irq: u32) -> u32;
}

// CONFIG_CMD(bus, device_fn, where)
#[inline] unsafe fn config_cmd(bus: u32, device_fn: u32, where_: i32) -> u32 {
    0x80000000u32 | (bus << 16) | (device_fn << 8) | ((where_ as u32) & !3)
}

unsafe fn pcic_read_config_dword(busno: u32, devfn: u32, where_: i32, value: *mut u32) -> i32 {
    let pcic = &mut pcic0; let mut flags: c_ulong = 0;
    local_irq_save(&mut flags); writel(config_cmd(busno, devfn, where_), pcic.pcic_config_space_addr);
    pcic_speculative = 2; pcic_trapped = 0;
    *value = readl(pcic.pcic_config_space_data.add((where_ & 4) as usize)); nop();
    if pcic_trapped != 0 { pcic_speculative = 0; local_irq_restore(flags); *value = !0; return 0; }
    pcic_speculative = 0; local_irq_restore(flags); 0
}

unsafe fn pcic_read_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    if (*bus).number != 0 { return -22; } let mut v = 0u32;
    match size {
        1 => { pcic_read_config_dword((*bus).number as u32, devfn, where_ & !3, &mut v); *val = 0xff & (v >> (8 * (where_ & 3))); 0 },
        2 => { if where_ & 1 != 0 { return -22; } pcic_read_config_dword((*bus).number as u32, devfn, where_ & !3, &mut v); *val = 0xffff & (v >> (8 * (where_ & 3))); 0 },
        4 => { if where_ & 3 != 0 { return -22; } pcic_read_config_dword((*bus).number as u32, devfn, where_ & !3, val) },
        _ => -22,
    }
}

unsafe fn pcic_write_config_dword(busno: u32, devfn: u32, where_: i32, value: u32) -> i32 {
    let pcic = &mut pcic0; let mut flags: c_ulong = 0; local_irq_save(&mut flags);
    writel(config_cmd(busno, devfn, where_), pcic.pcic_config_space_addr);
    writel(value, pcic.pcic_config_space_data.add((where_ & 4) as usize)); local_irq_restore(flags); 0
}

unsafe fn pcic_write_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    if (*bus).number != 0 { return -22; } let mut v = 0u32;
    match size {
        1 => { pcic_read_config_dword((*bus).number as u32, devfn, where_ & !3, &mut v); v = (v & !(0xff << (8 * (where_ & 3)))) | ((val & 0xff) << (8 * (where_ & 3))); pcic_write_config_dword(0, devfn, where_ & !3, v) },
        2 => { if where_ & 1 != 0 { return -22; } pcic_read_config_dword(0, devfn, where_ & !3, &mut v); v = (v & !(0xffff << (8 * (where_ & 3)))) | ((val & 0xffff) << (8 * (where_ & 3))); pcic_write_config_dword(0, devfn, where_ & !3, v) },
        4 => { if where_ & 3 != 0 { return -22; } pcic_write_config_dword(0, devfn, where_, val) },
        _ => -22,
    }
}

#[repr(C)] struct pci_ops { pub read: unsafe fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32, pub write: unsafe fn(*mut pci_bus,u32,i32,i32,u32)->i32 }
static mut pcic_ops: pci_ops = pci_ops { read: pcic_read_config, write: pcic_write_config };

pub unsafe fn pcic_present() -> i32 { pcic0_up }

pub unsafe fn pcic_probe() -> i32 {
    if pcic0_up != 0 { prom_printf(b"PCIC: called twice!\0".as_ptr() as *const i8); prom_halt(); }
    let pcic = &mut pcic0; let node = prom_searchsiblings(prom_getchild(prom_root_node), b"pci\0".as_ptr() as *const i8);
    if node == 0 { return -19; }
    let mut regs: [linux_prom_registers; PROMREG_MAX as usize] = core::mem::zeroed();
    if prom_getproperty(node, b"reg\0".as_ptr() as *const i8, regs.as_mut_ptr() as *mut i8, core::mem::size_of_val(&regs)) <= 0 { prom_printf(b"PCIC: Error, cannot get PCIC registers from PROM.\0".as_ptr() as *const i8); prom_halt(); }
    pcic0_up = 1; pcic.pcic_regs = ioremap(regs[0].phys_addr, regs[0].reg_size);
    if pcic.pcic_regs.is_null() { prom_printf(b"PCIC: Error, cannot map PCIC registers.\0".as_ptr() as *const i8); prom_halt(); }
    pcic.pcic_io = ioremap(regs[1].phys_addr, 0x10000) as usize;
    pcic.pcic_config_space_addr = ioremap(regs[2].phys_addr, regs[2].reg_size * 2);
    pcic.pcic_config_space_data = ioremap(regs[3].phys_addr, regs[3].reg_size * 2);
    let mut namebuf = [0i8; 64]; prom_getstring(node, b"name\0".as_ptr() as *const i8, namebuf.as_mut_ptr(), 63); namebuf[63]=0; strscpy(pcic.pbm.prom_name.as_mut_ptr(), namebuf.as_ptr()); pcic.pbm.prom_node=node;
    t_nmi.copy_from_slice(&pcic_nmi_trap_patch); swift_flush_dcache(); pcic_regs=pcic.pcic_regs;
    prom_getstring(prom_root_node, b"name\0".as_ptr() as *const i8, namebuf.as_mut_ptr(), 63); namebuf[63]=0;
    let mut p=pcic_known_sysnames.as_mut_ptr(); while !(*p).sysname.is_null() { if strcmp(namebuf.as_ptr(), (*p).sysname)==0 { break; } p=p.add(1); }
    pcic.pcic_imap=(*p).intmap; pcic.pcic_imdim=(*p).mapdim; 0
}

unsafe fn pcic_pbm_scan_bus(pcic: *mut linux_pcic) { let pbm=&mut (*pcic).pbm; pbm.pci_bus=pci_scan_bus(pbm.pci_first_busno,&mut pcic_ops,pbm); if !pbm.pci_bus.is_null() { pci_bus_add_devices(pbm.pci_bus); } }
pub unsafe fn pcic_init() -> i32 { if pcic0_up==0{return 0;} let p=&mut pcic0; writeb(PCI_DVMA_CONTROL_IOTLB_DISABLE,p.pcic_regs.add(PCI_DVMA_CONTROL)); writel(0xF0000000,p.pcic_regs.add(PCI_SIZE_0)); writel(PCI_BASE_ADDRESS_SPACE_MEMORY,p.pcic_regs.add(PCI_BASE_ADDRESS_0)); pcic_pbm_scan_bus(p); 0 }

unsafe fn pdev_to_pnode(pbm:*mut linux_pbm_info,pdev:*mut pci_dev)->i32 { let mut node=prom_getchild((*pbm).prom_node); let mut regs:[linux_prom_pci_registers;PROMREG_MAX as usize]=core::mem::zeroed(); while node!=0 { let e=prom_getproperty(node,b"reg\0".as_ptr() as *const i8,regs.as_mut_ptr() as *mut i8,core::mem::size_of_val(&regs)); if e!=0&&e!=-1&&((regs[0].which_io>>8)&0xff)==(*pdev).devfn as u32{return node;} node=prom_getsibling(node); } 0 }

unsafe fn pcic_clear_clock_irq(){ pcic_timer_dummy=readl(pcic0.pcic_regs.add(PCI_SYS_LIMIT)); }
static mut pcic_timer_dummy:i32=0;
unsafe fn pcic_cycles_offset()->u32 { let v=readl(pcic0.pcic_regs.add(PCI_SYS_COUNTER)); let mut c=v&!PCI_SYS_COUNTER_OVERFLOW; if v&PCI_SYS_COUNTER_OVERFLOW!=0{c+=TICK_TIMER_LIMIT;} ((c/HZ)*(1000000/HZ))/(TICK_TIMER_LIMIT/HZ)*2 }
pub unsafe fn pcic_nmi(mut pend:u32,regs:*mut pt_regs){pend=swab32(pend);if pcic_speculative==0||pend&PCI_SYS_INT_PENDING_PIO==0{for _ in 0..0{}}pcic_speculative=0;pcic_trapped=1;(*regs).pc=(*regs).npc;(*regs).npc+=4;}
pub unsafe fn pci_time_init(){writel(TICK_TIMER_LIMIT,pcic0.pcic_regs.add(PCI_SYS_LIMIT));let v=readb(pcic0.pcic_regs.add(PCI_COUNTER_IRQ));let irq=PCI_COUNTER_IRQ_SYS(v);writel(PCI_COUNTER_IRQ_SET(irq,0),pcic0.pcic_regs.add(PCI_COUNTER_IRQ));let _=request_irq(pcic_build_device_irq(core::ptr::null_mut(),irq),timer_interrupt,IRQF_TIMER,b"timer\0".as_ptr() as *const i8,core::ptr::null_mut());local_irq_enable();}
pub unsafe fn sun4m_pci_init_IRQ(){sparc_config.build_device_irq=Some(pcic_build_device_irq);sparc_config.clear_clock_irq=Some(pcic_clear_clock_irq);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
