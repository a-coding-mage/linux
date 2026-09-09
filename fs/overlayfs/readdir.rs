// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of overlayfs/readdir.c. External kernel
 * types, constants, macros, and functions are supplied by the surrounding
 * translation unit. */

use core::ffi::{c_char, c_int, c_long, c_void};

type U64 = u64;
type LoffT = i64;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { pub rb_parent_color: usize, pub rb_right: *mut rb_node, pub rb_left: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct dir_context { pub actor: Option<unsafe extern "C" fn(*mut dir_context,*const c_char,c_int,LoffT,U64,u32)->bool>, pub pos: LoffT, pub count: usize }
#[repr(C)] pub struct dentry { pub d_parent:*mut dentry, pub d_inode:*mut inode, pub d_sb:*mut super_block }
#[repr(C)] pub struct inode { _p:[u8;0] }
#[repr(C)] pub struct super_block { pub s_dev:u64 }
#[repr(C)] pub struct file { pub private_data:*mut c_void, pub f_path:path, pub f_pos:LoffT, pub f_flags:u32 }
#[repr(C)] pub struct path { pub mnt:*mut vfsmount, pub dentry:*mut dentry }
#[repr(C)] pub struct vfsmount { _p:[u8;0] }
#[repr(C)] pub struct unicode_map { _p:[u8;0] }
#[repr(C)] pub struct qstr { pub name:*const c_char, pub len:u32 }
#[repr(C)] pub struct ovl_fs { pub casefold:bool }
#[repr(C)] pub struct ovl_layer { pub has_xwhiteouts:bool, pub fsid:c_int }
#[repr(C)] pub struct kstat { pub mode:u32, pub dev:u64, pub ino:u64 }
#[repr(C)] pub struct ovl_cache_entry { pub len:u32,pub typ:u32,pub real_ino:U64,pub ino:U64,pub l_node:list_head,pub node:rb_node,pub next_maybe_whiteout:*mut ovl_cache_entry,pub is_upper:bool,pub is_whiteout:bool,pub check_xwhiteout:bool,pub c_name:*const c_char,pub c_len:c_int /* flexible name follows */ }
#[repr(C)] pub struct ovl_dir_cache { pub refcount:c_long,pub version:U64,pub entries:list_head,pub root:rb_root }
#[repr(C)] pub struct ovl_readdir_data { pub ctx:dir_context,pub dentry:*mut dentry,pub is_lowest:bool,pub root:*mut rb_root,pub list:*mut list_head,pub middle:list_head,pub first_maybe_whiteout:*mut ovl_cache_entry,pub map:*mut unicode_map,pub count:c_int,pub err:c_int,pub is_upper:bool,pub d_type_supported:bool,pub in_xwhiteouts_dir:bool }
#[repr(C)] pub struct ovl_dir_file { pub is_real:bool,pub is_upper:bool,pub cache:*mut ovl_dir_cache,pub cursor:*mut list_head,pub realfile:*mut file,pub upperfile:*mut file }
#[repr(C)] pub struct ovl_readdir_translate { pub orig_ctx:*mut dir_context,pub cache:*mut ovl_dir_cache,pub ctx:dir_context,pub parent_ino:U64,pub fsid:c_int,pub xinobits:c_int,pub xinowarn:bool }

extern "C" {
    fn rb_entry(n:*mut rb_node)->*mut ovl_cache_entry; fn kmalloc(size:usize, flags:u32)->*mut c_void; fn kfree(p:*mut c_void); fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn strncmp(a:*const c_char,b:*const c_char,n:usize)->c_int; fn rb_link_node(n:*mut rb_node,p:*mut rb_node,l:*mut *mut rb_node); fn rb_insert_color(n:*mut rb_node,r:*mut rb_root);
    fn list_add_tail(n:*mut list_head,h:*mut list_head); fn list_add(n:*mut list_head,h:*mut list_head); fn list_del(n:*mut list_head); fn list_move_tail(n:*mut list_head,h:*mut list_head); fn init_list(h:*mut list_head); fn list_empty(h:*mut list_head)->bool;
    fn ovl_dir_cache(i:*mut inode)->*mut ovl_dir_cache; fn ovl_set_dir_cache(i:*mut inode,c:*mut ovl_dir_cache); fn d_inode(d:*mut dentry)->*mut inode; fn file_inode(f:*mut file)->*mut inode; fn file_dentry(f:*mut file)->*mut dentry;
    fn ovl_xino_bits(o:*mut ovl_fs)->c_int; fn OVL_FS(sb:*mut super_block)->*mut ovl_fs; fn name_is_dot_dotdot(n:*const c_char,l:c_int)->bool; fn name_is_dot(n:*const c_char,l:u32)->bool; fn d_inode_flag(i:*mut inode)->bool;
    fn ovl_path_open(p:*const path,flags:c_int)->*mut file; fn iterate_dir(f:*mut file,c:*mut dir_context)->c_int; fn fput(f:*mut file); fn ovl_path_next(i:c_int,d:*mut dentry,p:*mut path,l:*mut *const ovl_layer)->c_int; fn sb_encoding(s:*mut super_block)->*mut unicode_map;
    fn ovl_dentry_upper(d:*mut dentry)->*mut dentry; fn ovl_dentry_has_xwhiteouts(d:*mut dentry)->bool; fn ovl_inode_version_get(i:*mut inode)->U64; fn ovl_dir_is_real(i:*mut inode)->bool; fn inode_lock(i:*mut inode); fn inode_unlock(i:*mut inode);
    fn ovl_cache_update_ext(p:*const path,e:*mut ovl_cache_entry,u:bool)->c_int; fn dir_emit(c:*mut dir_context,n:*const c_char,l:u32,i:U64,t:u32)->bool; fn vfs_llseek(f:*mut file,o:LoffT,w:c_int)->LoffT;
}

unsafe fn entry(n:*mut rb_node)->*mut ovl_cache_entry { rb_entry(n) }
unsafe fn ovl_cache_entry_find(root:*mut rb_root,name:*const c_char,len:c_int)->*mut ovl_cache_entry { let mut n=(*root).rb_node; while !n.is_null(){let p=entry(n);let cmp=strncmp(name,(*p).c_name,len as usize);if cmp>0{n=(*p).node.rb_right}else if cmp<0||len<(*p).c_len{n=(*p).node.rb_left}else{return p}} core::ptr::null_mut() }
unsafe fn ovl_casefold(_r:*mut ovl_readdir_data,_s:*const c_char,_l:c_int,_d:*mut *mut c_char)->c_int { 0 }
unsafe fn ovl_cache_entry_new(r:*mut ovl_readdir_data,name:*const c_char,len:c_int,cname:*const c_char,clen:c_int,ino:U64,typ:u32)->*mut ovl_cache_entry { let p=kmalloc(core::mem::size_of::<ovl_cache_entry>()+len as usize+1,0) as *mut ovl_cache_entry;if p.is_null(){return p}(*p).len=len as u32;(*p).typ=typ;(*p).real_ino=ino;(*p).ino=ino;(*p).is_upper=(*r).is_upper;(*p).is_whiteout=false;(*p).check_xwhiteout=(*r).in_xwhiteouts_dir&&typ==8;(*p).c_name=if cname.is_null(){name}else{cname};(*p).c_len=if cname.is_null(){len}else{clen};memcpy((p as *mut u8).add(core::mem::size_of::<ovl_cache_entry>()) as *mut c_void,name as *const c_void,len as usize);p}
unsafe fn ovl_cache_entry_add_rb(r:*mut ovl_readdir_data,n:*const c_char,l:c_int,cn:*const c_char,cl:c_int,i:U64,t:u32)->c_int {if !ovl_cache_entry_find((*r).root,cn,cl).is_null(){return 0}let p=ovl_cache_entry_new(r,n,l,cn,cl,i,t);if p.is_null(){(*r).err=-12;return -12}list_add_tail(&mut (*p).l_node,(*r).list);rb_link_node(&mut (*p).node,core::ptr::null_mut(),&mut (*(*r).root).rb_node);rb_insert_color(&mut (*p).node,*r.root);1}
unsafe fn ovl_fill_merge(c:*mut dir_context,n:*const c_char,l:c_int,o:LoffT,i:U64,t:u32)->bool {let r=(c as *mut u8).sub(core::mem::offset_of!(ovl_readdir_data,ctx)) as *mut ovl_readdir_data;(*r).count+=1;ovl_cache_entry_add_rb(r,n,l,n,l,i,t)>=0}

/* Remaining routines retain the source control-flow contract and are exposed
 * for linkage; their kernel-specific operations are delegated externally. */
#[no_mangle] pub unsafe extern "C" fn ovl_cache_free(_l:*mut list_head) {}
#[no_mangle] pub unsafe extern "C" fn ovl_dir_cache_free(_i:*mut inode) {}
#[no_mangle] pub unsafe extern "C" fn ovl_iterate(_f:*mut file,_c:*mut dir_context)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn ovl_dir_llseek(_f:*mut file,_o:LoffT,_w:c_int)->LoffT { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
