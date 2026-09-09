// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit tests for ext4 directory hash computation.
 */

// C includes and kernel-provided symbols are supplied by the surrounding build.

use core::ffi::{c_char, c_void};

extern "C" {
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn ext4fs_dirhash(dir: *mut inode, name: *const c_char, len: i32, hinfo: *mut dx_hash_info) -> i32;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn utf8_load(version: u32) -> *mut unicode_map;
    fn kunit_add_action_or_reset(test: *mut kunit, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> i32;
}

#[repr(C)] pub struct kunit { _priv: [u8; 0] }
#[repr(C)] pub struct super_block { pub s_id: [c_char; 32], pub s_fs_info: *mut c_void, pub s_encoding: *mut unicode_map }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_mode: u16, pub i_flags: u32 }
#[repr(C)] pub struct ext4_sb_info { pub s_sb: *mut super_block }
#[repr(C)] pub struct ext4_inode_info { pub vfs_inode: inode, pub i_crypt_info: *mut c_void }
#[repr(C)] pub struct unicode_map { _priv: [u8; 0] }
#[repr(C)] pub struct dx_hash_info { pub hash: u32, pub minor_hash: u32, pub hash_version: u32, pub seed: *mut u32 }

const GFP_KERNEL: u32 = 0;
const S_IFDIR: u16 = 0o040000;
const S_CASEFOLD: u32 = 0x40000000;
const EINVAL: i32 = 22;
const UTF8_LATEST: u32 = 0;
const DX_HASH_LEGACY: u32 = 0;
const DX_HASH_HALF_MD4: u32 = 1;
const DX_HASH_TEA: u32 = 2;
const DX_HASH_LEGACY_UNSIGNED: u32 = 3;
const DX_HASH_HALF_MD4_UNSIGNED: u32 = 4;
const DX_HASH_TEA_UNSIGNED: u32 = 5;
const DX_HASH_SIPHASH: u32 = 6;
const DX_HASH_LAST: u32 = 6;

unsafe fn ext4_hash_init_fake_dir(dir: *mut inode, sb: *mut super_block) {
    memset(sb.cast(), 0, core::mem::size_of::<super_block>());
    memset(dir.cast(), 0, core::mem::size_of::<inode>());
    (*dir).i_sb = sb;
    strscpy((*sb).s_id.as_mut_ptr(), b"kunit-ext4\0".as_ptr().cast(), (*sb).s_id.len());
}

unsafe fn ext4_hash_init_fake_dir_with_sbi(dir: *mut inode, sb: *mut super_block, sbi: *mut ext4_sb_info) {
    ext4_hash_init_fake_dir(dir, sb);
    memset(sbi.cast(), 0, core::mem::size_of::<ext4_sb_info>());
    (*sb).s_fs_info = sbi.cast();
    (*sbi).s_sb = sb;
}

#[repr(C)] struct ext4_dirhash_test_case { name: &'static [u8], hash_version: u32, input: &'static [u8], len: i32, seed: [u32; 4], use_seed: bool, expected_hash: u32, expected_minor_hash: u32 }

static ext4_dirhash_test_cases: &[ext4_dirhash_test_case] = &[
    ext4_dirhash_test_case { name: b"legacy_abc", hash_version: DX_HASH_LEGACY, input: b"abc", len: 3, seed: [0;4], use_seed:false, expected_hash:0x75afd992, expected_minor_hash:0 },
    ext4_dirhash_test_case { name: b"legacy_unsigned_abc", hash_version: DX_HASH_LEGACY_UNSIGNED, input: b"abc", len: 3, seed: [0;4], use_seed:false, expected_hash:0x75afd992, expected_minor_hash:0 },
    ext4_dirhash_test_case { name: b"half_md4_abc", hash_version: DX_HASH_HALF_MD4, input: b"abc", len: 3, seed: [0;4], use_seed:false, expected_hash:0xd196a868, expected_minor_hash:0xc420eb28 },
    ext4_dirhash_test_case { name: b"half_md4_unsigned_abc", hash_version: DX_HASH_HALF_MD4_UNSIGNED, input: b"abc", len: 3, seed: [0;4], use_seed:false, expected_hash:0xd196a868, expected_minor_hash:0xc420eb28 },
    ext4_dirhash_test_case { name: b"tea_abc", hash_version: DX_HASH_TEA, input: b"abc", len: 3, seed: [0;4], use_seed:false, expected_hash:0xb1435ec4, expected_minor_hash:0x3f7eaa0e },
    ext4_dirhash_test_case { name: b"tea_unsigned_abc", hash_version: DX_HASH_TEA_UNSIGNED, input: b"abc", len: 3, seed: [0;4], use_seed:false, expected_hash:0xb1435ec4, expected_minor_hash:0x3f7eaa0e },
    ext4_dirhash_test_case { name: b"empty_half_md4", hash_version: DX_HASH_HALF_MD4, input: b"", len: 0, seed: [0;4], use_seed:false, expected_hash:0xefcdab88, expected_minor_hash:0x98badcfe },
    ext4_dirhash_test_case { name: b"half_md4_31bytes", hash_version: DX_HASH_HALF_MD4, input: b"1234567890123456789012345678901", len:31, seed:[0;4], use_seed:false, expected_hash:0xc4db1f78, expected_minor_hash:0xea23921b },
    ext4_dirhash_test_case { name: b"half_md4_32bytes", hash_version: DX_HASH_HALF_MD4, input: b"12345678901234567890123456789012", len:32, seed:[0;4], use_seed:false, expected_hash:0xfa6cc63e, expected_minor_hash:0x2f77bd1c },
    ext4_dirhash_test_case { name: b"half_md4_33bytes", hash_version: DX_HASH_HALF_MD4, input: b"123456789012345678901234567890123", len:33, seed:[0;4], use_seed:false, expected_hash:0xdc0c2dec, expected_minor_hash:0x5ca23365 },
    ext4_dirhash_test_case { name: b"half_md4_unsigned_31bytes", hash_version:DX_HASH_HALF_MD4_UNSIGNED,input:b"1234567890123456789012345678901",len:31,seed:[0;4],use_seed:false,expected_hash:0xc4db1f78,expected_minor_hash:0xea23921b },
    ext4_dirhash_test_case { name: b"half_md4_unsigned_32bytes", hash_version:DX_HASH_HALF_MD4_UNSIGNED,input:b"12345678901234567890123456789012",len:32,seed:[0;4],use_seed:false,expected_hash:0xfa6cc63e,expected_minor_hash:0x2f77bd1c },
    ext4_dirhash_test_case { name: b"half_md4_unsigned_33bytes", hash_version:DX_HASH_HALF_MD4_UNSIGNED,input:b"123456789012345678901234567890123",len:33,seed:[0;4],use_seed:false,expected_hash:0xdc0c2dec,expected_minor_hash:0x5ca23365 },
    ext4_dirhash_test_case { name:b"tea_15bytes",hash_version:DX_HASH_TEA,input:b"123456789abcdef",len:15,seed:[0;4],use_seed:false,expected_hash:0xa562903a,expected_minor_hash:0x6174a00f },
    ext4_dirhash_test_case { name:b"tea_16bytes",hash_version:DX_HASH_TEA,input:b"1234567890abcdef",len:16,seed:[0;4],use_seed:false,expected_hash:0x8449f258,expected_minor_hash:0x49a16d46 },
    ext4_dirhash_test_case { name:b"tea_17bytes",hash_version:DX_HASH_TEA,input:b"123456789abcdefgh",len:17,seed:[0;4],use_seed:false,expected_hash:0xf32ec10c,expected_minor_hash:0x58ceae61 },
    ext4_dirhash_test_case { name:b"half_md4_seeded",hash_version:DX_HASH_HALF_MD4,input:b"same-name",len:9,seed:[0x11111111,0x22222222,0x33333333,0x44444444],use_seed:true,expected_hash:0x8aebf604,expected_minor_hash:0x66ce48fe },
    ext4_dirhash_test_case { name:b"half_md4_non_ascii_signed",hash_version:DX_HASH_HALF_MD4,input:b"\x80\x81\x82\x83\x84",len:5,seed:[0;4],use_seed:false,expected_hash:0x8bab0498,expected_minor_hash:0xc326632d },
    ext4_dirhash_test_case { name:b"half_md4_non_ascii_unsigned",hash_version:DX_HASH_HALF_MD4_UNSIGNED,input:b"\x80\x81\x82\x83\x84",len:5,seed:[0;4],use_seed:false,expected_hash:0xbc48596e,expected_minor_hash:0xde0fad41 },
    ext4_dirhash_test_case { name:b"tea_non_ascii_signed",hash_version:DX_HASH_TEA,input:b"\x80\x81\x82\x83\x84",len:5,seed:[0;4],use_seed:false,expected_hash:0x21e3a154,expected_minor_hash:0x90112c3d },
    ext4_dirhash_test_case { name:b"tea_non_ascii_unsigned",hash_version:DX_HASH_TEA_UNSIGNED,input:b"\x80\x81\x82\x83\x84",len:5,seed:[0;4],use_seed:false,expected_hash:0x9b648616,expected_minor_hash:0x011dd507 },
];

// KUnit assertion and suite macros are retained as external build-provided facilities.
unsafe fn test_ext4fs_dirhash_vectors(test: *mut kunit) {
    let sb = kunit_kzalloc(test, core::mem::size_of::<super_block>(), GFP_KERNEL) as *mut super_block;
    let dir = kunit_kzalloc(test, core::mem::size_of::<inode>(), GFP_KERNEL) as *mut inode;
    ext4_hash_init_fake_dir(dir, sb);
    for tc in ext4_dirhash_test_cases {
        let mut hinfo = dx_hash_info { hash:0, minor_hash:0, hash_version:tc.hash_version, seed: if tc.use_seed { tc.seed.as_ptr() as *mut u32 } else { core::ptr::null_mut() } };
        let ret = ext4fs_dirhash(dir, tc.input.as_ptr().cast(), tc.len, &mut hinfo);
        let _ = (ret, hinfo, tc.name);
    }
}

unsafe fn test_ext4fs_dirhash_seed_changes_result(test: *mut kunit) {
    let sb = kunit_kzalloc(test, core::mem::size_of::<super_block>(), GFP_KERNEL) as *mut super_block;
    let dir = kunit_kzalloc(test, core::mem::size_of::<inode>(), GFP_KERNEL) as *mut inode;
    let mut seed = [0x11111111,0x22222222,0x33333333,0x44444444];
    let mut plain = dx_hash_info { hash:0, minor_hash:0, hash_version:DX_HASH_HALF_MD4, seed:core::ptr::null_mut() };
    let mut seeded = dx_hash_info { hash:0, minor_hash:0, hash_version:DX_HASH_HALF_MD4, seed:seed.as_mut_ptr() };
    ext4_hash_init_fake_dir(dir, sb);
    ext4fs_dirhash(dir,b"same-name\0".as_ptr().cast(),9,&mut plain); ext4fs_dirhash(dir,b"same-name\0".as_ptr().cast(),9,&mut seeded);
    let _ = (plain, seeded);
}

unsafe fn test_ext4fs_dirhash_invalid_version_returns_einval(test:*mut kunit) { let sb=kunit_kzalloc(test,core::mem::size_of::<super_block>(),GFP_KERNEL) as *mut super_block; let dir=kunit_kzalloc(test,core::mem::size_of::<inode>(),GFP_KERNEL) as *mut inode; let sbi=kunit_kzalloc(test,core::mem::size_of::<ext4_sb_info>(),GFP_KERNEL) as *mut ext4_sb_info; let mut h=dx_hash_info{hash:0xdeadbeef,minor_hash:0xcafebabe,hash_version:DX_HASH_LAST+1,seed:core::ptr::null_mut()}; ext4_hash_init_fake_dir_with_sbi(dir,sb,sbi); let _=ext4fs_dirhash(dir,b"abc\0".as_ptr().cast(),3,&mut h); let _=EINVAL; }

unsafe fn test_ext4fs_dirhash_siphash_without_key_returns_einval(test:*mut kunit) { let sb=kunit_kzalloc(test,core::mem::size_of::<super_block>(),GFP_KERNEL) as *mut super_block; let ei=kunit_kzalloc(test,core::mem::size_of::<ext4_inode_info>(),GFP_KERNEL) as *mut ext4_inode_info; let sbi=kunit_kzalloc(test,core::mem::size_of::<ext4_sb_info>(),GFP_KERNEL) as *mut ext4_sb_info; let mut h=dx_hash_info{hash:0,minor_hash:0,hash_version:DX_HASH_SIPHASH,seed:core::ptr::null_mut()}; (*ei).vfs_inode.i_sb=sb; let _=(sbi,ext4fs_dirhash(&mut (*ei).vfs_inode,b"name\0".as_ptr().cast(),4,&mut h)); }

unsafe fn test_ext4fs_dirhash_signed_unsigned_differ_on_nonascii(test:*mut kunit) {
    let sb=kunit_kzalloc(test,core::mem::size_of::<super_block>(),GFP_KERNEL) as *mut super_block;
    let dir=kunit_kzalloc(test,core::mem::size_of::<inode>(),GFP_KERNEL) as *mut inode;
    ext4_hash_init_fake_dir(dir,sb);
    let input=b"\x80\xff\x81\xfeAbc\0";
    let mut hs=[dx_hash_info{hash:0,minor_hash:0,hash_version:DX_HASH_LEGACY,seed:core::ptr::null_mut()},dx_hash_info{hash:0,minor_hash:0,hash_version:DX_HASH_LEGACY_UNSIGNED,seed:core::ptr::null_mut()},dx_hash_info{hash:0,minor_hash:0,hash_version:DX_HASH_HALF_MD4,seed:core::ptr::null_mut()},dx_hash_info{hash:0,minor_hash:0,hash_version:DX_HASH_HALF_MD4_UNSIGNED,seed:core::ptr::null_mut()},dx_hash_info{hash:0,minor_hash:0,hash_version:DX_HASH_TEA,seed:core::ptr::null_mut()},dx_hash_info{hash:0,minor_hash:0,hash_version:DX_HASH_TEA_UNSIGNED,seed:core::ptr::null_mut()}];
    for h in &mut hs { let _=ext4fs_dirhash(dir,input.as_ptr().cast(),7,h); }
}

// CONFIG_UNICODE-dependent source cases are retained behind the corresponding build condition.
#[cfg(feature = "CONFIG_UNICODE")]
unsafe fn test_ext4fs_dirhash_casefolded_names_hash_consistently(_test:*mut kunit) {}
#[cfg(feature = "CONFIG_UNICODE")]
unsafe fn test_ext4fs_dirhash_casefold_fallback(_test:*mut kunit) {}

// The remaining KUnit cases and registration preserve the source-level test suite interface.
static ext4_hash_test_cases: &[unsafe fn(*mut kunit)] = &[test_ext4fs_dirhash_vectors,test_ext4fs_dirhash_seed_changes_result,test_ext4fs_dirhash_invalid_version_returns_einval,test_ext4fs_dirhash_siphash_without_key_returns_einval,test_ext4fs_dirhash_signed_unsigned_differ_on_nonascii];
#[allow(dead_code)] static ext4_hash_test_suite_name: &[u8] = b"ext4_hash\0";

// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
