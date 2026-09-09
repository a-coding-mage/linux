/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * cmdline.c: Kernel command line creation using ARCS argc/argv.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut arcs_cmdline: *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

static mut IGNORED: [*mut c_char; 7] = [
    b"ConsoleIn=\0" as *const u8 as *mut c_char,
    b"ConsoleOut=\0" as *const u8 as *mut c_char,
    b"SystemPartition=\0" as *const u8 as *mut c_char,
    b"OSLoader=\0" as *const u8 as *mut c_char,
    b"OSLoadPartition=\0" as *const u8 as *mut c_char,
    b"OSLoadFilename=\0" as *const u8 as *mut c_char,
    b"OSLoadOptions=\0" as *const u8 as *mut c_char,
];

static mut USED_ARC: [[*mut c_char; 2]; 2] = [
    [
        b"OSLoadPartition=\0" as *const u8 as *mut c_char,
        b"root=\0" as *const u8 as *mut c_char,
    ],
    [
        b"OSLoadOptions=\0" as *const u8 as *mut c_char,
        b"\0" as *const u8 as *mut c_char,
    ],
];

#[inline]
unsafe fn prom_argv(argv: *mut LONG, index: usize) -> *mut c_char {
    // A 32-bit ARC PROM passes arguments as 32-bit pointers; sign extension
    // is preserved by the intermediate signed pointer-sized conversion.
    (argv.add(index).read() as isize) as *mut c_char
}

unsafe fn move_firmware_args(argc: c_int, argv: *mut LONG, mut cp: *mut c_char) -> *mut c_char {
    let mut actr = 1; /* Always ignore argv[0] */

    while actr < argc {
        let mut i = 0;
        while i < USED_ARC.len() {
            let len = strlen(USED_ARC[i][0]);

            if strncmp(prom_argv(argv, actr as usize), USED_ARC[i][0], len) == 0 {
                /* Ok, we want it. First append the replacement... */
                strcat(cp, USED_ARC[i][1]);
                cp = cp.add(strlen(USED_ARC[i][1]));
                /* ... and now the argument */
                let mut s = strchr(prom_argv(argv, actr as usize), b'=' as c_int);
                if !s.is_null() {
                    s = s.add(1);
                    let len = strlen(s);
                    memcpy(cp as *mut c_void, s as *const c_void, len + 1);
                    cp = cp.add(len);
                }
                cp = cp.add(1);
                cp.sub(1).write(b' ' as c_char);
                break;
            }
            i += 1;
        }
        actr += 1;
    }

    cp
}

pub unsafe fn prom_init_cmdline(argc: c_int, argv: *mut LONG) {
    let mut actr = 1; /* Always ignore argv[0] */
    let mut cp = arcs_cmdline;
    let start = cp;
    /*
     * Move ARC variables to the beginning to make sure they can be
     * overridden by later arguments.
     */
    cp = move_firmware_args(argc, argv, cp);

    while actr < argc {
        let mut i = 0;
        let mut ignored = false;
        while i < IGNORED.len() {
            let len = strlen(IGNORED[i]);
            if strncmp(prom_argv(argv, actr as usize), IGNORED[i], len) == 0 {
                ignored = true;
                break;
            }
            i += 1;
        }

        if !ignored {
            let len = strlen(prom_argv(argv, actr as usize));
            memcpy(cp as *mut c_void, prom_argv(argv, actr as usize) as *const c_void, len + 1);
            cp = cp.add(len);
            cp.write(b' ' as c_char);
            cp = cp.add(1);
        }
        actr += 1;
    }

    if cp != start {
        cp = cp.sub(1);
    }
    cp.write(0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
