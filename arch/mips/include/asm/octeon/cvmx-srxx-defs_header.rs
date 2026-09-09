/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2012 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

/* The original header depends on CVMX_ADD_IO_SEG from the surrounding SDK. */

#[macro_export]
macro_rules! CVMX_SRXX_COM_CTL {
    ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180090000200u64) + (($block_id) & 1) * 0x8000000u64 };
}
#[macro_export]
macro_rules! CVMX_SRXX_IGN_RX_FULL {
    ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180090000218u64) + (($block_id) & 1) * 0x8000000u64 };
}
#[macro_export]
macro_rules! CVMX_SRXX_SPI4_CALX {
    ($offset:expr, $block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180090000000u64) + (((($offset) & 31) + (($block_id) & 1) * 0x1000000u64) * 8) };
}
#[macro_export]
macro_rules! CVMX_SRXX_SPI4_STAT {
    ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180090000208u64) + (($block_id) & 1) * 0x8000000u64 };
}
#[macro_export]
macro_rules! CVMX_SRXX_SW_TICK_CTL {
    ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180090000220u64) + (($block_id) & 1) * 0x8000000u64 };
}
#[macro_export]
macro_rules! CVMX_SRXX_SW_TICK_DAT {
    ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180090000228u64) + (($block_id) & 1) * 0x8000000u64 };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_srxx_com_ctl_s { pub bits: u64 }
#[repr(C)]
pub union cvmx_srxx_com_ctl { pub u64_: u64, pub s: cvmx_srxx_com_ctl_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_srxx_ign_rx_full_s { pub bits: u64 }
#[repr(C)]
pub union cvmx_srxx_ign_rx_full { pub u64_: u64, pub s: cvmx_srxx_ign_rx_full_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_srxx_spi4_calx_s { pub bits: u64 }
#[repr(C)]
pub union cvmx_srxx_spi4_calx { pub u64_: u64, pub s: cvmx_srxx_spi4_calx_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_srxx_spi4_stat_s { pub bits: u64 }
#[repr(C)]
pub union cvmx_srxx_spi4_stat { pub u64_: u64, pub s: cvmx_srxx_spi4_stat_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_srxx_sw_tick_ctl_s { pub bits: u64 }
#[repr(C)]
pub union cvmx_srxx_sw_tick_ctl { pub u64_: u64, pub s: cvmx_srxx_sw_tick_ctl_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_srxx_sw_tick_dat_s { pub bits: u64 }
#[repr(C)]
pub union cvmx_srxx_sw_tick_dat { pub u64_: u64, pub s: cvmx_srxx_sw_tick_dat_s }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
