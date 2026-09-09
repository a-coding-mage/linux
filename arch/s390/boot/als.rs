// SPDX-License-Identifier: GPL-2.0
/*
 *    Copyright IBM Corp. 2016
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_ulong};

const BITS_PER_LONG: usize = core::mem::size_of::<c_ulong>() * 8;

extern "C" {
    static mut stfle_fac_list: [c_ulong; 1];
    fn get_cpu_id(id: *mut cpuid);
    fn boot_emerg(fmt: *const c_char, ...);
    fn disabled_wait() -> !;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
}

#[repr(C)]
pub struct cpuid {
    pub machine: u16,
}

extern "C" {
    static FACILITIES_ALS: c_ulong;
}

// FACILITIES_ALS is a build-time facility mask supplied by the included headers.
const FACILITIES_ALS: c_ulong = 0;
static mut als: [c_ulong; 1] = [FACILITIES_ALS];

unsafe fn u16_to_decimal(mut str_: *mut c_char, mut val: u16) {
    let mut div: c_int = 1;

    while div * 10 <= val as c_int {
        div *= 10;
    }
    while div != 0 {
        *str_ = (b'0' + (val as c_int / div) as u8) as c_char;
        str_ = str_.add(1);
        val %= div as u16;
        div /= 10;
    }
    *str_ = 0;
}

pub unsafe fn print_missing_facilities() {
    static mut als_str: [u8; 80] = [0; 80];
    let mut val: c_ulong;
    let mut val_str = [0i8; 6];
    let mut first: c_int;

    first = 1;
    if als_str[0] == 0 {
        let prefix = b"Missing facilities: ";
        let mut n = 0;
        while n < prefix.len() {
            als_str[n] = prefix[n];
            n += 1;
        }
        als_str[prefix.len()] = 0;
    }
    for i in 0..als.len() {
        val = !stfle_fac_list[i] & als[i];
        for j in 0..BITS_PER_LONG {
            if (val & (1 as c_ulong << (BITS_PER_LONG - 1 - j))) == 0 {
                continue;
            }
            if first == 0 {
                strcat(als_str.as_mut_ptr().cast(), b",\0".as_ptr().cast());
            }
            /*
             * Make sure we stay within one line. Consider that
             * each facility bit adds up to five characters and
             * z/VM adds a four character prefix.
             */
            if strlen(als_str.as_ptr().cast()) > 70 {
                boot_emerg(b"%s\n\0".as_ptr().cast(), als_str.as_ptr().cast());
                als_str[0] = 0;
            }
            u16_to_decimal(val_str.as_mut_ptr(), (i * BITS_PER_LONG + j) as u16);
            strcat(als_str.as_mut_ptr().cast(), val_str.as_ptr().cast());
            first = 0;
        }
    }
    boot_emerg(b"%s\n\0".as_ptr().cast(), als_str.as_ptr().cast());
}

unsafe fn facility_mismatch() {
    let mut id = cpuid { machine: 0 };

    get_cpu_id(&mut id);
    boot_emerg(b"The Linux kernel requires more recent processor hardware\n\0".as_ptr().cast());
    boot_emerg(b"Detected machine-type number: %4x\n\0".as_ptr().cast(), id.machine as c_int);
    print_missing_facilities();
    boot_emerg(b"See z/Architecture Principles of Operation - Facility Indications\n\0".as_ptr().cast());
    disabled_wait();
}

pub unsafe fn verify_facilities() {
    // __stfle(stfle_fac_list, ARRAY_SIZE(stfle_fac_list));
    for i in 0..als.len() {
        if (stfle_fac_list[i] & als[i]) != als[i] {
            facility_mismatch();
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
