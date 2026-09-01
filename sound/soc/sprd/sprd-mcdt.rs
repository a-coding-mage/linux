// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Spreadtrum Communications Inc.

// Rust translation of the implementation source. C include dependencies:
// linux/errno.h, linux/interrupt.h, linux/io.h, linux/kernel.h, linux/module.h,
// linux/mutex.h, linux/of.h, linux/platform_device.h, linux/spinlock.h, and
// "sprd-mcdt.h".

use core::ffi::{c_char, c_int, c_void};

type u8 = u8;
type u32 = u32;
type resource_size_t = usize;
type irqreturn_t = c_int;
type gfp_t = u32;

const IRQ_HANDLED: irqreturn_t = 1;
const GFP_KERNEL: gfp_t = 0;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

/* MCDT registers definition */
const MCDT_CH0_TXD: u32 = 0x0;
const MCDT_CH0_RXD: u32 = 0x28;
const MCDT_DAC0_WTMK: u32 = 0x60;
const MCDT_ADC0_WTMK: u32 = 0x88;
const MCDT_DMA_EN: u32 = 0xb0;

const MCDT_INT_EN0: u32 = 0xb4;
const MCDT_INT_EN1: u32 = 0xb8;
const MCDT_INT_EN2: u32 = 0xbc;

const MCDT_INT_CLR0: u32 = 0xc0;
const MCDT_INT_CLR1: u32 = 0xc4;
const MCDT_INT_CLR2: u32 = 0xc8;

const MCDT_INT_RAW1: u32 = 0xcc;
const MCDT_INT_RAW2: u32 = 0xd0;
const MCDT_INT_RAW3: u32 = 0xd4;

const MCDT_INT_MSK1: u32 = 0xd8;
const MCDT_INT_MSK2: u32 = 0xdc;
const MCDT_INT_MSK3: u32 = 0xe0;

const MCDT_DAC0_FIFO_ADDR_ST: u32 = 0xe4;
const MCDT_ADC0_FIFO_ADDR_ST: u32 = 0xe8;

const MCDT_CH_FIFO_ST0: u32 = 0x134;
const MCDT_CH_FIFO_ST1: u32 = 0x138;
const MCDT_CH_FIFO_ST2: u32 = 0x13c;

const MCDT_INT_MSK_CFG0: u32 = 0x140;
const MCDT_INT_MSK_CFG1: u32 = 0x144;

const MCDT_DMA_CFG0: u32 = 0x148;
const MCDT_FIFO_CLR: u32 = 0x14c;
const MCDT_DMA_CFG1: u32 = 0x150;
const MCDT_DMA_CFG2: u32 = 0x154;
const MCDT_DMA_CFG3: u32 = 0x158;
const MCDT_DMA_CFG4: u32 = 0x15c;
const MCDT_DMA_CFG5: u32 = 0x160;

/* Channel water mark definition */
const MCDT_CH_FIFO_AE_SHIFT: u32 = 16;
const MCDT_CH_FIFO_AE_MASK: u32 = GENMASK(24, 16);
const MCDT_CH_FIFO_AF_MASK: u32 = GENMASK(8, 0);

/* DMA channel select definition */
const MCDT_DMA_CH0_SEL_MASK: u32 = GENMASK(3, 0);
const MCDT_DMA_CH0_SEL_SHIFT: u32 = 0;
const MCDT_DMA_CH1_SEL_MASK: u32 = GENMASK(7, 4);
const MCDT_DMA_CH1_SEL_SHIFT: u32 = 4;
const MCDT_DMA_CH2_SEL_MASK: u32 = GENMASK(11, 8);
const MCDT_DMA_CH2_SEL_SHIFT: u32 = 8;
const MCDT_DMA_CH3_SEL_MASK: u32 = GENMASK(15, 12);
const MCDT_DMA_CH3_SEL_SHIFT: u32 = 12;
const MCDT_DMA_CH4_SEL_MASK: u32 = GENMASK(19, 16);
const MCDT_DMA_CH4_SEL_SHIFT: u32 = 16;
const MCDT_DAC_DMA_SHIFT: u32 = 16;

/* DMA channel ACK select definition */
const MCDT_DMA_ACK_SEL_MASK: u32 = GENMASK(3, 0);

/* Channel FIFO definition */
const MCDT_CH_FIFO_ADDR_SHIFT: u32 = 16;
const MCDT_CH_FIFO_ADDR_MASK: u32 = GENMASK(9, 0);
const MCDT_ADC_FIFO_SHIFT: u32 = 16;
const MCDT_FIFO_LENGTH: u32 = 512;

const MCDT_ADC_CHANNEL_NUM: usize = 10;
const MCDT_DAC_CHANNEL_NUM: usize = 10;
const MCDT_CHANNEL_NUM: usize = MCDT_ADC_CHANNEL_NUM + MCDT_DAC_CHANNEL_NUM;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sprd_mcdt_fifo_int {
    MCDT_ADC_FIFO_AE_INT,
    MCDT_ADC_FIFO_AF_INT,
    MCDT_DAC_FIFO_AE_INT,
    MCDT_DAC_FIFO_AF_INT,
    MCDT_ADC_FIFO_OV_INT,
    MCDT_DAC_FIFO_OV_INT,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum sprd_mcdt_fifo_sts {
    MCDT_ADC_FIFO_REAL_FULL,
    MCDT_ADC_FIFO_REAL_EMPTY,
    MCDT_ADC_FIFO_AF,
    MCDT_ADC_FIFO_AE,
    MCDT_DAC_FIFO_REAL_FULL,
    MCDT_DAC_FIFO_REAL_EMPTY,
    MCDT_DAC_FIFO_AF,
    MCDT_DAC_FIFO_AE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum sprd_mcdt_dma_chan {
    SPRD_MCDT_DMA_CH0,
    SPRD_MCDT_DMA_CH1,
    SPRD_MCDT_DMA_CH2,
    SPRD_MCDT_DMA_CH3,
    SPRD_MCDT_DMA_CH4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sprd_mcdt_channel_type {
    SPRD_MCDT_ADC_CHAN,
    SPRD_MCDT_DAC_CHAN,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct resource {
    start: resource_size_t,
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: driver,
}

#[repr(C)]
struct sprd_mcdt_chan_callback {
    notify: Option<unsafe extern "C" fn(*mut c_void)>,
    data: *mut c_void,
}

#[repr(C)]
struct sprd_mcdt_chan {
    id: u8,
    type_: sprd_mcdt_channel_type,
    fifo_phys: resource_size_t,
    mcdt: *mut sprd_mcdt_dev,
    list: list_head,
    cb: *mut sprd_mcdt_chan_callback,
    dma_enable: bool,
    int_enable: bool,
}

#[repr(C)]
struct sprd_mcdt_dev {
    dev: *mut device,
    base: *mut c_void,
    lock: spinlock_t,
    chan: [sprd_mcdt_chan; MCDT_CHANNEL_NUM],
}

static mut sprd_mcdt_chan_list: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};
static mut sprd_mcdt_list_mutex: mutex = mutex { _private: [] };

extern "C" {
    fn readl_relaxed(addr: *const c_void) -> u32;
    fn writel_relaxed(val: u32, addr: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_int,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_irq(pdev: *mut platform_device, num: c_int) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: u32,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn list_del_init(entry: *mut list_head);
    fn list_entry_is_head(pos: *mut sprd_mcdt_chan, head: *mut list_head, member: *mut list_head)
        -> bool;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_del(entry: *mut list_head);
}

unsafe fn sprd_mcdt_update(mcdt: *mut sprd_mcdt_dev, reg: u32, val: u32, mask: u32) {
    let orig: u32 = readl_relaxed((*mcdt).base.byte_add(reg as usize));
    let tmp: u32 = (orig & !mask) | val;

    writel_relaxed(tmp, (*mcdt).base.byte_add(reg as usize));
}

unsafe fn sprd_mcdt_dac_set_watermark(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    full: u32,
    empty: u32,
) {
    let reg: u32 = MCDT_DAC0_WTMK + channel as u32 * 4;
    let mut water_mark: u32 = (empty << MCDT_CH_FIFO_AE_SHIFT) & MCDT_CH_FIFO_AE_MASK;

    water_mark |= full & MCDT_CH_FIFO_AF_MASK;
    sprd_mcdt_update(
        mcdt,
        reg,
        water_mark,
        MCDT_CH_FIFO_AE_MASK | MCDT_CH_FIFO_AF_MASK,
    );
}

unsafe fn sprd_mcdt_adc_set_watermark(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    full: u32,
    empty: u32,
) {
    let reg: u32 = MCDT_ADC0_WTMK + channel as u32 * 4;
    let mut water_mark: u32 = (empty << MCDT_CH_FIFO_AE_SHIFT) & MCDT_CH_FIFO_AE_MASK;

    water_mark |= full & MCDT_CH_FIFO_AF_MASK;
    sprd_mcdt_update(
        mcdt,
        reg,
        water_mark,
        MCDT_CH_FIFO_AE_MASK | MCDT_CH_FIFO_AF_MASK,
    );
}

unsafe fn sprd_mcdt_dac_dma_enable(mcdt: *mut sprd_mcdt_dev, channel: u8, enable: bool) {
    let shift: u32 = MCDT_DAC_DMA_SHIFT + channel as u32;

    if enable {
        sprd_mcdt_update(mcdt, MCDT_DMA_EN, BIT(shift), BIT(shift));
    } else {
        sprd_mcdt_update(mcdt, MCDT_DMA_EN, 0, BIT(shift));
    }
}

unsafe fn sprd_mcdt_adc_dma_enable(mcdt: *mut sprd_mcdt_dev, channel: u8, enable: bool) {
    if enable {
        sprd_mcdt_update(mcdt, MCDT_DMA_EN, BIT(channel as u32), BIT(channel as u32));
    } else {
        sprd_mcdt_update(mcdt, MCDT_DMA_EN, 0, BIT(channel as u32));
    }
}

unsafe fn sprd_mcdt_ap_int_enable(mcdt: *mut sprd_mcdt_dev, channel: u8, enable: bool) {
    if enable {
        sprd_mcdt_update(
            mcdt,
            MCDT_INT_MSK_CFG0,
            BIT(channel as u32),
            BIT(channel as u32),
        );
    } else {
        sprd_mcdt_update(mcdt, MCDT_INT_MSK_CFG0, 0, BIT(channel as u32));
    }
}

unsafe fn sprd_mcdt_dac_write_fifo(mcdt: *mut sprd_mcdt_dev, channel: u8, val: u32) {
    let reg: u32 = MCDT_CH0_TXD + channel as u32 * 4;

    writel_relaxed(val, (*mcdt).base.byte_add(reg as usize));
}

unsafe fn sprd_mcdt_adc_read_fifo(mcdt: *mut sprd_mcdt_dev, channel: u8, val: *mut u32) {
    let reg: u32 = MCDT_CH0_RXD + channel as u32 * 4;

    *val = readl_relaxed((*mcdt).base.byte_add(reg as usize));
}

unsafe fn sprd_mcdt_dac_dma_chn_select(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    dma_chan: sprd_mcdt_dma_chan,
) {
    match dma_chan {
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH0 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG0,
            (channel as u32) << MCDT_DMA_CH0_SEL_SHIFT,
            MCDT_DMA_CH0_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH1 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG0,
            (channel as u32) << MCDT_DMA_CH1_SEL_SHIFT,
            MCDT_DMA_CH1_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH2 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG0,
            (channel as u32) << MCDT_DMA_CH2_SEL_SHIFT,
            MCDT_DMA_CH2_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH3 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG0,
            (channel as u32) << MCDT_DMA_CH3_SEL_SHIFT,
            MCDT_DMA_CH3_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH4 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG0,
            (channel as u32) << MCDT_DMA_CH4_SEL_SHIFT,
            MCDT_DMA_CH4_SEL_MASK,
        ),
    }
}

unsafe fn sprd_mcdt_adc_dma_chn_select(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    dma_chan: sprd_mcdt_dma_chan,
) {
    match dma_chan {
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH0 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG1,
            (channel as u32) << MCDT_DMA_CH0_SEL_SHIFT,
            MCDT_DMA_CH0_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH1 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG1,
            (channel as u32) << MCDT_DMA_CH1_SEL_SHIFT,
            MCDT_DMA_CH1_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH2 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG1,
            (channel as u32) << MCDT_DMA_CH2_SEL_SHIFT,
            MCDT_DMA_CH2_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH3 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG1,
            (channel as u32) << MCDT_DMA_CH3_SEL_SHIFT,
            MCDT_DMA_CH3_SEL_MASK,
        ),
        sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH4 => sprd_mcdt_update(
            mcdt,
            MCDT_DMA_CFG1,
            (channel as u32) << MCDT_DMA_CH4_SEL_SHIFT,
            MCDT_DMA_CH4_SEL_MASK,
        ),
    }
}

fn sprd_mcdt_dma_ack_shift(channel: u8) -> u32 {
    match channel {
        1 | 9 => 4,
        2 => 8,
        3 => 12,
        4 => 16,
        5 => 20,
        6 => 24,
        7 => 28,
        _ => 0,
    }
}

unsafe fn sprd_mcdt_dac_dma_ack_select(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    dma_chan: sprd_mcdt_dma_chan,
) {
    let reg: u32;
    let shift: u32 = sprd_mcdt_dma_ack_shift(channel);
    let ack: u32 = dma_chan as u32;

    match channel {
        0..=7 => reg = MCDT_DMA_CFG2,
        8..=9 => reg = MCDT_DMA_CFG3,
        _ => return,
    }

    sprd_mcdt_update(mcdt, reg, ack << shift, MCDT_DMA_ACK_SEL_MASK << shift);
}

unsafe fn sprd_mcdt_adc_dma_ack_select(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    dma_chan: sprd_mcdt_dma_chan,
) {
    let reg: u32;
    let shift: u32 = sprd_mcdt_dma_ack_shift(channel);
    let ack: u32 = dma_chan as u32;

    match channel {
        0..=7 => reg = MCDT_DMA_CFG4,
        8..=9 => reg = MCDT_DMA_CFG5,
        _ => return,
    }

    sprd_mcdt_update(mcdt, reg, ack << shift, MCDT_DMA_ACK_SEL_MASK << shift);
}

unsafe fn sprd_mcdt_chan_fifo_sts(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    fifo_sts: sprd_mcdt_fifo_sts,
) -> bool {
    let reg: u32;
    let shift: u32;

    match channel {
        0..=3 => reg = MCDT_CH_FIFO_ST0,
        4..=7 => reg = MCDT_CH_FIFO_ST1,
        8..=9 => reg = MCDT_CH_FIFO_ST2,
        _ => return false,
    }

    match channel {
        0 | 4 | 8 => shift = fifo_sts as u32,
        1 | 5 | 9 => shift = 8 + fifo_sts as u32,
        2 | 6 => shift = 16 + fifo_sts as u32,
        3 | 7 => shift = 24 + fifo_sts as u32,
        _ => return false,
    }

    (readl_relaxed((*mcdt).base.byte_add(reg as usize)) & BIT(shift)) != 0
}

unsafe fn sprd_mcdt_dac_fifo_clear(mcdt: *mut sprd_mcdt_dev, channel: u8) {
    sprd_mcdt_update(mcdt, MCDT_FIFO_CLR, BIT(channel as u32), BIT(channel as u32));
}

unsafe fn sprd_mcdt_adc_fifo_clear(mcdt: *mut sprd_mcdt_dev, channel: u8) {
    let shift: u32 = MCDT_ADC_FIFO_SHIFT + channel as u32;

    sprd_mcdt_update(mcdt, MCDT_FIFO_CLR, BIT(shift), BIT(shift));
}

unsafe fn sprd_mcdt_dac_fifo_avail(mcdt: *mut sprd_mcdt_dev, channel: u8) -> u32 {
    let reg: u32 = MCDT_DAC0_FIFO_ADDR_ST + channel as u32 * 8;
    let r_addr: u32 =
        (readl_relaxed((*mcdt).base.byte_add(reg as usize)) >> MCDT_CH_FIFO_ADDR_SHIFT)
            & MCDT_CH_FIFO_ADDR_MASK;
    let w_addr: u32 =
        readl_relaxed((*mcdt).base.byte_add(reg as usize)) & MCDT_CH_FIFO_ADDR_MASK;

    if w_addr >= r_addr {
        4 * (MCDT_FIFO_LENGTH - w_addr + r_addr)
    } else {
        4 * (r_addr - w_addr)
    }
}

unsafe fn sprd_mcdt_adc_fifo_avail(mcdt: *mut sprd_mcdt_dev, channel: u8) -> u32 {
    let reg: u32 = MCDT_ADC0_FIFO_ADDR_ST + channel as u32 * 8;
    let r_addr: u32 =
        (readl_relaxed((*mcdt).base.byte_add(reg as usize)) >> MCDT_CH_FIFO_ADDR_SHIFT)
            & MCDT_CH_FIFO_ADDR_MASK;
    let w_addr: u32 =
        readl_relaxed((*mcdt).base.byte_add(reg as usize)) & MCDT_CH_FIFO_ADDR_MASK;

    if w_addr >= r_addr {
        4 * (w_addr - r_addr)
    } else {
        4 * (MCDT_FIFO_LENGTH - r_addr + w_addr)
    }
}

fn sprd_mcdt_int_type_shift(channel: u8, int_type: sprd_mcdt_fifo_int) -> u32 {
    match channel {
        0 | 4 | 8 => int_type as u32,
        1 | 5 | 9 => 8 + int_type as u32,
        2 | 6 => 16 + int_type as u32,
        3 | 7 => 24 + int_type as u32,
        _ => 0,
    }
}

unsafe fn sprd_mcdt_chan_int_en(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    int_type: sprd_mcdt_fifo_int,
    enable: bool,
) {
    let reg: u32;
    let shift: u32 = sprd_mcdt_int_type_shift(channel, int_type);

    match channel {
        0..=3 => reg = MCDT_INT_EN0,
        4..=7 => reg = MCDT_INT_EN1,
        8..=9 => reg = MCDT_INT_EN2,
        _ => return,
    }

    if enable {
        sprd_mcdt_update(mcdt, reg, BIT(shift), BIT(shift));
    } else {
        sprd_mcdt_update(mcdt, reg, 0, BIT(shift));
    }
}

unsafe fn sprd_mcdt_chan_int_clear(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    int_type: sprd_mcdt_fifo_int,
) {
    let reg: u32;
    let shift: u32 = sprd_mcdt_int_type_shift(channel, int_type);

    match channel {
        0..=3 => reg = MCDT_INT_CLR0,
        4..=7 => reg = MCDT_INT_CLR1,
        8..=9 => reg = MCDT_INT_CLR2,
        _ => return,
    }

    sprd_mcdt_update(mcdt, reg, BIT(shift), BIT(shift));
}

unsafe fn sprd_mcdt_chan_int_sts(
    mcdt: *mut sprd_mcdt_dev,
    channel: u8,
    int_type: sprd_mcdt_fifo_int,
) -> bool {
    let reg: u32;
    let shift: u32 = sprd_mcdt_int_type_shift(channel, int_type);

    match channel {
        0..=3 => reg = MCDT_INT_MSK1,
        4..=7 => reg = MCDT_INT_MSK2,
        8..=9 => reg = MCDT_INT_MSK3,
        _ => return false,
    }

    (readl_relaxed((*mcdt).base.byte_add(reg as usize)) & BIT(shift)) != 0
}

unsafe extern "C" fn sprd_mcdt_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mcdt: *mut sprd_mcdt_dev = dev_id as *mut sprd_mcdt_dev;
    let mut i: c_int;

    // C source uses guard(spinlock)(&mcdt->lock).

    i = 0;
    while i < MCDT_ADC_CHANNEL_NUM as c_int {
        if sprd_mcdt_chan_int_sts(
            mcdt,
            i as u8,
            sprd_mcdt_fifo_int::MCDT_ADC_FIFO_AF_INT,
        ) {
            let chan: *mut sprd_mcdt_chan = &mut (*mcdt).chan[i as usize];

            sprd_mcdt_chan_int_clear(
                mcdt,
                i as u8,
                sprd_mcdt_fifo_int::MCDT_ADC_FIFO_AF_INT,
            );
            if !(*chan).cb.is_null() {
                if let Some(notify) = (*(*chan).cb).notify {
                    notify((*(*chan).cb).data);
                }
            }
        }
        i += 1;
    }

    i = 0;
    while i < MCDT_DAC_CHANNEL_NUM as c_int {
        if sprd_mcdt_chan_int_sts(
            mcdt,
            i as u8,
            sprd_mcdt_fifo_int::MCDT_DAC_FIFO_AE_INT,
        ) {
            let chan: *mut sprd_mcdt_chan =
                &mut (*mcdt).chan[i as usize + MCDT_ADC_CHANNEL_NUM];

            sprd_mcdt_chan_int_clear(
                mcdt,
                i as u8,
                sprd_mcdt_fifo_int::MCDT_DAC_FIFO_AE_INT,
            );
            if !(*chan).cb.is_null() {
                if let Some(notify) = (*(*chan).cb).notify {
                    notify((*(*chan).cb).data);
                }
            }
        }
        i += 1;
    }

    IRQ_HANDLED
}

/**
 * sprd_mcdt_chan_write - write data to the MCDT channel's fifo
 * @chan: the MCDT channel
 * @tx_buf: send buffer
 * @size: data size
 *
 * Note: We can not write data to the channel fifo when enabling the DMA mode,
 * otherwise the channel fifo data will be invalid.
 *
 * If there are not enough space of the channel fifo, it will return errors
 * to users.
 *
 * Returns 0 on success, or an appropriate error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_write(
    chan: *mut sprd_mcdt_chan,
    tx_buf: *mut c_char,
    size: u32,
) -> c_int {
    let mcdt: *mut sprd_mcdt_dev = (*chan).mcdt;
    let avail: c_int;
    let mut i: c_int = 0;
    let words: c_int = (size / 4) as c_int;
    let mut buf: *mut u32 = tx_buf as *mut u32;

    // C source uses guard(spinlock_irqsave)(&mcdt->lock).

    if (*chan).dma_enable {
        dev_err(
            (*mcdt).dev,
            c"Can not write data when DMA mode enabled\n".as_ptr(),
        );
        return -EINVAL;
    }

    if sprd_mcdt_chan_fifo_sts(
        mcdt,
        (*chan).id,
        sprd_mcdt_fifo_sts::MCDT_DAC_FIFO_REAL_FULL,
    ) {
        dev_err((*mcdt).dev, c"Channel fifo is full now\n".as_ptr());
        return -EBUSY;
    }

    avail = sprd_mcdt_dac_fifo_avail(mcdt, (*chan).id) as c_int;
    if size > avail as u32 {
        dev_err(
            (*mcdt).dev,
            c"Data size is larger than the available fifo size\n".as_ptr(),
        );
        return -EBUSY;
    }

    while {
        i += 1;
        i < words + 1
    } {
        sprd_mcdt_dac_write_fifo(mcdt, (*chan).id, *buf);
        buf = buf.add(1);
    }

    0
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_chan_write);

/**
 * sprd_mcdt_chan_read - read data from the MCDT channel's fifo
 * @chan: the MCDT channel
 * @rx_buf: receive buffer
 * @size: data size
 *
 * Note: We can not read data from the channel fifo when enabling the DMA mode,
 * otherwise the reading data will be invalid.
 *
 * Usually user need start to read data once receiving the fifo full interrupt.
 *
 * Returns data size of reading successfully, or an error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_read(
    chan: *mut sprd_mcdt_chan,
    rx_buf: *mut c_char,
    size: u32,
) -> c_int {
    let mcdt: *mut sprd_mcdt_dev = (*chan).mcdt;
    let mut i: c_int = 0;
    let avail: c_int;
    let mut words: c_int = (size / 4) as c_int;
    let mut buf: *mut u32 = rx_buf as *mut u32;

    // C source uses guard(spinlock_irqsave)(&mcdt->lock).

    if (*chan).dma_enable {
        dev_err((*mcdt).dev, c"Can not read data when DMA mode enabled\n".as_ptr());
        return -EINVAL;
    }

    if sprd_mcdt_chan_fifo_sts(
        mcdt,
        (*chan).id,
        sprd_mcdt_fifo_sts::MCDT_ADC_FIFO_REAL_EMPTY,
    ) {
        dev_err((*mcdt).dev, c"Channel fifo is empty\n".as_ptr());
        return -EBUSY;
    }

    avail = sprd_mcdt_adc_fifo_avail(mcdt, (*chan).id) as c_int;
    if size > avail as u32 {
        words = avail / 4;
    }

    while {
        i += 1;
        i < words + 1
    } {
        sprd_mcdt_adc_read_fifo(mcdt, (*chan).id, buf);
        buf = buf.add(1);
    }

    words * 4
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_chan_read);

/**
 * sprd_mcdt_chan_int_enable - enable the interrupt mode for the MCDT channel
 * @chan: the MCDT channel
 * @water_mark: water mark to trigger a interrupt
 * @cb: callback when a interrupt happened
 *
 * Now it only can enable fifo almost full interrupt for ADC channel and fifo
 * almost empty interrupt for DAC channel. Morevoer for interrupt mode, user
 * should use sprd_mcdt_chan_read() or sprd_mcdt_chan_write() to read or write
 * data manually.
 *
 * For ADC channel, user can start to read data once receiving one fifo full
 * interrupt. For DAC channel, user can start to write data once receiving one
 * fifo empty interrupt or just call sprd_mcdt_chan_write() to write data
 * directly.
 *
 * Returns 0 on success, or an error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_int_enable(
    chan: *mut sprd_mcdt_chan,
    water_mark: u32,
    cb: *mut sprd_mcdt_chan_callback,
) -> c_int {
    let mcdt: *mut sprd_mcdt_dev = (*chan).mcdt;
    let mut ret: c_int = 0;

    // C source uses guard(spinlock_irqsave)(&mcdt->lock).

    if (*chan).dma_enable || (*chan).int_enable {
        dev_err((*mcdt).dev, c"Failed to set interrupt mode.\n".as_ptr());
        return -EINVAL;
    }

    match (*chan).type_ {
        sprd_mcdt_channel_type::SPRD_MCDT_ADC_CHAN => {
            sprd_mcdt_adc_fifo_clear(mcdt, (*chan).id);
            sprd_mcdt_adc_set_watermark(mcdt, (*chan).id, water_mark, MCDT_FIFO_LENGTH - 1);
            sprd_mcdt_chan_int_en(
                mcdt,
                (*chan).id,
                sprd_mcdt_fifo_int::MCDT_ADC_FIFO_AF_INT,
                true,
            );
            sprd_mcdt_ap_int_enable(mcdt, (*chan).id, true);
        }
        sprd_mcdt_channel_type::SPRD_MCDT_DAC_CHAN => {
            sprd_mcdt_dac_fifo_clear(mcdt, (*chan).id);
            sprd_mcdt_dac_set_watermark(mcdt, (*chan).id, MCDT_FIFO_LENGTH - 1, water_mark);
            sprd_mcdt_chan_int_en(
                mcdt,
                (*chan).id,
                sprd_mcdt_fifo_int::MCDT_DAC_FIFO_AE_INT,
                true,
            );
            sprd_mcdt_ap_int_enable(mcdt, (*chan).id, true);
        }
    }

    if ret == 0 {
        (*chan).cb = cb;
        (*chan).int_enable = true;
    }

    ret
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_chan_int_enable);

/**
 * sprd_mcdt_chan_int_disable - disable the interrupt mode for the MCDT channel
 * @chan: the MCDT channel
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_int_disable(chan: *mut sprd_mcdt_chan) {
    let mcdt: *mut sprd_mcdt_dev = (*chan).mcdt;

    // C source uses guard(spinlock_irqsave)(&mcdt->lock).

    if !(*chan).int_enable {
        return;
    }

    match (*chan).type_ {
        sprd_mcdt_channel_type::SPRD_MCDT_ADC_CHAN => {
            sprd_mcdt_chan_int_en(
                mcdt,
                (*chan).id,
                sprd_mcdt_fifo_int::MCDT_ADC_FIFO_AF_INT,
                false,
            );
            sprd_mcdt_chan_int_clear(
                mcdt,
                (*chan).id,
                sprd_mcdt_fifo_int::MCDT_ADC_FIFO_AF_INT,
            );
            sprd_mcdt_ap_int_enable(mcdt, (*chan).id, false);
        }
        sprd_mcdt_channel_type::SPRD_MCDT_DAC_CHAN => {
            sprd_mcdt_chan_int_en(
                mcdt,
                (*chan).id,
                sprd_mcdt_fifo_int::MCDT_DAC_FIFO_AE_INT,
                false,
            );
            sprd_mcdt_chan_int_clear(
                mcdt,
                (*chan).id,
                sprd_mcdt_fifo_int::MCDT_DAC_FIFO_AE_INT,
            );
            sprd_mcdt_ap_int_enable(mcdt, (*chan).id, false);
        }
    }

    (*chan).int_enable = false;
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_chan_int_disable);

/**
 * sprd_mcdt_chan_dma_enable - enable the DMA mode for the MCDT channel
 * @chan: the MCDT channel
 * @dma_chan: specify which DMA channel will be used for this MCDT channel
 * @water_mark: water mark to trigger a DMA request
 *
 * Enable the DMA mode for the MCDT channel, that means we can use DMA to
 * transfer data to the channel fifo and do not need reading/writing data
 * manually.
 *
 * Returns 0 on success, or an error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_dma_enable(
    chan: *mut sprd_mcdt_chan,
    dma_chan: sprd_mcdt_dma_chan,
    water_mark: u32,
) -> c_int {
    let mcdt: *mut sprd_mcdt_dev = (*chan).mcdt;
    let mut ret: c_int = 0;

    // C source uses guard(spinlock_irqsave)(&mcdt->lock).

    if (*chan).dma_enable
        || (*chan).int_enable
        || dma_chan > sprd_mcdt_dma_chan::SPRD_MCDT_DMA_CH4
    {
        dev_err((*mcdt).dev, c"Failed to set DMA mode\n".as_ptr());
        return -EINVAL;
    }

    match (*chan).type_ {
        sprd_mcdt_channel_type::SPRD_MCDT_ADC_CHAN => {
            sprd_mcdt_adc_fifo_clear(mcdt, (*chan).id);
            sprd_mcdt_adc_set_watermark(mcdt, (*chan).id, water_mark, MCDT_FIFO_LENGTH - 1);
            sprd_mcdt_adc_dma_enable(mcdt, (*chan).id, true);
            sprd_mcdt_adc_dma_chn_select(mcdt, (*chan).id, dma_chan);
            sprd_mcdt_adc_dma_ack_select(mcdt, (*chan).id, dma_chan);
        }
        sprd_mcdt_channel_type::SPRD_MCDT_DAC_CHAN => {
            sprd_mcdt_dac_fifo_clear(mcdt, (*chan).id);
            sprd_mcdt_dac_set_watermark(mcdt, (*chan).id, MCDT_FIFO_LENGTH - 1, water_mark);
            sprd_mcdt_dac_dma_enable(mcdt, (*chan).id, true);
            sprd_mcdt_dac_dma_chn_select(mcdt, (*chan).id, dma_chan);
            sprd_mcdt_dac_dma_ack_select(mcdt, (*chan).id, dma_chan);
        }
    }

    if ret == 0 {
        (*chan).dma_enable = true;
    }

    ret
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_chan_dma_enable);

/**
 * sprd_mcdt_chan_dma_disable - disable the DMA mode for the MCDT channel
 * @chan: the MCDT channel
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_chan_dma_disable(chan: *mut sprd_mcdt_chan) {
    let mcdt: *mut sprd_mcdt_dev = (*chan).mcdt;

    // C source uses guard(spinlock_irqsave)(&mcdt->lock).

    if !(*chan).dma_enable {
        return;
    }

    match (*chan).type_ {
        sprd_mcdt_channel_type::SPRD_MCDT_ADC_CHAN => {
            sprd_mcdt_adc_dma_enable(mcdt, (*chan).id, false);
            sprd_mcdt_adc_fifo_clear(mcdt, (*chan).id);
        }
        sprd_mcdt_channel_type::SPRD_MCDT_DAC_CHAN => {
            sprd_mcdt_dac_dma_enable(mcdt, (*chan).id, false);
            sprd_mcdt_dac_fifo_clear(mcdt, (*chan).id);
        }
    }

    (*chan).dma_enable = false;
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_chan_dma_disable);

/**
 * sprd_mcdt_request_chan - request one MCDT channel
 * @channel: channel id
 * @type: channel type, it can be one ADC channel or DAC channel
 *
 * Rreturn NULL if no available channel.
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_request_chan(
    channel: u8,
    type_: sprd_mcdt_channel_type,
) -> *mut sprd_mcdt_chan {
    let mut temp: *mut sprd_mcdt_chan = core::ptr::null_mut();

    // C source uses guard(mutex)(&sprd_mcdt_list_mutex).
    // C source iterates list_for_each_entry(temp, &sprd_mcdt_chan_list, list).
    TODO_list_for_each_entry_sprd_mcdt_chan(&mut temp, &raw mut sprd_mcdt_chan_list);
    if !temp.is_null() && (*temp).type_ == type_ && (*temp).id == channel {
        list_del_init(&mut (*temp).list);
    }

    if list_entry_is_head(temp, &raw mut sprd_mcdt_chan_list, &mut (*temp).list) {
        temp = core::ptr::null_mut();
    }

    temp
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_request_chan);

/**
 * sprd_mcdt_free_chan - free one MCDT channel
 * @chan: the channel to be freed
 */
#[no_mangle]
pub unsafe extern "C" fn sprd_mcdt_free_chan(chan: *mut sprd_mcdt_chan) {
    let mut temp: *mut sprd_mcdt_chan = core::ptr::null_mut();

    sprd_mcdt_chan_dma_disable(chan);
    sprd_mcdt_chan_int_disable(chan);

    // C source uses guard(mutex)(&sprd_mcdt_list_mutex).
    // C source iterates list_for_each_entry(temp, &sprd_mcdt_chan_list, list).
    TODO_list_for_each_entry_sprd_mcdt_chan(&mut temp, &raw mut sprd_mcdt_chan_list);
    if temp == chan {
        return;
    }

    list_add_tail(&mut (*chan).list, &raw mut sprd_mcdt_chan_list);
}
// EXPORT_SYMBOL_GPL(sprd_mcdt_free_chan);

extern "C" {
    fn TODO_list_for_each_entry_sprd_mcdt_chan(
        pos: *mut *mut sprd_mcdt_chan,
        head: *mut list_head,
    );
    fn TODO_list_for_each_entry_safe_sprd_mcdt_chan(
        pos: *mut *mut sprd_mcdt_chan,
        n: *mut *mut sprd_mcdt_chan,
        head: *mut list_head,
    );
}

unsafe fn sprd_mcdt_init_chans(mcdt: *mut sprd_mcdt_dev, res: *mut resource) {
    let mut i: c_int;

    i = 0;
    while i < MCDT_CHANNEL_NUM as c_int {
        let chan: *mut sprd_mcdt_chan = &mut (*mcdt).chan[i as usize];

        if i < MCDT_ADC_CHANNEL_NUM as c_int {
            (*chan).id = i as u8;
            (*chan).type_ = sprd_mcdt_channel_type::SPRD_MCDT_ADC_CHAN;
            (*chan).fifo_phys =
                (*res).start + MCDT_CH0_RXD as usize + i as usize * 4;
        } else {
            (*chan).id = (i - MCDT_ADC_CHANNEL_NUM as c_int) as u8;
            (*chan).type_ = sprd_mcdt_channel_type::SPRD_MCDT_DAC_CHAN;
            (*chan).fifo_phys = (*res).start
                + MCDT_CH0_TXD as usize
                + (i - MCDT_ADC_CHANNEL_NUM as c_int) as usize * 4;
        }

        (*chan).mcdt = mcdt;
        INIT_LIST_HEAD(&mut (*chan).list);

        // C source uses scoped_guard(mutex, &sprd_mcdt_list_mutex).
        list_add_tail(&mut (*chan).list, &raw mut sprd_mcdt_chan_list);

        i += 1;
    }
}

unsafe extern "C" fn sprd_mcdt_probe(pdev: *mut platform_device) -> c_int {
    let mcdt: *mut sprd_mcdt_dev;
    let mut res: *mut resource = core::ptr::null_mut();
    let ret: c_int;
    let irq: c_int;

    mcdt = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<sprd_mcdt_dev>(),
        GFP_KERNEL,
    ) as *mut sprd_mcdt_dev;
    if mcdt.is_null() {
        return -ENOMEM;
    }

    (*mcdt).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*mcdt).base) {
        return PTR_ERR((*mcdt).base);
    }

    (*mcdt).dev = &mut (*pdev).dev;
    spin_lock_init(&mut (*mcdt).lock);
    platform_set_drvdata(pdev, mcdt as *mut c_void);

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    ret = devm_request_irq(
        &mut (*pdev).dev,
        irq,
        sprd_mcdt_irq_handler,
        0,
        c"sprd-mcdt".as_ptr(),
        mcdt as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    sprd_mcdt_init_chans(mcdt, res);

    0
}

unsafe extern "C" fn sprd_mcdt_remove(pdev: *mut platform_device) {
    let mut chan: *mut sprd_mcdt_chan = core::ptr::null_mut();
    let mut temp: *mut sprd_mcdt_chan = core::ptr::null_mut();

    // C source uses guard(mutex)(&sprd_mcdt_list_mutex).
    // C source iterates list_for_each_entry_safe(chan, temp, &sprd_mcdt_chan_list, list).
    TODO_list_for_each_entry_safe_sprd_mcdt_chan(
        &mut chan,
        &mut temp,
        &raw mut sprd_mcdt_chan_list,
    );
    list_del(&mut (*chan).list);
}

static sprd_mcdt_of_match_compatible0: &[u8] = b"sprd,sc9860-mcdt\0";

static sprd_mcdt_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: sprd_mcdt_of_match_compatible0.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, sprd_mcdt_of_match);

static sprd_mcdt_driver_name: &[u8] = b"sprd-mcdt\0";

static mut sprd_mcdt_driver: platform_driver = platform_driver {
    probe: Some(sprd_mcdt_probe),
    remove: Some(sprd_mcdt_remove),
    driver: driver {
        name: sprd_mcdt_driver_name.as_ptr() as *const c_char,
        of_match_table: sprd_mcdt_of_match.as_ptr(),
    },
};

// module_platform_driver(sprd_mcdt_driver);

// MODULE_DESCRIPTION("Spreadtrum Multi-Channel Data Transfer Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
