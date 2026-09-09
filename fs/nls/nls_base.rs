/* Rust translation of linux/fs/nls/nls_base.c. External kernel symbols and
 * types are intentionally referenced rather than implemented here. */

#[repr(C)]
struct Utf8Table { cmask: i32, cval: i32, shift: i32, lmask: i64, lval: i64 }

const UNICODE_MAX: u64 = 0x0010ffff;
const PLANE_SIZE: u64 = 0x00010000;
const SURROGATE_MASK: u64 = 0xfffff800;
const SURROGATE_PAIR: u64 = 0x0000d800;
const SURROGATE_LOW: u64 = 0x00000400;
const SURROGATE_BITS: u64 = 0x000003ff;
const EILSEQ: i32 = 84;
const EOVERFLOW: i32 = 75;
const EINVAL: i32 = 22;
const ENAMETOOLONG: i32 = 36;

static UTF8_TABLE: [Utf8Table; 6] = [
    Utf8Table { cmask: 0x80, cval: 0, shift: 0, lmask: 0x7f, lval: 0 },
    Utf8Table { cmask: 0xe0, cval: 0xc0, shift: 6, lmask: 0x7ff, lval: 0x80 },
    Utf8Table { cmask: 0xf0, cval: 0xe0, shift: 12, lmask: 0xffff, lval: 0x800 },
    Utf8Table { cmask: 0xf8, cval: 0xf0, shift: 18, lmask: 0x1fffff, lval: 0x10000 },
    Utf8Table { cmask: 0xfc, cval: 0xf8, shift: 24, lmask: 0x3ffffff, lval: 0x200000 },
    Utf8Table { cmask: 0xfe, cval: 0xfc, shift: 30, lmask: 0x7fffffff, lval: 0x4000000 },
];

pub unsafe fn utf8_to_utf32(s: *const u8, inlen: i32, pu: *mut u32) -> i32 {
    let mut l = *s as u64; let c0 = *s as i32; let mut nc = 0;
    for t in UTF8_TABLE.iter() {
        nc += 1;
        if (c0 & t.cmask) == t.cval {
            l &= t.lmask as u64;
            if l < t.lval as u64 || l > UNICODE_MAX || (l & SURROGATE_MASK) == SURROGATE_PAIR { return -EILSEQ; }
            *pu = l as u32; return nc;
        }
        if inlen <= nc { return -EOVERFLOW; }
        let b = *s.add(nc as usize); let c = ((b ^ 0x80) & 0xff) as i32;
        if c & 0xc0 != 0 { return -EILSEQ; }
        l = (l << 6) | c as u64;
    }
    -EILSEQ
}

pub unsafe fn utf32_to_utf8(u: u32, s: *mut u8, mut maxout: i32) -> i32 {
    if s.is_null() { return 0; }
    let l = u as u64;
    if l > UNICODE_MAX || (l & SURROGATE_MASK) == SURROGATE_PAIR { return -EILSEQ; }
    let mut nc = 0; let mut p = s;
    for t in UTF8_TABLE.iter() {
        if maxout == 0 { break; } nc += 1; maxout -= 1;
        if l <= t.lmask as u64 {
            let mut c = t.shift; *p = (t.cval as u64 | (l >> c)) as u8;
            while c > 0 { c -= 6; p = p.add(1); *p = (0x80 | ((l >> c) & 0x3f)) as u8; }
            return nc;
        }
    } -EOVERFLOW
}

#[inline] unsafe fn put_utf16(s: *mut u16, c: u32, endian: i32) { *s = match endian { 1 => (c as u16).to_le(), 2 => (c as u16).to_be(), _ => c as u16 }; }
#[inline] unsafe fn get_utf16(c: u16, endian: i32) -> u64 { match endian { 1 => u16::from_le(c) as u64, 2 => u16::from_be(c) as u64, _ => c as u64 } }

pub unsafe fn utf8s_to_utf16s(mut s: *const u8, mut inlen: i32, endian: i32, pwcs: *mut u16, mut maxout: i32) -> i32 {
    let start = pwcs; let mut op = pwcs;
    while inlen > 0 && maxout > 0 && *s != 0 { if *s & 0x80 != 0 { let mut u=0; let size=utf8_to_utf32(s,inlen,&mut u); if size<0{return -EINVAL;} s=s.add(size as usize); inlen-=size; let mut v=u as u64; if v>=PLANE_SIZE {if maxout<2{break;} v-=PLANE_SIZE; put_utf16(op,(SURROGATE_PAIR|((v>>10)&SURROGATE_BITS)) as u32,endian); op=op.add(1); put_utf16(op,(SURROGATE_PAIR|SURROGATE_LOW|(v&SURROGATE_BITS)) as u32,endian);op=op.add(1);maxout-=2;} else {put_utf16(op,u,endian);op=op.add(1);maxout-=1;} } else {put_utf16(op,*s as u32,endian);op=op.add(1);s=s.add(1);inlen-=1;maxout-=1;} }
    op.offset_from(start) as i32
}

pub unsafe fn utf16s_to_utf8s(mut pwcs:*const u16, mut inlen:i32, endian:i32, s:*mut u8, mut maxout:i32)->i32 { let start=s; let mut op=s; while inlen>0&&maxout>0 { let mut u=get_utf16(*pwcs,endian); if u==0{break;} pwcs=pwcs.add(1);inlen-=1; if u>0x7f { if (u&SURROGATE_MASK)==SURROGATE_PAIR {if u&SURROGATE_LOW!=0{continue;} if inlen<=0{break;} let v=get_utf16(*pwcs,endian);if (v&SURROGATE_MASK)!=SURROGATE_PAIR||v&SURROGATE_LOW==0{continue;}u=PLANE_SIZE+((u&SURROGATE_BITS)<<10)+(v&SURROGATE_BITS);pwcs=pwcs.add(1);inlen-=1;} let size=utf32_to_utf8(u as u32,op,maxout);if size<0{if size==-EILSEQ{continue;}break;}op=op.add(size as usize);maxout-=size;}else{*op=u as u8;op=op.add(1);maxout-=1;} } op.offset_from(start) as i32 }

/* The remaining registration and default-table interfaces depend on kernel
 * nls_table/module definitions supplied by the surrounding translation. */
extern "C" { fn __register_nls(nls: *mut core::ffi::c_void, owner: *mut core::ffi::c_void) -> i32; fn unregister_nls(nls:*mut core::ffi::c_void)->i32; fn load_nls(charset:*const i8)->*mut core::ffi::c_void; fn unload_nls(nls:*mut core::ffi::c_void); }

const fn charset2uni_init() -> [u16; 256] { let mut a=[0u16;256]; let mut i=0; while i<256 {a[i]=i as u16;i+=1;} a }
const fn charset2lower_init() -> [u8;256] { let mut a=[0u8;256]; let mut i=0; while i<256 {a[i]=if i>=0x41&&i<=0x5a{(i+0x20)as u8}else{i as u8};i+=1;} a }
const fn charset2upper_init() -> [u8;256] { let mut a=[0u8;256]; let mut i=0; while i<256 {a[i]=if i>=0x61&&i<=0x7a{(i-0x20)as u8}else{i as u8};i+=1;} a }
static CHARSET2UNI: [u16;256] = charset2uni_init();
static CHARSET2LOWER: [u8;256] = charset2lower_init();
static CHARSET2UPPER: [u8;256] = charset2upper_init();

unsafe fn uni2char(uni:u16,out:*mut u8,boundlen:i32)->i32 { if boundlen<=0{return -ENAMETOOLONG;} if uni==0{return -EINVAL;} *out=uni as u8;1 }
unsafe fn char2uni(raw:*const u8,_boundlen:i32,uni:*mut u16)->i32 { *uni=CHARSET2UNI[*raw as usize];if *uni==0{-EINVAL}else{1} }

/* C's default nls_table instance and load_nls_default preserve the same
 * externally supplied kernel table layout in the complete translation. */
#[no_mangle] pub unsafe extern "C" fn load_nls_default() -> *mut core::ffi::c_void {
    let p = load_nls(b"default\0".as_ptr() as *const i8); if !p.is_null(){p}else{core::ptr::null_mut()}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
