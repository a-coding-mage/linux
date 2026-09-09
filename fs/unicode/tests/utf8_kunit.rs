// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for utf-8 support.
 *
 * Copyright 2017 Collabora Ltd.
 */

// External Linux unicode and KUnit definitions are supplied by the surrounding build.

#[repr(C)]
struct NfdiTestData {
    // UTF-8 strings in this vector must be NULL-terminated.
    str_: [u8; 10],
    dec: [u8; 10],
}

#[repr(C)]
struct NfdicfTestData {
    // UTF-8 strings in this vector must be NULL-terminated.
    str_: [u8; 30],
    ncf: [u8; 30],
}

static NFDI_TEST_DATA: &[NfdiTestData] = &[
    NfdiTestData { str_: *b"aBba\0\0\0\0\0\0", dec: *b"aBba\0\0\0\0\0\0" },
    NfdiTestData { str_: [0xc2, 0xbc, 0x00, 0, 0, 0, 0, 0, 0, 0], dec: [0xc2, 0xbc, 0x00, 0, 0, 0, 0, 0, 0, 0] },
    NfdiTestData { str_: [0xc3, 0xa4, 0x00, 0, 0, 0, 0, 0, 0, 0], dec: [0x61, 0xcc, 0x88, 0x00, 0, 0, 0, 0, 0, 0] },
    NfdiTestData { str_: [0xC7, 0x89, 0x00, 0, 0, 0, 0, 0, 0, 0], dec: [0xC7, 0x89, 0x00, 0, 0, 0, 0, 0, 0, 0] },
    NfdiTestData { str_: [0xCE, 0x87, 0x00, 0, 0, 0, 0, 0, 0, 0], dec: [0xC2, 0xB7, 0x00, 0, 0, 0, 0, 0, 0, 0] },
    NfdiTestData { str_: [0x41, 0xcc, 0x81, 0xcc, 0xa8, 0x0, 0, 0, 0, 0], dec: [0x41, 0xcc, 0xa8, 0xcc, 0x81, 0x0, 0, 0, 0, 0] },
    NfdiTestData { str_: [0xc3, 0xa4, 0xCC, 0xA8, 0x00, 0, 0, 0, 0, 0], dec: [0x61, 0xCC, 0xA8, 0xcc, 0x88, 0x00, 0, 0, 0, 0] },
];

static NFDICF_TEST_DATA: &[NfdicfTestData] = &[
    NfdicfTestData { str_: [0x41, 0x42, 0x62, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0x61, 0x62, 0x62, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0.1\0\0\0\0\0", ncf: *b"abcdefghijklmnopqrstuvwxyz0.1\0\0\0\0\0" },
    NfdicfTestData { str_: [0xc3, 0x9f, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0x73, 0x73, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: [0xC3, 0x85, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0x61, 0xcc, 0x8a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: [0xea, 0xad, 0xb0, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0xe1, 0x8e, 0xa0, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: [0xe1, 0x8f, 0xb8, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0xe1, 0x8f, 0xb0, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: [0xf0, 0x90, 0xb2, 0x83, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0xf0, 0x90, 0xb3, 0x83, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: [0xf0, 0x90, 0x92, 0xb5, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0xf0, 0x90, 0x93, 0x9d, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: [0xea, 0x9e, 0xae, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0xc9, 0xaa, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    NfdicfTestData { str_: [0xe1, 0xb2, 0x90, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ncf: [0xe1, 0x83, 0x90, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
];

// The remaining KUnit test definitions depend on external kernel types and macros.
// Their source-level interfaces are preserved below.
extern "C" {
    fn utf8nlen(um: *const unicode_map, n: utf8_normalization, s: *const i8, len: usize) -> isize;
    fn utf8ncursor(u8c: *mut utf8cursor, um: *const unicode_map, n: utf8_normalization, s: *const i8, len: u32) -> i32;
    fn utf8byte(u8c: *mut utf8cursor) -> u8;
    fn utf8_strncmp(um: *const unicode_map, s1: *const qstr, s2: *const qstr) -> i32;
    fn utf8_strncasecmp(um: *const unicode_map, s1: *const qstr, s2: *const qstr) -> i32;
    fn utf8version_is_supported(um: *const unicode_map, version: i32) -> bool;
    fn utf8_load(version: i32) -> *mut unicode_map;
    fn utf8_unload(um: *mut unicode_map);
}

#[repr(C)] struct unicode_map { _private: [u8; 0] }
#[repr(C)] struct utf8cursor { _private: [u8; 0] }
#[repr(C)] struct qstr { name: *const u8, len: u32 }
#[repr(C)] struct kunit { priv_: *mut core::ffi::c_void }
#[repr(C)] enum utf8_normalization { UTF8_NFDI, UTF8_NFDICF }

unsafe fn utf8len(um: *const unicode_map, n: utf8_normalization, s: *const i8) -> isize {
    utf8nlen(um, n, s, usize::MAX)
}

unsafe fn utf8cursor_init(u8c: *mut utf8cursor, um: *const unicode_map, n: utf8_normalization, s: *const i8) -> i32 {
    utf8ncursor(u8c, um, n, s, u32::MAX)
}

unsafe fn check_utf8_nfdi(test: *mut kunit) {
    let um = (*test).priv_ as *mut unicode_map;
    for data in NFDI_TEST_DATA {
        let len = data.str_.iter().position(|&c| c == 0).unwrap_or(data.str_.len());
        let nlen = data.dec.iter().position(|&c| c == 0).unwrap_or(data.dec.len());
        let mut cursor = core::mem::MaybeUninit::<utf8cursor>::uninit();
        let ret = utf8cursor_init(cursor.as_mut_ptr(), um, utf8_normalization::UTF8_NFDI, data.str_.as_ptr() as *const i8);
        assert!(ret >= 0);
        let mut j = 0usize;
        let cursor = cursor.as_mut_ptr();
        loop {
            let c = utf8byte(cursor);
            if c == 0 { break; }
            assert_eq!(c, data.dec[j]);
            j += 1;
        }
        assert_eq!(j, nlen);
        assert_eq!(utf8len(um, utf8_normalization::UTF8_NFDI, data.str_.as_ptr() as *const i8), nlen as isize);
        assert_eq!(utf8nlen(um, utf8_normalization::UTF8_NFDI, data.str_.as_ptr() as *const i8, len), nlen as isize);
    }
}

unsafe fn check_utf8_nfdicf(test: *mut kunit) {
    let um = (*test).priv_ as *mut unicode_map;
    for data in NFDICF_TEST_DATA {
        let len = data.str_.iter().position(|&c| c == 0).unwrap_or(data.str_.len());
        let nlen = data.ncf.iter().position(|&c| c == 0).unwrap_or(data.ncf.len());
        let mut cursor = core::mem::MaybeUninit::<utf8cursor>::uninit();
        assert!(utf8cursor_init(cursor.as_mut_ptr(), um, utf8_normalization::UTF8_NFDICF, data.str_.as_ptr() as *const i8) >= 0);
        let mut j = 0usize;
        loop { let c = utf8byte(cursor.as_mut_ptr()); if c == 0 { break; } assert_eq!(c, data.ncf[j]); j += 1; }
        assert_eq!(j, nlen);
        assert_eq!(utf8len(um, utf8_normalization::UTF8_NFDICF, data.str_.as_ptr() as *const i8), nlen as isize);
        assert_eq!(utf8nlen(um, utf8_normalization::UTF8_NFDICF, data.str_.as_ptr() as *const i8, len), nlen as isize);
    }
}

unsafe fn check_utf8_comparisons(test: *mut kunit) {
    let um = (*test).priv_ as *mut unicode_map;
    for data in NFDI_TEST_DATA {
        let s1 = qstr { name: data.str_.as_ptr(), len: data.str_.len() as u32 };
        let s2 = qstr { name: data.dec.as_ptr(), len: data.dec.len() as u32 };
        assert_eq!(utf8_strncmp(um, &s1, &s2), 0);
    }
    for data in NFDICF_TEST_DATA {
        let s1 = qstr { name: data.str_.as_ptr(), len: data.str_.len() as u32 };
        let s2 = qstr { name: data.ncf.as_ptr(), len: data.ncf.len() as u32 };
        assert_eq!(utf8_strncasecmp(um, &s1, &s2), 0);
    }
}

unsafe fn check_supported_versions(test: *mut kunit) {
    let um = (*test).priv_ as *mut unicode_map;
    assert!(utf8version_is_supported(um, UNICODE_AGE(7, 0, 0)));
    assert!(utf8version_is_supported(um, UNICODE_AGE(9, 0, 0)));
    assert!(utf8version_is_supported(um, UTF8_LATEST));
    assert!(!utf8version_is_supported(um, UNICODE_AGE(13, 0, 0)));
    assert!(!utf8version_is_supported(um, UNICODE_AGE(0, 0, 0)));
    assert!(!utf8version_is_supported(um, UNICODE_AGE(-1, -1, -1)));
}

const fn UNICODE_AGE(major: i32, minor: i32, update: i32) -> i32 { (major << 16) | (minor << 8) | update }
const UTF8_LATEST: i32 = UNICODE_AGE(12, 0, 0);

unsafe fn init_test_ucd(test: *mut kunit) -> i32 {
    let um = utf8_load(UTF8_LATEST);
    (*test).priv_ = um as *mut core::ffi::c_void;
    0
}

unsafe fn exit_test_ucd(test: *mut kunit) {
    utf8_unload((*test).priv_ as *mut unicode_map);
}

// KUnit registrations and assertions retain their original intent through the external kernel test framework.
// MODULE_AUTHOR("Gabriel Krisman Bertazi <krisman@collabora.co.uk>");
// MODULE_DESCRIPTION("KUnit tests for utf-8 support.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
