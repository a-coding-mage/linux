// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2025 Intel Corporation. All rights reserved.

// Preface all log entries with "cxl_translate"
// C preprocessor intent: #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// Dependencies from the original C includes:
// linux/moduleparam.h, linux/module.h, linux/kernel.h, linux/init.h,
// linux/slab.h, linux/acpi.h, cxlmem.h, cxl.h

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

const ULLONG_MAX: u64 = u64::MAX;
const U8_MAX: c_uint = u8::MAX as c_uint;
const U16_MAX: c_uint = u16::MAX as c_uint;
const EINVAL: c_int = 22;
const ERANGE: c_int = 34;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const CXL_DECODER_MAX_INTERLEAVE: usize = 16;

/* Maximum number of test vectors and entry length */
const MAX_TABLE_ENTRIES: usize = 128;
const MAX_ENTRY_LEN: usize = 128;

/* Expected number of parameters in each test vector */
const EXPECTED_PARAMS: c_int = 7;

/* Module parameters for test vectors */
static mut table: [*mut c_char; MAX_TABLE_ENTRIES] = [core::ptr::null_mut(); MAX_TABLE_ENTRIES];
static mut table_num: c_int = 0;

/* Interleave Arithmetic */
const MODULO_MATH: c_int = 0;
const XOR_MATH: c_int = 1;

/*
 * XOR mapping configuration
 * The test data sets all use the same set of xormaps. When additional
 * data sets arrive for validation, this static setup will need to
 * be changed to accept xormaps as additional parameters.
 */
#[repr(C)]
pub struct cxl_cxims_data {
    pub nr_maps: c_int,
    pub xormaps: [u64; 0],
}

static mut cximsd: *mut cxl_cxims_data = core::ptr::null_mut();
static mut xormaps: [u64; 4] = [
    0x2020900,
    0x4041200,
    0x1010400,
    0x800,
];

static mut nr_maps: c_int = 4;

const HBIW_TO_NR_MAPS_SIZE: usize = CXL_DECODER_MAX_INTERLEAVE + 1;
static hbiw_to_nr_maps: [c_int; HBIW_TO_NR_MAPS_SIZE] = [
    0, 0, 1, 0, 2, 0, 1, 0, 3, 0, 0, 0, 2, 0, 0, 0, 4,
];

extern "C" {
    fn cxl_calculate_hpa_offset(dpa_offset: u64, pos: c_int, r_eiw: u8, r_eig: u16) -> u64;
    fn cxl_do_xormap_calc(cximsd: *mut cxl_cxims_data, hpa_offset: u64, hb_ways: u8) -> u64;
    fn cxl_calculate_dpa_offset(hpa_offset: u64, r_eiw: u8, r_eig: u16) -> u64;
    fn cxl_calculate_position(hpa_offset: u64, r_eiw: u8, r_eig: u16) -> u64;
    fn cxl_validate_translation_params(eiw: u8, eig: u16, pos: c_int) -> c_int;
    fn eiw_to_ways(eiw: u8, ways: *mut c_int) -> c_int;
    fn get_random_u32() -> u32;
    fn get_random_u64() -> u64;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn cximsd_xormaps_ptr(p: *mut cxl_cxims_data) -> *mut u64 {
    (p as *mut u8).add(core::mem::size_of::<cxl_cxims_data>()) as *mut u64
}

unsafe fn struct_size_cximsd_xormaps(count: c_int) -> usize {
    core::mem::size_of::<cxl_cxims_data>() + (count as usize) * core::mem::size_of::<u64>()
}

/**
 * to_hpa - calculate an HPA offset from a DPA offset and position
 *
 * dpa_offset: device physical address offset
 * pos: devices position in interleave
 * r_eiw: region encoded interleave ways
 * r_eig: region encoded interleave granularity
 * hb_ways: host bridge interleave ways
 * math: interleave arithmetic (MODULO_MATH or XOR_MATH)
 *
 * Returns: host physical address offset
 */
unsafe fn to_hpa(dpa_offset: u64, pos: c_int, r_eiw: u8, r_eig: u16, hb_ways: u8,
                 math: u8) -> u64 {
    let mut hpa_offset: u64;

    /* Calculate base HPA offset from DPA and position */
    hpa_offset = cxl_calculate_hpa_offset(dpa_offset, pos, r_eiw, r_eig);
    if hpa_offset == ULLONG_MAX {
        return ULLONG_MAX;
    }

    if math as c_int == XOR_MATH {
        (*cximsd).nr_maps = hbiw_to_nr_maps[hb_ways as usize];
        if (*cximsd).nr_maps != 0 {
            return cxl_do_xormap_calc(cximsd, hpa_offset, hb_ways);
        }
    }
    hpa_offset
}

/**
 * to_dpa - translate an HPA offset to DPA offset
 *
 * hpa_offset: host physical address offset
 * r_eiw: region encoded interleave ways
 * r_eig: region encoded interleave granularity
 * hb_ways: host bridge interleave ways
 * math: interleave arithmetic (MODULO_MATH or XOR_MATH)
 *
 * Returns: device physical address offset
 */
unsafe fn to_dpa(hpa_offset: u64, r_eiw: u8, r_eig: u16, hb_ways: u8, math: u8) -> u64 {
    let mut offset: u64 = hpa_offset;

    if math as c_int == XOR_MATH {
        (*cximsd).nr_maps = hbiw_to_nr_maps[hb_ways as usize];
        if (*cximsd).nr_maps != 0 {
            offset = cxl_do_xormap_calc(cximsd, hpa_offset, hb_ways);
        }
    }
    cxl_calculate_dpa_offset(offset, r_eiw, r_eig)
}

/**
 * to_pos - extract an interleave position from an HPA offset
 *
 * hpa_offset: host physical address offset
 * r_eiw: region encoded interleave ways
 * r_eig: region encoded interleave granularity
 * hb_ways: host bridge interleave ways
 * math: interleave arithmetic (MODULO_MATH or XOR_MATH)
 *
 * Returns: devices position in region interleave
 */
unsafe fn to_pos(hpa_offset: u64, r_eiw: u8, r_eig: u16, hb_ways: u8, math: u8) -> u64 {
    let mut offset: u64 = hpa_offset;

    /* Reverse XOR mapping if specified */
    if math as c_int == XOR_MATH {
        offset = cxl_do_xormap_calc(cximsd, hpa_offset, hb_ways);
    }

    cxl_calculate_position(offset, r_eiw, r_eig)
}

/**
 * run_translation_test - execute forward and reverse translations
 *
 * @dpa: device physical address
 * @pos: expected position in region interleave
 * @r_eiw: region encoded interleave ways
 * @r_eig: region encoded interleave granularity
 * @hb_ways: host bridge interleave ways
 * @math: interleave arithmetic (MODULO_MATH or XOR_MATH)
 * @expect_spa: expected system physical address
 *
 * Returns: 0 on success, -1 on failure
 */
unsafe fn run_translation_test(dpa: u64, pos: c_int, r_eiw: u8, r_eig: u16,
                               hb_ways: u8, math: c_int, expect_hpa: u64) -> c_int {
    let mut translated_spa: u64;
    let mut reverse_dpa: u64;
    let mut reverse_pos: c_int;

    /* Test Device to Host translation: DPA + POS -> SPA */
    translated_spa = to_hpa(dpa, pos, r_eiw, r_eig, hb_ways, math as u8);
    if translated_spa != expect_hpa {
        pr_err(b"Device to host failed: expected HPA %llu, got %llu\n\0".as_ptr() as *const c_char,
               expect_hpa, translated_spa);
        return -1;
    }

    /* Test Host to Device DPA translation: SPA -> DPA */
    reverse_dpa = to_dpa(translated_spa, r_eiw, r_eig, hb_ways, math as u8);
    if reverse_dpa != dpa {
        pr_err(b"Host to Device DPA failed: expected %llu, got %llu\n\0".as_ptr() as *const c_char,
               dpa, reverse_dpa);
        return -1;
    }

    /* Test Host to Device Position translation: SPA -> POS */
    reverse_pos = to_pos(translated_spa, r_eiw, r_eig, hb_ways, math as u8) as c_int;
    if reverse_pos != pos {
        pr_err(b"Position lookup failed: expected %d, got %d\n\0".as_ptr() as *const c_char,
               pos, reverse_pos);
        return -1;
    }

    0
}

/**
 * parse_test_vector - parse a single test vector string
 *
 * entry: test vector string to parse
 * dpa: device physical address
 * pos: expected position in region interleave
 * r_eiw: region encoded interleave ways
 * r_eig: region encoded interleave granularity
 * hb_ways: host bridge interleave ways
 * math: interleave arithmetic (MODULO_MATH or XOR_MATH)
 * expect_spa: expected system physical address
 *
 * Returns: 0 on success, negative error code on failure
 */
unsafe fn parse_test_vector(entry: *const c_char, dpa: *mut u64, pos: *mut c_int, r_eiw: *mut u8,
                            r_eig: *mut u16, hb_ways: *mut u8, math: *mut c_int,
                            expect_hpa: *mut u64) -> c_int {
    let mut tmp_r_eiw: c_uint = 0;
    let mut tmp_r_eig: c_uint = 0;
    let mut tmp_hb_ways: c_uint = 0;
    let parsed: c_int;

    parsed = sscanf(entry, b"%llu %d %u %u %u %d %llu\0".as_ptr() as *const c_char,
                    dpa, pos, &mut tmp_r_eiw, &mut tmp_r_eig, &mut tmp_hb_ways,
                    math, expect_hpa);

    if parsed != EXPECTED_PARAMS {
        pr_err(b"Parse error: expected %d parameters, got %d in '%s'\n\0".as_ptr() as *const c_char,
               EXPECTED_PARAMS, parsed, entry);
        return -EINVAL;
    }
    if tmp_r_eiw > U8_MAX || tmp_r_eig > U16_MAX || tmp_hb_ways > U8_MAX {
        pr_err(b"Parameter overflow in entry: '%s'\n\0".as_ptr() as *const c_char, entry);
        return -ERANGE;
    }
    if *math != MODULO_MATH && *math != XOR_MATH {
        pr_err(b"Invalid math type %d in entry: '%s'\n\0".as_ptr() as *const c_char,
               *math, entry);
        return -EINVAL;
    }
    *r_eiw = tmp_r_eiw as u8;
    *r_eig = tmp_r_eig as u16;
    *hb_ways = tmp_hb_ways as u8;

    0
}

/*
 * setup_xor_mapping - Initialize XOR mapping data structure
 *
 * The test data sets all use the same HBIG so we can use one set
 * of xormaps, and set the number to apply based on HBIW before
 * calling cxl_do_xormap_calc().
 *
 * When additional data sets arrive for validation with different
 * HBIG's this static setup will need to be updated.
 *
 * Returns: 0 on success, negative error code on failure
 */
unsafe fn setup_xor_mapping() -> c_int {
    if nr_maps <= 0 {
        return -EINVAL;
    }

    cximsd = kzalloc(struct_size_cximsd_xormaps(nr_maps), GFP_KERNEL) as *mut cxl_cxims_data;
    if cximsd.is_null() {
        return -ENOMEM;
    }

    (*cximsd).nr_maps = nr_maps;
    memcpy(cximsd_xormaps_ptr(cximsd) as *mut c_void,
           xormaps.as_ptr() as *const c_void,
           nr_maps as usize * core::mem::size_of::<u64>());

    0
}

unsafe fn test_random_params() -> c_int {
    let valid_eiws: [u8; 8] = [0, 1, 2, 3, 4, 8, 9, 10];
    let valid_eigs: [u16; 7] = [0, 1, 2, 3, 4, 5, 6];
    let mut i: c_int;
    let mut ways: c_int = 0;
    let mut pos: c_int;
    let mut reverse_pos: c_int;
    let mut dpa: u64;
    let mut hpa: u64;
    let mut reverse_dpa: u64;
    let iterations: c_int = 10000;
    let mut failures: c_int = 0;

    i = 0;
    while i < iterations {
        /* Generate valid random parameters for eiw, eig, pos, dpa */
        let eiw: u8 = valid_eiws[(get_random_u32() as usize) % valid_eiws.len()];
        let eig: u16 = valid_eigs[(get_random_u32() as usize) % valid_eigs.len()];

        eiw_to_ways(eiw, &mut ways);
        pos = (get_random_u32() % ways as u32) as c_int;
        dpa = get_random_u64() >> 12;

        reverse_dpa = ULLONG_MAX;
        reverse_pos = -1;

        hpa = cxl_calculate_hpa_offset(dpa, pos, eiw, eig);
        if hpa != ULLONG_MAX {
            reverse_dpa = cxl_calculate_dpa_offset(hpa, eiw, eig);
            reverse_pos = cxl_calculate_position(hpa, eiw, eig) as c_int;
            if reverse_dpa == dpa && reverse_pos == pos {
                i += 1;
                continue;
            }
        }

        pr_err(b"test random iter %d FAIL hpa=%llu, dpa=%llu reverse_dpa=%llu, pos=%d reverse_pos=%d eiw=%u eig=%u\n\0".as_ptr() as *const c_char,
               i, hpa, dpa, reverse_dpa, pos, reverse_pos, eiw as c_uint, eig as c_uint);

        if failures > 10 {
            failures += 1;
            pr_err(b"test random too many failures, stop\n\0".as_ptr() as *const c_char);
            break;
        }
        failures += 1;
        i += 1;
    }
    pr_info(b"..... test random: PASS %d FAIL %d\n\0".as_ptr() as *const c_char,
            i - failures, failures);

    if failures != 0 {
        return -EINVAL;
    }

    0
}

#[repr(C)]
struct param_test {
    eiw: u8,
    eig: u16,
    pos: c_int,
    expect: bool, /* true: expect pass, false: expect fail */
    desc: *const c_char,
}

static mut param_tests: [param_test; 39] = [
    param_test { eiw: 0x0, eig: 0, pos: 0, expect: true, desc: b"1-way, min eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x0, eig: 3, pos: 0, expect: true, desc: b"1-way, mid eig=3, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x0, eig: 6, pos: 0, expect: true, desc: b"1-way, max eig=6, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x1, eig: 0, pos: 0, expect: true, desc: b"2-way, eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x1, eig: 3, pos: 1, expect: true, desc: b"2-way, eig=3, max pos=1\0".as_ptr() as *const c_char },
    param_test { eiw: 0x1, eig: 6, pos: 1, expect: true, desc: b"2-way, eig=6, max pos=1\0".as_ptr() as *const c_char },
    param_test { eiw: 0x2, eig: 0, pos: 0, expect: true, desc: b"4-way, eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x2, eig: 3, pos: 3, expect: true, desc: b"4-way, eig=3, max pos=3\0".as_ptr() as *const c_char },
    param_test { eiw: 0x2, eig: 6, pos: 3, expect: true, desc: b"4-way, eig=6, max pos=3\0".as_ptr() as *const c_char },
    param_test { eiw: 0x3, eig: 0, pos: 0, expect: true, desc: b"8-way, eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x3, eig: 3, pos: 7, expect: true, desc: b"8-way, eig=3, max pos=7\0".as_ptr() as *const c_char },
    param_test { eiw: 0x3, eig: 6, pos: 7, expect: true, desc: b"8-way, eig=6, max pos=7\0".as_ptr() as *const c_char },
    param_test { eiw: 0x4, eig: 0, pos: 0, expect: true, desc: b"16-way, eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x4, eig: 3, pos: 15, expect: true, desc: b"16-way, eig=3, max pos=15\0".as_ptr() as *const c_char },
    param_test { eiw: 0x4, eig: 6, pos: 15, expect: true, desc: b"16-way, eig=6, max pos=15\0".as_ptr() as *const c_char },
    param_test { eiw: 0x8, eig: 0, pos: 0, expect: true, desc: b"3-way, eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x8, eig: 3, pos: 2, expect: true, desc: b"3-way, eig=3, max pos=2\0".as_ptr() as *const c_char },
    param_test { eiw: 0x8, eig: 6, pos: 2, expect: true, desc: b"3-way, eig=6, max pos=2\0".as_ptr() as *const c_char },
    param_test { eiw: 0x9, eig: 0, pos: 0, expect: true, desc: b"6-way, eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x9, eig: 3, pos: 5, expect: true, desc: b"6-way, eig=3, max pos=5\0".as_ptr() as *const c_char },
    param_test { eiw: 0x9, eig: 6, pos: 5, expect: true, desc: b"6-way, eig=6, max pos=5\0".as_ptr() as *const c_char },
    param_test { eiw: 0xA, eig: 0, pos: 0, expect: true, desc: b"12-way, eig=0, pos=0\0".as_ptr() as *const c_char },
    param_test { eiw: 0xA, eig: 3, pos: 11, expect: true, desc: b"12-way, eig=3, max pos=11\0".as_ptr() as *const c_char },
    param_test { eiw: 0xA, eig: 6, pos: 11, expect: true, desc: b"12-way, eig=6, max pos=11\0".as_ptr() as *const c_char },
    param_test { eiw: 0x5, eig: 0, pos: 0, expect: false, desc: b"invalid eiw=5\0".as_ptr() as *const c_char },
    param_test { eiw: 0x7, eig: 0, pos: 0, expect: false, desc: b"invalid eiw=7\0".as_ptr() as *const c_char },
    param_test { eiw: 0xB, eig: 0, pos: 0, expect: false, desc: b"invalid eiw=0xB\0".as_ptr() as *const c_char },
    param_test { eiw: 0xFF, eig: 0, pos: 0, expect: false, desc: b"invalid eiw=0xFF\0".as_ptr() as *const c_char },
    param_test { eiw: 0x1, eig: 7, pos: 0, expect: false, desc: b"invalid eig=7 (out of range)\0".as_ptr() as *const c_char },
    param_test { eiw: 0x2, eig: 0x10, pos: 0, expect: false, desc: b"invalid eig=0x10\0".as_ptr() as *const c_char },
    param_test { eiw: 0x3, eig: 0xFFFF, pos: 0, expect: false, desc: b"invalid eig=0xFFFF\0".as_ptr() as *const c_char },
    param_test { eiw: 0x1, eig: 0, pos: -1, expect: false, desc: b"pos < 0\0".as_ptr() as *const c_char },
    param_test { eiw: 0x1, eig: 0, pos: 2, expect: false, desc: b"2-way, pos=2 (>= ways)\0".as_ptr() as *const c_char },
    param_test { eiw: 0x2, eig: 0, pos: 4, expect: false, desc: b"4-way, pos=4 (>= ways)\0".as_ptr() as *const c_char },
    param_test { eiw: 0x3, eig: 0, pos: 8, expect: false, desc: b"8-way, pos=8 (>= ways)\0".as_ptr() as *const c_char },
    param_test { eiw: 0x4, eig: 0, pos: 16, expect: false, desc: b"16-way, pos=16 (>= ways)\0".as_ptr() as *const c_char },
    param_test { eiw: 0x8, eig: 0, pos: 3, expect: false, desc: b"3-way, pos=3 (>= ways)\0".as_ptr() as *const c_char },
    param_test { eiw: 0x9, eig: 0, pos: 6, expect: false, desc: b"6-way, pos=6 (>= ways)\0".as_ptr() as *const c_char },
    param_test { eiw: 0xA, eig: 0, pos: 12, expect: false, desc: b"12-way, pos=12 (>= ways)\0".as_ptr() as *const c_char },
];

unsafe fn test_cxl_validate_translation_params() -> c_int {
    let mut i: usize = 0;
    let mut rc: c_int;
    let mut failures: c_int = 0;
    let mut valid: bool;

    while i < param_tests.len() {
        let t: *mut param_test = &mut param_tests[i];

        rc = cxl_validate_translation_params((*t).eiw, (*t).eig, (*t).pos);
        valid = rc == 0;

        if valid != (*t).expect {
            pr_err(b"test params failed: %s\n\0".as_ptr() as *const c_char, (*t).desc);
            failures += 1;
        }
        i += 1;
    }
    pr_info(b"..... test params: PASS %d FAIL %d\n\0".as_ptr() as *const c_char,
            i as c_int - failures, failures);

    if failures != 0 {
        return -EINVAL;
    }

    0
}

/*
 * cxl_translate_init
 *
 * Run the internal validation tests when no params are passed.
 * Otherwise, parse the parameters (test vectors), and kick off
 * the translation test.
 *
 * Returns: 0 on success, negative error code on failure
 */
unsafe fn cxl_translate_init() -> c_int {
    let mut rc: c_int;
    let mut i: c_int;

    /* If no tables are passed, validate module params only */
    if table_num == 0 {
        pr_info(b"Internal validation test start...\n\0".as_ptr() as *const c_char);
        rc = test_cxl_validate_translation_params();
        if rc != 0 {
            return rc;
        }

        rc = test_random_params();
        if rc != 0 {
            return rc;
        }

        pr_info(b"Internal validation test completed successfully\n\0".as_ptr() as *const c_char);

        return 0;
    }

    pr_info(b"CXL translate test module loaded with %d test vectors\n\0".as_ptr() as *const c_char,
            table_num);

    rc = setup_xor_mapping();
    if rc != 0 {
        return rc;
    }

    /* Process each test vector */
    i = 0;
    while i < table_num {
        let mut dpa: u64 = 0;
        let mut expect_spa: u64 = 0;
        let mut pos: c_int = 0;
        let mut math: c_int = 0;
        let mut r_eiw: u8 = 0;
        let mut hb_ways: u8 = 0;
        let mut r_eig: u16 = 0;

        pr_debug(b"Processing test vector %d: '%s'\n\0".as_ptr() as *const c_char,
                 i, table[i as usize]);

        /* Parse the test vector */
        rc = parse_test_vector(table[i as usize], &mut dpa, &mut pos, &mut r_eiw, &mut r_eig,
                               &mut hb_ways, &mut math, &mut expect_spa);
        if rc != 0 {
            pr_err(b"CXL Translate Test %d: FAIL\n    Failed to parse test vector '%s'\n\0".as_ptr() as *const c_char,
                   i, table[i as usize]);
            i += 1;
            continue;
        }
        /* Run the translation test */
        rc = run_translation_test(dpa, pos, r_eiw, r_eig, hb_ways, math, expect_spa);
        if rc != 0 {
            pr_err(b"CXL Translate Test %d: FAIL\n    dpa=%llu pos=%d r_eiw=%u r_eig=%u hb_ways=%u math=%s expect_spa=%llu\n\0".as_ptr() as *const c_char,
                   i, dpa, pos, r_eiw as c_uint, r_eig as c_uint, hb_ways as c_uint,
                   if math == XOR_MATH {
                       b"XOR\0".as_ptr() as *const c_char
                   } else {
                       b"MODULO\0".as_ptr() as *const c_char
                   },
                   expect_spa);
        } else {
            pr_info(b"CXL Translate Test %d: PASS\n\0".as_ptr() as *const c_char, i);
        }
        i += 1;
    }

    kfree(cximsd as *mut c_void);
    pr_info(b"CXL translate test completed\n\0".as_ptr() as *const c_char);

    0
}

unsafe fn cxl_translate_exit() {
    pr_info(b"CXL translate test module unloaded\n\0".as_ptr() as *const c_char);
}

// C module declarations translated as dependency intent:
// module_param_array(table, charp, &table_num, 0444);
// MODULE_PARM_DESC(table, "Test vectors as space-separated decimal strings");
//
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("cxl_test: cxl address translation test module");
// MODULE_IMPORT_NS("CXL");
//
// module_init(cxl_translate_init);
// module_exit(cxl_translate_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
