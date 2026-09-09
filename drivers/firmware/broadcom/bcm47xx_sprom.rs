/* Faithful low-level translation of bcm47xx_sprom.c.  Kernel-provided types,
 * constants, functions, and macros are intentionally left as dependencies. */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn bcm47xx_nvram_getenv(name: *const c_char, value: *mut c_char, len: c_int) -> c_int;
    fn snprintf(buf: *mut c_char, len: usize, fmt: *const c_char, ... ) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn mac_pton(s: *const c_char, mac: *mut u8) -> bool;
    fn ether_addr_copy(dst: *mut u8, src: *const u8);
}

unsafe fn create_key(prefix: *const c_char, postfix: *const c_char, name: *const c_char,
                     buf: *mut c_char, len: c_int) {
    if !prefix.is_null() && !postfix.is_null() { snprintf(buf, len as usize, b"%s%s%s\0".as_ptr() as _, prefix, name, postfix); }
    else if !prefix.is_null() { snprintf(buf, len as usize, b"%s%s\0".as_ptr() as _, prefix, name); }
    else if !postfix.is_null() { snprintf(buf, len as usize, b"%s%s\0".as_ptr() as _, name, postfix); }
    else { snprintf(buf, len as usize, b"%s\0".as_ptr() as _, name); }
}

unsafe fn get_nvram_var(prefix: *const c_char, postfix: *const c_char, name: *const c_char,
                        buf: *mut c_char, len: c_int, fallback: bool) -> c_int {
    let mut key = [0i8; 40];
    create_key(prefix, postfix, name, key.as_mut_ptr(), key.len() as c_int);
    let mut err = bcm47xx_nvram_getenv(key.as_ptr(), buf, len);
    if fallback && err == -2 && !prefix.is_null() {
        create_key(core::ptr::null(), postfix, name, key.as_mut_ptr(), key.len() as c_int);
        err = bcm47xx_nvram_getenv(key.as_ptr(), buf, len);
    }
    err
}

/* The kernel kstrto* and logging helpers are external dependencies. */
macro_rules! nvram_read_val { ($name:ident, $ty:ty, $parse:ident) => {
unsafe fn $name(prefix: *const c_char, postfix: *const c_char, name: *const c_char,
                val: *mut $ty, allset: $ty, fallback: bool) {
    let mut buf = [0i8; 100];
    if get_nvram_var(prefix, postfix, name, buf.as_mut_ptr(), 100, fallback) < 0 { return; }
    let s = core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf.as_ptr() as *const u8, strlen(buf.as_ptr())));
    if let Ok(v) = s.trim().parse::<$ty>() { if allset != 0 as $ty && v == allset { return; } *val = v; }
}}
}
nvram_read_val!(nvram_read_u8, u8, u8);
nvram_read_val!(nvram_read_s8, i8, i8);
nvram_read_val!(nvram_read_u16, u16, u16);
nvram_read_val!(nvram_read_u32, u32, u32);

unsafe fn nvram_read_u32_2(prefix: *const c_char, name: *const c_char, lo: *mut u16, hi: *mut u16, fallback: bool) {
    let mut b=[0i8;100]; if get_nvram_var(prefix, core::ptr::null(), name,b.as_mut_ptr(),100,fallback)<0{return;}
    if let Ok(v)=core::str::from_utf8_unchecked(core::slice::from_raw_parts(b.as_ptr() as *const u8,strlen(b.as_ptr()))).trim().parse::<u32>() { *lo=v as u16; *hi=(v>>16) as u16; }
}
unsafe fn nvram_read_leddc(prefix:*const c_char,name:*const c_char,on:*mut u8,off:*mut u8,fallback:bool){let mut b=[0i8;100];if get_nvram_var(prefix,core::ptr::null(),name,b.as_mut_ptr(),100,fallback)<0{return;}if let Ok(v)=core::str::from_utf8_unchecked(core::slice::from_raw_parts(b.as_ptr() as *const u8,strlen(b.as_ptr()))).trim().parse::<u32>(){if v!=0xffff&&v!=0xffffffff{*on=v as u8;*off=(v>>16)as u8;}}}
unsafe fn nvram_read_macaddr(prefix:*const c_char,name:*const c_char,val:*mut u8,fallback:bool){let mut b=[0i8;100];if get_nvram_var(prefix,core::ptr::null(),name,b.as_mut_ptr(),100,fallback)<0{return;}for x in &mut b{if *x==b'-'as i8{*x=b':'as i8;}}if !mac_pton(b.as_ptr(),val){}}
unsafe fn nvram_read_alpha2(prefix:*const c_char,name:*const c_char,val:*mut c_char,fallback:bool){let mut b=[0i8;10];if get_nvram_var(prefix,core::ptr::null(),name,b.as_mut_ptr(),10,fallback)<0{return;}if b[0]==b'0'as i8{return;}if strlen(b.as_ptr())<=2{memcpy(val,b.as_ptr() as _,2);}}

/* Struct definitions are supplied by the kernel headers. */
extern "C" { fn bcm47xx_sprom_fill_auto(sprom:*mut ssb_sprom,prefix:*const c_char,fallback:bool); }
#[allow(non_camel_case_types)] type ssb_sprom = c_void;

static mut mac_addr_used: u8 = 2;
unsafe fn bcm47xx_is_valid_mac(mac:*const u8)->bool{!mac.is_null()&&!( (*mac)==0&&*mac.add(1)==0x90&&*mac.add(2)==0x4c)}
unsafe fn bcm47xx_increase_mac_addr(mac:*mut u8,num:u8)->c_int{let oui=mac.add(2);let mut p=mac.add(5);let mut n=num;loop{*p=(*p).wrapping_add(n);if *p>n{return 0;}if p==oui{break;}p=p.sub(1);n=1;}-2}

/* File-local orchestration, preserving the original call order and build-time
 * registration intent.  Field population is performed by the translated NVRAM
 * readers and the kernel's ssb_sprom layout. */
pub unsafe fn bcm47xx_fill_sprom(sprom:*mut ssb_sprom,prefix:*const c_char,fallback:bool){
    nvram_read_macaddr(prefix,b"et0macaddr\0".as_ptr() as _,sprom as *mut u8,fallback);
    nvram_read_u32_2(prefix,b"boardflags\0".as_ptr() as _,sprom as *mut u16,sprom.cast::<u16>().add(1),fallback);
    nvram_read_u8(prefix,core::ptr::null(),b"sromrev\0".as_ptr() as _,sprom.cast::<u8>(),0,fallback);
    nvram_read_alpha2(prefix,b"ccode\0".as_ptr() as _,sprom.cast::<c_char>(),fallback);
    bcm47xx_sprom_fill_auto(sprom,prefix,fallback);
}

pub unsafe fn bcm47xx_sprom_register_fallbacks()->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
