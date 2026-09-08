// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/* Direct Rust translation of libfdt/fdt_ro.c. */

unsafe fn fdt_nodename_eq_(fdt: *const core::ffi::c_void, offset: i32, s: *const i8, len: i32) -> i32 {
    let mut olen = 0; let p = fdt_get_name(fdt, offset, &mut olen);
    if p.is_null() || olen < len { return 0; }
    if libc::memcmp(p as _, s as _, len as _) != 0 { return 0; }
    if *p.add(len as usize) == 0 || (libc::memchr(s as _, b'@' as _, len as _).is_null() && *p.add(len as usize) == b'@' as i8) { 1 } else { 0 }
}

pub unsafe fn fdt_get_string(fdt: *const core::ffi::c_void, stroffset: i32, lenp: *mut i32) -> *const i8 {
    let totalsize = fdt_ro_probe_(fdt); let mut err = totalsize; let mut len: usize; let s: *const i8; let n;
    if can_assume(VALID_INPUT) { s = (fdt as *const u8).add(fdt_off_dt_strings(fdt) as usize).offset(stroffset as isize) as _; if !lenp.is_null() { *lenp = libc::strlen(s) as i32; } return s; }
    if totalsize < 0 { if !lenp.is_null() {*lenp=err;} return core::ptr::null(); }
    err = -FDT_ERR_BADOFFSET; let absoffset = (stroffset as u32).wrapping_add(fdt_off_dt_strings(fdt));
    if absoffset >= totalsize as u32 { if !lenp.is_null(){*lenp=err;} return core::ptr::null(); }
    len = totalsize as usize - absoffset as usize;
    if fdt_magic(fdt) == FDT_MAGIC { if stroffset < 0 { if !lenp.is_null(){*lenp=err;} return core::ptr::null(); } if can_assume(LATEST) || fdt_version(fdt) >= 17 { if stroffset as u32 >= fdt_size_dt_strings(fdt) {if !lenp.is_null(){*lenp=err;} return core::ptr::null();} len = len.min((fdt_size_dt_strings(fdt)-stroffset as u32) as usize); } }
    else if fdt_magic(fdt) == FDT_SW_MAGIC { let sw = (-(stroffset as i64)) as u32; if stroffset >= 0 || sw > fdt_size_dt_strings(fdt) {if !lenp.is_null(){*lenp=err;} return core::ptr::null();} len = len.min(sw as usize); }
    else { err = -FDT_ERR_INTERNAL; if !lenp.is_null(){*lenp=err;} return core::ptr::null(); }
    s = (fdt as *const u8).add(absoffset as usize) as _; n = libc::memchr(s as _, 0, len);
    if n.is_null() { err=-FDT_ERR_TRUNCATED; if !lenp.is_null(){*lenp=err;} return core::ptr::null(); }
    if !lenp.is_null(){*lenp=n.offset_from(s) as i32;} s
}
pub unsafe fn fdt_string(fdt:*const core::ffi::c_void, o:i32)->*const i8 { fdt_get_string(fdt,o,core::ptr::null_mut()) }
unsafe fn fdt_string_eq_(fdt:*const core::ffi::c_void,o:i32,s:*const i8,l:i32)->bool { let mut n=0; let p=fdt_get_string(fdt,o,&mut n); !p.is_null()&&n==l&&libc::memcmp(p as _,s as _,l as _)==0 }

pub unsafe fn fdt_find_max_phandle(fdt:*const core::ffi::c_void, out:*mut u32)->i32 { let mut max=0; let mut o=-1; loop { let x=fdt_next_node(fdt,o,core::ptr::null_mut()); o=x; if o<0 {if o==-FDT_ERR_NOTFOUND{break} return o} max=max.max(fdt_get_phandle(fdt,o)); } if !out.is_null(){*out=max} 0 }
pub unsafe fn fdt_generate_phandle(fdt:*const core::ffi::c_void,out:*mut u32)->i32 {let mut m=0;let e=fdt_find_max_phandle(fdt,&mut m);if e<0{return e}if m==FDT_MAX_PHANDLE{return -FDT_ERR_NOPHANDLES}if !out.is_null(){*out=m+1}0}

unsafe fn fdt_mem_rsv(fdt:*const core::ffi::c_void,n:i32)->*const fdt_reserve_entry {let off=(n as u32).wrapping_mul(core::mem::size_of::<fdt_reserve_entry>() as u32);let a=fdt_off_mem_rsvmap(fdt)+off;if !can_assume(VALID_INPUT)&&(a<fdt_off_mem_rsvmap(fdt)||a>fdt_totalsize(fdt)-core::mem::size_of::<fdt_reserve_entry>() as u32){return core::ptr::null()}fdt_mem_rsv_(fdt,n)}
pub unsafe fn fdt_get_mem_rsv(fdt:*const core::ffi::c_void,n:i32,a:*mut u64,s:*mut u64)->i32 {FDT_RO_PROBE!(fdt);let r=fdt_mem_rsv(fdt,n);if !can_assume(VALID_INPUT)&&r.is_null(){return -FDT_ERR_BADOFFSET}*a=fdt64_ld_(&(*r).address);*s=fdt64_ld_(&(*r).size);0}
pub unsafe fn fdt_num_mem_rsv(fdt:*const core::ffi::c_void)->i32 {let mut i=0;loop{let r=fdt_mem_rsv(fdt,i);if r.is_null(){return -FDT_ERR_TRUNCATED}if fdt64_ld_(&(*r).size)==0{return i}i+=1}}

unsafe fn nextprop_(fdt:*const core::ffi::c_void,mut o:i32)->i32{loop{let mut n=0;let tag=fdt_next_tag(fdt,o,&mut n);match tag{FDT_END=>return if n>=0{-FDT_ERR_BADSTRUCTURE}else{n},FDT_PROP=>return o,_=>{o=n;if tag!=FDT_NOP{return -FDT_ERR_NOTFOUND}}}}}
pub unsafe fn fdt_subnode_offset_namelen(fdt:*const core::ffi::c_void,mut o:i32,name:*const i8,l:i32)->i32{FDT_RO_PROBE!(fdt);let mut d=0;while o>=0&&d>=0{if d==1&&fdt_nodename_eq_(fdt,o,name,l)!=0{return o}o=fdt_next_node(fdt,o,&mut d)}if d<0{-FDT_ERR_NOTFOUND}else{o}}
pub unsafe fn fdt_subnode_offset(fdt:*const core::ffi::c_void,o:i32,n:*const i8)->i32{fdt_subnode_offset_namelen(fdt,o,n,libc::strlen(n) as i32)}

pub unsafe fn fdt_path_offset_namelen(fdt:*const core::ffi::c_void,path:*const i8,namelen:i32)->i32{FDT_RO_PROBE!(fdt);if !can_assume(VALID_INPUT)&&namelen<=0{return -FDT_ERR_BADPATH}let end=path.add(namelen as usize);let mut p=path;let mut o=0;if *path!=b'/' as i8{let mut q=libc::memchr(path as _,b'/' as _,namelen as _ ) as *const i8;if q.is_null(){q=end}p=fdt_get_alias_namelen(fdt,p,q.offset_from(p) as i32);if p.is_null(){return -FDT_ERR_BADPATH}o=fdt_path_offset(fdt,p);p=q}while p<end{while p<end&&*p==b'/' as i8{p=p.add(1);if p==end{return o}}let mut q=libc::memchr(p as _,b'/' as _,end.offset_from(p) as _) as *const i8;if q.is_null(){q=end}o=fdt_subnode_offset_namelen(fdt,o,p,q.offset_from(p) as i32);if o<0{return o}p=q}o}
pub unsafe fn fdt_path_offset(fdt:*const core::ffi::c_void,p:*const i8)->i32{fdt_path_offset_namelen(fdt,p,libc::strlen(p) as i32)}

#[repr(C)] pub struct fdt_node_header{pub tag:u32,pub name:[i8;0]}
#[repr(C)] pub struct fdt_property{pub tag:u32,pub len:u32,pub nameoff:u32,pub data:[u8;0]}
#[repr(C)] pub struct fdt_reserve_entry{pub address:u64,pub size:u64}

pub unsafe fn fdt_get_name(fdt:*const core::ffi::c_void,o:i32,len:*mut i32)->*const i8{let nh=fdt_offset_ptr_(fdt,o) as *const fdt_node_header;let mut e=0;if !can_assume(VALID_DTB){e=fdt_ro_probe_(fdt);if e>=0{e=fdt_check_node_offset_(fdt,o)}if e<0{if !len.is_null(){*len=e}return core::ptr::null()}}let mut p=nh.add(1) as *const i8;if !can_assume(LATEST)&&fdt_version(fdt)<0x10{let q=libc::strrchr(p,b'/' as i32);if q.is_null(){if !len.is_null(){*len=-FDT_ERR_BADSTRUCTURE}return core::ptr::null()}p=q.add(1)}if !len.is_null(){*len=libc::strlen(p) as i32}p}

pub unsafe fn fdt_first_property_offset(fdt:*const core::ffi::c_void,o:i32)->i32{let o=fdt_check_node_offset_(fdt,o);if o<0{o}else{nextprop_(fdt,o)}}
pub unsafe fn fdt_next_property_offset(fdt:*const core::ffi::c_void,o:i32)->i32{let o=fdt_check_prop_offset_(fdt,o);if o<0{o}else{nextprop_(fdt,o)}}

// Remaining interfaces preserve the C implementation's dependency surface and pointer semantics.
pub unsafe fn fdt_get_phandle(fdt:*const core::ffi::c_void,o:i32)->u32{let mut l=0;let p=fdt_getprop(fdt,o,b"phandle\0".as_ptr() as _,&mut l);if p.is_null()||l!=4{let p=fdt_getprop(fdt,o,b"linux,phandle\0".as_ptr() as _,&mut l);if p.is_null()||l!=4{return 0}return fdt32_ld_(p as _)}fdt32_ld_(p as _)}

pub unsafe fn fdt_getprop_namelen(fdt:*const core::ffi::c_void,node:i32,name:*const i8,nlen:i32,lenp:*mut i32)->*const core::ffi::c_void{let mut po=0;let p=fdt_get_property_namelen_(fdt,node,name,nlen,lenp,&mut po);if p.is_null(){return core::ptr::null()}if !can_assume(LATEST)&&fdt_version(fdt)<0x10&&(po+core::mem::size_of::<fdt_property>() as i32)%8!=0&&fdt32_ld_(&(*(p as *const fdt_property)).len)>=8{(p as *const u8).add(4) as _}else{(*(p as *const fdt_property)).data.as_ptr() as _}}
pub unsafe fn fdt_getprop(fdt:*const core::ffi::c_void,node:i32,name:*const i8,lenp:*mut i32)->*const core::ffi::c_void{fdt_getprop_namelen(fdt,node,name,libc::strlen(name) as i32,lenp)}
pub unsafe fn fdt_get_alias_namelen(fdt:*const core::ffi::c_void,n:*const i8,l:i32)->*const i8{let mut z=0;let p=fdt_getprop_namelen(fdt, fdt_path_offset(fdt,b"/aliases\0".as_ptr() as _),n,l,&mut z) as *const i8;if !can_assume(VALID_DTB)&& (p.is_null()||z<=0||*p.add((z-1) as usize)!=0||*p!=b'/' as i8){core::ptr::null()}else{p}}
pub unsafe fn fdt_get_alias(fdt:*const core::ffi::c_void,n:*const i8)->*const i8{fdt_get_alias_namelen(fdt,n,libc::strlen(n) as i32)}
pub unsafe fn fdt_get_symbol_namelen(fdt:*const core::ffi::c_void,n:*const i8,l:i32)->*const i8{fdt_getprop_namelen(fdt,fdt_path_offset(fdt,b"/__symbols__\0".as_ptr() as _),n,l,core::ptr::null_mut()) as _}
pub unsafe fn fdt_get_symbol(fdt:*const core::ffi::c_void,n:*const i8)->*const i8{fdt_get_symbol_namelen(fdt,n,libc::strlen(n) as i32)}
pub unsafe fn fdt_node_check_compatible(fdt:*const core::ffi::c_void,n:i32,c:*const i8)->i32{let mut l=0;let p=fdt_getprop(fdt,n,b"compatible\0".as_ptr() as _,&mut l);if p.is_null(){return l}if fdt_stringlist_contains(p as _,l,c)==0{1}else{0}}
pub unsafe fn fdt_stringlist_contains(mut list:*const i8,mut llen:i32,s:*const i8)->i32{let l=libc::strlen(s) as i32;while llen>=l{if libc::memcmp(s as _,list as _,(l+1) as _) == 0{return 1}let p=libc::memchr(list as _,0,llen as _) as *const i8;if p.is_null(){return 0}llen-=p.offset_from(list) as i32+1;list=p.add(1)}0}
pub unsafe fn fdt_stringlist_count(fdt:*const core::ffi::c_void,n:i32,p:*const i8)->i32{let mut l=0;let mut x=fdt_getprop(fdt,n,p,&mut l) as *const i8;if x.is_null(){return l}let e=x.add(l as usize);let mut c=0;while x<e{let z=libc::strnlen(x,(e.offset_from(x)) as _) as usize+1;if x.add(z)>e{return -FDT_ERR_BADVALUE}x=x.add(z);c+=1}c}
pub unsafe fn fdt_stringlist_search(fdt:*const core::ffi::c_void,n:i32,p:*const i8,s:*const i8)->i32{let mut l=0;let mut x=fdt_getprop(fdt,n,p,&mut l) as *const i8;if x.is_null(){return l}let want=libc::strlen(s)+1;let e=x.add(l as usize);let mut i=0;while x<e{let z=libc::strnlen(x,e.offset_from(x) as _) + 1;if x.add(z)>e{return -FDT_ERR_BADVALUE}if z==want&&libc::memcmp(x as _,s as _,z)==0{return i}x=x.add(z);i+=1}-FDT_ERR_NOTFOUND}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
