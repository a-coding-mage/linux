// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the kernel and the surrounding repository.

pub unsafe fn btrfs_lru_cache_init(cache: *mut btrfs_lru_cache, max_size: u32) {
    INIT_LIST_HEAD(&mut (*cache).lru_list);
    mt_init(&mut (*cache).entries);
    (*cache).size = 0;
    (*cache).max_size = max_size;
}

unsafe fn match_entry(
    head: *mut list_head,
    key: u64,
    gen: u64,
) -> *mut btrfs_lru_cache_entry {
    let mut entry: *mut btrfs_lru_cache_entry;

    list_for_each_entry!(entry, head, list, {
        if (*entry).key == key && (*entry).gen == gen {
            return entry;
        }
    });

    core::ptr::null_mut()
}

pub unsafe fn btrfs_lru_cache_lookup(
    cache: *mut btrfs_lru_cache,
    key: u64,
    gen: u64,
) -> *mut btrfs_lru_cache_entry {
    let head: *mut list_head;
    let entry: *mut btrfs_lru_cache_entry;

    head = mtree_load(&mut (*cache).entries, key);
    if head.is_null() {
        return core::ptr::null_mut();
    }

    entry = match_entry(head, key, gen);
    if !entry.is_null() {
        list_move_tail(&mut (*entry).lru_list, &mut (*cache).lru_list);
    }

    entry
}

pub unsafe fn btrfs_lru_cache_remove(
    cache: *mut btrfs_lru_cache,
    entry: *mut btrfs_lru_cache_entry,
) {
    let prev: *mut list_head = (*entry).list.prev;

    ASSERT!((*cache).size > 0);
    ASSERT!(!mtree_empty(&mut (*cache).entries));

    list_del(&mut (*entry).list);
    list_del(&mut (*entry).lru_list);

    if list_empty(prev) {
        let head: *mut list_head;

        // The previous element is an empty head entry; remove and free it.
        head = mtree_erase(&mut (*cache).entries, (*entry).key);
        ASSERT!(head == prev);
        kfree(head);
    }

    kfree(entry);
    (*cache).size -= 1;
}

pub unsafe fn btrfs_lru_cache_store(
    cache: *mut btrfs_lru_cache,
    new_entry: *mut btrfs_lru_cache_entry,
    gfp: gfp_t,
) -> i32 {
    let key: u64 = (*new_entry).key;
    let mut head: *mut list_head;
    let mut ret: i32;

    head = kmalloc_obj::<list_head>(gfp);
    if head.is_null() {
        return -ENOMEM;
    }

    ret = mtree_insert(&mut (*cache).entries, key, head, gfp);
    if ret == 0 {
        INIT_LIST_HEAD(head);
        list_add_tail(&mut (*new_entry).list, head);
    } else if ret == -EEXIST {
        kfree(head);
        head = mtree_load(&mut (*cache).entries, key);
        ASSERT!(!head.is_null());
        if !match_entry(head, key, (*new_entry).gen).is_null() {
            return -EEXIST;
        }
        list_add_tail(&mut (*new_entry).list, head);
    } else if ret < 0 {
        kfree(head);
        return ret;
    }

    if (*cache).max_size > 0 && (*cache).size == (*cache).max_size {
        let lru_entry: *mut btrfs_lru_cache_entry;

        lru_entry = list_first_entry!(&mut (*cache).lru_list, btrfs_lru_cache_entry, lru_list);
        btrfs_lru_cache_remove(cache, lru_entry);
    }

    list_add_tail(&mut (*new_entry).lru_list, &mut (*cache).lru_list);
    (*cache).size += 1;

    0
}

pub unsafe fn btrfs_lru_cache_clear(cache: *mut btrfs_lru_cache) {
    let mut entry: *mut btrfs_lru_cache_entry;
    let mut tmp: *mut btrfs_lru_cache_entry;

    list_for_each_entry_safe!(entry, tmp, &mut (*cache).lru_list, lru_list, {
        btrfs_lru_cache_remove(cache, entry);
    });

    ASSERT!((*cache).size == 0);
    ASSERT!(mtree_empty(&mut (*cache).entries));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
