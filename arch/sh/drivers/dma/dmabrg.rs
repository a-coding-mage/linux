// SPDX-License-Identifier: GPL-2.0
/*
 * SH7760 DMABRG IRQ handling
 *
 * (c) 2007 MSC Vertriebsges.m.b.H, Manuel Lauss <mlau@msc-ge.com>
 */

// Linux kernel and architecture dependencies supplied by other translation units.

const DMARSRA: usize = 0xfe090000;
const DMAOR: usize = 0xffa00040;
const DMACHCR0: usize = 0xffa0000c;
const DMABRGCR: usize = 0xfe3c0000;

const DMAOR_BRG: u32 = 0x0000c000;
const DMAOR_DMEN: u32 = 0x00000001;

const DMABRGI0: i32 = 68;
const DMABRGI1: i32 = 69;
const DMABRGI2: i32 = 70;

#[repr(C)]
struct DmabrgHandler {
    handler: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    data: *mut core::ffi::c_void,
}

static mut dmabrg_handlers: *mut DmabrgHandler = core::ptr::null_mut();

const DMABRGIRQ_USBDMA: usize = 0;
const DMABRGIRQ_USBDMAERR: usize = 1;
const DMABRGIRQ_A0TXF: usize = 2;

extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
    fn __ffs(value: u32) -> u32;
    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
        flags: u32,
        name: *const u8,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn free_irq(irq: i32, data: *mut core::ffi::c_void);
    #[cfg(CONFIG_SH_DMA)]
    fn request_dma(channel: u32, name: *const u8) -> i32;
}

unsafe fn dmabrg_call_handler(i: usize) {
    let entry = &*dmabrg_handlers.add(i);
    if let Some(handler) = entry.handler {
        handler(entry.data);
    }
}

/*
 * main DMABRG irq handler. It acks irqs and then
 * handles every set and unmasked bit sequentially.
 * No locking and no validity checks; it should be
 * as fast as possible (audio!)
 */
unsafe extern "C" fn dmabrg_irq(_irq: i32, _data: *mut core::ffi::c_void) -> i32 {
    let mut dcr: u32;
    let mut i: u32;

    dcr = __raw_readl(DMABRGCR);
    __raw_writel(dcr & !0x00ff0003, DMABRGCR); // ack all
    dcr &= dcr >> 8; // ignore masked

    // USB stuff, get it out of the way first
    if dcr & 1 != 0 {
        dmabrg_call_handler(DMABRGIRQ_USBDMA);
    }
    if dcr & 2 != 0 {
        dmabrg_call_handler(DMABRGIRQ_USBDMAERR);
    }

    // Audio
    dcr >>= 16;
    while dcr != 0 {
        i = __ffs(dcr);
        dcr &= dcr.wrapping_sub(1);
        dmabrg_call_handler((i + DMABRGIRQ_A0TXF as u32) as usize);
    }
    1 // IRQ_HANDLED
}

unsafe fn dmabrg_disable_irq(dmairq: u32) {
    let mut dcr = __raw_readl(DMABRGCR);
    let shift = if dmairq > 1 { dmairq + 22 } else { dmairq + 8 };
    dcr &= !(1u32 << shift);
    __raw_writel(dcr, DMABRGCR);
}

unsafe fn dmabrg_enable_irq(dmairq: u32) {
    let mut dcr = __raw_readl(DMABRGCR);
    let shift = if dmairq > 1 { dmairq + 22 } else { dmairq + 8 };
    dcr |= 1u32 << shift;
    __raw_writel(dcr, DMABRGCR);
}

pub unsafe extern "C" fn dmabrg_request_irq(
    dmairq: u32,
    handler: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    data: *mut core::ffi::c_void,
) -> i32 {
    if dmairq > 9 || handler.is_none() {
        return -2; // -ENOENT
    }
    if (*dmabrg_handlers.add(dmairq as usize)).handler.is_some() {
        return -16; // -EBUSY
    }

    (*dmabrg_handlers.add(dmairq as usize)).handler = handler;
    (*dmabrg_handlers.add(dmairq as usize)).data = data;

    dmabrg_enable_irq(dmairq);
    0
}

pub unsafe extern "C" fn dmabrg_free_irq(dmairq: u32) {
    if dmairq < 10 {
        dmabrg_disable_irq(dmairq);
        (*dmabrg_handlers.add(dmairq as usize)).handler = None;
        (*dmabrg_handlers.add(dmairq as usize)).data = core::ptr::null_mut();
    }
}

unsafe extern "C" fn dmabrg_init() -> i32 {
    let mut or: u32;
    let mut ret: i32;

    dmabrg_handlers = kzalloc_objs::<DmabrgHandler>(10);
    if dmabrg_handlers.is_null() {
        return -12; // -ENOMEM
    }

    #[cfg(CONFIG_SH_DMA)]
    {
        // request DMAC channel 0 before anyone else can get it
        ret = request_dma(0, b"DMAC 0 (DMABRG)\0".as_ptr());
        if ret < 0 {
            // printk(KERN_INFO "DMABRG: DMAC ch0 not reserved!\n");
        }
    }

    __raw_writel(0, DMABRGCR);
    __raw_writel(0, DMACHCR0);
    __raw_writel(0x94000000, DMARSRA); // enable DMABRG in DMAC 0

    // enable DMABRG mode, enable the DMAC
    or = __raw_readl(DMAOR);
    __raw_writel(or | DMAOR_BRG | DMAOR_DMEN, DMAOR);

    ret = request_irq(DMABRGI0, dmabrg_irq, 0, b"DMABRG USB address error\0".as_ptr(), core::ptr::null_mut());
    if ret != 0 {
        kfree(dmabrg_handlers.cast());
        return ret;
    }

    ret = request_irq(DMABRGI1, dmabrg_irq, 0, b"DMABRG Transfer End\0".as_ptr(), core::ptr::null_mut());
    if ret != 0 {
        free_irq(DMABRGI0, core::ptr::null_mut());
        kfree(dmabrg_handlers.cast());
        return ret;
    }

    ret = request_irq(DMABRGI2, dmabrg_irq, 0, b"DMABRG Transfer Half\0".as_ptr(), core::ptr::null_mut());
    if ret == 0 {
        return ret;
    }

    free_irq(DMABRGI1, core::ptr::null_mut());
    free_irq(DMABRGI0, core::ptr::null_mut());
    kfree(dmabrg_handlers.cast());
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
