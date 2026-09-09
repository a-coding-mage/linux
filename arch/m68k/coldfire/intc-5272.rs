/*
 * intc.c  --  interrupt controller for ColdFire 5272 SoC
 *
 * (C) Copyright 2009, Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Linux and ColdFire headers supply the declarations referenced below. */

#[repr(C)]
struct irqmap {
    icr: u32,
    index: u8,
    ack: u8,
}

static mut INTC_IRQMAP: [irqmap; (MCFINT_VECMAX - MCFINT_VECBASE) as usize] = [
    irqmap { icr: 0, index: 0, ack: 0 },
    irqmap { icr: MCFSIM_ICR1, index: 28, ack: 1 },
    irqmap { icr: MCFSIM_ICR1, index: 24, ack: 1 },
    irqmap { icr: MCFSIM_ICR1, index: 20, ack: 1 },
    irqmap { icr: MCFSIM_ICR1, index: 16, ack: 1 },
    irqmap { icr: MCFSIM_ICR1, index: 12, ack: 0 },
    irqmap { icr: MCFSIM_ICR1, index: 8, ack: 0 },
    irqmap { icr: MCFSIM_ICR1, index: 4, ack: 0 },
    irqmap { icr: MCFSIM_ICR1, index: 0, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 28, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 24, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 20, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 16, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 12, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 8, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 4, ack: 0 },
    irqmap { icr: MCFSIM_ICR2, index: 0, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 28, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 24, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 20, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 16, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 12, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 8, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 4, ack: 0 },
    irqmap { icr: MCFSIM_ICR3, index: 0, ack: 0 },
    irqmap { icr: MCFSIM_ICR4, index: 28, ack: 0 },
    irqmap { icr: MCFSIM_ICR4, index: 24, ack: 1 },
    irqmap { icr: MCFSIM_ICR4, index: 20, ack: 1 },
    irqmap { icr: MCFSIM_ICR4, index: 16, ack: 0 },
];

unsafe fn intc_irq_mask(d: *mut irq_data) {
    let mut irq = (*d).irq;
    if irq >= MCFINT_VECBASE && irq <= MCFINT_VECMAX {
        irq -= MCFINT_VECBASE;
        let v: u32 = 0x8u32 << INTC_IRQMAP[irq as usize].index;
        mcf_write32(v, INTC_IRQMAP[irq as usize].icr);
    }
}

unsafe fn intc_irq_unmask(d: *mut irq_data) {
    let mut irq = (*d).irq;
    if irq >= MCFINT_VECBASE && irq <= MCFINT_VECMAX {
        irq -= MCFINT_VECBASE;
        let v: u32 = 0xd_u32 << INTC_IRQMAP[irq as usize].index;
        mcf_write32(v, INTC_IRQMAP[irq as usize].icr);
    }
}

unsafe fn intc_irq_ack(d: *mut irq_data) {
    let mut irq = (*d).irq;
    if irq >= MCFINT_VECBASE && irq <= MCFINT_VECMAX {
        irq -= MCFINT_VECBASE;
        if INTC_IRQMAP[irq as usize].ack != 0 {
            let mut v = mcf_read32(INTC_IRQMAP[irq as usize].icr);
            v &= 0x7u32 << INTC_IRQMAP[irq as usize].index;
            v |= 0x8u32 << INTC_IRQMAP[irq as usize].index;
            mcf_write32(v, INTC_IRQMAP[irq as usize].icr);
        }
    }
}

unsafe fn intc_irq_set_type(d: *mut irq_data, irq_type: unsigned_int) -> int {
    let mut irq = (*d).irq;
    if irq >= MCFINT_VECBASE && irq <= MCFINT_VECMAX {
        irq -= MCFINT_VECBASE;
        if INTC_IRQMAP[irq as usize].ack != 0 {
            let mut v = mcf_read32(MCFSIM_PITR);
            if irq_type == IRQ_TYPE_EDGE_FALLING {
                v &= !(0x1u32 << (32 - irq));
            } else {
                v |= 0x1u32 << (32 - irq);
            }
            mcf_write32(v, MCFSIM_PITR);
        }
    }
    0
}

unsafe fn intc_external_irq(desc: *mut irq_desc) {
    irq_desc_get_chip(desc).irq_ack(&mut (*desc).irq_data);
    handle_simple_irq(desc);
}

static mut intc_irq_chip: irq_chip = irq_chip {
    name: "CF-INTC",
    irq_mask: Some(intc_irq_mask),
    irq_unmask: Some(intc_irq_unmask),
    irq_mask_ack: Some(intc_irq_mask),
    irq_ack: Some(intc_irq_ack),
    irq_set_type: Some(intc_irq_set_type),
};

pub unsafe fn init_IRQ() {
    mcf_write32(0x88888888, MCFSIM_ICR1);
    mcf_write32(0x88888888, MCFSIM_ICR2);
    mcf_write32(0x88888888, MCFSIM_ICR3);
    mcf_write32(0x88888888, MCFSIM_ICR4);

    let mut irq: int = 0;
    while irq < NR_IRQS {
        irq_set_chip(irq, &mut intc_irq_chip);
        let mut edge = 0;
        if irq >= MCFINT_VECBASE && irq <= MCFINT_VECMAX {
            edge = INTC_IRQMAP[(irq - MCFINT_VECBASE) as usize].ack as int;
        }
        if edge != 0 {
            irq_set_irq_type(irq, IRQ_TYPE_EDGE_RISING);
            irq_set_handler(irq, intc_external_irq);
        } else {
            irq_set_irq_type(irq, IRQ_TYPE_LEVEL_HIGH);
            irq_set_handler(irq, handle_level_irq);
        }
        irq += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
