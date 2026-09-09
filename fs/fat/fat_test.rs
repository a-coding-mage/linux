// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit tests for FAT filesystems.
 *
 * Copyright (C) 2020 Google LLC.
 * Author: David Gow <davidgow@google.com>
 */

// External kernel/FAT declarations are supplied by the surrounding crate.

unsafe fn fat_checksum(name: *const u8) -> u8;
unsafe fn fat_clus_to_blknr(sbi: *const msdos_sb_info, cluster: u32) -> sector_t;
unsafe fn fat_get_blknr_offset(sbi: *const msdos_sb_info, entry: u32,
                               blknr: *mut sector_t, offset: *mut i32);
unsafe fn fat_time_fat2unix(sbi: *const msdos_sb_info, ts: *mut timespec64,
                            time: __le16, date: __le16, cs: u8);
unsafe fn fat_time_unix2fat(sbi: *const msdos_sb_info, ts: *const timespec64,
                            time: *mut __le16, date: *mut __le16, cs: *mut u8);
unsafe fn fat_truncate_atime(sbi: *const msdos_sb_info, ts: *const timespec64) -> timespec64;

#[repr(C)]
struct timespec64 { tv_sec: i64, tv_nsec: i64 }

#[repr(C)]
struct fat_timestamp_testcase {
    name: *const u8,
    ts: timespec64,
    time: __le16,
    date: __le16,
    cs: u8,
    time_offset: i32,
}

#[repr(C)]
struct fat_unix2fat_clamp_testcase {
    name: *const u8,
    ts: timespec64,
    time: __le16,
    date: __le16,
    cs: u8,
    time_offset: i32,
}

#[repr(C)]
struct fat_truncate_atime_testcase {
    name: *const u8,
    ts: timespec64,
    expected: timespec64,
    time_offset: i32,
}

unsafe fn fat_checksum_test(test: *mut kunit) {
    kunit_expect_eq!(test, fat_checksum(b"VMLINUX    \0".as_ptr()), 44u8);
    kunit_expect_eq!(test, fat_checksum(b"README  TXT\0".as_ptr()), 115u8);
    kunit_expect_eq!(test, fat_checksum(b"ABCDEFGHA  \0".as_ptr()), 98u8);
}

unsafe fn fat_clus_to_blknr_test(test: *mut kunit) {
    let sbi = msdos_sb_info { sec_per_clus: 4, data_start: 100, ..core::mem::zeroed() };
    kunit_expect_eq!(test, 100 as sector_t, fat_clus_to_blknr(&sbi, FAT_START_ENT));
    kunit_expect_eq!(test, 112 as sector_t, fat_clus_to_blknr(&sbi, 5));
}

unsafe fn fat_get_blknr_offset_test(test: *mut kunit) {
    let sbi = msdos_sb_info { dir_per_block: 16, dir_per_block_bits: 4, ..core::mem::zeroed() };
    let mut blknr: sector_t = 0;
    let mut offset: i32 = 0;
    fat_get_blknr_offset(&sbi, 0, &mut blknr, &mut offset);
    kunit_expect_eq!(test, 0 as sector_t, blknr);
    kunit_expect_eq!(test, 0, offset);
    fat_get_blknr_offset(&sbi, (10 << 4) | 7, &mut blknr, &mut offset);
    kunit_expect_eq!(test, 10 as sector_t, blknr);
    kunit_expect_eq!(test, 7, offset);
}

static mut time_test_cases: [fat_timestamp_testcase; 11] = [
    fat_timestamp_testcase { name: b"Earliest possible UTC (1980-01-01 00:00:00)\0".as_ptr(), ts: timespec64 { tv_sec: 315532800, tv_nsec: 0 }, time: 0, date: 33, cs: 0, time_offset: 0 },
    fat_timestamp_testcase { name: b"Latest possible UTC (2107-12-31 23:59:58)\0".as_ptr(), ts: timespec64 { tv_sec: 4354819198, tv_nsec: 0 }, time: 49021, date: 65439, cs: 0, time_offset: 0 },
    fat_timestamp_testcase { name: b"Earliest possible (UTC-11) (== 1979-12-31 13:00:00 UTC)\0".as_ptr(), ts: timespec64 { tv_sec: 315493200, tv_nsec: 0 }, time: 0, date: 33, cs: 0, time_offset: 11 * 60 },
    fat_timestamp_testcase { name: b"Latest possible (UTC+11) (== 2108-01-01 10:59:58 UTC)\0".as_ptr(), ts: timespec64 { tv_sec: 4354858798, tv_nsec: 0 }, time: 49021, date: 65439, cs: 0, time_offset: -11 * 60 },
    fat_timestamp_testcase { name: b"Leap Day / Year (1996-02-29 00:00:00)\0".as_ptr(), ts: timespec64 { tv_sec: 825552000, tv_nsec: 0 }, time: 0, date: 8285, cs: 0, time_offset: 0 },
    fat_timestamp_testcase { name: b"Year 2000 is leap year (2000-02-29 00:00:00)\0".as_ptr(), ts: timespec64 { tv_sec: 951782400, tv_nsec: 0 }, time: 0, date: 10333, cs: 0, time_offset: 0 },
    fat_timestamp_testcase { name: b"Year 2100 not leap year (2100-03-01 00:00:00)\0".as_ptr(), ts: timespec64 { tv_sec: 4107542400, tv_nsec: 0 }, time: 0, date: 61537, cs: 0, time_offset: 0 },
    fat_timestamp_testcase { name: b"Leap year + timezone UTC+1 (== 2004-02-29 00:30:00 UTC)\0".as_ptr(), ts: timespec64 { tv_sec: 1078014600, tv_nsec: 0 }, time: 48064, date: 12380, cs: 0, time_offset: -60 },
    fat_timestamp_testcase { name: b"Leap year + timezone UTC-1 (== 2004-02-29 23:30:00 UTC)\0".as_ptr(), ts: timespec64 { tv_sec: 1078097400, tv_nsec: 0 }, time: 960, date: 12385, cs: 0, time_offset: 60 },
    fat_timestamp_testcase { name: b"VFAT odd-second resolution (1999-12-31 23:59:59)\0".as_ptr(), ts: timespec64 { tv_sec: 946684799, tv_nsec: 0 }, time: 49021, date: 10143, cs: 100, time_offset: 0 },
    fat_timestamp_testcase { name: b"VFAT 10ms resolution (1980-01-01 00:00:00:0010)\0".as_ptr(), ts: timespec64 { tv_sec: 315532800, tv_nsec: 10000000 }, time: 0, date: 33, cs: 1, time_offset: 0 },
];
static mut unix2fat_clamp_test_cases: [fat_unix2fat_clamp_testcase; 4] = [
    fat_unix2fat_clamp_testcase { name: b"Clamp to earliest FAT date for 1979-12-31 23:59:59 UTC\0".as_ptr(), ts: timespec64 { tv_sec: 315532799, tv_nsec: 0 }, time: 0, date: 33, cs: 0, time_offset: 0 },
    fat_unix2fat_clamp_testcase { name: b"Clamp after time_offset=-60 pushes 1980-01-01 00:30 UTC below 1980\0".as_ptr(), ts: timespec64 { tv_sec: 315534600, tv_nsec: 0 }, time: 0, date: 33, cs: 0, time_offset: -60 },
    fat_unix2fat_clamp_testcase { name: b"Clamp to latest FAT date for 2108-01-01 00:00:00 UTC\0".as_ptr(), ts: timespec64 { tv_sec: 4354819200, tv_nsec: 0 }, time: 49021, date: 65439, cs: 199, time_offset: 0 },
    fat_unix2fat_clamp_testcase { name: b"Clamp after time_offset=60 pushes 2107-12-31 23:30 UTC beyond 2107\0".as_ptr(), ts: timespec64 { tv_sec: 4354817400, tv_nsec: 0 }, time: 49021, date: 65439, cs: 199, time_offset: 60 },
];
static mut truncate_atime_test_cases: [fat_truncate_atime_testcase; 3] = [
    fat_truncate_atime_testcase { name: b"UTC atime truncates to 2004-02-29 00:00:00\0".as_ptr(), ts: timespec64 { tv_sec: 1078058096, tv_nsec: 789000000 }, expected: timespec64 { tv_sec: 1078012800, tv_nsec: 0 }, time_offset: 0 },
    fat_truncate_atime_testcase { name: b"time_offset=-60 truncates 2004-02-29 00:30 UTC to previous local midnight\0".as_ptr(), ts: timespec64 { tv_sec: 1078014645, tv_nsec: 123000000 }, expected: timespec64 { tv_sec: 1077930000, tv_nsec: 0 }, time_offset: -60 },
    fat_truncate_atime_testcase { name: b"time_offset=60 truncates 2004-02-29 23:30 UTC to next local midnight\0".as_ptr(), ts: timespec64 { tv_sec: 1078097445, tv_nsec: 123000000 }, expected: timespec64 { tv_sec: 1078095600, tv_nsec: 0 }, time_offset: 60 },
];

unsafe fn fat_test_set_time_offset(sbi: *mut msdos_sb_info, time_offset: i32) {
    core::ptr::write_bytes(sbi, 0, 1);
    (*sbi).options.tz_set = 1;
    (*sbi).options.time_offset = time_offset;
}

unsafe fn time_testcase_desc(t: *mut fat_timestamp_testcase, desc: *mut u8) {
    strscpy(desc, (*t).name, KUNIT_PARAM_DESC_SIZE);
}
unsafe fn unix2fat_clamp_testcase_desc(t: *mut fat_unix2fat_clamp_testcase, desc: *mut u8) {
    strscpy(desc, (*t).name, KUNIT_PARAM_DESC_SIZE);
}
unsafe fn truncate_atime_testcase_desc(t: *mut fat_truncate_atime_testcase, desc: *mut u8) {
    strscpy(desc, (*t).name, KUNIT_PARAM_DESC_SIZE);
}

kunit_array_param!(fat_time, time_test_cases, time_testcase_desc);
kunit_array_param!(fat_unix2fat_clamp, unix2fat_clamp_test_cases, unix2fat_clamp_testcase_desc);
kunit_array_param!(fat_truncate_atime, truncate_atime_test_cases, truncate_atime_testcase_desc);

unsafe fn fat_time_fat2unix_test(test: *mut kunit) {
    let mut fake_sb: msdos_sb_info = core::mem::zeroed();
    let mut ts: timespec64 = core::mem::zeroed();
    let testcase = (*test).param_value as *mut fat_timestamp_testcase;
    fat_test_set_time_offset(&mut fake_sb, (*testcase).time_offset);
    fat_time_fat2unix(&fake_sb, &mut ts, (*testcase).time, (*testcase).date, (*testcase).cs);
    kunit_expect_eq_msg!(test, (*testcase).ts.tv_sec, ts.tv_sec, "Timestamp mismatch (seconds)\n");
    kunit_expect_eq_msg!(test, (*testcase).ts.tv_nsec, ts.tv_nsec, "Timestamp mismatch (nanoseconds)\n");
}

unsafe fn fat_time_unix2fat_test(test: *mut kunit) {
    let mut fake_sb: msdos_sb_info = core::mem::zeroed();
    let mut date: __le16 = 0; let mut time: __le16 = 0; let mut cs: u8 = 0;
    let testcase = (*test).param_value as *mut fat_timestamp_testcase;
    fat_test_set_time_offset(&mut fake_sb, (*testcase).time_offset);
    fat_time_unix2fat(&fake_sb, &(*testcase).ts, &mut time, &mut date, &mut cs);
    kunit_expect_eq_msg!(test, (*testcase).time, time, "Time mismatch\n");
    kunit_expect_eq_msg!(test, (*testcase).date, date, "Date mismatch\n");
    kunit_expect_eq_msg!(test, (*testcase).cs, cs, "Centisecond mismatch\n");
}

unsafe fn fat_time_unix2fat_clamp_test(test: *mut kunit) {
    let mut fake_sb: msdos_sb_info = core::mem::zeroed();
    let mut date: __le16 = 0; let mut time: __le16 = 0; let mut cs: u8 = 0;
    let testcase = (*test).param_value as *mut fat_unix2fat_clamp_testcase;
    fat_test_set_time_offset(&mut fake_sb, (*testcase).time_offset);
    fat_time_unix2fat(&fake_sb, &(*testcase).ts, &mut time, &mut date, &mut cs);
    kunit_expect_eq_msg!(test, (*testcase).time, time, "Clamped time mismatch\n");
    kunit_expect_eq_msg!(test, (*testcase).date, date, "Clamped date mismatch\n");
    kunit_expect_eq_msg!(test, (*testcase).cs, cs, "Clamped centisecond mismatch\n");
}

unsafe fn fat_time_unix2fat_no_csec_test(test: *mut kunit) {
    let mut fake_sb: msdos_sb_info = core::mem::zeroed();
    let ts = timespec64 { tv_sec: 946684799, tv_nsec: 0 };
    let mut date: __le16 = 0; let mut time: __le16 = 0;
    fat_test_set_time_offset(&mut fake_sb, 0);
    fat_time_unix2fat(&fake_sb, &ts, &mut time, &mut date, core::ptr::null_mut());
    kunit_expect_eq_msg!(test, 49021, le16_to_cpu(time), "Time mismatch without centiseconds\n");
    kunit_expect_eq_msg!(test, 10143, le16_to_cpu(date), "Date mismatch without centiseconds\n");
}

unsafe fn fat_truncate_atime_test(test: *mut kunit) {
    let mut fake_sb: msdos_sb_info = core::mem::zeroed();
    let testcase = (*test).param_value as *mut fat_truncate_atime_testcase;
    fat_test_set_time_offset(&mut fake_sb, (*testcase).time_offset);
    let actual = fat_truncate_atime(&fake_sb, &(*testcase).ts);
    kunit_expect_eq_msg!(test, (*testcase).expected.tv_sec, actual.tv_sec, "Atime truncation seconds mismatch\n");
    kunit_expect_eq_msg!(test, (*testcase).expected.tv_nsec, actual.tv_nsec, "Atime truncation nanoseconds mismatch\n");
}

static fat_test_cases: () = kunit_cases!(
    fat_checksum_test,
    fat_clus_to_blknr_test,
    fat_get_blknr_offset_test,
    fat_time_fat2unix_test,
    fat_time_unix2fat_test,
    fat_time_unix2fat_clamp_test,
    fat_time_unix2fat_no_csec_test,
    fat_truncate_atime_test,
);
static fat_test_suite: () = kunit_suite!("fat_test", fat_test_cases);

// KUnit parameter and suite registration are provided by the kernel test framework.
kunit_test_suites!(fat_test_suite);
module_description!("KUnit tests for FAT filesystems");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
