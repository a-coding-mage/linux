// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * Tests for libbpf's hashmap.
 *
 * Copyright (c) 2019 Facebook
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

static mut DURATION: c_int = 0;

const ENOENT: c_int = 2;
const EEXIST: c_int = 17;
const HASHMAP_ADD: c_int = 0;
const HASHMAP_SET: c_int = 1;

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: c_long,
    pub value: c_long,
    pub pkey: *const c_void,
    pub pvalue: *const c_void,
}

type HashmapHashFn = unsafe extern "C" fn(c_long, *mut c_void) -> usize;
type HashmapEqualFn = unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool;

unsafe extern "C" {
    fn hashmap__new(
        hash_fn: Option<HashmapHashFn>,
        equal_fn: Option<HashmapEqualFn>,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__insert(
        map: *mut hashmap,
        key: c_long,
        value: c_long,
        strategy: c_int,
        old_key: *mut c_long,
        old_value: *mut c_long,
    ) -> c_int;
    fn hashmap__add(map: *mut hashmap, key: c_long, value: c_long) -> c_int;
    fn hashmap__set(
        map: *mut hashmap,
        key: c_long,
        value: c_long,
        old_key: *mut c_long,
        old_value: *mut c_long,
    ) -> c_int;
    fn hashmap__update(
        map: *mut hashmap,
        key: c_long,
        value: c_long,
        old_key: *mut c_long,
        old_value: *mut c_long,
    ) -> c_int;
    fn hashmap__append(map: *mut hashmap, key: c_long, value: c_long) -> c_int;
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut c_long) -> bool;
    fn hashmap__delete(
        map: *mut hashmap,
        key: c_long,
        old_key: *mut c_long,
        old_value: *mut c_long,
    ) -> bool;
    fn hashmap__size(map: *mut hashmap) -> usize;
    fn hashmap__capacity(map: *mut hashmap) -> usize;
    fn hashmap__clear(map: *mut hashmap);

    fn hashmap__for_each_entry_next(
        map: *mut hashmap,
        entry: *mut *mut hashmap_entry,
        bkt: *mut c_int,
    ) -> bool;
    fn hashmap__for_each_entry_safe_next(
        map: *mut hashmap,
        entry: *mut *mut hashmap_entry,
        tmp: *mut *mut hashmap_entry,
        bkt: *mut c_int,
    ) -> bool;
    fn hashmap__for_each_key_entry_next(
        map: *mut hashmap,
        entry: *mut *mut hashmap_entry,
        key: c_long,
    ) -> bool;
    fn hashmap__for_each_key_entry_safe_next(
        map: *mut hashmap,
        entry: *mut *mut hashmap_entry,
        tmp: *mut *mut hashmap_entry,
        key: c_long,
    ) -> bool;

    fn str_hash(s: *mut c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe extern "C" fn hash_fn(k: c_long, _ctx: *mut c_void) -> usize {
    k as usize
}

unsafe extern "C" fn equal_fn(a: c_long, b: c_long, _ctx: *mut c_void) -> bool {
    a == b
}

#[inline]
fn next_pow_2(n: usize) -> usize {
    let mut r: usize = 1;

    while r < n {
        r <<= 1;
    }
    r
}

#[inline]
fn exp_cap(sz: usize) -> usize {
    let mut r = next_pow_2(sz);

    if sz * 4 / 3 > r {
        r <<= 1;
    }
    r
}

const ELEM_CNT: c_int = 62;

unsafe fn test_hashmap_generic() {
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let mut tmp: *mut hashmap_entry = ptr::null_mut();
    let mut err: c_int;
    let mut bkt: c_int = 0;
    let mut found_cnt: c_int;
    let mut i: c_int;
    let mut found_msk: i64;
    let map: *mut hashmap;

    map = hashmap__new(Some(hash_fn), Some(equal_fn), ptr::null_mut());
    if !ASSERT_OK_PTR(map as *const c_void, c"hashmap__new".as_ptr()) {
        return;
    }

    i = 0;
    while i < ELEM_CNT {
        let mut oldk: c_long = 0;
        let k: c_long = i as c_long;
        let mut oldv: c_long = 0;
        let v: c_long = 1024 + i as c_long;

        err = hashmap__update(map, k, v, &mut oldk, &mut oldv);
        if CHECK(
            err != -ENOENT,
            c"hashmap__update".as_ptr(),
            c"unexpected result: %d\n".as_ptr(),
            err,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }

        if i % 2 != 0 {
            err = hashmap__add(map, k, v);
        } else {
            err = hashmap__set(map, k, v, &mut oldk, &mut oldv);
            if CHECK(
                oldk != 0 || oldv != 0,
                c"check_kv".as_ptr(),
                c"unexpected k/v: %ld=%ld\n".as_ptr(),
                oldk,
                oldv,
            ) {
                goto_cleanup_hashmap_generic(map);
                return;
            }
        }

        if CHECK(
            err != 0,
            c"elem_add".as_ptr(),
            c"failed to add k/v %ld = %ld: %d\n".as_ptr(),
            k,
            v,
            err,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }

        if CHECK(
            !hashmap__find(map, k, &mut oldv),
            c"elem_find".as_ptr(),
            c"failed to find key %ld\n".as_ptr(),
            k,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        if CHECK(
            oldv != v,
            c"elem_val".as_ptr(),
            c"found value is wrong: %ld\n".as_ptr(),
            oldv,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        i += 1;
    }

    if CHECK(
        hashmap__size(map) != ELEM_CNT as usize,
        c"hashmap__size".as_ptr(),
        c"invalid map size: %zu\n".as_ptr(),
        hashmap__size(map),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }
    if CHECK(
        hashmap__capacity(map) != exp_cap(hashmap__size(map)),
        c"hashmap_cap".as_ptr(),
        c"unexpected map capacity: %zu\n".as_ptr(),
        hashmap__capacity(map),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }

    found_msk = 0;
    while hashmap__for_each_entry_next(map, &mut entry, &mut bkt) {
        let k = (*entry).key;
        let v = (*entry).value;

        found_msk |= (1_u64 << k) as i64;
        if CHECK(
            v - k != 1024,
            c"check_kv".as_ptr(),
            c"invalid k/v pair: %ld = %ld\n".as_ptr(),
            k,
            v,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
    }
    if CHECK(
        found_msk != ((1_u64 << ELEM_CNT) - 1) as i64,
        c"elem_cnt".as_ptr(),
        c"not all keys iterated: %llx\n".as_ptr(),
        found_msk,
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }

    i = 0;
    while i < ELEM_CNT {
        let mut oldk: c_long = 0;
        let k: c_long = i as c_long;
        let mut oldv: c_long = 0;
        let v: c_long = 256 + i as c_long;

        err = hashmap__add(map, k, v);
        if CHECK(
            err != -EEXIST,
            c"hashmap__add".as_ptr(),
            c"unexpected add result: %d\n".as_ptr(),
            err,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }

        if i % 2 != 0 {
            err = hashmap__update(map, k, v, &mut oldk, &mut oldv);
        } else {
            err = hashmap__set(map, k, v, &mut oldk, &mut oldv);
        }

        if CHECK(
            err != 0,
            c"elem_upd".as_ptr(),
            c"failed to update k/v %ld = %ld: %d\n".as_ptr(),
            k,
            v,
            err,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        if CHECK(
            !hashmap__find(map, k, &mut oldv),
            c"elem_find".as_ptr(),
            c"failed to find key %ld\n".as_ptr(),
            k,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        if CHECK(
            oldv != v,
            c"elem_val".as_ptr(),
            c"found value is wrong: %ld\n".as_ptr(),
            oldv,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        i += 1;
    }

    if CHECK(
        hashmap__size(map) != ELEM_CNT as usize,
        c"hashmap__size".as_ptr(),
        c"invalid updated map size: %zu\n".as_ptr(),
        hashmap__size(map),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }
    if CHECK(
        hashmap__capacity(map) != exp_cap(hashmap__size(map)),
        c"hashmap__capacity".as_ptr(),
        c"unexpected map capacity: %zu\n".as_ptr(),
        hashmap__capacity(map),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }

    found_msk = 0;
    while hashmap__for_each_entry_safe_next(map, &mut entry, &mut tmp, &mut bkt) {
        let k = (*entry).key;
        let v = (*entry).value;

        found_msk |= (1_u64 << k) as i64;
        if CHECK(
            v - k != 256,
            c"elem_check".as_ptr(),
            c"invalid updated k/v pair: %ld = %ld\n".as_ptr(),
            k,
            v,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
    }
    if CHECK(
        found_msk != ((1_u64 << ELEM_CNT) - 1) as i64,
        c"elem_cnt".as_ptr(),
        c"not all keys iterated after update: %llx\n".as_ptr(),
        found_msk,
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }

    found_cnt = 0;
    while hashmap__for_each_key_entry_next(map, &mut entry, 0) {
        found_cnt += 1;
    }
    if CHECK(
        found_cnt == 0,
        c"found_cnt".as_ptr(),
        c"didn't find any entries for key 0\n".as_ptr(),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }

    found_msk = 0;
    found_cnt = 0;
    while hashmap__for_each_key_entry_safe_next(map, &mut entry, &mut tmp, 0) {
        let mut oldk: c_long = 0;
        let k: c_long;
        let mut oldv: c_long = 0;
        let v: c_long;

        k = (*entry).key;
        v = (*entry).value;

        found_cnt += 1;
        found_msk |= (1_u64 << k) as i64;

        if CHECK(
            !hashmap__delete(map, k, &mut oldk, &mut oldv),
            c"elem_del".as_ptr(),
            c"failed to delete k/v %ld = %ld\n".as_ptr(),
            k,
            v,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        if CHECK(
            oldk != k || oldv != v,
            c"check_old".as_ptr(),
            c"invalid deleted k/v: expected %ld = %ld, got %ld = %ld\n".as_ptr(),
            k,
            v,
            oldk,
            oldv,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        if CHECK(
            hashmap__delete(map, k, &mut oldk, &mut oldv),
            c"elem_del".as_ptr(),
            c"unexpectedly deleted k/v %ld = %ld\n".as_ptr(),
            oldk,
            oldv,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
    }

    if CHECK(
        found_cnt == 0 || found_msk == 0,
        c"found_entries".as_ptr(),
        c"didn't delete any key entries\n".as_ptr(),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }
    if CHECK(
        hashmap__size(map) != (ELEM_CNT - found_cnt) as usize,
        c"elem_cnt".as_ptr(),
        c"invalid updated map size (already deleted: %d): %zu\n".as_ptr(),
        found_cnt,
        hashmap__size(map),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }
    if CHECK(
        hashmap__capacity(map) != exp_cap(hashmap__size(map)),
        c"hashmap__capacity".as_ptr(),
        c"unexpected map capacity: %zu\n".as_ptr(),
        hashmap__capacity(map),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }

    while hashmap__for_each_entry_safe_next(map, &mut entry, &mut tmp, &mut bkt) {
        let mut oldk: c_long = 0;
        let k: c_long;
        let mut oldv: c_long = 0;
        let v: c_long;

        k = (*entry).key;
        v = (*entry).value;

        found_cnt += 1;
        found_msk |= (1_u64 << k) as i64;

        if CHECK(
            !hashmap__delete(map, k, &mut oldk, &mut oldv),
            c"elem_del".as_ptr(),
            c"failed to delete k/v %ld = %ld\n".as_ptr(),
            k,
            v,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        if CHECK(
            oldk != k || oldv != v,
            c"elem_check".as_ptr(),
            c"invalid old k/v: expect %ld = %ld, got %ld = %ld\n".as_ptr(),
            k,
            v,
            oldk,
            oldv,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
        if CHECK(
            hashmap__delete(map, k, &mut oldk, &mut oldv),
            c"elem_del".as_ptr(),
            c"unexpectedly deleted k/v %ld = %ld\n".as_ptr(),
            k,
            v,
        ) {
            goto_cleanup_hashmap_generic(map);
            return;
        }
    }

    if CHECK(
        found_cnt != ELEM_CNT || found_msk != ((1_u64 << ELEM_CNT) - 1) as i64,
        c"found_cnt".as_ptr(),
        c"not all keys were deleted: found_cnt:%d, found_msk:%llx\n".as_ptr(),
        found_cnt,
        found_msk,
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }
    if CHECK(
        hashmap__size(map) != 0,
        c"hashmap__size".as_ptr(),
        c"invalid updated map size (already deleted: %d): %zu\n".as_ptr(),
        found_cnt,
        hashmap__size(map),
    ) {
        goto_cleanup_hashmap_generic(map);
        return;
    }

    found_cnt = 0;
    while hashmap__for_each_entry_next(map, &mut entry, &mut bkt) {
        CHECK(
            false,
            c"elem_exists".as_ptr(),
            c"unexpected map entries left: %ld = %ld\n".as_ptr(),
            (*entry).key,
            (*entry).value,
        );
        goto_cleanup_hashmap_generic(map);
        return;
    }

    hashmap__clear(map);
    while hashmap__for_each_entry_next(map, &mut entry, &mut bkt) {
        CHECK(
            false,
            c"elem_exists".as_ptr(),
            c"unexpected map entries left: %ld = %ld\n".as_ptr(),
            (*entry).key,
            (*entry).value,
        );
        goto_cleanup_hashmap_generic(map);
        return;
    }

    goto_cleanup_hashmap_generic(map);
}

unsafe fn goto_cleanup_hashmap_generic(map: *mut hashmap) {
    hashmap__free(map);
}

unsafe extern "C" fn str_hash_fn(a: c_long, _ctx: *mut c_void) -> usize {
    str_hash(a as *mut c_char)
}

unsafe extern "C" fn str_equal_fn(a: c_long, b: c_long, _ctx: *mut c_void) -> bool {
    strcmp(a as *mut c_char, b as *mut c_char) == 0
}

unsafe fn check_str(fn_name: *const c_char, var: *const c_char, expected: *const c_char) -> bool {
    CHECK(
        strcmp(var, expected) != 0,
        fn_name,
        c"wrong value: '%s' instead of '%s'\n".as_ptr(),
        var,
        expected,
    )
}

/* Verify that hashmap interface works with pointer keys and values */
unsafe fn test_hashmap_ptr_iface() {
    let mut key: *const c_char;
    let mut value: *const c_char = ptr::null();
    let mut old_key: *const c_char = ptr::null();
    let mut old_value: *const c_char = ptr::null();
    let mut cur: *mut hashmap_entry = ptr::null_mut();
    let map: *mut hashmap;
    let mut err: c_int;
    let mut i: c_int;
    let mut bkt: c_int = 0;

    map = hashmap__new(Some(str_hash_fn), Some(str_equal_fn), ptr::null_mut());
    if CHECK(
        map.is_null(),
        c"hashmap__new".as_ptr(),
        c"can't allocate hashmap\n".as_ptr(),
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }

    err = hashmap__insert(
        map,
        c"a".as_ptr() as c_long,
        c"apricot".as_ptr() as c_long,
        HASHMAP_ADD,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if CHECK(
        err != 0,
        c"hashmap__insert".as_ptr(),
        c"unexpected error: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }

    err = hashmap__insert(
        map,
        c"a".as_ptr() as c_long,
        c"apple".as_ptr() as c_long,
        HASHMAP_SET,
        &mut old_key as *mut *const c_char as *mut c_long,
        &mut old_value as *mut *const c_char as *mut c_long,
    );
    if CHECK(
        err != 0,
        c"hashmap__insert".as_ptr(),
        c"unexpected error: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }
    check_str(c"hashmap__update".as_ptr(), old_key, c"a".as_ptr());
    check_str(c"hashmap__update".as_ptr(), old_value, c"apricot".as_ptr());

    err = hashmap__add(map, c"b".as_ptr() as c_long, c"banana".as_ptr() as c_long);
    if CHECK(
        err != 0,
        c"hashmap__add".as_ptr(),
        c"unexpected error: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }

    err = hashmap__set(
        map,
        c"b".as_ptr() as c_long,
        c"breadfruit".as_ptr() as c_long,
        &mut old_key as *mut *const c_char as *mut c_long,
        &mut old_value as *mut *const c_char as *mut c_long,
    );
    if CHECK(
        err != 0,
        c"hashmap__set".as_ptr(),
        c"unexpected error: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }
    check_str(c"hashmap__set".as_ptr(), old_key, c"b".as_ptr());
    check_str(c"hashmap__set".as_ptr(), old_value, c"banana".as_ptr());

    err = hashmap__update(
        map,
        c"b".as_ptr() as c_long,
        c"blueberry".as_ptr() as c_long,
        &mut old_key as *mut *const c_char as *mut c_long,
        &mut old_value as *mut *const c_char as *mut c_long,
    );
    if CHECK(
        err != 0,
        c"hashmap__update".as_ptr(),
        c"unexpected error: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }
    check_str(c"hashmap__update".as_ptr(), old_key, c"b".as_ptr());
    check_str(c"hashmap__update".as_ptr(), old_value, c"breadfruit".as_ptr());

    err = hashmap__append(map, c"c".as_ptr() as c_long, c"cherry".as_ptr() as c_long);
    if CHECK(
        err != 0,
        c"hashmap__append".as_ptr(),
        c"unexpected error: %d\n".as_ptr(),
        err,
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }

    if CHECK(
        !hashmap__delete(
            map,
            c"c".as_ptr() as c_long,
            &mut old_key as *mut *const c_char as *mut c_long,
            &mut old_value as *mut *const c_char as *mut c_long,
        ),
        c"hashmap__delete".as_ptr(),
        c"expected to have entry for 'c'\n".as_ptr(),
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }
    check_str(c"hashmap__delete".as_ptr(), old_key, c"c".as_ptr());
    check_str(c"hashmap__delete".as_ptr(), old_value, c"cherry".as_ptr());

    CHECK(
        !hashmap__find(
            map,
            c"b".as_ptr() as c_long,
            &mut value as *mut *const c_char as *mut c_long,
        ),
        c"hashmap__find".as_ptr(),
        c"can't find value for 'b'\n".as_ptr(),
    );
    check_str(c"hashmap__find".as_ptr(), value, c"blueberry".as_ptr());

    if CHECK(
        !hashmap__delete(map, c"b".as_ptr() as c_long, ptr::null_mut(), ptr::null_mut()),
        c"hashmap__delete".as_ptr(),
        c"expected to have entry for 'b'\n".as_ptr(),
    ) {
        goto_cleanup_hashmap_ptr_iface(map);
        return;
    }

    i = 0;
    while hashmap__for_each_entry_next(map, &mut cur, &mut bkt) {
        if CHECK(
            i != 0,
            c"hashmap__for_each_entry".as_ptr(),
            c"too many entries".as_ptr(),
        ) {
            goto_cleanup_hashmap_ptr_iface(map);
            return;
        }
        key = (*cur).pkey as *const c_char;
        value = (*cur).pvalue as *const c_char;
        check_str(c"entry".as_ptr(), key, c"a".as_ptr());
        check_str(c"entry".as_ptr(), value, c"apple".as_ptr());
        i += 1;
    }

    goto_cleanup_hashmap_ptr_iface(map);
}

unsafe fn goto_cleanup_hashmap_ptr_iface(map: *mut hashmap) {
    hashmap__free(map);
}

unsafe extern "C" fn collision_hash_fn(_k: c_long, _ctx: *mut c_void) -> usize {
    0
}

unsafe fn test_hashmap_multimap() {
    let k1: c_long = 0;
    let k2: c_long = 1;
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let map: *mut hashmap;
    let mut found_msk: c_long;
    let mut err: c_int;
    let mut bkt: c_int = 0;

    /* force collisions */
    map = hashmap__new(Some(collision_hash_fn), Some(equal_fn), ptr::null_mut());
    if !ASSERT_OK_PTR(map as *const c_void, c"hashmap__new".as_ptr()) {
        return;
    }

    /*
     * set up multimap:
     * [0] -> 1, 2, 4;
     * [1] -> 8, 16, 32;
     */
    err = hashmap__append(map, k1, 1);
    if CHECK(err != 0, c"elem_add".as_ptr(), c"failed to add k/v: %d\n".as_ptr(), err) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }
    err = hashmap__append(map, k1, 2);
    if CHECK(err != 0, c"elem_add".as_ptr(), c"failed to add k/v: %d\n".as_ptr(), err) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }
    err = hashmap__append(map, k1, 4);
    if CHECK(err != 0, c"elem_add".as_ptr(), c"failed to add k/v: %d\n".as_ptr(), err) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }

    err = hashmap__append(map, k2, 8);
    if CHECK(err != 0, c"elem_add".as_ptr(), c"failed to add k/v: %d\n".as_ptr(), err) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }
    err = hashmap__append(map, k2, 16);
    if CHECK(err != 0, c"elem_add".as_ptr(), c"failed to add k/v: %d\n".as_ptr(), err) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }
    err = hashmap__append(map, k2, 32);
    if CHECK(err != 0, c"elem_add".as_ptr(), c"failed to add k/v: %d\n".as_ptr(), err) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }

    if CHECK(
        hashmap__size(map) != 6,
        c"hashmap_size".as_ptr(),
        c"invalid map size: %zu\n".as_ptr(),
        hashmap__size(map),
    ) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }

    /* verify global iteration still works and sees all values */
    found_msk = 0;
    while hashmap__for_each_entry_next(map, &mut entry, &mut bkt) {
        found_msk |= (*entry).value;
    }
    if CHECK(
        found_msk != (1 << 6) - 1,
        c"found_msk".as_ptr(),
        c"not all keys iterated: %lx\n".as_ptr(),
        found_msk,
    ) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }

    /* iterate values for key 1 */
    found_msk = 0;
    while hashmap__for_each_key_entry_next(map, &mut entry, k1) {
        found_msk |= (*entry).value;
    }
    if CHECK(
        found_msk != (1 | 2 | 4),
        c"found_msk".as_ptr(),
        c"invalid k1 values: %lx\n".as_ptr(),
        found_msk,
    ) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }

    /* iterate values for key 2 */
    found_msk = 0;
    while hashmap__for_each_key_entry_next(map, &mut entry, k2) {
        found_msk |= (*entry).value;
    }
    if CHECK(
        found_msk != (8 | 16 | 32),
        c"found_msk".as_ptr(),
        c"invalid k2 values: %lx\n".as_ptr(),
        found_msk,
    ) {
        goto_cleanup_hashmap_multimap(map);
        return;
    }

    goto_cleanup_hashmap_multimap(map);
}

unsafe fn goto_cleanup_hashmap_multimap(map: *mut hashmap) {
    hashmap__free(map);
}

unsafe fn test_hashmap_empty() {
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let mut bkt: c_int = 0;
    let map: *mut hashmap;
    let k: c_long = 0;

    /* force collisions */
    map = hashmap__new(Some(hash_fn), Some(equal_fn), ptr::null_mut());
    if !ASSERT_OK_PTR(map as *const c_void, c"hashmap__new".as_ptr()) {
        goto_cleanup_hashmap_empty(map);
        return;
    }

    if CHECK(
        hashmap__size(map) != 0,
        c"hashmap__size".as_ptr(),
        c"invalid map size: %zu\n".as_ptr(),
        hashmap__size(map),
    ) {
        goto_cleanup_hashmap_empty(map);
        return;
    }
    if CHECK(
        hashmap__capacity(map) != 0,
        c"hashmap__capacity".as_ptr(),
        c"invalid map capacity: %zu\n".as_ptr(),
        hashmap__capacity(map),
    ) {
        goto_cleanup_hashmap_empty(map);
        return;
    }
    if CHECK(
        hashmap__find(map, k, ptr::null_mut()),
        c"elem_find".as_ptr(),
        c"unexpected find\n".as_ptr(),
    ) {
        goto_cleanup_hashmap_empty(map);
        return;
    }
    if CHECK(
        hashmap__delete(map, k, ptr::null_mut(), ptr::null_mut()),
        c"elem_del".as_ptr(),
        c"unexpected delete\n".as_ptr(),
    ) {
        goto_cleanup_hashmap_empty(map);
        return;
    }

    while hashmap__for_each_entry_next(map, &mut entry, &mut bkt) {
        CHECK(false, c"elem_found".as_ptr(), c"unexpected iterated entry\n".as_ptr());
        goto_cleanup_hashmap_empty(map);
        return;
    }
    while hashmap__for_each_key_entry_next(map, &mut entry, k) {
        CHECK(false, c"key_found".as_ptr(), c"unexpected key entry\n".as_ptr());
        goto_cleanup_hashmap_empty(map);
        return;
    }

    goto_cleanup_hashmap_empty(map);
}

unsafe fn goto_cleanup_hashmap_empty(map: *mut hashmap) {
    hashmap__free(map);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_hashmap() {
    if test__start_subtest(c"generic".as_ptr()) {
        test_hashmap_generic();
    }
    if test__start_subtest(c"multimap".as_ptr()) {
        test_hashmap_multimap();
    }
    if test__start_subtest(c"empty".as_ptr()) {
        test_hashmap_empty();
    }
    if test__start_subtest(c"ptr_iface".as_ptr()) {
        test_hashmap_ptr_iface();
    }
}
