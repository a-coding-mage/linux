/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 Ralf Baechle (ralf@linux-mips.org)
 */

// Declarations supplied by the Linux kernel and IP32 platform headers remain
// external dependencies of this translation.

unsafe extern "C" {
    fn ip32_prepare_poweroff();
}

// MACEISA_SERIAL1_OFFS = offsetof(struct sgi_mace, isa.serial1)
// MACEISA_SERIAL2_OFFS = offsetof(struct sgi_mace, isa.serial2)

const MACEISA_SERIAL1_OFFS: usize = core::mem::offset_of!(sgi_mace, isa.serial1);
const MACEISA_SERIAL2_OFFS: usize = core::mem::offset_of!(sgi_mace, isa.serial2);

// MACE_PORT(offset, irq)
const fn mace_port(offset: usize, irq: i32) -> plat_serial8250_port {
    plat_serial8250_port {
        mapbase: MACE_BASE + offset,
        irq,
        uartclk: 1_843_200,
        iotype: UPIO_MEM,
        flags: UPF_SKIP_TEST | UPF_IOREMAP,
        regshift: 8,
        ..plat_serial8250_port::DEFAULT
    }
}

static mut uart8250_data: [plat_serial8250_port; 3] = [
    mace_port(MACEISA_SERIAL1_OFFS, MACEISA_SERIAL1_IRQ),
    mace_port(MACEISA_SERIAL2_OFFS, MACEISA_SERIAL2_IRQ),
    plat_serial8250_port::DEFAULT,
];

static mut uart8250_device: platform_device = platform_device {
    name: "serial8250",
    id: PLAT8250_DEV_PLATFORM,
    dev: device {
        platform_data: core::ptr::addr_of_mut!(uart8250_data) as *mut _,
        ..device::DEFAULT
    },
    ..platform_device::DEFAULT
};

unsafe extern "C" fn uart8250_init() -> i32 {
    platform_device_register(core::ptr::addr_of_mut!(uart8250_device))
}

// device_initcall(uart8250_init);

unsafe extern "C" fn meth_devinit() -> i32 {
    let mut pd: *mut platform_device;
    let mut ret: i32;

    pd = platform_device_alloc("meth", -1);
    if pd.is_null() {
        return -ENOMEM;
    }

    ret = platform_device_add(pd);
    if ret != 0 {
        platform_device_put(pd);
    }

    ret
}

// device_initcall(meth_devinit);

unsafe extern "C" fn sgio2audio_devinit() -> i32 {
    let mut pd: *mut platform_device;
    let mut ret: i32;

    pd = platform_device_alloc("sgio2audio", -1);
    if pd.is_null() {
        return -ENOMEM;
    }

    ret = platform_device_add(pd);
    if ret != 0 {
        platform_device_put(pd);
    }

    ret
}

// device_initcall(sgio2audio_devinit);

unsafe extern "C" fn sgio2btns_devinit() -> i32 {
    IS_ERR(platform_device_register_simple("sgibtns", -1, core::ptr::null_mut(), 0)) as i32
}

// device_initcall(sgio2btns_devinit);

const MACE_RTC_RES_START: usize =
    MACE_BASE + core::mem::offset_of!(sgi_mace, isa.rtc);
const MACE_RTC_RES_END: usize = MACE_RTC_RES_START + 32_767;

static mut ip32_rtc_resources: [resource; 2] = [
    resource {
        start: MACEISA_RTC_IRQ as usize,
        end: MACEISA_RTC_IRQ as usize,
        flags: IORESOURCE_IRQ,
        ..resource::DEFAULT
    },
    resource {
        start: MACE_RTC_RES_START,
        end: MACE_RTC_RES_END,
        flags: IORESOURCE_MEM,
        ..resource::DEFAULT
    },
];

/* RTC registers on IP32 are each padded by 256 bytes (0x100). */
static mut ip32_rtc_platform_data: [ds1685_rtc_platform_data; 1] = [
    ds1685_rtc_platform_data {
        regstep: 0x100,
        bcd_mode: true,
        no_irq: false,
        uie_unsupported: false,
        access_type: ds1685_reg_direct,
        plat_prepare_poweroff: Some(ip32_prepare_poweroff),
        ..ds1685_rtc_platform_data::DEFAULT
    },
];

pub static mut ip32_rtc_device: platform_device = platform_device {
    name: "rtc-ds1685",
    id: -1,
    dev: device {
        platform_data: core::ptr::addr_of_mut!(ip32_rtc_platform_data) as *mut _,
        ..device::DEFAULT
    },
    num_resources: ip32_rtc_resources.len(),
    resource: core::ptr::addr_of_mut!(ip32_rtc_resources) as *mut _,
    ..platform_device::DEFAULT
};

unsafe extern "C" fn sgio2_rtc_devinit() -> i32 {
    platform_device_register(core::ptr::addr_of_mut!(ip32_rtc_device))
}

// device_initcall(sgio2_rtc_devinit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
