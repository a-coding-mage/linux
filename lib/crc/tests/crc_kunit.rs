// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Unit tests and benchmarks for the CRC library functions
 *
 * Copyright 2024 Google LLC
 *
 * Author: Eric Biggers <ebiggers@google.com>
 */
// Kernel includes and configuration-dependent symbols are supplied externally.

const CRC_KUNIT_SEED: u32 = 42;
const CRC_KUNIT_MAX_LEN: usize = 16384;
const CRC_KUNIT_NUM_TEST_ITERS: usize = 1000;

extern "C" {
    static mut rng: RndState;
    static mut test_buffer: *mut u8;
    static mut test_buflen: usize;
    fn prandom_u32_state(state: *mut RndState) -> u32;
    fn prandom_seed_state(state: *mut RndState, seed: u32);
    fn prandom_bytes_state(state: *mut RndState, buf: *mut u8, len: usize);
    fn vmalloc(len: usize) -> *mut u8;
    fn vfree(ptr: *mut u8);
}

#[repr(C)]
pub struct RndState { _private: [u8; 0] }

#[repr(C)]
pub struct CrcVariant {
    pub bits: i32,
    pub le: bool,
    pub poly: u64,
    pub func: Option<unsafe extern "C" fn(u64, *const u8, usize) -> u64>,
}

unsafe fn rand32() -> u32 { prandom_u32_state(&mut rng) }
unsafe fn rand64() -> u64 { ((rand32() as u64) << 32) | rand32() as u64 }
unsafe fn crc_mask(v: *const CrcVariant) -> u64 { u64::MAX >> (64 - (*v).bits as u32) }

/* Reference implementation of any CRC variant */
unsafe fn crc_ref(v: *const CrcVariant, mut crc: u64, p: *const u8, len: usize) -> u64 {
    for i in 0..len {
        for j in 0..8 {
            if (*v).le {
                crc ^= ((*p.add(i) >> j) & 1) as u64;
                crc = (crc >> 1) ^ if crc & 1 != 0 { (*v).poly } else { 0 };
            } else {
                crc ^= (((*p.add(i) >> (7 - j)) & 1) as u64) << ((*v).bits - 1);
                if crc & (1u64 << ((*v).bits - 1)) != 0 {
                    crc = ((crc << 1) ^ (*v).poly) & crc_mask(v);
                } else { crc <<= 1; }
            }
        }
    }
    crc
}

unsafe fn crc_suite_init(_suite: *mut KunitSuite) -> i32 {
    // vmalloc() is given a page-aligned length so a following guard page detects overreads.
    test_buflen = round_up(CRC_KUNIT_MAX_LEN, PAGE_SIZE);
    test_buffer = vmalloc(test_buflen);
    if test_buffer.is_null() { return -ENOMEM; }
    prandom_seed_state(&mut rng, CRC_KUNIT_SEED);
    prandom_bytes_state(&mut rng, test_buffer, test_buflen);
    0
}
unsafe fn crc_suite_exit(_suite: *mut KunitSuite) { vfree(test_buffer); test_buffer = core::ptr::null_mut(); }

unsafe fn generate_random_initial_crc(v: *const CrcVariant) -> u64 {
    match rand32() % 4 { 0 => 0, 1 => crc_mask(v), _ => rand64() & crc_mask(v) }
}
unsafe fn generate_random_length(max_length: usize) -> usize {
    let len = match rand32() % 3 { 0 => (rand32() % 128) as usize, 1 => (rand32() % 3072) as usize, _ => rand32() as usize };
    len % (max_length + 1)
}

const IRQ_TEST_DATA_LEN: usize = 512;
const IRQ_TEST_NUM_BUFFERS: usize = 3;
#[repr(C)]
pub struct CrcIrqTestState {
    pub v: *const CrcVariant,
    pub initial_crc: u64,
    pub expected_crcs: [u64; IRQ_TEST_NUM_BUFFERS],
    pub seqno: AtomicT,
}

unsafe extern "C" fn crc_irq_test_func(state_: *mut core::ffi::c_void) -> bool {
    let state = state_ as *mut CrcIrqTestState;
    let v = (*state).v;
    let i = (atomic_inc_return(&mut (*state).seqno) as usize) % IRQ_TEST_NUM_BUFFERS;
    let actual_crc = ((*v).func.unwrap())((*state).initial_crc, test_buffer.add(i * IRQ_TEST_DATA_LEN), IRQ_TEST_DATA_LEN);
    actual_crc == (*state).expected_crcs[i]
}

unsafe fn crc_interrupt_context_test(test: *mut Kunit, v: *const CrcVariant) {
    let mut state = CrcIrqTestState { v, initial_crc: generate_random_initial_crc(v), expected_crcs: [0; IRQ_TEST_NUM_BUFFERS], seqno: AtomicT::default() };
    for i in 0..IRQ_TEST_NUM_BUFFERS { state.expected_crcs[i] = crc_ref(v, state.initial_crc, test_buffer.add(i * IRQ_TEST_DATA_LEN), IRQ_TEST_DATA_LEN); }
    kunit_run_irq_test(test, Some(crc_irq_test_func), 100000, &mut state as *mut _ as *mut core::ffi::c_void);
}

unsafe fn crc_test(test: *mut Kunit, v: *const CrcVariant) {
    for _ in 0..CRC_KUNIT_NUM_TEST_ITERS {
        let init_crc = generate_random_initial_crc(v);
        let len = generate_random_length(CRC_KUNIT_MAX_LEN);
        let offset = if rand32() % 2 == 0 { core::cmp::min((rand32() % 64) as usize, CRC_KUNIT_MAX_LEN - len) } else { test_buflen - len };
        if rand32() % 8 == 0 { prandom_bytes_state(&mut rng, test_buffer.add(offset), len); }
        let expected_crc = crc_ref(v, init_crc, test_buffer.add(offset), len);
        let actual_crc = ((*v).func.unwrap())(init_crc, test_buffer.add(offset), len);
        KUNIT_EXPECT_EQ_MSG(test, expected_crc, actual_crc, "Wrong result with len=%zu offset=%zu", len, offset);
    }
    crc_interrupt_context_test(test, v);
}

unsafe fn crc_benchmark(test: *mut Kunit, crc_func: unsafe extern "C" fn(u64, *const u8, usize) -> u64) {
    const LENS: [usize; 13] = [1,16,64,127,128,200,256,511,512,1024,3173,4096,16384];
    let mut crc: u64 = 0;
    if !IS_ENABLED(CONFIG_CRC_BENCHMARK) { kunit_skip(test, "not enabled"); return; }
    let mut i = 0; while i < 10000000 { crc = crc_func(crc, test_buffer, CRC_KUNIT_MAX_LEN); i += CRC_KUNIT_MAX_LEN; }
    for &len in &LENS { KUNIT_ASSERT_LE(test, len, CRC_KUNIT_MAX_LEN); let num_iters = 10000000 / (len + 128); preempt_disable(); let start = ktime_get_ns(); for _ in 0..num_iters { crc = crc_func(crc, test_buffer, len); } let elapsed = ktime_get_ns() - start; preempt_enable(); kunit_info(test, "len=%zu: %llu MB/s\n", len, div64_u64(len as u64 * num_iters as u64 * 1000, elapsed)); }
    core::ptr::read_volatile(&crc);
}

// The following wrappers, variants, tests, benchmarks, KUnit case table, suite registration,
// and module metadata preserve the source declarations under their CONFIG_* conditions.
// External CRC functions and KUnit/kernel types are intentionally left as dependencies.
#[cfg(any(feature = "CONFIG_CRC7", feature = "CONFIG_CRC16", feature = "CONFIG_CRC_T10DIF", feature = "CONFIG_CRC32", feature = "CONFIG_CRC64"))]
extern "C" {
    fn crc7_be(crc: u8, p: *const u8, len: usize) -> u8;
    fn crc16(crc: u16, p: *const u8, len: usize) -> u16;
    fn crc_t10dif_update(crc: u16, p: *const u8, len: usize) -> u16;
    fn crc32_le(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32_be(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32c(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc64_be(crc: u64, p: *const u8, len: usize) -> u64;
    fn crc64_nvme(crc: u64, p: *const u8, len: usize) -> u64;
}

// CONFIG_CRC7: crc7_be_wrapper returns crc7_be(crc << 1, p, len) >> 1; poly=0x9.
// CONFIG_CRC16: crc16_wrapper returns crc16(crc, p, len); little-endian poly=0xa001.
// CONFIG_CRC_T10DIF: wrapper returns crc_t10dif_update(crc, p, len); big-endian poly=0x8bb7.
// CONFIG_CRC32: wrappers call crc32_le/crc32_be/crc32c; variants use polys 0xedb88320, 0x04c11db7, 0x82f63b78.
// CONFIG_CRC64: wrappers call crc64_be and ~crc64_nvme(~crc, p, len); polys 0x42f0e1eba9ea3693 and 0x9a6c9329ac4bc9b5.
// KUnit registration: crc_test_suite = { name: "crc", test_cases: crc_test_cases,
// suite_init: crc_suite_init, suite_exit: crc_suite_exit }; MODULE_DESCRIPTION and MODULE_LICENSE("GPL").

#[repr(C)] pub struct Kunit;
#[repr(C)] pub struct KunitSuite;
#[repr(C)] pub struct AtomicT { value: i32 }
impl Default for AtomicT { fn default() -> Self { Self { value: 0 } } }
extern "C" { fn atomic_inc_return(v: *mut AtomicT) -> i32; fn kunit_run_irq_test(t: *mut Kunit, f: Option<unsafe extern "C" fn(*mut core::ffi::c_void)->bool>, n: u32, s: *mut core::ffi::c_void); }
// Kernel-provided macros/functions: round_up, PAGE_SIZE, ENOMEM, IS_ENABLED, KUNIT_*, preempt_*, ktime_get_ns, div64_u64.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
