/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1992 Linus Torvalds
 * Copyright (C) 1994 - 2000 Ralf Baechle
 * Copyright (C) 2006 Thomas Bogendoerfer
 */

// Dependencies supplied by the surrounding kernel translation.

pub type IrqreturnT = i32;

extern "C" {
    static mut sni_hwint: Option<unsafe extern "C" fn()>;

    fn i8259_irq() -> i32;
    fn generic_handle_irq(irq: i32);
    fn init_i8259_irqs();
    fn sni_a20r_irq_init();
    fn sni_pcit_irq_init();
    fn sni_pcit_cplus_irq_init();
    fn sni_rm200_irq_init();
    fn sni_pcimt_irq_init();

    static sni_brd_type: i32;
    static SNI_BRD_10: i32;
    static SNI_BRD_10NEW: i32;
    static SNI_BRD_TOWER_OASIC: i32;
    static SNI_BRD_MINITOWER: i32;
    static SNI_BRD_PCI_TOWER: i32;
    static SNI_BRD_PCI_TOWER_CPLUS: i32;
    static SNI_BRD_RM200: i32;
    static SNI_BRD_PCI_MTOWER: i32;
    static SNI_BRD_PCI_DESKTOP: i32;
    static SNI_BRD_PCI_MTOWER_CPLUS: i32;
    static IRQ_NONE: IrqreturnT;
    static IRQ_HANDLED: IrqreturnT;
}

pub unsafe extern "C" fn plat_irq_dispatch() {
    if let Some(handler) = sni_hwint {
        handler();
    }
}

/* ISA irq handler */
pub unsafe extern "C" fn sni_isa_irq_handler(_dummy: i32, _p: *mut core::ffi::c_void) -> IrqreturnT {
    let irq: i32 = i8259_irq();
    if irq < 0 {
        return IRQ_NONE;
    }

    generic_handle_irq(irq);
    IRQ_HANDLED
}

/*
 * On systems with i8259-style interrupt controllers we assume for
 * driver compatibility reasons interrupts 0 - 15 to be the i8295
 * interrupts even if the hardware uses a different interrupt numbering.
 */
pub unsafe extern "C" fn arch_init_irq() {
    init_i8259_irqs(); /* Integrated i8259  */
    match sni_brd_type {
        x if x == SNI_BRD_10 || x == SNI_BRD_10NEW || x == SNI_BRD_TOWER_OASIC || x == SNI_BRD_MINITOWER => {
            sni_a20r_irq_init();
        }

        x if x == SNI_BRD_PCI_TOWER => {
            sni_pcit_irq_init();
        }

        x if x == SNI_BRD_PCI_TOWER_CPLUS => {
            sni_pcit_cplus_irq_init();
        }

        x if x == SNI_BRD_RM200 => {
            sni_rm200_irq_init();
        }

        x if x == SNI_BRD_PCI_MTOWER || x == SNI_BRD_PCI_DESKTOP || x == SNI_BRD_PCI_MTOWER_CPLUS => {
            sni_pcimt_irq_init();
        }

        _ => {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
