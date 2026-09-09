// SPDX-License-Identifier: GPL-2.0-or-later
/* Board-specific reboot/shutdown routines
 *
 * Copyright (c) 2009 Philippe Vachon <philippe@cowpig.ca>
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// External symbols supplied by the platform headers and other translation units.
extern "C" {
    static mut LOONGSON_CHIPCFG: *mut u32;
    static mut mips_machtype: i32;
    static mut LOONGSON_GPIODATA: u32;
    static mut LOONGSON_GPIOIE: u32;

    fn readl(addr: *mut u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn _rdmsr(reg: u32, hi: *mut u32, lo: *mut u32);
    fn _wrmsr(reg: u32, hi: u32, lo: u32);
    fn inl(port: i32) -> u32;
    fn outl(value: u32, port: i32);
    fn mmiowb();
    fn ec_write(reg: u32, value: u32);
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn delay();
}

// Platform constants supplied by the included headers.
const LOONGSON_CHIPCFG_UNUSED: u32 = 0;
const DIVIL_SOFT_RESET: u32 = 0;
const DIVIL_LBAR_GPIO: u32 = 0;
const GPIOL_OUT_EN: i32 = 0;
const GPIOL_OUT_VAL: i32 = 0;
const REG_RESET: u32 = 0;
const BIT_RESET_ON: u32 = 0;
const MACH_LEMOTE_FL2F: i32 = 0;
const MACH_LEMOTE_NAS: i32 = 0;
const MACH_LEMOTE_LL2F: i32 = 0;
const MACH_LEMOTE_ML2F7: i32 = 0;
const MACH_LEMOTE_YL2F89: i32 = 0;

#[inline]
unsafe fn reset_cpu() {
    /*
     * reset cpu to full speed, this is needed when enabling cpu frequency
     * scalling
     */
    writel(readl(LOONGSON_CHIPCFG) | 0x7, LOONGSON_CHIPCFG);
}

/* reset support for fuloong2f */

unsafe fn fl2f_reboot() {
    reset_cpu();

    /* send a reset signal to south bridge.
     *
     * NOTE: if enable "Power Management" in kernel, rtl8169 will not reset
     * normally with this reset operation and it will not work in PMON, but
     * you can type halt command and then reboot, seems the hardware reset
     * logic not work normally.
     */
    {
        let mut hi: u32 = 0;
        let mut lo: u32 = 0;
        _rdmsr(DIVIL_SOFT_RESET, &mut hi, &mut lo);
        lo |= 0x00000001;
        _wrmsr(DIVIL_SOFT_RESET, hi, lo);
    }
}

unsafe fn fl2f_shutdown() {
    let mut hi: u32 = 0;
    let mut lo: u32 = 0;
    let mut val: u32;
    let gpio_base: i32;

    /* get gpio base */
    _rdmsr(DIVIL_LBAR_GPIO, &mut hi, &mut lo);
    gpio_base = (lo & 0xff00) as i32;

    /* make cs5536 gpio13 output enable */
    val = inl(gpio_base + GPIOL_OUT_EN);
    val &= !(1 << (16 + 13));
    val |= 1 << 13;
    outl(val, gpio_base + GPIOL_OUT_EN);
    mmiowb();
    /* make cs5536 gpio13 output low level voltage. */
    val = inl(gpio_base + GPIOL_OUT_VAL) & !(1 << 13);
    val |= 1 << (16 + 13);
    outl(val, gpio_base + GPIOL_OUT_VAL);
    mmiowb();
}

/* reset support for yeeloong2f and mengloong2f notebook */

unsafe fn ml2f_reboot() {
    reset_cpu();

    /* sending an reset signal to EC(embedded controller) */
    ec_write(REG_RESET, BIT_RESET_ON);
}

unsafe fn yl2f89_reboot() {
    ml2f_reboot();
}

const EC_SHUTDOWN_IO_PORT_HIGH: u16 = 0xff2d;
const EC_SHUTDOWN_IO_PORT_LOW: u16 = 0xff2e;
const EC_SHUTDOWN_IO_PORT_DATA: u16 = 0xff2f;
const REG_SHUTDOWN_HIGH: u8 = 0xFC;
const REG_SHUTDOWN_LOW: u8 = 0x29;
const BIT_SHUTDOWN_ON: u8 = 1 << 1;

unsafe fn ml2f_shutdown() {
    let mut val: u8;
    let mut i: u64;

    outb(REG_SHUTDOWN_HIGH, EC_SHUTDOWN_IO_PORT_HIGH);
    outb(REG_SHUTDOWN_LOW, EC_SHUTDOWN_IO_PORT_LOW);
    mmiowb();
    val = inb(EC_SHUTDOWN_IO_PORT_DATA);
    outb(val & !BIT_SHUTDOWN_ON, EC_SHUTDOWN_IO_PORT_DATA);
    mmiowb();
    /* need enough wait here... how many microseconds needs? */
    i = 0;
    while i < 0x10000 {
        delay();
        i += 1;
    }
    outb(val | BIT_SHUTDOWN_ON, EC_SHUTDOWN_IO_PORT_DATA);
    mmiowb();
}

unsafe fn yl2f89_shutdown() {
    /* cpu-gpio0 output low */
    LOONGSON_GPIODATA &= !0x00000001;
    /* cpu-gpio0 as output */
    LOONGSON_GPIOIE &= !0x00000001;
}

#[no_mangle]
pub unsafe extern "C" fn mach_prepare_reboot() {
    match mips_machtype {
        MACH_LEMOTE_FL2F | MACH_LEMOTE_NAS | MACH_LEMOTE_LL2F => fl2f_reboot(),
        MACH_LEMOTE_ML2F7 => ml2f_reboot(),
        MACH_LEMOTE_YL2F89 => yl2f89_reboot(),
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn mach_prepare_shutdown() {
    match mips_machtype {
        MACH_LEMOTE_FL2F | MACH_LEMOTE_NAS | MACH_LEMOTE_LL2F => fl2f_shutdown(),
        MACH_LEMOTE_ML2F7 => ml2f_shutdown(),
        MACH_LEMOTE_YL2F89 => yl2f89_shutdown(),
        _ => {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
