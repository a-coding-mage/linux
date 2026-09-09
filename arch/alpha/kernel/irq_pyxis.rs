// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/irq_pyxis.c
 *
 * Based on code written by David A Rusling (david.rusling@reo.mts.dec.com).
 *
 * IRQ Code common to all PYXIS core logic chips.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/sched.h, linux/irq.h, asm/io.h, asm/core_cia.h,
// proto.h, and irq_impl.h.

/* Note mask bit is true for ENABLED irqs.  */
static mut cached_irq_mask: ::core::ffi::c_ulong = 0;

#[inline]
unsafe fn pyxis_update_irq_hw(mask: ::core::ffi::c_ulong) {
    *(PYXIS_INT_MASK as *mut ::core::ffi::c_ulong) = mask;
    mb();
    let _ = *(PYXIS_INT_MASK as *const ::core::ffi::c_ulong);
}

#[inline]
unsafe fn pyxis_enable_irq(d: *mut irq_data) {
    let bit = 1usize.wrapping_shl((*d).irq.wrapping_sub(16));
    cached_irq_mask |= bit as ::core::ffi::c_ulong;
    pyxis_update_irq_hw(cached_irq_mask);
}

unsafe fn pyxis_disable_irq(d: *mut irq_data) {
    let bit = 1usize.wrapping_shl((*d).irq.wrapping_sub(16));
    cached_irq_mask &= !(bit as ::core::ffi::c_ulong);
    pyxis_update_irq_hw(cached_irq_mask);
}

unsafe fn pyxis_mask_and_ack_irq(d: *mut irq_data) {
    let bit = 1usize.wrapping_shl((*d).irq.wrapping_sub(16));
    let bit = bit as ::core::ffi::c_ulong;
    cached_irq_mask &= !bit;
    let mask = cached_irq_mask;

    /* Disable the interrupt.  */
    *(PYXIS_INT_MASK as *mut ::core::ffi::c_ulong) = mask;
    wmb();
    /* Ack PYXIS PCI interrupt.  */
    *(PYXIS_INT_REQ as *mut ::core::ffi::c_ulong) = bit;
    mb();
    /* Re-read to force both writes.  */
    let _ = *(PYXIS_INT_MASK as *const ::core::ffi::c_ulong);
}

static mut pyxis_irq_type: irq_chip = irq_chip {
    name: "PYXIS\\0".as_ptr() as *const ::core::ffi::c_char,
    irq_mask_ack: Some(pyxis_mask_and_ack_irq),
    irq_mask: Some(pyxis_disable_irq),
    irq_unmask: Some(pyxis_enable_irq),
};

unsafe fn pyxis_device_interrupt(vector: ::core::ffi::c_ulong) {
    let mut pld: ::core::ffi::c_ulong;
    let mut i: ::core::ffi::c_uint;

    /* Read the interrupt summary register of PYXIS */
    pld = *(PYXIS_INT_REQ as *const ::core::ffi::c_ulong);
    pld &= cached_irq_mask;

    /*
     * Now for every possible bit set, work through them and call
     * the appropriate interrupt handler.
     */
    while pld != 0 {
        i = (!pld).trailing_zeros();
        pld &= pld.wrapping_sub(1); /* clear least bit set */
        if i == 7 {
            isa_device_interrupt(vector);
        } else {
            handle_irq(16 + i);
        }
    }
}

unsafe fn init_pyxis_irqs(ignore_mask: ::core::ffi::c_ulong) {
    let mut i: isize;

    *(PYXIS_INT_MASK as *mut ::core::ffi::c_ulong) = 0; /* disable all */
    *(PYXIS_INT_REQ as *mut ::core::ffi::c_ulong) = !0; /* flush all */
    mb();

    /* Send -INTA pulses to clear any pending interrupts ...*/
    let _ = *(CIA_IACK_SC as *mut ::core::ffi::c_uint);

    i = 16;
    while i < 48 {
        if ((ignore_mask >> i) & 1) != 0 {
            i += 1;
            continue;
        }
        irq_set_chip_and_handler(i as ::core::ffi::c_uint, &raw mut pyxis_irq_type, handle_level_irq);
        irq_set_status_flags(i as ::core::ffi::c_uint, IRQ_LEVEL);
        i += 1;
    }

    if request_irq(16 + 7, no_action, 0, "isa-cascade\\0".as_ptr() as *const ::core::ffi::c_char, ::core::ptr::null_mut()) != 0 {
        pr_err("Failed to register isa-cascade interrupt\\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
