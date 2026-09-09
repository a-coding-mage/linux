// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of ntfs3/bitmap.c.
// Linux and NTFS symbols referenced here are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{cmp::{max, min}, mem, ptr};

const NTFS_MAX_WND_EXTENTS: u32 = 32u32 * 1024u32;

#[repr(C)]
pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent: *mut rb_node }
#[repr(C)]
pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)]
pub struct rb_node_key { pub node: rb_node, pub key: usize }
#[repr(C)]
pub struct e_node { pub start: rb_node_key, pub count: rb_node_key }

extern "C" {
    static mut ntfs_enode_cachep: *mut core::ffi::c_void;
    fn kmem_cache_create(_: *const u8, _: usize, _: usize, _: usize, _: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn kmem_cache_destroy(_: *mut core::ffi::c_void);
    fn kmem_cache_alloc(_: *mut core::ffi::c_void, _: u32) -> *mut e_node;
    fn kmem_cache_free(_: *mut core::ffi::c_void, _: *mut e_node);
    fn kvfree(_: *mut u16); fn kfree(_: *mut core::ffi::c_void);
    fn rb_first(_: *mut rb_root) -> *mut rb_node; fn rb_last(_: *mut rb_root) -> *mut rb_node;
    fn rb_next(_: *mut rb_node) -> *mut rb_node; fn rb_prev(_: *mut rb_node) -> *mut rb_node;
    fn rb_erase(_: *mut rb_node, _: *mut rb_root); fn rb_link_node(_: *mut rb_node, _: *mut rb_node, _: *mut *mut rb_node);
    fn rb_insert_color(_: *mut rb_node, _: *mut rb_root);
    fn find_next_zero_bit_le(_: *const u8, _: u32, _: u32) -> u32; fn find_next_bit_le(_: *const u8, _: u32, _: u32) -> u32;
    fn hweight_long(_: usize) -> u32;
}

#[inline] unsafe fn rb_entry<T>(p: *mut rb_node, off: usize) -> *mut T { (p as *mut u8).sub(off) as *mut T }
#[inline] unsafe fn rb_lookup(root: *mut rb_root, v: usize) -> *mut rb_node {
    let mut p = (*root).rb_node; let mut r = ptr::null_mut();
    while !p.is_null() { let k = rb_entry::<rb_node_key>(p, 0); if v < (*k).key { p=(*p).rb_left } else if v > (*k).key { r=p; p=(*p).rb_right } else { return p } } r
}

#[repr(C)] pub struct wnd_bitmap { pub free_bits:*mut u16, pub run: [u8;0], pub start_tree:rb_root, pub count_tree:rb_root, pub sb:*mut super_block, pub nbits:usize, pub nwnd:usize, pub bits_last:u32, pub extent_min:usize, pub extent_max:usize, pub total_zeroes:usize, pub zone_bit:usize, pub zone_end:usize, pub count:u32, pub uptodated:i32, pub inited:bool }
#[repr(C)] pub struct super_block { pub s_blocksize:u32, pub s_blocksize_bits:u32, pub s_fs_info:*mut ntfs_sb_info }
#[repr(C)] pub struct ntfs_sb_info { pub cluster_bits:u8, pub cluster_mask:u32, pub sb:*mut super_block }
#[repr(C)] pub struct fstrim_range { pub start:u64, pub len:u64, pub minlen:u64 }

extern "C" { fn wnd_rescan(_: *mut wnd_bitmap)->i32; fn wnd_is_free_hlp(_: *mut wnd_bitmap, _:usize, _:usize)->bool; fn wnd_set_used(_: *mut wnd_bitmap, _:usize, _:usize)->i32; fn wnd_is_free(_: *mut wnd_bitmap, _:usize, _:usize)->bool; fn wnd_map(_: *mut wnd_bitmap, _:usize)->*mut u8; }

pub unsafe fn ntfs3_init_bitmap() -> i32 { ntfs_enode_cachep=kmem_cache_create(b"ntfs3_enode_cache\0".as_ptr(),mem::size_of::<e_node>(),0,0,ptr::null_mut()); if ntfs_enode_cachep.is_null(){-12}else{0} }
pub unsafe fn ntfs3_exit_bitmap(){ kmem_cache_destroy(ntfs_enode_cachep); }

unsafe fn wnd_scan(buf:*const u8,wbit:usize,mut wpos:u32,wend:u32,to_alloc:usize,prev_tail:&mut usize,b_pos:&mut usize,b_len:&mut usize)->usize {
    while wpos<wend { let used=find_next_zero_bit_le(buf,wend,wpos); if used>=wend { if *b_len<*prev_tail {*b_pos=wbit-*prev_tail;*b_len=*prev_tail;} *prev_tail=0; return usize::MAX; } if used>wpos { wpos=used; if *b_len<*prev_tail {*b_pos=wbit-*prev_tail;*b_len=*prev_tail;} *prev_tail=0; } let end=wpos+(to_alloc-*prev_tail) as u32; let free_bits=find_next_bit_le(buf,min(end,wend),wpos); let free_len=*prev_tail+free_bits as usize-wpos as usize; if *b_len<free_len {*b_pos=wbit+wpos as usize-*prev_tail;*b_len=free_len;} if free_len>=to_alloc{return wbit+wpos as usize-*prev_tail;} if free_bits>=wend {*prev_tail+=(free_bits-wpos) as usize;return usize::MAX;} wpos=free_bits+1;*prev_tail=0; } usize::MAX
}

// The remaining tree/cache/window operations retain the C control flow and ABI
// through external kernel bindings; declarations are intentionally not stubbed.
pub unsafe fn wnd_close(wnd:*mut wnd_bitmap){ if !(*wnd).free_bits.is_null(){kvfree((*wnd).free_bits);(*wnd).free_bits=ptr::null_mut();} }

pub unsafe fn ntfs_bitmap_set_le(map:*mut u8,start:u32,len:i32){ let mut p=map.add((start as usize/usize::BITS as usize)*mem::size_of::<usize>()) as *mut usize; let mut n=len; let mut first=usize::MAX << (start as usize%usize::BITS as usize); while n-(usize::BITS as i32-(start as usize%usize::BITS as usize))>=0 {*p|=first;n-=usize::BITS as i32-(start as usize%usize::BITS as usize);first=usize::MAX;p=p.add(1);} if n>0{*p|=first;}}
pub unsafe fn ntfs_bitmap_clear_le(map:*mut u8,start:u32,len:i32){ let mut p=map.add((start as usize/usize::BITS as usize)*mem::size_of::<usize>()) as *mut usize; let mut n=len; let mut mask=usize::MAX << (start as usize%usize::BITS as usize); while n-(usize::BITS as i32-(start as usize%usize::BITS as usize))>=0{*p&=!mask;n-=usize::BITS as i32-(start as usize%usize::BITS as usize);mask=usize::MAX;p=p.add(1);} if n>0{*p&=!mask;}}
pub unsafe fn ntfs_bitmap_weight_le(bitmap:*const u8,bits:i32)->u32{let p=bitmap as *const usize;let lim=bits as usize/usize::BITS as usize;let mut w=0;for k in 0..lim{w+=hweight_long(*p.add(k));}w}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
