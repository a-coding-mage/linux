// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Fusion IO.  All rights reserved.
 */

// Kernel and btrfs declarations used by this translation are supplied by the
// surrounding build environment.

const PROCESS_UNLOCK: c_ulong = 1 << 0;
const PROCESS_RELEASE: c_ulong = 1 << 1;
const PROCESS_TEST_LOCKED: c_ulong = 1 << 2;
const STATE_FLAG_STR_LEN: usize = 256;

unsafe fn process_page_range(inode: *mut inode, start: u64, end: u64, flags: c_ulong) -> c_int {
    let mut fbatch = MaybeUninit::<folio_batch>::zeroed().assume_init();
    let mut index: pgoff_t = start >> PAGE_SHIFT;
    let end_index: pgoff_t = end >> PAGE_SHIFT;
    let mut count = 0;
    let mut loops = 0;
    folio_batch_init(&mut fbatch);
    while index <= end_index {
        let ret = filemap_get_folios_contig((*inode).i_mapping, &mut index, end_index, &mut fbatch);
        for i in 0..ret {
            let folio = fbatch.folios[i as usize];
            if flags & PROCESS_TEST_LOCKED != 0 && !folio_test_locked(folio) { count += 1; }
            if flags & PROCESS_UNLOCK != 0 && folio_test_locked(folio) { folio_unlock(folio); }
            if flags & PROCESS_RELEASE != 0 { folio_put(folio); }
        }
        folio_batch_release(&mut fbatch);
        cond_resched();
        loops += 1;
        if loops > 100000 {
            printk(KERN_ERR, cstr!("stuck in a loop, start %llu, end %llu, ret %d\n"), start, end, ret);
            break;
        }
    }
    count
}

unsafe fn extent_flag_to_str(state: *const extent_state, dest: *mut c_char) {
    *dest = 0;
    let mut cur = 0usize;
    macro_rules! flag { ($n:ident) => { if (*state).state & EXTENT_$n != 0 { cur += scnprintf(dest.add(cur), STATE_FLAG_STR_LEN - cur, cstr!("%s" concat!(stringify!($n))), if cur == 0 { cstr!("") } else { cstr!("|") }); } }; }
    flag!(DIRTY); flag!(LOCKED); flag!(DIRTY_LOG1); flag!(DIRTY_LOG2); flag!(DELALLOC);
    flag!(DEFRAG); flag!(BOUNDARY); flag!(NODATASUM); flag!(CLEAR_META_RESV);
    flag!(NEED_WAIT); flag!(NORESERVE); flag!(QGROUP_RESERVED); flag!(CLEAR_DATA_RESV);
}

unsafe fn dump_extent_io_tree(tree: *const extent_io_tree) {
    let mut node = rb_first(&(*tree).state);
    let mut flags_str = [0 as c_char; STATE_FLAG_STR_LEN];
    test_msg(cstr!("io tree content:"));
    while !node.is_null() {
        let state = rb_entry(node, extent_state, rb_node);
        extent_flag_to_str(state, flags_str.as_mut_ptr());
        test_msg(cstr!("  start=%llu len=%llu flags=%s"), (*state).start, (*state).end + 1 - (*state).start, flags_str.as_ptr());
        node = rb_next(node);
    }
}

unsafe fn test_find_delalloc(sectorsize: u32, nodesize: u32) -> c_int {
    let fs_info = btrfs_alloc_dummy_fs_info(nodesize, sectorsize);
    if fs_info.is_null() { test_std_err(TEST_ALLOC_FS_INFO); return -ENOMEM; }
    let mut root = btrfs_alloc_dummy_root(fs_info);
    let mut inode: *mut inode = null_mut();
    let mut tmp: *mut extent_io_tree;
    let mut locked_page: *mut page = null_mut();
    let max_bytes = BTRFS_MAX_EXTENT_SIZE;
    let total_dirty = 2 * max_bytes;
    let mut ret = -EINVAL;
    test_msg(cstr!("running find delalloc tests"));
    if IS_ERR(root) { test_std_err(TEST_ALLOC_ROOT); ret = PTR_ERR(root); goto!(out); }
    inode = btrfs_new_test_inode();
    if inode.is_null() { test_std_err(TEST_ALLOC_INODE); ret = -ENOMEM; goto!(out); }
    tmp = &mut BTRFS_I(inode).io_tree;
    BTRFS_I(inode).root = root;
    btrfs_extent_io_tree_init(null_mut(), tmp, IO_TREE_SELFTEST);
    let mut index: pgoff_t = 0;
    while index < total_dirty >> PAGE_SHIFT {
        let page = find_or_create_page((*inode).i_mapping, index, GFP_KERNEL);
        if page.is_null() { test_err(cstr!("failed to allocate test page")); ret = -ENOMEM; goto!(out); }
        SetPageDirty(page);
        if index != 0 { unlock_page(page); } else { get_page(page); locked_page = page; }
        index += 1;
    }
    btrfs_set_extent_bit(tmp, 0, sectorsize as u64 - 1, EXTENT_DELALLOC, null_mut());
    let mut start = 0u64; let mut end = PAGE_SIZE as u64 - 1;
    let mut found = find_lock_delalloc_range(inode, page_folio(locked_page), &mut start, &mut end);
    if !found || start != 0 || end != sectorsize as u64 - 1 { test_err(cstr!("delalloc range mismatch")); goto!(out_bits); }
    btrfs_unlock_extent(tmp, start, end, null_mut()); unlock_page(locked_page); put_page(locked_page); locked_page = null_mut();
    let test_start = SZ_64M;
    locked_page = find_lock_page((*inode).i_mapping, test_start >> PAGE_SHIFT);
    if locked_page.is_null() { test_err(cstr!("couldn't find the locked page")); goto!(out_bits); }
    btrfs_set_extent_bit(tmp, sectorsize as u64, max_bytes - 1, EXTENT_DELALLOC, null_mut());
    start = test_start; end = start + PAGE_SIZE as u64 - 1;
    found = find_lock_delalloc_range(inode, page_folio(locked_page), &mut start, &mut end);
    if !found || start != test_start || end != max_bytes - 1 || process_page_range(inode, start, end, PROCESS_TEST_LOCKED | PROCESS_UNLOCK) != 0 { test_err(cstr!("delalloc locked-page test failed")); goto!(out_bits); }
    btrfs_unlock_extent(tmp, start, end, null_mut()); put_page(locked_page); locked_page = null_mut();
    let test_start = max_bytes + sectorsize as u64;
    locked_page = find_lock_page((*inode).i_mapping, test_start >> PAGE_SHIFT);
    if locked_page.is_null() { goto!(out_bits); }
    start = test_start; end = start + PAGE_SIZE as u64 - 1;
    found = find_lock_delalloc_range(inode, page_folio(locked_page), &mut start, &mut end);
    if found { test_err(cstr!("found range when we shouldn't have")); goto!(out_bits); }
    btrfs_set_extent_bit(tmp, max_bytes, total_dirty - 1, EXTENT_DELALLOC, null_mut());
    start = test_start; end = start + PAGE_SIZE as u64 - 1;
    found = find_lock_delalloc_range(inode, page_folio(locked_page), &mut start, &mut end);
    if !found || start != test_start || end != total_dirty - 1 || process_page_range(inode, start, end, PROCESS_TEST_LOCKED | PROCESS_UNLOCK) != 0 { goto!(out_bits); }
    btrfs_unlock_extent(tmp, start, end, null_mut());
    let page = find_get_page((*inode).i_mapping, (max_bytes + SZ_1M) >> PAGE_SHIFT);
    if page.is_null() { goto!(out_bits); } ClearPageDirty(page); put_page(page);
    lock_page(locked_page); start = test_start; end = start + PAGE_SIZE as u64 - 1;
    found = find_lock_delalloc_range(inode, page_folio(locked_page), &mut start, &mut end);
    if !found || (start != test_start && end != test_start + PAGE_SIZE as u64 - 1) { goto!(out_bits); }
    if process_page_range(inode, start, end, PROCESS_TEST_LOCKED | PROCESS_UNLOCK) != 0 { goto!(out_bits); }
    ret = 0;
out_bits:
    if ret != 0 { dump_extent_io_tree(tmp); }
    btrfs_clear_extent_bit(tmp, 0, total_dirty - 1, !0u32, null_mut());
out:
    if !locked_page.is_null() { put_page(locked_page); }
    if !inode.is_null() { process_page_range(inode, 0, total_dirty - 1, PROCESS_UNLOCK | PROCESS_RELEASE); iput(inode); }
    btrfs_free_dummy_root(root); btrfs_free_dummy_fs_info(fs_info); ret
}

unsafe fn check_eb_bitmap(bitmap: *mut c_ulong, eb: *mut extent_buffer) -> c_int {
    let mut i = 0usize;
    while i < (*eb).len * BITS_PER_BYTE { let bit_set = test_bit(i, bitmap); let mut bit1_set = extent_buffer_test_bit(eb, 0, i); if bit1_set != bit_set { let mut has=0u8; read_extent_buffer(eb, &mut has as *mut _ as *mut c_void, i / BITS_PER_BYTE, 1); let expect=bitmap_get_value8(bitmap, ALIGN(i, BITS_PER_BYTE)); test_err(cstr!("bits do not match, start byte 0 bit %lu, byte %lu has 0x%02x expect 0x%02x"), i, i/BITS_PER_BYTE, has, expect); return -EINVAL; } bit1_set = extent_buffer_test_bit(eb, i / BITS_PER_BYTE, i % BITS_PER_BYTE); if bit1_set != bit_set { return -EINVAL; } i += 1; } 0
}

unsafe fn test_bitmap_set(name: *const c_char, bitmap: *mut c_ulong, eb: *mut extent_buffer, byte_start: c_ulong, bit_start: c_ulong, bit_len: c_ulong) -> c_int { bitmap_set(bitmap, byte_start * BITS_PER_BYTE + bit_start, bit_len); extent_buffer_bitmap_set(eb, byte_start, bit_start, bit_len); let ret=check_eb_bitmap(bitmap, eb); if ret < 0 { test_err(cstr!("%s test failed"), name); } ret }
unsafe fn test_bitmap_clear(name: *const c_char, bitmap: *mut c_ulong, eb: *mut extent_buffer, byte_start: c_ulong, bit_start: c_ulong, bit_len: c_ulong) -> c_int { bitmap_clear(bitmap, byte_start * BITS_PER_BYTE + bit_start, bit_len); extent_buffer_bitmap_clear(eb, byte_start, bit_start, bit_len); let ret=check_eb_bitmap(bitmap, eb); if ret < 0 { test_err(cstr!("%s test failed"), name); } ret }

unsafe fn __test_eb_bitmaps(bitmap: *mut c_ulong, eb: *mut extent_buffer) -> c_int {
    let n = (*eb).len as c_ulong; let mut ret;
    for (name, set, byte, bit, len) in [(cstr!("clear all run 1"),false,0,0,n*BITS_PER_BYTE),(cstr!("set all"),true,0,0,n*BITS_PER_BYTE),(cstr!("clear all run 2"),false,0,0,n*BITS_PER_BYTE),(cstr!("same byte set"),true,0,2,4),(cstr!("same byte partial clear"),false,0,4,1),(cstr!("cross byte set"),true,2,4,8),(cstr!("cross multi byte set"),true,4,4,24),(cstr!("cross byte clear"),false,2,6,4),(cstr!("cross multi byte clear"),false,4,6,20)] { ret=if set {test_bitmap_set(name,bitmap,eb,byte,bit,len)} else {test_bitmap_clear(name,bitmap,eb,byte,bit,len)}; if ret<0{return ret;} }
    let mut x=0u32; let mut i=0usize; while i < n as usize * BITS_PER_BYTE / 32 { x=((0x19660d_u64*x as u64+0x3c6ef35f)&0xffffffff) as u32; for j in 0..32 { if x & (1<<j)!=0 { bitmap_set(bitmap,i*32+j,1); extent_buffer_bitmap_set(eb,0,i*32+j,1); } } i+=1; } check_eb_bitmap(bitmap,eb)
}

unsafe fn test_eb_bitmaps(sectorsize:u32,nodesize:u32)->c_int { let fs=btrfs_alloc_dummy_fs_info(nodesize,sectorsize); if fs.is_null(){return -ENOMEM;} let bitmap=kmalloc(nodesize,GFP_KERNEL) as *mut c_ulong; if bitmap.is_null(){btrfs_free_dummy_fs_info(fs);return -ENOMEM;} let eb=alloc_dummy_extent_buffer(fs,0); let ret=__test_eb_bitmaps(bitmap,eb); free_extent_buffer(eb); btrfs_free_dummy_fs_info(fs); ret }

unsafe fn test_find_first_clear_extent_bit() -> c_int { let mut tree=MaybeUninit::<extent_io_tree>::zeroed().assume_init(); let mut start=0;let mut end=0; btrfs_extent_io_tree_init(null_mut(),&mut tree,IO_TREE_SELFTEST); btrfs_find_first_clear_extent_bit(&mut tree,0,&mut start,&mut end,CHUNK_TRIMMED); if start!=0||end!=!0{return -EINVAL;} btrfs_set_extent_bit(&mut tree,SZ_1M,SZ_4M-1,CHUNK_TRIMMED|CHUNK_ALLOCATED,null_mut()); btrfs_set_extent_bit(&mut tree,SZ_32M,SZ_64M-1,CHUNK_TRIMMED|CHUNK_ALLOCATED,null_mut()); btrfs_find_first_clear_extent_bit(&mut tree,12*SZ_1M,&mut start,&mut end,CHUNK_TRIMMED|CHUNK_ALLOCATED); if start!=SZ_4M||end!=SZ_32M-1{return -EINVAL;} btrfs_clear_extent_bit(&mut tree,0,!0,CHUNK_TRIMMED|CHUNK_ALLOCATED,null_mut()); 0 }

unsafe fn dump_eb_and_memory_contents(eb:*mut extent_buffer,memory:*mut c_void,test_name:*const c_char){ for i in 0..(*eb).len { let page=folio_page((*eb).folios[i>>PAGE_SHIFT],0); let addr=(page_address(page) as *mut u8).add(offset_in_page(i)); if memcmp(addr,(memory as *mut u8).add(i),1)!=0 {test_err(cstr!("%s failed"),test_name);return;} } }
unsafe fn verify_eb_and_memory(eb:*mut extent_buffer,memory:*mut c_void,name:*const c_char)->c_int { for i in 0..((*eb).len>>PAGE_SHIFT) { if memcmp((memory as *mut u8).add(i<<PAGE_SHIFT),folio_address((*eb).folios[i]),PAGE_SIZE)!=0 {dump_eb_and_memory_contents(eb,memory,name);return -EUCLEAN;} } 0 }
unsafe fn init_eb_and_memory(eb:*mut extent_buffer,memory:*mut c_void){get_random_bytes(memory,(*eb).len);write_extent_buffer(eb,memory,0,(*eb).len);}
unsafe fn test_eb_mem_ops(sectorsize:u32,nodesize:u32)->c_int { let fs=btrfs_alloc_dummy_fs_info(nodesize,sectorsize); if fs.is_null(){return -ENOMEM;} let mem=kvzalloc(nodesize,GFP_KERNEL); let eb=alloc_dummy_extent_buffer(fs,SZ_1M); if mem.is_null()||eb.is_null(){return -ENOMEM;} init_eb_and_memory(eb,mem); let mut ret=verify_eb_and_memory(eb,mem,cstr!("full eb write")); if ret==0 {memcpy(mem, (mem as *mut u8).add(16),16);memcpy_extent_buffer(eb,0,16,16);ret=verify_eb_and_memory(eb,mem,cstr!("same page non-overlapping memcpy 1"));} if ret==0 {memmove((mem as *mut u8).add(512),(mem as *mut u8).add(256),512);memmove_extent_buffer(eb,512,256,512);ret=verify_eb_and_memory(eb,mem,cstr!("same page overlapping memcpy 1"));} free_extent_buffer(eb);kvfree(mem);btrfs_free_dummy_fs_info(fs);ret }

pub unsafe fn btrfs_test_extent_io(sectorsize: u32, nodesize: u32) -> c_int { let mut ret=test_find_delalloc(sectorsize,nodesize); if ret==0 {ret=test_find_first_clear_extent_bit();} if ret==0 {ret=test_eb_bitmaps(sectorsize,nodesize);} if ret==0 {ret=test_eb_mem_ops(sectorsize,nodesize);} ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
