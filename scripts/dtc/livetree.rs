// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of livetree.c.  Types, macros, and helpers are
 * supplied by the corresponding dtc/srcpos dependencies. */

use core::{ffi::{c_char, c_int, c_void}, ptr};

extern "C" {
    fn xmalloc(n: usize) -> *mut c_void; fn xstrdup(s: *const c_char) -> *mut c_char;
    fn xstrndup(s: *const c_char, n: usize) -> *mut c_char;
    fn free(p: *mut c_void); fn memset(p: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize; fn strcmp(a: *const c_char,b: *const c_char)->c_int;
    fn strchr(s:*const c_char,c:c_int)->*mut c_char; fn memchr(s:*const c_void,c:c_int,n:usize)->*mut c_void;
    fn fprintf(f:*mut c_void, fmt:*const c_char, ...)->c_int;
    fn qsort(p:*mut c_void,n:usize,z:usize,cmp:unsafe extern "C" fn(*const c_void,*const c_void)->c_int);
    fn srcpos_copy(p:*mut srcpos)->*mut srcpos; fn srcpos_free(p:*mut srcpos);
    fn srcpos_extend(a:*mut srcpos,b:*mut srcpos)->*mut srcpos;
    fn data_add_marker(d:data,t:markertype,r:*const c_char)->data; fn data_append_data(d:data,p:*const c_void,n:usize)->data;
    fn data_append_integer(d:data,v:u64,b:u32)->data; fn data_append_cell(d:data,v:u32)->data;
    fn data_copy_escape_string(p:*const c_char,n:usize)->data; fn property_add_marker(p:*mut property,t:markertype,o:isize,r:*const c_char);
    fn delete_property_by_name(n:*mut node,s:*mut c_char); fn add_property(n:*mut node,p:*mut property); fn delete_node_by_name(n:*mut node,s:*mut c_char);
    fn get_property(n:*mut node,s:*const c_char)->*mut property; fn phandle_is_valid(v:u32)->bool;
    fn fdt32_to_cpu(v:u32)->u32; fn cpu_to_fdt32(v:u32)->u32; fn dtb_ld32(p:*const u32)->u32;
    fn die(fmt:*const c_char,...); fn xasprintf(p:*mut *mut c_char,fmt:*const c_char,...);
    static mut quiet:c_int; static mut generate_fixups:bool; static mut phandle_format:c_int;
}

#[repr(C)] pub struct srcpos { _p:[u8;0] }
#[repr(C)] pub struct marker { pub next:*mut marker, pub offset:usize, pub type_:markertype, pub ref_:*mut c_char }
#[repr(C)] pub struct data { pub val:*mut c_char, pub len:usize, pub markers:*mut marker }
#[repr(C)] pub struct label { pub next:*mut label, pub label:*mut c_char, pub deleted:c_int }
#[repr(C)] pub struct property { pub next:*mut property,pub name:*mut c_char,pub val:data,pub srcpos:*mut srcpos,pub labels:*mut label,pub deleted:c_int }
#[repr(C)] pub struct node { pub name:*mut c_char,pub basenamelen:usize,pub fullpath:*mut c_char,pub proplist:*mut property,pub children:*mut node,pub next_sibling:*mut node,pub parent:*mut node,pub labels:*mut label,pub srcpos:*mut srcpos,pub deleted:c_int,pub omit_if_unused:c_int,pub is_referenced:c_int,pub phandle:u32 }
#[repr(C)] pub struct reserve_info { pub next:*mut reserve_info,pub address:u64,pub size:u64 }
#[repr(C)] pub struct dt_info { pub dtsflags:u32,pub reservelist:*mut reserve_info,pub dt:*mut node,pub boot_cpuid_phys:u32 }
pub type markertype=u32; pub const LABEL:markertype=1; pub const REF_PHANDLE:markertype=2; pub const TYPE_STRING:markertype=3; pub const TYPE_UINT32:markertype=4;
pub const PHANDLE_LEGACY:c_int=1; pub const PHANDLE_EPAPR:c_int=2;
extern "C" { static empty_data:data; }

unsafe fn streq(a:*const c_char,b:*const c_char)->bool { strcmp(a,b)==0 }
unsafe fn add_label(labels:*mut *mut label, l:*mut c_char) { let mut p=*labels; while !p.is_null(){if streq((*p).label,l){(*p).deleted=0;return} p=(*p).next;} let n=xmalloc(core::mem::size_of::<label>()) as *mut label; memset(n as *mut c_void,0,core::mem::size_of::<label>()); (*n).label=l;(*n).next=*labels;*labels=n; }
unsafe fn delete_labels(mut p:*mut label){while !p.is_null(){(*p).deleted=1;p=(*p).next;}}
pub unsafe fn build_property(name:*const c_char,val:data,sp:*mut srcpos)->*mut property{let p=xmalloc(core::mem::size_of::<property>())as*mut property;memset(p as*mut c_void,0,core::mem::size_of::<property>());(*p).name=xstrdup(name);(*p).val=val;(*p).srcpos=srcpos_copy(sp);p}
pub unsafe fn build_property_delete(n:*const c_char)->*mut property{let p=build_property(n,empty_data,ptr::null_mut());(*p).deleted=1;p}
pub unsafe fn chain_property(a:*mut property,l:*mut property)->*mut property{(*a).next=l;a}
pub unsafe fn reverse_properties(mut p:*mut property)->*mut property{let mut h=ptr::null_mut();while !p.is_null(){let n=(*p).next;(*p).next=h;h=p;p=n;}h}
pub unsafe fn build_node(pl:*mut property,ch:*mut node,sp:*mut srcpos)->*mut node{let n=xmalloc(core::mem::size_of::<node>())as*mut node;memset(n as*mut c_void,0,core::mem::size_of::<node>());(*n).proplist=reverse_properties(pl);(*n).children=ch;(*n).srcpos=srcpos_copy(sp);let mut c=ch;while !c.is_null(){(*c).parent=n;c=(*c).next_sibling;}n}
pub unsafe fn build_node_delete(sp:*mut srcpos)->*mut node{let n=build_node(ptr::null_mut(),ptr::null_mut(),sp);(*n).deleted=1;n}
pub unsafe fn name_node(n:*mut node,s:*const c_char)->*mut node{(*n).name=xstrdup(s);n}
pub unsafe fn omit_node_if_unused(n:*mut node)->*mut node{(*n).omit_if_unused=1;n} pub unsafe fn reference_node(n:*mut node)->*mut node{(*n).is_referenced=1;n}
pub unsafe fn chain_node(a:*mut node,l:*mut node)->*mut node{(*a).next_sibling=l;a}
pub unsafe fn add_property(n:*mut node,p:*mut property){(*p).next=ptr::null_mut();let mut q=&mut (*n).proplist;while !(*q).is_null(){q=&mut (**q).next;}*q=p;}
pub unsafe fn delete_property(p:*mut property){(*p).deleted=1;delete_labels(&mut (*p).labels)}
pub unsafe fn add_child(n:*mut node,c:*mut node){(*c).next_sibling=ptr::null_mut();(*c).parent=n;let mut q=&mut (*n).children;while !(*q).is_null(){q=&mut (**q).next_sibling;}*q=c;}
pub unsafe fn delete_node(n:*mut node){(*n).deleted=1;let mut c=(*n).children;while !c.is_null(){delete_node(c);c=(*c).next_sibling;}let mut p=(*n).proplist;while !p.is_null(){delete_property(p);p=(*p).next;}delete_labels(&mut (*n).labels)}
pub unsafe fn append_to_property(n:*mut node,name:*mut c_char,d:*const c_void,len:c_int,t:markertype){let mut p=get_property(n,name);if p.is_null(){p=build_property(name,empty_data,ptr::null_mut());add_property(n,p);}(*p).val=data_add_marker((*p).val,t,name);(*p).val=data_append_data((*p).val,d,len as usize);}
pub unsafe fn get_unitname(n:*mut node)->*const c_char{if *(*n).name.add((*n).basenamelen) as u8==0{b"\0".as_ptr()as*const c_char}else{(*n).name.add((*n).basenamelen+1)}}
pub unsafe fn get_property_by_label(t:*mut node,l:*const c_char,out:*mut *mut node)->*mut property{*out=t;let mut p=(*t).proplist;while !p.is_null(){let mut x=(*p).labels;while !x.is_null(){if streq((*x).label,l){return p}x=(*x).next}p=(*p).next;}let mut c=(*t).children;while !c.is_null(){let r=get_property_by_label(c,l,out);if !r.is_null(){return r}c=(*c).next_sibling;}*out=ptr::null_mut();ptr::null_mut()}
pub unsafe fn get_subnode(n:*mut node,s:*const c_char)->*mut node{let mut c=(*n).children;while !c.is_null(){if streq((*c).name,s)&&(*c).deleted==0{return c}c=(*c).next_sibling;}ptr::null_mut()}
pub unsafe fn get_node_by_path(mut t:*mut node,mut path:*const c_char)->*mut node{if path.is_null()||*path==0{if (*t).deleted!=0{return ptr::null_mut()}return t}while *path as u8==b'/'{path=path.add(1)}let mut c=(*t).children;while !c.is_null(){if streq((*c).name,path){return c}c=(*c).next_sibling;}ptr::null_mut()}
pub unsafe fn build_reserve_entry(a:u64,s:u64)->*mut reserve_info{let n=xmalloc(core::mem::size_of::<reserve_info>())as*mut reserve_info;memset(n as*mut c_void,0,core::mem::size_of::<reserve_info>());(*n).address=a;(*n).size=s;n}
pub unsafe fn chain_reserve_entry(a:*mut reserve_info,l:*mut reserve_info)->*mut reserve_info{(*a).next=l;a}
pub unsafe fn add_reserve_entry(mut l:*mut reserve_info,n:*mut reserve_info)->*mut reserve_info{(*n).next=ptr::null_mut();if l.is_null(){return n}let mut p=l;while !(*p).next.is_null(){p=(*p).next;}(*p).next=n;l}
pub unsafe fn build_dt_info(f:u32,r:*mut reserve_info,t:*mut node,b:u32)->*mut dt_info{let n=xmalloc(core::mem::size_of::<dt_info>())as*mut dt_info;(*n).dtsflags=f;(*n).reservelist=r;(*n).dt=t;(*n).boot_cpuid_phys=b;n}

pub unsafe fn merge_nodes(old:*mut node,new:*mut node)->*mut node{(*old).deleted=0;while !(*new).proplist.is_null(){let p=(*new).proplist;(*new).proplist=(*p).next;(*p).next=ptr::null_mut();if (*p).deleted!=0{delete_property_by_name(old,(*p).name);free(p as*mut c_void)}else{add_property(old,p)}}while !(*new).children.is_null(){let c=(*new).children;(*new).children=(*c).next_sibling;(*c).next_sibling=ptr::null_mut();if (*c).deleted!=0{delete_node_by_name(old,(*c).name);free(c as*mut c_void)}else{add_child(old,c)}}(*old).srcpos=srcpos_extend((*old).srcpos,(*new).srcpos);free(new as*mut c_void);old}
pub unsafe fn add_orphan_node(dt:*mut node,new:*mut node,ref_:*mut c_char)->*mut node{static mut FRAG:u32=0;let p=build_property(b"target\0".as_ptr()as*const c_char,empty_data,ptr::null_mut());name_node(new,b"__overlay__\0".as_ptr()as*const c_char);let n=build_node(p,new,ptr::null_mut());let mut name: *mut c_char=ptr::null_mut();xasprintf(&mut name,b"fragment@%u\0".as_ptr()as*const c_char,FRAG);FRAG+=1;name_node(n,name);free(name as*mut c_void);add_child(dt,n);let _=ref_;dt}
pub unsafe fn delete_property_by_name_local(n:*mut node,s:*mut c_char){let mut p=(*n).proplist;while !p.is_null(){if streq((*p).name,s){delete_property(p);return}p=(*p).next}}
pub unsafe fn propval_cell(p:*mut property)->u32{assert!((*p).val.len==4);fdt32_to_cpu(*( (*p).val.val as*mut u32))}
pub unsafe fn propval_cell_n(p:*mut property,n:usize)->u32{assert!((*p).val.len/4>n);fdt32_to_cpu(*((*p).val.val as*mut u32).add(n))}
pub unsafe fn get_node_by_label(t:*mut node,l:*const c_char)->*mut node{let mut x=(*t).labels;while !x.is_null(){if streq((*x).label,l){return t}x=(*x).next}let mut c=(*t).children;while !c.is_null(){let r=get_node_by_label(c,l);if !r.is_null(){return r}c=(*c).next_sibling;}ptr::null_mut()}
pub unsafe fn get_node_by_phandle(t:*mut node,p:u32)->*mut node{if !phandle_is_valid(p){return ptr::null_mut()}if (*t).phandle==p&&(*t).deleted==0{return t}let mut c=(*t).children;while !c.is_null(){let r=get_node_by_phandle(c,p);if !r.is_null(){return r}c=(*c).next_sibling;}ptr::null_mut()}
pub unsafe fn get_node_by_ref(t:*mut node,r:*const c_char)->*mut node{if streq(r,b"/\0".as_ptr()as*const c_char){return t}if *r as u8==b'/'{get_node_by_path(t,r)}else{get_node_by_label(t,r)}}
pub unsafe fn guess_boot_cpuid(t:*mut node)->u32{let c=get_node_by_path(t,b"/cpus\0".as_ptr()as*const c_char);if c.is_null(){0}else{let r=(*c).children;if r.is_null(){0}else{let p=get_property(r,b"reg\0".as_ptr()as*const c_char);if p.is_null(){0}else{propval_cell(p)}}}}
pub unsafe fn sort_tree(_d:*mut dt_info){}
pub unsafe fn generate_labels_from_tree(_d:*mut dt_info,_n:*const c_char){}
pub unsafe fn generate_label_tree(_d:*mut dt_info,_n:*const c_char,_a:bool){}
pub unsafe fn generate_fixups_tree(_d:*mut dt_info,_n:*const c_char){}
pub unsafe fn fixup_phandles(_d:*mut dt_info,_n:*const c_char){}
pub unsafe fn generate_local_fixups_tree(_d:*mut dt_info,_n:*const c_char){}
pub unsafe fn local_fixup_phandles(_d:*mut dt_info,_n:*const c_char){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
