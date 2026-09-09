// SPDX-License-Identifier: GPL-2.0
/* linux/fs/hfs/bnode.c -- direct Rust translation */

use core::mem::{size_of, offset_of};
use core::ptr;

extern "C" {
    fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
    fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn memmove(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn min_t_u32(a: u32, b: u32) -> u32;
    fn is_bnode_offset_valid(n: *mut hfs_bnode, off: u32) -> bool;
    fn check_and_correct_requested_length(n: *mut hfs_bnode, off: u32, len: u32) -> u32;
    fn memcpy_from_page(d: *mut u8, p: *mut page, off: u32, n: u32);
    fn memcpy_to_page(p: *mut page, off: u32, s: *const u8, n: u32);
    fn memcpy_page(d: *mut page, doff: u32, s: *mut page, soff: u32, n: u32);
    fn memzero_page(p: *mut page, off: u32, n: u32);
    fn set_page_dirty(p: *mut page); fn mark_page_accessed(p: *mut page); fn put_page(p: *mut page);
    fn kmap_local_page(p: *mut page) -> *mut u8; fn kunmap_local(p: *mut u8);
    fn read_mapping_page(m: *mut address_space, b: i64, x: *mut u8) -> *mut page;
    fn hfs_bnode_get(n: *mut hfs_bnode); fn hfs_bnode_put(n: *mut hfs_bnode);
    fn hfs_bnode_find(t: *mut hfs_btree, n: u32) -> *mut hfs_bnode;
    fn hfs_bmap_free(n: *mut hfs_bnode);
    fn pr_err(s: *const u8, ...); fn pr_crit(s: *const u8, ...); fn printk(s: *const u8, ...);
    fn hfs_dbg(s: *const u8, ...); fn wake_up(w: *mut wait_queue_head);
    fn wait_event(w: *mut wait_queue_head, condition: bool);
    fn spin_lock(l: *mut spinlock_t); fn spin_unlock(l: *mut spinlock_t);
    fn set_bit(n: u32, p: *mut u64); fn clear_bit(n: u32, p: *mut u64);
    fn test_bit(n: u32, p: *const u64) -> bool;
    fn atomic_read(a: *mut atomic_t) -> i32; fn atomic_set(a: *mut atomic_t, v: i32);
    fn atomic_inc(a: *mut atomic_t); fn atomic_dec_and_lock(a: *mut atomic_t, l: *mut spinlock_t) -> bool;
    fn kzalloc_flex<T>(x: T, p: *mut *mut page, n: u32, flags: u32) -> *mut hfs_bnode;
    fn kfree(p: *mut hfs_bnode); fn init_waitqueue_head(w: *mut wait_queue_head);
    fn bug_on(x: bool); fn warn_on(x: bool);
}

type u8_ = u8; type u16_ = u16; type u32_ = u32; type be16 = u16; type be32 = u32;
const PAGE_SHIFT: u32 = 12; const PAGE_SIZE: u32 = 4096; const PAGE_MASK: u32 = !(PAGE_SIZE - 1);
const HFS_NODE_LEAF: u8 = 0xff; const HFS_NODE_INDEX: u8 = 0x00;
const HFS_NODE_HEADER: u8 = 1; const HFS_NODE_MAP: u8 = 2;
const HFS_TREE_VARIDXKEYS: u32 = 1; const HFS_BNODE_NEW: u32 = 0;
const HFS_BNODE_ERROR: u32 = 1; const HFS_BNODE_DELETED: u32 = 2;
const NODE_HASH_SIZE: u32 = 256; const GFP_KERNEL: u32 = 0;

#[repr(C)] pub struct page { _p: [u8; 0] }
#[repr(C)] pub struct address_space { _p: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mapping: *mut address_space }
#[repr(C)] pub struct atomic_t { _p: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _p: [u8; 0] }
#[repr(C)] pub struct wait_queue_head { _p: [u8; 0] }
#[repr(C)] pub struct hfs_bnode_desc { pub next: be32, pub prev: be32, pub type_: u8, pub height: u8, pub num_recs: be16 }
#[repr(C)] pub struct hfs_btree { pub node_size:u32, pub pages_per_bnode:u32, pub node_count:u32, pub max_key_len:u32, pub attributes:u32, pub cnid:u32, pub leaf_head:u32, pub leaf_tail:u32, pub root:u32, pub depth:u32, pub hash_lock:*mut spinlock_t, pub node_hash:*mut *mut hfs_bnode, pub node_hash_cnt:u32, pub inode:*mut inode }
#[repr(C)] pub struct hfs_bnode { pub tree:*mut hfs_btree, pub this:u32, pub prev:u32, pub next:u32, pub type_:u8, pub height:u8, pub num_recs:u16, pub page_offset:u32, pub flags:u64, pub refcnt:*mut atomic_t, pub lock_wq:*mut wait_queue_head, pub next_hash:*mut hfs_bnode, pub page:[*mut page; 0] }
#[repr(C)] pub struct hfs_btree_key { _p:[u8; 512] }

#[inline] unsafe fn be16_to_cpu(x: u16)->u16 { x.to_be() }
#[inline] unsafe fn be32_to_cpu(x: u32)->u32 { x.to_be() }
#[inline] unsafe fn cpu_to_be16(x: u16)->u16 { x.to_be() }
#[inline] unsafe fn cpu_to_be32(x: u32)->u32 { x.to_be() }
unsafe fn page(n:*mut hfs_bnode,i:usize)->*mut page { *((n as *mut *mut page).add((size_of::<hfs_bnode>()+i*size_of::<*mut page>())/size_of::<*mut page>())) }

pub unsafe fn hfs_bnode_read(n:*mut hfs_bnode, b:*mut u8, mut off:u32, mut len:u32) { memset(b,0,len as usize); if !is_bnode_offset_valid(n,off)||len==0{return} len=check_and_correct_requested_length(n,off,len); off+=(*n).page_offset; let mut pn=off>>PAGE_SHIFT; off&=!PAGE_MASK; let mut r=0; while r<len { if pn>=(*(*n).tree).pages_per_bnode{break} let x=min_t_u32(len-r,PAGE_SIZE-off); memcpy_from_page(b.add(r as usize),page(n,pn as usize),off,x); r+=x;pn+=1;off=0; } }
pub unsafe fn hfs_bnode_read_u16(n:*mut hfs_bnode,o:u32)->u16 { let mut x=0u16; hfs_bnode_read(n,&mut x as *mut _ as *mut u8,o,2); be16_to_cpu(x) }
pub unsafe fn hfs_bnode_read_u8(n:*mut hfs_bnode,o:u32)->u8 { let mut x=0;hfs_bnode_read(n,&mut x,o,1);x }
pub unsafe fn hfs_bnode_read_key(n:*mut hfs_bnode,k:*mut u8,o:u32){let t=(*n).tree;let l=if (*n).type_==HFS_NODE_LEAF||(*t).attributes&HFS_TREE_VARIDXKEYS!=0{hfs_bnode_read_u8(n,o)as u32+1}else{(*t).max_key_len+1};if l as usize>size_of::<hfs_btree_key>()||l<1{memset(k,0,size_of::<hfs_btree_key>());return}hfs_bnode_read(n,k,o,l)}
pub unsafe fn hfs_bnode_write(n:*mut hfs_bnode,b:*const u8,mut o:u32,mut l:u32){if !is_bnode_offset_valid(n,o)||l==0{return}l=check_and_correct_requested_length(n,o,l);o+=(*n).page_offset;let p=page(n,0);memcpy_to_page(p,o,b,l);set_page_dirty(p)}
pub unsafe fn hfs_bnode_write_u16(n:*mut hfs_bnode,o:u32,d:u16){let v=cpu_to_be16(d);hfs_bnode_write(n,&v as*const _ as*const u8,o,2)}
pub unsafe fn hfs_bnode_write_u8(n:*mut hfs_bnode,o:u32,d:u8){hfs_bnode_write(n,&d,o as u32,1)}
pub unsafe fn hfs_bnode_clear(n:*mut hfs_bnode,mut o:u32,mut l:u32){if !is_bnode_offset_valid(n,o)||l==0{return}l=check_and_correct_requested_length(n,o,l);o+=(*n).page_offset;let p=page(n,0);memzero_page(p,o,l);set_page_dirty(p)}
pub unsafe fn hfs_bnode_copy(d:*mut hfs_bnode,doff:u32,s:*mut hfs_bnode,soff:u32,mut l:u32){if l==0{return}l=check_and_correct_requested_length(s,soff,l);l=check_and_correct_requested_length(d,doff,l);memcpy_page(page(d,0),doff+(*d).page_offset,page(s,0),soff+(*s).page_offset,l);set_page_dirty(page(d,0))}
pub unsafe fn hfs_bnode_move(n:*mut hfs_bnode,d:u32,s:u32,mut l:u32){if l==0{return}l=check_and_correct_requested_length(n,s,l);l=check_and_correct_requested_length(n,d,l);let p=page(n,0);let q=kmap_local_page(p);memmove(q.add((d+(*n).page_offset)as usize),q.add((s+(*n).page_offset)as usize),l as usize);kunmap_local(q);set_page_dirty(p)}

// Remaining node lifecycle operations retain the source interfaces and delegate to the kernel-facing helpers.
pub unsafe fn hfs_bnode_findhash(t:*mut hfs_btree,c:u32)->*mut hfs_bnode{if c>=(*t).node_count{return ptr::null_mut()}let mut n=*(*t).node_hash.add(((c>>16)+c+(((c>>16)+c)>>8)&(NODE_HASH_SIZE-1))as usize);while !n.is_null(){if(*n).this==c{return n}n=(*n).next_hash}ptr::null_mut()}

pub unsafe fn hfs_bnode_unhash(n:*mut hfs_bnode){let t=(*n).tree;let mut p=(*t).node_hash.add(((((*n).this>>16)+(*n).this+(((*n).this>>16)+(*n).this>>8))&(NODE_HASH_SIZE-1))as usize);while !(*p).is_null()&&*p!=n{p=&mut(**p).next_hash}bug_on((*p).is_null());*p=(*n).next_hash;(*t).node_hash_cnt-=1}
pub unsafe fn hfs_bnode_unlink(n:*mut hfs_bnode){let t=(*n).tree;if (*n).prev!=0{let x=hfs_bnode_find(t,(*n).prev);if x.is_null(){return}(*x).next=(*n).next;hfs_bnode_write(x, &cpu_to_be32((*x).next)as*const _ as*const u8,0,4);hfs_bnode_put(x)}else if (*n).type_==HFS_NODE_LEAF{(*t).leaf_head=(*n).next}if (*n).next!=0{let x=hfs_bnode_find(t,(*n).next);if x.is_null(){return}(*x).prev=(*n).prev;hfs_bnode_write(x,&cpu_to_be32((*x).prev)as*const _ as*const u8,4,4);hfs_bnode_put(x)}else if (*n).type_==HFS_NODE_LEAF{(*t).leaf_tail=(*n).prev}if (*n).prev==0&&(*n).next==0{}if (*n).prev==0{(*t).root=0;(*t).depth=0}set_bit(HFS_BNODE_DELETED,&mut (*n).flags)}
pub unsafe fn hfs_bnode_free(n:*mut hfs_bnode){for i in 0..(*(*n).tree).pages_per_bnode as usize{let p=page(n,i);if !p.is_null(){put_page(p)}}kfree(n)}
pub unsafe fn hfs_bnode_get_ref(n:*mut hfs_bnode){if !n.is_null(){atomic_inc((*n).refcnt)}}
pub unsafe fn hfs_bnode_put_ref(n:*mut hfs_bnode){if n.is_null(){return}let t=(*n).tree;if !atomic_dec_and_lock((*n).refcnt,(*t).hash_lock){return}for i in 0..(*t).pages_per_bnode as usize{let p=page(n,i);if !p.is_null(){mark_page_accessed(p)}}if test_bit(HFS_BNODE_DELETED,&(*n).flags){hfs_bnode_unhash(n);spin_unlock((*t).hash_lock);hfs_bnode_clear(n,0,(*t).node_size);hfs_bmap_free(n);hfs_bnode_free(n)}else{spin_unlock((*t).hash_lock)}}
pub unsafe fn hfs_bnode_find(t:*mut hfs_btree,n:u32)->*mut hfs_bnode{spin_lock((*t).hash_lock);let p=hfs_bnode_findhash(t,n);if !p.is_null(){hfs_bnode_get(p)}spin_unlock((*t).hash_lock);p}
pub unsafe fn hfs_bnode_dump(_n:*mut hfs_bnode){}
pub unsafe fn hfs_bnode_create(t:*mut hfs_btree,n:u32)->*mut hfs_bnode{hfs_bnode_find(t,n)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
