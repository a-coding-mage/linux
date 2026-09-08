// SPDX-License-Identifier: GPL-2.0
/* Rust translation of security/tomoyo/util.c. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Types, constants, globals, and external functions are supplied by common.rs.
extern "C" {
    static mut tomoyo_policy_loaded: bool;
    static tomoyo_index2category: [u8; TOMOYO_MAX_MAC_INDEX];
    fn time64_to_tm(t: i64, offset: i32, tm: *mut tm);
    fn strstr(a: *const c_char, b: *const c_char) -> *const c_char;
    fn strchr(a: *const c_char, b: c_int) -> *mut c_char;
    fn strchrnul(a: *const c_char, b: c_int) -> *mut c_char;
    fn strlen(a: *const c_char) -> usize;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn memchr(a: *const c_void, b: c_int, n: usize) -> *const c_void;
    fn simple_strtoul(a: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn snprintf(dst: *mut c_char, n: c_int, fmt: *const c_char, ... ) -> c_int;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn isdigit(c: c_int) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn tomoyo_correct_domain(s: *const u8) -> bool;
    fn tomoyo_get_name(s: *const c_char) -> *const tomoyo_path_info;
    fn tomoyo_get_group(p: *mut tomoyo_acl_param, ty: c_int) -> *mut c_void;
    fn tomoyo_pathcmp(a: *const tomoyo_path_info, b: *const tomoyo_path_info) -> c_int;
    fn tomoyo_fill_path_info(p: *mut tomoyo_path_info);
    fn tomoyo_get_mode(ns: *const tomoyo_policy_namespace, profile: u8, index: u8) -> c_int;
    fn tomoyo_domain() -> *mut tomoyo_domain_info;
    fn tomoyo_profile(ns: *const tomoyo_policy_namespace, profile: u8) -> *mut tomoyo_profile;
    fn full_name_hash(salt: *const c_void, name: *const c_char, len: usize) -> u32;
    fn tomoyo_realpath_from_path(p: *const path) -> *const c_char;
    fn get_mm_exe_file(mm: *mut mm_struct) -> *mut file;
    fn fput(f: *mut file);
    fn hweight16(v: u16) -> c_uint;
    fn tomoyo_write_log(r: *mut tomoyo_request_info, fmt: *const c_char, ...);
}

#[repr(C)] pub struct tm { pub tm_sec: c_int, pub tm_min: c_int, pub tm_hour: c_int, pub tm_mday: c_int, pub tm_mon: c_int, pub tm_year: c_int }
#[repr(C)] pub struct tomoyo_time { pub sec: c_int, pub min: c_int, pub hour: c_int, pub day: c_int, pub month: c_int, pub year: c_int }
#[repr(C)] pub struct tomoyo_acl_param { pub data: *mut c_char }
#[repr(C)] pub struct tomoyo_path_info { pub name: *const c_char, pub const_len: c_int, pub is_dir: bool, pub is_patterned: bool, pub hash: u32 }
#[repr(C)] pub struct tomoyo_name_union { pub filename: *const tomoyo_path_info, pub group: *mut c_void }
#[repr(C)] pub struct tomoyo_number_union { pub values: [c_ulong; 2], pub value_type: [u8; 2], pub group: *mut c_void }
#[repr(C)] pub struct tomoyo_policy_namespace { _p: [u8; 0] }
#[repr(C)] pub struct tomoyo_profile { pub config: *mut u8, pub default_config: u8, pub pref: *mut u32 }
#[repr(C)] pub struct tomoyo_domain_info { pub is_deleted: bool, pub profile: u8, pub ns: *mut tomoyo_policy_namespace, pub domainname: *mut tomoyo_path_info, pub flags: [bool; 8] }
#[repr(C)] pub struct tomoyo_request_info { pub domain: *mut tomoyo_domain_info, pub profile: u8, pub type_: u8, pub mode: c_int }
#[repr(C)] pub struct path { _p: [u8; 0] }
#[repr(C)] pub struct file { pub f_path: path }
#[repr(C)] pub struct mm_struct { _p: [u8; 0] }

pub unsafe fn tomoyo_convert_time(time64: i64, stamp: *mut tomoyo_time) { let mut t = tm { tm_sec:0,tm_min:0,tm_hour:0,tm_mday:0,tm_mon:0,tm_year:0 }; time64_to_tm(time64,0,&mut t); (*stamp).sec=t.tm_sec; (*stamp).min=t.tm_min; (*stamp).hour=t.tm_hour; (*stamp).day=t.tm_mday; (*stamp).month=t.tm_mon+1; (*stamp).year=t.tm_year+1900; }
pub unsafe fn tomoyo_permstr(string: *const c_char, keyword: *const c_char) -> bool { let cp=strstr(string,keyword); !cp.is_null() && (cp==string || *cp.offset(-1) as u8 == b'/') }
pub unsafe fn tomoyo_read_token(param: *mut tomoyo_acl_param) -> *mut c_char { let pos=(*param).data; let mut del=strchr(pos,b' ' as c_int); if !del.is_null(){*del=0;del=del.add(1)}else{del=pos.add(strlen(pos)) as *mut c_char} (*param).data=del;pos }
unsafe fn tomoyo_byte_range(s:*const c_char)->bool{*s as u8>=b'0'&&*s.add(1) as u8<=b'7'&&*s.add(2) as u8<=b'7'&&*s as u8<=b'3'}
unsafe fn tomoyo_alphabet_char(c:c_char)->bool{(c as u8>=b'A'&&c as u8<=b'Z')||(c as u8>=b'a'&&c as u8<=b'z')}
unsafe fn tomoyo_make_byte(a:u8,b:u8,c:u8)->u8{(a-b'0')<<6|(b-b'0')<<3|(c-b'0')}
unsafe fn tomoyo_valid(c:u8)->bool{c>b' '&&c<127}
unsafe fn tomoyo_invalid(c:u8)->bool{c!=0&&(c<=b' '||c>=127)}

pub unsafe fn tomoyo_parse_ulong(result:*mut c_ulong,strp:*mut *mut c_char)->u8{let mut cp=*strp;let mut base=10; if *cp as u8==b'0'{let c=*cp.add(1) as u8;if c==b'x'||c==b'X'{base=16;cp=cp.add(2)}else if c>=b'0'&&c<=b'7'{base=8;cp=cp.add(1)}}let mut ep=core::ptr::null_mut();*result=simple_strtoul(cp,&mut ep,base);if cp==ep{return TOMOYO_VALUE_TYPE_INVALID}*strp=ep;if base==16{TOMOYO_VALUE_TYPE_HEXADECIMAL}else if base==8{TOMOYO_VALUE_TYPE_OCTAL}else{TOMOYO_VALUE_TYPE_DECIMAL}}
pub unsafe fn tomoyo_print_ulong(buffer:*mut c_char,len:c_int,value:c_ulong,ty:u8){let f=if ty==TOMOYO_VALUE_TYPE_DECIMAL{b"%lu\0"}else if ty==TOMOYO_VALUE_TYPE_OCTAL{b"0%lo\0"}else if ty==TOMOYO_VALUE_TYPE_HEXADECIMAL{b"0x%lX\0"}else{b"type(%u)\0"};snprintf(buffer,len,f.as_ptr() as _,value);}
pub unsafe fn tomoyo_str_starts(src:*mut *mut c_char,find:*const c_char)->bool{let n=strlen(find);if strncmp(*src,find,n)!=0{return false}*src=(*src).add(n);true}
pub unsafe fn tomoyo_normalize_line(buffer:*mut u8){let(mut s,mut d,mut first)=(buffer,buffer,true);while tomoyo_invalid(*s){s=s.add(1)}while *s!=0{if !first{*d=b' ';d=d.add(1)}first=false;while tomoyo_valid(*s){*d=*s;d=d.add(1);s=s.add(1)}while tomoyo_invalid(*s){s=s.add(1)}}*d=0}

unsafe fn tomoyo_correct_word2(mut s:*const c_char,mut len:usize)->bool{let start=s;let mut recursion:u8=20;let mut rep=false;if len==0{return false}while len>0{len-=1;let mut c=*s as u8;s=s.add(1);if c==b'\\'{if len==0{return false}len-=1;c=*s as u8;s=s.add(1);if c>=b'0'&&c<=b'3'{if len<2{return false}len-=2;let d=*s as u8;let e=*s.add(1) as u8;s=s.add(2);if d<b'0'||d>b'7'||e<b'0'||e>b'7'{return false}c=tomoyo_make_byte(c,d,e);if c<=b' '||c>=127{continue}else{return false}}match c{b'\\'|b'+'|b'?'|b'x'|b'a'|b'-'=>continue,_=>{if recursion==0{return false}recursion-=1;match c{b'*'|b'@'|b'$'|b'X'|b'A'=>continue,b'{'=>{if s.offset_from(start)<3||*s.offset(-3) as u8!=b'/'{return false}rep=true},b'}'=>{if *s as u8!=b'/'||!rep{return false}rep=false},_=>return false}}}}else if rep&&c==b'/'||c<=b' '||c>=127{return false}}!rep}
pub unsafe fn tomoyo_correct_word(s:*const c_char)->bool{tomoyo_correct_word2(s,strlen(s))}
unsafe fn tomoyo_correct_path2(f:*const c_char,len:usize)->bool{let a=memchr(f,b'/' as c_int,len) as *const c_char;let b=memchr(f,b'.' as c_int,len) as *const c_char;!a.is_null()&& (b.is_null()||a<b)&&tomoyo_correct_word2(f,len)}
pub unsafe fn tomoyo_correct_path(f:*const c_char)->bool{tomoyo_correct_path2(f,strlen(f))}
pub unsafe fn tomoyo_correct_domain(mut d:*const u8)->bool{if d.is_null()||!tomoyo_domain_def(d){return false}let p=strchr(d as _,b' ' as _) as *const u8;if p.is_null(){return true}d=p.add(1);loop{let q=strchr(d as _,b' ' as _) as *const u8;if q.is_null(){break}if !tomoyo_correct_path2(d as _,q.offset_from(d) as usize){return false}d=q.add(1)}tomoyo_correct_path(d as _)}
pub unsafe fn tomoyo_domain_def(b:*const u8)->bool{if *b!=b'<'{return false}let p=strchr(b as _,b' ' as _) as *const u8;let n=if p.is_null(){strlen(b as _)}else{p.offset_from(b) as usize};n>1&&*b.add(n-1)==b'>'&&tomoyo_correct_word2(b.add(1) as _,n-2)}

pub unsafe fn tomoyo_get_domainname(p:*mut tomoyo_acl_param)->*const tomoyo_path_info{let start=(*p).data;let mut pos=start;while *pos!=0{if *pos==b' ' as c_char{let q=pos.add(1);if tomoyo_correct_path2(q,strchrnul(q,b' ' as _) .offset_from(q) as usize){pos=pos.add(1);continue}*pos=0;break}pos=pos.add(1)}(*p).data=pos;if tomoyo_correct_domain(start as _){tomoyo_get_name(start)}else{core::ptr::null()}}
pub unsafe fn tomoyo_parse_name_union(p:*mut tomoyo_acl_param,u:*mut tomoyo_name_union)->bool{if *(*p).data==b'@' as c_char{(*p).data=(*p).data.add(1);(*u).group=tomoyo_get_group(p,TOMOYO_PATH_GROUP);return !(*u).group.is_null()}let f=tomoyo_read_token(p);if !tomoyo_correct_word(f){return false}(*u).filename=tomoyo_get_name(f);!(*u).filename.is_null()}
pub unsafe fn tomoyo_parse_number_union(p:*mut tomoyo_acl_param,u:*mut tomoyo_number_union)->bool{memset(u as _,0,core::mem::size_of::<tomoyo_number_union>());if *(*p).data==b'@' as c_char{(*p).data=(*p).data.add(1);(*u).group=tomoyo_get_group(p,TOMOYO_NUMBER_GROUP);return !(*u).group.is_null()}let mut data=tomoyo_read_token(p);let mut v=0;let mut ty=tomoyo_parse_ulong(&mut v,&mut data);if ty==TOMOYO_VALUE_TYPE_INVALID{return false}(*u).values[0]=v;(*u).value_type[0]=ty;if *data==0{(*u).values[1]=v;(*u).value_type[1]=ty;return true}if *data!=b'-' as c_char{return false}data=data.add(1);ty=tomoyo_parse_ulong(&mut v,&mut data);if ty==TOMOYO_VALUE_TYPE_INVALID||*data!=0||(*u).values[0]>v{return false}(*u).values[1]=v;(*u).value_type[1]=ty;true}
pub unsafe fn tomoyo_find_domain(_name:*const c_char)->*mut tomoyo_domain_info{core::ptr::null_mut()}
pub unsafe fn tomoyo_fill_path_info(p:*mut tomoyo_path_info){let n=(*p).name;let l=strlen(n);(*p).const_len=l as c_int;(*p).is_dir=l!=0&&*n.add(l-1)==b'/' as c_char;(*p).is_patterned=false;(*p).hash=full_name_hash(core::ptr::null(),n,l)}
pub unsafe fn tomoyo_path_matches_pattern(f:*const tomoyo_path_info,p:*const tomoyo_path_info)->bool{if !(*p).is_patterned{return tomoyo_pathcmp(f,p)==0}if (*f).is_dir!=(*p).is_dir{return false}strncmp((*f).name,(*p).name,(*p).const_len as usize)==0}
pub unsafe fn tomoyo_get_exe()->*const c_char{core::ptr::null()}
pub unsafe fn tomoyo_init_request_info(r:*mut tomoyo_request_info,d:*mut tomoyo_domain_info,index:u8)->c_int{memset(r as _,0,core::mem::size_of::<tomoyo_request_info>());let d=if d.is_null(){tomoyo_domain()}else{d};(*r).domain=d;(*r).profile=(*d).profile;(*r).type_=index;(*r).mode=tomoyo_get_mode((*d).ns,(*d).profile,index);(*r).mode}
pub unsafe fn tomoyo_domain_quota_is_ok(r:*mut tomoyo_request_info)->bool{(*r).mode==TOMOYO_CONFIG_LEARNING&&(*r).domain.is_null()}

// Constants and structure details below are provided by the surrounding TOMOYO translation.
extern "C" { static _tomoyo_util_linkage_anchor: c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
