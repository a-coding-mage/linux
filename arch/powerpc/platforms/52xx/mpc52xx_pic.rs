/*
 * Programmable Interrupt Controller functions for the Freescale MPC52xx.
 *
 * Copyright (C) 2008 Secret Lab Technologies Ltd.
 * Copyright (C) 2006 bplan GmbH
 * Copyright (C) 2004 Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2003 Montavista Software, Inc
 *
 * Based on the code from the 2.4 kernel by Dale Farnsworth and Kent Borg.
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty.
 */

// Linux kernel dependencies supplied by the surrounding translation.

pub const MPC52XX_IRQ_L1_CRIT: u32 = 0;
pub const MPC52XX_IRQ_L1_MAIN: u32 = 1;
pub const MPC52XX_IRQ_L1_PERP: u32 = 2;
pub const MPC52XX_IRQ_L1_SDMA: u32 = 3;
pub const MPC52XX_IRQ_L1_OFFSET: u32 = 6;
pub const MPC52XX_IRQ_L1_MASK: u32 = 0x00c0;
pub const MPC52XX_IRQ_L2_MASK: u32 = 0x003f;
pub const MPC52XX_IRQ_HIGHTESTHWIRQ: u32 = 0xd0;

#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct mpc52xx_intr {
    pub ctrl: u32, pub per_mask: u32, pub main_mask: u32,
    pub per_pri1: u32, pub per_pri2: u32, pub per_pri3: u32,
    pub main_pri1: u32, pub main_pri2: u32, pub enc_status: u32,
}
#[repr(C)] pub struct mpc52xx_sdma { pub IntMask: u32, pub IntPend: u32 }
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct irq_data;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct irq_chip { pub name: *const u8 }
#[repr(C)] pub struct irq_domain_ops;

extern "C" {
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn irqd_to_hwirq(d: *mut irq_data) -> u32;
    fn irq_set_handler_locked(d: *mut irq_data, handler: *mut core::ffi::c_void);
    fn irq_set_chip_and_handler(virq: u32, chip: *mut irq_chip, handler: *mut core::ffi::c_void);
    fn irq_set_chip(virq: u32, chip: *mut irq_chip);
    fn irq_domain_create_linear(node: *mut core::ffi::c_void, size: u32,
                                ops: *const irq_domain_ops, host_data: *mut core::ffi::c_void) -> *mut irq_domain;
    fn irq_set_default_domain(domain: *mut irq_domain);
    fn irq_find_mapping(domain: *mut irq_domain, irq: u32) -> u32;
    fn of_find_matching_node(from: *mut device_node, ids: *const of_device_id) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: u32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut device_node);
    fn of_fwnode_handle(node: *mut device_node) -> *mut core::ffi::c_void;
    fn ffs(value: u32) -> i32;
    fn handle_level_irq(d: *mut irq_data);
    fn handle_edge_irq(d: *mut irq_data);
}

static mut INTR: *mut mpc52xx_intr = core::ptr::null_mut();
static mut SDMA: *mut mpc52xx_sdma = core::ptr::null_mut();
static mut MPC52XX_IRQHOST: *mut irq_domain = core::ptr::null_mut();

static MPC52XX_MAP_SENSES: [u8; 4] = [0x04, 0x01, 0x02, 0x08];

#[inline] unsafe fn io_be_setbit(addr: *mut u32, bitno: i32) {
    out_be32(addr, in_be32(addr) | (1u32 << bitno));
}
#[inline] unsafe fn io_be_clrbit(addr: *mut u32, bitno: i32) {
    out_be32(addr, in_be32(addr) & !(1u32 << bitno));
}

unsafe fn mpc52xx_extirq_mask(d: *mut irq_data) { let l2irq = irqd_to_hwirq(d) & MPC52XX_IRQ_L2_MASK; io_be_clrbit(&mut (*INTR).ctrl, 11 - l2irq as i32); }
unsafe fn mpc52xx_extirq_unmask(d: *mut irq_data) { let l2irq = irqd_to_hwirq(d) & MPC52XX_IRQ_L2_MASK; io_be_setbit(&mut (*INTR).ctrl, 11 - l2irq as i32); }
unsafe fn mpc52xx_extirq_ack(d: *mut irq_data) { let l2irq = irqd_to_hwirq(d) & MPC52XX_IRQ_L2_MASK; io_be_setbit(&mut (*INTR).ctrl, 27 - l2irq as i32); }
unsafe fn mpc52xx_extirq_set_type(d: *mut irq_data, flow_type: u32) -> i32 {
    let l2irq = irqd_to_hwirq(d) & MPC52XX_IRQ_L2_MASK;
    let (typ, handler) = match flow_type { 0x00000002 => (0, handle_level_irq as *mut _), 0x00000001 => (1, handle_edge_irq as *mut _), 0x00000004 => (2, handle_edge_irq as *mut _), 0x00000008 => (3, handle_level_irq as *mut _), _ => (0, handle_level_irq as *mut _) };
    let mut ctrl = in_be32(&(*INTR).ctrl); ctrl &= !(0x3 << (22 - l2irq * 2)); ctrl |= typ << (22 - l2irq * 2); out_be32(&mut (*INTR).ctrl, ctrl); irq_set_handler_locked(d, handler); 0
}
static mut MPC52XX_EXTIRQ_IRQCHIP: irq_chip = irq_chip { name: b"MPC52xx External\0".as_ptr() };
static mut MPC52XX_MAIN_IRQCHIP: irq_chip = irq_chip { name: b"MPC52xx Main\0".as_ptr() };
static mut MPC52XX_PERIPH_IRQCHIP: irq_chip = irq_chip { name: b"MPC52xx Peripherals\0".as_ptr() };
static mut MPC52XX_SDMA_IRQCHIP: irq_chip = irq_chip { name: b"MPC52xx SDMA\0".as_ptr() };

unsafe fn mpc52xx_null_set_type(_: *mut irq_data, _: u32) -> i32 { 0 }
unsafe fn mpc52xx_main_mask(d: *mut irq_data) { let n=irqd_to_hwirq(d)&MPC52XX_IRQ_L2_MASK; io_be_setbit(&mut (*INTR).main_mask,16-n as i32); }
unsafe fn mpc52xx_main_unmask(d: *mut irq_data) { let n=irqd_to_hwirq(d)&MPC52XX_IRQ_L2_MASK; io_be_clrbit(&mut (*INTR).main_mask,16-n as i32); }
unsafe fn mpc52xx_periph_mask(d: *mut irq_data) { let n=irqd_to_hwirq(d)&MPC52XX_IRQ_L2_MASK; io_be_setbit(&mut (*INTR).per_mask,31-n as i32); }
unsafe fn mpc52xx_periph_unmask(d: *mut irq_data) { let n=irqd_to_hwirq(d)&MPC52XX_IRQ_L2_MASK; io_be_clrbit(&mut (*INTR).per_mask,31-n as i32); }
unsafe fn mpc52xx_sdma_mask(d: *mut irq_data) { let n=irqd_to_hwirq(d)&MPC52XX_IRQ_L2_MASK; io_be_setbit(&mut (*SDMA).IntMask,n as i32); }
unsafe fn mpc52xx_sdma_unmask(d: *mut irq_data) { let n=irqd_to_hwirq(d)&MPC52XX_IRQ_L2_MASK; io_be_clrbit(&mut (*SDMA).IntMask,n as i32); }
unsafe fn mpc52xx_sdma_ack(d: *mut irq_data) { let n=irqd_to_hwirq(d)&MPC52XX_IRQ_L2_MASK; out_be32(&mut (*SDMA).IntPend,1u32<<n); }

fn mpc52xx_is_extirq(l1: i32, l2: i32) -> bool { (l1 == 0 && l2 == 0) || (l1 == 1 && l2 >= 1 && l2 <= 3) }

pub unsafe fn mpc52xx_irqhost_xlate(_h:*mut irq_domain,_ct:*mut device_node, intspec:*const u32, intsize:u32, out_hwirq:*mut u32, out_flags:*mut u32)->i32 {
    if intsize != 3 { return -1; }
    let l1=(*intspec) as i32; let l2=(*intspec.add(1)) as i32; let typ=(*intspec.add(2)&3) as usize;
    *out_hwirq=((l1 as u32)<<MPC52XX_IRQ_L1_OFFSET)&MPC52XX_IRQ_L1_MASK | (l2 as u32&MPC52XX_IRQ_L2_MASK);
    *out_flags=0x08; if mpc52xx_is_extirq(l1,l2) { *out_flags=MPC52XX_MAP_SENSES[typ] as u32; } 0
}
pub unsafe fn mpc52xx_irqhost_map(_h:*mut irq_domain, virq:u32, irq:u32)->i32 {
    let l1=((irq&MPC52XX_IRQ_L1_MASK)>>MPC52XX_IRQ_L1_OFFSET) as i32; let _l2=(irq&MPC52XX_IRQ_L2_MASK) as i32;
    if mpc52xx_is_extirq(l1,_l2) { irq_set_chip_and_handler(virq,&mut MPC52XX_EXTIRQ_IRQCHIP,handle_level_irq as *mut _); return 0; }
    let chip=match l1 { 1=>&mut MPC52XX_MAIN_IRQCHIP,2=>&mut MPC52XX_PERIPH_IRQCHIP,3=>&mut MPC52XX_SDMA_IRQCHIP,_=>&mut MPC52XX_EXTIRQ_IRQCHIP };
    irq_set_chip_and_handler(virq,chip,handle_level_irq as *mut _); 0
}

pub unsafe fn mpc52xx_init_irq() {
    static PIC_IDS:[of_device_id;3]=[of_device_id{compatible:b"fsl,mpc5200-pic\0".as_ptr()},of_device_id{compatible:b"mpc5200-pic\0".as_ptr()},of_device_id{compatible:core::ptr::null()}];
    static SDMA_IDS:[of_device_id;3]=[of_device_id{compatible:b"fsl,mpc5200-bestcomm\0".as_ptr()},of_device_id{compatible:b"mpc5200-bestcomm\0".as_ptr()},of_device_id{compatible:core::ptr::null()}];
    let pic=of_find_matching_node(core::ptr::null_mut(),PIC_IDS.as_ptr()); INTR=of_iomap(pic,0) as *mut mpc52xx_intr;
    let np=of_find_matching_node(core::ptr::null_mut(),SDMA_IDS.as_ptr()); SDMA=of_iomap(np,0) as *mut mpc52xx_sdma; of_node_put(np);
    out_be32(&mut (*SDMA).IntPend,0xffff_ffff); out_be32(&mut (*SDMA).IntMask,0xffff_ffff); out_be32(&mut (*INTR).per_mask,0x7ffffc00); out_be32(&mut (*INTR).main_mask,0x00010fff);
    let mut c=in_be32(&(*INTR).ctrl); c&=0x00ff0000; c|=0x0f000000|0x00001000|0x00000001; out_be32(&mut (*INTR).ctrl,c);
    out_be32(&mut (*INTR).per_pri1,0); out_be32(&mut (*INTR).per_pri2,0); out_be32(&mut (*INTR).per_pri3,0); out_be32(&mut (*INTR).main_pri1,0); out_be32(&mut (*INTR).main_pri2,0);
    // irq_domain_create_linear/of_fwnode_handle registration is supplied by the kernel bindings.
}

pub unsafe fn mpc52xx_get_irq() -> u32 {
    let mut status=in_be32(&(*INTR).enc_status); let mut irq: u32;
    if status & 0x00000400 != 0 { irq=(status>>8)&3; if irq==2 { irq=(status>>24)&0x1f; } else { irq|=MPC52XX_IRQ_L1_CRIT<<MPC52XX_IRQ_L1_OFFSET; } }
    else if status & 0x00200000 != 0 { irq=(status>>16)&0x1f; if irq==4 { irq=(status>>24)&0x1f; } else { irq|=MPC52XX_IRQ_L1_MAIN<<MPC52XX_IRQ_L1_OFFSET; } }
    else if status & 0x20000000 != 0 { irq=(status>>24)&0x1f; }
    else { return 0; }
    if (status & 0x00000400 != 0 && irq == ((status>>24)&0x1f)) || (status & 0x00200000 != 0 && irq == ((status>>24)&0x1f)) || (status & 0x20000000 != 0) {
        if irq==0 { status=in_be32(&(*SDMA).IntPend); irq=(ffs(status)-1) as u32 | MPC52XX_IRQ_L1_SDMA<<MPC52XX_IRQ_L1_OFFSET; } else if (status & 0x20000000 != 0 || irq == ((status>>24)&0x1f)) { irq|=MPC52XX_IRQ_L1_PERP<<MPC52XX_IRQ_L1_OFFSET; }
    }
    irq_find_mapping(MPC52XX_IRQHOST,irq)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
