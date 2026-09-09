/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 * Copyright (C) 2012 Kevin Cernekee <cernekee@gmail.com>
 * Copyright (C) 2012 Broadcom Corporation
 */

// The declarations below are supplied by the kernel and BCM63xx headers.

use core::ffi::c_void;

const NUM_MMIO: usize = 2;
const NUM_IRQ: usize = 7;

#[repr(C)]
pub struct Resource {
    pub start: u64,
    pub end: u64,
    pub name: *const i8,
    pub flags: u64,
    pub desc: u64,
    pub parent: *mut Resource,
    pub sibling: *mut Resource,
    pub child: *mut Resource,
}

#[repr(C)]
pub struct Device {
    pub dma_mask: *mut u64,
    pub coherent_dma_mask: u64,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const i8,
    pub id: i32,
    pub num_resources: u32,
    pub resource: *mut Resource,
    pub dev: Device,
}

#[repr(C)]
pub struct Bcm63xxUsbdPlatformData {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bcm63xx_regset_address(regset: i32) -> u64;
    fn bcm63xx_get_irq_number(irq: i32) -> u64;
    fn platform_device_add_data(
        device: *mut PlatformDevice,
        data: *const c_void,
        size: usize,
    ) -> i32;
    fn platform_device_register(device: *mut PlatformDevice) -> i32;
    fn bcmcpu_is_6328() -> bool;
    fn bcmcpu_is_6368() -> bool;
}

unsafe extern "C" {
    static IRQ_USBD: i32;
    static IRQ_USBD_RXDMA0: i32;
    static IRQ_USBD_TXDMA0: i32;
    static IRQ_USBD_RXDMA1: i32;
    static IRQ_USBD_TXDMA1: i32;
    static IRQ_USBD_RXDMA2: i32;
    static IRQ_USBD_TXDMA2: i32;
    static RSET_USBD: i32;
    static RSET_USBDMA: i32;
    static RSET_USBD_SIZE: u64;
    static RSET_USBDMA_SIZE: u64;
    static IORESOURCE_MEM: u64;
    static IORESOURCE_IRQ: u64;
}

static mut USBD_RESOURCES: [Resource; NUM_MMIO + NUM_IRQ] = [
    Resource {
        start: 0,
        end: 0,
        name: core::ptr::null(),
        flags: 0,
        desc: 0,
        parent: core::ptr::null_mut(),
        sibling: core::ptr::null_mut(),
        child: core::ptr::null_mut(),
    };
    NUM_MMIO + NUM_IRQ
];

static mut USBD_DMAMASK: u64 = 0xffff_ffff;

static mut BCM63XX_USBD_DEVICE: PlatformDevice = PlatformDevice {
    name: b"bcm63xx_udc\0".as_ptr() as *const i8,
    id: -1,
    num_resources: (NUM_MMIO + NUM_IRQ) as u32,
    resource: core::ptr::addr_of_mut!(USBD_RESOURCES) as *mut Resource,
    dev: Device {
        dma_mask: core::ptr::addr_of_mut!(USBD_DMAMASK),
        coherent_dma_mask: 0xffff_ffff,
    },
};

pub unsafe fn bcm63xx_usbd_register(pd: *const Bcm63xxUsbdPlatformData) -> i32 {
    let irq_list: [i32; NUM_IRQ] = [
        IRQ_USBD,
        IRQ_USBD_RXDMA0,
        IRQ_USBD_TXDMA0,
        IRQ_USBD_RXDMA1,
        IRQ_USBD_TXDMA1,
        IRQ_USBD_RXDMA2,
        IRQ_USBD_TXDMA2,
    ];
    let mut i: usize;

    if !bcmcpu_is_6328() && !bcmcpu_is_6368() {
        return 0;
    }

    USBD_RESOURCES[0].start = bcm63xx_regset_address(RSET_USBD);
    USBD_RESOURCES[0].end = USBD_RESOURCES[0].start + RSET_USBD_SIZE - 1;
    USBD_RESOURCES[0].flags = IORESOURCE_MEM;

    USBD_RESOURCES[1].start = bcm63xx_regset_address(RSET_USBDMA);
    USBD_RESOURCES[1].end = USBD_RESOURCES[1].start + RSET_USBDMA_SIZE - 1;
    USBD_RESOURCES[1].flags = IORESOURCE_MEM;

    i = 0;
    while i < NUM_IRQ {
        let r: *mut Resource = &mut USBD_RESOURCES[NUM_MMIO + i];
        (*r).start = bcm63xx_get_irq_number(irq_list[i]);
        (*r).end = (*r).start;
        (*r).flags = IORESOURCE_IRQ;
        i += 1;
    }

    platform_device_add_data(
        core::ptr::addr_of_mut!(BCM63XX_USBD_DEVICE),
        pd as *const c_void,
        core::mem::size_of::<Bcm63xxUsbdPlatformData>(),
    );

    platform_device_register(core::ptr::addr_of_mut!(BCM63XX_USBD_DEVICE))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
