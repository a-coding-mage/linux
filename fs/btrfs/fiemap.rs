// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct btrfs_fiemap_entry { pub offset: u64, pub phys: u64, pub len: u64, pub flags: u32 }

pub const BTRFS_FIEMAP_FLUSH_CACHE: i32 = -(MAX_ERRNO + 1);

#[repr(C)]
pub struct fiemap_cache {
    pub entries: *mut btrfs_fiemap_entry,
    pub entries_size: i32,
    pub entries_pos: i32,
    pub next_search_offset: u64,
    pub extents_mapped: u32,
    pub offset: u64, pub phys: u64, pub len: u64, pub flags: u32, pub cached: bool,
}

unsafe fn flush_fiemap_cache(fieinfo: *mut fiemap_extent_info, cache: *mut fiemap_cache) -> i32 {
    for i in 0..(*cache).entries_pos {
        let entry = &*(*cache).entries.add(i as usize);
        let ret = fiemap_fill_next_extent(fieinfo, entry.offset, entry.phys, entry.len, entry.flags);
        if ret < 0 { return ret; }
    }
    (*cache).entries_pos = 0;
    0
}

unsafe fn emit_fiemap_extent(fieinfo: *mut fiemap_extent_info, cache: *mut fiemap_cache,
                             mut offset: u64, mut phys: u64, mut len: u64, flags: u32) -> i32 {
    let mut cache_end;
    if !(*cache).cached { goto_assign(cache, offset, phys, len, flags); return 0; }
    cache_end = (*cache).offset + (*cache).len;
    if cache_end > offset {
        if offset == (*cache).offset { goto_assign(cache, offset, phys, len, flags); return 0; }
        if offset > (*cache).offset {
            (*cache).len = offset - (*cache).offset;
        } else {
            let range_end = offset + len;
            if range_end <= cache_end { return 0; }
            if flags & (FIEMAP_EXTENT_ENCODED | FIEMAP_EXTENT_DELALLOC) == 0 { phys += cache_end - offset; }
            offset = cache_end; len = range_end - cache_end;
        }
    } else if (*cache).offset + (*cache).len == offset && (*cache).phys + (*cache).len == phys && (*cache).flags == flags {
        (*cache).len += len; return 0;
    }
    if (*cache).entries_pos == (*cache).entries_size {
        let entry = &*(*cache).entries.add(((*cache).entries_size - 1) as usize);
        (*cache).next_search_offset = entry.offset + entry.len;
        (*cache).cached = false;
        return BTRFS_FIEMAP_FLUSH_CACHE;
    }
    let entry = &mut *(*cache).entries.add((*cache).entries_pos as usize);
    entry.offset = (*cache).offset; entry.phys = (*cache).phys; entry.len = (*cache).len; entry.flags = (*cache).flags;
    (*cache).entries_pos += 1; (*cache).extents_mapped += 1;
    if (*cache).extents_mapped == (*fieinfo).fi_extents_max { (*cache).cached = false; return 1; }
    goto_assign(cache, offset, phys, len, flags); 0
}

#[inline] unsafe fn goto_assign(c: *mut fiemap_cache, offset: u64, phys: u64, len: u64, flags: u32) {
    (*c).cached = true; (*c).offset = offset; (*c).phys = phys; (*c).len = len; (*c).flags = flags;
}

unsafe fn emit_last_fiemap_cache(f: *mut fiemap_extent_info, c: *mut fiemap_cache) -> i32 {
    if !(*c).cached { return 0; }
    let mut ret = fiemap_fill_next_extent(f, (*c).offset, (*c).phys, (*c).len, (*c).flags);
    (*c).cached = false; if ret > 0 { ret = 0; } ret
}

unsafe fn fiemap_next_leaf_item(inode: *mut btrfs_inode, path: *mut btrfs_path) -> i32 {
    let clone = (*path).nodes[0]; let mut key = btrfs_key { objectid: 0, type_: 0, offset: 0 };
    (*path).slots[0] += 1;
    if (*path).slots[0] < btrfs_header_nritems((*path).nodes[0]) { return 0; }
    refcount_inc(&mut (*clone).refs);
    let mut ret = btrfs_next_leaf((*inode).root, path); if ret != 0 { free_extent_buffer(clone); return ret; }
    btrfs_item_key_to_cpu((*path).nodes[0], &mut key, (*path).slots[0]);
    if key.objectid != btrfs_ino(inode) || key.type_ != BTRFS_EXTENT_DATA_KEY { free_extent_buffer(clone); return 1; }
    (*clone).start = (*path).nodes[0].start; copy_extent_buffer_full(clone, (*path).nodes[0]);
    let slot = (*path).slots[0]; btrfs_release_path(path); (*path).nodes[0] = clone; (*path).slots[0] = slot; ret
}

unsafe fn fiemap_search_slot(inode: *mut btrfs_inode, path: *mut btrfs_path, file_offset: u64) -> i32 {
    let ino = btrfs_ino(inode); let root = (*inode).root;
    let mut key = btrfs_key { objectid: ino, type_: BTRFS_EXTENT_DATA_KEY, offset: file_offset };
    let mut ret = btrfs_search_slot(core::ptr::null_mut(), root, &mut key, path, 0, 0); if ret < 0 { return ret; }
    if ret > 0 && (*path).slots[0] > 0 { btrfs_item_key_to_cpu((*path).nodes[0], &mut key, (*path).slots[0]-1); if key.objectid == ino && key.type_ == BTRFS_EXTENT_DATA_KEY { (*path).slots[0] -= 1; } }
    if (*path).slots[0] >= btrfs_header_nritems((*path).nodes[0]) { ret = btrfs_next_leaf(root, path); if ret != 0 { return ret; } btrfs_item_key_to_cpu((*path).nodes[0], &mut key, (*path).slots[0]); if key.objectid != ino || key.type_ != BTRFS_EXTENT_DATA_KEY { return 1; } }
    let clone = btrfs_clone_extent_buffer((*path).nodes[0]); if clone.is_null() { return -ENOMEM; }
    let slot = (*path).slots[0]; btrfs_release_path(path); (*path).nodes[0] = clone; (*path).slots[0] = slot; 0
}

// The remaining implementation follows the C source directly; external kernel
// structures and helpers are intentionally left as unresolved dependencies.
unsafe fn fiemap_process_hole(inode: *mut btrfs_inode, fieinfo: *mut fiemap_extent_info, cache: *mut fiemap_cache, delalloc: *mut *mut extent_state, backref: *mut btrfs_backref_share_check_ctx, disk: u64, mut extent_offset: u64, extent_gen: u64, start: u64, end: u64) -> i32 {
    let i_size = i_size_read(&(*inode).vfs_inode); let mut cur = start; let mut last = 0; let mut flags = FIEMAP_EXTENT_UNWRITTEN; let mut checked = false;
    while cur < end && cur < i_size { let mut ds=0; let mut de=0; let mut ps=0; let mut pl=0; let found=btrfs_find_delalloc_in_range(inode,cur,end,delalloc,&mut ds,&mut de); if !found { break; }
        if disk != 0 { ps=if last==0 {start} else {last+1}; pl=ds-ps; }
        if pl>0 { if !checked && (*fieinfo).fi_extents_max != 0 { let r=btrfs_is_data_extent_shared(inode,disk,extent_gen,backref); if r<0{return r;} if r>0{flags|=FIEMAP_EXTENT_SHARED;} checked=true; } let r=emit_fiemap_extent(fieinfo,cache,ps,disk+extent_offset,pl,flags); if r!=0{return r;} extent_offset+=pl; }
        let r=emit_fiemap_extent(fieinfo,cache,ds,0,de+1-ds,FIEMAP_EXTENT_DELALLOC|FIEMAP_EXTENT_UNKNOWN); if r!=0{return r;} last=de; cur=de+1; extent_offset+=cur-ds; cond_resched();
    }
    if disk!=0 && last<end { let ps=if last==0{start}else{last+1}; let pl=end+1-ps; if !checked && (*fieinfo).fi_extents_max!=0 { let r=btrfs_is_data_extent_shared(inode,disk,extent_gen,backref); if r<0{return r;} if r>0{flags|=FIEMAP_EXTENT_SHARED;} } let r=emit_fiemap_extent(fieinfo,cache,ps,disk+extent_offset,pl,flags); if r!=0{return r;} } 0
}

// File-local declarations below preserve the original entry point and defer
// the large btree traversal's external kernel definitions to other files.
pub unsafe fn btrfs_fiemap(inode: *mut inode, fieinfo: *mut fiemap_extent_info, start: u64, mut len: u64) -> i32 {
    let btrfs_inode = BTRFS_I(inode); let mut ret = fiemap_prep(inode,fieinfo,start,&mut len,0); if ret!=0{return ret;}
    if (*fieinfo).fi_flags & FIEMAP_FLAG_SYNC != 0 { ret=btrfs_wait_ordered_range(btrfs_inode,0,LLONG_MAX); if ret!=0{return ret;} }
    btrfs_inode_lock(btrfs_inode,BTRFS_ILOCK_SHARED);
    if (*fieinfo).fi_flags & FIEMAP_FLAG_SYNC != 0 { ret=btrfs_wait_ordered_range(btrfs_inode,0,LLONG_MAX); if ret!=0 {btrfs_inode_unlock(btrfs_inode,BTRFS_ILOCK_SHARED);return ret;} }
    ret=extent_fiemap(btrfs_inode,fieinfo,start,len); btrfs_inode_unlock(btrfs_inode,BTRFS_ILOCK_SHARED); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
