/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006, 07 MIPS Technologies, Inc.
 *   written by Ralf Baechle (ralf@linux-mips.org)
 *     written by Ralf Baechle <ralf@linux-mips.org>
 *
 * Copyright (C) 2008 Wind River Systems, Inc.
 *   updated by Tiejun Chen <tiejun.chen@windriver.com>
 *
 * 1. Probe driver for the Malta's UART ports:
 *
 *   o 2 ports in the SMC SuperIO
 *   o 1 port in the CBUS UART, a discrete 16550 which normally is only used
 *     for bringups.
 *
 * We don't use 8250_platform.c on Malta as it would result in the CBUS
 * UART becoming ttyS0.
 *
 * 2. Register RTC-CMOS platform device on Malta.
 */

// Linux kernel dependencies supplied by other translation units.
extern "C" {
    fn platform_add_devices(devices: *const *mut platform_device, count: usize) -> i32;
}

#[repr(C)]
pub struct plat_serial8250_port {
    pub iobase: u64,
    pub mapbase: u64,
    pub irq: i32,
    pub uartclk: u32,
    pub iotype: u32,
    pub flags: u32,
    pub regshift: u32,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: i32,
    pub dev: device,
}

// External kernel constants and configuration-dependent symbols.
extern "C" {
    static MIPS_CPU_IRQ_BASE: i32;
    static MIPSCPU_INT_MB2: i32;
    static PLAT8250_DEV_PLATFORM: i32;
    static UPIO_PORT: u32;
    static UPIO_MEM32BE: u32;
    static UPIO_MEM32: u32;
    static UPF_BOOT_AUTOCONF: u32;
    static UPF_SKIP_TEST: u32;
    static UPF_MAGIC_MULTIPLIER: u32;
    static UPF_IOREMAP: u32;
}

#[inline]
const fn smc_port(base: u64, int: i32) -> plat_serial8250_port {
    plat_serial8250_port {
        iobase: base,
        mapbase: 0,
        irq: int,
        uartclk: 1_843_200,
        iotype: 0, // UPIO_PORT; filled below from the external kernel constant.
        flags: 0,  // UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_MAGIC_MULTIPLIER.
        regshift: 0,
    }
}

// CBUS_UART_FLAGS = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_IOREMAP.
// The C initializer's external macro values are retained symbolically below.
#[no_mangle]
pub static mut uart8250_data: [plat_serial8250_port; 4] = [
    plat_serial8250_port {
        iobase: 0x3f8,
        mapbase: 0,
        irq: 4,
        uartclk: 1_843_200,
        iotype: 0, // UPIO_PORT
        flags: 0, // UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_MAGIC_MULTIPLIER
        regshift: 0,
    },
    plat_serial8250_port {
        iobase: 0x2f8,
        mapbase: 0,
        irq: 3,
        uartclk: 1_843_200,
        iotype: 0, // UPIO_PORT
        flags: 0, // UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_MAGIC_MULTIPLIER
        regshift: 0,
    },
    plat_serial8250_port {
        iobase: 0,
        mapbase: 0x1f000900,
        irq: 0, // MIPS_CPU_IRQ_BASE + MIPSCPU_INT_MB2
        uartclk: 3_686_400, // Twice the usual clk!
        // IS_ENABLED(CONFIG_CPU_BIG_ENDIAN) ? UPIO_MEM32BE : UPIO_MEM32
        iotype: 0,
        flags: 0, // CBUS_UART_FLAGS
        regshift: 3,
    },
    plat_serial8250_port {
        iobase: 0,
        mapbase: 0,
        irq: 0,
        uartclk: 0,
        iotype: 0,
        flags: 0,
        regshift: 0,
    },
];

#[no_mangle]
pub static mut malta_uart8250_device: platform_device = platform_device {
    name: b"serial8250\0".as_ptr(),
    id: 0, // PLAT8250_DEV_PLATFORM
    dev: device {
        platform_data: core::ptr::addr_of_mut!(uart8250_data) as *mut core::ffi::c_void,
    },
};

#[no_mangle]
pub static mut malta_devices: [*mut platform_device; 1] = [
    core::ptr::addr_of_mut!(malta_uart8250_device),
];

#[no_mangle]
pub unsafe extern "C" fn malta_add_devices() -> i32 {
    platform_add_devices(malta_devices.as_ptr(), malta_devices.len())
}

// device_initcall(malta_add_devices);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
