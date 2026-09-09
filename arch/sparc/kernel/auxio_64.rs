// SPDX-License-Identifier: GPL-2.0
/* auxio.c: Probing for the Sparc AUXIO register at boot time.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 *
 * Refactoring for unified NCR/PCIO support 2002 Eric Brower (ebrower@usa.net)
 */

// Linux kernel and SPARC declarations are supplied by the surrounding build.

pub static mut auxio_register: *mut core::ffi::c_void = core::ptr::null_mut();

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum auxio_type {
    AUXIO_TYPE_NODEV = 0,
    AUXIO_TYPE_SBUS,
    AUXIO_TYPE_EBUS,
}

static mut auxio_devtype: auxio_type = auxio_type::AUXIO_TYPE_NODEV;
static mut auxio_lock: spinlock_t = unsafe { core::mem::zeroed() };

unsafe fn __auxio_rmw(bits_on: u8, bits_off: u8, ebus: i32) {
    if !auxio_register.is_null() {
        let mut flags: c_ulong;
        let regval: u8;
        let mut newval: u8;

        spin_lock_irqsave(&raw mut auxio_lock, &mut flags);

        regval = if ebus != 0 {
            readl(auxio_register) as u8
        } else {
            sbus_readb(auxio_register)
        };
        newval = regval | bits_on;
        newval &= !bits_off;
        if ebus == 0 {
            newval &= !AUXIO_AUX1_MASK;
        }
        if ebus != 0 {
            writel(newval as u32, auxio_register);
        } else {
            sbus_writeb(newval, auxio_register);
        }

        spin_unlock_irqrestore(&raw mut auxio_lock, flags);
    }
}

unsafe fn __auxio_set_bit(_bit: u8, on: i32, ebus: i32) {
    let mut bits_on: u8 = if ebus != 0 { AUXIO_PCIO_LED } else { AUXIO_AUX1_LED };
    let mut bits_off: u8 = 0;

    if on == 0 {
        let tmp: u8 = bits_off;
        bits_off = bits_on;
        bits_on = tmp;
    }
    __auxio_rmw(bits_on, bits_off, ebus);
}

#[no_mangle]
pub unsafe extern "C" fn auxio_set_led(on: i32) {
    let ebus: i32 = if auxio_devtype == auxio_type::AUXIO_TYPE_EBUS { 1 } else { 0 };
    let bit: u8;

    bit = if ebus != 0 { AUXIO_PCIO_LED } else { AUXIO_AUX1_LED };
    __auxio_set_bit(bit, on, ebus);
}

unsafe fn __auxio_sbus_set_lte(on: i32) {
    __auxio_set_bit(AUXIO_AUX1_LTE, on, 0);
}

#[no_mangle]
pub unsafe extern "C" fn auxio_set_lte(on: i32) {
    match auxio_devtype {
        auxio_type::AUXIO_TYPE_SBUS => __auxio_sbus_set_lte(on),
        auxio_type::AUXIO_TYPE_EBUS | auxio_type::AUXIO_TYPE_NODEV => {}
    }
}

static auxio_match: [of_device_id; 2] = [
    of_device_id { name: "auxio" },
    of_device_id { name: core::ptr::null() },
];

unsafe fn auxio_probe(dev: *mut platform_device) -> i32 {
    let dp = (*dev).dev.of_node;
    let size: usize;

    if of_node_name_eq((*dp).parent, "ebus") {
        auxio_devtype = auxio_type::AUXIO_TYPE_EBUS;
        size = core::mem::size_of::<u32>();
    } else if of_node_name_eq((*dp).parent, "sbus") {
        auxio_devtype = auxio_type::AUXIO_TYPE_SBUS;
        size = 1;
    } else {
        printk!("auxio: Unknown parent bus type [%pOFn]\n", (*dp).parent);
        return -ENODEV;
    }
    auxio_register = of_ioremap(&(*dev).resource[0], 0, size, "auxio");
    if auxio_register.is_null() {
        return -ENODEV;
    }

    printk!(KERN_INFO "AUXIO: Found device at %pOF\n", dp);

    if auxio_devtype == auxio_type::AUXIO_TYPE_EBUS {
        auxio_set_led(AUXIO_LED_ON);
    }

    0
}

static mut auxio_driver: platform_driver = platform_driver {
    probe: Some(auxio_probe),
    driver: driver {
        name: "auxio",
        of_match_table: auxio_match.as_ptr(),
    },
};

unsafe fn auxio_init() -> i32 {
    platform_driver_register(&raw mut auxio_driver)
}

/* Must be after subsys_initcall() so that busses are probed.  Must
 * be before device_initcall() because things like the floppy driver
 * need to use the AUXIO register.
 */
fs_initcall!(auxio_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
