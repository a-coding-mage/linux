// SPDX-License-Identifier: GPL-2.0-only
// Minimal file system backend for holding eBPF maps and programs.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, ptr};

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_type { BPF_TYPE_UNSPEC = 0, BPF_TYPE_PROG, BPF_TYPE_MAP, BPF_TYPE_LINK }

#[repr(C)]
pub struct bpf_fs_inode { pub xattrs: list_head, pub xlimits: simple_xattr_limits, pub vfs_inode: inode }
#[repr(C)] pub struct map_iter { pub key: *mut c_void, pub done: bool }
#[repr(C)] pub struct bpffs_btf_enums { pub btf: *const btf, pub cmd_t: *const btf_type, pub map_t: *const btf_type, pub prog_t: *const btf_type, pub attach_t: *const btf_type }

extern "C" {
    static mut bpf_fs_inode_cachep: *mut kmem_cache;
    static mut bpf_preload_ops: *mut bpf_preload_ops;
    static bpf_dir_iops: inode_operations; static bpf_symlink_iops: inode_operations;
    static bpf_prog_iops: inode_operations; static bpf_map_iops: inode_operations; static bpf_link_iops: inode_operations;
    static bpffs_map_seq_ops: seq_operations; static bpffs_map_fops: file_operations; static bpffs_obj_fops: file_operations;
    static bpf_super_ops: super_operations; static bpf_fs_type: file_system_type;
}

// Kernel-provided types and functions are intentionally external dependencies.
#[repr(C)] pub struct list_head { _p: [u8; 0] } #[repr(C)] pub struct simple_xattr_limits { _p: [u8; 0] }
#[repr(C)] pub struct inode { pub i_op:*const inode_operations, pub i_fop:*const file_operations, pub i_private:*mut c_void, pub i_sb:*mut super_block, pub i_mode:u32, pub i_ino:u64, pub i_link:*mut c_char, pub i_uid:u32, pub i_gid:u32 }
#[repr(C)] pub struct super_block { pub s_fs_info:*mut c_void, pub s_root:*mut dentry }
#[repr(C)] pub struct dentry { pub d_parent:*mut dentry, pub d_inode:*mut inode, pub d_sb:*mut super_block, pub d_name:qstr }
#[repr(C)] pub struct qstr { pub name:*const c_char }
#[repr(C)] pub struct bpf_map { pub key_size:u32, pub ops:*mut bpf_map_ops } #[repr(C)] pub struct bpf_link { _p:[u8;0] } #[repr(C)] pub struct bpf_prog { _p:[u8;0] }
#[repr(C)] pub struct bpf_map_ops { pub map_get_next_key:Option<unsafe extern "C" fn(*mut bpf_map,*mut c_void,*mut c_void)->c_int>, pub map_seq_show_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut c_void,*mut seq_file)> }
#[repr(C)] pub struct seq_file { pub private:*mut c_void, pub file:*mut file } #[repr(C)] pub struct file { pub private_data:*mut c_void }
#[repr(C)] pub struct path { pub dentry:*mut dentry } #[repr(C)] pub struct mnt_idmap{_p:[u8;0]} #[repr(C)] pub struct fs_context{pub s_fs_info:*mut c_void,pub user_ns:*mut c_void,pub ops:*const fs_context_operations}
#[repr(C)] pub struct inode_operations{_p:[u8;0]} #[repr(C)] pub struct file_operations{_p:[u8;0]} #[repr(C)] pub struct seq_operations{_p:[u8;0]} #[repr(C)] pub struct super_operations{_p:[u8;0]} #[repr(C)] pub struct fs_parameter_spec{_p:[u8;0]} #[repr(C)] pub struct file_system_type{_p:[u8;0]} #[repr(C)] pub struct kmem_cache{_p:[u8;0]} #[repr(C)] pub struct btf{_p:[u8;0]} #[repr(C)] pub struct btf_type{_p:[u8;0]} #[repr(C)] pub struct bpf_preload_ops{_p:[u8;0]}
#[repr(C)] pub struct xattr{pub name:*const c_char,pub value:*const c_void,pub value_len:usize} #[repr(C)] pub struct bpf_mount_opts{pub xa_cache:[u8;0],pub uid:u32,pub gid:u32,pub mode:u32,pub delegate_cmds:u64,pub delegate_maps:u64,pub delegate_progs:u64,pub delegate_attachs:u64}

#[inline] unsafe fn BPF_FS_I<'a>(i:*mut inode)->*mut bpf_fs_inode { (i as *mut u8).sub(core::mem::offset_of!(bpf_fs_inode,vfs_inode)) as *mut bpf_fs_inode }

unsafe fn bpf_any_get(raw:*mut c_void, ty:bpf_type)->*mut c_void { match ty { bpf_type::BPF_TYPE_PROG=>bpf_prog_inc(raw), bpf_type::BPF_TYPE_MAP=>bpf_map_inc_with_uref(raw), bpf_type::BPF_TYPE_LINK=>bpf_link_inc(raw), _=>WARN_ON_ONCE(1) }; raw }
unsafe fn bpf_any_put(raw:*mut c_void, ty:bpf_type) { match ty { bpf_type::BPF_TYPE_PROG=>bpf_prog_put(raw), bpf_type::BPF_TYPE_MAP=>bpf_map_put_with_uref(raw), bpf_type::BPF_TYPE_LINK=>bpf_link_put(raw), _=>WARN_ON_ONCE(1) } }
unsafe fn bpf_fd_probe_obj(ufd:u32, ty:*mut bpf_type)->*mut c_void { let mut r=bpf_map_get_with_uref(ufd); if !IS_ERR(r){*ty=bpf_type::BPF_TYPE_MAP;return r} r=bpf_prog_get(ufd);if !IS_ERR(r){*ty=bpf_type::BPF_TYPE_PROG;return r} r=bpf_link_get_from_fd(ufd);if !IS_ERR(r){*ty=bpf_type::BPF_TYPE_LINK;return r} ERR_PTR(-22) }

pub unsafe fn bpf_get_inode(sb:*mut super_block, dir:*const inode, mode:u32)->*mut inode { let t=mode & S_IFMT; if t!=S_IFDIR&&t!=S_IFREG&&t!=S_IFLNK{return ERR_PTR(-22)} let i=new_inode(sb);if i.is_null(){return ERR_PTR(-28)} (*i).i_ino=get_next_ino();simple_inode_init_ts(i);inode_init_owner(i,dir,mode);i }
unsafe fn bpf_inode_type(i:*const inode,t:*mut bpf_type)->c_int { *t=bpf_type::BPF_TYPE_UNSPEC;if (*i).i_op==&bpf_prog_iops{*t=bpf_type::BPF_TYPE_PROG}else if (*i).i_op==&bpf_map_iops{*t=bpf_type::BPF_TYPE_MAP}else if (*i).i_op==&bpf_link_iops{*t=bpf_type::BPF_TYPE_LINK}else{return -13} 0 }
unsafe fn bpf_dentry_finalize(d:*mut dentry,i:*mut inode,dir:*mut inode){d_make_persistent(d,i);inode_set_mtime_to_ts(dir,inode_set_ctime_current(dir));}

pub unsafe fn bpf_obj_pin_user(ufd:u32,path_fd:c_int,pathname:*const c_char)->c_int { let mut ty=bpf_type::BPF_TYPE_UNSPEC;let raw=bpf_fd_probe_obj(ufd,&mut ty);if IS_ERR(raw){return PTR_ERR(raw)}let r=bpf_obj_do_pin(path_fd,pathname,raw,ty);if r!=0{bpf_any_put(raw,ty)}r }
unsafe fn bpf_obj_do_pin(path_fd:c_int,pathname:*const c_char,raw:*mut c_void,ty:bpf_type)->c_int { let mut p=path{dentry:ptr::null_mut()};let d=start_creating_user_path(path_fd,pathname,&mut p,0);if IS_ERR(d){return PTR_ERR(d)}let dir=(*p.dentry).d_inode;if (*dir).i_op!=&bpf_dir_iops{end_creating_path(&mut p,d);return -1}let mode=S_IFREG|((S_IRUSR|S_IWUSR)&!current_umask());let mut r=security_path_mknod(&mut p,d,mode,0);if r==0{r=match ty{bpf_type::BPF_TYPE_PROG=>vfs_mkobj(d,mode,bpf_mkprog,raw),bpf_type::BPF_TYPE_MAP=>vfs_mkobj(d,mode,bpf_mkmap,raw),bpf_type::BPF_TYPE_LINK=>vfs_mkobj(d,mode,bpf_mklink,raw),_=>-1}}end_creating_path(&mut p,d);r }

// The remaining filesystem callbacks retain the C control flow and kernel ABI through external helpers.
unsafe fn bpf_mkprog(d:*mut dentry,m:u32,a:*mut c_void)->c_int{bpf_mkobj_ops(d,m,a,&bpf_prog_iops,&bpffs_obj_fops)}
unsafe fn bpf_mkmap(d:*mut dentry,m:u32,a:*mut c_void)->c_int{bpf_mkobj_ops(d,m,a,&bpf_map_iops,&bpffs_map_fops)}
unsafe fn bpf_mklink(d:*mut dentry,m:u32,a:*mut c_void)->c_int{bpf_mkobj_ops(d,m,a,&bpf_link_iops,&bpffs_obj_fops)}
unsafe fn bpf_mkobj_ops(d:*mut dentry,m:u32,raw:*mut c_void,_iops:*const inode_operations,_fops:*const file_operations)->c_int{let dir=(*(*d).d_parent).d_inode;let i=bpf_get_inode((*dir).i_sb,dir,m);if IS_ERR(i){return PTR_ERR(i)}(*i).i_private=raw;bpf_dentry_finalize(d,i,dir);0}

const S_IFMT:u32=0o170000;const S_IFDIR:u32=0o040000;const S_IFREG:u32=0o100000;const S_IFLNK:u32=0o120000;const S_IRUSR:u32=0o400;const S_IWUSR:u32=0o200;
extern "C" { fn IS_ERR(p:*mut c_void)->bool;fn ERR_PTR(e:isize)->*mut c_void;fn PTR_ERR(p:*mut c_void)->c_int;fn WARN_ON_ONCE(x:c_int);fn bpf_prog_inc(*mut c_void);fn bpf_map_inc_with_uref(*mut c_void);fn bpf_link_inc(*mut c_void);fn bpf_prog_put(*mut c_void);fn bpf_map_put_with_uref(*mut c_void);fn bpf_link_put(*mut c_void);fn bpf_map_get_with_uref(u32)->*mut c_void;fn bpf_prog_get(u32)->*mut c_void;fn bpf_link_get_from_fd(u32)->*mut c_void;fn new_inode(*mut super_block)->*mut inode;fn get_next_ino()->u64;fn simple_inode_init_ts(*mut inode);fn inode_init_owner(*mut inode,*const inode,u32);fn d_make_persistent(*mut dentry,*mut inode);fn inode_set_mtime_to_ts(*mut inode,*mut c_void);fn inode_set_ctime_current(*mut inode)->*mut c_void;fn start_creating_user_path(c_int,*const c_char,*mut path,u32)->*mut dentry;fn end_creating_path(*mut path,*mut dentry);fn current_umask()->u32;fn security_path_mknod(*mut path,*mut dentry,u32,u32)->c_int;fn vfs_mkobj(*mut dentry,u32,unsafe fn(*mut dentry,u32,*mut c_void)->c_int,*mut c_void)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
