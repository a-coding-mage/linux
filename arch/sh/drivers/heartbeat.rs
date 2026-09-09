// SPDX-License-Identifier: GPL-2.0
/*
 * Generic heartbeat driver for regular LED banks
 *
 * Copyright (C) 2007 - 2010  Paul Mundt
 *
 * Most SH reference boards include a number of individual LEDs that can
 * be independently controlled (either via a pre-defined hardware
 * function or via the LED class, if desired -- the hardware tends to
 * encapsulate some of the same "triggers" that the LED class supports,
 * so there's not too much value in it).
 *
 * Additionally, most of these boards also have a LED bank that we've
 * traditionally used for strobing the load average. This use case is
 * handled by this driver, rather than giving each LED bit position its
 * own struct device.
 */

// C dependencies: linux/init.h, linux/platform_device.h, linux/sched.h,
// linux/sched/loadavg.h, linux/timer.h, linux/io.h, linux/slab.h, and
// asm/heartbeat.h.

const DRV_NAME: &[u8] = b"heartbeat\0";
const DRV_VERSION: &[u8] = b"0.1.2\0";

static mut DEFAULT_BIT_POS: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct heartbeat_data {
    pub base: *mut core::ffi::c_void,
    pub bit_pos: *mut u8,
    pub nr_bits: u32,
    pub mask: u32,
    pub regsize: u32,
    pub flags: u32,
    pub timer: timer_list,
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub num_resources: u32,
    pub dev: device,
}

const HEARTBEAT_INVERTED: u32 = 1; // Supplied by asm/heartbeat.h.
const IORESOURCE_MEM: u32 = 0;
const IORESOURCE_MEM_TYPE_MASK: usize = 0x0000_01f0;
const IORESOURCE_MEM_32BIT: usize = 0x0000_0100;
const IORESOURCE_MEM_16BIT: usize = 0x0000_0200;
const IORESOURCE_MEM_8BIT: usize = 0x0000_0400;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENXIO: i32 = 6;
const FSHIFT: u32 = 11;

extern "C" {
    static mut jiffies: usize;
    static mut avenrun: [u64; 3];
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn ioread16(addr: *mut core::ffi::c_void) -> u16;
    fn iowrite16(value: u16, addr: *mut core::ffi::c_void);
    fn ioread8(addr: *mut core::ffi::c_void) -> u8;
    fn iowrite8(value: u8, addr: *mut core::ffi::c_void);
    fn ioremap(start: usize, size: usize) -> *mut core::ffi::c_void;
    fn resource_size(res: *const resource) -> usize;
    fn platform_get_resource(pdev: *mut platform_device, ty: u32, index: u32) -> *mut resource;
    fn mod_timer(timer: *mut timer_list, expires: usize) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut heartbeat_data);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn printk(format: *const u8, ...);
    fn dev_err(dev: *mut device, format: *const u8, ...);
    fn kzalloc_heartbeat_data() -> *mut heartbeat_data;
    fn kfree(ptr: *mut heartbeat_data);
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: u32);
    fn timer_container_of(timer: *mut timer_list) -> *mut heartbeat_data;
}

#[repr(C)]
pub struct driver {
    pub name: *const u8,
    pub suppress_bind_attrs: bool,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: driver,
}

unsafe fn heartbeat_toggle_bit(hd: *mut heartbeat_data, bit: u32, inverted: u32) {
    let mut new: u32 = 1u32.wrapping_shl((*hd).bit_pos.add(bit as usize).read() as u32);
    if inverted != 0 {
        new = !new;
    }
    new &= (*hd).mask;

    match (*hd).regsize {
        32 => {
            new |= ioread32((*hd).base) & !(*hd).mask;
            iowrite32(new, (*hd).base);
        }
        16 => {
            new |= ioread16((*hd).base) as u32 & !(*hd).mask;
            iowrite16(new as u16, (*hd).base);
        }
        _ => {
            new |= ioread8((*hd).base) as u32 & !(*hd).mask;
            iowrite8(new as u8, (*hd).base);
        }
    }
}

unsafe extern "C" fn heartbeat_timer(t: *mut timer_list) {
    let hd = timer_container_of(t);
    static mut BIT: u32 = 0;
    static mut UP: i32 = 1;

    heartbeat_toggle_bit(hd, BIT, (*hd).flags & HEARTBEAT_INVERTED);

    BIT = BIT.wrapping_add(UP as u32);
    if BIT == 0 || BIT == (*hd).nr_bits - 1 {
        UP = -UP;
    }

    mod_timer(&mut (*hd).timer, jiffies + (110 - ((300 << FSHIFT) / ((avenrun[0] / 5) + (3 << FSHIFT))) as usize));
}

unsafe extern "C" fn heartbeat_drv_probe(pdev: *mut platform_device) -> i32 {
    let res: *mut resource;
    let hd: *mut heartbeat_data;
    let mut i: u32;

    if (*pdev).num_resources != 1 {
        dev_err(&mut (*pdev).dev, b"invalid number of resources\n\0".as_ptr());
        return -EINVAL;
    }
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, b"invalid resource\n\0".as_ptr());
        return -EINVAL;
    }
    if !(*pdev).dev.platform_data.is_null() {
        hd = (*pdev).dev.platform_data as *mut heartbeat_data;
    } else {
        hd = kzalloc_heartbeat_data();
        if hd.is_null() { return -ENOMEM; }
    }
    (*hd).base = ioremap((*res).start, resource_size(res));
    if (*hd).base.is_null() {
        dev_err(&mut (*pdev).dev, b"ioremap failed\n\0".as_ptr());
        if (*pdev).dev.platform_data.is_null() { kfree(hd); }
        return -ENXIO;
    }
    if (*hd).nr_bits == 0 {
        (*hd).bit_pos = DEFAULT_BIT_POS.as_mut_ptr();
        (*hd).nr_bits = DEFAULT_BIT_POS.len() as u32;
    }
    (*hd).mask = 0;
    i = 0;
    while i < (*hd).nr_bits {
        (*hd).mask |= 1u32.wrapping_shl((*hd).bit_pos.add(i as usize).read() as u32);
        i += 1;
    }
    if (*hd).regsize == 0 {
        (*hd).regsize = match (*res).flags & IORESOURCE_MEM_TYPE_MASK {
            IORESOURCE_MEM_32BIT => 32,
            IORESOURCE_MEM_16BIT => 16,
            _ => 8,
        };
    }
    timer_setup(&mut (*hd).timer, heartbeat_timer, 0);
    platform_set_drvdata(pdev, hd);
    mod_timer(&mut (*hd).timer, jiffies + 1)
}

static mut HEARTBEAT_DRIVER: platform_driver = platform_driver {
    probe: Some(heartbeat_drv_probe),
    driver: driver { name: DRV_NAME.as_ptr(), suppress_bind_attrs: true },
};

unsafe extern "C" fn heartbeat_init() -> i32 {
    printk(b"heartbeat: version %s loaded\n\0".as_ptr(), DRV_VERSION.as_ptr());
    platform_driver_register(&mut HEARTBEAT_DRIVER)
}

// device_initcall(heartbeat_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
