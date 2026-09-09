/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2021 Sean Anderson <sean.anderson@seco.com>
 */

// The original header includes <linux/compiler.h> for compiler declarations.

pub const TCSR0: u32 = 0x00;
pub const TLR0: u32 = 0x04;
pub const TCR0: u32 = 0x08;
pub const TCSR1: u32 = 0x10;
pub const TLR1: u32 = 0x14;
pub const TCR1: u32 = 0x18;

pub const TCSR_MDT: u32 = 1u32 << 0;
pub const TCSR_UDT: u32 = 1u32 << 1;
pub const TCSR_GENT: u32 = 1u32 << 2;
pub const TCSR_CAPT: u32 = 1u32 << 3;
pub const TCSR_ARHT: u32 = 1u32 << 4;
pub const TCSR_LOAD: u32 = 1u32 << 5;
pub const TCSR_ENIT: u32 = 1u32 << 6;
pub const TCSR_ENT: u32 = 1u32 << 7;
pub const TCSR_TINT: u32 = 1u32 << 8;
pub const TCSR_PWMA: u32 = 1u32 << 9;
pub const TCSR_ENALL: u32 = 1u32 << 10;
pub const TCSR_CASC: u32 = 1u32 << 11;

pub enum clk {}
pub enum device_node {}
pub enum regmap {}

/**
 * struct xilinx_timer_priv - Private data for Xilinx AXI timer drivers
 * @map: Regmap of the device, possibly with an offset
 * @clk: Parent clock
 * @max: Maximum value of the counters
 */
#[repr(C)]
pub struct xilinx_timer_priv {
    pub map: *mut regmap,
    pub clk: *mut clk,
    pub max: u64,
}

/**
 * xilinx_timer_tlr_cycles() - Calculate the TLR for a period specified
 *                             in clock cycles
 * @priv: The timer's private data
 * @tcsr: The value of the TCSR register for this counter
 * @cycles: The number of cycles in this period
 *
 * Callers of this function MUST ensure that @cycles is representable as
 * a TLR.
 *
 * Return: The calculated value for TLR
 */
unsafe extern "C" {
    pub fn xilinx_timer_tlr_cycles(
        priv_: *mut xilinx_timer_priv,
        tcsr: u32,
        cycles: u64,
    ) -> u32;

    /**
     * xilinx_timer_get_period() - Get the current period of a counter
     * @priv: The timer's private data
     * @tlr: The value of TLR for this counter
     * @tcsr: The value of TCSR for this counter
     *
     * Return: The period, in ns
     */
    pub fn xilinx_timer_get_period(
        priv_: *mut xilinx_timer_priv,
        tlr: u32,
        tcsr: u32,
    ) -> core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
