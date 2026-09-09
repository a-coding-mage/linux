// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *   Copyright (C) 2011 John Crispin <john@phrozen.org>
 */

// External Linux/Lantiq dependencies supplied by other translation units.

const LTQ_DMA_ID: usize = 0x08;
const LTQ_DMA_CTRL: usize = 0x10;
const LTQ_DMA_CPOLL: usize = 0x14;
const LTQ_DMA_CS: usize = 0x18;
const LTQ_DMA_CCTRL: usize = 0x1c;
const LTQ_DMA_CDBA: usize = 0x20;
const LTQ_DMA_CDLEN: usize = 0x24;
const LTQ_DMA_CIS: usize = 0x28;
const LTQ_DMA_CIE: usize = 0x2c;
const LTQ_DMA_PS: usize = 0x40;
const LTQ_DMA_PCTRL: usize = 0x44;
const LTQ_DMA_IRNEN: usize = 0xf4;

const DMA_ID_CHNR: u32 = 0x07f0_0000;
const DMA_DESCPT: u32 = 1 << 3;
const DMA_TX: u32 = 1 << 8;
const DMA_CHAN_ON: u32 = 1 << 0;
const DMA_PDEN: u32 = 1 << 6;
const DMA_CHAN_RST: u32 = 1 << 1;
const DMA_RESET: u32 = 1 << 0;
const DMA_IRQ_ACK: u32 = 0x7e;
const DMA_POLL: u32 = 1 << 31;
const DMA_CLK_DIV4: u32 = 1 << 6;
const DMA_PCTRL_2W_BURST: u32 = 0x1;
const DMA_PCTRL_4W_BURST: u32 = 0x2;
const DMA_PCTRL_8W_BURST: u32 = 0x3;
const DMA_TX_BURST_SHIFT: u32 = 4;
const DMA_RX_BURST_SHIFT: u32 = 2;
const DMA_ETOP_ENDIANNESS: u32 = 0xf << 8;
const DMA_WEIGHT: u32 = (1 << 17) | (1 << 16);

extern "C" {
    static mut ltq_dma_membase: *mut core::ffi::c_void;
    static mut ltq_dma_lock: core::ffi::c_void;
    fn ltq_r32(addr: *mut core::ffi::c_void) -> u32;
    fn ltq_w32(value: u32, addr: *mut core::ffi::c_void);
    fn ltq_w32_mask(clear: u32, set: u32, addr: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
    fn dma_alloc_coherent(dev: *mut core::ffi::c_void, size: usize,
                          phys: *mut u64, flags: u32) -> *mut core::ffi::c_void;
    fn dma_free_coherent(dev: *mut core::ffi::c_void, size: usize,
                         addr: *mut core::ffi::c_void, phys: u64);
    fn wmb();
    fn clk_get(dev: *mut core::ffi::c_void, id: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn clk_enable(clk: *mut core::ffi::c_void) -> i32;
    fn usleep_range(min: u32, max: u32);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: u32,
                                              res: *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn panic(message: *const core::ffi::c_char) -> !;
    fn dev_info(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct ltq_dma_channel {
    pub nr: u32,
    pub dev: *mut core::ffi::c_void,
    pub desc: u32,
    pub desc_base: *mut core::ffi::c_void,
    pub phys: u64,
}

#[repr(C)]
pub struct platform_device { pub dev: core::ffi::c_void }
#[repr(C)]
pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, pub driver: core::ffi::c_void }

const LTQ_DESC_NUM: usize = 0;
const LTQ_DESC_SIZE: usize = 0;
const DMA_PORT_ETOP: i32 = 0;
const GFP_ATOMIC: u32 = 0;

unsafe fn dma_r32(x: usize) -> u32 { ltq_r32(ltq_dma_membase.add(x)) }
unsafe fn dma_w32(x: u32, y: usize) { ltq_w32(x, ltq_dma_membase.add(y)); }
unsafe fn dma_w32_mask(x: u32, y: u32, z: usize) { ltq_w32_mask(x, y, ltq_dma_membase.add(z)); }

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_enable_irq(ch: *mut ltq_dma_channel) {
    let mut flags = 0; spin_lock_irqsave(&mut ltq_dma_lock, &mut flags);
    dma_w32((*ch).nr, LTQ_DMA_CS); dma_w32_mask(0, 1 << (*ch).nr, LTQ_DMA_IRNEN);
    spin_unlock_irqrestore(&mut ltq_dma_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_disable_irq(ch: *mut ltq_dma_channel) {
    let mut flags = 0; spin_lock_irqsave(&mut ltq_dma_lock, &mut flags);
    dma_w32((*ch).nr, LTQ_DMA_CS); dma_w32_mask(1 << (*ch).nr, 0, LTQ_DMA_IRNEN);
    spin_unlock_irqrestore(&mut ltq_dma_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_ack_irq(ch: *mut ltq_dma_channel) {
    let mut flags = 0; spin_lock_irqsave(&mut ltq_dma_lock, &mut flags);
    dma_w32((*ch).nr, LTQ_DMA_CS); dma_w32(DMA_IRQ_ACK, LTQ_DMA_CIS);
    spin_unlock_irqrestore(&mut ltq_dma_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_open(ch: *mut ltq_dma_channel) {
    let mut flag = 0; spin_lock_irqsave(&mut ltq_dma_lock, &mut flag);
    dma_w32((*ch).nr, LTQ_DMA_CS); dma_w32_mask(0, DMA_CHAN_ON, LTQ_DMA_CCTRL);
    spin_unlock_irqrestore(&mut ltq_dma_lock, flag);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_close(ch: *mut ltq_dma_channel) {
    let mut flag = 0; spin_lock_irqsave(&mut ltq_dma_lock, &mut flag);
    dma_w32((*ch).nr, LTQ_DMA_CS); dma_w32_mask(DMA_CHAN_ON, 0, LTQ_DMA_CCTRL);
    dma_w32_mask(1 << (*ch).nr, 0, LTQ_DMA_IRNEN);
    spin_unlock_irqrestore(&mut ltq_dma_lock, flag);
}

unsafe fn ltq_dma_alloc(ch: *mut ltq_dma_channel) {
    let mut flags = 0; (*ch).desc = 0;
    (*ch).desc_base = dma_alloc_coherent((*ch).dev, LTQ_DESC_NUM * LTQ_DESC_SIZE, &mut (*ch).phys, GFP_ATOMIC);
    spin_lock_irqsave(&mut ltq_dma_lock, &mut flags);
    dma_w32((*ch).nr, LTQ_DMA_CS); dma_w32((*ch).phys as u32, LTQ_DMA_CDBA);
    dma_w32(LTQ_DESC_NUM as u32, LTQ_DMA_CDLEN); dma_w32_mask(DMA_CHAN_ON, 0, LTQ_DMA_CCTRL);
    wmb(); dma_w32_mask(0, DMA_CHAN_RST, LTQ_DMA_CCTRL);
    while dma_r32(LTQ_DMA_CCTRL) & DMA_CHAN_RST != 0 {}
    spin_unlock_irqrestore(&mut ltq_dma_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_alloc_tx(ch: *mut ltq_dma_channel) {
    ltq_dma_alloc(ch); let mut flags = 0; spin_lock_irqsave(&mut ltq_dma_lock, &mut flags);
    dma_w32(DMA_DESCPT, LTQ_DMA_CIE); dma_w32_mask(0, 1 << (*ch).nr, LTQ_DMA_IRNEN);
    dma_w32(DMA_WEIGHT | DMA_TX, LTQ_DMA_CCTRL); spin_unlock_irqrestore(&mut ltq_dma_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_alloc_rx(ch: *mut ltq_dma_channel) {
    ltq_dma_alloc(ch); let mut flags = 0; spin_lock_irqsave(&mut ltq_dma_lock, &mut flags);
    dma_w32(DMA_DESCPT, LTQ_DMA_CIE); dma_w32_mask(0, 1 << (*ch).nr, LTQ_DMA_IRNEN);
    dma_w32(DMA_WEIGHT, LTQ_DMA_CCTRL); spin_unlock_irqrestore(&mut ltq_dma_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_free(ch: *mut ltq_dma_channel) {
    if (*ch).desc_base.is_null() { return; }
    ltq_dma_close(ch); dma_free_coherent((*ch).dev, LTQ_DESC_NUM * LTQ_DESC_SIZE, (*ch).desc_base, (*ch).phys);
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_init_port(p: i32, tx_burst: i32, rx_burst: i32) {
    dma_w32(p as u32, LTQ_DMA_PS);
    if p == DMA_PORT_ETOP { dma_w32_mask(0, DMA_ETOP_ENDIANNESS | DMA_PDEN, LTQ_DMA_PCTRL); }
    match rx_burst { 8 => dma_w32_mask(0x0c, DMA_PCTRL_8W_BURST << DMA_RX_BURST_SHIFT, LTQ_DMA_PCTRL), 4 => dma_w32_mask(0x0c, DMA_PCTRL_4W_BURST << DMA_RX_BURST_SHIFT, LTQ_DMA_PCTRL), 2 => dma_w32_mask(0x0c, DMA_PCTRL_2W_BURST << DMA_RX_BURST_SHIFT, LTQ_DMA_PCTRL), _ => {} }
    match tx_burst { 8 => dma_w32_mask(0x30, DMA_PCTRL_8W_BURST << DMA_TX_BURST_SHIFT, LTQ_DMA_PCTRL), 4 => dma_w32_mask(0x30, DMA_PCTRL_4W_BURST << DMA_TX_BURST_SHIFT, LTQ_DMA_PCTRL), 2 => dma_w32_mask(0x30, DMA_PCTRL_2W_BURST << DMA_TX_BURST_SHIFT, LTQ_DMA_PCTRL), _ => {} }
}

#[no_mangle]
pub unsafe extern "C" fn ltq_dma_init(pdev: *mut platform_device) -> i32 {
    let mut clk: *mut core::ffi::c_void;
    let mut id: u32;
    let mut nchannels: u32;
    let mut i: i32;

    ltq_dma_membase = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if ltq_dma_membase as isize == -1 { panic(b"Failed to remap dma resource\0".as_ptr() as *const _); }

    clk = clk_get(&mut (*pdev).dev, core::ptr::null());
    if clk as isize == -1 { panic(b"Failed to get dma clock\0".as_ptr() as *const _); }
    clk_enable(clk);
    dma_w32_mask(0, DMA_RESET, LTQ_DMA_CTRL);
    usleep_range(1, 10);
    dma_w32(0, LTQ_DMA_IRNEN);

    id = dma_r32(LTQ_DMA_ID);
    nchannels = (id & DMA_ID_CHNR) >> 20;
    i = 0;
    while i < nchannels as i32 {
        dma_w32(i as u32, LTQ_DMA_CS);
        dma_w32(DMA_CHAN_RST, LTQ_DMA_CCTRL);
        dma_w32(DMA_POLL | DMA_CLK_DIV4, LTQ_DMA_CPOLL);
        dma_w32_mask(DMA_CHAN_ON, 0, LTQ_DMA_CCTRL);
        i += 1;
    }
    dev_info(&mut (*pdev).dev, b"Init done - hw rev: %X, ports: %d, channels: %d\n\0".as_ptr() as *const _, id & 0x1f, (id >> 16) & 0xf, nchannels);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dma_init() -> i32 {
    static mut DMA_DRIVER: platform_driver = platform_driver { probe: Some(ltq_dma_init), driver: core::ffi::c_void {} };
    platform_driver_register(&mut DMA_DRIVER)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
