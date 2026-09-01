// SPDX-License-Identifier: GPL-2.0
/*
 * selftest for sparc64's privileged ADI driver
 *
 * Author: Tom Hromatka <tom.hromatka@oracle.com>
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type FILE = c_void;
type off_t = i64;
type loff_t = i64;
type ssize_t = isize;

const DEBUG_LEVEL_1_BIT: c_int = 0x0001;
const DEBUG_LEVEL_2_BIT: c_int = 0x0002;
const DEBUG_LEVEL_3_BIT: c_int = 0x0004;
const DEBUG_LEVEL_4_BIT: c_int = 0x0008;
const DEBUG_TIMING_BIT: c_int = 0x1000;

/* bit mask of enabled bits to print */
const DEBUG: c_int = 0x0001;

macro_rules! DEBUG_PRINT_L1 {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        if (DEBUG & DEBUG_LEVEL_1_BIT) != 0 {
            unsafe { printf(concat!($fmt, "\0").as_ptr() as *const c_char $(, $arg)*); }
        }
    };
}

macro_rules! DEBUG_PRINT_L2 {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        if (DEBUG & DEBUG_LEVEL_2_BIT) != 0 {
            unsafe { printf(concat!($fmt, "\0").as_ptr() as *const c_char $(, $arg)*); }
        }
    };
}

macro_rules! DEBUG_PRINT_L3 {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        if (DEBUG & DEBUG_LEVEL_3_BIT) != 0 {
            unsafe { printf(concat!($fmt, "\0").as_ptr() as *const c_char $(, $arg)*); }
        }
    };
}

macro_rules! DEBUG_PRINT_L4 {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        if (DEBUG & DEBUG_LEVEL_4_BIT) != 0 {
            unsafe { printf(concat!($fmt, "\0").as_ptr() as *const c_char $(, $arg)*); }
        }
    };
}

macro_rules! DEBUG_PRINT_T {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        if (DEBUG & DEBUG_TIMING_BIT) != 0 {
            unsafe { printf(concat!($fmt, "\0").as_ptr() as *const c_char $(, $arg)*); }
        }
    };
}

macro_rules! min {
    ($x:expr, $y:expr) => {
        if $x < $y { $x } else { $y }
    };
}

macro_rules! RETURN_FROM_TEST {
    ($ret:expr) => {{
        DEBUG_PRINT_L1!("\tTest %s returned %d\n", function_name!(), $ret);
        return $ret;
    }};
}

macro_rules! TEST_STEP_FAILURE {
    ($ret:expr) => {{
        unsafe {
            fprintf(
                stderr,
                b"\tTest step failure: %d at %s:%d\n\0".as_ptr() as *const c_char,
                $ret,
                function_name!(),
                line!() as c_int,
            );
        }
        break 'out;
    }};
}

macro_rules! RDTICK {
    ($x:expr) => {
        unsafe {
            asm!("rd %tick, {}", out(reg) $x, options(nostack, preserves_flags));
        }
    };
}

macro_rules! function_name {
    () => {
        b"<translated>\0".as_ptr() as *const c_char
    };
}

const ADI_BLKSZ: c_ulong = 64;
const ADI_MAX_VERSION: c_long = 15;

fn random_version() -> c_int {
    let mut tick: c_long;

    RDTICK!(tick);

    (tick % (ADI_MAX_VERSION + 1)) as c_int
}

const MAX_RANGES_SUPPORTED: usize = 5;
static system_ram_str: &[u8] = b"System RAM\n\0";
static mut range_count: c_int = 0;
static mut start_addr: [u64; MAX_RANGES_SUPPORTED] = [0; MAX_RANGES_SUPPORTED];
static mut end_addr: [u64; MAX_RANGES_SUPPORTED] = [0; MAX_RANGES_SUPPORTED];

#[repr(C)]
struct stats {
    name: [c_char; 16],
    total: c_ulong,
    count: c_ulong,
    bytes: c_ulong,
}

static mut read_stats: stats = stats {
    name: [
        b'r' as c_char, b'e' as c_char, b'a' as c_char, b'd' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    total: 0,
    count: 0,
    bytes: 0,
};
static mut pread_stats: stats = stats {
    name: [
        b'p' as c_char, b'r' as c_char, b'e' as c_char, b'a' as c_char, b'd' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    total: 0,
    count: 0,
    bytes: 0,
};
static mut write_stats: stats = stats {
    name: [
        b'w' as c_char, b'r' as c_char, b'i' as c_char, b't' as c_char, b'e' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    total: 0,
    count: 0,
    bytes: 0,
};
static mut pwrite_stats: stats = stats {
    name: [
        b'p' as c_char, b'w' as c_char, b'r' as c_char, b'i' as c_char, b't' as c_char, b'e' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    total: 0,
    count: 0,
    bytes: 0,
};
static mut seek_stats: stats = stats {
    name: [
        b's' as c_char, b'e' as c_char, b'e' as c_char, b'k' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    total: 0,
    count: 0,
    bytes: 0,
};

unsafe fn update_stats(ustats: *mut stats, measurement: c_ulong, bytes: c_ulong) {
    (*ustats).total += measurement;
    (*ustats).bytes += bytes;
    (*ustats).count += 1;
}

unsafe fn print_ustats(ustats: *const stats) {
    DEBUG_PRINT_L1!(
        "%s\t%7d\t%7.0f\t%7.0f\n",
        (*ustats).name.as_ptr(),
        (*ustats).count as c_int,
        (*ustats).total as f64 / (*ustats).count as f64,
        (*ustats).bytes as f64 / (*ustats).count as f64
    );
}

unsafe fn print_stats() {
    DEBUG_PRINT_L1!(
        "\nSyscall\tCall\tAvgTime\tAvgSize\n\tCount\t(ticks)\t(bytes)\n-------------------------------\n"
    );

    print_ustats(&raw const read_stats);
    print_ustats(&raw const pread_stats);
    print_ustats(&raw const write_stats);
    print_ustats(&raw const pwrite_stats);
    print_ustats(&raw const seek_stats);
}

unsafe fn build_memory_map() -> c_int {
    let mut line: [c_char; 256] = [0; 256];
    let fp: *mut FILE;
    let mut i: c_int;

    range_count = 0;

    fp = fopen(b"/proc/iomem\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"/proc/iomem: error %d: %s\n\0".as_ptr() as *const c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        return -*__errno_location();
    }

    while !fgets(line.as_mut_ptr(), line.len() as c_int, fp).is_null() {
        if !strstr(line.as_ptr(), system_ram_str.as_ptr() as *const c_char).is_null() {
            let dash: *mut c_char;
            let mut end_ptr: *mut c_char = core::ptr::null_mut();

            /* Given a line like this:
             * d0400000-10ffaffff : System RAM
             * replace the "-" with a space
             */
            dash = strstr(line.as_ptr(), b"-\0".as_ptr() as *const c_char);
            *dash = 0x20;

            start_addr[range_count as usize] = strtoull(line.as_ptr(), &mut end_ptr, 16);
            end_addr[range_count as usize] = strtoull(end_ptr, core::ptr::null_mut(), 16);
            range_count += 1;
        }
    }

    fclose(fp);

    DEBUG_PRINT_L1!("RAM Ranges\n");
    i = 0;
    while i < range_count {
        DEBUG_PRINT_L1!(
            "\trange %d: 0x%llx\t- 0x%llx\n",
            i,
            start_addr[i as usize],
            end_addr[i as usize]
        );
        i += 1;
    }

    if range_count == 0 {
        fprintf(stderr, b"No valid address ranges found.  Error.\n\0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

unsafe fn read_adi(fd: c_int, buf: *mut u8, buf_sz: c_int) -> c_int {
    let mut ret: c_int;
    let mut bytes_read: c_int = 0;
    let mut start: c_long;
    let mut end: c_long;
    let mut elapsed_time: c_long = 0;

    loop {
        RDTICK!(start);
        ret = read(fd, buf.offset(bytes_read as isize) as *mut c_void, (buf_sz - bytes_read) as usize) as c_int;
        RDTICK!(end);
        if ret < 0 {
            return -*__errno_location();
        }

        elapsed_time += end - start;
        update_stats(&raw mut read_stats, elapsed_time as c_ulong, buf_sz as c_ulong);
        bytes_read += ret;

        if !(bytes_read < buf_sz) {
            break;
        }
    }

    DEBUG_PRINT_T!("\tread elapsed timed = %ld\n", elapsed_time);
    DEBUG_PRINT_L3!("\tRead  %d bytes\n", bytes_read);

    bytes_read
}

unsafe fn pread_adi(fd: c_int, buf: *mut u8, buf_sz: c_int, offset: c_ulong) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let mut bytes_read: c_int = 0;
    let mut cur_offset: c_ulong;
    let mut start: c_long;
    let mut end: c_long;
    let mut elapsed_time: c_long = 0;

    cur_offset = offset;
    loop {
        RDTICK!(start);
        ret = pread(fd, buf.offset(bytes_read as isize) as *mut c_void, (buf_sz - bytes_read) as usize, cur_offset as off_t) as c_int;
        RDTICK!(end);
        if ret < 0 {
            return -*__errno_location();
        }

        elapsed_time += end - start;
        update_stats(&raw mut pread_stats, elapsed_time as c_ulong, buf_sz as c_ulong);
        bytes_read += ret;
        cur_offset += ret as c_ulong;

        if !(bytes_read < buf_sz) {
            break;
        }
    }

    DEBUG_PRINT_T!("\tpread elapsed timed = %ld\n", elapsed_time);
    DEBUG_PRINT_L3!("\tRead  %d bytes starting at offset 0x%lx\n", bytes_read, offset);
    i = 0;
    while i < bytes_read {
        DEBUG_PRINT_L4!("\t\t0x%lx\t%d\n", offset + i as c_ulong, *buf.offset(i as isize) as c_int);
        i += 1;
    }

    bytes_read
}

unsafe fn write_adi(fd: c_int, buf: *const u8, buf_sz: c_int) -> c_int {
    let mut ret: c_int;
    let mut bytes_written: c_int = 0;
    let mut start: c_long;
    let mut end: c_long;
    let mut elapsed_time: c_long = 0;

    loop {
        RDTICK!(start);
        ret = write(fd, buf.offset(bytes_written as isize) as *const c_void, (buf_sz - bytes_written) as usize) as c_int;
        RDTICK!(end);
        if ret < 0 {
            return -*__errno_location();
        }

        elapsed_time += end - start;
        update_stats(&raw mut write_stats, elapsed_time as c_ulong, buf_sz as c_ulong);
        bytes_written += ret;
        if !(bytes_written < buf_sz) {
            break;
        }
    }

    DEBUG_PRINT_T!("\twrite elapsed timed = %ld\n", elapsed_time);
    DEBUG_PRINT_L3!("\tWrote %d of %d bytes\n", bytes_written, buf_sz);

    bytes_written
}

unsafe fn pwrite_adi(fd: c_int, buf: *const u8, buf_sz: c_int, offset: c_ulong) -> c_int {
    let mut ret: c_int;
    let mut bytes_written: c_int = 0;
    let mut cur_offset: c_ulong;
    let mut start: c_long;
    let mut end: c_long;
    let mut elapsed_time: c_long = 0;

    cur_offset = offset;

    loop {
        RDTICK!(start);
        ret = pwrite(
            fd,
            buf.offset(bytes_written as isize) as *const c_void,
            (buf_sz - bytes_written) as usize,
            cur_offset as off_t,
        ) as c_int;
        RDTICK!(end);
        if ret < 0 {
            fprintf(
                stderr,
                b"pwrite(): error %d: %s\n\0".as_ptr() as *const c_char,
                *__errno_location(),
                strerror(*__errno_location()),
            );
            return -*__errno_location();
        }

        elapsed_time += end - start;
        update_stats(&raw mut pwrite_stats, elapsed_time as c_ulong, buf_sz as c_ulong);
        bytes_written += ret;
        cur_offset += ret as c_ulong;

        if !(bytes_written < buf_sz) {
            break;
        }
    }

    DEBUG_PRINT_T!("\tpwrite elapsed timed = %ld\n", elapsed_time);
    DEBUG_PRINT_L3!(
        "\tWrote %d of %d bytes starting at address 0x%lx\n",
        bytes_written,
        buf_sz,
        offset
    );

    bytes_written
}

unsafe fn seek_adi(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    let mut start: c_long;
    let mut end: c_long;
    let ret: off_t;

    RDTICK!(start);
    ret = lseek(fd, offset, whence);
    RDTICK!(end);
    DEBUG_PRINT_L2!("\tlseek ret = 0x%llx\n", ret);
    if ret < 0 {
        lseek(fd, 0, SEEK_END);
        return ret;
    }

    DEBUG_PRINT_T!("\tlseek elapsed timed = %ld\n", end - start);
    update_stats(&raw mut seek_stats, (end - start) as c_ulong, 0);

    lseek(fd, 0, SEEK_END);
    ret
}

unsafe fn test0_prpw_aligned_1byte(fd: c_int) -> c_int {
    /* somewhat arbitrarily chosen address */
    let paddr: c_ulong = (end_addr[(range_count - 1) as usize] as c_ulong - 0x1000) & !(ADI_BLKSZ - 1);
    let mut version: [u8; 1] = [0; 1];
    let expected_version: u8;
    let offset: loff_t;
    let mut ret: c_int = 0;

    version[0] = random_version() as u8;
    expected_version = version[0];

    offset = (paddr / ADI_BLKSZ) as loff_t;

    'out: loop {
        ret = pwrite_adi(fd, version.as_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int {
            TEST_STEP_FAILURE!(ret);
        }

        ret = pread_adi(fd, version.as_mut_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int {
            TEST_STEP_FAILURE!(ret);
        }

        if expected_version != version[0] {
            DEBUG_PRINT_L2!("\tExpected version %d but read version %d\n", expected_version as c_int, version[0] as c_int);
            TEST_STEP_FAILURE!(-(expected_version as c_int));
        }

        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

const TEST1_VERSION_SZ: usize = 4096;
unsafe fn test1_prpw_aligned_4096bytes(fd: c_int) -> c_int {
    let paddr: c_ulong = (end_addr[(range_count - 1) as usize] as c_ulong - 0x6000) & !(ADI_BLKSZ - 1);
    let mut version: [u8; TEST1_VERSION_SZ] = [0; TEST1_VERSION_SZ];
    let mut expected_version: [u8; TEST1_VERSION_SZ] = [0; TEST1_VERSION_SZ];
    let offset: loff_t;
    let mut ret: c_int = 0;
    let mut i: c_int;

    i = 0;
    while i < TEST1_VERSION_SZ as c_int {
        version[i as usize] = random_version() as u8;
        expected_version[i as usize] = version[i as usize];
        i += 1;
    }

    offset = (paddr / ADI_BLKSZ) as loff_t;

    'out: loop {
        ret = pwrite_adi(fd, version.as_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }
        ret = pread_adi(fd, version.as_mut_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }
        i = 0;
        while i < TEST1_VERSION_SZ as c_int {
            if expected_version[i as usize] != version[i as usize] {
                DEBUG_PRINT_L2!("\tExpected version %d but read version %d\n", expected_version[i as usize] as c_int, version[0] as c_int);
                TEST_STEP_FAILURE!(-(expected_version[i as usize] as c_int));
            }
            i += 1;
        }
        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

const TEST2_VERSION_SZ: usize = 10327;
unsafe fn test2_prpw_aligned_10327bytes(fd: c_int) -> c_int {
    let paddr: c_ulong = (start_addr[0] as c_ulong + 0x6000) & !(ADI_BLKSZ - 1);
    let mut version: [u8; TEST2_VERSION_SZ] = [0; TEST2_VERSION_SZ];
    let mut expected_version: [u8; TEST2_VERSION_SZ] = [0; TEST2_VERSION_SZ];
    let offset: loff_t = (paddr / ADI_BLKSZ) as loff_t;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    while i < TEST2_VERSION_SZ as c_int {
        version[i as usize] = random_version() as u8;
        expected_version[i as usize] = version[i as usize];
        i += 1;
    }

    'out: loop {
        ret = pwrite_adi(fd, version.as_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }
        ret = pread_adi(fd, version.as_mut_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }
        i = 0;
        while i < TEST2_VERSION_SZ as c_int {
            if expected_version[i as usize] != version[i as usize] {
                DEBUG_PRINT_L2!("\tExpected version %d but read version %d\n", expected_version[i as usize] as c_int, version[0] as c_int);
                TEST_STEP_FAILURE!(-(expected_version[i as usize] as c_int));
            }
            i += 1;
        }
        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

const TEST3_VERSION_SZ: usize = 12541;
unsafe fn test3_prpw_unaligned_12541bytes(fd: c_int) -> c_int {
    let paddr: c_ulong = ((start_addr[0] as c_ulong + 0xC000) & !(ADI_BLKSZ - 1)) + 17;
    let mut version: [u8; TEST3_VERSION_SZ] = [0; TEST3_VERSION_SZ];
    let mut expected_version: [u8; TEST3_VERSION_SZ] = [0; TEST3_VERSION_SZ];
    let offset: loff_t = (paddr / ADI_BLKSZ) as loff_t;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    while i < TEST3_VERSION_SZ as c_int {
        version[i as usize] = random_version() as u8;
        expected_version[i as usize] = version[i as usize];
        i += 1;
    }

    'out: loop {
        ret = pwrite_adi(fd, version.as_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }
        ret = pread_adi(fd, version.as_mut_ptr(), version.len() as c_int, offset as c_ulong);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }
        i = 0;
        while i < TEST3_VERSION_SZ as c_int {
            if expected_version[i as usize] != version[i as usize] {
                DEBUG_PRINT_L2!("\tExpected version %d but read version %d\n", expected_version[i as usize] as c_int, version[0] as c_int);
                TEST_STEP_FAILURE!(-(expected_version[i as usize] as c_int));
            }
            i += 1;
        }
        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

unsafe fn test4_lseek(fd: c_int) -> c_int {
    const OFFSET_ADD: off_t = 0x100;
    const OFFSET_SUBTRACT: off_t = 0xFFFFFFF000000000u64 as off_t;

    let mut offset_out: off_t;
    let offset_in: off_t;
    let mut ret: c_int = 0;

    offset_in = 0x123456789abcdef0;
    'out: loop {
        offset_out = seek_adi(fd, offset_in, SEEK_SET);
        if offset_out != offset_in {
            ret = -1;
            TEST_STEP_FAILURE!(ret);
        }

        /* seek to the current offset.  this should return EINVAL */
        offset_out = seek_adi(fd, offset_in, SEEK_SET);
        if offset_out < 0 && *__errno_location() == EINVAL {
            DEBUG_PRINT_L2!("\tSEEK_SET failed as designed. Not an error\n");
        } else {
            ret = -2;
            TEST_STEP_FAILURE!(ret);
        }

        offset_out = seek_adi(fd, 0, SEEK_CUR);
        if offset_out != offset_in {
            ret = -3;
            TEST_STEP_FAILURE!(ret);
        }

        offset_out = seek_adi(fd, OFFSET_ADD, SEEK_CUR);
        if offset_out != offset_in + OFFSET_ADD {
            ret = -4;
            TEST_STEP_FAILURE!(ret);
        }

        offset_out = seek_adi(fd, OFFSET_SUBTRACT, SEEK_CUR);
        if offset_out != offset_in + OFFSET_ADD + OFFSET_SUBTRACT {
            ret = -5;
            TEST_STEP_FAILURE!(ret);
        }

        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

unsafe fn test5_rw_aligned_1byte(fd: c_int) -> c_int {
    let paddr: c_ulong = (end_addr[(range_count - 1) as usize] as c_ulong - 0xF000) & !(ADI_BLKSZ - 1);
    let mut version: u8;
    let expected_version: u8;
    let offset: loff_t;
    let mut oret: off_t;
    let mut ret: c_int = 0;

    offset = (paddr / ADI_BLKSZ) as loff_t;
    version = random_version() as u8;
    expected_version = version;

    'out: loop {
        oret = seek_adi(fd, offset, SEEK_SET);
        if oret != offset {
            ret = -1;
            TEST_STEP_FAILURE!(ret);
        }

        ret = write_adi(fd, &version, core::mem::size_of_val(&version) as c_int);
        if ret != core::mem::size_of_val(&version) as c_int { TEST_STEP_FAILURE!(ret); }

        oret = seek_adi(fd, offset, SEEK_SET);
        if oret != offset {
            ret = -1;
            TEST_STEP_FAILURE!(ret);
        }

        ret = read_adi(fd, &mut version, core::mem::size_of_val(&version) as c_int);
        if ret != core::mem::size_of_val(&version) as c_int { TEST_STEP_FAILURE!(ret); }

        if expected_version != version {
            DEBUG_PRINT_L2!("\tExpected version %d but read version %d\n", expected_version as c_int, version as c_int);
            TEST_STEP_FAILURE!(-(expected_version as c_int));
        }

        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

const TEST6_VERSION_SZ: usize = 9434;
unsafe fn test6_rw_aligned_9434bytes(fd: c_int) -> c_int {
    let paddr: c_ulong = (end_addr[(range_count - 1) as usize] as c_ulong - 0x5F000) & !(ADI_BLKSZ - 1);
    let mut version: [u8; TEST6_VERSION_SZ] = [0; TEST6_VERSION_SZ];
    let mut expected_version: [u8; TEST6_VERSION_SZ] = [0; TEST6_VERSION_SZ];
    let offset: loff_t;
    let mut oret: off_t;
    let mut ret: c_int = 0;
    let mut i: c_int;

    offset = (paddr / ADI_BLKSZ) as loff_t;
    i = 0;
    while i < TEST6_VERSION_SZ as c_int {
        version[i as usize] = random_version() as u8;
        expected_version[i as usize] = version[i as usize];
        i += 1;
    }

    'out: loop {
        oret = seek_adi(fd, offset, SEEK_SET);
        if oret != offset { ret = -1; TEST_STEP_FAILURE!(ret); }

        ret = write_adi(fd, version.as_ptr(), version.len() as c_int);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }

        memset(version.as_mut_ptr() as *mut c_void, 0, TEST6_VERSION_SZ);

        oret = seek_adi(fd, offset, SEEK_SET);
        if oret != offset { ret = -1; TEST_STEP_FAILURE!(ret); }

        ret = read_adi(fd, version.as_mut_ptr(), version.len() as c_int);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }

        i = 0;
        while i < TEST6_VERSION_SZ as c_int {
            if expected_version[i as usize] != version[i as usize] {
                DEBUG_PRINT_L2!("\tExpected version %d but read version %d\n", expected_version[i as usize] as c_int, version[i as usize] as c_int);
                TEST_STEP_FAILURE!(-(expected_version[i as usize] as c_int));
            }
            i += 1;
        }

        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

const TEST7_VERSION_SZ: usize = 14963;
unsafe fn test7_rw_aligned_14963bytes(fd: c_int) -> c_int {
    let mut paddr: c_ulong = ((start_addr[(range_count - 1) as usize] as c_ulong + 0xF000) & !(ADI_BLKSZ - 1)) + 39;
    let mut version: [u8; TEST7_VERSION_SZ] = [0; TEST7_VERSION_SZ];
    let mut expected_version: [u8; TEST7_VERSION_SZ] = [0; TEST7_VERSION_SZ];
    let offset: loff_t;
    let mut oret: off_t;
    let mut ret: c_int = 0;
    let mut i: c_int;

    offset = (paddr / ADI_BLKSZ) as loff_t;
    i = 0;
    while i < TEST7_VERSION_SZ as c_int {
        version[i as usize] = random_version() as u8;
        expected_version[i as usize] = version[i as usize];
        i += 1;
    }

    'out: loop {
        oret = seek_adi(fd, offset, SEEK_SET);
        if oret != offset { ret = -1; TEST_STEP_FAILURE!(ret); }

        ret = write_adi(fd, version.as_ptr(), version.len() as c_int);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }

        memset(version.as_mut_ptr() as *mut c_void, 0, TEST7_VERSION_SZ);

        oret = seek_adi(fd, offset, SEEK_SET);
        if oret != offset { ret = -1; TEST_STEP_FAILURE!(ret); }

        ret = read_adi(fd, version.as_mut_ptr(), version.len() as c_int);
        if ret != version.len() as c_int { TEST_STEP_FAILURE!(ret); }

        i = 0;
        while i < TEST7_VERSION_SZ as c_int {
            if expected_version[i as usize] != version[i as usize] {
                DEBUG_PRINT_L2!("\tExpected version %d but read version %d\n", expected_version[i as usize] as c_int, version[i as usize] as c_int);
                TEST_STEP_FAILURE!(-(expected_version[i as usize] as c_int));
            }

            paddr += ADI_BLKSZ;
            i += 1;
        }

        ret = 0;
        break;
    }
    RETURN_FROM_TEST!(ret);
}

static tests: [unsafe fn(c_int) -> c_int; 8] = [
    test0_prpw_aligned_1byte,
    test1_prpw_aligned_4096bytes,
    test2_prpw_aligned_10327bytes,
    test3_prpw_unaligned_12541bytes,
    test4_lseek,
    test5_rw_aligned_1byte,
    test6_rw_aligned_9434bytes,
    test7_rw_aligned_14963bytes,
];
const TEST_COUNT: usize = tests.len();

fn main() -> c_int {
    unsafe {
        let fd: c_int;
        let mut ret: c_int;
        let mut test: c_int;

        ret = build_memory_map();
        if ret < 0 {
            return ret;
        }

        fd = open(b"/dev/adi\0".as_ptr() as *const c_char, O_RDWR);
        if fd < 0 {
            fprintf(
                stderr,
                b"open: error %d: %s\n\0".as_ptr() as *const c_char,
                *__errno_location(),
                strerror(*__errno_location()),
            );
            return -*__errno_location();
        }

        test = 0;
        while test < TEST_COUNT as c_int {
            DEBUG_PRINT_L1!("Running test #%d\n", test);

            ret = tests[test as usize](fd);
            if ret != 0 {
                ksft_test_result_fail(b"Test #%d failed: error %d\n\0".as_ptr() as *const c_char, test, ret);
            } else {
                ksft_test_result_pass(b"Test #%d passed\n\0".as_ptr() as *const c_char, test);
            }
            test += 1;
        }

        print_stats();
        close(fd);

        if ksft_get_fail_cnt() > 0 {
            ksft_exit_fail();
        } else {
            ksft_exit_pass();
        }

        /* it's impossible to get here, but the compiler throws a warning
         * about control reaching the end of non-void function.  bah.
         */
        0
    }
}

const SEEK_SET: c_int = 0;
const SEEK_CUR: c_int = 1;
const SEEK_END: c_int = 2;
const O_RDWR: c_int = 2;
const EINVAL: c_int = 22;

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn pread(fd: c_int, buf: *mut c_void, count: usize, offset: off_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: usize, offset: off_t) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn __errno_location() -> *mut c_int;

    fn ksft_test_result_fail(msg: *const c_char, ...) -> c_void;
    fn ksft_test_result_pass(msg: *const c_char, ...) -> c_void;
    fn ksft_get_fail_cnt() -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
