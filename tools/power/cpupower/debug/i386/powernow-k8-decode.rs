// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004 Bruno Ducrot <ducrot@poupinou.org>
 *
 * Based on code found in
 * linux/arch/i386/kernel/cpu/cpufreq/powernow-k8.c
 * and originally developed by Paul Devriendt
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

const MCPU: u32 = 32;

const MSR_FIDVID_STATUS: i64 = 0xc0010042;

const MSR_S_HI_CURRENT_VID: u32 = 0x0000001f;
const MSR_S_LO_CURRENT_FID: u32 = 0x0000003f;

const O_RDONLY: c_int = 0;
const SEEK_CUR: c_int = 1;

unsafe extern "C" {
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

unsafe fn get_fidvid(cpu: u32, fid: *mut u32, vid: *mut u32) -> c_int {
    let mut err: c_int = 1;
    let mut msr: u64 = 0;
    let fd: c_int;
    let mut file: [c_char; 20] = [0; 20];

    if cpu > MCPU {
        return err;
    }

    sprintf(
        file.as_mut_ptr(),
        c"/dev/cpu/%d/msr".as_ptr(),
        cpu,
    );

    fd = open(file.as_ptr(), O_RDONLY);
    if fd < 0 {
        return err;
    }
    lseek(fd, MSR_FIDVID_STATUS, SEEK_CUR);
    if read(
        fd,
        (&mut msr as *mut u64).cast::<c_void>(),
        8,
    ) != 8
    {
        close(fd);
        return err;
    }

    *fid = ((msr & 0xffffffff_u64) as u32) & MSR_S_LO_CURRENT_FID;
    *vid = (((msr >> 32) & 0xffffffff_u64) as u32) & MSR_S_HI_CURRENT_VID;
    err = 0;
    close(fd);
    err
}

/* Return a frequency in MHz, given an input fid */
fn find_freq_from_fid(fid: u32) -> u32 {
    800 + (fid * 100)
}

/* Return a voltage in miliVolts, given an input vid */
fn find_millivolts_from_vid(vid: u32) -> u32 {
    1550 - vid * 25
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let err: c_int;
    let cpu: c_int;
    let mut fid: u32 = 0;
    let mut vid: u32 = 0;

    if argc < 2 {
        cpu = 0;
    } else {
        cpu = strtoul(*argv.add(1), core::ptr::null_mut(), 0) as c_int;
    }

    err = get_fidvid(cpu as u32, &mut fid, &mut vid);

    if err != 0 {
        printf(c"can't get fid, vid from MSR\n".as_ptr());
        printf(c"Possible trouble: you don't run a powernow-k8 capable cpu\n".as_ptr());
        printf(c"or you are not root, or the msr driver is not present\n".as_ptr());
        exit(1);
    }

    printf(
        c"cpu %d currently at %d MHz and %d mV\n".as_ptr(),
        cpu,
        find_freq_from_fid(fid),
        find_millivolts_from_vid(vid),
    );

    0
}
