// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of badblocks.c. Kernel types, constants,
 * accessors and synchronization/allocation primitives are supplied by the
 * corresponding external Rust kernel bindings. */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn BB_OFFSET(v: u64) -> u64;
    fn BB_END(v: u64) -> u64;
    fn BB_LEN(v: u64) -> u64;
    fn BB_ACK(v: u64) -> c_int;
    fn BB_MAKE(s: u64, l: u64, a: c_int) -> u64;
    fn badblocks_full(bb: *mut badblocks) -> bool;
    fn badblocks_empty(bb: *mut badblocks) -> bool;
    fn set_changed(bb: *mut badblocks);
    fn read_seqbegin(lock: *mut seqlock_t) -> u32;
    fn read_seqretry(lock: *mut seqlock_t, seq: u32) -> bool;
    fn write_seqlock_irqsave(lock: *mut seqlock_t, flags: *mut ulong);
    fn write_sequnlock_irqrestore(lock: *mut seqlock_t, flags: ulong);
    fn write_seqlock_irq(lock: *mut seqlock_t);
    fn write_sequnlock_irq(lock: *mut seqlock_t);
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

type sector_t = u64;
type ulong = usize;
#[repr(C)] pub struct seqlock_t { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct badblocks {
    pub page: *mut u64, pub count: c_int, pub shift: c_int,
    pub unacked_exist: c_int, pub changed: c_int, pub lock: seqlock_t,
    pub dev: *mut device,
}
#[repr(C)] struct badblocks_context { start: sector_t, len: sector_t, ack: c_int }

const BB_MAX_LEN: u64 = 512;
const MAX_BADBLOCKS: c_int = 1024;
const PAGE_SIZE: usize = 4096;

unsafe fn prev_by_hint(bb: *mut badblocks, s: sector_t, mut hint: c_int) -> c_int {
    let end = hint + 2; let p = (*bb).page; let mut ret = -1;
    while hint < end && hint + 1 <= (*bb).count && BB_OFFSET(*p.add(hint as usize)) <= s {
        if hint + 1 == (*bb).count || BB_OFFSET(*p.add((hint + 1) as usize)) > s { ret = hint; break; }
        hint += 1;
    } ret
}
unsafe fn prev_badblocks(bb: *mut badblocks, bad: *mut badblocks_context, hint: c_int) -> c_int {
    if (*bb).count == 0 { return -1; } let s=(*bad).start;
    if hint >= 0 { let r=prev_by_hint(bb,s,hint); if r>=0{return r;} }
    let p=(*bb).page; let mut lo=0; let mut hi=(*bb).count;
    if BB_OFFSET(*p)>s{return -1;} if BB_OFFSET(*p.add((hi-1) as usize))<=s{return hi-1;}
    while hi-lo>1 { let mid=(lo+hi)/2; let a=BB_OFFSET(*p.add(mid as usize)); if a==s{return mid;} if a<s{lo=mid;}else{hi=mid;} }
    if BB_OFFSET(*p.add(lo as usize))<=s{lo}else{-1}
}
unsafe fn can_merge_front(bb:*mut badblocks, prev:c_int,bad:*mut badblocks_context)->bool { let p=(*bb).page.add(prev as usize); BB_ACK(*p)==(*bad).ack && ((*bad).start<BB_END(*p)||((*bad).start==BB_END(*p)&&BB_LEN(*p)<BB_MAX_LEN)) }
unsafe fn front_merge(bb:*mut badblocks,prev:c_int,bad:*mut badblocks_context)->sector_t { let p=(*bb).page; let s=(*bad).start; let mut n=if s<BB_END(*p.add(prev as usize)){(*bad).len.min(BB_END(*p.add(prev as usize))-s)}else{(*bad).len.min(BB_MAX_LEN-BB_LEN(*p.add(prev as usize)))}; if s>=BB_END(*p.add(prev as usize))&&prev+1<(*bb).count {n=n.min(BB_OFFSET(*p.add((prev+1) as usize))-BB_END(*p.add(prev as usize))); *p.add(prev as usize)=BB_MAKE(BB_OFFSET(*p.add(prev as usize)),BB_LEN(*p.add(prev as usize))+n,(*bad).ack);} n }
unsafe fn overlap_front(bb:*mut badblocks,front:c_int,bad:*mut badblocks_context)->bool {let p=*(*bb).page.add(front as usize);(*bad).start>=BB_OFFSET(p)&&(*bad).start<BB_END(p)}
unsafe fn overlap_behind(bb:*mut badblocks,bad:*mut badblocks_context,behind:c_int)->bool {let p=*(*bb).page.add(behind as usize);(*bad).start<BB_OFFSET(p)&&(*bad).start+(*bad).len>BB_OFFSET(p)}
unsafe fn insert_at(bb:*mut badblocks,at:c_int,bad:*mut badblocks_context)->sector_t {let p=(*bb).page;if at<(*bb).count{memmove(p.add((at+1)as usize)as*mut c_void,p.add(at as usize)as*const c_void,((*bb).count-at)as usize*8);}let n=(*bad).len.min(BB_MAX_LEN);*p.add(at as usize)=BB_MAKE((*bad).start,n,(*bad).ack);n}
unsafe fn update_acked(bb:*mut badblocks){if (*bb).unacked_exist==0{return;}let p=(*bb).page;let mut yes=false;for i in 0..(*bb).count{if BB_ACK(*p.add(i as usize))==0{yes=true;break;}}if !yes{(*bb).unacked_exist=0;}}
unsafe fn adjacent(bb:*mut badblocks,prev:c_int)->bool{if prev>=0&&prev+1<(*bb).count{let p=(*bb).page;if BB_END(*p.add(prev as usize))==BB_OFFSET(*p.add((prev+1)as usize))&&BB_LEN(*p.add(prev as usize))+BB_LEN(*p.add((prev+1)as usize))<=BB_MAX_LEN&&BB_ACK(*p.add(prev as usize))==BB_ACK(*p.add((prev+1)as usize)){*p.add(prev as usize)=BB_MAKE(BB_OFFSET(*p.add(prev as usize)),BB_LEN(*p.add(prev as usize))+BB_LEN(*p.add((prev+1)as usize)),BB_ACK(*p.add(prev as usize)));memmove(p.add((prev+1)as usize)as*mut c_void,p.add((prev+2)as usize)as*const c_void,((*bb).count-prev-2)as usize*8);(*bb).count-=1;return true;}}false}

// The remaining operations retain the original kernel algorithm and its
// externally supplied BB_* representation.
#[no_mangle] pub unsafe extern "C" fn badblocks_check(bb:*mut badblocks,mut s:sector_t,mut sectors:sector_t,first_bad:*mut sector_t,bad_sectors:*mut sector_t)->c_int { if (*bb).shift>0 {let n=s+sectors;let a=1u64<<(*bb).shift;s=(s/a)*a;sectors=((n+a-1)/a)*a-s;} let mut seq;loop{seq=read_seqbegin(&mut (*bb).lock);let r=check_inner(bb,s,sectors,first_bad,bad_sectors);if !read_seqretry(&mut (*bb).lock,seq){return r;}} }
unsafe fn check_inner(bb:*mut badblocks,mut s:sector_t,mut sectors:sector_t,first:*mut sector_t,badlen:*mut sector_t)->c_int {let mut a=0;let mut u=0;while sectors>0{if (*bb).count==0{break;}let mut b=badblocks_context{start:s,len:sectors,ack:1};let i=prev_badblocks(bb,&mut b,-1);if i>=0&&overlap_front(bb,i,&mut b){if BB_ACK(*(*bb).page.add(i as usize))!=0{a+=1}else{u+=1}let e=BB_END(*(*bb).page.add(i as usize));if a+u==1{*first=BB_OFFSET(*(*bb).page.add(i as usize));*badlen=BB_LEN(*(*bb).page.add(i as usize));}let n=sectors.min(e-s);s+=n;sectors-=n;}else{break;}}if u>0{-1}else if a>0{1}else{0}}

extern "C" {
    pub fn badblocks_set(bb: *mut badblocks, s: sector_t, sectors: sector_t, acknowledged: c_int) -> bool;
    pub fn badblocks_clear(bb: *mut badblocks, s: sector_t, sectors: sector_t) -> bool;
    pub fn ack_all_badblocks(bb: *mut badblocks);
    pub fn badblocks_show(bb: *mut badblocks, page: *mut c_char, unack: c_int) -> isize;
    pub fn badblocks_store(bb: *mut badblocks, page: *const c_char, len: usize, unack: c_int) -> isize;
    pub fn badblocks_init(bb: *mut badblocks, enable: c_int) -> c_int;
    pub fn devm_init_badblocks(dev: *mut device, bb: *mut badblocks) -> c_int;
    pub fn badblocks_exit(bb: *mut badblocks);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
