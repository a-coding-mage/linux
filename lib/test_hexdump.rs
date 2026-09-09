/*
 * Test cases for lib/hexdump.c module.
 */

// The following names are supplied by the surrounding kernel translation unit.

static DATA_B: [u8; 32] = [
    0xbe, 0x32, 0xdb, 0x7b, 0x0a, 0x18, 0x93, 0xb2,
    0x70, 0xba, 0xc4, 0x24, 0x7d, 0x83, 0x34, 0x9b,
    0xa6, 0x9c, 0x31, 0xad, 0x9c, 0x0f, 0xac, 0xe9,
    0x4c, 0xd1, 0x19, 0x99, 0x43, 0xb1, 0xaf, 0x0c,
];

static DATA_A: &[u8] = b".2.{....p..$}.4...1.....L...C...";

static TEST_DATA_1: [&str; 32] = [
    "be", "32", "db", "7b", "0a", "18", "93", "b2",
    "70", "ba", "c4", "24", "7d", "83", "34", "9b",
    "a6", "9c", "31", "ad", "9c", "0f", "ac", "e9",
    "4c", "d1", "19", "99", "43", "b1", "af", "0c",
];
static TEST_DATA_2_LE: [&str; 16] = ["32be", "7bdb", "180a", "b293", "ba70", "24c4", "837d", "9b34", "9ca6", "ad31", "0f9c", "e9ac", "d14c", "9919", "b143", "0caf"];
static TEST_DATA_2_BE: [&str; 16] = ["be32", "db7b", "0a18", "93b2", "70ba", "c424", "7d83", "349b", "a69c", "31ad", "9c0f", "ace9", "4cd1", "1999", "43b1", "af0c"];
static TEST_DATA_4_LE: [&str; 8] = ["7bdb32be", "b293180a", "24c4ba70", "9b34837d", "ad319ca6", "e9ac0f9c", "9919d14c", "0cafb143"];
static TEST_DATA_4_BE: [&str; 8] = ["be32db7b", "0a1893b2", "70bac424", "7d83349b", "a69c31ad", "9c0face9", "4cd11999", "43b1af0c"];
static TEST_DATA_8_LE: [&str; 4] = ["b293180a7bdb32be", "9b34837d24c4ba70", "e9ac0f9cad319ca6", "0cafb1439919d14c"];
static TEST_DATA_8_BE: [&str; 4] = ["be32db7b0a1893b2", "70bac4247d83349b", "a69c31ad9c0face9", "4cd1199943b1af0c"];

const FILL_CHAR: u8 = b'#';
const TEST_HEXDUMP_BUF_SIZE: usize = 32 * 3 + 2 + 32 + 1;

static mut TOTAL_TESTS: u32 = 0;
static mut FAILED_TESTS: u32 = 0;

unsafe fn test_hexdump_prepare_test(len: usize, rowsize: i32, groupsize: i32, test: *mut u8, _testlen: usize, ascii: bool) {
    let mut p = test;
    let mut l = len;
    let mut rs = rowsize;
    let mut gs = groupsize;
    if rs != 16 && rs != 32 { rs = 16; }
    if l > rs as usize { l = rs as usize; }
    if !gs.is_power_of_two() || gs > 8 || len % gs as usize != 0 { gs = 1; }
    let result: &[&str] = if gs == 8 { if cfg!(target_endian = "big") { &TEST_DATA_8_BE } else { &TEST_DATA_8_LE } }
        else if gs == 4 { if cfg!(target_endian = "big") { &TEST_DATA_4_BE } else { &TEST_DATA_4_LE } }
        else if gs == 2 { if cfg!(target_endian = "big") { &TEST_DATA_2_BE } else { &TEST_DATA_2_LE } }
        else { &TEST_DATA_1 };
    let mut i = 0usize;
    while i < l / gs as usize {
        let q = result[i].as_bytes();
        core::ptr::copy_nonoverlapping(q.as_ptr(), p, q.len());
        p = p.add(q.len()); *p = b' '; p = p.add(1); i += 1;
    }
    if i != 0 { p = p.sub(1); }
    if ascii {
        loop { *p = b' '; p = p.add(1); if p >= test.add(rs as usize * 2 + rs as usize / gs as usize + 1) { break; } }
        core::ptr::copy_nonoverlapping(DATA_A.as_ptr(), p, l); p = p.add(l);
    }
    *p = 0;
}

// Kernel-provided helper and logging interfaces are intentionally referenced, not implemented here.
unsafe fn test_hexdump(len: usize, rowsize: i32, groupsize: i32, ascii: bool) {
    let mut test = [0u8; TEST_HEXDUMP_BUF_SIZE]; let mut real = [0u8; TEST_HEXDUMP_BUF_SIZE];
    TOTAL_TESTS += 1; real.fill(FILL_CHAR);
    let r = hex_dump_to_buffer(DATA_B.as_ptr(), len, rowsize, groupsize, real.as_mut_ptr(), real.len(), ascii);
    let _ = r; test.fill(FILL_CHAR); test_hexdump_prepare_test(len, rowsize, groupsize, test.as_mut_ptr(), test.len(), ascii);
    if test != real { FAILED_TESTS += 1; }
}

unsafe fn test_hexdump_set(rowsize: i32, ascii: bool) {
    let d = core::cmp::min(DATA_B.len(), rowsize as usize); let len = get_random_u32_inclusive(1, d as u32) as usize;
    test_hexdump(len, rowsize, 4, ascii); test_hexdump(len, rowsize, 2, ascii); test_hexdump(len, rowsize, 8, ascii); test_hexdump(len, rowsize, 1, ascii);
}

unsafe fn test_hexdump_overflow(buflen: usize, len: usize, rowsize: i32, groupsize: i32, ascii: bool) {
    let mut test = [0u8; TEST_HEXDUMP_BUF_SIZE]; let mut buf = [0u8; TEST_HEXDUMP_BUF_SIZE];
    TOTAL_TESTS += 1; buf.fill(FILL_CHAR);
    let r = hex_dump_to_buffer(DATA_B.as_ptr(), len, rowsize, groupsize, buf.as_mut_ptr(), buflen, ascii);
    let ae = rowsize * 2 + rowsize / groupsize + 1 + len as i32;
    let he = (groupsize * 2 + 1) * len as i32 / groupsize - 1;
    let e = if ascii { ae } else { he };
    let f = core::cmp::min((e + 1) as usize, buflen);
    if buflen != 0 { test.fill(FILL_CHAR); test_hexdump_prepare_test(len, rowsize, groupsize, test.as_mut_ptr(), test.len(), ascii); test[f - 1] = 0; }
    if f <= test.len() { test[f..].fill(FILL_CHAR); }
    if r != e { FAILED_TESTS += 1; }
}

unsafe fn test_hexdump_overflow_set(buflen: usize, ascii: bool) {
    let rs = get_random_u32_inclusive(1, 2) as i32 * 16;
    for i in 0..4 { let gs = 1i32 << i; let len = get_random_u32_below(rs as u32) as usize + gs as usize; test_hexdump_overflow(buflen, len / gs as usize * gs as usize, rs, gs, ascii); }
}

unsafe fn test_hexdump_init() -> i32 {
    let mut rowsize = get_random_u32_inclusive(1, 2) as i32 * 16;
    for _ in 0..16 { test_hexdump_set(rowsize, false); }
    rowsize = get_random_u32_inclusive(1, 2) as i32 * 16;
    for _ in 0..16 { test_hexdump_set(rowsize, true); }
    for i in 0..=TEST_HEXDUMP_BUF_SIZE { test_hexdump_overflow_set(i, false); }
    for i in 0..=TEST_HEXDUMP_BUF_SIZE { test_hexdump_overflow_set(i, true); }
    if FAILED_TESTS == 0 { pr_info!("all {} tests passed\n", TOTAL_TESTS); } else { pr_err!("failed {} out of {} tests\n", FAILED_TESTS, TOTAL_TESTS); }
    if FAILED_TESTS != 0 { -22 } else { 0 }
}

unsafe fn test_hexdump_exit() { /* do nothing */ }

extern "C" {
    fn hex_dump_to_buffer(src: *const u8, len: usize, rowsize: i32, groupsize: i32, buf: *mut u8, buflen: usize, ascii: bool) -> i32;
    fn get_random_u32_inclusive(min: u32, max: u32) -> u32;
    fn get_random_u32_below(max: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
