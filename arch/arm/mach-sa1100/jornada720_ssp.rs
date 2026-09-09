// SPDX-License-Identifier: GPL-2.0-only
/*
 *  arch/arm/mach-sa1100/jornada720_ssp.c
 *
 *  Copyright (C) 2006/2007 Kristoffer Ericson <Kristoffer.Ericson@gmail.com>
 *   Copyright (C) 2006 Filip Zyzniewski <filip.zyzniewski@tefnet.pl>
 *
 *  SSP driver for the HP Jornada 710/720/728
 */

// External kernel, machine, and SSP declarations supplied by other files.

static mut jornada_ssp_lock: spinlock_t = spinlock_t { _private: [] };
static mut jornada_ssp_flags: c_ulong = 0;

#[inline]
pub fn jornada_ssp_reverse(byte: u8) -> u8 {
    ((0x80 & byte) >> 7)
        | ((0x40 & byte) >> 5)
        | ((0x20 & byte) >> 3)
        | ((0x10 & byte) >> 1)
        | ((0x08 & byte) << 1)
        | ((0x04 & byte) << 3)
        | ((0x02 & byte) << 5)
        | ((0x01 & byte) << 7)
}

#[no_mangle]
pub unsafe extern "C" fn jornada_ssp_byte(byte: u8) -> c_int {
    let mut timeout: c_int = 400000;
    let mut ret: u16;

    while (GPLR & GPIO_GPIO10) != 0 {
        timeout -= 1;
        if timeout == 0 {
            printk(KERN_WARNING.as_ptr(), b"SSP: timeout while waiting for transmit\0".as_ptr());
            return -ETIMEDOUT;
        }
        cpu_relax();
    }

    ret = (jornada_ssp_reverse(byte) as u16) << 8;
    ssp_write_word(ret);
    ssp_read_word(&mut ret as *mut u16);
    jornada_ssp_reverse(ret as u8) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn jornada_ssp_inout(byte: u8) -> c_int {
    let ret: c_int;
    let mut i: c_int;

    /* true means command byte */
    if byte != TXDUMMY {
        ret = jornada_ssp_byte(byte);
        /* Proper return to commands is TxDummy */
        if ret != TXDUMMY as c_int {
            i = 0;
            while i < 256 {
                /* flushing bus */
                if jornada_ssp_byte(TXDUMMY) == -1 {
                    break;
                }
                i += 1;
            }
            return -ETIMEDOUT;
        }
    } else {
        /* Exchange TxDummy for data */
        ret = jornada_ssp_byte(TXDUMMY);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn jornada_ssp_start() {
    spin_lock_irqsave(&mut jornada_ssp_lock, &mut jornada_ssp_flags);
    GPCR = GPIO_GPIO25;
    udelay(50);
}

#[no_mangle]
pub unsafe extern "C" fn jornada_ssp_end() {
    GPSR = GPIO_GPIO25;
    spin_unlock_irqrestore(&mut jornada_ssp_lock, jornada_ssp_flags);
}

unsafe fn jornada_ssp_probe(_dev: *mut platform_device) -> c_int {
    let mut ret: c_int;

    GPSR = GPIO_GPIO25;
    ret = ssp_init();

    /* worked fine, lets not bother with anything else */
    if ret == 0 {
        printk(KERN_INFO.as_ptr(), b"SSP: device initialized with irq\0".as_ptr());
        return ret;
    }

    printk(KERN_WARNING.as_ptr(), b"SSP: initialization failed, trying non-irq solution \0".as_ptr());

    /* init of Serial 4 port */
    Ser4MCCR0 = 0;
    Ser4SSCR0 = 0x0387;
    Ser4SSCR1 = 0x18;

    /* clear out any left over data */
    ssp_flush();

    /* enable MCU */
    jornada_ssp_start();

    /* see if return value makes sense */
    ret = jornada_ssp_inout(GETBRIGHTNESS);

    /* seems like it worked, just feed it with TxDummy to get rid of data */
    if ret == TXDUMMY as c_int {
        jornada_ssp_inout(TXDUMMY);
    }

    jornada_ssp_end();

    /* failed, lets just kill everything */
    if ret == -ETIMEDOUT {
        printk(KERN_WARNING.as_ptr(), b"SSP: attempts failed, bailing\0".as_ptr());
        ssp_exit();
        return -ENODEV;
    }

    /* all fine */
    printk(KERN_INFO.as_ptr(), b"SSP: device initialized\0".as_ptr());
    0
}

unsafe fn jornada_ssp_remove(_dev: *mut platform_device) {
    /* Note that this doesn't actually remove the driver, since theres nothing to remove
     * It just makes sure everything is turned off */
    GPSR = GPIO_GPIO25;
    ssp_exit();
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const u8,
}

#[repr(C)]
pub struct platform_device;

#[repr(C)]
pub struct spinlock_t {
    pub _private: [u8; 0],
}

pub static mut jornadassp_driver: platform_driver = platform_driver {
    probe: Some(jornada_ssp_probe),
    remove: Some(jornada_ssp_remove),
    driver: driver { name: b"jornada_ssp\0".as_ptr() },
};

unsafe fn jornada_ssp_init() -> c_int {
    platform_driver_register(&mut jornadassp_driver)
}

// module_init(jornada_ssp_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
