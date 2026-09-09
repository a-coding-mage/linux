/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TXx9 SoC DMA Controller
 */

// Dependency: declarations from <linux/dmaengine.h> are supplied externally.

pub const TXX9_DMA_MAX_NR_CHANNELS: i32 = 4;

/**
 * struct txx9dmac_platform_data - Controller configuration parameters
 * @memcpy_chan: Channel used for DMA_MEMCPY
 * @have_64bit_regs: DMAC have 64 bit registers
 */
#[repr(C)]
pub struct txx9dmac_platform_data {
    pub memcpy_chan: i32,
    pub have_64bit_regs: bool,
}

/**
 * struct txx9dmac_chan_platform_data - Channel configuration parameters
 * @dmac_dev: A platform device for DMAC
 */
#[repr(C)]
pub struct txx9dmac_chan_platform_data {
    pub dmac_dev: *mut platform_device,
}

/**
 * struct txx9dmac_slave - Controller-specific information about a slave
 * @tx_reg: physical address of data register used for
 *\tmemory-to-peripheral transfers
 * @rx_reg: physical address of data register used for
 *\tperipheral-to-memory transfers
 * @reg_width: peripheral register width
 */
#[repr(C)]
pub struct txx9dmac_slave {
    pub tx_reg: u64,
    pub rx_reg: u64,
    pub reg_width: u32,
}

unsafe extern "C" {
    pub fn txx9_dmac_init(
        id: i32,
        baseaddr: ::core::ffi::c_ulong,
        irq: i32,
        pdata: *const txx9dmac_platform_data,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
