// SPDX-License-Identifier: GPL-2.0+
/*
 * linux/fs/jbd2/revoke.c
 *
 * Rust translation of the source implementation. External kernel types,
 * functions, constants, and macros are supplied by other translation units.
 */

use core::ffi::c_void;

static mut jbd2_revoke_record_cache: *mut kmem_cache = core::ptr::null_mut();
static mut jbd2_revoke_table_cache: *mut kmem_cache = core::ptr::null_mut();

#[repr(C)]
pub struct jbd2_revoke_record_s {
    pub hash: list_head,
    pub sequence: tid_t,
    pub blocknr: u64,
}

#[repr(C)]
pub struct jbd2_revoke_table_s {
    pub hash_size: i32,
    pub hash_shift: i32,
    pub hash_table: *mut list_head,
}

extern "C" {
    type kmem_cache;
    type list_head;
    type journal_t;
    type transaction_t;
    type handle_t;
    type buffer_head;
    type journal_head;
    type block_device;
    type address_space;
    type folio;
    type inode;
    type jbd2_journal_block_tail;
    type jbd2_journal_revoke_header_t;
    type gfp_t;
    type tid_t;

    static mut journal_oom_retry: i32;

    fn hash_64(block: u64, bits: i32) -> i32;
    fn kmem_cache_alloc(cache: *mut kmem_cache, flags: gfp_t) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut c_void);
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kvmalloc_objs(size: usize, count: usize) -> *mut list_head;
    fn kvfree(ptr: *mut c_void);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn init_list_head(head: *mut list_head);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn spin_lock_init(lock: *mut c_void);
    fn jbd2_journal_set_features(journal: *mut journal_t, compat: u32, ro: u32, incompat: u32) -> bool;
    fn find_get_block_nonatomic(dev: *mut block_device, block: u64, size: u32) -> *mut buffer_head;
    fn put_bh(bh: *mut buffer_head);
    fn brelse(bh: *mut buffer_head);
    fn __brelse(bh: *mut buffer_head);
    fn buffer_revokevalid(bh: *mut buffer_head) -> bool;
    fn buffer_revoked(bh: *mut buffer_head) -> bool;
    fn set_buffer_revoked(bh: *mut buffer_head);
    fn set_buffer_revokevalid(bh: *mut buffer_head);
    fn test_set_buffer_revokevalid(bh: *mut buffer_head) -> bool;
    fn test_clear_buffer_revoked(bh: *mut buffer_head) -> bool;
    fn clear_buffer_revoked(bh: *mut buffer_head);
    fn jh2bh(jh: *mut journal_head) -> *mut buffer_head;
    fn jbd2_journal_forget(handle: *mut handle_t, bh: *mut buffer_head);
    fn sb_is_blkdev_sb(sb: *mut c_void) -> bool;
    fn is_power_of_2(value: i32) -> bool;
    fn tid_gt(a: tid_t, b: tid_t) -> bool;
}

const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const EIO: i32 = 5;

unsafe fn hash(journal: *mut journal_t, block: u64) -> i32 {
    hash_64(block, (*(*journal).j_revoke).hash_shift)
}

unsafe fn insert_revoke_hash(journal: *mut journal_t, blocknr: u64, seq: tid_t) -> i32 {
    let mut gfp_mask: gfp_t = GFP_NOFS;
    if journal_oom_retry != 0 { gfp_mask |= __GFP_NOFAIL; }
    let record = kmem_cache_alloc(jbd2_revoke_record_cache, gfp_mask) as *mut jbd2_revoke_record_s;
    if record.is_null() { return -ENOMEM; }
    (*record).sequence = seq;
    (*record).blocknr = blocknr;
    let hash_list = (*(*journal).j_revoke).hash_table.add(hash(journal, blocknr) as usize);
    spin_lock(&mut (*journal).j_revoke_lock as *mut _ as *mut c_void);
    list_add(&mut (*record).hash, hash_list);
    spin_unlock(&mut (*journal).j_revoke_lock as *mut _ as *mut c_void);
    0
}

unsafe fn find_revoke_record(journal: *mut journal_t, blocknr: u64) -> *mut jbd2_revoke_record_s {
    let hash_list = (*(*journal).j_revoke).hash_table.add(hash(journal, blocknr) as usize);
    spin_lock(&mut (*journal).j_revoke_lock as *mut _ as *mut c_void);
    let mut record = (*hash_list).next as *mut jbd2_revoke_record_s;
    while !record.is_null() && (&mut (*record).hash as *mut list_head) != hash_list {
        if (*record).blocknr == blocknr { spin_unlock(&mut (*journal).j_revoke_lock as *mut _ as *mut c_void); return record; }
        record = (*(*record).hash.next as *mut jbd2_revoke_record_s).hash.next as *mut jbd2_revoke_record_s;
    }
    spin_unlock(&mut (*journal).j_revoke_lock as *mut _ as *mut c_void);
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_destroy_revoke_record_cache() { kmem_cache_destroy(jbd2_revoke_record_cache); jbd2_revoke_record_cache = core::ptr::null_mut(); }

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_destroy_revoke_table_cache() { kmem_cache_destroy(jbd2_revoke_table_cache); jbd2_revoke_table_cache = core::ptr::null_mut(); }

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_init_revoke_record_cache() -> i32 {
    jbd2_revoke_record_cache = KMEM_CACHE_RECORD();
    if jbd2_revoke_record_cache.is_null() { return -ENOMEM; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_init_revoke_table_cache() -> i32 {
    jbd2_revoke_table_cache = KMEM_CACHE_TABLE();
    if jbd2_revoke_table_cache.is_null() { return -ENOMEM; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_init_revoke_table(hash_size: i32) -> *mut jbd2_revoke_table_s {
    let table = kmem_cache_alloc(jbd2_revoke_table_cache, GFP_KERNEL) as *mut jbd2_revoke_table_s;
    if table.is_null() { return core::ptr::null_mut(); }
    let mut shift = 0;
    let mut tmp = hash_size;
    while { tmp >>= 1; tmp != 0 } { shift += 1; }
    (*table).hash_size = hash_size;
    (*table).hash_shift = shift;
    (*table).hash_table = kvmalloc_objs(core::mem::size_of::<list_head>(), hash_size as usize);
    if (*table).hash_table.is_null() { kmem_cache_free(jbd2_revoke_table_cache, table as *mut c_void); return core::ptr::null_mut(); }
    for i in 0..hash_size as usize { init_list_head((*table).hash_table.add(i)); }
    table
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_destroy_revoke_table(table: *mut jbd2_revoke_table_s) {
    for i in 0..(*table).hash_size as usize { let _ = list_empty((*table).hash_table.add(i)); }
    kvfree((*table).hash_table as *mut c_void);
    kmem_cache_free(jbd2_revoke_table_cache, table as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_init_revoke(journal: *mut journal_t, hash_size: i32) -> i32 {
    if !is_power_of_2(hash_size) { return -EINVAL; }
    (*journal).j_revoke_table[0] = jbd2_journal_init_revoke_table(hash_size);
    if (*journal).j_revoke_table[0].is_null() { return -ENOMEM; }
    (*journal).j_revoke_table[1] = jbd2_journal_init_revoke_table(hash_size);
    if (*journal).j_revoke_table[1].is_null() { jbd2_journal_destroy_revoke_table((*journal).j_revoke_table[0]); (*journal).j_revoke_table[0] = core::ptr::null_mut(); return -ENOMEM; }
    (*journal).j_revoke = (*journal).j_revoke_table[1];
    spin_lock_init(&mut (*journal).j_revoke_lock as *mut _ as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_destroy_revoke(journal: *mut journal_t) {
    (*journal).j_revoke = core::ptr::null_mut();
    if !(*journal).j_revoke_table[0].is_null() { jbd2_journal_destroy_revoke_table((*journal).j_revoke_table[0]); }
    if !(*journal).j_revoke_table[1].is_null() { jbd2_journal_destroy_revoke_table((*journal).j_revoke_table[1]); }
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_set_revoke(journal: *mut journal_t, blocknr: u64, sequence: tid_t) -> i32 {
    let record = find_revoke_record(journal, blocknr);
    if !record.is_null() { if tid_gt(sequence, (*record).sequence) { (*record).sequence = sequence; } return 0; }
    insert_revoke_hash(journal, blocknr, sequence)
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_test_revoke(journal: *mut journal_t, blocknr: u64, sequence: tid_t) -> i32 {
    let record = find_revoke_record(journal, blocknr);
    if record.is_null() || tid_gt(sequence, (*record).sequence) { 0 } else { 1 }
}

#[no_mangle]
pub unsafe extern "C" fn jbd2_journal_clear_revoke(journal: *mut journal_t) {
    let revoke = (*journal).j_revoke;
    for i in 0..(*revoke).hash_size as usize {
        let hash_list = (*revoke).hash_table.add(i);
        while !list_empty(hash_list) {
            let record = (*hash_list).next as *mut jbd2_revoke_record_s;
            list_del(&mut (*record).hash);
            kmem_cache_free(jbd2_revoke_record_cache, record as *mut c_void);
        }
    }
}

const GFP_NOFS: gfp_t = 0 as gfp_t;
const __GFP_NOFAIL: gfp_t = 0 as gfp_t;
const GFP_KERNEL: gfp_t = 0 as gfp_t;

extern "C" { fn KMEM_CACHE_RECORD() -> *mut kmem_cache; fn KMEM_CACHE_TABLE() -> *mut kmem_cache; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
