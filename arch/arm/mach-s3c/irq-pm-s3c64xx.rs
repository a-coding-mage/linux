// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//      Ben Dooks <ben@simtec.co.uk>
//      http://armlinux.simtec.co.uk/
//
// S3C64XX - Interrupt handling Power Management

/*
 * NOTE: Code in this file is not used when booting with Device Tree support.
 */

// Linux kernel, syscore, interrupt, serial, IRQ, I/O, device-tree, map,
// GPIO-register, CPU, and power-management dependencies are supplied by the
// surrounding translation unit.

/* We handled all the IRQ types in this code, to save having to make several
 * small files to handle each different type separately. Having the EINT_GRP
 * code here shouldn't be as much bloat as the IRQ table space needed when
 * they are enabled. The added benefit is we ensure that these registers are
 * in the same state as we suspended.
 */

// `SAVE_ITEM` entries from the C source; the sleep-save type and register
// constants are provided by the platform dependencies.
static mut irq_save: [sleep_save; 8] = [
    SAVE_ITEM!(S3C64XX_PRIORITY),
    SAVE_ITEM!(S3C64XX_EINT0CON0),
    SAVE_ITEM!(S3C64XX_EINT0CON1),
    SAVE_ITEM!(S3C64XX_EINT0FLTCON0),
    SAVE_ITEM!(S3C64XX_EINT0FLTCON1),
    SAVE_ITEM!(S3C64XX_EINT0FLTCON2),
    SAVE_ITEM!(S3C64XX_EINT0FLTCON3),
    SAVE_ITEM!(S3C64XX_EINT0MASK),
];

#[repr(C)]
struct irq_grp_save {
    fltcon: u32,
    con: u32,
    mask: u32,
}

static mut eint_grp_save: [irq_grp_save; 5] = [
    irq_grp_save { fltcon: 0, con: 0, mask: 0 },
    irq_grp_save { fltcon: 0, con: 0, mask: 0 },
    irq_grp_save { fltcon: 0, con: 0, mask: 0 },
    irq_grp_save { fltcon: 0, con: 0, mask: 0 },
    irq_grp_save { fltcon: 0, con: 0, mask: 0 },
];

// CONFIG_SERIAL_SAMSUNG_UARTS is a build-time configuration constant.
#[cfg(not(CONFIG_SERIAL_SAMSUNG_UARTS))]
const SERIAL_SAMSUNG_UARTS: usize = 0;
#[cfg(CONFIG_SERIAL_SAMSUNG_UARTS)]
const SERIAL_SAMSUNG_UARTS: usize = CONFIG_SERIAL_SAMSUNG_UARTS as usize;

static mut irq_uart_mask: [u32; SERIAL_SAMSUNG_UARTS] = [0; SERIAL_SAMSUNG_UARTS];

unsafe fn s3c64xx_irq_pm_suspend(data: *mut core::ffi::c_void) -> i32 {
    let mut grp: *mut irq_grp_save = eint_grp_save.as_mut_ptr();
    let mut i: usize;

    S3C_PMDBG!("%s: suspending IRQs\n", "s3c64xx_irq_pm_suspend");

    s3c_pm_do_save(irq_save.as_mut_ptr(), irq_save.len());

    i = 0;
    while i < SERIAL_SAMSUNG_UARTS {
        irq_uart_mask[i] = __raw_readl(S3C_VA_UARTx(i) + S3C64XX_UINTM);
        i += 1;
    }

    i = 0;
    while i < eint_grp_save.len() {
        (*grp).con = __raw_readl(S3C64XX_EINT12CON + (i * 4));
        (*grp).mask = __raw_readl(S3C64XX_EINT12MASK + (i * 4));
        (*grp).fltcon = __raw_readl(S3C64XX_EINT12FLTCON + (i * 4));
        grp = grp.add(1);
        i += 1;
    }

    0
}

unsafe fn s3c64xx_irq_pm_resume(data: *mut core::ffi::c_void) {
    let mut grp: *mut irq_grp_save = eint_grp_save.as_mut_ptr();
    let mut i: usize;

    S3C_PMDBG!("%s: resuming IRQs\n", "s3c64xx_irq_pm_resume");

    s3c_pm_do_restore(irq_save.as_mut_ptr(), irq_save.len());

    i = 0;
    while i < SERIAL_SAMSUNG_UARTS {
        __raw_writel(irq_uart_mask[i], S3C_VA_UARTx(i) + S3C64XX_UINTM);
        i += 1;
    }

    i = 0;
    while i < eint_grp_save.len() {
        __raw_writel((*grp).con, S3C64XX_EINT12CON + (i * 4));
        __raw_writel((*grp).mask, S3C64XX_EINT12MASK + (i * 4));
        __raw_writel((*grp).fltcon, S3C64XX_EINT12FLTCON + (i * 4));
        grp = grp.add(1);
        i += 1;
    }

    S3C_PMDBG!("%s: IRQ configuration restored\n", "s3c64xx_irq_pm_resume");
}

static s3c64xx_irq_syscore_ops: syscore_ops = syscore_ops {
    suspend: Some(s3c64xx_irq_pm_suspend),
    resume: Some(s3c64xx_irq_pm_resume),
};

static mut s3c64xx_irq_syscore: syscore = syscore {
    ops: &s3c64xx_irq_syscore_ops,
};

unsafe fn s3c64xx_syscore_init() -> i32 {
    /* Appropriate drivers (pinctrl, uart) handle this when using DT. */
    if of_have_populated_dt() || !soc_is_s3c64xx() {
        return 0;
    }

    register_syscore(&mut s3c64xx_irq_syscore);

    0
}

// Equivalent of core_initcall(s3c64xx_syscore_init).
core_initcall!(s3c64xx_syscore_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
