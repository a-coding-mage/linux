/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const DAIF_PROCCTX: u64 = 0;
pub const DAIF_PROCCTX_NOIRQ: u64 = PSR_I_BIT | PSR_F_BIT;
pub const DAIF_ERRCTX: u64 = PSR_A_BIT | PSR_I_BIT | PSR_F_BIT;
pub const DAIF_MASK: u64 = PSR_D_BIT | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT;

/* mask/save/unmask/restore all exceptions, including interrupts. */
#[inline(always)]
pub unsafe fn local_daif_mask() {
    warn_on(
        system_has_prio_mask_debugging()
            && (read_sysreg_s(SYS_ICC_PMR_EL1) == (GIC_PRIO_IRQOFF | GIC_PRIO_PSR_I_SET)),
    );

    core::arch::asm!("msr daifset, #0xf // local_daif_mask", options(nostack, preserves_flags));

    /* Don't really care for a dsb here, we don't intend to enable IRQs */
    if system_uses_irq_prio_masking() {
        gic_write_pmr(GIC_PRIO_IRQON | GIC_PRIO_PSR_I_SET);
    }

    trace_hardirqs_off();
}

#[inline(always)]
pub unsafe fn local_daif_save_flags() -> u64 {
    let mut flags: u64;

    flags = read_sysreg(daif);

    if system_uses_irq_prio_masking() {
        /* If IRQs are masked with PMR, reflect it in the flags */
        if read_sysreg_s(SYS_ICC_PMR_EL1) != GIC_PRIO_IRQON {
            flags |= PSR_I_BIT | PSR_F_BIT;
        }
    }

    flags
}

#[inline(always)]
pub unsafe fn local_daif_save() -> u64 {
    let flags = local_daif_save_flags();

    local_daif_mask();

    flags
}

#[inline(always)]
pub unsafe fn local_daif_restore(mut flags: u64) {
    let irq_disabled = flags & PSR_I_BIT != 0;

    warn_on(
        system_has_prio_mask_debugging()
            && (read_sysreg(daif) & (PSR_I_BIT | PSR_F_BIT)) != (PSR_I_BIT | PSR_F_BIT),
    );

    if !irq_disabled {
        trace_hardirqs_on();

        if system_uses_irq_prio_masking() {
            gic_write_pmr(GIC_PRIO_IRQON);
            pmr_sync();
        }
    } else if system_uses_irq_prio_masking() {
        let pmr: u64;

        if flags & PSR_A_BIT == 0 {
            /*
             * If interrupts are disabled but we can take
             * asynchronous errors, we can take NMIs
             */
            flags &= !(PSR_I_BIT | PSR_F_BIT);
            pmr = GIC_PRIO_IRQOFF;
        } else {
            pmr = GIC_PRIO_IRQON | GIC_PRIO_PSR_I_SET;
        }

        /*
         * There has been concern that the write to daif
         * might be reordered before this write to PMR.
         * From the ARM ARM DDI 0487D.a, section D1.7.1
         * "Accessing PSTATE fields":
         *   Writes to the PSTATE fields have side-effects on
         *   various aspects of the PE operation. All of these
         *   side-effects are guaranteed:
         *     - Not to be visible to earlier instructions in
         *       the execution stream.
         *     - To be visible to later instructions in
         *       the execution stream
         *
         * Also, writes to PMR are self-synchronizing, so no
         * interrupts with a lower priority than PMR is signaled
         * to the PE after the write.
         *
         * So we don't need additional synchronization here.
         */
        gic_write_pmr(pmr);
    }

    write_sysreg(flags, daif);

    if irq_disabled {
        trace_hardirqs_off();
    }
}

/*
 * Called by synchronous exception handlers to restore the DAIF bits that were
 * modified by taking an exception.
 */
#[inline(always)]
pub unsafe fn local_daif_inherit(regs: *mut pt_regs) {
    let flags = (*regs).pstate & DAIF_MASK;

    if !regs_irqs_disabled(regs) {
        trace_hardirqs_on();
    }

    if system_uses_irq_prio_masking() {
        gic_write_pmr((*regs).pmr);
    }

    /*
     * We can't use local_daif_restore(regs->pstate) here as
     * system_has_prio_mask_debugging() won't restore the I bit if it can
     * use the pmr instead.
     */
    write_sysreg(flags, daif);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
