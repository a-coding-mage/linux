// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra30_ahub.rs - Tegra30 AHUB driver
 *
 * Copyright (c) 2011,2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Translated from C implementation source. Kernel includes and module
// registration macros are represented by external declarations or comments.

const DRV_NAME: *const core::ffi::c_char = b"tegra30-ahub\0".as_ptr() as *const core::ffi::c_char;

type u32 = u32;
type dma_addr_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control_bulk_data {
    pub id: *const core::ffi::c_char,
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const core::ffi::c_char,
}

#[repr(C)]
pub struct tegra30_ahub_cif_conf {
    pub threshold: u32,
    pub audio_channels: u32,
    pub client_channels: u32,
    pub audio_bits: u32,
    pub client_bits: u32,
    pub expand: u32,
    pub stereo_conv: u32,
    pub replicate: u32,
    pub direction: u32,
    pub truncate: u32,
    pub mono_conv: u32,
}

#[repr(C)]
pub struct tegra30_ahub_soc_data {
    pub num_resets: core::ffi::c_int,
    pub set_audio_cif: Option<
        unsafe extern "C" fn(*mut regmap, core::ffi::c_uint, *mut tegra30_ahub_cif_conf),
    >,
}

#[repr(C)]
pub struct tegra30_ahub {
    pub regmap_apbif: *mut regmap,
    pub regmap_ahub: *mut regmap,
    pub nclocks: core::ffi::c_int,
    pub clocks: [clk_bulk_data; TEGRA30_AHUB_CLOCK_COUNT],
    pub nresets: core::ffi::c_int,
    pub resets: [reset_control_bulk_data; TEGRA30_AHUB_RESET_COUNT],
    pub rx_usage: *mut core::ffi::c_ulong,
    pub tx_usage: *mut core::ffi::c_ulong,
    pub apbif_addr: dma_addr_t,
    pub soc_data: *const tegra30_ahub_soc_data,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const core::ffi::c_char,
    pub reg_bits: core::ffi::c_uint,
    pub val_bits: core::ffi::c_uint,
    pub reg_stride: core::ffi::c_uint,
    pub max_register: core::ffi::c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    pub cache_type: core::ffi::c_int,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_driver,
}

type tegra30_ahub_rxcif = core::ffi::c_int;
type tegra30_ahub_txcif = core::ffi::c_int;

extern "C" {
    static mut TEGRA30_AHUB_CHANNEL_CTRL_COUNT: core::ffi::c_int;
    static mut TEGRA30_AHUB_RXCIF_APBIF_RX0: tegra30_ahub_rxcif;
    static mut TEGRA30_AHUB_TXCIF_APBIF_TX0: tegra30_ahub_txcif;
    static mut TEGRA30_AHUB_CHANNEL_RXFIFO: u32;
    static mut TEGRA30_AHUB_CHANNEL_RXFIFO_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_TXFIFO: u32;
    static mut TEGRA30_AHUB_CHANNEL_TXFIFO_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_CTRL: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_RX_THRESHOLD_MASK: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_MASK: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_RX_THRESHOLD_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_EN: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_16: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_TX_THRESHOLD_MASK: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_MASK: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_TX_THRESHOLD_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_EN: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_16: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_RX_EN: u32;
    static mut TEGRA30_AHUB_CHANNEL_CTRL_TX_EN: u32;
    static mut TEGRA30_AHUB_CIF_RX_CTRL: u32;
    static mut TEGRA30_AHUB_CIF_RX_CTRL_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_CIF_TX_CTRL: u32;
    static mut TEGRA30_AHUB_CIF_TX_CTRL_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_AUDIO_RX: u32;
    static mut TEGRA30_AHUB_AUDIO_RX_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_BITS_16: u32;
    static mut TEGRA30_AUDIOCIF_DIRECTION_RX: u32;
    static mut TEGRA30_AUDIOCIF_DIRECTION_TX: u32;
    static mut TEGRA30_AHUB_CONFIG_LINK_CTRL: u32;
    static mut TEGRA30_AHUB_MISC_CTRL: u32;
    static mut TEGRA30_AHUB_APBDMA_LIVE_STATUS: u32;
    static mut TEGRA30_AHUB_I2S_LIVE_STATUS: u32;
    static mut TEGRA30_AHUB_SPDIF_LIVE_STATUS: u32;
    static mut TEGRA30_AHUB_I2S_INT_MASK: u32;
    static mut TEGRA30_AHUB_DAM_INT_MASK: u32;
    static mut TEGRA30_AHUB_SPDIF_INT_MASK: u32;
    static mut TEGRA30_AHUB_APBIF_INT_MASK: u32;
    static mut TEGRA30_AHUB_I2S_INT_STATUS: u32;
    static mut TEGRA30_AHUB_DAM_INT_STATUS: u32;
    static mut TEGRA30_AHUB_SPDIF_INT_STATUS: u32;
    static mut TEGRA30_AHUB_APBIF_INT_STATUS: u32;
    static mut TEGRA30_AHUB_I2S_INT_SOURCE: u32;
    static mut TEGRA30_AHUB_DAM_INT_SOURCE: u32;
    static mut TEGRA30_AHUB_SPDIF_INT_SOURCE: u32;
    static mut TEGRA30_AHUB_APBIF_INT_SOURCE: u32;
    static mut TEGRA30_AHUB_I2S_INT_SET: u32;
    static mut TEGRA30_AHUB_DAM_INT_SET: u32;
    static mut TEGRA30_AHUB_SPDIF_INT_SET: u32;
    static mut TEGRA30_AHUB_APBIF_INT_SET: u32;
    static mut TEGRA30_AHUB_CHANNEL_CLEAR: u32;
    static mut TEGRA30_AHUB_CHANNEL_CLEAR_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_CLEAR_COUNT: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_STATUS: u32;
    static mut TEGRA30_AHUB_CHANNEL_STATUS_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_CHANNEL_STATUS_COUNT: core::ffi::c_int;
    static mut TEGRA30_AHUB_DAM_LIVE_STATUS: u32;
    static mut TEGRA30_AHUB_DAM_LIVE_STATUS_STRIDE: core::ffi::c_int;
    static mut TEGRA30_AHUB_DAM_LIVE_STATUS_COUNT: core::ffi::c_int;
    static mut TEGRA30_AHUB_AUDIO_RX_COUNT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_FIFO_THRESHOLD_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_AUDIO_CHANNELS_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_CLIENT_CHANNELS_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_AUDIO_BITS_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_CLIENT_BITS_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_EXPAND_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_STEREO_CONV_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_REPLICATE_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_DIRECTION_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_TRUNCATE_SHIFT: core::ffi::c_int;
    static mut TEGRA30_AUDIOCIF_CTRL_MONO_CONV_SHIFT: core::ffi::c_int;
    static mut TEGRA124_AUDIOCIF_CTRL_FIFO_THRESHOLD_SHIFT: core::ffi::c_int;
    static mut TEGRA124_AUDIOCIF_CTRL_AUDIO_CHANNELS_SHIFT: core::ffi::c_int;
    static mut TEGRA124_AUDIOCIF_CTRL_CLIENT_CHANNELS_SHIFT: core::ffi::c_int;

    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> core::ffi::c_int;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> core::ffi::c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> core::ffi::c_int;
    fn clk_bulk_disable_unprepare(num_clks: core::ffi::c_int, clks: *mut clk_bulk_data);
    fn clk_bulk_prepare_enable(
        num_clks: core::ffi::c_int,
        clks: *mut clk_bulk_data,
    ) -> core::ffi::c_int;
    fn reset_control_bulk_assert(
        num_rstcs: core::ffi::c_int,
        rstcs: *mut reset_control_bulk_data,
    ) -> core::ffi::c_int;
    fn reset_control_bulk_deassert(
        num_rstcs: core::ffi::c_int,
        rstcs: *mut reset_control_bulk_data,
    ) -> core::ffi::c_int;
    fn usleep_range(min: core::ffi::c_ulong, max: core::ffi::c_ulong);
    fn find_first_zero_bit(
        addr: *mut core::ffi::c_ulong,
        size: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn __set_bit(nr: core::ffi::c_int, addr: *mut core::ffi::c_ulong);
    fn __clear_bit(nr: core::ffi::c_int, addr: *mut core::ffi::c_ulong);
    fn snprintf(
        buf: *mut core::ffi::c_char,
        size: core::ffi::c_int,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> core::ffi::c_int;
    fn pm_runtime_put(dev: *mut device) -> core::ffi::c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const tegra30_ahub_soc_data;
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    fn devm_clk_bulk_get(
        dev: *mut device,
        num_clks: core::ffi::c_int,
        clks: *mut clk_bulk_data,
    ) -> core::ffi::c_int;
    fn devm_reset_control_bulk_get_exclusive(
        dev: *mut device,
        num_rstcs: core::ffi::c_int,
        rstcs: *mut reset_control_bulk_data,
    ) -> core::ffi::c_int;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: core::ffi::c_uint,
        res: *mut *mut resource,
    ) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> core::ffi::c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn of_platform_populate(
        root: *mut core::ffi::c_void,
        matches: *const core::ffi::c_void,
        lookup: *const core::ffi::c_void,
        parent: *mut device,
    ) -> core::ffi::c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> core::ffi::c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> core::ffi::c_int;
}

const EBUSY: core::ffi::c_int = 16;
const EINVAL: core::ffi::c_int = 22;
const ENOMEM: core::ffi::c_int = 12;
const GFP_KERNEL: core::ffi::c_int = 0;
const REGCACHE_FLAT: core::ffi::c_int = 1;
const TEGRA30_AHUB_CLOCK_COUNT: usize = 2;
const TEGRA30_AHUB_RESET_COUNT: usize = 21;

static mut ahub: *mut tegra30_ahub = core::ptr::null_mut();

#[inline]
unsafe fn tegra30_apbif_write(reg: u32, val: u32) {
    regmap_write((*ahub).regmap_apbif, reg, val);
}

#[inline]
unsafe fn tegra30_apbif_read(reg: u32) -> u32 {
    let mut val: u32 = 0;

    regmap_read((*ahub).regmap_apbif, reg, &mut val);
    val
}

#[inline]
unsafe fn tegra30_audio_write(reg: u32, val: u32) {
    regmap_write((*ahub).regmap_ahub, reg, val);
}

unsafe extern "C" fn tegra30_ahub_runtime_suspend(_dev: *mut device) -> core::ffi::c_int {
    regcache_cache_only((*ahub).regmap_apbif, true);
    regcache_cache_only((*ahub).regmap_ahub, true);

    clk_bulk_disable_unprepare((*ahub).nclocks, (*ahub).clocks.as_mut_ptr());

    0
}

/*
 * clk_apbif isn't required for an I2S<->I2S configuration where no PCM data
 * is read from or sent to memory. However, that's not something the rest of
 * the driver supports right now, so we'll just treat the two clocks as one
 * for now.
 *
 * These functions should not be a plain ref-count. Instead, each active stream
 * contributes some requirement to the minimum clock rate, so starting or
 * stopping streams should dynamically adjust the clock as required.  However,
 * this is not yet implemented.
 */
unsafe extern "C" fn tegra30_ahub_runtime_resume(_dev: *mut device) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;

    ret = reset_control_bulk_assert((*ahub).nresets, (*ahub).resets.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    ret = clk_bulk_prepare_enable((*ahub).nclocks, (*ahub).clocks.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    usleep_range(10, 100);

    ret = reset_control_bulk_deassert((*ahub).nresets, (*ahub).resets.as_mut_ptr());
    if ret != 0 {
        clk_bulk_disable_unprepare((*ahub).nclocks, (*ahub).clocks.as_mut_ptr());
        return ret;
    }

    regcache_cache_only((*ahub).regmap_apbif, false);
    regcache_cache_only((*ahub).regmap_ahub, false);
    regcache_mark_dirty((*ahub).regmap_apbif);
    regcache_mark_dirty((*ahub).regmap_ahub);

    ret = regcache_sync((*ahub).regmap_apbif);
    if ret != 0 {
        clk_bulk_disable_unprepare((*ahub).nclocks, (*ahub).clocks.as_mut_ptr());
        return ret;
    }

    ret = regcache_sync((*ahub).regmap_ahub);
    if ret != 0 {
        clk_bulk_disable_unprepare((*ahub).nclocks, (*ahub).clocks.as_mut_ptr());
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_allocate_rx_fifo(
    rxcif: *mut tegra30_ahub_rxcif,
    dmachan: *mut core::ffi::c_char,
    dmachan_len: core::ffi::c_int,
    fiforeg: *mut dma_addr_t,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int;
    let mut reg: u32;
    let mut val: u32;
    let mut cif_conf: tegra30_ahub_cif_conf;

    channel = find_first_zero_bit((*ahub).rx_usage, TEGRA30_AHUB_CHANNEL_CTRL_COUNT);
    if channel >= TEGRA30_AHUB_CHANNEL_CTRL_COUNT {
        return -EBUSY;
    }

    __set_bit(channel, (*ahub).rx_usage);

    *rxcif = TEGRA30_AHUB_RXCIF_APBIF_RX0 + channel;
    snprintf(dmachan, dmachan_len, b"rx%d\0".as_ptr() as *const core::ffi::c_char, channel);
    *fiforeg = (*ahub).apbif_addr
        + TEGRA30_AHUB_CHANNEL_RXFIFO as usize
        + (channel * TEGRA30_AHUB_CHANNEL_RXFIFO_STRIDE) as usize;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_CHANNEL_CTRL
        + (channel * TEGRA30_AHUB_CHANNEL_CTRL_STRIDE) as u32;
    val = tegra30_apbif_read(reg);
    val &= !(TEGRA30_AHUB_CHANNEL_CTRL_RX_THRESHOLD_MASK | TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_MASK);
    val |= (7u32 << TEGRA30_AHUB_CHANNEL_CTRL_RX_THRESHOLD_SHIFT)
        | TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_EN
        | TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_16;
    tegra30_apbif_write(reg, val);

    cif_conf = tegra30_ahub_cif_conf {
        threshold: 0,
        audio_channels: 2,
        client_channels: 2,
        audio_bits: TEGRA30_AUDIOCIF_BITS_16,
        client_bits: TEGRA30_AUDIOCIF_BITS_16,
        expand: 0,
        stereo_conv: 0,
        replicate: 0,
        direction: TEGRA30_AUDIOCIF_DIRECTION_RX,
        truncate: 0,
        mono_conv: 0,
    };

    reg = TEGRA30_AHUB_CIF_RX_CTRL + (channel * TEGRA30_AHUB_CIF_RX_CTRL_STRIDE) as u32;
    ((*(*ahub).soc_data).set_audio_cif.unwrap())((*ahub).regmap_apbif, reg, &mut cif_conf);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_allocate_rx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_enable_rx_fifo(
    rxcif: tegra30_ahub_rxcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = rxcif - TEGRA30_AHUB_RXCIF_APBIF_RX0;
    let reg: core::ffi::c_int;
    let mut val: core::ffi::c_int;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_CHANNEL_CTRL as core::ffi::c_int
        + (channel * TEGRA30_AHUB_CHANNEL_CTRL_STRIDE);
    val = tegra30_apbif_read(reg as u32) as core::ffi::c_int;
    val |= TEGRA30_AHUB_CHANNEL_CTRL_RX_EN as core::ffi::c_int;
    tegra30_apbif_write(reg as u32, val as u32);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_enable_rx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_disable_rx_fifo(
    rxcif: tegra30_ahub_rxcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = rxcif - TEGRA30_AHUB_RXCIF_APBIF_RX0;
    let reg: core::ffi::c_int;
    let mut val: core::ffi::c_int;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_CHANNEL_CTRL as core::ffi::c_int
        + (channel * TEGRA30_AHUB_CHANNEL_CTRL_STRIDE);
    val = tegra30_apbif_read(reg as u32) as core::ffi::c_int;
    val &= !(TEGRA30_AHUB_CHANNEL_CTRL_RX_EN as core::ffi::c_int);
    tegra30_apbif_write(reg as u32, val as u32);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_disable_rx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_free_rx_fifo(
    rxcif: tegra30_ahub_rxcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = rxcif - TEGRA30_AHUB_RXCIF_APBIF_RX0;

    __clear_bit(channel, (*ahub).rx_usage);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_free_rx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_allocate_tx_fifo(
    txcif: *mut tegra30_ahub_txcif,
    dmachan: *mut core::ffi::c_char,
    dmachan_len: core::ffi::c_int,
    fiforeg: *mut dma_addr_t,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int;
    let mut reg: u32;
    let mut val: u32;
    let mut cif_conf: tegra30_ahub_cif_conf;

    channel = find_first_zero_bit((*ahub).tx_usage, TEGRA30_AHUB_CHANNEL_CTRL_COUNT);
    if channel >= TEGRA30_AHUB_CHANNEL_CTRL_COUNT {
        return -EBUSY;
    }

    __set_bit(channel, (*ahub).tx_usage);

    *txcif = TEGRA30_AHUB_TXCIF_APBIF_TX0 + channel;
    snprintf(dmachan, dmachan_len, b"tx%d\0".as_ptr() as *const core::ffi::c_char, channel);
    *fiforeg = (*ahub).apbif_addr
        + TEGRA30_AHUB_CHANNEL_TXFIFO as usize
        + (channel * TEGRA30_AHUB_CHANNEL_TXFIFO_STRIDE) as usize;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_CHANNEL_CTRL
        + (channel * TEGRA30_AHUB_CHANNEL_CTRL_STRIDE) as u32;
    val = tegra30_apbif_read(reg);
    val &= !(TEGRA30_AHUB_CHANNEL_CTRL_TX_THRESHOLD_MASK | TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_MASK);
    val |= (7u32 << TEGRA30_AHUB_CHANNEL_CTRL_TX_THRESHOLD_SHIFT)
        | TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_EN
        | TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_16;
    tegra30_apbif_write(reg, val);

    cif_conf = tegra30_ahub_cif_conf {
        threshold: 0,
        audio_channels: 2,
        client_channels: 2,
        audio_bits: TEGRA30_AUDIOCIF_BITS_16,
        client_bits: TEGRA30_AUDIOCIF_BITS_16,
        expand: 0,
        stereo_conv: 0,
        replicate: 0,
        direction: TEGRA30_AUDIOCIF_DIRECTION_TX,
        truncate: 0,
        mono_conv: 0,
    };

    reg = TEGRA30_AHUB_CIF_TX_CTRL + (channel * TEGRA30_AHUB_CIF_TX_CTRL_STRIDE) as u32;
    ((*(*ahub).soc_data).set_audio_cif.unwrap())((*ahub).regmap_apbif, reg, &mut cif_conf);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_allocate_tx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_enable_tx_fifo(
    txcif: tegra30_ahub_txcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = txcif - TEGRA30_AHUB_TXCIF_APBIF_TX0;
    let reg: core::ffi::c_int;
    let mut val: core::ffi::c_int;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_CHANNEL_CTRL as core::ffi::c_int
        + (channel * TEGRA30_AHUB_CHANNEL_CTRL_STRIDE);
    val = tegra30_apbif_read(reg as u32) as core::ffi::c_int;
    val |= TEGRA30_AHUB_CHANNEL_CTRL_TX_EN as core::ffi::c_int;
    tegra30_apbif_write(reg as u32, val as u32);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_enable_tx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_disable_tx_fifo(
    txcif: tegra30_ahub_txcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = txcif - TEGRA30_AHUB_TXCIF_APBIF_TX0;
    let reg: core::ffi::c_int;
    let mut val: core::ffi::c_int;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_CHANNEL_CTRL as core::ffi::c_int
        + (channel * TEGRA30_AHUB_CHANNEL_CTRL_STRIDE);
    val = tegra30_apbif_read(reg as u32) as core::ffi::c_int;
    val &= !(TEGRA30_AHUB_CHANNEL_CTRL_TX_EN as core::ffi::c_int);
    tegra30_apbif_write(reg as u32, val as u32);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_disable_tx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_free_tx_fifo(
    txcif: tegra30_ahub_txcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = txcif - TEGRA30_AHUB_TXCIF_APBIF_TX0;

    __clear_bit(channel, (*ahub).tx_usage);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_free_tx_fifo);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_set_rx_cif_source(
    rxcif: tegra30_ahub_rxcif,
    txcif: tegra30_ahub_txcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = rxcif - TEGRA30_AHUB_RXCIF_APBIF_RX0;
    let reg: core::ffi::c_int;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_AUDIO_RX as core::ffi::c_int
        + (channel * TEGRA30_AHUB_AUDIO_RX_STRIDE);
    tegra30_audio_write(reg as u32, 1u32 << txcif);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_set_rx_cif_source);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_unset_rx_cif_source(
    rxcif: tegra30_ahub_rxcif,
) -> core::ffi::c_int {
    let channel: core::ffi::c_int = rxcif - TEGRA30_AHUB_RXCIF_APBIF_RX0;
    let reg: core::ffi::c_int;

    pm_runtime_get_sync((*ahub).dev);

    reg = TEGRA30_AHUB_AUDIO_RX as core::ffi::c_int
        + (channel * TEGRA30_AHUB_AUDIO_RX_STRIDE);
    tegra30_audio_write(reg as u32, 0);

    pm_runtime_put((*ahub).dev);

    0
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_unset_rx_cif_source);

static mut tegra30_ahub_resets_data: [reset_control_bulk_data; TEGRA30_AHUB_RESET_COUNT] = [
    reset_control_bulk_data { id: b"d_audio\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"apbif\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"i2s0\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"i2s1\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"i2s2\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"i2s3\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"i2s4\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"dam0\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"dam1\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"dam2\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"spdif\0".as_ptr() as *const core::ffi::c_char },
    reset_control_bulk_data { id: b"amx\0".as_ptr() as *const core::ffi::c_char }, /* Tegra114+ */
    reset_control_bulk_data { id: b"adx\0".as_ptr() as *const core::ffi::c_char }, /* Tegra114+ */
    reset_control_bulk_data { id: b"amx1\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
    reset_control_bulk_data { id: b"adx1\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
    reset_control_bulk_data { id: b"afc0\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
    reset_control_bulk_data { id: b"afc1\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
    reset_control_bulk_data { id: b"afc2\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
    reset_control_bulk_data { id: b"afc3\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
    reset_control_bulk_data { id: b"afc4\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
    reset_control_bulk_data { id: b"afc5\0".as_ptr() as *const core::ffi::c_char }, /* Tegra124 */
];

unsafe fn LAST_REG(base: u32, stride: core::ffi::c_int, count: core::ffi::c_int) -> u32 {
    base + (stride * count) as u32 - 4
}

unsafe fn REG_IN_ARRAY(
    reg: core::ffi::c_uint,
    base: u32,
    stride: core::ffi::c_int,
    count: core::ffi::c_int,
) -> bool {
    reg >= base
        && reg <= LAST_REG(base, stride, count)
        && ((reg - base) % stride as u32) == 0
}

unsafe extern "C" fn tegra30_ahub_apbif_wr_rd_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    if reg == TEGRA30_AHUB_CONFIG_LINK_CTRL
        || reg == TEGRA30_AHUB_MISC_CTRL
        || reg == TEGRA30_AHUB_APBDMA_LIVE_STATUS
        || reg == TEGRA30_AHUB_I2S_LIVE_STATUS
        || reg == TEGRA30_AHUB_SPDIF_LIVE_STATUS
        || reg == TEGRA30_AHUB_I2S_INT_MASK
        || reg == TEGRA30_AHUB_DAM_INT_MASK
        || reg == TEGRA30_AHUB_SPDIF_INT_MASK
        || reg == TEGRA30_AHUB_APBIF_INT_MASK
        || reg == TEGRA30_AHUB_I2S_INT_STATUS
        || reg == TEGRA30_AHUB_DAM_INT_STATUS
        || reg == TEGRA30_AHUB_SPDIF_INT_STATUS
        || reg == TEGRA30_AHUB_APBIF_INT_STATUS
        || reg == TEGRA30_AHUB_I2S_INT_SOURCE
        || reg == TEGRA30_AHUB_DAM_INT_SOURCE
        || reg == TEGRA30_AHUB_SPDIF_INT_SOURCE
        || reg == TEGRA30_AHUB_APBIF_INT_SOURCE
        || reg == TEGRA30_AHUB_I2S_INT_SET
        || reg == TEGRA30_AHUB_DAM_INT_SET
        || reg == TEGRA30_AHUB_SPDIF_INT_SET
        || reg == TEGRA30_AHUB_APBIF_INT_SET
    {
        return true;
    }

    if REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_CTRL, TEGRA30_AHUB_CHANNEL_CTRL_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_CLEAR, TEGRA30_AHUB_CHANNEL_CLEAR_STRIDE, TEGRA30_AHUB_CHANNEL_CLEAR_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_STATUS, TEGRA30_AHUB_CHANNEL_STATUS_STRIDE, TEGRA30_AHUB_CHANNEL_STATUS_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_TXFIFO, TEGRA30_AHUB_CHANNEL_TXFIFO_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_RXFIFO, TEGRA30_AHUB_CHANNEL_RXFIFO_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CIF_TX_CTRL, TEGRA30_AHUB_CIF_TX_CTRL_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CIF_RX_CTRL, TEGRA30_AHUB_CIF_RX_CTRL_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_DAM_LIVE_STATUS, TEGRA30_AHUB_DAM_LIVE_STATUS_STRIDE, TEGRA30_AHUB_DAM_LIVE_STATUS_COUNT)
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra30_ahub_apbif_volatile_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    if reg == TEGRA30_AHUB_CONFIG_LINK_CTRL
        || reg == TEGRA30_AHUB_MISC_CTRL
        || reg == TEGRA30_AHUB_APBDMA_LIVE_STATUS
        || reg == TEGRA30_AHUB_I2S_LIVE_STATUS
        || reg == TEGRA30_AHUB_SPDIF_LIVE_STATUS
        || reg == TEGRA30_AHUB_I2S_INT_STATUS
        || reg == TEGRA30_AHUB_DAM_INT_STATUS
        || reg == TEGRA30_AHUB_SPDIF_INT_STATUS
        || reg == TEGRA30_AHUB_APBIF_INT_STATUS
        || reg == TEGRA30_AHUB_I2S_INT_SET
        || reg == TEGRA30_AHUB_DAM_INT_SET
        || reg == TEGRA30_AHUB_SPDIF_INT_SET
        || reg == TEGRA30_AHUB_APBIF_INT_SET
    {
        return true;
    }

    if REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_CLEAR, TEGRA30_AHUB_CHANNEL_CLEAR_STRIDE, TEGRA30_AHUB_CHANNEL_CLEAR_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_STATUS, TEGRA30_AHUB_CHANNEL_STATUS_STRIDE, TEGRA30_AHUB_CHANNEL_STATUS_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_TXFIFO, TEGRA30_AHUB_CHANNEL_TXFIFO_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_RXFIFO, TEGRA30_AHUB_CHANNEL_RXFIFO_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_DAM_LIVE_STATUS, TEGRA30_AHUB_DAM_LIVE_STATUS_STRIDE, TEGRA30_AHUB_DAM_LIVE_STATUS_COUNT)
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra30_ahub_apbif_precious_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    if REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_TXFIFO, TEGRA30_AHUB_CHANNEL_TXFIFO_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
        || REG_IN_ARRAY(reg, TEGRA30_AHUB_CHANNEL_RXFIFO, TEGRA30_AHUB_CHANNEL_RXFIFO_STRIDE, TEGRA30_AHUB_CHANNEL_CTRL_COUNT)
    {
        return true;
    }

    false
}

static mut tegra30_ahub_apbif_regmap_config: regmap_config = regmap_config {
    name: b"apbif\0".as_ptr() as *const core::ffi::c_char,
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: unsafe { TEGRA30_AHUB_APBIF_INT_SET },
    writeable_reg: Some(tegra30_ahub_apbif_wr_rd_reg),
    readable_reg: Some(tegra30_ahub_apbif_wr_rd_reg),
    volatile_reg: Some(tegra30_ahub_apbif_volatile_reg),
    precious_reg: Some(tegra30_ahub_apbif_precious_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn tegra30_ahub_ahub_wr_rd_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    if REG_IN_ARRAY(reg, TEGRA30_AHUB_AUDIO_RX, TEGRA30_AHUB_AUDIO_RX_STRIDE, TEGRA30_AHUB_AUDIO_RX_COUNT) {
        return true;
    }

    false
}

static mut tegra30_ahub_ahub_regmap_config: regmap_config = regmap_config {
    name: b"ahub\0".as_ptr() as *const core::ffi::c_char,
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: unsafe { LAST_REG(TEGRA30_AHUB_AUDIO_RX, TEGRA30_AHUB_AUDIO_RX_STRIDE, TEGRA30_AHUB_AUDIO_RX_COUNT) },
    writeable_reg: Some(tegra30_ahub_ahub_wr_rd_reg),
    readable_reg: Some(tegra30_ahub_ahub_wr_rd_reg),
    volatile_reg: None,
    precious_reg: None,
    cache_type: REGCACHE_FLAT,
};

static mut soc_data_tegra30: tegra30_ahub_soc_data = tegra30_ahub_soc_data {
    num_resets: 11,
    set_audio_cif: Some(tegra30_ahub_set_cif),
};

static mut soc_data_tegra114: tegra30_ahub_soc_data = tegra30_ahub_soc_data {
    num_resets: 13,
    set_audio_cif: Some(tegra30_ahub_set_cif),
};

static mut soc_data_tegra124: tegra30_ahub_soc_data = tegra30_ahub_soc_data {
    num_resets: 21,
    set_audio_cif: Some(tegra124_ahub_set_cif),
};

static mut tegra30_ahub_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: b"nvidia,tegra124-ahub\0".as_ptr() as *const core::ffi::c_char,
        data: unsafe { &soc_data_tegra124 as *const _ as *const core::ffi::c_void },
    },
    of_device_id {
        compatible: b"nvidia,tegra114-ahub\0".as_ptr() as *const core::ffi::c_char,
        data: unsafe { &soc_data_tegra114 as *const _ as *const core::ffi::c_void },
    },
    of_device_id {
        compatible: b"nvidia,tegra30-ahub\0".as_ptr() as *const core::ffi::c_char,
        data: unsafe { &soc_data_tegra30 as *const _ as *const core::ffi::c_void },
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, tegra30_ahub_of_match);

unsafe extern "C" fn tegra30_ahub_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let soc_data: *const tegra30_ahub_soc_data;
    let mut res0: *mut resource = core::ptr::null_mut();
    let regs_apbif: *mut core::ffi::c_void;
    let regs_ahub: *mut core::ffi::c_void;
    let mut ret: core::ffi::c_int = 0;

    soc_data = of_device_get_match_data(&mut (*pdev).dev);
    if soc_data.is_null() {
        return -EINVAL;
    }

    ahub = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<tegra30_ahub>(),
        GFP_KERNEL,
    ) as *mut tegra30_ahub;
    if ahub.is_null() {
        return -ENOMEM;
    }
    dev_set_drvdata(&mut (*pdev).dev, ahub as *mut core::ffi::c_void);

    // BUILD_BUG_ON(sizeof(ahub->resets) != sizeof(tegra30_ahub_resets_data));
    memcpy(
        (*ahub).resets.as_mut_ptr() as *mut core::ffi::c_void,
        tegra30_ahub_resets_data.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ahub).resets),
    );

    (*ahub).nresets = (*soc_data).num_resets;
    (*ahub).soc_data = soc_data;
    (*ahub).dev = &mut (*pdev).dev;

    (*ahub).clocks[(*ahub).nclocks as usize].id = b"apbif\0".as_ptr() as *const core::ffi::c_char;
    (*ahub).nclocks += 1;
    (*ahub).clocks[(*ahub).nclocks as usize].id = b"d_audio\0".as_ptr() as *const core::ffi::c_char;
    (*ahub).nclocks += 1;

    ret = devm_clk_bulk_get(&mut (*pdev).dev, (*ahub).nclocks, (*ahub).clocks.as_mut_ptr());
    if ret != 0 {
        ahub = core::ptr::null_mut();
        return ret;
    }

    ret = devm_reset_control_bulk_get_exclusive(
        &mut (*pdev).dev,
        (*ahub).nresets,
        (*ahub).resets.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"Can't get resets: %d\n\0".as_ptr() as *const core::ffi::c_char, ret);
        ahub = core::ptr::null_mut();
        return ret;
    }

    regs_apbif = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res0);
    if IS_ERR(regs_apbif) {
        ret = PTR_ERR(regs_apbif);
        ahub = core::ptr::null_mut();
        return ret;
    }

    (*ahub).apbif_addr = (*res0).start;

    (*ahub).regmap_apbif = devm_regmap_init_mmio(
        &mut (*pdev).dev,
        regs_apbif,
        &tegra30_ahub_apbif_regmap_config,
    );
    if IS_ERR((*ahub).regmap_apbif as *const core::ffi::c_void) {
        dev_err(&mut (*pdev).dev, b"apbif regmap init failed\n\0".as_ptr() as *const core::ffi::c_char);
        ret = PTR_ERR((*ahub).regmap_apbif as *const core::ffi::c_void);
        ahub = core::ptr::null_mut();
        return ret;
    }
    regcache_cache_only((*ahub).regmap_apbif, true);

    regs_ahub = devm_platform_ioremap_resource(pdev, 1);
    if IS_ERR(regs_ahub) {
        ret = PTR_ERR(regs_ahub);
        ahub = core::ptr::null_mut();
        return ret;
    }

    (*ahub).regmap_ahub = devm_regmap_init_mmio(
        &mut (*pdev).dev,
        regs_ahub,
        &tegra30_ahub_ahub_regmap_config,
    );
    if IS_ERR((*ahub).regmap_ahub as *const core::ffi::c_void) {
        dev_err(&mut (*pdev).dev, b"ahub regmap init failed\n\0".as_ptr() as *const core::ffi::c_char);
        ret = PTR_ERR((*ahub).regmap_ahub as *const core::ffi::c_void);
        ahub = core::ptr::null_mut();
        return ret;
    }
    regcache_cache_only((*ahub).regmap_ahub, true);

    pm_runtime_enable(&mut (*pdev).dev);

    of_platform_populate(core::ptr::null_mut(), core::ptr::null(), core::ptr::null(), &mut (*pdev).dev);

    0
}

unsafe extern "C" fn tegra30_ahub_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);

    ahub = core::ptr::null_mut();
}

// static const struct dev_pm_ops tegra30_ahub_pm_ops = {
//     RUNTIME_PM_OPS(tegra30_ahub_runtime_suspend,
//                    tegra30_ahub_runtime_resume, NULL)
//     SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
// };
static tegra30_ahub_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut tegra30_ahub_driver: platform_driver = platform_driver {
    probe: Some(tegra30_ahub_probe),
    remove: Some(tegra30_ahub_remove),
    driver: platform_driver_driver {
        name: DRV_NAME,
        of_match_table: unsafe { tegra30_ahub_of_match.as_ptr() },
        pm: &tegra30_ahub_pm_ops,
    },
};
// module_platform_driver(tegra30_ahub_driver);

#[no_mangle]
pub unsafe extern "C" fn tegra30_ahub_set_cif(
    regmap: *mut regmap,
    reg: core::ffi::c_uint,
    conf: *mut tegra30_ahub_cif_conf,
) {
    let value: core::ffi::c_uint;

    value = ((*conf).threshold << TEGRA30_AUDIOCIF_CTRL_FIFO_THRESHOLD_SHIFT)
        | (((*conf).audio_channels - 1) << TEGRA30_AUDIOCIF_CTRL_AUDIO_CHANNELS_SHIFT)
        | (((*conf).client_channels - 1) << TEGRA30_AUDIOCIF_CTRL_CLIENT_CHANNELS_SHIFT)
        | ((*conf).audio_bits << TEGRA30_AUDIOCIF_CTRL_AUDIO_BITS_SHIFT)
        | ((*conf).client_bits << TEGRA30_AUDIOCIF_CTRL_CLIENT_BITS_SHIFT)
        | ((*conf).expand << TEGRA30_AUDIOCIF_CTRL_EXPAND_SHIFT)
        | ((*conf).stereo_conv << TEGRA30_AUDIOCIF_CTRL_STEREO_CONV_SHIFT)
        | ((*conf).replicate << TEGRA30_AUDIOCIF_CTRL_REPLICATE_SHIFT)
        | ((*conf).direction << TEGRA30_AUDIOCIF_CTRL_DIRECTION_SHIFT)
        | ((*conf).truncate << TEGRA30_AUDIOCIF_CTRL_TRUNCATE_SHIFT)
        | ((*conf).mono_conv << TEGRA30_AUDIOCIF_CTRL_MONO_CONV_SHIFT);

    regmap_write(regmap, reg, value);
}
// EXPORT_SYMBOL_GPL(tegra30_ahub_set_cif);

#[no_mangle]
pub unsafe extern "C" fn tegra124_ahub_set_cif(
    regmap: *mut regmap,
    reg: core::ffi::c_uint,
    conf: *mut tegra30_ahub_cif_conf,
) {
    let value: core::ffi::c_uint;

    value = ((*conf).threshold << TEGRA124_AUDIOCIF_CTRL_FIFO_THRESHOLD_SHIFT)
        | (((*conf).audio_channels - 1) << TEGRA124_AUDIOCIF_CTRL_AUDIO_CHANNELS_SHIFT)
        | (((*conf).client_channels - 1) << TEGRA124_AUDIOCIF_CTRL_CLIENT_CHANNELS_SHIFT)
        | ((*conf).audio_bits << TEGRA30_AUDIOCIF_CTRL_AUDIO_BITS_SHIFT)
        | ((*conf).client_bits << TEGRA30_AUDIOCIF_CTRL_CLIENT_BITS_SHIFT)
        | ((*conf).expand << TEGRA30_AUDIOCIF_CTRL_EXPAND_SHIFT)
        | ((*conf).stereo_conv << TEGRA30_AUDIOCIF_CTRL_STEREO_CONV_SHIFT)
        | ((*conf).replicate << TEGRA30_AUDIOCIF_CTRL_REPLICATE_SHIFT)
        | ((*conf).direction << TEGRA30_AUDIOCIF_CTRL_DIRECTION_SHIFT)
        | ((*conf).truncate << TEGRA30_AUDIOCIF_CTRL_TRUNCATE_SHIFT)
        | ((*conf).mono_conv << TEGRA30_AUDIOCIF_CTRL_MONO_CONV_SHIFT);

    regmap_write(regmap, reg, value);
}
// EXPORT_SYMBOL_GPL(tegra124_ahub_set_cif);

// MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>");
// MODULE_DESCRIPTION("Tegra30 AHUB driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
