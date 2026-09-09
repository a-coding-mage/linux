// SPDX-License-Identifier: GPL-2.0
// C header dependencies are supplied by the surrounding kernel translation.

/*
 * ISA PIC or low IO-APIC triggered (INTA-cycle or APIC) interrupts:
 * (these are usually mapped to vectors 0x30-0x3f)
 */

/*
 * The IO-APIC gives us many more interrupt sources. Most of these
 * are unused but an SMP system is supposed to have enough memory ...
 * sometimes (mostly wrt. hw bugs) we get corrupted vectors all
 * across the spectrum, so we really want to be prepared to get all
 * of these. Plus, more powerful systems might have more than 64
 * IO-APIC registers.
 *
 * (these are usually mapped into the 0x30-0xff vector range)
 */

// DEFINE_PER_CPU(vector_irq_t, vector_irq) = { [0 ... NR_VECTORS - 1] = VECTOR_UNUSED };
#[no_mangle]
pub static mut vector_irq: [vector_irq_t; NR_VECTORS] = [VECTOR_UNUSED; NR_VECTORS];

pub unsafe fn init_ISA_irqs() {
    let chip = (*legacy_pic).chip;
    let mut i: i32;

    /*
     * Try to set up the through-local-APIC virtual wire mode earlier.
     *
     * On some 32-bit UP machines, whose APIC has been disabled by BIOS
     * and then got re-enabled by "lapic", it hangs at boot time without this.
     */
    init_bsp_APIC();

    ((*legacy_pic).init)(0);

    i = 0;
    while i < nr_legacy_irqs() {
        irq_set_chip_and_handler(i, chip, handle_level_irq);
        irq_set_status_flags(i, IRQ_LEVEL);
        i += 1;
    }
}

pub unsafe fn init_IRQ() {
    let mut i: i32;

    /*
     * On cpu 0, Assign ISA_IRQ_VECTOR(irq) to IRQ 0..15.
     * If these IRQ's are handled by legacy interrupt-controllers like PIC,
     * then this configuration will likely be static after the boot. If
     * these IRQs are handled by more modern controllers like IO-APIC,
     * then this vector space can be freed and re-used dynamically as the
     * irq's migrate etc.
     */
    i = 0;
    while i < nr_legacy_irqs() {
        per_cpu(vector_irq, 0)[ISA_IRQ_VECTOR(i)] = irq_to_desc(i);
        i += 1;
    }

    BUG_ON(irq_init_percpu_irqstack(smp_processor_id()));

    (x86_init.irqs.intr_init)();
}

pub unsafe fn native_init_IRQ() {
    /* Execute any quirks before the call gates are initialised: */
    (x86_init.irqs.pre_vector_init)();

    /* FRED's IRQ path may be used even if FRED isn't fully enabled. */
    if IS_ENABLED(CONFIG_X86_FRED) {
        fred_complete_exception_setup();
    }

    if !cpu_feature_enabled(X86_FEATURE_FRED) {
        idt_setup_apic_and_irq_gates();
    }

    lapic_assign_system_vectors();

    if !acpi_ioapic && !of_ioapic && nr_legacy_irqs() != 0 {
        /* IRQ2 is cascade interrupt to second interrupt controller */
        if request_irq(2, no_action, IRQF_NO_THREAD, c"cascade", core::ptr::null_mut()) != 0 {
            pr_err(c"%s: request_irq() failed\n", c"cascade");
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
