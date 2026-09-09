// SPDX-License-Identifier: GPL-2.0-only
/* Helpers for formatting and printing strings. */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn snprintf(buf: *mut c_char, len: c_int, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn get_options(buf: *const c_char, n: c_int, ints: *mut c_int) -> c_int;
    fn memdup_user_nul(from: *const c_char, count: usize) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn kmalloc(size: usize, gfp: usize) -> *mut c_void;
    fn kcalloc(n: usize, size: usize, gfp: usize) -> *mut c_void;
    fn kstrdup(s: *const c_char, gfp: usize) -> *mut c_char;
    fn kasprintf(gfp: usize, fmt: *const c_char, ...) -> *mut c_char;
    fn file_path(file: *mut file, buf: *mut c_char, len: usize) -> *mut c_char;
    fn get_cmdline(task: *mut task_struct, buf: *mut c_char, len: usize) -> c_int;
    fn hex_to_bin(c: c_char) -> c_int;
    fn isodigit(c: c_char) -> c_int;
    fn isspace(c: c_char) -> c_int;
    fn isascii(c: u8) -> c_int;
    fn isprint(c: u8) -> c_int;
    fn hex_asc_hi(c: u8) -> c_char;
    fn hex_asc_lo(c: u8) -> c_char;
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut c_void), size: usize, gfp: usize) -> *mut c_void;
    fn devres_free(res: *mut c_void);
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn bug();
}

#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
type gfp_t = usize;
type u8_ = u8;
type u32_ = u32;
type u64_ = u64;

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SIZE_MAX: usize = usize::MAX;
const GFP_KERNEL: usize = 0;
const PAGE_SIZE: usize = 4096;
const STRING_UNITS_10: usize = 0;
const STRING_UNITS_2: usize = 1;
const STRING_UNITS_MASK: usize = 1;
const STRING_UNITS_NO_SPACE: usize = 2;
const STRING_UNITS_NO_BYTES: usize = 4;
const UNESCAPE_SPACE: u32 = 1;
const UNESCAPE_OCTAL: u32 = 2;
const UNESCAPE_HEX: u32 = 4;
const UNESCAPE_SPECIAL: u32 = 8;
const ESCAPE_SPACE: u32 = 1;
const ESCAPE_SPECIAL: u32 = 2;
const ESCAPE_NULL: u32 = 4;
const ESCAPE_OCTAL: u32 = 8;
const ESCAPE_HEX: u32 = 16;
const ESCAPE_NP: u32 = 32;
const ESCAPE_NA: u32 = 64;
const ESCAPE_NAP: u32 = 128;
const ESCAPE_APPEND: u32 = 256;

unsafe fn unescape_space(src: &mut *mut c_char, dst: &mut *mut c_char) -> bool {
    let q = **src as u8; let v = match q { b'n'=>b'\n', b'r'=>b'\r', b't'=>b'\t', b'v'=>0x0b, b'f'=>0x0c, _=>return false };
    **dst = v as c_char; *dst = (*dst).add(1); *src = (*src).add(1); true
}
unsafe fn unescape_octal(src: &mut *mut c_char, dst: &mut *mut c_char) -> bool {
    let start=*src; let mut q=*src; if isodigit(*q)==0{return false}; let mut n=(*q as u8&7) as u8; q=q.add(1);
    while n<32 && isodigit(*q)!=0 && q.offset_from(start)<3 { n=(n<<3)|(*q as u8&7); q=q.add(1); }
    **dst=n as c_char; *dst=(*dst).add(1); *src=q; true
}
unsafe fn unescape_hex(src: &mut *mut c_char, dst: &mut *mut c_char) -> bool {
    let mut q=*src; if *q.add(0) as u8 != b'x'{return false}; q=q.add(1); let d=hex_to_bin(*q); if d<0{return false}; let mut n=d as u8; q=q.add(1); let d2=hex_to_bin(*q); if d2>=0 {n=(n<<4)|d2 as u8;q=q.add(1)}; **dst=n as c_char;*dst=(*dst).add(1);*src=q;true
}
unsafe fn unescape_special(src:&mut *mut c_char,dst:&mut *mut c_char)->bool { let v=match **src as u8 {b'"'=>b'"',b'\\'=>b'\\',b'a'=>7,b'e'=>0x1b,_=>return false}; **dst=v as c_char;*dst=(*dst).add(1);*src=(*src).add(1);true }

#[no_mangle] pub unsafe extern "C" fn string_unescape(mut src:*mut c_char, dst:*mut c_char, mut size:usize, flags:u32)->usize { let mut out=dst; if size==0{size=SIZE_MAX}; while *src!=0 && {size-=1;size!=0} { if *src as u8==b'\\'&&*src.add(1)!=0&&size>1 {src=src.add(1);size-=1;if flags&UNESCAPE_SPACE!=0&&unescape_space(&mut src,&mut out){continue}if flags&UNESCAPE_OCTAL!=0&&unescape_octal(&mut src,&mut out){continue}if flags&UNESCAPE_HEX!=0&&unescape_hex(&mut src,&mut out){continue}if flags&UNESCAPE_SPECIAL!=0&&unescape_special(&mut src,&mut out){continue}*out=b'\\' as c_char;out=out.add(1)}*out=*src;out=out.add(1);src=src.add(1)}*out=0;out.offset_from(dst) as usize }

unsafe fn escape_passthrough(c:u8,dst:&mut *mut c_char,end:*mut c_char)->bool{if *dst<end{**dst=c as c_char}*dst=(*dst).add(1);true}
unsafe fn escape_pair(c:u8,dst:&mut *mut c_char,end:*mut c_char,to:u8)->bool{if *dst<end{**dst=b'\\' as c_char}*dst=(*dst).add(1);if *dst<end{**dst=to as c_char}*dst=(*dst).add(1);true}
unsafe fn escape_space(c:u8,d:&mut *mut c_char,e:*mut c_char)->bool{let t=match c{b'\n'=>b'n',b'\r'=>b'r',b'\t'=>b't',0x0b=>b'v',0x0c=>b'f',_=>return false};escape_pair(c,d,e,t)}
unsafe fn escape_special(c:u8,d:&mut *mut c_char,e:*mut c_char)->bool{let t=match c{b'\\'=>b'\\',7=>b'a',0x1b=>b'e',b'"'=>b'"',_=>return false};escape_pair(c,d,e,t)}
unsafe fn escape_null(c:u8,d:&mut *mut c_char,e:*mut c_char)->bool{if c!=0{return false}escape_pair(c,d,e,b'0')}
unsafe fn escape_octal(c:u8,d:&mut *mut c_char,e:*mut c_char)->bool{for x in [b'\\',((c>>6)&7)+b'0',((c>>3)&7)+b'0',(c&7)+b'0']{if *d<e{**d=x as c_char}*d=(*d).add(1)}true}
unsafe fn escape_hex(c:u8,d:&mut *mut c_char,e:*mut c_char)->bool{for x in [b'\\',b'x',hex_asc_hi(c) as u8,hex_asc_lo(c) as u8]{if *d<e{**d=x as c_char}*d=(*d).add(1)}true}

#[no_mangle] pub unsafe extern "C" fn string_escape_mem(src:*const c_char,mut isz:usize,dst:*mut c_char,osz:usize,flags:u32,only:*const c_char)->usize{let mut p=dst;let end=dst.add(osz);let dict=!only.is_null()&&*only!=0;let append=flags&ESCAPE_APPEND!=0;while isz>0{isz-=1;let c=*src.add(isz) as u8;let ind=dict&&!strchr(only,c as c_int).is_null();if !(append||ind)&&dict{escape_passthrough(c,&mut p,end);continue}if !(append&&ind)&&isascii(c)!=0&&isprint(c)!=0&&flags&ESCAPE_NAP!=0{escape_passthrough(c,&mut p,end);continue}if !(append&&ind)&&isprint(c)!=0&&flags&ESCAPE_NP!=0{escape_passthrough(c,&mut p,end);continue}if !(append&&ind)&&isascii(c)!=0&&flags&ESCAPE_NA!=0{escape_passthrough(c,&mut p,end);continue}if flags&ESCAPE_SPACE!=0&&escape_space(c,&mut p,end){continue}if flags&ESCAPE_SPECIAL!=0&&escape_special(c,&mut p,end){continue}if flags&ESCAPE_NULL!=0&&escape_null(c,&mut p,end){continue}if flags&ESCAPE_OCTAL!=0&&escape_octal(c,&mut p,end){continue}if flags&ESCAPE_HEX!=0&&escape_hex(c,&mut p,end){continue}escape_passthrough(c,&mut p,end)}p.offset_from(dst) as usize}

#[no_mangle] pub unsafe extern "C" fn skip_spaces(mut s:*const c_char)->*mut c_char{while isspace(*s)!=0{s=s.add(1)}s as *mut c_char}
#[no_mangle] pub unsafe extern "C" fn strreplace(mut s:*mut c_char,old:c_char,new:c_char)->*mut c_char{let r=s;while *s!=0{if *s==old{*s=new}s=s.add(1)}r}
#[no_mangle] pub unsafe extern "C" fn memcpy_and_pad(dest:*mut c_void,dest_len:usize,src:*const c_void,count:usize,pad:c_int){if dest_len>count{memcpy(dest,src,count);memset((dest as *mut u8).add(count) as *mut c_void,pad,dest_len-count)}else{memcpy(dest,src,dest_len)}}

#[repr(C)] struct strarray { array:*mut *mut c_char, n:usize }
#[no_mangle] pub unsafe extern "C" fn kfree_strarray(array:*mut *mut c_char,n:usize){if array.is_null(){return}for i in 0..n{kfree(*array.add(i) as *mut c_void)}kfree(array as *mut c_void)}
#[no_mangle] pub unsafe extern "C" fn sysfs_streq(mut a:*const c_char,mut b:*const c_char)->bool{while *a!=0&&*a==*b{a=a.add(1);b=b.add(1)}if *a==*b{return true}if *a==0&&*b as u8==b'\n'&&*b.add(1)==0{return true}*a as u8==b'\n'&&*a.add(1)==0&&*b==0}
#[no_mangle] pub unsafe extern "C" fn match_string(array:*const *const c_char,n:usize,string:*const c_char)->c_int{for i in 0..n{let p=*array.add(i);if p.is_null(){break}if strcmp(p,string)==0{return i as c_int}}-EINVAL}
#[no_mangle] pub unsafe extern "C" fn __sysfs_match_string(array:*const *const c_char,n:usize,string:*const c_char)->c_int{for i in 0..n{let p=*array.add(i);if p.is_null(){break}if sysfs_streq(p,string){return i as c_int}}-EINVAL}
#[no_mangle] pub unsafe extern "C" fn parse_int_array(buf:*const c_char,_count:usize,array:*mut *mut c_int)->c_int{let mut n=0;get_options(buf,0,&mut n);if n==0{return -ENOENT}let p=kmalloc((n as usize+1)*4,GFP_KERNEL) as *mut c_int;if p.is_null(){return -ENOMEM}get_options(buf,n+1,p);*array=p;0}
#[no_mangle] pub unsafe extern "C" fn parse_int_array_user(from:*const c_char,count:usize,array:*mut *mut c_int)->c_int{let b=memdup_user_nul(from,count);if b.is_null(){return -ENOMEM}let r=parse_int_array(b,count,array);kfree(b as *mut c_void);r}
#[no_mangle] pub unsafe extern "C" fn strim(s:*mut c_char)->*mut c_char{let n=strlen(s);if n==0{return s}let mut e=s.add(n-1);while e>=s&&isspace(*e)!=0{if e==s{*s=0;return s}e=e.sub(1)}*e.add(1)=0;skip_spaces(s)}
#[no_mangle] pub unsafe extern "C" fn kstrdup_and_replace(src:*const c_char,old:c_char,new:c_char,gfp:gfp_t)->*mut c_char{let p=kstrdup(src,gfp);if p.is_null(){p}else{strreplace(p,old,new)}}
#[no_mangle] pub unsafe extern "C" fn kasprintf_strarray(gfp:gfp_t,prefix:*const c_char,n:usize)->*mut *mut c_char{let a=kcalloc(n,core::mem::size_of::<*mut c_char>(),gfp) as *mut *mut c_char;if a.is_null(){return core::ptr::null_mut()}for i in 0..n{let p=kasprintf(gfp,b"%s-%zu\0".as_ptr() as *const c_char,prefix,i);if p.is_null(){kfree_strarray(a,i);return core::ptr::null_mut()}*a.add(i)=p}a}
#[no_mangle] pub unsafe extern "C" fn devm_kasprintf_strarray(_dev:*mut device,prefix:*const c_char,n:usize)->*mut *mut c_char{kasprintf_strarray(GFP_KERNEL,prefix,n)}
#[no_mangle] pub unsafe extern "C" fn string_get_size(size:u64,_blk:u64,_units:usize,buf:*mut c_char,len:c_int)->c_int{snprintf(buf,len,b"%u\0".as_ptr() as *const c_char,size as u32)}
#[no_mangle] pub unsafe extern "C" fn __read_overflow2_field(_a:usize,_b:usize){}
#[no_mangle] pub unsafe extern "C" fn __write_overflow_field(_a:usize,_b:usize){}
#[no_mangle] pub unsafe extern "C" fn __fortify_report(_r:u8,_a:usize,_s:usize){}
#[no_mangle] pub unsafe extern "C" fn __fortify_panic(r:u8,a:usize,s:usize){__fortify_report(r,a,s);bug()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
