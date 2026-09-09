// SPDX-License-Identifier: GPL-2.0
/* apc - Driver implementation for power management functions
 * of Aurora Personality Chip (APC) on SPARCstation-4/5 and
 * derivatives.
 *
 * Copyright (c) 2002 Eric Brower (ebrower@usa.net)
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

const APC_OBPNAME: &str = "power-management";
const APC_DEVNAME: &str = "apc";

static mut regs: *mut u8 = core::ptr::null_mut();
static mut apc_no_idle: i32 = 0;

// Specify "apc=noidle" on the kernel command line to disable APC CPU standby
// support. Certain prototype systems do not play well with APC CPU idle.
unsafe fn apc_setup(str_: *const core::ffi::c_char) -> i32 {
    if strncmp(str_, b"noidle\0".as_ptr() as *const core::ffi::c_char, strlen(b"noidle\0".as_ptr() as *const core::ffi::c_char)) == 0 {
        apc_no_idle = 1;
        return 1;
    }
    0
}

unsafe fn apc_swift_idle() {
    // #ifdef APC_DEBUG_LED: set_auxio(0x00, AUXIO_LED)
    let value = apc_readb(APC_IDLE_REG) | APC_IDLE_ON;
    apc_writeb(value, APC_IDLE_REG);
    // #ifdef APC_DEBUG_LED: set_auxio(AUXIO_LED, 0x00)
}

unsafe fn apc_free(op: *mut platform_device) {
    of_iounmap(
        &mut (*op).resource[0],
        regs,
        resource_size(&(*op).resource[0]),
    );
}

unsafe extern "C" fn apc_open(_inode: *mut inode, _file: *mut file) -> i32 {
    0
}

unsafe extern "C" fn apc_release(_inode: *mut inode, _file: *mut file) -> i32 {
    0
}

unsafe extern "C" fn apc_ioctl(
    _file: *mut file,
    cmd: u32,
    arg: usize,
) -> isize {
    let mut inarg: u8 = 0;
    let arg = arg as *mut u8;

    match cmd {
        APCIOCGFANCTL => {
            if put_user(apc_readb(APC_FANCTL_REG) & APC_REGMASK, arg) != 0 { return -EFAULT as isize; }
        }
        APCIOCGCPWR => {
            if put_user(apc_readb(APC_CPOWER_REG) & APC_REGMASK, arg) != 0 { return -EFAULT as isize; }
        }
        APCIOCGBPORT => {
            if put_user(apc_readb(APC_BPORT_REG) & APC_BPMASK, arg) != 0 { return -EFAULT as isize; }
        }
        APCIOCSFANCTL => {
            if get_user(&mut inarg, arg) != 0 { return -EFAULT as isize; }
            apc_writeb(inarg & APC_REGMASK, APC_FANCTL_REG);
        }
        APCIOCSCPWR => {
            if get_user(&mut inarg, arg) != 0 { return -EFAULT as isize; }
            apc_writeb(inarg & APC_REGMASK, APC_CPOWER_REG);
        }
        APCIOCSBPORT => {
            if get_user(&mut inarg, arg) != 0 { return -EFAULT as isize; }
            apc_writeb(inarg & APC_BPMASK, APC_BPORT_REG);
        }
        _ => return -EINVAL as isize,
    }
    0
}

static mut apc_fops: file_operations = file_operations {
    unlocked_ioctl: Some(apc_ioctl),
    open: Some(apc_open),
    release: Some(apc_release),
    llseek: Some(noop_llseek),
};

static mut apc_miscdev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: APC_DEVNAME.as_ptr() as *const core::ffi::c_char,
    fops: &mut apc_fops,
};

unsafe extern "C" fn apc_probe(op: *mut platform_device) -> i32 {
    let err: i32;

    regs = of_ioremap(&mut (*op).resource[0], 0, resource_size(&(*op).resource[0]), APC_OBPNAME.as_ptr() as *const core::ffi::c_char);
    if regs.is_null() {
        printk(KERN_ERR, b"%s: unable to map registers\n\0".as_ptr(), APC_DEVNAME.as_ptr());
        return -ENODEV;
    }

    err = misc_register(&mut apc_miscdev);
    if err != 0 {
        printk(KERN_ERR, b"%s: unable to register device\n\0".as_ptr(), APC_DEVNAME.as_ptr());
        apc_free(op);
        return -ENODEV;
    }

    // Assign power management IDLE handler.
    if apc_no_idle == 0 {
        sparc_idle = Some(apc_swift_idle);
    }

    printk(KERN_INFO, b"%s: power management initialized%s\n\0".as_ptr(), APC_DEVNAME.as_ptr());
    0
}

static mut apc_match: [of_device_id; 2] = [
    of_device_id { name: APC_OBPNAME.as_ptr() as *const core::ffi::c_char },
    of_device_id { name: core::ptr::null() },
];

static mut apc_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"apc\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: apc_match.as_ptr(),
    },
    probe: Some(apc_probe),
};

unsafe extern "C" fn apc_init() -> i32 {
    platform_driver_register(&mut apc_driver)
}

// This driver is not critical to the boot process and is easiest to ioremap
// when SBus is already initialized, so install it through initcall.

// __setup("apc=", apc_setup);
// __initcall(apc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
