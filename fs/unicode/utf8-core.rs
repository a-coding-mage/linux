/* SPDX-License-Identifier: GPL-2.0 */

// Linux kernel headers and "utf8n.h" provide the declarations used below.

extern "C" {
    fn utf8nlen(um: *const unicode_map, flag: ::std::os::raw::c_int,
                name: *const u8, len: u32) -> i32;
    fn utf8ncursor(cur: *mut utf8cursor, um: *const unicode_map,
                   flag: ::std::os::raw::c_int, name: *const u8, len: u32) -> i32;
    fn utf8byte(cur: *mut utf8cursor) -> i32;
    fn init_name_hash(salt: *const core::ffi::c_void) -> usize;
    fn partial_name_hash(c: u8, hash: usize) -> usize;
    fn end_name_hash(hash: usize) -> u32;
    fn utf8version_is_supported(um: *const unicode_map, version: u32) -> bool;
    fn symbol_request() -> *mut utf8data_table;
    fn symbol_put();
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree<T>(ptr: *mut T);
    fn match_token(version: *mut u8, token: *const match_token_entry,
                   args: *mut substring) -> i32;
    fn match_uint(arg: *const substring, value: *mut u32) -> i32;
}

const UTF8_NFDI: i32 = 0;
const UTF8_NFDICF: i32 = 1;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const U8_MAX: u32 = 255;

#[repr(C)]
pub struct qstr {
    pub name: *const u8,
    pub len: u32,
    pub hash: u32,
}

#[repr(C)]
pub struct utf8cursor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct utf8data {
    pub maxage: u32,
}

#[repr(C)]
pub struct utf8data_table {
    pub utf8nfdidata: *const utf8data,
    pub utf8nfdidata_size: usize,
    pub utf8nfdicfdata: *const utf8data,
    pub utf8nfdicfdata_size: usize,
}

#[repr(C)]
pub struct unicode_map {
    pub version: u32,
    pub tables: *mut utf8data_table,
    pub ntab: [*const utf8data; 2],
}

#[repr(C)]
pub struct substring {
    _private: [u8; 0],
}

#[repr(C)]
pub struct match_token_entry {
    pub token: u32,
    pub pattern: *const u8,
}

#[inline]
pub unsafe fn utf8_validate(um: *const unicode_map, str_: *const qstr) -> i32 {
    if utf8nlen(um, UTF8_NFDI, (*str_).name, (*str_).len) < 0 {
        return -1;
    }
    0
}

pub unsafe fn utf8_strncmp(um: *const unicode_map, s1: *const qstr, s2: *const qstr) -> i32 {
    let mut cur1: utf8cursor = core::mem::zeroed();
    let mut cur2: utf8cursor = core::mem::zeroed();
    if utf8ncursor(&mut cur1, um, UTF8_NFDI, (*s1).name, (*s1).len) < 0 ||
       utf8ncursor(&mut cur2, um, UTF8_NFDI, (*s2).name, (*s2).len) < 0 { return -EINVAL; }
    loop {
        let c1 = utf8byte(&mut cur1); let c2 = utf8byte(&mut cur2);
        if c1 < 0 || c2 < 0 { return -EINVAL; }
        if c1 != c2 { return 1; }
        if c1 == 0 { return 0; }
    }
}

pub unsafe fn utf8_strncasecmp(um: *const unicode_map, s1: *const qstr, s2: *const qstr) -> i32 {
    let mut cur1: utf8cursor = core::mem::zeroed(); let mut cur2: utf8cursor = core::mem::zeroed();
    if utf8ncursor(&mut cur1, um, UTF8_NFDICF, (*s1).name, (*s1).len) < 0 || utf8ncursor(&mut cur2, um, UTF8_NFDICF, (*s2).name, (*s2).len) < 0 { return -EINVAL; }
    loop { let c1=utf8byte(&mut cur1); let c2=utf8byte(&mut cur2); if c1<0 || c2<0{return -EINVAL;} if c1!=c2{return 1;} if c1==0{return 0;} }
}

pub unsafe fn utf8_strncasecmp_folded(um: *const unicode_map, cf: *const qstr, s1: *const qstr) -> i32 {
    let mut cur1: utf8cursor = core::mem::zeroed(); let mut i=0usize;
    if utf8ncursor(&mut cur1,um,UTF8_NFDICF,(*s1).name,(*s1).len)<0{return -EINVAL;}
    loop { let c1=utf8byte(&mut cur1); let c2=*(*cf).name.add(i) as i32; i+=1; if c1<0{return -EINVAL;} if c1!=c2{return 1;} if c1==0{return 0;} }
}

pub unsafe fn utf8_casefold(um:*const unicode_map,str_:*const qstr,dest:*mut u8,dlen:usize)->i32 { let mut cur:utf8cursor=core::mem::zeroed(); if utf8ncursor(&mut cur,um,UTF8_NFDICF,(*str_).name,(*str_).len)<0{return -EINVAL;} for n in 0..dlen { let c=utf8byte(&mut cur); *dest.add(n)=c as u8; if c==0{return n as i32;} if c==-1{break;} } -EINVAL }

pub unsafe fn utf8_casefold_hash(um:*const unicode_map,salt:*const core::ffi::c_void,str_:*mut qstr)->i32 {
    let mut cur:utf8cursor=core::mem::zeroed(); let mut hash=init_name_hash(salt);
    if utf8ncursor(&mut cur,um,UTF8_NFDICF,(*str_).name,(*str_).len)<0{return -EINVAL;}
    loop { let c=utf8byte(&mut cur); if c==0{break;} if c<0{return -EINVAL;} hash=partial_name_hash(c as u8,hash); }
    (*str_).hash=end_name_hash(hash); 0
}

unsafe fn find_table_version(table:*const utf8data,nr_entries:usize,version:u32)->*const utf8data {
    let mut i=nr_entries-1; while version<(*table.add(i)).maxage{i-=1;} if version>(*table.add(i)).maxage{core::ptr::null()}else{table.add(i)}
}

pub unsafe fn utf8_load(version:u32)->*mut unicode_map {
    let um=kzalloc_obj::<unicode_map>(); if um.is_null(){return (-ENOMEM) as isize as *mut unicode_map;} (*um).version=version;
    (*um).tables=symbol_request(); if (*um).tables.is_null(){kfree(um);return (-EINVAL) as isize as *mut unicode_map;}
    if !utf8version_is_supported(um,version){symbol_put();kfree(um);return (-EINVAL) as isize as *mut unicode_map;}
    (*um).ntab[UTF8_NFDI as usize]=find_table_version((*(*um).tables).utf8nfdidata,(*(*um).tables).utf8nfdidata_size,version); if (*um).ntab[0].is_null(){symbol_put();kfree(um);return (-EINVAL) as isize as *mut unicode_map;}
    (*um).ntab[UTF8_NFDICF as usize]=find_table_version((*(*um).tables).utf8nfdicfdata,(*(*um).tables).utf8nfdicfdata_size,version); if (*um).ntab[1].is_null(){symbol_put();kfree(um);return (-EINVAL) as isize as *mut unicode_map;} um
}

pub unsafe fn utf8_unload(um:*mut unicode_map){if !um.is_null(){symbol_put();kfree(um);}}

pub unsafe fn utf8_parse_version(version:*mut u8)->i32 {
    let mut args:[substring;3]=[core::mem::zeroed(),core::mem::zeroed(),core::mem::zeroed()]; let mut maj=0;let mut min=0;let mut rev=0;
    let token=[match_token_entry{token:1,pattern:b"%u.%u.%u\0".as_ptr()},match_token_entry{token:0,pattern:core::ptr::null()}];
    if match_token(version,token.as_ptr(),args.as_mut_ptr())!=1{return -EINVAL;}
    if match_uint(&args[0],&mut maj)!=0||match_uint(&args[1],&mut min)!=0||match_uint(&args[2],&mut rev)!=0{return -EINVAL;}
    if maj>U8_MAX||min>U8_MAX||rev>U8_MAX{return -EINVAL;} ((maj<<16)|(min<<8)|rev) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
