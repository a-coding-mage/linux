/* Dependencies supplied by the surrounding repository. */
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const MD4_DIGEST_SIZE: usize = 16;
const MD4_HMAC_BLOCK_SIZE: usize = 64;
const MD4_BLOCK_WORDS: usize = 16;
const MD4_HASH_WORDS: usize = 4;

#[repr(C)]
struct md4_ctx {
    hash: [u32; MD4_HASH_WORDS],
    block: [u32; MD4_BLOCK_WORDS],
    byte_count: u64,
}

extern "C" {
    fn ntohl(x: u32) -> u32;
    fn htonl(x: u32) -> u32;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn isspace(c: c_int) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn read_text_file(name: *const c_char) -> *mut c_char;
    fn xmalloc(size: usize) -> *mut c_char;
    fn get_basename(path: *const c_char) -> *const c_char;
    fn get_line(pos: *mut *mut c_char) -> *mut c_char;
    fn strstarts(s: *const c_char, prefix: *const c_char) -> bool;
    fn warn(format: *const c_char, ...);
}

#[inline]
unsafe fn lshift(mut x: u32, s: u32) -> u32 {
    x &= 0xFFFF_FFFF;
    (x.wrapping_shl(s) & 0xFFFF_FFFF) | (x >> (32 - s))
}

#[inline]
unsafe fn f(x: u32, y: u32, z: u32) -> u32 { (x & y) | ((!x) & z) }
#[inline]
unsafe fn g(x: u32, y: u32, z: u32) -> u32 { (x & y) | (x & z) | (y & z) }
#[inline]
unsafe fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }

#[inline]
unsafe fn round1(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(f(b, c, d)).wrapping_add(k), s);
}
#[inline]
unsafe fn round2(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(g(b, c, d)).wrapping_add(k).wrapping_add(0x5A82_7999), s);
}
#[inline]
unsafe fn round3(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(h(b, c, d)).wrapping_add(k).wrapping_add(0x6ED9_EBA1), s);
}

#[inline]
unsafe fn le32_to_cpu_array(buf: *mut u32, mut words: u32) {
    while words != 0 { *buf = ntohl(*buf); buf = buf.add(1); words -= 1; }
}

#[inline]
unsafe fn cpu_to_le32_array(buf: *mut u32, mut words: u32) {
    while words != 0 { *buf = htonl(*buf); buf = buf.add(1); words -= 1; }
}

unsafe fn md4_transform(hash: *mut u32, input: *const u32) {
    let mut a = *hash.add(0); let mut b = *hash.add(1); let mut c = *hash.add(2); let mut d = *hash.add(3);
    macro_rules! r1 { ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr) => { round1(&mut $a,$b,$c,$d,*input.add($k),$s) }; }
    macro_rules! r2 { ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr) => { round2(&mut $a,$b,$c,$d,*input.add($k),$s) }; }
    macro_rules! r3 { ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr) => { round3(&mut $a,$b,$c,$d,*input.add($k),$s) }; }
    r1!(a,b,c,d,0,3); r1!(d,a,b,c,1,7); r1!(c,d,a,b,2,11); r1!(b,c,d,a,3,19); r1!(a,b,c,d,4,3); r1!(d,a,b,c,5,7); r1!(c,d,a,b,6,11); r1!(b,c,d,a,7,19); r1!(a,b,c,d,8,3); r1!(d,a,b,c,9,7); r1!(c,d,a,b,10,11); r1!(b,c,d,a,11,19); r1!(a,b,c,d,12,3); r1!(d,a,b,c,13,7); r1!(c,d,a,b,14,11); r1!(b,c,d,a,15,19);
    r2!(a,b,c,d,0,3); r2!(d,a,b,c,4,5); r2!(c,d,a,b,8,9); r2!(b,c,d,a,12,13); r2!(a,b,c,d,1,3); r2!(d,a,b,c,5,5); r2!(c,d,a,b,9,9); r2!(b,c,d,a,13,13); r2!(a,b,c,d,2,3); r2!(d,a,b,c,6,5); r2!(c,d,a,b,10,9); r2!(b,c,d,a,14,13); r2!(a,b,c,d,3,3); r2!(d,a,b,c,7,5); r2!(c,d,a,b,11,9); r2!(b,c,d,a,15,13);
    r3!(a,b,c,d,0,3); r3!(d,a,b,c,8,9); r3!(c,d,a,b,4,11); r3!(b,c,d,a,12,15); r3!(a,b,c,d,2,3); r3!(d,a,b,c,10,9); r3!(c,d,a,b,6,11); r3!(b,c,d,a,14,15); r3!(a,b,c,d,1,3); r3!(d,a,b,c,9,9); r3!(c,d,a,b,5,11); r3!(b,c,d,a,13,15); r3!(a,b,c,d,3,3); r3!(d,a,b,c,11,9); r3!(c,d,a,b,7,11); r3!(b,c,d,a,15,15);
    *hash.add(0) = (*hash.add(0)).wrapping_add(a); *hash.add(1) = (*hash.add(1)).wrapping_add(b); *hash.add(2) = (*hash.add(2)).wrapping_add(c); *hash.add(3) = (*hash.add(3)).wrapping_add(d);
}

#[inline] unsafe fn md4_transform_helper(ctx: *mut md4_ctx) { le32_to_cpu_array((*ctx).block.as_mut_ptr(), 16); md4_transform((*ctx).hash.as_mut_ptr(), (*ctx).block.as_ptr()); }
unsafe fn md4_init(ctx: *mut md4_ctx) { (*ctx).hash = [0x67452301,0xefcdab89,0x98badcfe,0x10325476]; (*ctx).byte_count = 0; }

unsafe fn md4_update(ctx: *mut md4_ctx, data: *const u8, mut len: u32) {
    let avail = 64 - (((*ctx).byte_count & 0x3f) as usize); (*ctx).byte_count += len as u64;
    if avail > len as usize { memcpy(((*ctx).block.as_mut_ptr() as *mut u8).add(64-avail) as *mut c_void, data as *const c_void, len as usize); return; }
    memcpy(((*ctx).block.as_mut_ptr() as *mut u8).add(64-avail) as *mut c_void, data as *const c_void, avail); let mut data = data.add(avail); len -= avail as u32;
    while len >= 64 { memcpy((*ctx).block.as_mut_ptr() as *mut c_void, data as *const c_void, 64); md4_transform_helper(ctx); data = data.add(64); len -= 64; }
    memcpy((*ctx).block.as_mut_ptr() as *mut c_void, data as *const c_void, len as usize);
}

unsafe fn md4_final_ascii(ctx: *mut md4_ctx, out: *mut c_char, len: u32) {
    let offset = ((*ctx).byte_count & 0x3f) as usize; let mut p = ( (*ctx).block.as_mut_ptr() as *mut u8).add(offset); let mut padding = 56 - (offset as i32 + 1); *p = 0x80; p = p.add(1);
    if padding < 0 { memset(p as *mut c_void, 0, (padding + 8) as usize); md4_transform_helper(ctx); p = (*ctx).block.as_mut_ptr() as *mut u8; padding = 56; }
    memset(p as *mut c_void, 0, padding as usize); (*ctx).block[14] = ((*ctx).byte_count << 3) as u32; (*ctx).block[15] = ((*ctx).byte_count >> 29) as u32; le32_to_cpu_array((*ctx).block.as_mut_ptr(), 14); md4_transform((*ctx).hash.as_mut_ptr(), (*ctx).block.as_ptr()); cpu_to_le32_array((*ctx).hash.as_mut_ptr(), 4);
    snprintf(out, len as usize, b"%08X%08X%08X%08X\0".as_ptr() as *const c_char, (*ctx).hash[0],(*ctx).hash[1],(*ctx).hash[2],(*ctx).hash[3]);
}

#[inline] unsafe fn add_char(c: u8, md: *mut md4_ctx) { md4_update(md, &c, 1); }
unsafe fn parse_string(file: *const c_char, len: c_ulong, md: *mut md4_ctx) -> c_int { add_char(*file as u8,md); let mut i=1; while i<len { add_char(*file.add(i as usize) as u8,md); if *file.add(i as usize)==b'"' as c_char && *file.add(i as usize-1)!=b'\\' as c_char { break; } i+=1; } i as c_int }
unsafe fn parse_comment(file: *const c_char, len: c_ulong) -> c_int { let mut i=2; while i<len { if *file.add(i as usize-1)==b'*' as c_char && *file.add(i as usize)==b'/' as c_char { break; } i+=1; } i as c_int }

/* FIXME: Handle .s files differently (eg. # starts comments) --RR */
unsafe fn parse_file(fname: *const c_char, md: *mut md4_ctx) -> c_int { let file=read_text_file(fname); let len=strlen(file); let mut i=0; while i<len { if *file.add(i)==b'\\' as c_char && i+1<len && *file.add(i+1)==b'\n' as c_char { i+=1; continue; } if isspace(*file.add(i) as c_int)!=0 { i+=1; continue; } if *file.add(i)==b'"' as c_char { i += parse_string(file.add(i), (len-i) as c_ulong, md) as usize; continue; } if *file.add(i)==b'/' as c_char && *file.add(i+1)==b'*' as c_char { i += parse_comment(file.add(i), (len-i) as c_ulong) as usize; continue; } add_char(*file.add(i) as u8,md); i+=1; } free(file as *mut c_void); 1 }

unsafe fn is_static_library(objfile: *const c_char) -> bool { let len=strlen(objfile); *objfile.add(len-2)==b'.' as c_char && *objfile.add(len-1)==b'a' as c_char }

unsafe fn parse_source_files(objfile: *const c_char, md: *mut md4_ctx) -> c_int {
    let cmd=xmalloc(strlen(objfile)+5); let base=get_basename(objfile); let dirlen=base.offset_from(objfile) as usize; snprintf(cmd, strlen(objfile)+5, b"%.*s.%s.cmd\0".as_ptr() as *const c_char, dirlen, objfile, base); let dir=xmalloc(dirlen+1); strncpy(dir,objfile,dirlen); *dir.add(dirlen)=0; let file=read_text_file(cmd); let mut pos=file; let mut check_files=0; let mut ret=0;
    while { let line=get_line(&mut pos); !line.is_null() } { let mut line=get_line(&mut pos); while isspace(*line as c_int)!=0 { line=line.add(1); } let mut p=line; if strstarts(line,b"source_\0".as_ptr() as *const c_char) { p=strrchr(line,b' ' as c_int); if p.is_null(){ warn(b"malformed line: %s\n\0".as_ptr() as *const c_char,line); break; } p=p.add(1); if parse_file(p,md)==0 { warn(b"could not open %s\n\0".as_ptr() as *const c_char,p); break; } continue; } if strstarts(line,b"deps_\0".as_ptr() as *const c_char){check_files=1;continue;} if check_files==0{continue;} let n=strlen(p); if *p.add(n-1)!=b'\\' as c_char{break;} while *p!=0 {if isspace(*p as c_int)!=0{*p=0;break;} p=p.add(1);} if !strstr(line,dir).is_null() && parse_file(line,md)==0 {warn(b"could not open %s\n\0".as_ptr() as *const c_char,line);break;} }
    ret=1; free(file as *mut c_void); free(dir as *mut c_void); free(cmd as *mut c_void); ret
}

/* Calc and record src checksum. */
#[no_mangle]
pub unsafe extern "C" fn get_src_version(modname: *const c_char, sum: *mut c_char, sumlen: c_uint) {
    let filelist=alloca(4096) as *mut c_char; snprintf(filelist,4096,b"%s.mod\0".as_ptr() as *const c_char,modname); let buf=read_text_file(filelist); let mut pos=buf; let mut md=md4_ctx{hash:[0;4],block:[0;16],byte_count:0}; md4_init(&mut md);
    loop { let fname=strsep(&mut pos,b"\n\0".as_ptr() as *const c_char); if fname.is_null(){break;} if *fname==0{continue;} if !is_static_library(fname) && parse_source_files(fname,&mut md)==0{break;} } md4_final_ascii(&mut md,sum,sumlen); free(buf as *mut c_void);
}

extern "C" { fn alloca(size: usize) -> *mut c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
