// SPDX-License-Identifier: GPL-2.0-only
/* crc32hash.c - derived from linux/lib/crc32.c, GNU GPL v2 */
/* Usage example:
$ ./crc32hash "Dual Speed"
*/

use std::os::raw::{c_char, c_int, c_uint, c_uchar};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn printf(format: *const c_char, ...) -> c_int;
}

unsafe fn crc32(mut p: *const c_uchar, mut len: c_uint) -> c_uint {
    let mut i: c_int;
    let mut crc: c_uint = 0;
    while len != 0 {
        len = len.wrapping_sub(1);
        crc ^= *p as c_uint;
        p = p.add(1);
        i = 0;
        while i < 8 {
            crc = (crc >> 1) ^ if (crc & 1) != 0 { 0xedb88320 } else { 0 };
            i += 1;
        }
    }
    crc
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let result: c_uint;
    if argc != 2 {
        printf(b"no string passed as argument\n\0".as_ptr() as *const c_char);
        return -1;
    }
    result = crc32(
        *argv.add(1) as *const c_uchar,
        strlen(*argv.add(1)) as c_uint,
    );
    printf(
        b"0x%x\n\0".as_ptr() as *const c_char,
        result,
    );
    0
}
