/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 Keith M Wesolowski
 * Copyright (C) 2005 Ilya A. Volynets (Total Knowledge)
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct Crime {
    pub bank_ctrl: [u64; 8],
}

unsafe extern "C" {
    pub fn crime_init();
    pub static mut crime: *mut Crime;
    pub fn printk(fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn memblock_add(base: u64, size: u64) -> i32;
}

// CRIME_MAXBANKS, CRIME_MEM_BANK_CONTROL_ADDR,
// CRIME_MEM_BANK_CONTROL_SDRAM_SIZE, and CRIME_HI_MEM_BASE are supplied by
// the corresponding platform headers.

/// C `__init` function.
pub unsafe fn prom_meminit() {
    let mut base: u64;
    let mut size: u64;
    let mut bank: i32;

    crime_init();

    bank = 0;
    while bank < CRIME_MAXBANKS {
        let bankctl: u64 = (*crime).bank_ctrl[bank as usize];
        base = (bankctl & CRIME_MEM_BANK_CONTROL_ADDR) << 25;
        if bank != 0 && base == 0 {
            bank += 1;
            continue;
        }
        size = if (bankctl & CRIME_MEM_BANK_CONTROL_SDRAM_SIZE) != 0 {
            128
        } else {
            32
        };
        size <<= 20;
        if base + size > (256u64 << 20) {
            base += CRIME_HI_MEM_BASE;
        }

        printk(
            b"CRIME MC: bank %u base 0x%016Lx size %LuMiB\0".as_ptr()
                as *const core::ffi::c_char,
            bank as u32,
            base,
            size >> 20,
        );
        memblock_add(base, size);
        bank += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
