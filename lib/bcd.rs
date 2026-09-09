// SPDX-License-Identifier: GPL-2.0
// Translated from Linux kernel declarations in <linux/bcd.h> and <linux/export.h>.

#[no_mangle]
pub extern "C" fn _bcd2bin(val: u8) -> u32 {
    ((val & 0x0f) as u32) + ((val as u32) >> 4) * 10
}

// EXPORT_SYMBOL(_bcd2bin);

#[no_mangle]
pub extern "C" fn _bin2bcd(val: u32) -> u8 {
    let t: u32 = (val * 103) >> 10;

    ((t << 4) | (val - t * 10)) as u8
}

// EXPORT_SYMBOL(_bin2bcd);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
