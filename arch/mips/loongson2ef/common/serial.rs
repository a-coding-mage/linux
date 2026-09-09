/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 Ralf Baechle (ralf@linux-mips.org)
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Yan hua (yanhua@lemote.com)
 * Author: Wu Zhangjin (wuzhangjin@gmail.com)
 */

// Dependencies supplied by the Linux/MIPS platform and serial subsystems.

#[repr(C)]
pub struct PlatSerial8250Port {
    pub irq: i32,
    pub uartclk: u32,
    pub iotype: u8,
    pub membase: *mut core::ffi::c_void,
    pub flags: u32,
    pub regshift: u32,
    pub mapbase: u64,
    pub iobase: u64,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub dev: Device,
}

extern "C" {
    static mut mips_machtype: usize;
    static loongson_uart_base: u64;
    static _loongson_uart_base: u64;

    fn platform_device_register(device: *mut PlatformDevice) -> i32;
    fn platform_device_unregister(device: *mut PlatformDevice);
    fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize) -> *mut core::ffi::c_void;
}

const UPIO_PORT: u8 = 0;
const UPIO_MEM: u8 = 1;
const UPF_BOOT_AUTOCONF: u32 = 1 << 0;
const UPF_SKIP_TEST: u32 = 1 << 1;
const MIPS_CPU_IRQ_BASE: i32 = 0;
const PLAT8250_DEV_PLATFORM: i32 = -1;
const LOONGSON_PCIIO_BASE: u64 = 0;

const MACH_LOONGSON_UNKNOWN: usize = 0;
const MACH_LEMOTE_FL2E: usize = 1;
const MACH_LEMOTE_FL2F: usize = 2;
const MACH_LEMOTE_ML2F7: usize = 3;
const MACH_LEMOTE_YL2F89: usize = 4;
const MACH_DEXXON_GDIUM2F10: usize = 5;
const MACH_LEMOTE_NAS: usize = 6;
const MACH_LEMOTE_LL2F: usize = 7;
const MACH_LOONGSON_END: usize = 8;

static mut UART8250_DATA: [PlatSerial8250Port; MACH_LOONGSON_END + 1] = [
    PlatSerial8250Port { irq: 0, uartclk: 0, iotype: 0, membase: core::ptr::null_mut(), flags: 0, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: 4, uartclk: 1843200, iotype: UPIO_PORT, membase: core::ptr::null_mut(), flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: 3, uartclk: 1843200, iotype: UPIO_PORT, membase: core::ptr::null_mut(), flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: MIPS_CPU_IRQ_BASE + 3, uartclk: 3686400, iotype: UPIO_MEM, membase: core::ptr::null_mut(), flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: MIPS_CPU_IRQ_BASE + 3, uartclk: 3686400, iotype: UPIO_MEM, membase: core::ptr::null_mut(), flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: MIPS_CPU_IRQ_BASE + 3, uartclk: 3686400, iotype: UPIO_MEM, membase: core::ptr::null_mut(), flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: MIPS_CPU_IRQ_BASE + 3, uartclk: 3686400, iotype: UPIO_MEM, membase: core::ptr::null_mut(), flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: 3, uartclk: 1843200, iotype: UPIO_PORT, membase: core::ptr::null_mut(), flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST, regshift: 0, mapbase: 0, iobase: 0 },
    PlatSerial8250Port { irq: 0, uartclk: 0, iotype: 0, membase: core::ptr::null_mut(), flags: 0, regshift: 0, mapbase: 0, iobase: 0 },
];

static mut UART8250_DEVICE: PlatformDevice = PlatformDevice {
    name: b"serial8250\0".as_ptr() as *const core::ffi::c_char,
    id: PLAT8250_DEV_PLATFORM,
    dev: Device { platform_data: core::ptr::null_mut() },
};

pub unsafe extern "C" fn serial_init() -> i32 {
    let iotype: u8 = UART8250_DATA[mips_machtype].iotype;

    if UPIO_MEM == iotype {
        UART8250_DATA[mips_machtype].mapbase = loongson_uart_base;
        UART8250_DATA[mips_machtype].membase = _loongson_uart_base as *mut core::ffi::c_void;
    } else if UPIO_PORT == iotype {
        UART8250_DATA[mips_machtype].iobase = loongson_uart_base - LOONGSON_PCIIO_BASE;
    }

    memset(
        (&mut UART8250_DATA[mips_machtype + 1]) as *mut PlatSerial8250Port as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<PlatSerial8250Port>(),
    );
    UART8250_DEVICE.dev.platform_data = (&mut UART8250_DATA[mips_machtype]) as *mut PlatSerial8250Port as *mut core::ffi::c_void;

    platform_device_register(&mut UART8250_DEVICE)
}

pub unsafe extern "C" fn serial_exit() {
    platform_device_unregister(&mut UART8250_DEVICE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
