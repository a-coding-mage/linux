/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI DaVinci Audio Serial Port support
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com/
 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
pub struct davinci_mcasp_pdata {
    pub tx_dma_offset: u32,
    pub rx_dma_offset: u32,
    pub asp_chan_q: i32, /* event queue number for ASP channel */
    pub ram_chan_q: i32, /* event queue number for RAM channel */
    /*
     * Allowing this is more efficient and eliminates left and right swaps
     * caused by underruns, but will swap the left and right channels
     * when compared to previous behavior.
     */
    pub enable_channel_combine: u32,
    pub sram_size_playback: u32,
    pub sram_size_capture: u32,
    pub sram_pool: *mut gen_pool,

    /*
     * This flag works when both clock and FS are outputs for the cpu
     * and makes clock more accurate (FS is not symmetrical and the
     * clock is very fast.
     * The clock becoming faster is named
     * i2s continuous serial clock (I2S_SCK) and it is an externally
     * visible bit clock.
     *
     * first line : WordSelect
     * second line : ContinuousSerialClock
     * third line: SerialData
     *
     * SYMMETRICAL APPROACH:
     *   _______________________          LEFT
     * _|         RIGHT         |______________________|
     *     _   _         _   _   _   _         _   _
     *   _| |_| |_ x16 _| |_| |_| |_| |_ x16 _| |_| |_
     *     _   _         _   _   _   _         _   _
     *   _/ \_/ \_ ... _/ \_/ \_/ \_/ \_ ... _/ \_/ \_
     *    \_/ \_/       \_/ \_/ \_/ \_/       \_/ \_/
     *
     * ACCURATE CLOCK APPROACH:
     *   ______________          LEFT
     * _|     RIGHT    |_______________________________|
     *     _         _   _         _   _   _   _   _   _
     *   _| |_ x16 _| |_| |_ x16 _| |_| |_| |_| |_| |_| |
     *     _         _   _          _      dummy cycles
     *   _/ \_ ... _/ \_/ \_  ... _/ \__________________
     *    \_/       \_/ \_/        \_/
     *
     */
    pub i2s_accurate_sck: bool,

    /* McASP specific fields */
    pub tdm_slots_tx: i32,
    pub tdm_slots_rx: i32,
    pub op_mode: u8,
    pub dismod: u8,
    pub num_serializer: u8,
    pub serial_dir: *mut u8,
    pub version: u8,
    pub txnumevt: u8,
    pub rxnumevt: u8,
    pub tx_dma_channel: i32,
    pub rx_dma_channel: i32,
}

/* TODO: Fix arch/arm/mach-davinci/ users and remove this define */
pub type snd_platform_data = davinci_mcasp_pdata;

pub const MCASP_VERSION_1: i32 = 0; /* DM646x */
pub const MCASP_VERSION_2: i32 = 1; /* DA8xx/OMAPL1x */
pub const MCASP_VERSION_3: i32 = 2; /* TI81xx/AM33xx */
pub const MCASP_VERSION_4: i32 = 3; /* DRA7xxx */
pub const MCASP_VERSION_OMAP: i32 = 4; /* OMAP4/5 */

pub const INACTIVE_MODE: i32 = 0;
pub const TX_MODE: i32 = 1;
pub const RX_MODE: i32 = 2;

pub const DAVINCI_MCASP_IIS_MODE: i32 = 0;
pub const DAVINCI_MCASP_DIT_MODE: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
