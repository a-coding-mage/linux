// SPDX-License-Identifier: GPL-2.0-only
/*
 * bcsr.h -- Db1xxx/Pb1xxx Devboard CPLD registers ("BCSR") abstraction.
 *
 * All Alchemy development boards (except, of course, the weird PB1000)
 * have a few registers in a CPLD with standardised layout; they mostly
 * only differ in base address.
 * All registers are 16bits wide with 32bit spacing.
 */

// Linux and architecture dependencies supplied by other files.

#[repr(C)]
struct BcsrReg {
    raddr: *mut core::ffi::c_void,
    lock: spinlock_t,
}

static mut BCSR_REGS: [BcsrReg; BCSR_CNT] = [const { BcsrReg {
    raddr: core::ptr::null_mut(),
    lock: spinlock_t::new(),
} }; BCSR_CNT];

static mut BCSR_VIRT: *mut core::ffi::c_void = core::ptr::null_mut();
static mut BCSR_CSC_BASE: i32 = 0;

pub unsafe fn bcsr_init(mut bcsr1_phys: usize, mut bcsr2_phys: usize) {
    let mut i: i32;

    bcsr1_phys = KSEG1ADDR(CPHYSADDR(bcsr1_phys));
    bcsr2_phys = KSEG1ADDR(CPHYSADDR(bcsr2_phys));

    BCSR_VIRT = bcsr1_phys as *mut core::ffi::c_void;

    i = 0;
    while i < BCSR_CNT {
        if i >= BCSR_HEXLEDS {
            BCSR_REGS[i as usize].raddr = (bcsr2_phys + (0x04 * (i - BCSR_HEXLEDS)) as usize)
                as *mut core::ffi::c_void;
        } else {
            BCSR_REGS[i as usize].raddr = (bcsr1_phys + (0x04 * i) as usize)
                as *mut core::ffi::c_void;
        }

        spin_lock_init(&mut BCSR_REGS[i as usize].lock);
        i += 1;
    }
}

pub unsafe fn bcsr_read(reg: enum_bcsr_id) -> u16 {
    let mut r: u16;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut BCSR_REGS[reg as usize].lock, &mut flags);
    r = __raw_readw(BCSR_REGS[reg as usize].raddr);
    spin_unlock_irqrestore(&mut BCSR_REGS[reg as usize].lock, flags);
    r
}

pub unsafe fn bcsr_write(reg: enum_bcsr_id, val: u16) {
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut BCSR_REGS[reg as usize].lock, &mut flags);
    __raw_writew(val, BCSR_REGS[reg as usize].raddr);
    wmb();
    spin_unlock_irqrestore(&mut BCSR_REGS[reg as usize].lock, flags);
}

pub unsafe fn bcsr_mod(reg: enum_bcsr_id, clr: u16, set: u16) {
    let mut r: u16;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut BCSR_REGS[reg as usize].lock, &mut flags);
    r = __raw_readw(BCSR_REGS[reg as usize].raddr);
    r &= !clr;
    r |= set;
    __raw_writew(r, BCSR_REGS[reg as usize].raddr);
    wmb();
    spin_unlock_irqrestore(&mut BCSR_REGS[reg as usize].lock, flags);
}

/* DB1200/PB1200 CPLD IRQ muxer */
unsafe fn bcsr_csc_handler(d: *mut irq_desc) {
    let bisr: u16 = __raw_readw(BCSR_VIRT.add(BCSR_REG_INTSTAT as usize));
    let chip: *mut irq_chip = irq_desc_get_chip(d);

    chained_irq_enter(chip, d);
    generic_handle_irq(BCSR_CSC_BASE + __ffs(bisr as usize) as i32);
    chained_irq_exit(chip, d);
}

unsafe fn bcsr_irq_mask(d: *mut irq_data) {
    let v: u16 = 1u16 << ((*d).irq - BCSR_CSC_BASE) as u32;
    __raw_writew(v, BCSR_VIRT.add(BCSR_REG_MASKCLR as usize));
    wmb();
}

unsafe fn bcsr_irq_maskack(d: *mut irq_data) {
    let v: u16 = 1u16 << ((*d).irq - BCSR_CSC_BASE) as u32;
    __raw_writew(v, BCSR_VIRT.add(BCSR_REG_MASKCLR as usize));
    __raw_writew(v, BCSR_VIRT.add(BCSR_REG_INTSTAT as usize)); // ack
    wmb();
}

unsafe fn bcsr_irq_unmask(d: *mut irq_data) {
    let v: u16 = 1u16 << ((*d).irq - BCSR_CSC_BASE) as u32;
    __raw_writew(v, BCSR_VIRT.add(BCSR_REG_MASKSET as usize));
    wmb();
}

static mut BCSR_IRQ_TYPE: irq_chip = irq_chip {
    name: "CPLD",
    irq_mask: Some(bcsr_irq_mask),
    irq_mask_ack: Some(bcsr_irq_maskack),
    irq_unmask: Some(bcsr_irq_unmask),
};

pub unsafe fn bcsr_init_irq(csc_start: i32, csc_end: i32, hook_irq: i32) {
    let mut irq: u32;

    /* mask & enable & ack all */
    __raw_writew(0xffff, BCSR_VIRT.add(BCSR_REG_MASKCLR as usize));
    __raw_writew(0xffff, BCSR_VIRT.add(BCSR_REG_INTSET as usize));
    __raw_writew(0xffff, BCSR_VIRT.add(BCSR_REG_INTSTAT as usize));
    wmb();

    BCSR_CSC_BASE = csc_start;

    irq = csc_start as u32;
    while irq <= csc_end as u32 {
        irq_set_chip_and_handler_name(
            irq as i32,
            &mut BCSR_IRQ_TYPE,
            handle_level_irq,
            "level",
        );
        irq += 1;
    }

    irq_set_chained_handler(hook_irq, bcsr_csc_handler);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
