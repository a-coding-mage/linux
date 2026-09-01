// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2003 - 2004  Dominik Brodowski <linux@dominikbrodowski.de>
 *
 * Based on code found in
 * linux/arch/i386/kernel/cpu/cpufreq/speedstep-centrino.c
 * and originally developed by Jeremy Fitzhardinge.
 *
 * USAGE: simply run it to decode the current settings on CPU 0,
 *	  or pass the CPU number as argument, or pass the MSR content
 *	  as argument.
 */

use std::ffi::{c_char, c_int, c_uint, c_void, CString};

const MCPU: c_uint = 32;

const MSR_IA32_PERF_STATUS: c_uint = 0x198;

const O_RDONLY: c_int = 0;
const SEEK_CUR: c_int = 1;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn lseek(fd: c_int, offset: isize, whence: c_int) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
}

fn rdmsr(cpu: c_uint, msr: c_uint, lo: *mut c_uint, hi: *mut c_uint) -> c_int {
    let fd: c_int;
    let file: CString;
    let mut val: u64 = 0;
    let mut retval: c_int = -1;

    unsafe {
        *hi = 0;
        *lo = *hi;
    }

    if cpu > MCPU {
        return retval;
    }

    file = CString::new(format!("/dev/cpu/{}/msr", cpu)).unwrap();
    fd = unsafe { open(file.as_ptr(), O_RDONLY) };

    if fd < 0 {
        return retval;
    }

    if unsafe { lseek(fd, msr as isize, SEEK_CUR) } == -1 {
        unsafe {
            close(fd);
        }
        return retval;
    }

    if unsafe { read(fd, (&mut val as *mut u64).cast::<c_void>(), 8) } != 8 {
        unsafe {
            close(fd);
        }
        return retval;
    }

    unsafe {
        *lo = (val & 0xffffffff_u64) as u32;
        *hi = ((val >> 32) & 0xffffffff_u64) as u32;
    }

    retval = 0;
    unsafe {
        close(fd);
    }
    retval
}

fn decode(msr: c_uint) {
    let multiplier: c_uint;
    let mv: c_uint;

    multiplier = (msr >> 8) & 0xFF;

    mv = ((msr & 0xFF) * 16) + 700;

    println!("0x{:x} means multiplier {} @ {} mV", msr, multiplier, mv);
}

fn decode_live(cpu: c_uint) -> c_int {
    let mut lo: c_uint = 0;
    let mut hi: c_uint = 0;
    let err: c_int;

    err = rdmsr(cpu, MSR_IA32_PERF_STATUS, &mut lo, &mut hi);

    if err != 0 {
        println!("can't get MSR_IA32_PERF_STATUS for cpu {}", cpu);
        println!("Possible trouble: you don't run an Enhanced SpeedStep capable cpu");
        println!("or you are not root, or the msr driver is not present");
        return 1;
    }

    decode(lo);

    0
}

fn strtoul_base0(s: &str) -> c_uint {
    let bytes = s.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    let mut negative = false;
    if i < bytes.len() {
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            negative = true;
            i += 1;
        }
    }

    let mut base = 10u32;
    if i < bytes.len() && bytes[i] == b'0' {
        base = 8;
        if i + 1 < bytes.len() && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
            base = 16;
            i += 2;
        }
    }

    let mut value = 0u32;
    while i < bytes.len() {
        let digit = match bytes[i] {
            b'0'..=b'9' => (bytes[i] - b'0') as u32,
            b'a'..=b'z' => (bytes[i] - b'a') as u32 + 10,
            b'A'..=b'Z' => (bytes[i] - b'A') as u32 + 10,
            _ => break,
        };

        if digit >= base {
            break;
        }

        value = value.wrapping_mul(base).wrapping_add(digit);
        i += 1;
    }

    if negative {
        value.wrapping_neg()
    } else {
        value
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut cpu: c_uint;
    let mut mode: c_uint = 0;

    if args.len() < 2 {
        cpu = 0;
    } else {
        cpu = strtoul_base0(&args[1]);
        if cpu >= MCPU {
            mode = 1;
        }
    }

    if mode != 0 {
        decode(cpu);
    } else {
        decode_live(cpu);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
