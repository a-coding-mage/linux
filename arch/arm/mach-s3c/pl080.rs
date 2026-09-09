// SPDX-License-Identifier: GPL-2.0
//
// Samsung's S3C64XX generic DMA support using amba-pl08x driver.
//
// Copyright (c) 2013 Tomasz Figa <tomasz.figa@gmail.com>

// Linux dependencies: kernel, AMBA, PL080/PL08X, device tree, CPU, IRQ,
// memory-map, and S3C64XX system-register definitions are supplied externally.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct pl08x_channel_data {
    pub bus_id: *const c_char,
    pub min_signal: c_int,
    pub max_signal: c_int,
    pub periph_buses: c_uint,
}

#[repr(C)]
pub struct dma_slave_map {
    pub slave: *const c_char,
    pub peripheral: *const c_char,
    pub slave_data: *mut pl08x_channel_data,
}

#[repr(C)]
pub struct pl08x_platform_data {
    pub memcpy_burst_size: c_uint,
    pub memcpy_bus_width: c_uint,
    pub memcpy_prot_buff: bool,
    pub memcpy_prot_cache: bool,
    pub lli_buses: c_uint,
    pub mem_buses: c_uint,
    pub get_xfer_signal: Option<unsafe extern "C" fn(*const pl08x_channel_data) -> c_int>,
    pub put_xfer_signal: Option<unsafe extern "C" fn(*const pl08x_channel_data, c_int)>,
    pub slave_channels: *mut pl08x_channel_data,
    pub num_slave_channels: usize,
    pub slave_map: *const dma_slave_map,
    pub slave_map_len: usize,
}

extern "C" {
    fn soc_is_s3c64xx() -> c_int;
    fn of_have_populated_dt() -> c_int;
    fn writel(value: c_uint, address: usize);
    fn amba_device_register(device: *mut c_void, resource: *mut c_void) -> c_int;
    static mut S3C64XX_SDMA_SEL: usize;
    static mut iomem_resource: c_void;
}

const PL08X_AHB1: c_uint = 1;
const PL08X_AHB2: c_uint = 2;
const PL08X_BURST_SZ_4: c_uint = 4;
const PL08X_BUS_WIDTH_32_BITS: c_uint = 32;
const IRQ_DMA0: c_uint = 0;
const IRQ_DMA1: c_uint = 1;

unsafe extern "C" fn pl08x_get_xfer_signal(cd: *const pl08x_channel_data) -> c_int {
    (*cd).min_signal
}

unsafe extern "C" fn pl08x_put_xfer_signal(_cd: *const pl08x_channel_data, _ch: c_int) {}

static mut s3c64xx_dma0_info: [pl08x_channel_data; 16] = [
    pl08x_channel_data { bus_id: b"uart0_tx\0".as_ptr() as *const c_char, min_signal: 0, max_signal: 0, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"uart0_rx\0".as_ptr() as *const c_char, min_signal: 1, max_signal: 1, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"uart1_tx\0".as_ptr() as *const c_char, min_signal: 2, max_signal: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"uart1_rx\0".as_ptr() as *const c_char, min_signal: 3, max_signal: 3, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"uart2_tx\0".as_ptr() as *const c_char, min_signal: 4, max_signal: 4, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"uart2_rx\0".as_ptr() as *const c_char, min_signal: 5, max_signal: 5, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"uart3_tx\0".as_ptr() as *const c_char, min_signal: 6, max_signal: 6, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"uart3_rx\0".as_ptr() as *const c_char, min_signal: 7, max_signal: 7, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"pcm0_tx\0".as_ptr() as *const c_char, min_signal: 8, max_signal: 8, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"pcm0_rx\0".as_ptr() as *const c_char, min_signal: 9, max_signal: 9, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"i2s0_tx\0".as_ptr() as *const c_char, min_signal: 10, max_signal: 10, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"i2s0_rx\0".as_ptr() as *const c_char, min_signal: 11, max_signal: 11, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"spi0_tx\0".as_ptr() as *const c_char, min_signal: 12, max_signal: 12, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"spi0_rx\0".as_ptr() as *const c_char, min_signal: 13, max_signal: 13, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"i2s2_tx\0".as_ptr() as *const c_char, min_signal: 14, max_signal: 14, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"i2s2_rx\0".as_ptr() as *const c_char, min_signal: 15, max_signal: 15, periph_buses: PL08X_AHB2 },
];

static mut s3c64xx_dma1_info: [pl08x_channel_data; 12] = [
    pl08x_channel_data { bus_id: b"pcm1_tx\0".as_ptr() as *const c_char, min_signal: 0, max_signal: 0, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"pcm1_rx\0".as_ptr() as *const c_char, min_signal: 1, max_signal: 1, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"i2s1_tx\0".as_ptr() as *const c_char, min_signal: 2, max_signal: 2, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"i2s1_rx\0".as_ptr() as *const c_char, min_signal: 3, max_signal: 3, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"spi1_tx\0".as_ptr() as *const c_char, min_signal: 4, max_signal: 4, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"spi1_rx\0".as_ptr() as *const c_char, min_signal: 5, max_signal: 5, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"ac97_out\0".as_ptr() as *const c_char, min_signal: 6, max_signal: 6, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"ac97_in\0".as_ptr() as *const c_char, min_signal: 7, max_signal: 7, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"ac97_mic\0".as_ptr() as *const c_char, min_signal: 8, max_signal: 8, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"pwm\0".as_ptr() as *const c_char, min_signal: 9, max_signal: 9, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"irda\0".as_ptr() as *const c_char, min_signal: 10, max_signal: 10, periph_buses: PL08X_AHB2 },
    pl08x_channel_data { bus_id: b"external\0".as_ptr() as *const c_char, min_signal: 11, max_signal: 11, periph_buses: PL08X_AHB2 },
];

static mut s3c64xx_dma0_slave_map: [dma_slave_map; 16] = [
    dma_slave_map { slave: b"s3c6400-uart.0\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[0] } },
    dma_slave_map { slave: b"s3c6400-uart.0\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[1] } },
    dma_slave_map { slave: b"s3c6400-uart.1\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[2] } },
    dma_slave_map { slave: b"s3c6400-uart.1\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[3] } },
    dma_slave_map { slave: b"s3c6400-uart.2\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[4] } },
    dma_slave_map { slave: b"s3c6400-uart.2\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[5] } },
    dma_slave_map { slave: b"s3c6400-uart.3\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[6] } },
    dma_slave_map { slave: b"s3c6400-uart.3\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[7] } },
    dma_slave_map { slave: b"samsung-pcm.0\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[8] } },
    dma_slave_map { slave: b"samsung-pcm.0\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[9] } },
    dma_slave_map { slave: b"samsung-i2s.0\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[10] } },
    dma_slave_map { slave: b"samsung-i2s.0\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[11] } },
    dma_slave_map { slave: b"s3c6410-spi.0\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[12] } },
    dma_slave_map { slave: b"s3c6410-spi.0\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[13] } },
    dma_slave_map { slave: b"samsung-i2s.2\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[14] } },
    dma_slave_map { slave: b"samsung-i2s.2\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma0_info[15] } },
];

static mut s3c64xx_dma1_slave_map: [dma_slave_map; 6] = [
    dma_slave_map { slave: b"samsung-pcm.1\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma1_info[0] } },
    dma_slave_map { slave: b"samsung-pcm.1\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma1_info[1] } },
    dma_slave_map { slave: b"samsung-i2s.1\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma1_info[2] } },
    dma_slave_map { slave: b"samsung-i2s.1\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma1_info[3] } },
    dma_slave_map { slave: b"s3c6410-spi.1\0".as_ptr() as *const c_char, peripheral: b"tx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma1_info[4] } },
    dma_slave_map { slave: b"s3c6410-spi.1\0".as_ptr() as *const c_char, peripheral: b"rx\0".as_ptr() as *const c_char, slave_data: unsafe { &mut s3c64xx_dma1_info[5] } },
];

pub static mut s3c64xx_dma0_plat_data: pl08x_platform_data = pl08x_platform_data {
    memcpy_burst_size: PL08X_BURST_SZ_4, memcpy_bus_width: PL08X_BUS_WIDTH_32_BITS,
    memcpy_prot_buff: true, memcpy_prot_cache: true, lli_buses: PL08X_AHB1, mem_buses: PL08X_AHB1,
    get_xfer_signal: Some(pl08x_get_xfer_signal), put_xfer_signal: Some(pl08x_put_xfer_signal),
    slave_channels: unsafe { s3c64xx_dma0_info.as_mut_ptr() }, num_slave_channels: 16,
    slave_map: unsafe { s3c64xx_dma0_slave_map.as_ptr() }, slave_map_len: 16,
};

pub static mut s3c64xx_dma1_plat_data: pl08x_platform_data = pl08x_platform_data {
    memcpy_burst_size: PL08X_BURST_SZ_4, memcpy_bus_width: PL08X_BUS_WIDTH_32_BITS,
    memcpy_prot_buff: true, memcpy_prot_cache: true, lli_buses: PL08X_AHB1, mem_buses: PL08X_AHB1,
    get_xfer_signal: Some(pl08x_get_xfer_signal), put_xfer_signal: Some(pl08x_put_xfer_signal),
    slave_channels: unsafe { s3c64xx_dma1_info.as_mut_ptr() }, num_slave_channels: 12,
    slave_map: unsafe { s3c64xx_dma1_slave_map.as_ptr() }, slave_map_len: 6,
};

unsafe extern "C" fn s3c64xx_pl080_init() -> c_int {
    if soc_is_s3c64xx() == 0 { return 0; }
    // Set all DMA configuration to be DMA, not SDMA.
    writel(0xffffff, S3C64XX_SDMA_SEL);
    if of_have_populated_dt() != 0 { return 0; }
    // AMBA_AHB_DEVICE declarations register these two external devices.
    amba_device_register(core::ptr::null_mut(), &mut iomem_resource);
    amba_device_register(core::ptr::null_mut(), &mut iomem_resource);
    0
}

// arch_initcall(s3c64xx_pl080_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
