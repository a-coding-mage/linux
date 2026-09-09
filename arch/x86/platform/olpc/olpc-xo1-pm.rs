// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for power management features of the OLPC XO-1 laptop
 *
 * Copyright (C) 2010 Andres Salomon <dilinger@queued.net>
 * Copyright (C) 2010 One Laptop per Child
 * Copyright (C) 2006 Red Hat, Inc.
 * Copyright (C) 2006 Advanced Micro Devices, Inc.
 */

// Translated from the C implementation. Kernel-provided symbols and types are
// intentionally referenced as external dependencies.

const DRV_NAME: &str = "olpc-xo1-pm";

static mut ACPI_BASE: ::core::primitive::c_ulong = 0;
static mut PMS_BASE: ::core::primitive::c_ulong = 0;

static mut WAKEUP_MASK: u16 = CS5536_PM_PWRBTN;

#[repr(C)]
struct OfwBiosEntry {
    address: ::core::primitive::c_ulong,
    segment: u16,
}

static mut OFW_BIOS_ENTRY: OfwBiosEntry = OfwBiosEntry {
    address: 0xF0000 + PAGE_OFFSET,
    segment: __KERNEL_CS,
};

/* Set bits in the wakeup mask */
#[no_mangle]
pub unsafe extern "C" fn olpc_xo1_pm_wakeup_set(value: u16) {
    WAKEUP_MASK |= value;
}

/* Clear bits in the wakeup mask */
#[no_mangle]
pub unsafe extern "C" fn olpc_xo1_pm_wakeup_clear(value: u16) {
    WAKEUP_MASK &= !value;
}

unsafe fn xo1_power_state_enter(pm_state: suspend_state_t) -> i32 {
    let mut saved_sci_mask: ::core::primitive::c_ulong;

    /* Only STR is supported */
    if pm_state != PM_SUSPEND_MEM {
        return -EINVAL;
    }

    /*
     * Save SCI mask (this gets lost since PM1_EN is used as a mask for
     * wakeup events, which is not necessarily the same event set)
     */
    saved_sci_mask = inl(ACPI_BASE + CS5536_PM1_STS);
    saved_sci_mask &= 0xffff0000;

    /* Save CPU state */
    do_olpc_suspend_lowlevel();

    /* Resume path starts here */

    /* Restore SCI mask (using dword access to CS5536_PM1_EN) */
    outl(saved_sci_mask, ACPI_BASE + CS5536_PM1_STS);

    0
}

#[no_mangle]
pub unsafe extern "C" fn xo1_do_sleep(sleep_state: u8) -> i32 {
    let pgd_addr: *mut ::core::ffi::c_void = __va(read_cr3_pa());

    /* Program wakeup mask (using dword access to CS5536_PM1_EN) */
    outl((WAKEUP_MASK as ::core::primitive::c_ulong) << 16,
         ACPI_BASE + CS5536_PM1_STS);

    core::arch::asm!("movl {0}, %eax", in(reg) pgd_addr);
    core::arch::asm!("call *(%edi); cld", in("edi") &OFW_BIOS_ENTRY);
    core::arch::asm!(
        "movb $0x34, %al",
        "outb %al, $0x70",
        "movb $0x30, %al",
        "outb %al, $0x71",
    );
    0
}

unsafe fn xo1_power_off() {
    printk(KERN_INFO, b"OLPC XO-1 power off sequence...\n\0".as_ptr());

    /* Enable all of these controls with 0 delay */
    outl(0x40000000, PMS_BASE + CS5536_PM_SCLK);
    outl(0x40000000, PMS_BASE + CS5536_PM_IN_SLPCTL);
    outl(0x40000000, PMS_BASE + CS5536_PM_WKXD);
    outl(0x40000000, PMS_BASE + CS5536_PM_WKD);

    /* Clear status bits (possibly unnecessary) */
    outl(0x0002ffff, PMS_BASE + CS5536_PM_SSC);
    outl(0xffffffff, ACPI_BASE + CS5536_PM_GPE0_STS);

    /* Write SLP_EN bit to start the machinery */
    outl(0x00002000, ACPI_BASE + CS5536_PM1_CNT);
}

unsafe fn xo1_power_state_valid(pm_state: suspend_state_t) -> bool {
    /* suspend-to-RAM only */
    pm_state == PM_SUSPEND_MEM
}

#[repr(C)]
static XO1_SUSPEND_OPS: platform_suspend_ops = platform_suspend_ops {
    valid: Some(xo1_power_state_valid),
    enter: Some(xo1_power_state_enter),
};

unsafe fn xo1_pm_probe(pdev: *mut platform_device) -> i32 {
    let mut res: *mut resource;

    /* don't run on non-XOs */
    if !machine_is_olpc() {
        return -ENODEV;
    }

    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, b"can't fetch device resource info\n\0".as_ptr());
        return -EIO;
    }
    if strcmp((*pdev).name, b"cs5535-pms\0".as_ptr()) == 0 {
        PMS_BASE = (*res).start;
    } else if strcmp((*pdev).name, b"olpc-xo1-pm-acpi\0".as_ptr()) == 0 {
        ACPI_BASE = (*res).start;
    }

    /* If we have both addresses, we can override the poweroff hook */
    if PMS_BASE != 0 && ACPI_BASE != 0 {
        suspend_set_ops(&XO1_SUSPEND_OPS);
        pm_power_off = Some(xo1_power_off);
        printk(KERN_INFO, b"OLPC XO-1 support registered\n\0".as_ptr());
    }

    0
}

unsafe fn xo1_pm_remove(pdev: *mut platform_device) {
    if strcmp((*pdev).name, b"cs5535-pms\0".as_ptr()) == 0 {
        PMS_BASE = 0;
    } else if strcmp((*pdev).name, b"olpc-xo1-pm-acpi\0".as_ptr()) == 0 {
        ACPI_BASE = 0;
    }

    pm_power_off = None;
}

static mut CS5535_PMS_DRIVER: platform_driver = platform_driver {
    driver: driver { name: b"cs5535-pms\0".as_ptr() },
    probe: Some(xo1_pm_probe),
    remove: Some(xo1_pm_remove),
};

static mut CS5535_ACPI_DRIVER: platform_driver = platform_driver {
    driver: driver { name: b"olpc-xo1-pm-acpi\0".as_ptr() },
    probe: Some(xo1_pm_probe),
    remove: Some(xo1_pm_remove),
};

unsafe fn xo1_pm_init() -> i32 {
    let mut r: i32;

    r = platform_driver_register(&mut CS5535_PMS_DRIVER);
    if r != 0 {
        return r;
    }

    r = platform_driver_register(&mut CS5535_ACPI_DRIVER);
    if r != 0 {
        platform_driver_unregister(&mut CS5535_PMS_DRIVER);
    }

    r
}

arch_initcall!(xo1_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
