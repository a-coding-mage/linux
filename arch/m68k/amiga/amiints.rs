/*
 * Amiga Linux interrupt handling code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

use crate::amiga_hw::*;
use crate::amiga_ints::*;
use crate::amipcmcia::*;
use crate::irq::*;
use crate::traps::*;

/* Enable/disable a particular machine specific interrupt source. */
unsafe fn amiga_irq_enable(data: *mut irq_data) {
    amiga_custom.intena = IF_SETCLR | (1u16 << ((*data).irq - IRQ_USER));
}

unsafe fn amiga_irq_disable(data: *mut irq_data) {
    amiga_custom.intena = 1u16 << ((*data).irq - IRQ_USER);
}

static mut amiga_irq_chip: irq_chip = irq_chip {
    name: "amiga",
    irq_enable: Some(amiga_irq_enable),
    irq_disable: Some(amiga_irq_disable),
};

/* The builtin Amiga hardware interrupt handlers. */

unsafe fn ami_int1(_desc: *mut irq_desc) {
    let ints: u16 = amiga_custom.intreqr & amiga_custom.intenar;

    /* if serial transmit buffer empty, interrupt */
    if ints & IF_TBE != 0 {
        amiga_custom.intreq = IF_TBE;
        generic_handle_irq(IRQ_AMIGA_TBE);
    }

    /* if floppy disk transfer complete, interrupt */
    if ints & IF_DSKBLK != 0 {
        amiga_custom.intreq = IF_DSKBLK;
        generic_handle_irq(IRQ_AMIGA_DSKBLK);
    }

    /* if software interrupt set, interrupt */
    if ints & IF_SOFT != 0 {
        amiga_custom.intreq = IF_SOFT;
        generic_handle_irq(IRQ_AMIGA_SOFT);
    }
}

unsafe fn ami_int3(_desc: *mut irq_desc) {
    let ints: u16 = amiga_custom.intreqr & amiga_custom.intenar;

    /* if a blitter interrupt */
    if ints & IF_BLIT != 0 {
        amiga_custom.intreq = IF_BLIT;
        generic_handle_irq(IRQ_AMIGA_BLIT);
    }

    /* if a copper interrupt */
    if ints & IF_COPER != 0 {
        amiga_custom.intreq = IF_COPER;
        generic_handle_irq(IRQ_AMIGA_COPPER);
    }

    /* if a vertical blank interrupt */
    if ints & IF_VERTB != 0 {
        amiga_custom.intreq = IF_VERTB;
        generic_handle_irq(IRQ_AMIGA_VERTB);
    }
}

unsafe fn ami_int4(_desc: *mut irq_desc) {
    let ints: u16 = amiga_custom.intreqr & amiga_custom.intenar;

    /* if audio 0 interrupt */
    if ints & IF_AUD0 != 0 {
        amiga_custom.intreq = IF_AUD0;
        generic_handle_irq(IRQ_AMIGA_AUD0);
    }
    /* if audio 1 interrupt */
    if ints & IF_AUD1 != 0 {
        amiga_custom.intreq = IF_AUD1;
        generic_handle_irq(IRQ_AMIGA_AUD1);
    }
    /* if audio 2 interrupt */
    if ints & IF_AUD2 != 0 {
        amiga_custom.intreq = IF_AUD2;
        generic_handle_irq(IRQ_AMIGA_AUD2);
    }
    /* if audio 3 interrupt */
    if ints & IF_AUD3 != 0 {
        amiga_custom.intreq = IF_AUD3;
        generic_handle_irq(IRQ_AMIGA_AUD3);
    }
}

unsafe fn ami_int5(_desc: *mut irq_desc) {
    let ints: u16 = amiga_custom.intreqr & amiga_custom.intenar;

    /* if serial receive buffer full interrupt */
    if ints & IF_RBF != 0 {
        /* acknowledge of IF_RBF must be done by the serial interrupt */
        generic_handle_irq(IRQ_AMIGA_RBF);
    }

    /* if a disk sync interrupt */
    if ints & IF_DSKSYN != 0 {
        amiga_custom.intreq = IF_DSKSYN;
        generic_handle_irq(IRQ_AMIGA_DSKSYN);
    }
}

/*
 * void amiga_init_IRQ(void)
 *
 * Parameters: None
 * Returns: Nothing
 */
pub unsafe fn amiga_init_IRQ() {
    m68k_setup_irq_controller(&mut amiga_irq_chip, handle_simple_irq, IRQ_USER, AMI_STD_IRQS);

    irq_set_chained_handler(IRQ_AUTO_1, ami_int1);
    irq_set_chained_handler(IRQ_AUTO_3, ami_int3);
    irq_set_chained_handler(IRQ_AUTO_4, ami_int4);
    irq_set_chained_handler(IRQ_AUTO_5, ami_int5);

    /* turn off PCMCIA interrupts */
    if AMIGAHW_PRESENT(PCMCIA) {
        gayle.inten = GAYLE_IRQ_IDE;
    }

    /* turn off all interrupts and enable the master interrupt bit */
    amiga_custom.intena = 0x7fff;
    amiga_custom.intreq = 0x7fff;
    amiga_custom.intena = IF_SETCLR | IF_INTEN;

    cia_init_IRQ(&mut ciaa_base);
    cia_init_IRQ(&mut ciab_base);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
