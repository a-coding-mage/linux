// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Technologic Systems TS-5500 Single Board Computer support
 *
 * Copyright (C) 2013-2014 Savoir-faire Linux Inc.
 *	Vivien Didelot <vivien.didelot@savoirfairelinux.com>
 *
 * This driver registers the Technologic Systems TS-5500 Single Board Computer
 * (SBC) and its devices, and exposes information to userspace such as jumpers'
 * state or available options. For further information about sysfs entries, see
 * Documentation/ABI/testing/sysfs-platform-ts5500.
 *
 * This code may be extended to support similar x86-based platforms.
 * Actually, the TS-5500 and TS-5400 are supported.
 */

use core::ffi::{c_char, c_int, c_void};

// Linux kernel dependencies supplied externally.
extern "C" {
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn check_signature(addr: *mut c_void, signature: *const c_char, length: usize) -> c_int;
    fn request_region(addr: u16, len: u32, name: *const c_char) -> *mut c_void;
    fn release_region(addr: u16, len: u32);
    fn inb(addr: u16) -> u8;
    fn outb(value: u8, addr: u16);
    fn udelay(usec: u32);
}

const TS5500_PRODUCT_CODE_ADDR: u16 = 0x74;
const TS5500_PRODUCT_CODE: i32 = 0x60;
const TS5400_PRODUCT_CODE: i32 = 0x40;

const TS5500_SRAM_RS485_ADC_ADDR: u16 = 0x75;
const TS5500_SRAM: u8 = 1 << 0;
const TS5500_RS485: u8 = 1 << 1;
const TS5500_ADC: u8 = 1 << 2;
const TS5500_RS485_RTS: u8 = 1 << 6;
const TS5500_RS485_AUTO: u8 = 1 << 7;

const TS5500_ERESET_ITR_ADDR: u16 = 0x76;
const TS5500_ERESET: u8 = 1 << 0;
const TS5500_ITR: u8 = 1 << 1;

const TS5500_LED_JP_ADDR: u16 = 0x77;
const TS5500_LED: u8 = 1 << 0;
const TS5500_JP1: u8 = 1 << 1;
const TS5500_JP2: u8 = 1 << 2;
const TS5500_JP3: u8 = 1 << 3;
const TS5500_JP4: u8 = 1 << 4;
const TS5500_JP5: u8 = 1 << 5;
const TS5500_JP6: u8 = 1 << 6;
const TS5500_JP7: u8 = 1 << 7;

const TS5500_ADC_CONV_BUSY_ADDR: u16 = 0x195;
const TS5500_ADC_CONV_BUSY: u8 = 1 << 0;
const TS5500_ADC_CONV_INIT_LSB_ADDR: u16 = 0x196;
const TS5500_ADC_CONV_MSB_ADDR: u16 = 0x197;
const TS5500_ADC_CONV_DELAY: u32 = 12;

#[repr(C)]
pub struct ts5500_sbc {
    pub name: *const c_char,
    pub id: c_int,
    pub sram: bool,
    pub rs485: bool,
    pub adc: bool,
    pub ereset: bool,
    pub itr: bool,
    pub jumpers: u8,
}

#[repr(C)]
struct ts5500_signature {
    string: *const c_char,
    offset: isize,
}

static TS5500_SIGNATURE_STRING: &[u8] = b"TS-5x00 AMD Elan\0";
static TS5500_SIGNATURES: [ts5500_signature; 1] = [ts5500_signature {
    string: TS5500_SIGNATURE_STRING.as_ptr() as *const c_char,
    offset: 0xb14,
}];

#[no_mangle]
pub unsafe extern "C" fn ts5500_check_signature() -> c_int {
    let bios = ioremap(0xf0000, 0x10000);
    if bios.is_null() { return -12; }
    let mut ret: c_int = -19;
    for sig in TS5500_SIGNATURES.iter() {
        if check_signature(bios.offset(sig.offset), sig.string, 17) != 0 {
            ret = 0;
            break;
        }
    }
    iounmap(bios);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ts5500_detect_config(sbc: *mut ts5500_sbc) -> c_int {
    let name = b"ts5500\0";
    if request_region(TS5500_PRODUCT_CODE_ADDR, 4, name.as_ptr() as *const c_char).is_null() { return -16; }
    let mut ret = 0;
    (*sbc).id = inb(TS5500_PRODUCT_CODE_ADDR) as c_int;
    if (*sbc).id == TS5500_PRODUCT_CODE {
        (*sbc).name = b"TS-5500\0".as_ptr() as *const c_char;
    } else if (*sbc).id == TS5400_PRODUCT_CODE {
        (*sbc).name = b"TS-5400\0".as_ptr() as *const c_char;
    } else {
        ret = -19;
    }
    if ret == 0 {
        let tmp = inb(TS5500_SRAM_RS485_ADC_ADDR);
        (*sbc).sram = (tmp & TS5500_SRAM) != 0;
        (*sbc).rs485 = (tmp & TS5500_RS485) != 0;
        (*sbc).adc = (tmp & TS5500_ADC) != 0;
        let tmp = inb(TS5500_ERESET_ITR_ADDR);
        (*sbc).ereset = (tmp & TS5500_ERESET) != 0;
        (*sbc).itr = (tmp & TS5500_ITR) != 0;
        (*sbc).jumpers = inb(TS5500_LED_JP_ADDR) & !TS5500_LED;
    }
    release_region(TS5500_PRODUCT_CODE_ADDR, 4);
    ret
}

// Sysfs show callbacks retain the original interfaces; kernel device types are external.
extern "C" {
    fn dev_get_drvdata(dev: *mut c_void) -> *mut ts5500_sbc;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> isize;
}

pub unsafe extern "C" fn name_show(dev: *mut c_void, _attr: *mut c_void, buf: *mut c_char) -> isize {
    sprintf(buf, b"%s\n\0".as_ptr() as *const c_char, (*dev_get_drvdata(dev)).name)
}
pub unsafe extern "C" fn id_show(dev: *mut c_void, _attr: *mut c_void, buf: *mut c_char) -> isize {
    sprintf(buf, b"0x%.2x\n\0".as_ptr() as *const c_char, (*dev_get_drvdata(dev)).id)
}
pub unsafe extern "C" fn jumpers_show(dev: *mut c_void, _attr: *mut c_void, buf: *mut c_char) -> isize {
    sprintf(buf, b"0x%.2x\n\0".as_ptr() as *const c_char, ((*dev_get_drvdata(dev)).jumpers >> 1) as c_int)
}

macro_rules! ts5500_attr_bool {
    ($field:ident) => {
        pub unsafe extern "C" fn $field##_show(dev: *mut c_void, _attr: *mut c_void, buf: *mut c_char) -> isize {
            sprintf(buf, b"%d\n\0".as_ptr() as *const c_char, (*dev_get_drvdata(dev)).$field as c_int)
        }
    };
}
ts5500_attr_bool!(sram);
ts5500_attr_bool!(rs485);
ts5500_attr_bool!(adc);
ts5500_attr_bool!(ereset);
ts5500_attr_bool!(itr);

#[no_mangle]
pub unsafe extern "C" fn ts5500_led_set(_led_cdev: *mut c_void, brightness: c_int) {
    outb((brightness != 0) as u8, TS5500_LED_JP_ADDR);
}

#[no_mangle]
pub unsafe extern "C" fn ts5500_led_get(_led_cdev: *mut c_void) -> c_int {
    if inb(TS5500_LED_JP_ADDR) & TS5500_LED != 0 { 255 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn ts5500_adc_convert(ctrl: u8) -> c_int {
    outb(ctrl & 0x1f, TS5500_ADC_CONV_INIT_LSB_ADDR);
    udelay(TS5500_ADC_CONV_DELAY);
    if inb(TS5500_ADC_CONV_BUSY_ADDR) & TS5500_ADC_CONV_BUSY != 0 { return -16; }
    let lsb = inb(TS5500_ADC_CONV_INIT_LSB_ADDR);
    let msb = inb(TS5500_ADC_CONV_MSB_ADDR);
    ((msb as c_int) << 8) | lsb as c_int
}

// The platform-device registration data and ts5500_init are supplied by the kernel integration layer.
#[no_mangle]
pub unsafe extern "C" fn ts5500_init() -> c_int {
    ts5500_check_signature()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
