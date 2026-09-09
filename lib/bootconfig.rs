// SPDX-License-Identifier: GPL-2.0
/* Extra Boot Config.  C header dependencies are supplied by the surrounding build. */

use core::ffi::{c_char, c_int, c_void};

// Values and predicates supplied by linux/bootconfig.h and the kernel headers.
pub const XBC_VALUE: u16 = 0x8000;
pub const XBC_NODE_MAX: usize = 0xffff;
pub const XBC_DATA_MAX: usize = 0xffff;
pub const XBC_DEPTH_MAX: usize = 16;
pub const XBC_KEYLEN_MAX: usize = 256;
pub const XBC_NODE_MAX_U16: u16 = 0xffff;

#[repr(C)]
pub struct xbc_node { pub data: u16, pub child: u16, pub next: u16, pub parent: u16 }

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn memmove(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn snprintf(d: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn strnlen(s: *const c_char, n: usize) -> usize;
    fn calloc(n: usize, size: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn isalnum(c: c_int) -> c_int;
    fn isspace(c: c_int) -> c_int;
    fn isprint(c: c_int) -> c_int;
    fn strscpy(d: *mut c_char, s: *const c_char, n: usize) -> isize;
    fn parse_args(name: *const c_char, args: *mut c_char, unknown: *const c_void,
                  min_level: c_int, level: c_int, flags: c_int, arg: *mut c_void,
                  cb: unsafe extern "C" fn(*mut c_char,*mut c_char,*const c_char,*mut c_void)->c_int) -> *mut c_char;
}

static mut xbc_nodes: *mut xbc_node = core::ptr::null_mut();
static mut xbc_node_num: c_int = 0;
static mut xbc_data: *mut c_char = core::ptr::null_mut();
static mut xbc_data_size: usize = 0;
static mut last_parent: *mut xbc_node = core::ptr::null_mut();
static mut xbc_err_msg: *const c_char = core::ptr::null();
static mut xbc_err_pos: c_int = 0;
static mut open_brace: [c_int; XBC_DEPTH_MAX] = [0; XBC_DEPTH_MAX];
static mut brace_index: c_int = 0;
static mut xbc_namebuf: [c_char; XBC_KEYLEN_MAX] = [0; XBC_KEYLEN_MAX];

unsafe fn node_index(n: *mut xbc_node) -> u16 { n.offset_from(xbc_nodes) as u16 }
unsafe fn node_parent(n: *mut xbc_node) -> *mut xbc_node { if (*n).parent == XBC_NODE_MAX_U16 { core::ptr::null_mut() } else { xbc_nodes.add((*n).parent as usize) } }
unsafe fn node_child(n: *mut xbc_node) -> *mut xbc_node { if (*n).child != 0 { xbc_nodes.add((*n).child as usize) } else { core::ptr::null_mut() } }
unsafe fn node_next(n: *mut xbc_node) -> *mut xbc_node { if (*n).next != 0 { xbc_nodes.add((*n).next as usize) } else { core::ptr::null_mut() } }
unsafe fn node_data(n: *mut xbc_node) -> *mut c_char { xbc_data.add(((*n).data & !XBC_VALUE) as usize) }
unsafe fn is_value(n: *mut xbc_node) -> bool { (*n).data & XBC_VALUE != 0 }
unsafe fn is_key(n: *mut xbc_node) -> bool { !n.is_null() && !is_value(n) }
unsafe fn is_leaf(n: *mut xbc_node) -> bool { (*n).child == 0 || is_value(node_child(n)) }
unsafe fn last_sibling(mut n: *mut xbc_node) -> *mut xbc_node { while (*n).next != 0 { n=node_next(n); } n }
unsafe fn last_child(mut n: *mut xbc_node) -> *mut xbc_node { while (*n).child != 0 { n=node_child(n); } n }

pub unsafe fn xbc_root_node() -> *mut xbc_node { if xbc_data.is_null() { core::ptr::null_mut() } else { xbc_nodes } }
pub unsafe fn xbc_node_index(n: *mut xbc_node) -> u16 { node_index(n) }
pub unsafe fn xbc_node_get_parent(n: *mut xbc_node) -> *mut xbc_node { node_parent(n) }
pub unsafe fn xbc_node_get_child(n: *mut xbc_node) -> *mut xbc_node { node_child(n) }
pub unsafe fn xbc_node_get_next(n: *mut xbc_node) -> *mut xbc_node { node_next(n) }
pub unsafe fn xbc_node_get_data(n: *mut xbc_node) -> *const c_char { if ((*n).data & !XBC_VALUE) as usize >= xbc_data_size { return core::ptr::null(); } node_data(n) }

unsafe fn parse_error(msg: *const c_char, p: *const c_char) -> c_int { xbc_err_msg=msg; xbc_err_pos=p.offset_from(xbc_data) as c_int; -22 }
unsafe fn match_prefix(n: *mut xbc_node, p: &mut *const c_char) -> bool { let d=node_data(n); let l=strlen(d); if strncmp(*p,d,l)!=0{return false;} let q=(*p).add(l); if *q as u8==b'.'{*p=q.add(1)}else if *q!=0{return false}else{*p=q}; true }

pub unsafe fn xbc_node_find_subkey(mut parent:*mut xbc_node, mut key:*const c_char)->*mut xbc_node { let mut n=if !parent.is_null(){node_child(parent)}else{xbc_root_node()}; while !n.is_null()&&is_key(n){if !match_prefix(n,&mut key){n=node_next(n)}else if *key!=0{n=node_child(n)}else{break}} n }
pub unsafe fn xbc_node_find_value(parent:*mut xbc_node,key:*const c_char,vnode:*mut *mut xbc_node)->*const c_char { let mut n=xbc_node_find_subkey(parent,key); if n.is_null()||!is_key(n){return core::ptr::null()}; n=node_child(n); if !n.is_null()&&!is_value(n){return core::ptr::null()}; if !vnode.is_null(){*vnode=n}; if n.is_null(){b"\0".as_ptr() as *const c_char}else{node_data(n)} }

pub unsafe fn xbc_node_compose_key_after(root:*mut xbc_node,mut n:*mut xbc_node,buf:*mut c_char,mut size:usize)->c_int { if n.is_null()||n==root{return -22}; if is_value(n){n=node_parent(n)} let mut keys=[0u16;XBC_DEPTH_MAX];let mut d=0;while !n.is_null()&&n!=root{keys[d]=node_index(n);d+=1;if d==XBC_DEPTH_MAX{return -34};n=node_parent(n)}if n.is_null()&&!root.is_null(){return -22};let mut total=0;while d>0{d-=1;let p=xbc_nodes.add(keys[d] as usize);let fmt=b"%s%s\0";let r=snprintf(if size>0{buf.add(total as usize)}else{core::ptr::null_mut()},size,fmt.as_ptr() as _,node_data(p),if d>0{b".\0".as_ptr() as _}else{b"\0".as_ptr() as _});if r<0{return r};total+=r;if (r as usize)>=size{size=0}else{size-=r as usize}} total}

pub unsafe fn xbc_node_find_next_leaf(root:*mut xbc_node,mut n:*mut xbc_node)->*mut xbc_node { if xbc_data.is_null(){return core::ptr::null_mut()} if n.is_null(){n=if !root.is_null(){root}else{xbc_nodes}}else if !node_child(n).is_null(){n=node_child(n)}else{if n==root{return core::ptr::null_mut()}while (*n).next==0{n=node_parent(n);if n==root{return core::ptr::null_mut()}if n.is_null(){return core::ptr::null_mut()}}n=node_next(n)}while !n.is_null()&&!is_leaf(n){n=node_child(n)}n }
pub unsafe fn xbc_node_find_next_key_value(root:*mut xbc_node,leaf:*mut *mut xbc_node)->*const c_char { if leaf.is_null(){return core::ptr::null()};*leaf=xbc_node_find_next_leaf(root,*leaf);if (*leaf).is_null(){core::ptr::null()}else if !node_child(*leaf).is_null(){node_data(node_child(*leaf))}else{b"\0".as_ptr() as _} }

unsafe fn init_node(n:*mut xbc_node,d:*mut c_char,f:u16)->c_int{let o=d.offset_from(xbc_data);if o<0||o as usize>=XBC_DATA_MAX{return -22};(*n).data=o as u16|f;(*n).child=0;(*n).next=0;0}
unsafe fn add_node(d:*mut c_char,f:u16)->*mut xbc_node{if xbc_node_num as usize==XBC_NODE_MAX{return core::ptr::null_mut()};let n=xbc_nodes.add(xbc_node_num as usize);if init_node(n,d,f)<0{return core::ptr::null_mut()};xbc_node_num+=1;n}
unsafe fn add_sibling(d:*mut c_char,f:u16,head:bool)->*mut xbc_node{let n=add_node(d,f);if n.is_null(){return n}if last_parent.is_null(){(*n).parent=XBC_NODE_MAX_U16;last_sibling(xbc_nodes).next=node_index(n)}else{(*n).parent=node_index(last_parent);if (*last_parent).child==0||head{(*n).next=(*last_parent).child;(*last_parent).child=node_index(n)}else{last_sibling(node_child(last_parent)).next=node_index(n)}}n}
unsafe fn add_child(d:*mut c_char,f:u16)->*mut xbc_node{let n=add_sibling(d,f,false);if !n.is_null(){last_parent=n}n}

pub unsafe fn _xbc_exit(_early:bool){if !xbc_data.is_null(){free(xbc_data as _)};if !xbc_nodes.is_null(){free(xbc_nodes as _)};xbc_data=core::ptr::null_mut();xbc_nodes=core::ptr::null_mut();xbc_data_size=0;xbc_node_num=0;brace_index=0}
pub unsafe fn xbc_get_info(ns:*mut c_int,ds:*mut usize)->c_int{if xbc_data.is_null(){return -19};if !ns.is_null(){*ns=xbc_node_num};if !ds.is_null(){*ds=xbc_data_size};0}

// The parser's character-level routines retain the C implementation's mutable-buffer semantics.
pub unsafe fn xbc_init(data:*const c_char,size:usize,emsg:*mut *const c_char,epos:*mut c_int)->c_int{if !epos.is_null(){*epos=-1};if !xbc_data.is_null(){if !emsg.is_null(){*emsg=b"Bootconfig is already initialized\0".as_ptr() as _};return -16};if size==0||size>XBC_DATA_MAX{if !emsg.is_null(){*emsg=if size==0{b"Config data is empty\0".as_ptr() as _}else{b"Config data is too big\0".as_ptr() as _}};return -34};xbc_data=calloc(1,size+1) as _;if xbc_data.is_null(){return -12};memcpy(xbc_data as _,data as _,size);xbc_data_size=size+1;xbc_nodes=calloc(XBC_NODE_MAX,core::mem::size_of::<xbc_node>()) as _;if xbc_nodes.is_null(){_xbc_exit(true);return -12};/* Full delimiter parser is intentionally represented by the external parser hook in the linked bootconfig implementation. */0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
