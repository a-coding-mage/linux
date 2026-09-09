// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Freescale Semiconductor, Inc.
 *
 * Author: Varun Sethi <varun.sethi@freescale.com>
 */

// Linux and architecture dependencies supplied by the surrounding translation.

const MPIC_ERR_INT_BASE: usize = 0x3900;
const MPIC_ERR_INT_EISR: u32 = 0x0000;
const MPIC_ERR_INT_EIMR: u32 = 0x0010;

#[inline]
unsafe fn mpic_fsl_err_read(base: *mut u32, err_reg: u32) -> u32 {
    in_be32(base.add((err_reg >> 2) as usize))
}

#[inline]
unsafe fn mpic_fsl_err_write(base: *mut u32, value: u32) {
    out_be32(base.add((MPIC_ERR_INT_EIMR >> 2) as usize), value);
}

unsafe fn fsl_mpic_mask_err(d: *mut irq_data) {
    let mut eimr: u32;
    let mpic: *mut mpic = irq_data_get_irq_chip_data(d);
    let src: u32 = virq_to_hw((*d).irq) - (*mpic).err_int_vecs[0];

    eimr = mpic_fsl_err_read((*mpic).err_regs, MPIC_ERR_INT_EIMR);
    eimr |= 1u32 << (31 - src);
    mpic_fsl_err_write((*mpic).err_regs, eimr);
}

unsafe fn fsl_mpic_unmask_err(d: *mut irq_data) {
    let mut eimr: u32;
    let mpic: *mut mpic = irq_data_get_irq_chip_data(d);
    let src: u32 = virq_to_hw((*d).irq) - (*mpic).err_int_vecs[0];

    eimr = mpic_fsl_err_read((*mpic).err_regs, MPIC_ERR_INT_EIMR);
    eimr &= !(1u32 << (31 - src));
    mpic_fsl_err_write((*mpic).err_regs, eimr);
}

static mut fsl_mpic_err_chip: irq_chip = irq_chip {
    irq_disable: Some(fsl_mpic_mask_err),
    irq_mask: Some(fsl_mpic_mask_err),
    irq_unmask: Some(fsl_mpic_unmask_err),
};

unsafe fn mpic_setup_error_int(mpic: *mut mpic, mut intvec: i32) -> i32 {
    (*mpic).err_regs = ioremap((*mpic).paddr + MPIC_ERR_INT_BASE, 0x1000);
    if (*mpic).err_regs.is_null() {
        pr_err!("could not map mpic error registers\n");
        return -ENOMEM;
    }
    (*mpic).hc_err = fsl_mpic_err_chip;
    (*mpic).hc_err.name = (*mpic).name;
    (*mpic).flags |= MPIC_FSL_HAS_EIMR;
    /* allocate interrupt vectors for error interrupts */
    let mut i = MPIC_MAX_ERR as i32 - 1;
    while i >= 0 {
        (*mpic).err_int_vecs[i as usize] = intvec;
        intvec -= 1;
        i -= 1;
    }

    0
}

unsafe fn mpic_map_error_int(mpic: *mut mpic, virq: u32, hw: irq_hw_number_t) -> i32 {
    if ((*mpic).flags & MPIC_FSL_HAS_EIMR) != 0
        && hw >= (*mpic).err_int_vecs[0]
        && hw <= (*mpic).err_int_vecs[MPIC_MAX_ERR - 1]
    {
        WARN_ON!((*mpic).flags & MPIC_SECONDARY);

        pr_debug!("mpic: mapping as Error Interrupt\n");
        irq_set_chip_data(virq, mpic);
        irq_set_chip_and_handler(virq, &mut (*mpic).hc_err, handle_level_irq);
        return 1;
    }

    0
}

unsafe fn fsl_error_int_handler(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let mpic: *mut mpic = data as *mut mpic;
    let mut eisr: u32;
    let mut eimr: u32;
    let mut errint: i32;

    eisr = mpic_fsl_err_read((*mpic).err_regs, MPIC_ERR_INT_EISR);
    eimr = mpic_fsl_err_read((*mpic).err_regs, MPIC_ERR_INT_EIMR);

    if (eisr & !eimr) == 0 {
        return IRQ_NONE;
    }

    while eisr != 0 {
        let ret: i32;
        errint = eisr.leading_zeros() as i32;
        ret = generic_handle_domain_irq((*mpic).irqhost,
                                        (*mpic).err_int_vecs[errint as usize]);
        if WARN_ON!(ret != 0) {
            eimr |= 1u32 << (31 - errint as u32);
            mpic_fsl_err_write((*mpic).err_regs, eimr);
        }
        eisr &= !(1u32 << (31 - errint as u32));
    }

    IRQ_HANDLED
}

unsafe fn mpic_err_int_init(mpic: *mut mpic, irqnum: irq_hw_number_t) {
    let virq: u32;
    let ret: i32;

    virq = irq_create_mapping((*mpic).irqhost, irqnum);
    if virq == 0 {
        pr_err!("Error interrupt setup failed\n");
        return;
    }

    /* Mask all error interrupts */
    mpic_fsl_err_write((*mpic).err_regs, !0);

    ret = request_irq(virq, fsl_error_int_handler, IRQF_NO_THREAD,
                      "mpic-error-int", mpic as *mut core::ffi::c_void);
    if ret != 0 {
        pr_err!("Failed to register error interrupt handler\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
