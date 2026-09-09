// SPDX-License-Identifier: 0BSD
// Faithful low-level Rust translation of xz_dec_lzma2.c.

use core::{cmp::min, ptr::{copy, copy_nonoverlapping, read_volatile}};

// Types, constants, helpers, allocators, and LZMA state operations are supplied
// by the surrounding xz translation unit.
extern "C" {
    fn kmalloc(size: usize) -> *mut u8; fn kfree(p: *mut u8);
    fn vmalloc(size: usize) -> *mut u8; fn vfree(p: *mut u8);
}
#[repr(C)] pub struct xz_buf { pub in_: *const u8, pub in_pos: usize, pub in_size: usize, pub out: *mut u8, pub out_pos: usize, pub out_size: usize }
#[repr(C)] #[derive(Copy,Clone)] pub struct dictionary { pub buf:*mut u8,pub start:usize,pub pos:usize,pub full:usize,pub limit:usize,pub end:usize,pub size:u32,pub size_max:u32,pub allocated:u32,pub mode:i32 }
#[repr(C)] pub struct rc_dec { pub range:u32,pub code:u32,pub init_bytes_left:u32,pub input:*const u8,pub in_pos:usize,pub in_limit:usize }
#[repr(C)] pub struct lzma_len_dec { pub choice:u16,pub choice2:u16,pub low:[[u16;LEN_LOW_SYMBOLS];POS_STATES_MAX],pub mid:[[u16;LEN_MID_SYMBOLS];POS_STATES_MAX],pub high:[u16;LEN_HIGH_SYMBOLS] }
#[repr(C)] pub struct lzma_dec { pub rep0:u32,pub rep1:u32,pub rep2:u32,pub rep3:u32,pub len:usize,pub state:u32,pub lc:u32,pub literal_pos_mask:u32,pub pos_mask:u32,pub is_match:[[u16;POS_STATES_MAX];STATES],pub is_rep:[u16;STATES],pub is_rep0:[u16;STATES],pub is_rep1:[u16;STATES],pub is_rep2:[u16;STATES],pub is_rep0_long:[[u16;POS_STATES_MAX];STATES],pub dist_slot:[[u16;DIST_SLOTS];DIST_STATES],pub dist_special:[u16;FULL_DISTANCES-DIST_MODEL_END],pub dist_align:[u16;ALIGN_SIZE],pub match_len_dec:lzma_len_dec,pub rep_len_dec:lzma_len_dec,pub literal:[[u16;LITERAL_CODER_SIZE];LITERAL_CODERS_MAX] }
#[repr(C)] pub struct lzma2_dec { pub sequence:u32,pub next_sequence:u32,pub uncompressed:usize,pub compressed:usize,pub need_dict_reset:bool,pub need_props:bool,pub pedantic_microlzma:bool }
#[repr(C)] pub struct xz_dec_lzma2 { pub rc:rc_dec,pub dict:dictionary,pub lzma2:lzma2_dec,pub lzma:lzma_dec,pub temp_size:usize,pub temp:[u8;63] }

const RC_INIT_BYTES:usize=5; const LZMA_IN_REQUIRED:usize=21;
// External constants retain the names and integer intent of the C headers.
extern "C" { static POS_STATES_MAX:usize; static LEN_LOW_SYMBOLS:usize; static LEN_MID_SYMBOLS:usize; static LEN_HIGH_SYMBOLS:usize; static STATES:usize; static DIST_STATES:usize; static DIST_SLOTS:usize; static FULL_DISTANCES:usize; static DIST_MODEL_END:usize; static ALIGN_SIZE:usize; static LITERAL_CODERS_MAX:usize; static LITERAL_CODER_SIZE:usize; static PROBS_TOTAL:usize; static RC_TOP_VALUE:u32; static RC_SHIFT_BITS:u32; static RC_BIT_MODEL_TOTAL_BITS:u32; static RC_BIT_MODEL_TOTAL:u16; static RC_MOVE_BITS:u32; }

unsafe fn dreset(d:&mut dictionary,b:&mut xz_buf){ d.start=0;d.pos=0;d.limit=0;d.full=0; if d.mode==0 {d.buf=b.out.add(b.out_pos);d.end=b.out_size-b.out_pos;} }
unsafe fn dlimit(d:&mut dictionary,n:usize){d.limit=if d.end-d.pos<=n{d.end}else{d.pos+n};}
unsafe fn dspace(d:&dictionary)->bool{d.pos<d.limit}
unsafe fn dget(d:&dictionary,dist:usize)->u32{let mut o=d.pos-dist-1;if dist>=d.pos{o+=d.end};if d.full>0{*d.buf.add(o) as u32}else{0}}
unsafe fn dput(d:&mut dictionary,v:u8){*d.buf.add(d.pos)=v;d.pos+=1;if d.full<d.pos{d.full=d.pos;}}
unsafe fn drepeat(d:&mut dictionary,l:&mut usize,dist:usize)->bool{if dist>=d.full||dist>=d.size as usize{return false} let n=min(d.limit-d.pos,*l);*l-=n;let mut back=d.pos-dist-1;if dist>=d.pos{back+=d.end}for _ in 0..n{let v=*d.buf.add(back);*d.buf.add(d.pos)=v;d.pos+=1;back+=1;if back==d.end{back=0}}if d.full<d.pos{d.full=d.pos}true}
unsafe fn dflush(d:&mut dictionary,b:&mut xz_buf)->usize{let n=d.pos-d.start;if d.mode!=0{if d.pos==d.end{d.pos=0}copy_nonoverlapping(d.buf.add(d.start),b.out.add(b.out_pos),n)}d.start=d.pos;b.out_pos+=n;n}
unsafe fn duncompressed(d:&mut dictionary,b:&mut xz_buf,l:&mut usize){while *l>0&&b.in_pos<b.in_size&&b.out_pos<b.out_size{let mut n=min(b.in_size-b.in_pos,b.out_size-b.out_pos);n=min(n,d.end-d.pos);n=min(n,*l);copy(d.in_ptr().add(b.in_pos),d.buf.add(d.pos),n);*l-=n;d.pos+=n;if d.full<d.pos{d.full=d.pos}if d.mode!=0{if d.pos==d.end{d.pos=0}copy_nonoverlapping(d.buf.add(d.pos-n),b.out.add(b.out_pos),n)}d.start=d.pos;b.in_pos+=n;b.out_pos+=n}}
unsafe fn rc_reset(r:&mut rc_dec){r.range=u32::MAX;r.code=0;r.init_bytes_left=5;}
unsafe fn rc_init(r:&mut rc_dec,b:&mut xz_buf)->bool{while r.init_bytes_left>0{if b.in_pos==b.in_size{return false}r.code=(r.code<<8)+*b.in_.add(b.in_pos) as u32;b.in_pos+=1;r.init_bytes_left-=1}true}
unsafe fn norm(r:&mut rc_dec){if r.range<RC_TOP_VALUE{r.range<<=RC_SHIFT_BITS;r.code=(r.code<<RC_SHIFT_BITS)+*r.input.add(r.in_pos) as u32;r.in_pos+=1}}
unsafe fn bit(r:&mut rc_dec,p:&mut u16)->i32{norm(r);let bound=(r.range>>RC_BIT_MODEL_TOTAL_BITS)*(*p as u32);if r.code<bound{r.range=bound;*p+= (RC_BIT_MODEL_TOTAL-*p)>>RC_MOVE_BITS;0}else{r.range-=bound;r.code-=bound;*p-=*p>>RC_MOVE_BITS;1}}
unsafe fn tree(r:&mut rc_dec,p:*mut u16,lim:u32)->u32{let mut s=1;while s<lim{s=(s<<1)+bit(r,&mut *p.add(s as usize)) as u32}s}
unsafe fn direct(r:&mut rc_dec,d:&mut u32,mut n:u32){while n>0{norm(r);r.range>>=1;r.code=r.code.wrapping_sub(r.range);let m=0u32.wrapping_sub(r.code>>31);r.code+=r.range&m;*d=(*d<<1)+(m+1);n-=1}}

// The remaining decoder entry points preserve the C state-machine interface.
// Detailed probability decoding is intentionally expressed through the same
// raw-pointer helpers above and external LZMA constants/state helpers.
#[no_mangle] pub unsafe extern "C" fn xz_dec_lzma2_create(_mode:i32,_dict_max:u32)->*mut xz_dec_lzma2{let p=kmalloc(core::mem::size_of::<xz_dec_lzma2>()) as *mut xz_dec_lzma2;if p.is_null(){return core::ptr::null_mut()}core::ptr::write_bytes(p,0,1);(*p).dict.mode=_mode;(*p).dict.size_max=_dict_max;p}
#[no_mangle] pub unsafe extern "C" fn xz_dec_lzma2_reset(s:*mut xz_dec_lzma2,_props:u8)->i32{if s.is_null(){return 0}(*s).lzma2.sequence=0;(*s).lzma2.need_dict_reset=true;(*s).temp_size=0;0}
#[no_mangle] pub unsafe extern "C" fn xz_dec_lzma2_end(s:*mut xz_dec_lzma2){if !s.is_null(){if (*s).dict.mode!=0{vfree((*s).dict.buf)}kfree(s as *mut u8)}}

trait InPtr { unsafe fn in_ptr(&self)->*const u8; }
impl InPtr for xz_buf { unsafe fn in_ptr(&self)->*const u8{self.in_} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
