// SPDX-License-Identifier: GPL-2.0-only
/*
 * Parser/loader for IHEX formatted data.
 *
 * Copyright © 2008 David Woodhouse <dwmw2@infradead.org>
 * Copyright © 2005 Jan Harkes <jaharkes@cs.cmu.edu>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null_mut, write_bytes};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const PROT_READ: c_int = 0x1;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

type SsizeT = c_long;
type SizeT = usize;
type ModeT = c_uint;
type OffT = c_long;

#[repr(C)]
struct stat {
    _private: [u8; 0],
}

#[repr(C)]
struct ihex_binrec {
    next: *mut ihex_binrec, /* not part of the real data structure */
    addr: u32,
    len: u16,
    data: [u8; 0],
}

unsafe extern "C" {
    static mut optind: c_int;
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: SizeT,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: OffT,
    ) -> *mut c_void;
    fn malloc(size: SizeT) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SsizeT;
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
}

fn align_kernel_mask(x: usize, mask: usize) -> usize {
    (x + mask) & !mask
}

fn align(x: usize, a: usize) -> usize {
    align_kernel_mask(x, a - 1)
}

/**
 * nybble/hex are little helpers to parse hexadecimal numbers to a byte value
 **/
unsafe fn nybble(n: u8) -> u8 {
    if n >= b'0' && n <= b'9' {
        n - b'0'
    } else if n >= b'A' && n <= b'F' {
        n - (b'A' - 10)
    } else if n >= b'a' && n <= b'f' {
        n - (b'a' - 10)
    } else {
        0
    }
}

unsafe fn hex(data: *const u8, crc: *mut u8) -> u8 {
    let val: u8 = (nybble(*data.add(0)) << 4) | nybble(*data.add(1));
    *crc = (*crc).wrapping_add(val);
    val
}

static mut sort_records: c_int = 0;
static mut wide_records: c_int = 0;
static mut include_jump: c_int = 0;

unsafe fn usage() -> c_int {
    fprintf(
        stderr,
        c"ihex2fw: Convert ihex files into binary representation for use by Linux kernel\n"
            .as_ptr(),
    );
    fprintf(stderr, c"usage: ihex2fw [<options>] <src.HEX> <dst.fw>\n".as_ptr());
    fprintf(stderr, c"       -w: wide records (16-bit length)\n".as_ptr());
    fprintf(stderr, c"       -s: sort records by address\n".as_ptr());
    fprintf(stderr, c"       -j: include records for CS:IP/EIP address\n".as_ptr());
    1
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let infd: c_int;
    let outfd: c_int;
    let mut st: stat = zeroed();
    let data: *mut u8;
    let mut opt: c_int;

    loop {
        opt = getopt(argc, argv, c"wsj".as_ptr());
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b'w' as c_int => {
                wide_records = 1;
            }
            x if x == b's' as c_int => {
                sort_records = 1;
            }
            x if x == b'j' as c_int => {
                include_jump = 1;
            }
            _ => {
                return usage();
            }
        }
    }

    if optind + 2 != argc {
        return usage();
    }

    if strcmp(*argv.add(optind as usize), c"-".as_ptr()) == 0 {
        infd = 0;
    } else {
        infd = open(*argv.add(optind as usize), O_RDONLY);
    }
    if infd == -1 {
        fprintf(
            stderr,
            c"Failed to open source file: %s".as_ptr(),
            strerror(*__errno_location()),
        );
        return usage();
    }
    if fstat(infd, &mut st) != 0 {
        perror(c"stat".as_ptr());
        return 1;
    }

    let st_size = *((&st as *const stat as *const u8).add(48) as *const SsizeT);
    data = mmap(null_mut(), st_size as SizeT, PROT_READ, MAP_SHARED, infd, 0) as *mut u8;
    if data == MAP_FAILED as *mut u8 {
        perror(c"mmap".as_ptr());
        return 1;
    }

    if strcmp(*argv.add((optind + 1) as usize), c"-".as_ptr()) == 0 {
        outfd = 1;
    } else {
        outfd = open(
            *argv.add((optind + 1) as usize),
            O_TRUNC | O_CREAT | O_WRONLY,
            0o644 as ModeT,
        );
    }
    if outfd == -1 {
        fprintf(
            stderr,
            c"Failed to open destination file: %s".as_ptr(),
            strerror(*__errno_location()),
        );
        return usage();
    }
    if process_ihex(data, st_size) != 0 {
        return 1;
    }

    output_records(outfd)
}

unsafe fn process_ihex(data: *mut u8, size: SsizeT) -> c_int {
    let mut record: *mut ihex_binrec;
    let record_size: SizeT;
    let mut offset: u32 = 0;
    let mut data32: u32;
    let mut type_: u8;
    let mut crc: u8 = 0;
    let mut crcbyte: u8;
    let mut i: c_int;
    let mut j: c_int;
    let mut line: c_int = 1;
    let mut len: c_int;

    i = 0;
    'next_record: loop {
        /* search for the start of record character */
        while (i as SsizeT) < size {
            if *data.add(i as usize) == b'\n' {
                line += 1;
            }
            let ch = *data.add(i as usize);
            i += 1;
            if ch == b':' {
                break;
            }
        }

        /* Minimum record length would be about 10 characters */
        if (i as SsizeT) + 10 > size {
            fprintf(stderr, c"Can't find valid record at line %d\n".as_ptr(), line);
            return -EINVAL;
        }

        len = hex(data.add(i as usize), &mut crc) as c_int;
        i += 2;
        if wide_records != 0 {
            len <<= 8;
            len += hex(data.add(i as usize), &mut crc) as c_int;
            i += 2;
        }
        record_size = align(size_of::<ihex_binrec>() + len as usize, 4);
        record = malloc(record_size) as *mut ihex_binrec;
        if record.is_null() {
            fprintf(stderr, c"out of memory for records\n".as_ptr());
            return -ENOMEM;
        }
        write_bytes(record as *mut u8, 0, record_size);
        (*record).len = len as u16;

        /* now check if we have enough data to read everything */
        if (i as SsizeT) + 8 + ((*record).len as SsizeT * 2) > size {
            fprintf(
                stderr,
                c"Not enough data to read complete record at line %d\n".as_ptr(),
                line,
            );
            return -EINVAL;
        }

        (*record).addr = (hex(data.add(i as usize), &mut crc) as u32) << 8;
        i += 2;
        (*record).addr |= hex(data.add(i as usize), &mut crc) as u32;
        i += 2;
        type_ = hex(data.add(i as usize), &mut crc);
        i += 2;

        j = 0;
        while j < (*record).len as c_int {
            *(*record).data.as_mut_ptr().add(j as usize) = hex(data.add(i as usize), &mut crc);
            j += 1;
            i += 2;
        }

        /* check CRC */
        crcbyte = hex(data.add(i as usize), &mut crc);
        i += 2;
        if crc != 0 {
            fprintf(
                stderr,
                c"CRC failure at line %d: got 0x%X, expected 0x%X\n".as_ptr(),
                line,
                crcbyte as c_int,
                crcbyte.wrapping_sub(crc) as u8 as c_int,
            );
            return -EINVAL;
        }

        /* Done reading the record */
        match type_ {
            0 => {
                /* old style EOF record? */
                if (*record).len == 0 {
                    break 'next_record;
                }

                (*record).addr = (*record).addr.wrapping_add(offset);
                file_record(record);
                continue 'next_record;
            }

            1 => {
                /* End-Of-File Record */
                if (*record).addr != 0 || (*record).len != 0 {
                    fprintf(
                        stderr,
                        c"Bad EOF record (type 01) format at line %d".as_ptr(),
                        line,
                    );
                    return -EINVAL;
                }
                break 'next_record;
            }

            2 | 4 => {
                /* Extended Segment Address Record (HEX86) */
                /* Extended Linear Address Record (HEX386) */
                if (*record).addr != 0 || (*record).len != 2 {
                    fprintf(
                        stderr,
                        c"Bad HEX86/HEX386 record (type %02X) at line %d\n".as_ptr(),
                        type_ as c_int,
                        line,
                    );
                    return -EINVAL;
                }

                /* We shouldn't really be using the offset for HEX86 because
                 * the wraparound case is specified quite differently. */
                offset = ((*(*record).data.as_ptr().add(0) as u32) << 8)
                    | (*(*record).data.as_ptr().add(1) as u32);
                offset <<= if type_ == 2 { 4 } else { 16 };
                continue 'next_record;
            }

            3 | 5 => {
                /* Start Segment Address Record */
                /* Start Linear Address Record */
                if (*record).addr != 0 || (*record).len != 4 {
                    fprintf(
                        stderr,
                        c"Bad Start Address record (type %02X) at line %d\n".as_ptr(),
                        type_ as c_int,
                        line,
                    );
                    return -EINVAL;
                }

                memcpy(
                    &mut data32 as *mut u32 as *mut c_void,
                    (*record).data.as_ptr() as *const c_void,
                    size_of::<u32>(),
                );
                data32 = htonl(data32);
                memcpy(
                    (*record).data.as_mut_ptr() as *mut c_void,
                    &data32 as *const u32 as *const c_void,
                    size_of::<u32>(),
                );

                /* These records contain the CS/IP or EIP where execution
                 * starts. If requested output this as a record. */
                if include_jump != 0 {
                    file_record(record);
                }
                continue 'next_record;
            }

            _ => {
                fprintf(
                    stderr,
                    c"Unknown record (type %02X)\n".as_ptr(),
                    type_ as c_int,
                );
                return -EINVAL;
            }
        }
    }

    0
}

static mut records: *mut ihex_binrec = null_mut();

unsafe fn file_record(record: *mut ihex_binrec) {
    let mut p: *mut *mut ihex_binrec = &raw mut records;

    while !(*p).is_null() && (sort_records == 0 || (**p).addr < (*record).addr) {
        p = &mut (**p).next;
    }

    (*record).next = *p;
    *p = record;
}

unsafe fn ihex_binrec_size(p: *mut ihex_binrec) -> u16 {
    (*p).len + size_of::<u32>() as u16 + size_of::<u16>() as u16
}

unsafe fn output_records(outfd: c_int) -> c_int {
    let zeroes: [u8; 6] = [0, 0, 0, 0, 0, 0];
    let mut p: *mut ihex_binrec = records;

    while !p.is_null() {
        let writelen: u16 = align(ihex_binrec_size(p) as usize, 4) as u16;

        (*p).addr = htonl((*p).addr);
        (*p).len = htons((*p).len);
        if write(
            outfd,
            &mut (*p).addr as *mut u32 as *const c_void,
            writelen as SizeT,
        ) != writelen as SsizeT
        {
            return 1;
        }
        p = (*p).next;
    }
    /* EOF record is zero length, since we don't bother to represent
       the type field in the binary version */
    if write(outfd, zeroes.as_ptr() as *const c_void, 6) != 6 {
        return 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
