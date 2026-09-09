// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-footbridge/irq.c
 *
 *  Copyright (C) 1996-2000 Russell King
 *
 *  Changelog:
 *   22-Aug-1998 RMK	Restructured IRQ routines
 *   03-Sep-1998 PJB	Merged CATS support
 *   20-Jan-1998 RMK	Started merge of EBSA286, CATS and NetWinder
 *   26-Jan-1999 PJB	Don't use IACK on CATS
 *   16-Mar-1999 RMK	Added autodetect of ISA PICs
 */

unsafe fn isa_mask_pic_lo_irq(d: *mut irq_data) {
    let mask: u32 = 1 << ((*d).irq & 7);
    outb(inb(PIC_MASK_LO) | mask as u8, PIC_MASK_LO);
}

unsafe fn isa_ack_pic_lo_irq(d: *mut irq_data) {
    let mask: u32 = 1 << ((*d).irq & 7);
    outb(inb(PIC_MASK_LO) | mask as u8, PIC_MASK_LO);
    outb(0x20, PIC_LO);
}

unsafe fn isa_unmask_pic_lo_irq(d: *mut irq_data) {
    let mask: u32 = 1 << ((*d).irq & 7);
    outb(inb(PIC_MASK_LO) & !(mask as u8), PIC_MASK_LO);
}

static mut isa_lo_chip: irq_chip = irq_chip {
    irq_ack: Some(isa_ack_pic_lo_irq),
    irq_mask: Some(isa_mask_pic_lo_irq),
    irq_unmask: Some(isa_unmask_pic_lo_irq),
};

unsafe fn isa_mask_pic_hi_irq(d: *mut irq_data) {
    let mask: u32 = 1 << ((*d).irq & 7);
    outb(inb(PIC_MASK_HI) | mask as u8, PIC_MASK_HI);
}

unsafe fn isa_ack_pic_hi_irq(d: *mut irq_data) {
    let mask: u32 = 1 << ((*d).irq & 7);
    outb(inb(PIC_MASK_HI) | mask as u8, PIC_MASK_HI);
    outb(0x62, PIC_LO);
    outb(0x20, PIC_HI);
}

unsafe fn isa_unmask_pic_hi_irq(d: *mut irq_data) {
    let mask: u32 = 1 << ((*d).irq & 7);
    outb(inb(PIC_MASK_HI) & !(mask as u8), PIC_MASK_HI);
}

static mut isa_hi_chip: irq_chip = irq_chip {
    irq_ack: Some(isa_ack_pic_hi_irq),
    irq_mask: Some(isa_mask_pic_hi_irq),
    irq_unmask: Some(isa_unmask_pic_hi_irq),
};

unsafe fn isa_irq_handler(desc: *mut irq_desc) {
    let isa_irq: u32 = *(PCIIACK_BASE as *const u8) as u32;

    if isa_irq < _ISA_IRQ(0) || isa_irq >= _ISA_IRQ(16) {
        handle_bad_irq(desc);
        return;
    }

    generic_handle_irq(isa_irq);
}

static mut pic1_resource: resource = resource {
    name: "pic1",
    start: 0x20,
    end: 0x3f,
};

static mut pic2_resource: resource = resource {
    name: "pic2",
    start: 0xa0,
    end: 0xbf,
};

unsafe fn isa_init_irq(mut host_irq: u32) {
    let mut irq: u32;

    /*
     * Setup, and then probe for an ISA PIC
     * If the PIC is not there, then we
     * ignore the PIC.
     */
    outb(0x11, PIC_LO);
    outb(_ISA_IRQ(0) as u8, PIC_MASK_LO); // IRQ number
    outb(0x04, PIC_MASK_LO); // Slave on Ch2
    outb(0x01, PIC_MASK_LO); // x86
    outb(0xf5, PIC_MASK_LO); // pattern: 11110101

    outb(0x11, PIC_HI);
    outb(_ISA_IRQ(8) as u8, PIC_MASK_HI); // IRQ number
    outb(0x02, PIC_MASK_HI); // Slave on Ch1
    outb(0x01, PIC_MASK_HI); // x86
    outb(0xfa, PIC_MASK_HI); // pattern: 11111010

    outb(0x0b, PIC_LO);
    outb(0x0b, PIC_HI);

    if inb(PIC_MASK_LO) == 0xf5 && inb(PIC_MASK_HI) == 0xfa {
        outb(0xff, PIC_MASK_LO); // mask all IRQs
        outb(0xff, PIC_MASK_HI); // mask all IRQs
    } else {
        printk(KERN_INFO, "IRQ: ISA PIC not found\n");
        host_irq = u32::MAX;
    }

    if host_irq != u32::MAX {
        irq = _ISA_IRQ(0);
        while irq < _ISA_IRQ(8) {
            irq_set_chip_and_handler(irq, &raw mut isa_lo_chip, handle_level_irq);
            irq_clear_status_flags(irq, IRQ_NOREQUEST | IRQ_NOPROBE);
            irq += 1;
        }

        irq = _ISA_IRQ(8);
        while irq < _ISA_IRQ(16) {
            irq_set_chip_and_handler(irq, &raw mut isa_hi_chip, handle_level_irq);
            irq_clear_status_flags(irq, IRQ_NOREQUEST | IRQ_NOPROBE);
            irq += 1;
        }

        request_resource(&raw mut ioport_resource, &raw mut pic1_resource);
        request_resource(&raw mut ioport_resource, &raw mut pic2_resource);

        irq = IRQ_ISA_CASCADE;
        if request_irq(irq, no_action, 0, "cascade", core::ptr::null_mut()) != 0 {
            pr_err!("Failed to request irq {} (cascade)\n", irq);
        }

        irq_set_chained_handler(host_irq, isa_irq_handler);

        /*
         * On the NetWinder, don't automatically
         * enable ISA IRQ11 when it is requested.
         * There appears to be a missing pull-up
         * resistor on this line.
         */
        if machine_is_netwinder() {
            irq_modify_status(_ISA_IRQ(11),
                IRQ_NOREQUEST | IRQ_NOPROBE, IRQ_NOAUTOEN);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
