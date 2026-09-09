/*
 * linux/arch/m68k/kernel/ints.c -- Linux/m68k general interrupt handling code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// C headers and local dependencies are supplied by the surrounding kernel.

extern "C" {
    static mut auto_irqhandler_fixup: [u32; 0];
    static mut user_irqvec_fixup: [u16; 0];
}

static mut m68k_first_user_vec: i32 = 0;

static mut auto_irq_chip: irq_chip = irq_chip {
    name: b"auto\0".as_ptr() as *const i8,
    irq_startup: Some(m68k_irq_startup),
    irq_shutdown: Some(m68k_irq_shutdown),
};

static mut user_irq_chip: irq_chip = irq_chip {
    name: b"user\0".as_ptr() as *const i8,
    irq_startup: Some(m68k_irq_startup),
    irq_shutdown: Some(m68k_irq_shutdown),
};

/*
 * void init_IRQ(void)
 *
 * Parameters: None
 * Returns: Nothing
 *
 * This function should be called during kernel startup to initialize
 * the IRQ handling routines.
 */
pub unsafe extern "C" fn init_IRQ() {
    let mut i: i32;

    i = IRQ_AUTO_1;
    while i <= IRQ_AUTO_7 {
        irq_set_chip_and_handler(
            i as u32,
            &mut auto_irq_chip,
            handle_simple_irq,
        );
        i += 1;
    }

    mach_init_IRQ();
}

/**
 * m68k_setup_auto_interrupt
 * @handler: called from auto vector interrupts
 *
 * setup the handler to be called from auto vector interrupts instead of the
 * standard do_IRQ(), it will be called with irq numbers in the range
 * from IRQ_AUTO_1 - IRQ_AUTO_7.
 */
pub unsafe extern "C" fn m68k_setup_auto_interrupt(
    handler: Option<unsafe extern "C" fn(u32, *mut pt_regs)>,
) {
    if let Some(handler) = handler {
        auto_irqhandler_fixup[0] = handler as usize as u32;
    }
    flush_icache();
}

/**
 * m68k_setup_user_interrupt
 * @vec: first user vector interrupt to handle
 * @cnt: number of active user vector interrupts
 *
 * setup user vector interrupts, this includes activating the specified range
 * of interrupts, only then these interrupts can be requested (note: this is
 * different from auto vector interrupts).
 */
pub unsafe extern "C" fn m68k_setup_user_interrupt(vec: u32, cnt: u32) {
    let mut i: u32;

    BUG_ON(IRQ_USER + cnt > NR_IRQS);
    m68k_first_user_vec = vec as i32;
    i = 0;
    while i < cnt {
        irq_set_chip_and_handler(i, &mut user_irq_chip, handle_simple_irq);
        i += 1;
    }
    user_irqvec_fixup[0] = (vec - IRQ_USER) as u16;
    flush_icache();
}

/**
 * m68k_setup_irq_controller
 * @chip: irq chip which controls specified irq
 * @handle: flow handler which handles specified irq
 * @irq: first irq to be managed by the controller
 * @cnt: number of irqs to be managed by the controller
 */
pub unsafe extern "C" fn m68k_setup_irq_controller(
    chip: *mut irq_chip,
    handle: irq_flow_handler_t,
    irq: u32,
    cnt: u32,
) {
    let mut i: u32 = 0;

    while i < cnt {
        irq_set_chip(irq + i, chip);
        if handle.is_some() {
            irq_set_handler(irq + i, handle);
        }
        i += 1;
    }
}

pub unsafe extern "C" fn m68k_irq_startup_irq(irq: u32) -> u32 {
    if irq <= IRQ_AUTO_7 {
        vectors[(VEC_SPUR + irq) as usize] = auto_inthandler;
    } else {
        vectors[(m68k_first_user_vec as u32 + irq - IRQ_USER) as usize] = user_inthandler;
    }
    0
}

pub unsafe extern "C" fn m68k_irq_startup(data: *mut irq_data) -> u32 {
    m68k_irq_startup_irq((*data).irq)
}

pub unsafe extern "C" fn m68k_irq_shutdown(data: *mut irq_data) {
    let irq: u32 = (*data).irq;

    if irq <= IRQ_AUTO_7 {
        vectors[(VEC_SPUR + irq) as usize] = bad_inthandler;
    } else {
        vectors[(m68k_first_user_vec as u32 + irq - IRQ_USER) as usize] = bad_inthandler;
    }
}

pub unsafe extern "C" fn irq_canonicalize(mut irq: u32) -> u32 {
    #[cfg(CONFIG_Q40)]
    {
        if MACH_IS_Q40 && irq == 11 {
            irq = 10;
        }
    }
    irq
}

pub unsafe extern "C" fn handle_badint(regs: *mut pt_regs) {
    atomic_inc(&mut irq_err_count);
    pr_warn!("unexpected interrupt from %u\n", (*regs).vector);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
