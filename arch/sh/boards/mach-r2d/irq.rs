// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/renesas/rts7751r2d/irq.c
 *
 * Copyright (C) 2007  Magnus Damm
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * Renesas Technology Sales RTS7751R2D Support, R2D-PLUS and R2D-1.
 *
 * Modified for RTS7751R2D by
 * Atom Create Engineering Co., Ltd. 2002.
 */

const R2D_NR_IRL: usize = 13;

#[repr(C)]
#[derive(Copy, Clone)]
enum IrqSource {
    UNUSED = 0,
    // board specific interrupt sources (R2D-1 and R2D-PLUS)
    EXT,
    // EXT_INT0-3
    RTC_T,
    RTC_A,
    // Real Time Clock
    AX88796,
    // Ethernet controller (R2D-1 board)
    KEY,
    // Key input (R2D-PLUS board)
    SDCARD,
    CF_CD,
    CF_IDE,
    // CF Card Detect + CF IDE
    SM501,
    // SM501 aka Voyager
    PCI_INTD_RTL8139,
    // Ethernet controller
    PCI_INTC_PCI1520,
    // Cardbus/PCMCIA bridge
    PCI_INTB_RTL8139,
    // Ethernet controller with HUB (R2D-PLUS board)
    PCI_INTB_SLOT,
    // PCI Slot 3.3v (R2D-1 board)
    PCI_INTA_SLOT,
    // PCI Slot 3.3v
    TP,
    // Touch Panel
}

#[cfg(CONFIG_RTS7751R2D_1)]
static mut vectors_r2d_1: [intc_vect; 13] = [
    INTC_IRQ!(EXT, IRQ_EXT),
    INTC_IRQ!(RTC_T, IRQ_RTC_T), INTC_IRQ!(RTC_A, IRQ_RTC_A),
    INTC_IRQ!(AX88796, IRQ_AX88796), INTC_IRQ!(SDCARD, IRQ_SDCARD),
    INTC_IRQ!(CF_CD, IRQ_CF_CD), INTC_IRQ!(CF_IDE, IRQ_CF_IDE), // ng
    INTC_IRQ!(SM501, IRQ_VOYAGER),
    INTC_IRQ!(PCI_INTD_RTL8139, IRQ_PCI_INTD),
    INTC_IRQ!(PCI_INTC_PCI1520, IRQ_PCI_INTC),
    INTC_IRQ!(PCI_INTB_SLOT, IRQ_PCI_INTB),
    INTC_IRQ!(PCI_INTA_SLOT, IRQ_PCI_INTA),
    INTC_IRQ!(TP, IRQ_TP),
];

#[cfg(CONFIG_RTS7751R2D_1)]
static mut mask_registers_r2d_1: [intc_mask_reg; 1] = [intc_mask_reg {
    addr: 0xa4000000,
    set_reg: 0,
    clr_reg: 16,
    enum_ids: [TP, PCI_INTA_SLOT, PCI_INTB_SLOT,
        PCI_INTC_PCI1520, PCI_INTD_RTL8139,
        SM501, CF_IDE, CF_CD, SDCARD, AX88796,
        RTC_A, RTC_T, UNUSED, UNUSED, UNUSED, EXT],
}];

#[cfg(CONFIG_RTS7751R2D_1)]
static mut irl2irq_r2d_1: [u8; R2D_NR_IRL] = [
    IRQ_PCI_INTD, IRQ_CF_IDE, IRQ_CF_CD, IRQ_PCI_INTC,
    IRQ_VOYAGER, IRQ_AX88796, IRQ_RTC_A, IRQ_RTC_T,
    IRQ_SDCARD, IRQ_PCI_INTA, IRQ_PCI_INTB, IRQ_EXT,
    IRQ_TP,
];

#[cfg(CONFIG_RTS7751R2D_1)]
static mut intc_desc_r2d_1: intc_desc = DECLARE_INTC_DESC!(
    "r2d-1", vectors_r2d_1, None, mask_registers_r2d_1, None, None
);

#[cfg(CONFIG_RTS7751R2D_PLUS)]
static mut vectors_r2d_plus: [intc_vect; 13] = [
    INTC_IRQ!(EXT, IRQ_EXT),
    INTC_IRQ!(RTC_T, IRQ_RTC_T), INTC_IRQ!(RTC_A, IRQ_RTC_A),
    INTC_IRQ!(KEY, IRQ_KEY), INTC_IRQ!(SDCARD, IRQ_SDCARD),
    INTC_IRQ!(CF_CD, IRQ_CF_CD), INTC_IRQ!(CF_IDE, IRQ_CF_IDE),
    INTC_IRQ!(SM501, IRQ_VOYAGER),
    INTC_IRQ!(PCI_INTD_RTL8139, IRQ_PCI_INTD),
    INTC_IRQ!(PCI_INTC_PCI1520, IRQ_PCI_INTC),
    INTC_IRQ!(PCI_INTB_RTL8139, IRQ_PCI_INTB),
    INTC_IRQ!(PCI_INTA_SLOT, IRQ_PCI_INTA),
    INTC_IRQ!(TP, IRQ_TP),
];

#[cfg(CONFIG_RTS7751R2D_PLUS)]
static mut mask_registers_r2d_plus: [intc_mask_reg; 1] = [intc_mask_reg {
    addr: 0xa4000000,
    set_reg: 0,
    clr_reg: 16,
    enum_ids: [TP, PCI_INTA_SLOT, PCI_INTB_RTL8139,
        PCI_INTC_PCI1520, PCI_INTD_RTL8139,
        SM501, CF_IDE, CF_CD, SDCARD, KEY,
        RTC_A, RTC_T, UNUSED, UNUSED, UNUSED, EXT],
}];

#[cfg(CONFIG_RTS7751R2D_PLUS)]
static mut irl2irq_r2d_plus: [u8; R2D_NR_IRL] = [
    IRQ_PCI_INTD, IRQ_CF_IDE, IRQ_CF_CD, IRQ_PCI_INTC,
    IRQ_VOYAGER, IRQ_KEY, IRQ_RTC_A, IRQ_RTC_T,
    IRQ_SDCARD, IRQ_PCI_INTA, IRQ_PCI_INTB, IRQ_EXT,
    IRQ_TP,
];

#[cfg(CONFIG_RTS7751R2D_PLUS)]
static mut intc_desc_r2d_plus: intc_desc = DECLARE_INTC_DESC!(
    "r2d-plus", vectors_r2d_plus, None, mask_registers_r2d_plus, None, None
);

static mut irl2irq: [u8; R2D_NR_IRL] = [0; R2D_NR_IRL];

unsafe fn rts7751r2d_irq_demux(irq: i32) -> i32 {
    if irq >= (R2D_NR_IRL as i32) + 16 || irq < 16 || irl2irq[(irq - 16) as usize] == 0 {
        return irq;
    }
    irl2irq[(irq - 16) as usize] as i32
}

/*
 * Initialize IRQ setting
 */
unsafe fn init_rts7751r2d_IRQ() {
    let d: *mut intc_desc;

    match __raw_readw(PA_VERREG) & 0xf0 {
        #[cfg(CONFIG_RTS7751R2D_PLUS)]
        0x10 => {
            printk!(KERN_INFO, "Using R2D-PLUS interrupt controller.\n");
            d = &raw mut intc_desc_r2d_plus;
            core::ptr::copy_nonoverlapping(irl2irq_r2d_plus.as_ptr(), irl2irq.as_mut_ptr(), R2D_NR_IRL);
        }
        #[cfg(CONFIG_RTS7751R2D_1)]
        0x00 | 0x30 => {
            printk!(KERN_INFO, "Using R2D-1 interrupt controller.\n");
            d = &raw mut intc_desc_r2d_1;
            core::ptr::copy_nonoverlapping(irl2irq_r2d_1.as_ptr(), irl2irq.as_mut_ptr(), R2D_NR_IRL);
        }
        _ => {
            printk!(KERN_INFO, "Unknown R2D interrupt controller 0x%04x\n", __raw_readw(PA_VERREG));
            return;
        }
    }

    register_intc_controller(d);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
