// SPDX-License-Identifier: GPL-2.0
// Translated from lib/symbol/kallsyms.c.
// Dependencies originally supplied by:
//   "symbol/kallsyms.h"
//   "api/io.h"
//   <stdio.h>
//   <sys/stat.h>
//   <fcntl.h>

use core::ffi::{c_char, c_int, c_void};

// External constants/types/functions are expected from the translated
// dependency set corresponding to the original C includes.
extern "C" {
    static STT_FUNC: u8;
    static STT_OBJECT: u8;
    static KSYM_NAME_LEN: usize;

    fn tolower(c: c_int) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: usize);
    fn io__get_char(io: *mut io) -> c_int;
    fn io__get_hex(io: *mut io, hex: *mut u64) -> c_int;
}

// C stdio BUFSIZ, used exactly as the fixed buffer size in the original file.
const BUFSIZ: usize = 8192;
const O_RDONLY: c_int = 0;

#[repr(C)]
pub struct io {
    pub fd: c_int,
    // The complete layout is supplied by api/io.h in the original source tree.
    // This file accesses only fd and eof.
    pub eof: bool,
}

pub unsafe extern "C" fn kallsyms2elf_type(mut type_: c_char) -> u8 {
    type_ = tolower(type_ as c_int) as c_char;
    if type_ as c_int == b't' as c_int || type_ as c_int == b'w' as c_int {
        STT_FUNC
    } else {
        STT_OBJECT
    }
}

pub unsafe extern "C" fn kallsyms__is_function(mut symbol_type: c_char) -> bool {
    symbol_type = toupper(symbol_type as c_int) as c_char;
    symbol_type as c_int == b'T' as c_int || symbol_type as c_int == b'W' as c_int
}

unsafe fn read_to_eol(io: *mut io) {
    let mut ch: c_int;

    loop {
        ch = io__get_char(io);
        if ch < 0 || ch == b'\n' as c_int {
            return;
        }
    }
}

pub unsafe extern "C" fn kallsyms__parse(
    filename: *const c_char,
    arg: *mut c_void,
    process_symbol: Option<
        unsafe extern "C" fn(
            arg: *mut c_void,
            name: *const c_char,
            type_: c_char,
            start: u64,
        ) -> c_int,
    >,
) -> c_int {
    let mut io: io = core::mem::zeroed();
    let mut bf: [c_char; BUFSIZ] = [0; BUFSIZ];
    let mut err: c_int;

    io.fd = open(filename, O_RDONLY, 0);

    if io.fd < 0 {
        return -1;
    }

    io__init(&mut io, io.fd, bf.as_mut_ptr(), core::mem::size_of_val(&bf));

    err = 0;
    while !io.eof {
        let mut start: u64 = 0;
        let mut ch: c_int;
        let mut i: usize;
        let symbol_type: c_char;
        let name_len = KSYM_NAME_LEN;
        let mut symbol_name = vec![0 as c_char; name_len + 1];

        if io__get_hex(&mut io, &mut start) != b' ' as c_int {
            read_to_eol(&mut io);
            continue;
        }
        symbol_type = io__get_char(&mut io) as c_char;
        if io__get_char(&mut io) != b' ' as c_int {
            read_to_eol(&mut io);
            continue;
        }
        i = 0;
        while i < name_len {
            ch = io__get_char(&mut io);
            if ch < 0 || ch == b'\n' as c_int {
                break;
            }
            symbol_name[i] = ch as c_char;
            i += 1;
        }
        symbol_name[i] = b'\0' as c_char;

        if i == name_len {
            read_to_eol(&mut io);
        }

        err = process_symbol.expect("process_symbol is NULL")(
            arg,
            symbol_name.as_ptr(),
            symbol_type,
            start,
        );
        if err != 0 {
            break;
        }
    }

    close(io.fd);
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
