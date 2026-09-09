/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1992 Linus Torvalds
 * Copyright (C) 1994 - 2001, 2003, 07 Ralf Baechle
 */

// C includes are supplied by the surrounding kernel translation unit.

static mut R4030_LOCK: RawSpinlock = RawSpinlock::new();

unsafe fn enable_r4030_irq(d: *mut IrqData) {
    let mut mask: u32 = 1u32 << ((*d).irq - JAZZ_IRQ_START);
    let mut flags: c_ulong = 0;

    raw_spin_lock_irqsave(&mut R4030_LOCK, &mut flags);
    mask |= r4030_read_reg16(JAZZ_IO_IRQ_ENABLE) as u32;
    r4030_write_reg16(JAZZ_IO_IRQ_ENABLE, mask as u16);
    raw_spin_unlock_irqrestore(&mut R4030_LOCK, flags);
}

pub unsafe fn disable_r4030_irq(d: *mut IrqData) {
    let mut mask: u32 = !(1u32 << ((*d).irq - JAZZ_IRQ_START));
    let mut flags: c_ulong = 0;

    raw_spin_lock_irqsave(&mut R4030_LOCK, &mut flags);
    mask &= r4030_read_reg16(JAZZ_IO_IRQ_ENABLE) as u32;
    r4030_write_reg16(JAZZ_IO_IRQ_ENABLE, mask as u16);
    raw_spin_unlock_irqrestore(&mut R4030_LOCK, flags);
}

static mut R4030_IRQ_TYPE: IrqChip = IrqChip {
    name: "R4030\0".as_ptr() as *const c_char,
    irq_mask: Some(disable_r4030_irq),
    irq_unmask: Some(enable_r4030_irq),
};

pub unsafe fn init_r4030_ints() {
    let mut i: c_int;

    i = JAZZ_IRQ_START;
    while i <= JAZZ_IRQ_END {
        irq_set_chip_and_handler(i, &mut R4030_IRQ_TYPE, handle_level_irq);
        i += 1;
    }

    r4030_write_reg16(JAZZ_IO_IRQ_ENABLE, 0);
    r4030_read_reg16(JAZZ_IO_IRQ_SOURCE); /* clear pending IRQs */
    r4030_read_reg32(JAZZ_R4030_INVAL_ADDR); /* clear error bits */
}

/*
 * On systems with i8259-style interrupt controllers we assume for
 * driver compatibility reasons interrupts 0 - 15 to be the i8259
 * interrupts even if the hardware uses a different interrupt numbering.
 */
pub unsafe fn arch_init_irq() {
    /*
     * this is a hack to get back the still needed wired mapping
     * killed by init_mm()
     */

    /* Map 0xe0000000 -> 0x0:800005C0, 0xe0010000 -> 0x1:30000580 */
    add_wired_entry(0x02000017, 0x03c00017, 0xe0000000, PM_64K);
    /* Map 0xe2000000 -> 0x0:900005C0, 0xe3010000 -> 0x0:910005C0 */
    add_wired_entry(0x02400017, 0x02440017, 0xe2000000, PM_16M);
    /* Map 0xe4000000 -> 0x0:600005C0, 0xe4100000 -> 400005C0 */
    add_wired_entry(0x01800017, 0x01000017, 0xe4000000, PM_4M);

    init_i8259_irqs(); /* Integrated i8259 */
    mips_cpu_irq_init();
    init_r4030_ints();

    change_c0_status(ST0_IM, IE_IRQ2 | IE_IRQ1);
}

pub unsafe fn plat_irq_dispatch() {
    let pending: u32 = read_c0_cause() & read_c0_status();
    let mut irq: u32;

    if pending & IE_IRQ4 != 0 {
        r4030_read_reg32(JAZZ_TIMER_REGISTER);
        do_IRQ(JAZZ_TIMER_IRQ);
    } else if pending & IE_IRQ2 != 0 {
        irq = *(JAZZ_EISA_IRQ_ACK as *const volatile u8) as u32;
        do_IRQ(irq);
    } else if pending & IE_IRQ1 != 0 {
        irq = (JAZZ_IO_IRQ_SOURCE as *const volatile u8).read() as u32 >> 2;
        if likely(irq > 0) {
            do_IRQ(irq + JAZZ_IRQ_START as u32 - 1);
        } else {
            panic!("Unimplemented loc_no_irq handler");
        }
    }
}

pub static mut R4030_CLOCKEVENT: ClockEventDevice = ClockEventDevice {
    name: "r4030\0".as_ptr() as *const c_char,
    features: CLOCK_EVT_FEAT_PERIODIC,
    rating: 300,
    irq: JAZZ_TIMER_IRQ,
};

unsafe fn r4030_timer_interrupt(_irq: c_int, dev_id: *mut c_void) -> Irqreturn {
    let cd = dev_id as *mut ClockEventDevice;

    ((*cd).event_handler)(cd);
    IRQ_HANDLED
}

pub unsafe fn plat_time_init() {
    let cd: *mut ClockEventDevice = &mut R4030_CLOCKEVENT;
    let cpu: c_uint = smp_processor_id();

    BUG_ON(HZ != 100);

    (*cd).cpumask = cpumask_of(cpu);
    clockevents_register_device(cd);
    if request_irq(JAZZ_TIMER_IRQ, Some(r4030_timer_interrupt), IRQF_TIMER,
                   "R4030 timer\0".as_ptr() as *const c_char, cd) != 0 {
        pr_err!("Failed to register R4030 timer interrupt\n");
    }

    /*
     * Set clock to 100Hz.
     *
     * The R4030 timer receives an input clock of 1kHz which is divided by
     * a programmable 4-bit divider.  This makes it fairly inflexible.
     */
    r4030_write_reg32(JAZZ_TIMER_INTERVAL, 9);
    setup_pit_timer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
