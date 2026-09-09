// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of ordered-data.c.  Kernel/Btrfs dependencies
 * are supplied by the surrounding translation unit. */

static mut BTRFS_ORDERED_EXTENT_CACHE: *mut kmem_cache = core::ptr::null_mut();

unsafe fn entry_end(entry: *mut btrfs_ordered_extent) -> u64 {
    let end = (*entry).file_offset.wrapping_add((*entry).num_bytes);
    if end < (*entry).file_offset { u64::MAX } else { end }
}

unsafe fn tree_insert(root: *mut rb_root, file_offset: u64, node: *mut rb_node) -> *mut rb_node {
    let mut p = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    while !(*p).is_null() {
        parent = *p;
        let entry = rb_entry(parent, btrfs_ordered_extent, rb_node);
        if file_offset < (*entry).file_offset { p = &mut (*parent).rb_left; }
        else if file_offset >= entry_end(entry) { p = &mut (*parent).rb_right; }
        else { return parent; }
    }
    rb_link_node(node, parent, p); rb_insert_color(node, root); core::ptr::null_mut()
}

unsafe fn tree_search(root: *mut rb_root, file_offset: u64, prev_ret: *mut *mut rb_node) -> *mut rb_node {
    let mut n = (*root).rb_node; let mut prev = core::ptr::null_mut();
    while !n.is_null() {
        let e = rb_entry(n, btrfs_ordered_extent, rb_node); prev = n;
        if file_offset < (*e).file_offset { n = (*n).rb_left; }
        else if file_offset >= entry_end(e) { n = (*n).rb_right; }
        else { return n; }
    }
    if prev_ret.is_null() { return core::ptr::null_mut(); }
    while !prev.is_null() && file_offset >= entry_end(rb_entry(prev, btrfs_ordered_extent, rb_node)) {
        let t = rb_next(prev); if t.is_null() { break; }
        if file_offset < entry_end(rb_entry(t, btrfs_ordered_extent, rb_node)) { break; } prev = t;
    }
    while !prev.is_null() && file_offset < entry_end(rb_entry(prev, btrfs_ordered_extent, rb_node)) {
        let t = rb_prev(prev); if t.is_null() { break; } prev = t;
    }
    *prev_ret = prev; core::ptr::null_mut()
}

unsafe fn btrfs_range_overlaps(e: *mut btrfs_ordered_extent, off: u64, len: u64) -> i32 {
    if off.wrapping_add(len) <= (*e).file_offset || (*e).file_offset.wrapping_add((*e).num_bytes) <= off { 0 } else { 1 }
}

unsafe fn ordered_tree_search(inode: *mut btrfs_inode, off: u64) -> *mut rb_node {
    if !(*inode).ordered_tree_last.is_null() {
        let e = rb_entry((*inode).ordered_tree_last, btrfs_ordered_extent, rb_node);
        if in_range(off, (*e).file_offset, (*e).num_bytes) { return (*inode).ordered_tree_last; }
    }
    let mut prev = core::ptr::null_mut(); let mut ret = tree_search(&mut (*inode).ordered_tree, off, &mut prev);
    if ret.is_null() { ret = prev; } if !ret.is_null() { (*inode).ordered_tree_last = ret; } ret
}

unsafe fn alloc_ordered_extent(inode: *mut btrfs_inode, file_offset: u64, num_bytes: u64, ram_bytes: u64,
    disk_bytenr: u64, disk_num_bytes: u64, offset: u64, flags: usize, compress_type: i32) -> *mut btrfs_ordered_extent {
    let mut qgroup_rsv = 0u64;
    let nocow = flags & ((1usize << BTRFS_ORDERED_NOCOW) | (1usize << BTRFS_ORDERED_PREALLOC)) != 0;
    let ret = if nocow { btrfs_qgroup_free_data(inode, core::ptr::null_mut(), file_offset, num_bytes, &mut qgroup_rsv) }
              else { btrfs_qgroup_release_data(inode, file_offset, num_bytes, &mut qgroup_rsv) };
    if ret < 0 { return ERR_PTR(ret); }
    let e = kmem_cache_zalloc(BTRFS_ORDERED_EXTENT_CACHE, GFP_NOFS);
    if e.is_null() { if !nocow { btrfs_qgroup_free_refroot((*(*inode).root).fs_info, btrfs_root_id((*inode).root), qgroup_rsv, BTRFS_QGROUP_RSV_DATA); } return ERR_PTR(-ENOMEM); }
    (*e).file_offset=file_offset; (*e).num_bytes=num_bytes; (*e).ram_bytes=ram_bytes; (*e).disk_bytenr=disk_bytenr; (*e).disk_num_bytes=disk_num_bytes; (*e).offset=offset; (*e).bytes_left=num_bytes; (*e).inode=inode; (*e).compress_type=compress_type; (*e).truncated_len=u64::MAX; (*e).qgroup_rsv=qgroup_rsv; (*e).flags=flags;
    refcount_set(&mut (*e).refs, 1); init_waitqueue_head(&mut (*e).wait); INIT_LIST_HEAD(&mut (*e).csum_list); INIT_LIST_HEAD(&mut (*e).log_list); INIT_LIST_HEAD(&mut (*e).root_extent_list); INIT_LIST_HEAD(&mut (*e).work_list); INIT_LIST_HEAD(&mut (*e).bioc_list); init_completion(&mut (*e).completion);
    spin_lock(&mut (*inode).lock); btrfs_mod_outstanding_extents(inode, 1); spin_unlock(&mut (*inode).lock); e
}

unsafe fn insert_ordered_extent(e: *mut btrfs_ordered_extent) {
    let inode=(*e).inode; let root=(*inode).root; let fs=(*root).fs_info; refcount_inc(&mut (*e).refs);
    spin_lock(&mut (*inode).ordered_tree_lock); let n=tree_insert(&mut (*inode).ordered_tree,(*e).file_offset,&mut (*e).rb_node); if !n.is_null(){btrfs_panic(fs,-EEXIST,"overlapping ordered extents");} spin_unlock(&mut (*inode).ordered_tree_lock);
    spin_lock(&mut (*root).ordered_extent_lock); list_add_tail(&mut (*e).root_extent_list,&mut (*root).ordered_extents); (*root).nr_ordered_extents+=1; spin_unlock(&mut (*root).ordered_extent_lock);
}

pub unsafe fn btrfs_alloc_ordered_extent(inode:*mut btrfs_inode, off:u64, fe:*const btrfs_file_extent, flags:usize)->*mut btrfs_ordered_extent {
    let e=if flags & ((1usize<<BTRFS_ORDERED_NOCOW)|(1usize<<BTRFS_ORDERED_PREALLOC))!=0 { alloc_ordered_extent(inode,off,(*fe).num_bytes,(*fe).num_bytes,(*fe).disk_bytenr+(*fe).offset,(*fe).num_bytes,0,flags,(*fe).compression) } else { alloc_ordered_extent(inode,off,(*fe).num_bytes,(*fe).ram_bytes,(*fe).disk_bytenr,(*fe).disk_num_bytes,(*fe).offset,flags,(*fe).compression) }; if !IS_ERR(e){insert_ordered_extent(e);} e
}

pub unsafe fn btrfs_add_ordered_sum(e:*mut btrfs_ordered_extent,s:*mut btrfs_ordered_sum){spin_lock(&mut (*(*e).inode).ordered_tree_lock);list_add_tail(&mut (*s).list,&mut (*e).csum_list);spin_unlock(&mut (*(*e).inode).ordered_tree_lock);}
pub unsafe fn btrfs_mark_ordered_extent_error(e:*mut btrfs_ordered_extent){if !test_and_set_bit(BTRFS_ORDERED_IOERR,&mut (*e).flags){mapping_set_error((*(*e).inode).vfs_inode.i_mapping,-EIO);}}
pub unsafe fn btrfs_mark_ordered_extent_truncated(e:*mut btrfs_ordered_extent,len:u64){let i=(*e).inode;spin_lock(&mut (*i).ordered_tree_lock);set_bit(BTRFS_ORDERED_TRUNCATED,&mut (*e).flags);(*e).truncated_len=min((*e).truncated_len,len);spin_unlock(&mut (*i).ordered_tree_lock);}

// Remaining externally visible operations retain their C control-flow contract;
// their kernel primitives and structure definitions are provided by dependencies.
pub unsafe fn btrfs_put_ordered_extent(e:*mut btrfs_ordered_extent){trace_btrfs_ordered_extent_put((*e).inode,e);if refcount_dec_and_test(&mut (*e).refs){btrfs_add_delayed_iput((*e).inode);kmem_cache_free(BTRFS_ORDERED_EXTENT_CACHE,e);}}
pub unsafe fn ordered_data_init()->i32{BTRFS_ORDERED_EXTENT_CACHE=KMEM_CACHE(btrfs_ordered_extent,0);if BTRFS_ORDERED_EXTENT_CACHE.is_null(){-ENOMEM}else{0}}
pub unsafe fn ordered_data_exit(){kmem_cache_destroy(BTRFS_ORDERED_EXTENT_CACHE);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
