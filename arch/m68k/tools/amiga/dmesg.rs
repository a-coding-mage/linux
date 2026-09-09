/*
 *  linux/arch/m68k/tools/amiga/dmesg.c -- Retrieve the kernel messages stored
 *                                         in Chip RAM with the kernel command
 *                                         line option `debug=mem'.
 *
 *  © Copyright 1996 by Geert Uytterhoeven <geert@linux-m68k.org>
 *
 *
 *  Usage:
 *
 *      dmesg
 *      dmesg <CHIPMEM_END>
 *
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of the Linux
 *  distribution for more details.
 */

// C dependencies: stdio.h, stdlib.h, unistd.h

const CHIPMEM_START: u_long = 0x00000000;
const CHIPMEM_END: u_long = 0x00200000; // overridden by argv[1]

const SAVEKMSG_MAGIC1: u_long = 0x5341_5645; // 'SAVE'
const SAVEKMSG_MAGIC2: u_long = 0x4B4D_5347; // 'KMSG'

type u_long = u32;

#[repr(C)]
struct savekmsg {
    magic1: u_long, // SAVEKMSG_MAGIC1
    magic2: u_long, // SAVEKMSG_MAGIC2
    magicptr: u_long, // address of magic1
    size: u_long,
    data: [u8; 0],
}

unsafe extern "C" {
    fn strtoul(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> u_long;
    fn printf(format: *const i8, ...) -> i32;
    fn puts(s: *const i8) -> i32;
    fn fflush(stream: *mut core::ffi::c_void) -> i32;
    fn write(fd: i32, buf: *const core::ffi::c_void, count: usize) -> isize;
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: i32, argv: *mut *mut i8) -> i32 {
    let start: u_long = CHIPMEM_START;
    let mut end: u_long = CHIPMEM_END;
    let mut p: u_long;
    let mut found = 0;
    let mut m: *mut savekmsg = core::ptr::null_mut();

    if argc >= 2 {
        end = strtoul(*argv.add(1), core::ptr::null_mut(), 0);
    }
    printf(c"Searching for SAVEKMSG magic...\n".as_ptr());
    p = start;
    while p <= end - core::mem::size_of::<savekmsg>() as u_long {
        m = p as *mut savekmsg;
        if (*m).magic1 == SAVEKMSG_MAGIC1
            && (*m).magic2 == SAVEKMSG_MAGIC2
            && (*m).magicptr == p
        {
            found = 1;
            break;
        }
        p = p.wrapping_add(4);
    }
    if found == 0 {
        puts(c"Not found\0".as_ptr());
    } else {
        printf(c"Found %ld bytes at 0x%08lx\n\0".as_ptr(), (*m).size, (*m).data.as_ptr());
        puts(c">>>>>>>>>>>>>>>>>>>>\0".as_ptr());
        fflush(core::ptr::null_mut());
        write(1, (*m).data.as_ptr() as *const core::ffi::c_void, (*m).size as usize);
        fflush(core::ptr::null_mut());
        puts(c"<<<<<<<<<<<<<<<<<<<<\0".as_ptr());
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
