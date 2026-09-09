// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC irq.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_ulong;

extern "C" {
    fn mfspr(spr: c_ulong) -> c_ulong;
    fn mtspr(spr: c_ulong, value: c_ulong);
    fn irqchip_init();
}

// C preprocessor constants supplied by the OpenRISC dependencies:
// SPR_SR, SPR_SR_IEE, and SPR_SR_TEE.

/* read interrupt enabled status */
pub unsafe fn arch_local_save_flags() -> c_ulong {
    mfspr(SPR_SR) & (SPR_SR_IEE | SPR_SR_TEE)
}

// EXPORT_SYMBOL(arch_local_save_flags);

/* set interrupt enabled status */
pub unsafe fn arch_local_irq_restore(flags: c_ulong) {
    mtspr(
        SPR_SR,
        (mfspr(SPR_SR) & !(SPR_SR_IEE | SPR_SR_TEE)) | flags,
    );
}

// EXPORT_SYMBOL(arch_local_irq_restore);

// The C __init annotation is a linker/build attribute supplied by the kernel.
pub unsafe fn init_IRQ() {
    irqchip_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
