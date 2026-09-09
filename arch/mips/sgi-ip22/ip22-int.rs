// SPDX-License-Identifier: GPL-2.0
/*
 * ip22-int.c: Routines for generic manipulation of the INT[23] ASIC
 *            found on INDY and Indigo2 workstations.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1997, 1998 Ralf Baechle (ralf@gnu.org)
 * Copyright (C) 1999 Andrew R. Baker (andrewb@uab.edu)
 *            - Indigo2 changes
 *            - Interrupt handling fixes
 * Copyright (C) 2001, 2003 Ladislav Michl (ladis@linux-mips.org)
 */

// So far nothing hangs here.
// #undef USE_LIO3_IRQ

static mut sgint: *mut sgint_regs = core::ptr::null_mut();

static mut lc0msk_to_irqnr: [i8; 256] = [0; 256];
static mut lc1msk_to_irqnr: [i8; 256] = [0; 256];
static mut lc2msk_to_irqnr: [i8; 256] = [0; 256];
static mut lc3msk_to_irqnr: [i8; 256] = [0; 256];

extern "C" {
    fn ip22_eisa_init() -> i32;
    fn ip22_be_interrupt(irq: i32);
    fn do_IRQ(irq: i32);
    fn irq_enter();
    fn kstat_incr_irq_this_cpu(irq: i32);
    fn irq_exit();
    fn read_c0_status() -> u32;
    fn read_c0_cause() -> u32;
    fn indy_8254timer_irq();
    fn mips_cpu_irq_init();
    fn irq_set_chip_and_handler(irq: i32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(), flags: u32, name: *const i8, dev: *mut core::ffi::c_void) -> i32;
    fn no_action();
    fn handle_level_irq();
    fn pr_err(message: *const i8);
    fn ip22_is_fullhouse() -> bool;
}

#[repr(C)]
pub struct irq_data {
    pub irq: i32,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const i8,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

unsafe fn enable_local0_irq(d: *mut irq_data) {
    if (*d).irq != SGI_MAP_0_IRQ {
        (*sgint).imask0 |= 1u8 << ((*d).irq - SGINT_LOCAL0);
    }
}

unsafe fn disable_local0_irq(d: *mut irq_data) {
    (*sgint).imask0 &= !(1u8 << ((*d).irq - SGINT_LOCAL0));
}

static mut ip22_local0_irq_type: irq_chip = irq_chip {
    name: b"IP22 local 0\0".as_ptr() as *const i8,
    irq_mask: Some(disable_local0_irq),
    irq_unmask: Some(enable_local0_irq),
};

unsafe fn enable_local1_irq(d: *mut irq_data) {
    if (*d).irq != SGI_MAP_1_IRQ {
        (*sgint).imask1 |= 1u8 << ((*d).irq - SGINT_LOCAL1);
    }
}

unsafe fn disable_local1_irq(d: *mut irq_data) {
    (*sgint).imask1 &= !(1u8 << ((*d).irq - SGINT_LOCAL1));
}

static mut ip22_local1_irq_type: irq_chip = irq_chip {
    name: b"IP22 local 1\0".as_ptr() as *const i8,
    irq_mask: Some(disable_local1_irq),
    irq_unmask: Some(enable_local1_irq),
};

unsafe fn enable_local2_irq(d: *mut irq_data) {
    (*sgint).imask0 |= 1u8 << (SGI_MAP_0_IRQ - SGINT_LOCAL0);
    (*sgint).cmeimask0 |= 1u8 << ((*d).irq - SGINT_LOCAL2);
}

unsafe fn disable_local2_irq(d: *mut irq_data) {
    (*sgint).cmeimask0 &= !(1u8 << ((*d).irq - SGINT_LOCAL2));
    if (*sgint).cmeimask0 == 0 {
        (*sgint).imask0 &= !(1u8 << (SGI_MAP_0_IRQ - SGINT_LOCAL0));
    }
}

static mut ip22_local2_irq_type: irq_chip = irq_chip {
    name: b"IP22 local 2\0".as_ptr() as *const i8,
    irq_mask: Some(disable_local2_irq),
    irq_unmask: Some(enable_local2_irq),
};

unsafe fn enable_local3_irq(d: *mut irq_data) {
    (*sgint).imask1 |= 1u8 << (SGI_MAP_1_IRQ - SGINT_LOCAL1);
    (*sgint).cmeimask1 |= 1u8 << ((*d).irq - SGINT_LOCAL3);
}

unsafe fn disable_local3_irq(d: *mut irq_data) {
    (*sgint).cmeimask1 &= !(1u8 << ((*d).irq - SGINT_LOCAL3));
    if (*sgint).cmeimask1 == 0 {
        (*sgint).imask1 &= !(1u8 << (SGI_MAP_1_IRQ - SGINT_LOCAL1));
    }
}

static mut ip22_local3_irq_type: irq_chip = irq_chip {
    name: b"IP22 local 3\0".as_ptr() as *const i8,
    irq_mask: Some(disable_local3_irq),
    irq_unmask: Some(enable_local3_irq),
};

unsafe fn indy_local0_irqdispatch() {
    let mask: u8 = (*sgint).istat0 & (*sgint).imask0;
    let irq: i32;
    if mask & SGINT_ISTAT0_LIO2 != 0 {
        let mask2 = (*sgint).vmeistat & (*sgint).cmeimask0;
        irq = lc2msk_to_irqnr[mask2 as usize] as i32;
    } else {
        irq = lc0msk_to_irqnr[mask as usize] as i32;
    }
    if irq != 0 { do_IRQ(irq); } else { do_IRQ(SGINT_LOCAL0); }
}

unsafe fn indy_local1_irqdispatch() {
    let mask: u8 = (*sgint).istat1 & (*sgint).imask1;
    let irq: i32;
    if mask & SGINT_ISTAT1_LIO3 != 0 {
        let mask2 = (*sgint).vmeistat & (*sgint).cmeimask1;
        irq = lc3msk_to_irqnr[mask2 as usize] as i32;
    } else {
        irq = lc1msk_to_irqnr[mask as usize] as i32;
    }
    if irq != 0 { do_IRQ(irq); }
}

unsafe fn indy_buserror_irq() {
    let irq = SGI_BUSERR_IRQ;
    irq_enter();
    kstat_incr_irq_this_cpu(irq);
    ip22_be_interrupt(irq);
    irq_exit();
}

// #ifdef USE_LIO3_IRQ: SGI_INTERRUPTS = SGINT_END; otherwise SGINT_LOCAL3.
const SGI_INTERRUPTS: i32 = SGINT_LOCAL3;

pub unsafe fn plat_irq_dispatch() {
    let pending = read_c0_status() & read_c0_cause();
    if pending & CAUSEF_IP7 != 0 { do_IRQ(SGI_TIMER_IRQ); }
    else if pending & CAUSEF_IP2 != 0 { indy_local0_irqdispatch(); }
    else if pending & CAUSEF_IP3 != 0 { indy_local1_irqdispatch(); }
    else if pending & CAUSEF_IP6 != 0 { indy_buserror_irq(); }
    else if pending & (CAUSEF_IP4 | CAUSEF_IP5) != 0 { indy_8254timer_irq(); }
}

pub unsafe fn arch_init_irq() {
    for i in 0..256usize {
        let (a, b, c, d, off) = if i & 0x80 != 0 { (7, 7, 7, 7, 0) }
            else if i & 0x40 != 0 { (6, 6, 6, 6, 0) }
            else if i & 0x20 != 0 { (5, 5, 5, 5, 0) }
            else if i & 0x10 != 0 { (4, 4, 4, 4, 0) }
            else if i & 0x08 != 0 { (3, 3, 3, 3, 0) }
            else if i & 0x04 != 0 { (2, 2, 2, 2, 0) }
            else if i & 0x02 != 0 { (1, 1, 1, 1, 0) }
            else if i & 0x01 != 0 { (0, 0, 0, 0, 0) }
            else { (0, 0, 0, 0, 1) };
        if off != 0 { lc0msk_to_irqnr[i] = 0; lc1msk_to_irqnr[i] = 0; lc2msk_to_irqnr[i] = 0; lc3msk_to_irqnr[i] = 0; }
        else { lc0msk_to_irqnr[i] = (SGINT_LOCAL0 + a) as i8; lc1msk_to_irqnr[i] = (SGINT_LOCAL1 + b) as i8; lc2msk_to_irqnr[i] = (SGINT_LOCAL2 + c) as i8; lc3msk_to_irqnr[i] = (SGINT_LOCAL3 + d) as i8; }
    }
    (*sgint).imask0 = 0; (*sgint).imask1 = 0; (*sgint).cmeimask0 = 0; (*sgint).cmeimask1 = 0;
    mips_cpu_irq_init();
    for i in SGINT_LOCAL0..SGI_INTERRUPTS { let handler = if i < SGINT_LOCAL1 { &mut ip22_local0_irq_type } else if i < SGINT_LOCAL2 { &mut ip22_local1_irq_type } else if i < SGINT_LOCAL3 { &mut ip22_local2_irq_type } else { &mut ip22_local3_irq_type }; irq_set_chip_and_handler(i, handler, handle_level_irq); }
    // vector handler: these registers the IRQs as non-sharable.
    request_irq(SGI_LOCAL_0_IRQ, no_action, IRQF_NO_THREAD, b"local0 cascade\0".as_ptr() as *const i8, core::ptr::null_mut());
    request_irq(SGI_LOCAL_1_IRQ, no_action, IRQF_NO_THREAD, b"local1 cascade\0".as_ptr() as *const i8, core::ptr::null_mut());
    request_irq(SGI_BUSERR_IRQ, no_action, IRQF_NO_THREAD, b"Bus Error\0".as_ptr() as *const i8, core::ptr::null_mut());
    request_irq(SGI_MAP_0_IRQ, no_action, IRQF_NO_THREAD, b"mapable0 cascade\0".as_ptr() as *const i8, core::ptr::null_mut());
    // #ifdef USE_LIO3_IRQ: register SGI_MAP_1_IRQ as "mapable1 cascade".
    // #ifdef CONFIG_EISA: if (ip22_is_fullhouse()) ip22_eisa_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
