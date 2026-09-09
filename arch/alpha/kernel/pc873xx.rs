// SPDX-License-Identifier: GPL-2.0
//
// C dependencies: <linux/ioport.h>, <asm/io.h>, and "pc873xx.h" are
// supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_uchar, c_void};

extern "C" {
    fn outb(value: c_uchar, port: c_uint);
    fn inb(port: c_uint) -> c_uchar;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn request_region(start: c_uint, n: c_uint, name: *const c_char) -> *mut c_void;
    fn release_region(start: c_uint, n: c_uint);
    fn printk(fmt: *const c_char, ...);
}

// REG_SID, REG_PCR, REG_FER, KERN_INFO, PC87303, PC87306, PC87332, and
// PC87334 are supplied by pc873xx.h and the surrounding kernel translation.

static mut pc873xx_probelist: [c_uint; 3] = [0x398, 0x26e, 0];

static mut pc873xx_names: [*mut c_char; 5] = [
    b"PC87303\0".as_ptr() as *mut c_char,
    b"PC87306\0".as_ptr() as *mut c_char,
    b"PC87312\0".as_ptr() as *mut c_char,
    b"PC87332\0".as_ptr() as *mut c_char,
    b"PC87334\0".as_ptr() as *mut c_char,
];

static mut base: c_uint = 0;
static mut model: c_uint = 0;

pub unsafe extern "C" fn pc873xx_get_base() -> c_uint {
    base
}

pub unsafe extern "C" fn pc873xx_get_model() -> *mut c_char {
    pc873xx_names[model as usize]
}

unsafe fn pc873xx_read(base: c_uint, reg: c_int) -> c_uchar {
    outb(reg as c_uchar, base);
    inb(base.wrapping_add(1))
}

unsafe fn pc873xx_write(base: c_uint, reg: c_int, data: c_uchar) {
    let mut flags: c_ulong = 0;

    local_irq_save(&mut flags as *mut c_ulong);
    outb(reg as c_uchar, base);
    outb(data, base.wrapping_add(1));
    outb(data, base.wrapping_add(1)); // Must be written twice
    local_irq_restore(flags);
}

pub unsafe extern "C" fn pc873xx_probe() -> c_int {
    let mut val: c_int;
    let mut index: usize = 0;

    while {
        base = pc873xx_probelist[index];
        index += 1;
        base != 0
    } {
        if request_region(base, 2, b"Super IO PC873xx\0".as_ptr() as *const c_char).is_null() {
            continue;
        }

        val = pc873xx_read(base, REG_SID);
        if (val & 0xf0) == 0x10 {
            model = PC87332;
            break;
        } else if (val & 0xf8) == 0x70 {
            model = PC87306;
            break;
        } else if (val & 0xf8) == 0x50 {
            model = PC87334;
            break;
        } else if (val & 0xf8) == 0x40 {
            model = PC87303;
            break;
        }

        release_region(base, 2);
    }

    if base == 0 { -1 } else { 1 }
}

pub unsafe extern "C" fn pc873xx_enable_epp19() {
    let data: c_uchar;

    printk(b"PC873xx enabling EPP v1.9\n\0".as_ptr() as *const c_char);
    data = pc873xx_read(base, REG_PCR);
    pc873xx_write(base, REG_PCR, (data & 0xfc) | 0x02);
}

pub unsafe extern "C" fn pc873xx_enable_ide() {
    let data: c_uchar;

    printk(b"PC873xx enabling IDE interrupt\n\0".as_ptr() as *const c_char);
    data = pc873xx_read(base, REG_FER);
    pc873xx_write(base, REG_FER, data | 0x40);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
