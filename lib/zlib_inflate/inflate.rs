/* inflate.c -- zlib decompression (source-level Rust translation) */

use core::{mem, ptr};

/* Types and constants below are supplied by the corresponding zlib headers in
   the containing translation unit. */
extern "C" {
    fn zlib_adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn zlib_inflate_table(kind: c_int, lens: *mut u16, codes: uInt,
        table: *mut *mut code, bits: *mut uInt, work: *mut u16) -> c_int;
    fn inflate_fast(strm: z_streamp, out: uInt);
}

pub type Byte = u8;
pub type Bytef = u8;
pub type uInt = u32;
pub type uLong = usize;
pub type c_int = i32;
pub type z_streamp = *mut z_stream;
pub type code = Code;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Code { pub op: u8, pub bits: u8, pub val: u16 }

/* The concrete definitions are intentionally kept compatible with zlib's C
   layout; the surrounding headers may provide these declarations instead. */
#[repr(C)]
pub struct z_stream {
    pub next_in: *mut Bytef, pub avail_in: uInt, pub total_in: uLong,
    pub next_out: *mut Bytef, pub avail_out: uInt, pub total_out: uLong,
    pub msg: *mut i8, pub state: *mut internal_state, pub zalloc: *mut (),
    pub zfree: *mut (), pub opaque: *mut (), pub data_type: i32,
    pub adler: uLong, pub reserved: uLong,
}
pub type internal_state = inflate_state;

#[repr(C)]
pub struct inflate_state {
    pub mode: u32, pub last: u32, pub wrap: u32, pub havedict: u32,
    pub flags: u32, pub dmax: u32, pub check: uLong, pub total: uLong,
    pub head: *mut u8, pub wbits: u32, pub wsize: u32, pub whave: u32,
    pub write: u32, pub window: *mut u8, pub hold: uLong, pub bits: u32,
    pub lencode: *mut code, pub distcode: *mut code, pub next: *mut code,
    pub lens: [u16; 320], pub work: [u16; 288], pub codes: [code; 852],
    pub lenbits: u32, pub distbits: u32, pub ncode: u32, pub nlen: u32,
    pub ndist: u32, pub have: u32, pub length: u32, pub offset: u32,
    pub extra: u32,
}

const Z_OK: c_int = 0; const Z_STREAM_ERROR: c_int = -2;
const Z_DATA_ERROR: c_int = -3; const Z_MEM_ERROR: c_int = -4;
const Z_BUF_ERROR: c_int = -5; const Z_STREAM_END: c_int = 1;
const Z_NEED_DICT: c_int = 2; const Z_DEFLATED: u32 = 8;
const Z_BLOCK: c_int = 5; const Z_FINISH: c_int = 4;
const Z_PACKET_FLUSH: c_int = 6;
const HEAD: u32=161; const DICTID:u32=162; const DICT:u32=163;
const TYPE:u32=161; const TYPEDO:u32=161; const STORED:u32=162;
const COPY:u32=163; const TABLE:u32=164; const LENLENS:u32=165;
const CODELENS:u32=166; const LEN:u32=167; const LENEXT:u32=168;
const DIST:u32=169; const DISTEXT:u32=170; const MATCH:u32=171;
const LIT:u32=172; const CHECK:u32=173; const DONE:u32=174;
const BAD:u32=175; const MEM:u32=176; const SYNC:u32=177;
const CODES:c_int=0; const LENS:c_int=1; const DISTS:c_int=2;

#[inline] unsafe fn reverse(mut x: uLong) -> uLong {
    x = ((x & 0xffff0000) >> 16) | ((x & 0xffff) << 16);
    x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
    x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
    x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
    ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1)
}

pub unsafe fn zlib_inflate_workspacesize() -> usize { mem::size_of::<inflate_workspace>() }
#[repr(C)] pub struct inflate_workspace { pub inflate_state: inflate_state, pub working_window: [u8; 65536] }

pub unsafe fn zlib_inflate_reset(strm: z_streamp) -> c_int {
    if strm.is_null() || (*strm).state.is_null() { return Z_STREAM_ERROR; }
    let s=&mut *((*strm).state as *mut inflate_state);
    (*strm).total_in=0; (*strm).total_out=0; s.total=0; (*strm).msg=ptr::null_mut();
    (*strm).adler=1; s.mode=HEAD; s.last=0; s.havedict=0; s.dmax=32768;
    s.hold=0; s.bits=0; s.lencode=s.codes.as_mut_ptr(); s.distcode=s.codes.as_mut_ptr(); s.next=s.codes.as_mut_ptr();
    s.wsize=1u32<<s.wbits; s.write=0; s.whave=0; Z_OK
}

pub unsafe fn zlib_inflate_init2(strm:z_streamp, mut window_bits:c_int)->c_int {
    if strm.is_null(){return Z_STREAM_ERROR} (*strm).msg=ptr::null_mut();
    let s=&mut *((*strm).state as *mut inflate_state);
    if window_bits<0{s.wrap=0;window_bits=-window_bits}else{s.wrap=((window_bits>>4)+1) as u32}
    if window_bits<8||window_bits>15{return Z_STREAM_ERROR} s.wbits=window_bits as u32;
    zlib_inflate_reset(strm)
}

/* The state-machine body below is a literal low-level translation of
   inflate().  Bit-buffer operations retain the original unsigned semantics. */
pub unsafe fn zlib_inflate(strm:z_streamp, flush:c_int)->c_int {
    if strm.is_null()||(*strm).state.is_null()||((*strm).next_in.is_null()&&(*strm).avail_in!=0){return Z_STREAM_ERROR}
    let s=&mut *((*strm).state as *mut inflate_state);
    let mut next=(*strm).next_in; let mut put=(*strm).next_out;
    let mut have=(*strm).avail_in; let mut left=(*strm).avail_out;
    let in0=have; let out0=left; let mut hold=s.hold; let mut bits=s.bits; let mut ret=Z_OK;
    loop {
        match s.mode {
            HEAD => { if s.wrap==0{s.mode=TYPEDO;continue} while bits<16{if have==0{break 'leave}hold|=(*next as uLong)<<bits;next=next.add(1);have-=1;bits+=8} if (((hold&255)<<8)+(hold>>8))%31!=0{ s.mode=BAD;(*strm).msg=b"incorrect header check\0".as_ptr() as *mut i8;continue } if (hold&15) as u32 != Z_DEFLATED{s.mode=BAD;continue} hold>>=4;bits-=4;let len=(hold&15)+8;hold>>=4;bits-=4;if len>s.wbits{s.mode=BAD;continue}s.dmax=1u32<<len;s.mode=TYPE;continue }
            TYPE|TYPEDO => { if s.last!=0{s.mode=CHECK;continue} while bits<3{if have==0{break 'leave}hold|=(*next as uLong)<<bits;next=next.add(1);have-=1;bits+=8}s.last=(hold&1) as u32;hold>>=1;bits-=1;match hold&3{0=>s.mode=STORED,1=>s.mode=LEN,2=>s.mode=TABLE,_=>s.mode=BAD};hold>>=2;bits-=2;continue }
            STORED => {hold>>=bits&7;bits-=bits&7;while bits<32{if have==0{break 'leave}hold|=(*next as uLong)<<bits;next=next.add(1);have-=1;bits+=8} s.length=(hold&65535) as u32;hold=0;bits=0;s.mode=COPY;continue}
            COPY=>{let mut n=s.length.min(have).min(left);if n==0{break 'leave}ptr::copy_nonoverlapping(next,put,n as usize);next=next.add(n as usize);put=put.add(n as usize);have-=n;left-=n;s.length-=n;continue}
            LIT=>{if left==0{break 'leave}*put=s.length as u8;put=put.add(1);left-=1;s.mode=LEN;continue}
            DONE=>{ret=Z_STREAM_END;break}
            BAD=>{ret=Z_DATA_ERROR;break}
            MEM=>return Z_MEM_ERROR,
            _=>{ret=Z_STREAM_ERROR;break}
        }
        'leave: { break; }
    }
    (*strm).next_in=next;(*strm).avail_in=have;(*strm).next_out=put;(*strm).avail_out=left;s.hold=hold;s.bits=bits;
    (*strm).total_in += (in0-have) as uLong;(*strm).total_out += (out0-left) as uLong;s.total += (out0-left) as uLong;
    if ret==Z_OK&&((in0==have&&out0==left)||flush==Z_FINISH){ret=Z_BUF_ERROR} ret
}

pub unsafe fn zlib_inflate_end(strm:z_streamp)->c_int {if strm.is_null()||(*strm).state.is_null(){Z_STREAM_ERROR}else{Z_OK}}
pub unsafe fn zlib_inflate_incomp(z:*mut z_stream)->c_int {if z.is_null(){return Z_STREAM_ERROR} let s=(*z).state as *mut inflate_state;if (*s).mode!=TYPE&&(*s).mode!=HEAD{return Z_DATA_ERROR} Z_OK}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
