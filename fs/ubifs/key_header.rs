/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from key.h. */

pub unsafe fn key_mask_hash(mut hash: u32) -> u32 {
    hash &= UBIFS_S_KEY_HASH_MASK;
    if hash <= 2 {
        hash = hash.wrapping_add(3);
    }
    hash
}

pub unsafe fn key_r5_hash(s: *const i8, mut len: i32) -> u32 {
    let mut a: u32 = 0;
    let mut str_ = s;
    while len != 0 {
        let ch = *str_ as i32;
        a = a.wrapping_add((ch << 4) as u32);
        a = a.wrapping_add((ch >> 4) as u32);
        a = a.wrapping_mul(11);
        str_ = str_.add(1);
        len -= 1;
    }
    key_mask_hash(a)
}

pub unsafe fn key_test_hash(str_: *const i8, mut len: i32) -> u32 {
    let mut a: u32 = 0;
    len = core::cmp::min(len as u32, 4) as i32;
    core::ptr::copy_nonoverlapping(str_ as *const u8, &mut a as *mut u32 as *mut u8, len as usize);
    key_mask_hash(a)
}

pub unsafe fn ino_key_init(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t) {
    (*key).u32[0] = inum as u32;
    (*key).u32[1] = UBIFS_INO_KEY << UBIFS_S_KEY_BLOCK_BITS;
}

pub unsafe fn ino_key_init_flash(c: *const ubifs_info, k: *mut core::ffi::c_void, inum: ino_t) {
    let key = k as *mut ubifs_key;
    (*key).j32[0] = cpu_to_le32(inum as u32);
    (*key).j32[1] = cpu_to_le32(UBIFS_INO_KEY << UBIFS_S_KEY_BLOCK_BITS);
    core::ptr::write_bytes((k as *mut u8).add(8), 0, (UBIFS_MAX_KEY_LEN - 8) as usize);
}

pub unsafe fn lowest_ino_key(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t) {
    (*key).u32[0] = inum as u32; (*key).u32[1] = 0;
}
pub unsafe fn highest_ino_key(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t) {
    (*key).u32[0] = inum as u32; (*key).u32[1] = 0xffff_ffff;
}

pub unsafe fn dent_key_init(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t, nm: *const fscrypt_name) {
    let hash = (*c).key_hash(fname_name(nm), fname_len(nm));
    ubifs_assert(c, !(hash & !UBIFS_S_KEY_HASH_MASK));
    (*key).u32[0] = inum as u32;
    (*key).u32[1] = hash | (UBIFS_DENT_KEY << UBIFS_S_KEY_HASH_BITS);
}
pub unsafe fn dent_key_init_hash(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t, hash: u32) {
    ubifs_assert(c, !(hash & !UBIFS_S_KEY_HASH_MASK));
    (*key).u32[0] = inum as u32; (*key).u32[1] = hash | (UBIFS_DENT_KEY << UBIFS_S_KEY_HASH_BITS);
}
pub unsafe fn dent_key_init_flash(c: *const ubifs_info, k: *mut core::ffi::c_void, inum: ino_t, nm: *const fscrypt_name) {
    let key = k as *mut ubifs_key; let hash = (*c).key_hash(fname_name(nm), fname_len(nm));
    ubifs_assert(c, !(hash & !UBIFS_S_KEY_HASH_MASK));
    (*key).j32[0] = cpu_to_le32(inum as u32); (*key).j32[1] = cpu_to_le32(hash | (UBIFS_DENT_KEY << UBIFS_S_KEY_HASH_BITS));
    core::ptr::write_bytes((k as *mut u8).add(8), 0, (UBIFS_MAX_KEY_LEN - 8) as usize);
}
pub unsafe fn lowest_dent_key(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t) {
    (*key).u32[0] = inum as u32; (*key).u32[1] = UBIFS_DENT_KEY << UBIFS_S_KEY_HASH_BITS;
}

pub unsafe fn xent_key_init(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t, nm: *const fscrypt_name) {
    let hash = (*c).key_hash(fname_name(nm), fname_len(nm));
    ubifs_assert(c, !(hash & !UBIFS_S_KEY_HASH_MASK));
    (*key).u32[0] = inum as u32; (*key).u32[1] = hash | (UBIFS_XENT_KEY << UBIFS_S_KEY_HASH_BITS);
}
pub unsafe fn xent_key_init_flash(c: *const ubifs_info, k: *mut core::ffi::c_void, inum: ino_t, nm: *const fscrypt_name) {
    let key = k as *mut ubifs_key; let hash = (*c).key_hash(fname_name(nm), fname_len(nm));
    ubifs_assert(c, !(hash & !UBIFS_S_KEY_HASH_MASK));
    (*key).j32[0] = cpu_to_le32(inum as u32); (*key).j32[1] = cpu_to_le32(hash | (UBIFS_XENT_KEY << UBIFS_S_KEY_HASH_BITS));
    core::ptr::write_bytes((k as *mut u8).add(8), 0, (UBIFS_MAX_KEY_LEN - 8) as usize);
}
pub unsafe fn lowest_xent_key(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t) {
    (*key).u32[0] = inum as u32; (*key).u32[1] = UBIFS_XENT_KEY << UBIFS_S_KEY_HASH_BITS;
}
pub unsafe fn data_key_init(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t, block: u32) {
    ubifs_assert(c, !(block & !UBIFS_S_KEY_BLOCK_MASK));
    (*key).u32[0] = inum as u32; (*key).u32[1] = block | (UBIFS_DATA_KEY << UBIFS_S_KEY_BLOCK_BITS);
}
pub unsafe fn highest_data_key(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t) { data_key_init(c, key, inum, UBIFS_S_KEY_BLOCK_MASK); }
pub unsafe fn trun_key_init(c: *const ubifs_info, key: *mut ubifs_key, inum: ino_t) { (*key).u32[0] = inum as u32; (*key).u32[1] = UBIFS_TRUN_KEY << UBIFS_S_KEY_BLOCK_BITS; }
pub unsafe fn invalid_key_init(c: *const ubifs_info, key: *mut ubifs_key) { (*key).u32[0] = 0xDEADBEAF; (*key).u32[1] = UBIFS_INVALID_KEY; }

pub unsafe fn key_type(c: *const ubifs_info, key: *const ubifs_key) -> i32 { ((*key).u32[1] >> UBIFS_S_KEY_BLOCK_BITS) as i32 }
pub unsafe fn key_type_flash(c: *const ubifs_info, k: *const core::ffi::c_void) -> i32 { (le32_to_cpu((*(k as *const ubifs_key)).j32[1]) >> UBIFS_S_KEY_BLOCK_BITS) as i32 }
pub unsafe fn key_inum(c: *const ubifs_info, k: *const core::ffi::c_void) -> ino_t { (*(k as *const ubifs_key)).u32[0] as ino_t }
pub unsafe fn key_inum_flash(c: *const ubifs_info, k: *const core::ffi::c_void) -> ino_t { le32_to_cpu((*(k as *const ubifs_key)).j32[0]) as ino_t }
pub unsafe fn key_hash(c: *const ubifs_info, key: *const ubifs_key) -> u32 { (*key).u32[1] & UBIFS_S_KEY_HASH_MASK }
pub unsafe fn key_hash_flash(c: *const ubifs_info, k: *const core::ffi::c_void) -> u32 { le32_to_cpu((*(k as *const ubifs_key)).j32[1]) & UBIFS_S_KEY_HASH_MASK }
pub unsafe fn key_block(c: *const ubifs_info, key: *const ubifs_key) -> u32 { (*key).u32[1] & UBIFS_S_KEY_BLOCK_MASK }
pub unsafe fn key_block_flash(c: *const ubifs_info, k: *const core::ffi::c_void) -> u32 { le32_to_cpu((*(k as *const ubifs_key)).j32[1]) & UBIFS_S_KEY_BLOCK_MASK }

pub unsafe fn key_read(c: *const ubifs_info, from: *const core::ffi::c_void, to: *mut ubifs_key) {
    let f = from as *const ubifs_key; (*to).u32[0] = le32_to_cpu((*f).j32[0]); (*to).u32[1] = le32_to_cpu((*f).j32[1]);
}
pub unsafe fn key_write(c: *const ubifs_info, from: *const ubifs_key, to: *mut core::ffi::c_void) {
    let t = to as *mut ubifs_key; (*t).j32[0] = cpu_to_le32((*from).u32[0]); (*t).j32[1] = cpu_to_le32((*from).u32[1]);
    core::ptr::write_bytes((to as *mut u8).add(8), 0, (UBIFS_MAX_KEY_LEN - 8) as usize);
}
pub unsafe fn key_write_idx(c: *const ubifs_info, from: *const ubifs_key, to: *mut core::ffi::c_void) {
    let t = to as *mut ubifs_key; (*t).j32[0] = cpu_to_le32((*from).u32[0]); (*t).j32[1] = cpu_to_le32((*from).u32[1]);
}
pub unsafe fn key_copy(c: *const ubifs_info, from: *const ubifs_key, to: *mut ubifs_key) { (*to).u64[0] = (*from).u64[0]; }

pub unsafe fn keys_cmp(c: *const ubifs_info, key1: *const ubifs_key, key2: *const ubifs_key) -> i32 {
    if (*key1).u32[0] < (*key2).u32[0] { return -1; } if (*key1).u32[0] > (*key2).u32[0] { return 1; }
    if (*key1).u32[1] < (*key2).u32[1] { return -1; } if (*key1).u32[1] > (*key2).u32[1] { return 1; } 0
}
pub unsafe fn keys_eq(c: *const ubifs_info, key1: *const ubifs_key, key2: *const ubifs_key) -> i32 {
    if (*key1).u32[0] != (*key2).u32[0] || (*key1).u32[1] != (*key2).u32[1] { 0 } else { 1 }
}
pub unsafe fn is_hash_key(c: *const ubifs_info, key: *const ubifs_key) -> i32 {
    let ty = key_type(c, key); (ty == UBIFS_DENT_KEY || ty == UBIFS_XENT_KEY) as i32
}
pub unsafe fn key_max_inode_size(c: *const ubifs_info) -> u64 {
    match (*c).key_fmt { UBIFS_SIMPLE_KEY_FMT => (1u64 << UBIFS_S_KEY_BLOCK_BITS) * UBIFS_BLOCK_SIZE as u64, _ => 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
