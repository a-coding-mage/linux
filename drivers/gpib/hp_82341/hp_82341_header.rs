/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright            : (C) 2002, 2005 by Frank Mori Hess             *
 ***************************************************************************/

// #include "tms9914.h"
// #include "gpibP.h"

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hp_82341_hardware_version {
    HW_VERSION_UNKNOWN,
    HW_VERSION_82341C,
    HW_VERSION_82341D,
}

// struct which defines private_data for board
#[repr(C)]
pub struct hp_82341_priv {
    pub tms9914_priv: tms9914_priv,
    pub irq: ::core::ffi::c_uint,
    pub config_control_bits: ::core::ffi::c_ushort,
    pub mode_control_bits: ::core::ffi::c_ushort,
    pub event_status_bits: ::core::ffi::c_ushort,
    pub pnp_dev: *mut pnp_dev,
    pub iobase: [::core::ffi::c_ulong; 4],
    pub io_region_offset: ::core::ffi::c_ulong,
    pub hw_version: hp_82341_hardware_version,
}

pub const hp_82341_region_iosize: ::core::ffi::c_int = 0x8;
pub const hp_82341_num_io_regions: ::core::ffi::c_int = 4;
pub const hp_82341_fifo_size: ::core::ffi::c_int = 0xffe;
pub const hp_82341c_firmware_length: ::core::ffi::c_int = 5764;
pub const hp_82341d_firmware_length: ::core::ffi::c_int = 5302;

// hp 82341 register offsets
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hp_82341_region_0_registers {
    CONFIG_CONTROL_STATUS_REG = 0x0,
    MODE_CONTROL_STATUS_REG = 0x1,
    MONITOR_REG = 0x2, // after initialization
    XILINX_DATA_REG = 0x2, // before initialization, write only
    INTERRUPT_ENABLE_REG = 0x3,
    EVENT_STATUS_REG = 0x4,
    EVENT_ENABLE_REG = 0x5,
    STREAM_STATUS_REG = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hp_82341_region_1_registers {
    ID0_REG = 0x2,
    ID1_REG = 0x3,
    TRANSFER_COUNT_LOW_REG = 0x4,
    TRANSFER_COUNT_MID_REG = 0x5,
    TRANSFER_COUNT_HIGH_REG = 0x6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hp_82341_region_3_registers {
    BUFFER_PORT_LOW_REG = 0x0,
    BUFFER_PORT_HIGH_REG = 0x1,
    ID2_REG = 0x2,
    ID3_REG = 0x3,
    BUFFER_FLUSH_REG = 0x4,
    BUFFER_CONTROL_REG = 0x7,
}

pub const IRQ_SELECT_MASK: ::core::ffi::c_uint = 0x7;
pub const DMA_CONFIG_MASK: ::core::ffi::c_uint = 0x18;
pub const ENABLE_DMA_CONFIG_BIT: ::core::ffi::c_uint = 0x20;
pub const XILINX_READY_BIT: ::core::ffi::c_uint = 0x40; // read only
pub const DONE_PGL_BIT: ::core::ffi::c_uint = 0x80;

pub unsafe fn IRQ_SELECT_BITS(irq: ::core::ffi::c_int) -> ::core::ffi::c_uint {
    match irq {
        3 => 0x3,
        5 => 0x2,
        7 => 0x1,
        9 => 0x0,
        10 => 0x7,
        11 => 0x6,
        12 => 0x5,
        15 => 0x4,
        _ => 0x0,
    }
}

pub const SLOT8_BIT: ::core::ffi::c_uint = 0x1; // read only
pub const ACTIVE_CONTROLLER_BIT: ::core::ffi::c_uint = 0x2; // read only
pub const ENABLE_DMA_BIT: ::core::ffi::c_uint = 0x4;
pub const SYSTEM_CONTROLLER_BIT: ::core::ffi::c_uint = 0x8;
pub const MONITOR_BIT: ::core::ffi::c_uint = 0x10;
pub const ENABLE_IRQ_CONFIG_BIT: ::core::ffi::c_uint = 0x20;
pub const ENABLE_TI_STREAM_BIT: ::core::ffi::c_uint = 0x40;

pub const MONITOR_INTERRUPT_PENDING_BIT: ::core::ffi::c_uint = 0x1; // read only
pub const MONITOR_CLEAR_HOLDOFF_BIT: ::core::ffi::c_uint = 0x2; // write only
pub const MONITOR_PPOLL_BIT: ::core::ffi::c_uint = 0x4; // write clear
pub const MONITOR_SRQ_BIT: ::core::ffi::c_uint = 0x8; // write clear
pub const MONITOR_IFC_BIT: ::core::ffi::c_uint = 0x10; // write clear
pub const MONITOR_REN_BIT: ::core::ffi::c_uint = 0x20; // write clear
pub const MONITOR_END_BIT: ::core::ffi::c_uint = 0x40; // write clear
pub const MONITOR_DAV_BIT: ::core::ffi::c_uint = 0x80; // write clear

pub const ENABLE_TI_INTERRUPT_BIT: ::core::ffi::c_uint = 0x1;
pub const ENABLE_POINTERS_EQUAL_INTERRUPT_BIT: ::core::ffi::c_uint = 0x4;
pub const ENABLE_BUFFER_END_INTERRUPT_BIT: ::core::ffi::c_uint = 0x10;
pub const ENABLE_TERMINAL_COUNT_INTERRUPT_BIT: ::core::ffi::c_uint = 0x20;
pub const ENABLE_DMA_TERMINAL_COUNT_INTERRUPT_BIT: ::core::ffi::c_uint = 0x80;

pub const TI_INTERRUPT_EVENT_BIT: ::core::ffi::c_uint = 0x1; // write clear
pub const INTERRUPT_PENDING_EVENT_BIT: ::core::ffi::c_uint = 0x2; // read only
pub const POINTERS_EQUAL_EVENT_BIT: ::core::ffi::c_uint = 0x4; // write clear
pub const BUFFER_END_EVENT_BIT: ::core::ffi::c_uint = 0x10; // write clear
pub const TERMINAL_COUNT_EVENT_BIT: ::core::ffi::c_uint = 0x20; // write clear
pub const DMA_TERMINAL_COUNT_EVENT_BIT: ::core::ffi::c_uint = 0x80; // write clear

pub const ENABLE_TI_INTERRUPT_EVENT_BIT: ::core::ffi::c_uint = 0x1; // write clear
pub const ENABLE_POINTERS_EQUAL_EVENT_BIT: ::core::ffi::c_uint = 0x4; // write clear
pub const ENABLE_BUFFER_END_EVENT_BIT: ::core::ffi::c_uint = 0x10; // write clear
pub const ENABLE_TERMINAL_COUNT_EVENT_BIT: ::core::ffi::c_uint = 0x20; // write clear
pub const ENABLE_DMA_TERMINAL_COUNT_EVENT_BIT: ::core::ffi::c_uint = 0x80; // write clear

pub const HALTED_STATUS_BIT: ::core::ffi::c_uint = 0x1; // read
pub const RESTART_STREAM_BIT: ::core::ffi::c_uint = 0x1; // write

pub const DIRECTION_GPIB_TO_HOST_BIT: ::core::ffi::c_uint = 0x20; // transfer direction (set for gpib to host)
pub const ENABLE_TI_BUFFER_BIT: ::core::ffi::c_uint = 0x40; // enable fifo
pub const FAST_WR_EN_BIT: ::core::ffi::c_uint = 0x80; // 350 ns t1 delay?

// registers accessible through isapnp chip on 82341d
pub const PIO_DATA_REG: ::core::ffi::c_uint = 0x20; // read/write pio data lines
pub const PIO_DIRECTION_REG: ::core::ffi::c_uint = 0x21; // set pio data line directions (set for input)

pub const HP_82341D_XILINX_READY_BIT: ::core::ffi::c_uint = 0x1;
pub const HP_82341D_XILINX_DONE_BIT: ::core::ffi::c_uint = 0x2;
// use register layout compatible with C and older versions instead of 32 contiguous ioports
pub const HP_82341D_LEGACY_MODE_BIT: ::core::ffi::c_uint = 0x4;
pub const HP_82341D_NOT_PROG_BIT: ::core::ffi::c_uint = 0x8; // clear to reinitialize xilinx

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
