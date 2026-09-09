/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright            : (C) 2002 by Frank Mori Hess                   *
 ***************************************************************************/

// C header guard: _HP82335_H

// Dependencies supplied by other translated files:
// #include "tms9914.h"
// #include "gpibP.h"

// struct which defines private_data for board
#[repr(C)]
pub struct hp82335_priv {
    pub tms9914_priv: tms9914_priv,
    pub irq: ::core::ffi::c_uint,
    pub raw_iobase: ::core::ffi::c_ulong,
}

// size of io memory region used
pub const hp82335_rom_size: ::core::ffi::c_int = 0x2000;
pub const hp82335_upper_iomem_size: ::core::ffi::c_int = 0x2000;

// hp82335 register offsets
#[repr(i32)]
pub enum hp_read_regs {
    HPREG_CSR = 0x17f8,
    HPREG_STATUS = 0x1ffc,
}

#[repr(i32)]
pub enum hp_write_regs {
    HPREG_INTR_CLEAR = 0x17f7,
    HPREG_CCR = HPREG_CSR as i32,
}

#[repr(i32)]
pub enum ccr_bits {
    DMA_ENABLE = 1 << 0,       /* DMA enable                  */
    DMA_CHAN_SELECT = 1 << 1,  /* DMA channel select  O=3,1=2 */
    INTR_ENABLE = 1 << 2,      /* interrupt enable            */
    SYS_DISABLE = 1 << 3,      /* system controller disable   */
}

#[repr(i32)]
pub enum csr_bits {
    SWITCH6 = 1 << 0,             /* switch 6 position           */
    SWITCH5 = 1 << 1,             /* switch 5 position           */
    SYS_CONTROLLER = 1 << 2,      /* system controller bit       */
    DMA_ENABLE_STATUS = 1 << 4,   /* DMA enabled                 */
    DMA_CHAN_STATUS = 1 << 5,     /* DMA channel   0=3,1=2       */
    INTR_ENABLE_STATUS = 1 << 6,  /* Interrupt enable            */
    INTR_PENDING = 1 << 7,        /* Interrupt Pending           */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
