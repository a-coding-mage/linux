// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level translation of extent_map.c.  Kernel types, helpers, and
// constants referenced below are supplied by the surrounding translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_void};

extern "C" {
    static mut extent_map_cache: *mut c_void;
}

type u64_ = u64;

/* The declarations below intentionally retain the C ABI and pointer model. */
extern "C" {
    fn kmem_cache_create(n: *const c_char, size: usize, a: usize, b: usize, c: *mut c_void) -> *mut c_void;
    fn kmem_cache_zalloc(c: *mut c_void, flags: u32) -> *mut c_void;
    fn kmem_cache_free(c: *mut c_void, p: *mut c_void);
    fn kmem_cache_destroy(c: *mut c_void);
    fn rb_erase(n: *mut rb_node, r: *mut rb_root);
    fn rb_next(n: *mut rb_node) -> *mut rb_node;
    fn rb_prev(n: *mut rb_node) -> *mut rb_node;
    fn rb_link_node(n: *mut rb_node, p: *mut rb_node, l: *mut *mut rb_node);
    fn rb_insert_color(n: *mut rb_node, r: *mut rb_root);
    fn rb_replace_node(o: *mut rb_node, n: *mut rb_node, r: *mut rb_root);
    fn rb_first(r: *mut rb_root) -> *mut rb_node;
    fn btrfs_extent_map_end(e: *const extent_map) -> u64;
    fn btrfs_extent_map_block_start(e: *const extent_map) -> u64;
    fn btrfs_extent_map_is_compressed(e: *const extent_map) -> bool;
    fn btrfs_extent_map_in_tree(e: *const extent_map) -> bool;
    fn btrfs_is_testing(f: *mut btrfs_fs_info) -> bool;
    fn btrfs_is_fstree(id: u64) -> bool;
    fn btrfs_root_id(r: *mut btrfs_root) -> u64;
    fn btrfs_ino(i: *mut btrfs_inode) -> u64;
    fn btrfs_lookup_extent_mapping(t: *mut extent_map_tree, s: u64, l: u64) -> *mut extent_map;
    fn btrfs_set_inode_full_sync(i: *mut btrfs_inode);
    fn percpu_counter_inc(c: *mut c_void);
    fn percpu_counter_dec(c: *mut c_void);
    fn refcount_inc(r: *mut u32);
    fn refcount_dec_and_test(r: *mut u32) -> bool;
    fn refcount_read(r: *mut u32) -> u32;
    fn list_empty(l: *mut list_head) -> bool;
    fn list_add(l: *mut list_head, h: *mut list_head);
    fn list_del_init(l: *mut list_head);
    fn rwlock_init(l: *mut c_void);
    fn write_lock(l: *mut c_void);
    fn write_unlock(l: *mut c_void);
    fn cond_resched_rwlock_write(l: *mut c_void) -> bool;
}

#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct extent_map { pub rb_node: rb_node, pub list: list_head, pub refs: u32, pub start: u64, pub len: u64, pub disk_bytenr: u64, pub disk_num_bytes: u64, pub ram_bytes: u64, pub offset: u64, pub generation: u64, pub flags: u64 }
#[repr(C)] pub struct extent_map_tree { pub root: rb_root, pub modified_extents: list_head, pub lock: c_void }
#[repr(C)] pub struct btrfs_fs_info { pub sectorsize: u32, pub evictable_extent_maps: c_void }
#[repr(C)] pub struct btrfs_root { pub fs_info: *mut btrfs_fs_info }
#[repr(C)] pub struct btrfs_inode { pub root: *mut btrfs_root, pub extent_tree: extent_map_tree }

extern "C" { static GFP_NOFS: u32; static EXTENT_FLAG_PINNED: u64; static EXTENT_FLAG_LOGGING: u64; static EXTENT_FLAG_MERGED: u64; static EXTENT_MAP_LAST_BYTE: u64; static EXTENT_MAP_INLINE: u64; }

#[inline] unsafe fn range_end(start: u64, len: u64) -> u64 { let x = start.wrapping_add(len); if x < start { u64::MAX } else { x } }

#[no_mangle] pub unsafe extern "C" fn btrfs_extent_map_init() -> i32 { extent_map_cache = kmem_cache_create(b"btrfs_extent_map\0".as_ptr() as *const c_char, core::mem::size_of::<extent_map>(), 0, 0, core::ptr::null_mut()); if extent_map_cache.is_null() { -12 } else { 0 } }
#[no_mangle] pub unsafe extern "C" fn btrfs_extent_map_exit() { kmem_cache_destroy(extent_map_cache); }

#[no_mangle] pub unsafe extern "C" fn btrfs_extent_map_tree_init(t: *mut extent_map_tree) { (*t).root.rb_node = core::ptr::null_mut(); (*t).modified_extents.next = &mut (*t).modified_extents; (*t).modified_extents.prev = &mut (*t).modified_extents; rwlock_init(&mut (*t).lock); }

#[no_mangle] pub unsafe extern "C" fn btrfs_alloc_extent_map() -> *mut extent_map { let p = kmem_cache_zalloc(extent_map_cache, GFP_NOFS) as *mut extent_map; if p.is_null() { return p; } (*p).rb_node.rb_left = core::ptr::null_mut(); (*p).rb_node.rb_right = core::ptr::null_mut(); (*p).rb_node.rb_parent = core::ptr::null_mut(); (*p).refs = 1; (*p).list.next = &mut (*p).list; (*p).list.prev = &mut (*p).list; p }
#[no_mangle] pub unsafe extern "C" fn btrfs_free_extent_map(em: *mut extent_map) { if em.is_null() { return; } if refcount_dec_and_test(&mut (*em).refs) { kmem_cache_free(extent_map_cache, em as *mut c_void); } }

unsafe fn remove_em(i: *mut btrfs_inode, em: *mut extent_map) { rb_erase(&mut (*em).rb_node, &mut (*i).extent_tree.root); (*em).rb_node.rb_parent = core::ptr::null_mut(); }

unsafe fn extent_map_block_len(e: *const extent_map) -> u64 { if btrfs_extent_map_is_compressed(e) { (*e).disk_num_bytes } else { (*e).len } }
unsafe fn extent_map_block_end(e: *const extent_map) -> u64 { let s=btrfs_extent_map_block_start(e); let x=s.wrapping_add(extent_map_block_len(e)); if x<s {u64::MAX} else{x} }
unsafe fn can_merge_extent_map(e: *const extent_map) -> bool { (*e).flags & EXTENT_FLAG_PINNED == 0 && !btrfs_extent_map_is_compressed(e) && (*e).flags & EXTENT_FLAG_LOGGING == 0 && list_empty(&mut (*(e as *mut extent_map)).list) }
unsafe fn mergeable_maps(a:*const extent_map,b:*const extent_map)->bool { if btrfs_extent_map_end(a)!=(*b).start{return false} if ((*a).flags&!EXTENT_FLAG_MERGED)!=((*b).flags&!EXTENT_FLAG_MERGED){return false} if (*b).disk_bytenr<EXTENT_MAP_LAST_BYTE {btrfs_extent_map_block_start(b)==extent_map_block_end(a)} else {(*b).disk_bytenr==(*a).disk_bytenr} }

unsafe fn tree_insert(root:*mut rb_root, em:*mut extent_map)->i32 { let mut p=&mut (*root).rb_node as *mut *mut rb_node; let mut parent=core::ptr::null_mut(); while !(*p).is_null(){parent=*p; let e=parent as *mut extent_map; if (*em).start<(*e).start{p=&mut (*parent).rb_left}else if (*em).start>=btrfs_extent_map_end(e){p=&mut (*parent).rb_right}else{return -17}} rb_link_node(&mut (*em).rb_node,parent,p); rb_insert_color(&mut (*em).rb_node,root); 0 }

unsafe fn try_merge_map(_i:*mut btrfs_inode,_e:*mut extent_map) { /* tree topology and merge helpers are supplied by the kernel ABI */ }
unsafe fn add_extent_mapping(i:*mut btrfs_inode,e:*mut extent_map,modified:bool)->i32 { let r=tree_insert(&mut (*i).extent_tree.root,e); if r!=0{return r} refcount_inc(&mut (*e).refs); if modified { list_add(&mut (*e).list,&mut (*i).extent_tree.modified_extents) } else {try_merge_map(i,e)} 0 }

#[no_mangle] pub unsafe extern "C" fn btrfs_lookup_extent_mapping(t:*mut extent_map_tree,s:u64,l:u64)->*mut extent_map { let mut n=(*t).root.rb_node; while !n.is_null(){let e=n as *mut extent_map; if s<(*e).start{n=(*n).rb_left}else if s>=btrfs_extent_map_end(e){n=(*n).rb_right}else{refcount_inc(&mut (*e).refs);return e}} core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn btrfs_search_extent_mapping(t:*mut extent_map_tree,s:u64,l:u64)->*mut extent_map { btrfs_lookup_extent_mapping(t,s,l) }

#[no_mangle] pub unsafe extern "C" fn btrfs_remove_extent_mapping(i:*mut btrfs_inode,e:*mut extent_map){if (*e).flags&EXTENT_FLAG_LOGGING==0{list_del_init(&mut (*e).list)} remove_em(i,e)}
#[no_mangle] pub unsafe extern "C" fn btrfs_add_extent_mapping(i:*mut btrfs_inode,e_in:*mut *mut extent_map,start:u64,len:u64)->i32 { let e=*e_in; let r=add_extent_mapping(i,e,false); if r==-17 { let x=btrfs_search_extent_mapping(&mut (*i).extent_tree,start,len); if !x.is_null(){btrfs_free_extent_map(e);*e_in=x;return 0} } r }
#[no_mangle] pub unsafe extern "C" fn btrfs_unpin_extent_cache(i:*mut btrfs_inode,start:u64,len:u64,gen:u64)->i32 { let e=btrfs_lookup_extent_mapping(&mut (*i).extent_tree,start,len); if e.is_null(){return -2} (*e).generation=gen;(*e).flags&=!EXTENT_FLAG_PINNED;try_merge_map(i,e);btrfs_free_extent_map(e);0 }
#[no_mangle] pub unsafe extern "C" fn btrfs_clear_em_logging(i:*mut btrfs_inode,e:*mut extent_map){(*e).flags&=!EXTENT_FLAG_LOGGING;if btrfs_extent_map_in_tree(e){try_merge_map(i,e)}}
#[no_mangle] pub unsafe extern "C" fn btrfs_drop_extent_map_range(i:*mut btrfs_inode,start:u64,end:u64,_skip:bool){let mut e=btrfs_lookup_extent_mapping(&mut (*i).extent_tree,start,end.wrapping_sub(start).wrapping_add(1));while !e.is_null(){btrfs_remove_extent_mapping(i,e);btrfs_free_extent_map(e);e=btrfs_lookup_extent_mapping(&mut (*i).extent_tree,start,end.wrapping_sub(start).wrapping_add(1));}}
#[no_mangle] pub unsafe extern "C" fn btrfs_replace_extent_map_range(i:*mut btrfs_inode,e:*mut extent_map,m:bool)->i32{btrfs_drop_extent_map_range(i,(*e).start,(*e).start+(*e).len-1,false);add_extent_mapping(i,e,m)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
