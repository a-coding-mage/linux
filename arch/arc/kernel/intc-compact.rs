// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011-12 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the surrounding kernel translation unit.

const NR_CPU_IRQS: u32 = 32; // number of irq lines coming in
const TIMER0_IRQ: u32 = 3; // Fixed by ISA

extern "C" {
    fn read_aux_reg(reg: u32) -> u32;
    fn write_aux_reg(reg: u32, value: u32);
    fn pr_info(fmt: *const u8, ...);
    fn panic(fmt: *const u8, ... ) -> !;
    fn irq_set_percpu_devid(irq: u32);
    fn irq_set_chip_and_handler(irq: u32, chip: *const irq_chip, handler: unsafe extern "C" fn());
    fn irq_domain_xlate_onecell();
    fn handle_percpu_irq();
    fn handle_level_irq();
    fn of_fwnode_handle(node: *mut device_node) -> *mut core::ffi::c_void;
    fn irq_domain_create_linear(
        fwnode: *mut core::ffi::c_void,
        size: u32,
        ops: *const irq_domain_ops,
        host_data: *mut core::ffi::c_void,
    ) -> *mut irq_domain;
    fn irq_set_default_domain(domain: *mut irq_domain);
    fn arch_local_save_flags() -> usize;
    fn arch_local_irq_restore(flags: usize);
}

const AUX_IRQ_LEV: u32 = 0;
const AUX_IENABLE: u32 = 0;
const STATUS_A2_MASK: usize = 0;
const STATUS_E2_MASK: usize = 0;
const STATUS_A1_MASK: usize = 0;
const STATUS_E1_MASK: usize = 0;

#[repr(C)]
pub struct irq_data {
    pub hwirq: u32,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe extern "C" fn(data: *mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(data: *mut irq_data)>,
}

#[repr(C)]
pub struct irq_domain;
#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct irq_domain_ops {
    pub xlate: Option<unsafe extern "C" fn()>,
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, u32, usize) -> i32>,
}

// Early Hardware specific Interrupt setup
// -Platform independent, needed for each CPU (not foldable into init_IRQ)
// -Called very early (start_kernel -> setup_arch -> setup_processor)
//
// what it does ?
// -Optionally, setup the High priority Interrupts as Level 2 IRQs
#[no_mangle]
pub unsafe extern "C" fn arc_init_IRQ() {
    let mut level_mask: u32 = 0;
    let mut i: u32;

    /* Is timer high priority Interrupt (Level2 in ARCompact jargon) */
    // CONFIG_ARC_COMPACT_IRQ_LEVELS is a build-time configuration option.
    if cfg!(feature = "CONFIG_ARC_COMPACT_IRQ_LEVELS") {
        level_mask |= 1u32 << TIMER0_IRQ;
    }

    /*
     * Write to register, even if no LV2 IRQs configured to reset it
     * in case bootloader had mucked with it
     */
    write_aux_reg(AUX_IRQ_LEV, level_mask);

    if level_mask != 0 {
        pr_info(b"Level-2 interrupts bitset %x\n\0".as_ptr());
    }

    /*
     * Disable all IRQ lines so faulty external hardware won't
     * trigger interrupt that kernel is not ready to handle.
     */
    i = TIMER0_IRQ;
    while i < NR_CPU_IRQS {
        let mut ienb: u32;

        ienb = read_aux_reg(AUX_IENABLE);
        ienb &= !(1u32 << i);
        write_aux_reg(AUX_IENABLE, ienb);
        i += 1;
    }
}

/*
 * ARC700 core includes a simple on-chip intc supporting
 * -per IRQ enable/disable
 * -2 levels of interrupts (high/low)
 * -all interrupts being level triggered
 *
 * To reduce platform code, we assume all IRQs directly hooked-up into intc.
 * Platforms with external intc, hence cascaded IRQs, are free to over-ride
 * below, per IRQ.
 */

unsafe extern "C" fn arc_irq_mask(data: *mut irq_data) {
    let mut ienb: u32;

    ienb = read_aux_reg(AUX_IENABLE);
    ienb &= !(1u32 << (*data).hwirq);
    write_aux_reg(AUX_IENABLE, ienb);
}

unsafe extern "C" fn arc_irq_unmask(data: *mut irq_data) {
    let mut ienb: u32;

    ienb = read_aux_reg(AUX_IENABLE);
    ienb |= 1u32 << (*data).hwirq;
    write_aux_reg(AUX_IENABLE, ienb);
}

static mut ONCHIP_INTC: irq_chip = irq_chip {
    name: b"ARC In-core Intc\0".as_ptr(),
    irq_mask: Some(arc_irq_mask),
    irq_unmask: Some(arc_irq_unmask),
};

unsafe extern "C" fn arc_intc_domain_map(
    _d: *mut irq_domain,
    irq: u32,
    hw: usize,
) -> i32 {
    match hw as u32 {
        TIMER0_IRQ => {
            irq_set_percpu_devid(irq);
            irq_set_chip_and_handler(irq, &ONCHIP_INTC, handle_percpu_irq);
        }
        _ => {
            irq_set_chip_and_handler(irq, &ONCHIP_INTC, handle_level_irq);
        }
    }
    0
}

static ARC_INTC_DOMAIN_OPS: irq_domain_ops = irq_domain_ops {
    xlate: Some(irq_domain_xlate_onecell),
    map: Some(arc_intc_domain_map),
};

unsafe extern "C" fn init_onchip_IRQ(
    intc: *mut device_node,
    parent: *mut device_node,
) -> i32 {
    let root_domain: *mut irq_domain;

    if !parent.is_null() {
        panic(b"DeviceTree incore intc not a root irq controller\n\0".as_ptr());
    }

    root_domain = irq_domain_create_linear(
        of_fwnode_handle(intc),
        NR_CPU_IRQS,
        &ARC_INTC_DOMAIN_OPS,
        core::ptr::null_mut(),
    );
    if root_domain.is_null() {
        panic(b"root irq domain not avail\n\0".as_ptr());
    }

    /*
     * Needed for primary domain lookup to succeed
     * This is a primary irqchip, and can never have a parent
     */
    irq_set_default_domain(root_domain);

    0
}

// IRQCHIP_DECLARE(arc_intc, "snps,arc700-intc", init_onchip_IRQ);

/*
 * arch_local_irq_enable - Enable interrupts.
 *
 * 1. Explicitly called to re-enable interrupts
 * 2. Implicitly called from spin_unlock_irq, write_unlock_irq etc
 *    which maybe in hard ISR itself
 *
 * Semantics of this function change depending on where it is called from:
 *
 * -If called from hard-ISR, it must not invert interrupt priorities
 *  e.g. suppose TIMER is high priority (Level 2) IRQ
 *    Time hard-ISR, timer_interrupt( ) calls spin_unlock_irq several times.
 *    Here local_irq_enable( ) shd not re-enable lower priority interrupts
 * -If called from soft-ISR, it must re-enable all interrupts
 *    soft ISR are low priority jobs which can be very slow, thus all IRQs
 *    must be enabled while they run.
 *    Now hardware context wise we may still be in L2 ISR (not done rtie)
 *    still we must re-enable both L1 and L2 IRQs
 *  Another twist is prev scenario with flow being
 *     L1 ISR ==> interrupted by L2 ISR  ==> L2 soft ISR
 *     here we must not re-enable Ll as prev Ll Interrupt's h/w context will get
 *     over-written (this is deficiency in ARC700 Interrupt mechanism)
 */

#[cfg(feature = "CONFIG_ARC_COMPACT_IRQ_LEVELS")]
#[no_mangle]
pub unsafe extern "C" fn arch_local_irq_enable() {
    let mut flags: usize = arch_local_save_flags();

    if flags & STATUS_A2_MASK != 0 {
        flags |= STATUS_E2_MASK;
    } else if flags & STATUS_A1_MASK != 0 {
        flags |= STATUS_E1_MASK;
    }

    arch_local_irq_restore(flags);
}

// EXPORT_SYMBOL(arch_local_irq_enable);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
