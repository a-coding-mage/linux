// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// S3C series CPU initialisation

/*
 * NOTE: Code in this file is not used on S3C64xx when booting with
 * Device Tree support.
 */

// Linux kernel dependencies supplied by the surrounding translation.

static mut CPU: *mut cpu_table = core::ptr::null_mut();

unsafe fn s3c_lookup_cpu(
    idcode: c_ulong,
    mut tab: *mut cpu_table,
    mut count: c_uint,
) -> *mut cpu_table {
    while count != 0 {
        if (idcode & (*tab).idmask) == ((*tab).idcode & (*tab).idmask) {
            return tab;
        }
        count -= 1;
        tab = tab.add(1);
    }

    core::ptr::null_mut()
}

pub unsafe fn s3c_init_cpu(
    idcode: c_ulong,
    cputab: *mut cpu_table,
    cputab_size: c_uint,
) {
    CPU = s3c_lookup_cpu(idcode, cputab, cputab_size);

    if CPU.is_null() {
        printk!(KERN_ERR, "Unknown CPU type 0x{:08lx}\n", idcode);
        panic!("Unknown S3C24XX CPU");
    }

    printk!("CPU {} (id 0x{:08lx})\n", (*CPU).name, idcode);

    if (*CPU).init.is_none() {
        printk!(KERN_ERR, "CPU {} support not enabled\n", (*CPU).name);
        panic!("Unsupported Samsung CPU");
    }

    if let Some(map_io) = (*CPU).map_io {
        map_io();
    }

    pr_err!("The platform is deprecated and scheduled for removal. Please reach to the maintainers of the platform and linux-samsung-soc@vger.kernel.org if you still use it.  Without such feedback, the platform will be removed after 2022.\n");
}

/* uart management */
// The following section is enabled when CONFIG_SAMSUNG_ATAGS is enabled.
static mut nr_uarts: c_int = 0;

// CONFIG_SERIAL_SAMSUNG_UARTS provides the uart_cfgs array.
static mut uart_cfgs: [s3c2410_uartcfg; CONFIG_SERIAL_SAMSUNG_UARTS] =
    [s3c2410_uartcfg::default(); CONFIG_SERIAL_SAMSUNG_UARTS];

/* s3c24xx_init_uartdevs
 *
 * copy the specified platform data and configuration into our central
 * set of devices, before the data is thrown away after the init process.
 *
 * This also fills in the array passed to the serial driver for the
 * early initialisation of the console.
 */
pub unsafe fn s3c24xx_init_uartdevs(
    name: *mut c_char,
    res: *mut s3c24xx_uart_resources,
    cfg: *mut s3c2410_uartcfg,
    no: c_int,
) {
    let mut cfgptr = uart_cfgs.as_mut_ptr();
    core::ptr::copy_nonoverlapping(
        cfg,
        cfgptr,
        no as usize,
    );

    let mut uart = 0;
    while uart < no {
        let platdev = s3c24xx_uart_src[(*cfgptr).hwport as usize];
        let resp = res.add((*cfgptr).hwport as usize);

        s3c24xx_uart_devs[uart as usize] = platdev;

        (*platdev).name = name;
        (*platdev).resource = (*resp).resources;
        (*platdev).num_resources = (*resp).nr_resources;

        (*platdev).dev.platform_data = cfgptr as *mut c_void;

        uart += 1;
        cfg = cfg.add(1);
        cfgptr = cfgptr.add(1);
    }

    nr_uarts = no;
}

pub unsafe fn s3c24xx_init_uarts(cfg: *mut s3c2410_uartcfg, no: c_int) {
    if CPU.is_null() {
        return;
    }

    if (*CPU).init_uarts.is_none() {
        printk!(KERN_ERR, "s3c24xx_init_uarts: cpu has no uart init\n");
    } else {
        ((*CPU).init_uarts.unwrap())(cfg, no);
    }
}

unsafe fn s3c_arch_init() -> c_int {
    let mut ret: c_int;

    /* init is only needed for ATAGS based platforms */
    // If CONFIG_ATAGS is disabled, this function returns 0.
    if !CONFIG_ATAGS {
        return 0;
    }

    // do the correct init for cpu
    if CPU.is_null() {
        /* Not needed when booting with device tree. */
        if of_have_populated_dt() {
            return 0;
        }
        panic!("s3c_arch_init: NULL cpu\n");
    }

    ret = ((*CPU).init.unwrap())();
    if ret != 0 {
        return ret;
    }

    // CONFIG_SAMSUNG_ATAGS adds the UART platform devices here.
    ret = platform_add_devices(s3c24xx_uart_devs.as_mut_ptr(), nr_uarts as usize);
    ret
}

arch_initcall!(s3c_arch_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
