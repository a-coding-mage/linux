/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 * Quancom pci stuff
 * copyright (C) 2005 by Frank Mori Hess
 ***************************************************************************/

/* quancom registers */
#[repr(i32)]
pub enum quancom_regs {
    QUANCOM_IRQ_CONTROL_STATUS_REG = 0xfc,
}

#[repr(i32)]
pub enum quancom_irq_control_status_bits {
    QUANCOM_IRQ_ASSERTED_BIT = 0x1, /* readable */
    /* (any write to the register clears the interrupt)*/
    QUANCOM_IRQ_ENABLE_BIT = 0x4, /* writeable */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
