/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependencies supplied by other headers:
// use linux_types::u32;
// use asm_schid::subchannel_id;

/*
 * I/O-Interruption Code as stored by TEST PENDING INTERRUPTION (TPI).
 *
 * The C bit-fields are represented by their containing 32-bit words.  The
 * masks and shifts below preserve the original field layout and intent.
 */
#[repr(C, packed(4))]
pub struct tpi_info {
    pub schid: subchannel_id,
    pub intparm: u32,
    pub adapter_IO_directed_irq_isc_reserved: u32,
    pub type_reserved: u32,
}

pub const TPI_INFO_ADAPTER_IO_SHIFT: u32 = 0;
pub const TPI_INFO_ADAPTER_IO_MASK: u32 = 0x1;
pub const TPI_INFO_DIRECTED_IRQ_SHIFT: u32 = 1;
pub const TPI_INFO_DIRECTED_IRQ_MASK: u32 = 0x1 << TPI_INFO_DIRECTED_IRQ_SHIFT;
pub const TPI_INFO_ISC_SHIFT: u32 = 2;
pub const TPI_INFO_ISC_MASK: u32 = 0x7 << TPI_INFO_ISC_SHIFT;
pub const TPI_INFO_TYPE_SHIFT: u32 = 0;
pub const TPI_INFO_TYPE_MASK: u32 = 0x7;

/* I/O-Interruption Code as stored by TPI for an Adapter I/O */
#[repr(C, packed(4))]
pub struct tpi_adapter_info {
    pub aism_reserved_error_forward: u32,
    pub reserved: u32,
    pub adapter_IO_directed_irq_isc_reserved: u32,
}

pub const TPI_ADAPTER_INFO_AISM_SHIFT: u32 = 0;
pub const TPI_ADAPTER_INFO_AISM_MASK: u32 = 0xff;
pub const TPI_ADAPTER_INFO_ERROR_SHIFT: u32 = 30;
pub const TPI_ADAPTER_INFO_ERROR_MASK: u32 = 0x1 << TPI_ADAPTER_INFO_ERROR_SHIFT;
pub const TPI_ADAPTER_INFO_FORWARD_SHIFT: u32 = 31;
pub const TPI_ADAPTER_INFO_FORWARD_MASK: u32 = 0x1 << TPI_ADAPTER_INFO_FORWARD_SHIFT;
pub const TPI_ADAPTER_INFO_ADAPTER_IO_SHIFT: u32 = 0;
pub const TPI_ADAPTER_INFO_ADAPTER_IO_MASK: u32 = 0x1;
pub const TPI_ADAPTER_INFO_DIRECTED_IRQ_SHIFT: u32 = 1;
pub const TPI_ADAPTER_INFO_DIRECTED_IRQ_MASK: u32 = 0x1 << TPI_ADAPTER_INFO_DIRECTED_IRQ_SHIFT;
pub const TPI_ADAPTER_INFO_ISC_SHIFT: u32 = 2;
pub const TPI_ADAPTER_INFO_ISC_MASK: u32 = 0x7 << TPI_ADAPTER_INFO_ISC_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
