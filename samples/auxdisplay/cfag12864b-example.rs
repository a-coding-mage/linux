// SPDX-License-Identifier: GPL-2.0
/*
 *    Filename: cfag12864b-example.c
 *     Version: 0.1.0
 * Description: cfag12864b LCD userspace example program
 *
 *      Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
 *        Date: 2006-10-31
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// C library and system calls supplied by the target environment.
unsafe extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn getchar() -> c_int;
}

const CFAG12864B_WIDTH: usize = 128;
const CFAG12864B_HEIGHT: usize = 64;
const CFAG12864B_SIZE: usize = 128 * 64 / 8;
const CFAG12864B_BPB: usize = 8;
const EXAMPLES: u8 = 6;

static mut CFAG12864B_FD: c_int = 0;
static mut CFAG12864B_MEM: *mut u8 = core::ptr::null_mut();
static mut CFAG12864B_BUFFER: [u8; CFAG12864B_SIZE] = [0; CFAG12864B_SIZE];

/* init a cfag12864b framebuffer device */
unsafe fn cfag12864b_init(path: *mut c_char) -> c_int {
    const O_RDWR: c_int = 2;
    const PROT_READ: c_int = 1;
    const PROT_WRITE: c_int = 2;
    const MAP_SHARED: c_int = 1;
    const MAP_FAILED: *mut c_void = -1isize as *mut c_void;
    CFAG12864B_FD = open(path, O_RDWR);
    if CFAG12864B_FD == -1 { return -1; }
    CFAG12864B_MEM = mmap(core::ptr::null_mut(), CFAG12864B_SIZE,
        PROT_READ | PROT_WRITE, MAP_SHARED, CFAG12864B_FD, 0) as *mut u8;
    if CFAG12864B_MEM as *mut c_void == MAP_FAILED {
        close(CFAG12864B_FD);
        return -2;
    }
    0
}

unsafe fn cfag12864b_exit() {
    munmap(CFAG12864B_MEM as *mut c_void, CFAG12864B_SIZE);
    close(CFAG12864B_FD);
}

unsafe fn cfag12864b_set(x: u8, y: u8) {
    if (x as usize) < CFAG12864B_WIDTH && (y as usize) < CFAG12864B_HEIGHT {
        let address = (y as usize) * CFAG12864B_WIDTH / CFAG12864B_BPB + (x as usize) / CFAG12864B_BPB;
        CFAG12864B_BUFFER[address] |= 1u8 << (x as usize % CFAG12864B_BPB);
    }
}

unsafe fn cfag12864b_unset(x: u8, y: u8) {
    if (x as usize) < CFAG12864B_WIDTH && (y as usize) < CFAG12864B_HEIGHT {
        let address = (y as usize) * CFAG12864B_WIDTH / CFAG12864B_BPB + (x as usize) / CFAG12864B_BPB;
        CFAG12864B_BUFFER[address] &= !(1u8 << (x as usize % CFAG12864B_BPB));
    }
}

unsafe fn cfag12864b_isset(x: u8, y: u8) -> u8 {
    if (x as usize) < CFAG12864B_WIDTH && (y as usize) < CFAG12864B_HEIGHT {
        let address = (y as usize) * CFAG12864B_WIDTH / CFAG12864B_BPB + (x as usize) / CFAG12864B_BPB;
        if CFAG12864B_BUFFER[address] & (1u8 << (x as usize % CFAG12864B_BPB)) != 0 { return 1; }
    }
    0
}

unsafe fn cfag12864b_not(x: u8, y: u8) {
    if cfag12864b_isset(x, y) != 0 { cfag12864b_unset(x, y); } else { cfag12864b_set(x, y); }
}

unsafe fn cfag12864b_fill() { for i in 0..CFAG12864B_SIZE { CFAG12864B_BUFFER[i] = 0xff; } }
unsafe fn cfag12864b_clear() { for i in 0..CFAG12864B_SIZE { CFAG12864B_BUFFER[i] = 0; } }

unsafe fn cfag12864b_format(matrix: *mut u8) {
    for i in 0..CFAG12864B_HEIGHT { for j in 0..CFAG12864B_WIDTH / CFAG12864B_BPB {
        let address = i * CFAG12864B_WIDTH / CFAG12864B_BPB + j;
        CFAG12864B_BUFFER[address] = 0;
        for n in 0..CFAG12864B_BPB {
            if *matrix.add(i * CFAG12864B_WIDTH + j * CFAG12864B_BPB + n) != 0 { CFAG12864B_BUFFER[address] |= 1u8 << n; }
        }
    }}
}

unsafe fn cfag12864b_blit() { memcpy(CFAG12864B_MEM as *mut c_void, CFAG12864B_BUFFER.as_ptr() as *const c_void, CFAG12864B_SIZE); }

unsafe fn example(n: u8) {
    let mut matrix = [0u8; CFAG12864B_WIDTH * CFAG12864B_HEIGHT];
    if n > EXAMPLES { return; }
    printf(b"Example %i/%i - \0".as_ptr() as *const c_char, n as c_uint, EXAMPLES as c_uint);
    match n {
        1 => { printf(b"Draw points setting bits\0".as_ptr() as *const c_char); cfag12864b_clear(); for i in (0..CFAG12864B_WIDTH).step_by(2) { for j in (0..CFAG12864B_HEIGHT).step_by(2) { cfag12864b_set(i as u8, j as u8); }} }
        2 => { printf(b"Clear the LCD\0".as_ptr() as *const c_char); cfag12864b_clear(); }
        3 => { printf(b"Draw rows formatting a [128*64] matrix\0".as_ptr() as *const c_char); memset(matrix.as_mut_ptr() as *mut c_void, 0, matrix.len()); for i in 0..CFAG12864B_WIDTH { for j in (0..CFAG12864B_HEIGHT).step_by(2) { matrix[j * CFAG12864B_WIDTH + i] = 1; }} cfag12864b_format(matrix.as_mut_ptr()); }
        4 => { printf(b"Fill the lcd\0".as_ptr() as *const c_char); cfag12864b_fill(); }
        5 => { printf(b"Draw columns unsetting bits\0".as_ptr() as *const c_char); for i in (0..CFAG12864B_WIDTH).step_by(2) { for j in 0..CFAG12864B_HEIGHT { cfag12864b_unset(i as u8, j as u8); }} }
        6 => { printf(b"Do negative not-ing all bits\0".as_ptr() as *const c_char); for i in 0..CFAG12864B_WIDTH { for j in 0..CFAG12864B_HEIGHT { cfag12864b_not(i as u8, j as u8); }} }
        _ => {}
    }
    puts(b" - [Press Enter]\0".as_ptr() as *const c_char);
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc != 2 { printf(b"Syntax:  %s fbdev\nUsually: /dev/fb0, /dev/fb1...\n\0".as_ptr() as *const c_char, *argv); return -1; }
    if cfag12864b_init(*argv.add(1)) != 0 { printf(b"Can't init %s fbdev\n\0".as_ptr() as *const c_char, *argv.add(1)); return -2; }
    for n in 1..=EXAMPLES { example(n); cfag12864b_blit(); while getchar() != b'\n' as c_int {} }
    cfag12864b_exit();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
