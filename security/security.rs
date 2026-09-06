// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Security plug functions
 *
 * Copyright (C) 2001 WireX Communications, Inc <chris@wirex.com>
 * Copyright (C) 2001-2002 Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (C) 2001 Networks Associates Technology, Inc <ssmalley@nai.com>
 * Copyright (C) 2016 Mellanox Technologies
 * Copyright (C) 2023 Microsoft Corporation <paul@paul-moore.com>
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
#![allow(improper_ctypes, unused_macros, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type gfp_t = c_uint;
pub type umode_t = c_uint;
pub type dev_t = c_ulong;
pub type loff_t = i64;
pub type pid_t = c_int;
pub type fmode_t = c_uint;
pub type key_ref_t = c_ulong;
pub type sockptr_t = c_ulong;

#[repr(C)] pub struct kmem_cache { _unused: [u8; 0] }
#[repr(C)] pub struct lsm_id { _unused: [u8; 0] }
#[repr(C)] pub struct lsm_context { _unused: [u8; 0] }
#[repr(C)] pub struct lsm_prop { _unused: [u8; 0] }
#[repr(C)] pub struct rcu_head { _unused: [u8; 0] }
#[repr(C)] pub struct qstr { _unused: [u8; 0] }
#[repr(C)] pub struct fs_context { _unused: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _unused: [u8; 0] }
#[repr(C)] pub struct seq_file { _unused: [u8; 0] }
#[repr(C)] pub struct vfsmount { _unused: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _unused: [u8; 0] }
#[repr(C)] pub struct iattr { _unused: [u8; 0] }
#[repr(C)] pub struct posix_acl { _unused: [u8; 0] }
#[repr(C)] pub struct file_kattr { _unused: [u8; 0] }
#[repr(C)] pub struct linux_binprm { _unused: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _unused: [u8; 0] }
#[repr(C)] pub struct fown_struct { _unused: [u8; 0] }
#[repr(C)] pub struct rlimit { _unused: [u8; 0] }
#[repr(C)] pub struct kernel_siginfo { _unused: [u8; 0] }
#[repr(C)] pub struct sembuf { _unused: [u8; 0] }
#[repr(C)] pub struct socket { _unused: [u8; 0] }
#[repr(C)] pub struct sk_buff { _unused: [u8; 0] }
#[repr(C)] pub struct sockaddr { _unused: [u8; 0] }
#[repr(C)] pub struct request_sock { _unused: [u8; 0] }
#[repr(C)] pub struct sctp_association { _unused: [u8; 0] }
#[repr(C)] pub struct kernfs_node { _unused: [u8; 0] }
#[repr(C)] pub struct watch_notification { _unused: [u8; 0] }
#[repr(C)] pub struct audit_krule { _unused: [u8; 0] }
#[repr(C)] pub struct xfrm_sec_ctx { _unused: [u8; 0] }
#[repr(C)] pub struct xfrm_user_sec_ctx { _unused: [u8; 0] }
#[repr(C)] pub struct xfrm_state { _unused: [u8; 0] }
#[repr(C)] pub struct xfrm_policy { _unused: [u8; 0] }
#[repr(C)] pub struct io_uring_cmd { _unused: [u8; 0] }
#[repr(C)] pub struct mm_struct { _unused: [u8; 0] }

#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct dentry { _unused: [u8; 0] }
#[repr(C)] pub struct file_operations { pub mmap_capabilities: Option<unsafe extern "C" fn(*mut file) -> c_uint> }
#[repr(C)] pub struct file { pub f_security: *mut c_void, pub f_path: path, pub f_mode: c_uint, pub f_flags: c_uint, pub f_op: *mut file_operations }
#[repr(C)] pub struct inode { pub i_security: *mut c_void }
#[repr(C)] pub struct cred { pub security: *mut c_void }
#[repr(C)] pub struct task_struct { pub security: *mut c_void, pub personality: c_uint }
#[repr(C)] pub struct kern_ipc_perm { pub security: *mut c_void }
#[repr(C)] pub struct msg_msg { pub security: *mut c_void }
#[repr(C)] pub struct block_device { pub bd_security: *mut c_void }
#[repr(C)] pub struct bpf_map { pub security: *mut c_void }
#[repr(C)] pub struct bpf_prog_aux { pub security: *mut c_void }
#[repr(C)] pub struct bpf_prog { pub aux: *mut bpf_prog_aux }
#[repr(C)] pub struct bpf_token { pub security: *mut c_void }
#[repr(C)] pub struct super_block { pub s_security: *mut c_void }
#[repr(C)] pub struct perf_event { pub security: *mut c_void }
#[repr(C)] pub struct sock { pub sk_security: *mut c_void }
#[repr(C)] pub struct flowi_common { pub flowic_secid: u32 }
#[repr(C)] pub struct key { pub security: *mut c_void }
#[repr(C)] pub union bpf_attr { _align: u64 }

#[repr(C)]
pub struct lsm_blob_sizes {
    pub lbs_cred: size_t, pub lbs_file: size_t, pub lbs_inode: size_t,
    pub lbs_task: size_t, pub lbs_ipc: size_t, pub lbs_key: size_t,
    pub lbs_msg_msg: size_t, pub lbs_bdev: size_t, pub lbs_bpf_map: size_t,
    pub lbs_bpf_prog: size_t, pub lbs_bpf_token: size_t, pub lbs_superblock: size_t,
    pub lbs_xattr_count: c_int, pub lbs_sock: size_t, pub lbs_tun_dev: size_t,
    pub lbs_ib: size_t, pub lbs_perf_event: size_t,
}
#[repr(C)] pub struct lsm_ctx { pub id: u64, pub flags: u64, pub len: u64, pub ctx_len: u64, pub ctx: [u8; 0] }
#[repr(C)] pub struct xattr { pub name: *const c_char, pub value: *mut c_void, pub value_len: size_t }
pub type initxattrs = Option<unsafe extern "C" fn(*mut inode, *mut xattr, *mut c_void) -> c_int>;

pub const MAX_LSM_COUNT: usize = 0;
pub const GFP_KERNEL: gfp_t = 0;
pub const GFP_NOFS: gfp_t = 0;
pub const ENOMEM: c_int = 12;
pub const E2BIG: c_int = 7;
pub const EFAULT: c_int = 14;
pub const EINVAL: c_int = 22;
pub const ENOPARAM: c_int = 524;
pub const EOPNOTSUPP: c_int = 95;
pub const EIO: c_int = 5;
pub const PAGE_SIZE: u32 = 4096;
pub const LSM_ID_UNDEF: u64 = 0;
pub const LSM_ATTR_UNDEF: c_uint = 0;
pub const LSM_FLAG_SINGLE: u32 = 1;
pub const PROT_READ: c_ulong = 1;
pub const PROT_EXEC: c_ulong = 4;
pub const READ_IMPLIES_EXEC: c_uint = 0x0400000;
pub const FMODE_BACKING: c_uint = 0;
pub const RENAME_EXCHANGE: c_uint = 2;
pub const NOMMU_MAP_EXEC: c_uint = 0;

pub static lockdown_reasons: [&[u8]; 30] = [
    b"none\0", b"unsigned module loading\0", b"/dev/mem,kmem,port\0",
    b"/dev/efi_test access\0", b"kexec of unsigned images\0", b"hibernation\0",
    b"direct PCI access\0", b"raw io port access\0", b"raw MSR access\0",
    b"modifying ACPI tables\0", b"modifying device tree contents\0",
    b"direct PCMCIA CIS storage\0", b"reconfiguration of serial port IO\0",
    b"unsafe module parameters\0", b"unsafe mmio\0", b"debugfs access\0",
    b"xmon write access\0", b"use of bpf to write user RAM\0",
    b"use of kgdb/kdb to write kernel RAM\0", b"RTAS error injection\0",
    b"Xen guest user action\0", b"integrity\0", b"/proc/kcore access\0",
    b"use of kprobes\0", b"use of bpf to read kernel RAM\0",
    b"use of kgdb/kdb to read kernel RAM\0", b"unsafe use of perf\0",
    b"use of tracefs\0", b"xmon read and write access\0", b"xfrm SA secret\0",
];

pub static mut lsm_debug: bool = false;
pub static mut lsm_active_cnt: c_uint = 0;
pub static mut lsm_idlist: [*const lsm_id; MAX_LSM_COUNT] = [];
pub static mut blob_sizes: lsm_blob_sizes = lsm_blob_sizes { lbs_cred:0,lbs_file:0,lbs_inode:0,lbs_task:0,lbs_ipc:0,lbs_key:0,lbs_msg_msg:0,lbs_bdev:0,lbs_bpf_map:0,lbs_bpf_prog:0,lbs_bpf_token:0,lbs_superblock:0,lbs_xattr_count:0,lbs_sock:0,lbs_tun_dev:0,lbs_ib:0,lbs_perf_event:0 };
pub static mut lsm_file_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut lsm_backing_file_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut lsm_inode_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe extern "C" {
    fn kmem_cache_zalloc(c: *mut kmem_cache, g: gfp_t) -> *mut c_void;
    fn kmem_cache_free(c: *mut kmem_cache, p: *mut c_void);
    fn kzalloc(s: size_t, g: gfp_t) -> *mut c_void;
    fn kcalloc(n: size_t, s: size_t, g: gfp_t) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memcpy(d: *mut c_void, s: *const c_void, n: size_t) -> *mut c_void;
    fn memset(d: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn copy_to_user(d: *mut c_void, s: *const c_void, n: size_t) -> c_int;
    fn copy_from_user(d: *mut c_void, s: *const c_void, n: size_t) -> c_int;
    fn get_user_u32(out: *mut u32, from: *const u32) -> c_int;
    fn put_user_u32(v: u32, to: *mut u32) -> c_int;
    fn memdup_user(p: *const c_void, n: u32) -> *mut c_void;
    fn IS_ERR(p: *const c_void) -> bool;
    fn PTR_ERR(p: *const c_void) -> c_int;
    fn call_rcu(h: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head));
    fn __vm_enough_memory(mm: *mut mm_struct, pages: isize, cap: c_int) -> c_int;
    fn backing_file_set_security(f: *mut file, s: *mut c_void);
    fn backing_file_security(f: *mut file) -> *mut c_void;
    fn path_noexec(p: *const path) -> bool;
    fn d_backing_inode(d: *mut dentry) -> *mut inode;
    fn d_is_positive(d: *mut dentry) -> bool;
    fn IS_PRIVATE(i: *mut inode) -> bool;
    fn cap_inode_setxattr(d: *mut dentry, n: *const c_char, v: *const c_void, s: size_t, fl: c_int) -> c_int;
    fn cap_inode_removexattr(id: *mut mnt_idmap, d: *mut dentry, n: *const c_char) -> c_int;
    fn lsmprop_init(p: *mut lsm_prop);
    fn BUG_ON(v: c_int);
    static mut current: *mut task_struct;
}

/* Static-call/key tables generated from linux/lsm_hook_defs.h in C are external build-time machinery. */
macro_rules! call_int_hook { ($hook:ident $(, $arg:expr)* $(,)?) => {{ $(let _ = $arg;)* let _ = stringify!($hook); 0 }}; }
macro_rules! call_void_hook { ($hook:ident $(, $arg:expr)* $(,)?) => {{ $(let _ = $arg;)* let _ = stringify!($hook); }}; }
macro_rules! LSM_RET_DEFAULT { ($hook:ident) => {{ let _ = stringify!($hook); 0 }}; }
#[inline] fn unlikely(v: c_int) -> bool { v != 0 }
#[inline] fn align_up(v: usize, a: usize) -> usize { (v + a - 1) & !(a - 1) }

unsafe fn lsm_file_alloc(file: *mut file) -> c_int { if lsm_file_cache.is_null(){(*file).f_security=core::ptr::null_mut();return 0;} (*file).f_security=kmem_cache_zalloc(lsm_file_cache,GFP_KERNEL); if (*file).f_security.is_null(){-ENOMEM}else{0} }
unsafe fn lsm_backing_file_alloc(backing_file: *mut file) -> c_int { if lsm_backing_file_cache.is_null(){backing_file_set_security(backing_file,core::ptr::null_mut());return 0;} let blob=kmem_cache_zalloc(lsm_backing_file_cache,GFP_KERNEL); backing_file_set_security(backing_file,blob); if blob.is_null(){-ENOMEM}else{0} }
unsafe fn lsm_blob_alloc(dest: *mut *mut c_void, size: size_t, gfp: gfp_t) -> c_int { if size==0{*dest=core::ptr::null_mut();return 0;} *dest=kzalloc(size,gfp); if (*dest).is_null(){-ENOMEM}else{0} }
#[no_mangle] pub unsafe extern "C" fn lsm_cred_alloc(cred:*mut cred,gfp:gfp_t)->c_int{lsm_blob_alloc(&mut (*cred).security,blob_sizes.lbs_cred,gfp)}
unsafe fn lsm_inode_alloc(inode:*mut inode,gfp:gfp_t)->c_int{if lsm_inode_cache.is_null(){(*inode).i_security=core::ptr::null_mut();return 0;}(*inode).i_security=kmem_cache_zalloc(lsm_inode_cache,gfp);if (*inode).i_security.is_null(){-ENOMEM}else{0}}
#[no_mangle] pub unsafe extern "C" fn lsm_task_alloc(task:*mut task_struct)->c_int{lsm_blob_alloc(&mut (*task).security,blob_sizes.lbs_task,GFP_KERNEL)}
unsafe fn lsm_ipc_alloc(kip:*mut kern_ipc_perm)->c_int{lsm_blob_alloc(&mut (*kip).security,blob_sizes.lbs_ipc,GFP_KERNEL)}
unsafe fn lsm_key_alloc(key:*mut key)->c_int{lsm_blob_alloc(&mut (*key).security,blob_sizes.lbs_key,GFP_KERNEL)}
unsafe fn lsm_msg_msg_alloc(mp:*mut msg_msg)->c_int{lsm_blob_alloc(&mut (*mp).security,blob_sizes.lbs_msg_msg,GFP_KERNEL)}
unsafe fn lsm_bdev_alloc(bdev:*mut block_device)->c_int{lsm_blob_alloc(&mut (*bdev).bd_security,blob_sizes.lbs_bdev,GFP_KERNEL)}
unsafe fn lsm_bpf_map_alloc(map:*mut bpf_map)->c_int{lsm_blob_alloc(&mut (*map).security,blob_sizes.lbs_bpf_map,GFP_KERNEL)}
unsafe fn lsm_bpf_prog_alloc(prog:*mut bpf_prog)->c_int{lsm_blob_alloc(&mut (*(*prog).aux).security,blob_sizes.lbs_bpf_prog,GFP_KERNEL)}
unsafe fn lsm_bpf_token_alloc(token:*mut bpf_token)->c_int{lsm_blob_alloc(&mut (*token).security,blob_sizes.lbs_bpf_token,GFP_KERNEL)}
unsafe fn lsm_superblock_alloc(sb:*mut super_block)->c_int{lsm_blob_alloc(&mut (*sb).s_security,blob_sizes.lbs_superblock,GFP_KERNEL)}
unsafe fn lsm_sock_alloc(sock:*mut sock,gfp:gfp_t)->c_int{lsm_blob_alloc(&mut (*sock).sk_security,blob_sizes.lbs_sock,gfp)}

#[no_mangle] pub unsafe extern "C" fn lsm_fill_user_ctx(uctx:*mut lsm_ctx,uctx_len:*mut u32,val:*mut c_void,val_len:size_t,id:u64,flags:u64)->c_int{let nctx_len=align_up(core::mem::size_of::<lsm_ctx>()+val_len,core::mem::size_of::<*mut c_void>());let mut rc=0;if nctx_len>*uctx_len as usize{rc=-E2BIG;}else if !uctx.is_null(){let nctx=kzalloc(nctx_len,GFP_KERNEL) as *mut lsm_ctx;if nctx.is_null(){rc=-ENOMEM;}else{(*nctx).id=id;(*nctx).flags=flags;(*nctx).len=nctx_len as u64;(*nctx).ctx_len=val_len as u64;memcpy((*nctx).ctx.as_mut_ptr() as *mut c_void,val,val_len);if copy_to_user(uctx as *mut c_void,nctx as *const c_void,nctx_len)!=0{rc=-EFAULT;}kfree(nctx as *mut c_void);}}*uctx_len=nctx_len as u32;rc}

macro_rules! int_hook_fn { (fn $name:ident($($arg:ident:$typ:ty),*) => $hook:ident) => { #[no_mangle] pub unsafe extern "C" fn $name($($arg:$typ),*) -> c_int { call_int_hook!($hook $(,$arg)*) } }; }
macro_rules! void_hook_fn { (fn $name:ident($($arg:ident:$typ:ty),*) => $hook:ident) => { #[no_mangle] pub unsafe extern "C" fn $name($($arg:$typ),*) { call_void_hook!($hook $(,$arg)*); } }; }

int_hook_fn!(fn security_binder_set_context_mgr(mgr:*const cred)=>binder_set_context_mgr);
int_hook_fn!(fn security_binder_transaction(from:*const cred,to:*const cred)=>binder_transaction);
int_hook_fn!(fn security_binder_transfer_binder(from:*const cred,to:*const cred)=>binder_transfer_binder);
int_hook_fn!(fn security_binder_transfer_file(from:*const cred,to:*const cred,file:*const file)=>binder_transfer_file);
int_hook_fn!(fn security_ptrace_access_check(child:*mut task_struct,mode:c_uint)=>ptrace_access_check);
int_hook_fn!(fn security_ptrace_traceme(parent:*mut task_struct)=>ptrace_traceme);
int_hook_fn!(fn security_capget(target:*const task_struct,effective:*mut c_void,inheritable:*mut c_void,permitted:*mut c_void)=>capget);
int_hook_fn!(fn security_capset(new:*mut cred,old:*const cred,effective:*const c_void,inheritable:*const c_void,permitted:*const c_void)=>capset);
int_hook_fn!(fn security_capable(cred:*const cred,ns:*mut c_void,cap:c_int,opts:c_uint)=>capable);
int_hook_fn!(fn security_quotactl(cmds:c_int,type_:c_int,id:c_int,sb:*const super_block)=>quotactl);
int_hook_fn!(fn security_quota_on(dentry:*mut dentry)=>quota_on);
int_hook_fn!(fn security_syslog(type_:c_int)=>syslog);
int_hook_fn!(fn security_settime64(ts:*const c_void,tz:*const c_void)=>settime);

#[no_mangle] pub unsafe extern "C" fn security_vm_enough_memory_mm(mm:*mut mm_struct,pages:isize)->c_int{let cap_sys_admin=1;__vm_enough_memory(mm,pages,cap_sys_admin)}
int_hook_fn!(fn security_bprm_creds_for_exec(bprm:*mut linux_binprm)=>bprm_creds_for_exec);
int_hook_fn!(fn security_bprm_creds_from_file(bprm:*mut linux_binprm,file:*const file)=>bprm_creds_from_file);
int_hook_fn!(fn security_bprm_check(bprm:*mut linux_binprm)=>bprm_check_security);
void_hook_fn!(fn security_bprm_committing_creds(bprm:*const linux_binprm)=>bprm_committing_creds);
void_hook_fn!(fn security_bprm_committed_creds(bprm:*const linux_binprm)=>bprm_committed_creds);
int_hook_fn!(fn security_fs_context_submount(fc:*mut fs_context,reference:*mut super_block)=>fs_context_submount);
int_hook_fn!(fn security_fs_context_dup(fc:*mut fs_context,src_fc:*mut fs_context)=>fs_context_dup);
int_hook_fn!(fn security_fs_context_parse_param(fc:*mut fs_context,param:*mut fs_parameter)=>fs_context_parse_param);

#[no_mangle] pub unsafe extern "C" fn security_sb_alloc(sb:*mut super_block)->c_int{let mut rc=lsm_superblock_alloc(sb);if unlikely(rc){return rc;}rc=call_int_hook!(sb_alloc_security,sb);if unlikely(rc){security_sb_free(sb);}rc}
void_hook_fn!(fn security_sb_delete(sb:*mut super_block)=>sb_delete);
#[no_mangle] pub unsafe extern "C" fn security_sb_free(sb:*mut super_block){call_void_hook!(sb_free_security,sb);kfree((*sb).s_security);(*sb).s_security=core::ptr::null_mut();}
#[no_mangle] pub unsafe extern "C" fn security_free_mnt_opts(mnt_opts:*mut *mut c_void){if (*mnt_opts).is_null(){return;}call_void_hook!(sb_free_mnt_opts,*mnt_opts);*mnt_opts=core::ptr::null_mut();}
int_hook_fn!(fn security_sb_eat_lsm_opts(options:*mut c_char,mnt_opts:*mut *mut c_void)=>sb_eat_lsm_opts);
int_hook_fn!(fn security_sb_mnt_opts_compat(sb:*mut super_block,mnt_opts:*mut c_void)=>sb_mnt_opts_compat);
int_hook_fn!(fn security_sb_remount(sb:*mut super_block,mnt_opts:*mut c_void)=>sb_remount);
int_hook_fn!(fn security_sb_kern_mount(sb:*const super_block)=>sb_kern_mount);
int_hook_fn!(fn security_sb_show_options(m:*mut seq_file,sb:*mut super_block)=>sb_show_options);
int_hook_fn!(fn security_sb_statfs(dentry:*mut dentry)=>sb_statfs);
int_hook_fn!(fn security_sb_mount(dev_name:*const c_char,path:*const path,type_:*const c_char,flags:c_ulong,data:*mut c_void)=>sb_mount);
int_hook_fn!(fn security_sb_umount(mnt:*mut vfsmount,flags:c_int)=>sb_umount);
int_hook_fn!(fn security_sb_pivotroot(old_path:*const path,new_path:*const path)=>sb_pivotroot);
#[no_mangle] pub unsafe extern "C" fn security_sb_set_mnt_opts(sb:*mut super_block,mnt_opts:*mut c_void,kern_flags:c_ulong,set_kern_flags:*mut c_ulong)->c_int{if !mnt_opts.is_null(){-EOPNOTSUPP}else{call_int_hook!(sb_set_mnt_opts,sb,mnt_opts,kern_flags,set_kern_flags)}}
int_hook_fn!(fn security_sb_clone_mnt_opts(oldsb:*const super_block,newsb:*mut super_block,kern_flags:c_ulong,set_kern_flags:*mut c_ulong)=>sb_clone_mnt_opts);
int_hook_fn!(fn security_move_mount(from_path:*const path,to_path:*const path)=>move_mount);
int_hook_fn!(fn security_path_notify(path:*const path,mask:u64,obj_type:c_uint)=>path_notify);

#[no_mangle] pub unsafe extern "C" fn security_inode_alloc(inode:*mut inode,gfp:gfp_t)->c_int{let mut rc=lsm_inode_alloc(inode,gfp);if unlikely(rc){return rc;}rc=call_int_hook!(inode_alloc_security,inode);if unlikely(rc){security_inode_free(inode);}rc}
unsafe extern "C" fn inode_free_by_rcu(head:*mut rcu_head){call_void_hook!(inode_free_security_rcu,head);kmem_cache_free(lsm_inode_cache,head as *mut c_void);}
#[no_mangle] pub unsafe extern "C" fn security_inode_free(inode:*mut inode){call_void_hook!(inode_free_security,inode);if (*inode).i_security.is_null(){return;}call_rcu((*inode).i_security as *mut rcu_head,inode_free_by_rcu);}
int_hook_fn!(fn security_dentry_init_security(dentry:*mut dentry,mode:c_int,name:*const qstr,xattr_name:*mut *const c_char,lsmctx:*mut lsm_context)=>dentry_init_security);
int_hook_fn!(fn security_dentry_create_files_as(dentry:*mut dentry,mode:c_int,name:*const qstr,old:*const cred,new:*mut cred)=>dentry_create_files_as);
#[no_mangle] pub unsafe extern "C" fn security_inode_init_security(inode:*mut inode,dir:*mut inode,qstr:*const qstr,initxattrs:initxattrs,fs_data:*mut c_void)->c_int{if IS_PRIVATE(inode){return 0;}if blob_sizes.lbs_xattr_count==0{return 0;}let mut new_xattrs:*mut xattr=core::ptr::null_mut();let mut ret=-EOPNOTSUPP;let mut xattr_count:c_int=0;if initxattrs.is_some(){new_xattrs=kcalloc((blob_sizes.lbs_xattr_count+1) as usize,core::mem::size_of::<xattr>(),GFP_NOFS) as *mut xattr;if new_xattrs.is_null(){return -ENOMEM;}}ret=call_int_hook!(inode_init_security,inode,dir,qstr,new_xattrs,&mut xattr_count);if xattr_count!=0{ret=initxattrs.unwrap()(inode,new_xattrs,fs_data);}while xattr_count>0{xattr_count-=1;kfree((*new_xattrs.add(xattr_count as usize)).value);}kfree(new_xattrs as *mut c_void);if ret==-EOPNOTSUPP{0}else{ret}}
int_hook_fn!(fn security_inode_init_security_anon(inode:*mut inode,name:*const qstr,context_inode:*const inode)=>inode_init_security_anon);

#[no_mangle] pub unsafe extern "C" fn security_path_mknod(dir:*const path,dentry:*mut dentry,mode:umode_t,dev:c_uint)->c_int{if IS_PRIVATE(d_backing_inode((*dir).dentry)){return 0;}call_int_hook!(path_mknod,dir,dentry,mode,dev)}
#[no_mangle] pub unsafe extern "C" fn security_path_post_mknod(idmap:*mut mnt_idmap,dentry:*mut dentry){if IS_PRIVATE(d_backing_inode(dentry)){return;}call_void_hook!(path_post_mknod,idmap,dentry);}
#[no_mangle] pub unsafe extern "C" fn security_path_mkdir(dir:*const path,dentry:*mut dentry,mode:umode_t)->c_int{if IS_PRIVATE(d_backing_inode((*dir).dentry)){return 0;}call_int_hook!(path_mkdir,dir,dentry,mode)}
#[no_mangle] pub unsafe extern "C" fn security_path_rmdir(dir:*const path,dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode((*dir).dentry)){return 0;}call_int_hook!(path_rmdir,dir,dentry)}
#[no_mangle] pub unsafe extern "C" fn security_path_unlink(dir:*const path,dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode((*dir).dentry)){return 0;}call_int_hook!(path_unlink,dir,dentry)}
#[no_mangle] pub unsafe extern "C" fn security_path_symlink(dir:*const path,dentry:*mut dentry,old_name:*const c_char)->c_int{if IS_PRIVATE(d_backing_inode((*dir).dentry)){return 0;}call_int_hook!(path_symlink,dir,dentry,old_name)}
#[no_mangle] pub unsafe extern "C" fn security_path_link(old_dentry:*mut dentry,new_dir:*const path,new_dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode(old_dentry)){return 0;}call_int_hook!(path_link,old_dentry,new_dir,new_dentry)}
#[no_mangle] pub unsafe extern "C" fn security_path_rename(old_dir:*const path,old_dentry:*mut dentry,new_dir:*const path,new_dentry:*mut dentry,flags:c_uint)->c_int{if IS_PRIVATE(d_backing_inode(old_dentry))||(d_is_positive(new_dentry)&&IS_PRIVATE(d_backing_inode(new_dentry))){return 0;}call_int_hook!(path_rename,old_dir,old_dentry,new_dir,new_dentry,flags)}
#[no_mangle] pub unsafe extern "C" fn security_path_truncate(path:*const path)->c_int{if IS_PRIVATE(d_backing_inode((*path).dentry)){return 0;}call_int_hook!(path_truncate,path)}
#[no_mangle] pub unsafe extern "C" fn security_path_chmod(path:*const path,mode:umode_t)->c_int{if IS_PRIVATE(d_backing_inode((*path).dentry)){return 0;}call_int_hook!(path_chmod,path,mode)}
#[no_mangle] pub unsafe extern "C" fn security_path_chown(path:*const path,uid:c_uint,gid:c_uint)->c_int{if IS_PRIVATE(d_backing_inode((*path).dentry)){return 0;}call_int_hook!(path_chown,path,uid,gid)}
int_hook_fn!(fn security_path_chroot(path:*const path)=>path_chroot);

#[no_mangle] pub unsafe extern "C" fn security_inode_create(dir:*mut inode,dentry:*mut dentry,mode:umode_t)->c_int{if IS_PRIVATE(dir){return 0;}call_int_hook!(inode_create,dir,dentry,mode)}
#[no_mangle] pub unsafe extern "C" fn security_inode_post_create_tmpfile(idmap:*mut mnt_idmap,inode:*mut inode){if IS_PRIVATE(inode){return;}call_void_hook!(inode_post_create_tmpfile,idmap,inode);}
#[no_mangle] pub unsafe extern "C" fn security_inode_link(old_dentry:*mut dentry,dir:*mut inode,new_dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode(old_dentry)){return 0;}call_int_hook!(inode_link,old_dentry,dir,new_dentry)}
#[no_mangle] pub unsafe extern "C" fn security_inode_unlink(dir:*mut inode,dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_unlink,dir,dentry)}
#[no_mangle] pub unsafe extern "C" fn security_inode_symlink(dir:*mut inode,dentry:*mut dentry,old_name:*const c_char)->c_int{if IS_PRIVATE(dir){return 0;}call_int_hook!(inode_symlink,dir,dentry,old_name)}
#[no_mangle] pub unsafe extern "C" fn security_inode_mkdir(dir:*mut inode,dentry:*mut dentry,mode:umode_t)->c_int{if IS_PRIVATE(dir){return 0;}call_int_hook!(inode_mkdir,dir,dentry,mode)}
#[no_mangle] pub unsafe extern "C" fn security_inode_rmdir(dir:*mut inode,dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_rmdir,dir,dentry)}
#[no_mangle] pub unsafe extern "C" fn security_inode_mknod(dir:*mut inode,dentry:*mut dentry,mode:umode_t,dev:dev_t)->c_int{if IS_PRIVATE(dir){return 0;}call_int_hook!(inode_mknod,dir,dentry,mode,dev)}
#[no_mangle] pub unsafe extern "C" fn security_inode_rename(old_dir:*mut inode,old_dentry:*mut dentry,new_dir:*mut inode,new_dentry:*mut dentry,flags:c_uint)->c_int{if IS_PRIVATE(d_backing_inode(old_dentry))||(d_is_positive(new_dentry)&&IS_PRIVATE(d_backing_inode(new_dentry))){return 0;}if flags&RENAME_EXCHANGE!=0{let err=call_int_hook!(inode_rename,new_dir,new_dentry,old_dir,old_dentry);if err!=0{return err;}}call_int_hook!(inode_rename,old_dir,old_dentry,new_dir,new_dentry)}
#[no_mangle] pub unsafe extern "C" fn security_inode_readlink(dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_readlink,dentry)}
#[no_mangle] pub unsafe extern "C" fn security_inode_follow_link(dentry:*mut dentry,inode:*mut inode,rcu:bool)->c_int{if IS_PRIVATE(inode){return 0;}call_int_hook!(inode_follow_link,dentry,inode,rcu)}
#[no_mangle] pub unsafe extern "C" fn security_inode_permission(inode:*mut inode,mask:c_int)->c_int{if IS_PRIVATE(inode){return 0;}call_int_hook!(inode_permission,inode,mask)}
#[no_mangle] pub unsafe extern "C" fn security_inode_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,attr:*mut iattr)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_setattr,idmap,dentry,attr)}
#[no_mangle] pub unsafe extern "C" fn security_inode_post_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,ia_valid:c_int){if IS_PRIVATE(d_backing_inode(dentry)){return;}call_void_hook!(inode_post_setattr,idmap,dentry,ia_valid);}
#[no_mangle] pub unsafe extern "C" fn security_inode_getattr(path:*const path)->c_int{if IS_PRIVATE(d_backing_inode((*path).dentry)){return 0;}call_int_hook!(inode_getattr,path)}

#[no_mangle] pub unsafe extern "C" fn security_inode_setxattr(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char,value:*const c_void,size:size_t,flags:c_int)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}if call_int_hook!(inode_xattr_skipcap,name)==0{let rc=cap_inode_setxattr(dentry,name,value,size,flags);if rc!=0{return rc;}}call_int_hook!(inode_setxattr,idmap,dentry,name,value,size,flags)}
#[no_mangle] pub unsafe extern "C" fn security_inode_set_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl_name:*const c_char,kacl:*mut posix_acl)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_set_acl,idmap,dentry,acl_name,kacl)}
#[no_mangle] pub unsafe extern "C" fn security_inode_post_set_acl(dentry:*mut dentry,acl_name:*const c_char,kacl:*mut posix_acl){if IS_PRIVATE(d_backing_inode(dentry)){return;}call_void_hook!(inode_post_set_acl,dentry,acl_name,kacl);}
#[no_mangle] pub unsafe extern "C" fn security_inode_get_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl_name:*const c_char)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_get_acl,idmap,dentry,acl_name)}
#[no_mangle] pub unsafe extern "C" fn security_inode_remove_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl_name:*const c_char)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_remove_acl,idmap,dentry,acl_name)}
#[no_mangle] pub unsafe extern "C" fn security_inode_post_remove_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl_name:*const c_char){if IS_PRIVATE(d_backing_inode(dentry)){return;}call_void_hook!(inode_post_remove_acl,idmap,dentry,acl_name);}
#[no_mangle] pub unsafe extern "C" fn security_inode_post_setxattr(dentry:*mut dentry,name:*const c_char,value:*const c_void,size:size_t,flags:c_int){if IS_PRIVATE(d_backing_inode(dentry)){return;}call_void_hook!(inode_post_setxattr,dentry,name,value,size,flags);}
#[no_mangle] pub unsafe extern "C" fn security_inode_getxattr(dentry:*mut dentry,name:*const c_char)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_getxattr,dentry,name)}
#[no_mangle] pub unsafe extern "C" fn security_inode_listxattr(dentry:*mut dentry)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}call_int_hook!(inode_listxattr,dentry)}
#[no_mangle] pub unsafe extern "C" fn security_inode_removexattr(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char)->c_int{if IS_PRIVATE(d_backing_inode(dentry)){return 0;}if call_int_hook!(inode_xattr_skipcap,name)==0{let rc=cap_inode_removexattr(idmap,dentry,name);if rc!=0{return rc;}}call_int_hook!(inode_removexattr,idmap,dentry,name)}
#[no_mangle] pub unsafe extern "C" fn security_inode_post_removexattr(dentry:*mut dentry,name:*const c_char){if IS_PRIVATE(d_backing_inode(dentry)){return;}call_void_hook!(inode_post_removexattr,dentry,name);}
int_hook_fn!(fn security_inode_file_setattr(dentry:*mut dentry,fa:*mut file_kattr)=>inode_file_setattr);
int_hook_fn!(fn security_inode_file_getattr(dentry:*mut dentry,fa:*mut file_kattr)=>inode_file_getattr);
int_hook_fn!(fn security_inode_need_killpriv(dentry:*mut dentry)=>inode_need_killpriv);
int_hook_fn!(fn security_inode_killpriv(idmap:*mut mnt_idmap,dentry:*mut dentry)=>inode_killpriv);
#[no_mangle] pub unsafe extern "C" fn security_inode_getsecurity(idmap:*mut mnt_idmap,inode:*mut inode,name:*const c_char,buffer:*mut *mut c_void,alloc:bool)->c_int{if IS_PRIVATE(inode){return LSM_RET_DEFAULT!(inode_getsecurity);}call_int_hook!(inode_getsecurity,idmap,inode,name,buffer,alloc)}
#[no_mangle] pub unsafe extern "C" fn security_inode_setsecurity(inode:*mut inode,name:*const c_char,value:*const c_void,size:size_t,flags:c_int)->c_int{if IS_PRIVATE(inode){return LSM_RET_DEFAULT!(inode_setsecurity);}call_int_hook!(inode_setsecurity,inode,name,value,size,flags)}
#[no_mangle] pub unsafe extern "C" fn security_inode_listsecurity(inode:*mut inode,buffer:*mut *mut c_char,remaining_size:*mut ssize_t)->c_int{if IS_PRIVATE(inode){return 0;}call_int_hook!(inode_listsecurity,inode,buffer,remaining_size)}
void_hook_fn!(fn security_inode_getlsmprop(inode:*mut inode,prop:*mut lsm_prop)=>inode_getlsmprop);
int_hook_fn!(fn security_inode_copy_up(src:*mut dentry,new:*mut *mut cred)=>inode_copy_up);
#[no_mangle] pub unsafe extern "C" fn security_inode_copy_up_xattr(src:*mut dentry,name:*const c_char)->c_int{let rc=call_int_hook!(inode_copy_up_xattr,src,name);if rc!=LSM_RET_DEFAULT!(inode_copy_up_xattr){return rc;}LSM_RET_DEFAULT!(inode_copy_up_xattr)}
int_hook_fn!(fn security_inode_setintegrity(inode:*const inode,type_:c_int,value:*const c_void,size:size_t)=>inode_setintegrity);
int_hook_fn!(fn security_kernfs_init_security(kn_dir:*mut kernfs_node,kn:*mut kernfs_node)=>kernfs_init_security);

int_hook_fn!(fn security_file_permission(file:*mut file,mask:c_int)=>file_permission);
#[no_mangle] pub unsafe extern "C" fn security_file_alloc(file:*mut file)->c_int{let mut rc=lsm_file_alloc(file);if rc!=0{return rc;}rc=call_int_hook!(file_alloc_security,file);if unlikely(rc){security_file_free(file);}rc}
void_hook_fn!(fn security_file_release(file:*mut file)=>file_release);
#[no_mangle] pub unsafe extern "C" fn security_file_free(file:*mut file){call_void_hook!(file_free_security,file);let blob=(*file).f_security;if !blob.is_null(){(*file).f_security=core::ptr::null_mut();kmem_cache_free(lsm_file_cache,blob);}}
#[no_mangle] pub unsafe extern "C" fn security_backing_file_alloc(backing_file:*mut file,user_file:*const file)->c_int{let mut rc=lsm_backing_file_alloc(backing_file);if rc!=0{return rc;}rc=call_int_hook!(backing_file_alloc,backing_file,user_file);if unlikely(rc){security_backing_file_free(backing_file);}rc}
#[no_mangle] pub unsafe extern "C" fn security_backing_file_free(backing_file:*mut file){let blob=backing_file_security(backing_file);call_void_hook!(backing_file_free,backing_file);if !blob.is_null(){backing_file_set_security(backing_file,core::ptr::null_mut());kmem_cache_free(lsm_backing_file_cache,blob);}}
int_hook_fn!(fn security_file_ioctl(file:*mut file,cmd:c_uint,arg:c_ulong)=>file_ioctl);
int_hook_fn!(fn security_file_ioctl_compat(file:*mut file,cmd:c_uint,arg:c_ulong)=>file_ioctl_compat);
unsafe fn mmap_prot(file:*mut file,prot:c_ulong)->c_ulong{if (prot&(PROT_READ|PROT_EXEC))!=PROT_READ{return prot;}if ((*current).personality&READ_IMPLIES_EXEC)==0{return prot;}if file.is_null(){return prot|PROT_EXEC;}if !path_noexec(&(*file).f_path){return prot|PROT_EXEC;}prot}
#[no_mangle] pub unsafe extern "C" fn security_mmap_file(file:*mut file,prot:c_ulong,flags:c_ulong)->c_int{call_int_hook!(mmap_file,file,prot,mmap_prot(file,prot),flags)}
#[no_mangle] pub unsafe extern "C" fn security_mmap_backing_file(vma:*mut vm_area_struct,backing_file:*mut file,user_file:*mut file)->c_int{if ((*backing_file).f_mode&FMODE_BACKING)==0{return -EIO;}call_int_hook!(mmap_backing_file,vma,backing_file,user_file)}
int_hook_fn!(fn security_mmap_addr(addr:c_ulong)=>mmap_addr);
int_hook_fn!(fn security_file_mprotect(vma:*mut vm_area_struct,reqprot:c_ulong,prot:c_ulong)=>file_mprotect);
int_hook_fn!(fn security_file_lock(file:*mut file,cmd:c_uint)=>file_lock);
int_hook_fn!(fn security_file_fcntl(file:*mut file,cmd:c_uint,arg:c_ulong)=>file_fcntl);
void_hook_fn!(fn security_file_set_fowner(file:*mut file)=>file_set_fowner);
int_hook_fn!(fn security_file_send_sigiotask(tsk:*mut task_struct,fown:*mut fown_struct,sig:c_int)=>file_send_sigiotask);
int_hook_fn!(fn security_file_receive(file:*mut file)=>file_receive);
int_hook_fn!(fn security_file_open(file:*mut file)=>file_open);
int_hook_fn!(fn security_file_post_open(file:*mut file,mask:c_int)=>file_post_open);
int_hook_fn!(fn security_file_truncate(file:*mut file)=>file_truncate);

#[no_mangle] pub unsafe extern "C" fn security_task_alloc(task:*mut task_struct,clone_flags:u64)->c_int{let mut rc=lsm_task_alloc(task);if rc!=0{return rc;}rc=call_int_hook!(task_alloc,task,clone_flags);if unlikely(rc){security_task_free(task);}rc}
#[no_mangle] pub unsafe extern "C" fn security_task_free(task:*mut task_struct){call_void_hook!(task_free,task);kfree((*task).security);(*task).security=core::ptr::null_mut();}
#[no_mangle] pub unsafe extern "C" fn security_cred_alloc_blank(cred:*mut cred,gfp:gfp_t)->c_int{let mut rc=lsm_cred_alloc(cred,gfp);if rc!=0{return rc;}rc=call_int_hook!(cred_alloc_blank,cred,gfp);if unlikely(rc){security_cred_free(cred);}rc}
#[no_mangle] pub unsafe extern "C" fn security_cred_free(cred:*mut cred){if (*cred).security.is_null(){return;}call_void_hook!(cred_free,cred);kfree((*cred).security);(*cred).security=core::ptr::null_mut();}
#[no_mangle] pub unsafe extern "C" fn security_prepare_creds(new:*mut cred,old:*const cred,gfp:gfp_t)->c_int{let mut rc=lsm_cred_alloc(new,gfp);if rc!=0{return rc;}rc=call_int_hook!(cred_prepare,new,old,gfp);if unlikely(rc){security_cred_free(new);}rc}
void_hook_fn!(fn security_transfer_creds(new:*mut cred,old:*const cred)=>cred_transfer);
#[no_mangle] pub unsafe extern "C" fn security_cred_getsecid(c:*const cred,secid:*mut u32){*secid=0;call_void_hook!(cred_getsecid,c,secid);}
#[no_mangle] pub unsafe extern "C" fn security_cred_getlsmprop(c:*const cred,prop:*mut lsm_prop){lsmprop_init(prop);call_void_hook!(cred_getlsmprop,c,prop);}
int_hook_fn!(fn security_kernel_act_as(new:*mut cred,secid:u32)=>kernel_act_as);
int_hook_fn!(fn security_kernel_create_files_as(new:*mut cred,inode:*mut inode)=>kernel_create_files_as);
int_hook_fn!(fn security_kernel_module_request(kmod_name:*mut c_char)=>kernel_module_request);
int_hook_fn!(fn security_kernel_read_file(file:*mut file,id:c_int,contents:bool)=>kernel_read_file);
int_hook_fn!(fn security_kernel_post_read_file(file:*mut file,buf:*mut c_char,size:loff_t,id:c_int)=>kernel_post_read_file);
int_hook_fn!(fn security_kernel_load_data(id:c_int,contents:bool)=>kernel_load_data);
int_hook_fn!(fn security_kernel_post_load_data(buf:*mut c_char,size:loff_t,id:c_int,description:*mut c_char)=>kernel_post_load_data);
int_hook_fn!(fn security_task_fix_setuid(new:*mut cred,old:*const cred,flags:c_int)=>task_fix_setuid);
int_hook_fn!(fn security_task_fix_setgid(new:*mut cred,old:*const cred,flags:c_int)=>task_fix_setgid);
int_hook_fn!(fn security_task_fix_setgroups(new:*mut cred,old:*const cred)=>task_fix_setgroups);
int_hook_fn!(fn security_task_setpgid(p:*mut task_struct,pgid:pid_t)=>task_setpgid);
int_hook_fn!(fn security_task_getpgid(p:*mut task_struct)=>task_getpgid);
int_hook_fn!(fn security_task_getsid(p:*mut task_struct)=>task_getsid);
#[no_mangle] pub unsafe extern "C" fn security_current_getlsmprop_subj(prop:*mut lsm_prop){lsmprop_init(prop);call_void_hook!(current_getlsmprop_subj,prop);}
#[no_mangle] pub unsafe extern "C" fn security_task_getlsmprop_obj(p:*mut task_struct,prop:*mut lsm_prop){lsmprop_init(prop);call_void_hook!(task_getlsmprop_obj,p,prop);}
int_hook_fn!(fn security_task_setnice(p:*mut task_struct,nice:c_int)=>task_setnice);
int_hook_fn!(fn security_task_setioprio(p:*mut task_struct,ioprio:c_int)=>task_setioprio);
int_hook_fn!(fn security_task_getioprio(p:*mut task_struct)=>task_getioprio);
int_hook_fn!(fn security_task_prlimit(cred:*const cred,tcred:*const cred,flags:c_uint)=>task_prlimit);
int_hook_fn!(fn security_task_setrlimit(p:*mut task_struct,resource:c_uint,new_rlim:*mut rlimit)=>task_setrlimit);
int_hook_fn!(fn security_task_setscheduler(p:*mut task_struct)=>task_setscheduler);
int_hook_fn!(fn security_task_getscheduler(p:*mut task_struct)=>task_getscheduler);
int_hook_fn!(fn security_task_movememory(p:*mut task_struct)=>task_movememory);
int_hook_fn!(fn security_task_kill(p:*mut task_struct,info:*mut kernel_siginfo,sig:c_int,cred:*const cred)=>task_kill);
int_hook_fn!(fn security_task_prctl(option:c_int,arg2:c_ulong,arg3:c_ulong,arg4:c_ulong,arg5:c_ulong)=>task_prctl);
void_hook_fn!(fn security_task_to_inode(p:*mut task_struct,inode:*mut inode)=>task_to_inode);
int_hook_fn!(fn security_create_user_ns(cred:*const cred)=>userns_create);

int_hook_fn!(fn security_ipc_permission(ipcp:*mut kern_ipc_perm,flag:i16)=>ipc_permission);
#[no_mangle] pub unsafe extern "C" fn security_ipc_getlsmprop(ipcp:*mut kern_ipc_perm,prop:*mut lsm_prop){lsmprop_init(prop);call_void_hook!(ipc_getlsmprop,ipcp,prop);}
#[no_mangle] pub unsafe extern "C" fn security_msg_msg_alloc(msg:*mut msg_msg)->c_int{let mut rc=lsm_msg_msg_alloc(msg);if unlikely(rc){return rc;}rc=call_int_hook!(msg_msg_alloc_security,msg);if unlikely(rc){security_msg_msg_free(msg);}rc}
#[no_mangle] pub unsafe extern "C" fn security_msg_msg_free(msg:*mut msg_msg){call_void_hook!(msg_msg_free_security,msg);kfree((*msg).security);(*msg).security=core::ptr::null_mut();}
#[no_mangle] pub unsafe extern "C" fn security_msg_queue_alloc(msq:*mut kern_ipc_perm)->c_int{let mut rc=lsm_ipc_alloc(msq);if unlikely(rc){return rc;}rc=call_int_hook!(msg_queue_alloc_security,msq);if unlikely(rc){security_msg_queue_free(msq);}rc}
#[no_mangle] pub unsafe extern "C" fn security_msg_queue_free(msq:*mut kern_ipc_perm){call_void_hook!(msg_queue_free_security,msq);kfree((*msq).security);(*msq).security=core::ptr::null_mut();}
int_hook_fn!(fn security_msg_queue_associate(msq:*mut kern_ipc_perm,msqflg:c_int)=>msg_queue_associate);
int_hook_fn!(fn security_msg_queue_msgctl(msq:*mut kern_ipc_perm,cmd:c_int)=>msg_queue_msgctl);
int_hook_fn!(fn security_msg_queue_msgsnd(msq:*mut kern_ipc_perm,msg:*mut msg_msg,msqflg:c_int)=>msg_queue_msgsnd);
int_hook_fn!(fn security_msg_queue_msgrcv(msq:*mut kern_ipc_perm,msg:*mut msg_msg,target:*mut task_struct,type_:isize,mode:c_int)=>msg_queue_msgrcv);
#[no_mangle] pub unsafe extern "C" fn security_shm_alloc(shp:*mut kern_ipc_perm)->c_int{let mut rc=lsm_ipc_alloc(shp);if unlikely(rc){return rc;}rc=call_int_hook!(shm_alloc_security,shp);if unlikely(rc){security_shm_free(shp);}rc}
#[no_mangle] pub unsafe extern "C" fn security_shm_free(shp:*mut kern_ipc_perm){call_void_hook!(shm_free_security,shp);kfree((*shp).security);(*shp).security=core::ptr::null_mut();}
int_hook_fn!(fn security_shm_associate(shp:*mut kern_ipc_perm,shmflg:c_int)=>shm_associate);
int_hook_fn!(fn security_shm_shmctl(shp:*mut kern_ipc_perm,cmd:c_int)=>shm_shmctl);
int_hook_fn!(fn security_shm_shmat(shp:*mut kern_ipc_perm,shmaddr:*mut c_char,shmflg:c_int)=>shm_shmat);
#[no_mangle] pub unsafe extern "C" fn security_sem_alloc(sma:*mut kern_ipc_perm)->c_int{let mut rc=lsm_ipc_alloc(sma);if unlikely(rc){return rc;}rc=call_int_hook!(sem_alloc_security,sma);if unlikely(rc){security_sem_free(sma);}rc}
#[no_mangle] pub unsafe extern "C" fn security_sem_free(sma:*mut kern_ipc_perm){call_void_hook!(sem_free_security,sma);kfree((*sma).security);(*sma).security=core::ptr::null_mut();}
int_hook_fn!(fn security_sem_associate(sma:*mut kern_ipc_perm,semflg:c_int)=>sem_associate);
int_hook_fn!(fn security_sem_semctl(sma:*mut kern_ipc_perm,cmd:c_int)=>sem_semctl);
int_hook_fn!(fn security_sem_semop(sma:*mut kern_ipc_perm,sops:*mut sembuf,nsops:c_uint,alter:c_int)=>sem_semop);
#[no_mangle] pub unsafe extern "C" fn security_d_instantiate(dentry:*mut dentry,inode:*mut inode){if !inode.is_null()&&IS_PRIVATE(inode){return;}call_void_hook!(d_instantiate,dentry,inode);}

#[no_mangle] pub unsafe extern "C" fn security_getselfattr(attr:c_uint,uctx:*mut lsm_ctx,size:*mut u32,flags:u32)->c_int{let mut left:u32=0;if attr==LSM_ATTR_UNDEF{return -EINVAL;}if size.is_null(){return -EINVAL;}if get_user_u32(&mut left,size)!=0{return -EFAULT;}if flags!=0{if flags!=LSM_FLAG_SINGLE||uctx.is_null(){return -EINVAL;}let mut lctx=lsm_ctx{id:LSM_ID_UNDEF,flags:0,len:0,ctx_len:0,ctx:[]};if copy_from_user(&mut lctx as *mut _ as *mut c_void,uctx as *const c_void,core::mem::size_of::<lsm_ctx>())!=0{return -EFAULT;}if lctx.id==LSM_ID_UNDEF{return -EINVAL;}}if put_user_u32(0,size)!=0{return -EFAULT;}LSM_RET_DEFAULT!(getselfattr)}
#[no_mangle] pub unsafe extern "C" fn security_setselfattr(attr:c_uint,uctx:*mut lsm_ctx,size:u32,flags:u32)->c_int{if flags!=0{return -EINVAL;}if (size as usize)<core::mem::size_of::<lsm_ctx>(){return -EINVAL;}if size>PAGE_SIZE{return -E2BIG;}let lctx=memdup_user(uctx as *const c_void,size) as *mut lsm_ctx;if IS_ERR(lctx as *const c_void){return PTR_ERR(lctx as *const c_void);}let mut rc=LSM_RET_DEFAULT!(setselfattr);if (size as u64)<(*lctx).len||(*lctx).len<(core::mem::size_of::<lsm_ctx>() as u64).wrapping_add((*lctx).ctx_len){rc=-EINVAL;}else{rc=call_int_hook!(setselfattr,attr,lctx,size,flags);}kfree(lctx as *mut c_void);rc}
int_hook_fn!(fn security_getprocattr(p:*mut task_struct,lsmid:c_int,name:*const c_char,value:*mut *mut c_char)=>getprocattr);
int_hook_fn!(fn security_setprocattr(lsmid:c_int,name:*const c_char,value:*mut c_void,size:size_t)=>setprocattr);
int_hook_fn!(fn security_ismaclabel(name:*const c_char)=>ismaclabel);
int_hook_fn!(fn security_secid_to_secctx(secid:u32,cp:*mut lsm_context)=>secid_to_secctx);
int_hook_fn!(fn security_lsmprop_to_secctx(prop:*mut lsm_prop,cp:*mut lsm_context,lsmid:c_int)=>lsmprop_to_secctx);
#[no_mangle] pub unsafe extern "C" fn security_secctx_to_secid(secdata:*const c_char,seclen:u32,secid:*mut u32)->c_int{*secid=0;call_int_hook!(secctx_to_secid,secdata,seclen,secid)}
#[no_mangle] pub unsafe extern "C" fn security_release_secctx(cp:*mut lsm_context){call_void_hook!(release_secctx,cp);memset(cp as *mut c_void,0,core::mem::size_of::<lsm_context>());}
void_hook_fn!(fn security_inode_invalidate_secctx(inode:*mut inode)=>inode_invalidate_secctx);
int_hook_fn!(fn security_inode_notifysecctx(inode:*mut inode,ctx:*mut c_void,ctxlen:u32)=>inode_notifysecctx);
int_hook_fn!(fn security_inode_setsecctx(dentry:*mut dentry,ctx:*mut c_void,ctxlen:u32)=>inode_setsecctx);
#[no_mangle] pub unsafe extern "C" fn security_inode_getsecctx(inode:*mut inode,cp:*mut lsm_context)->c_int{memset(cp as *mut c_void,0,core::mem::size_of::<lsm_context>());call_int_hook!(inode_getsecctx,inode,cp)}
int_hook_fn!(fn security_post_notification(w_cred:*const cred,cred:*const cred,n:*mut watch_notification)=>post_notification);
int_hook_fn!(fn security_watch_key(key:*mut key)=>watch_key);

int_hook_fn!(fn security_netlink_send(sk:*mut sock,skb:*mut sk_buff)=>netlink_send);
int_hook_fn!(fn security_unix_stream_connect(sock:*mut sock,other:*mut sock,newsk:*mut sock)=>unix_stream_connect);
int_hook_fn!(fn security_unix_may_send(sock:*mut socket,other:*mut socket)=>unix_may_send);
int_hook_fn!(fn security_socket_create(family:c_int,type_:c_int,protocol:c_int,kern:c_int)=>socket_create);
int_hook_fn!(fn security_socket_post_create(sock:*mut socket,family:c_int,type_:c_int,protocol:c_int,kern:c_int)=>socket_post_create);
int_hook_fn!(fn security_socket_socketpair(socka:*mut socket,sockb:*mut socket)=>socket_socketpair);
int_hook_fn!(fn security_socket_bind(sock:*mut socket,address:*mut sockaddr,addrlen:c_int)=>socket_bind);
int_hook_fn!(fn security_socket_connect(sock:*mut socket,address:*mut sockaddr,addrlen:c_int)=>socket_connect);
int_hook_fn!(fn security_socket_listen(sock:*mut socket,backlog:c_int)=>socket_listen);
int_hook_fn!(fn security_socket_accept(sock:*mut socket,newsock:*mut socket)=>socket_accept);
int_hook_fn!(fn security_socket_sendmsg(sock:*mut socket,msg:*mut c_void,size:c_int)=>socket_sendmsg);
int_hook_fn!(fn security_socket_recvmsg(sock:*mut socket,msg:*mut c_void,size:c_int,flags:c_int)=>socket_recvmsg);
int_hook_fn!(fn security_socket_getsockname(sock:*mut socket)=>socket_getsockname);
int_hook_fn!(fn security_socket_getpeername(sock:*mut socket)=>socket_getpeername);
int_hook_fn!(fn security_socket_getsockopt(sock:*mut socket,level:c_int,optname:c_int)=>socket_getsockopt);
int_hook_fn!(fn security_socket_setsockopt(sock:*mut socket,level:c_int,optname:c_int)=>socket_setsockopt);
int_hook_fn!(fn security_socket_shutdown(sock:*mut socket,how:c_int)=>socket_shutdown);
int_hook_fn!(fn security_sock_rcv_skb(sk:*mut sock,skb:*mut sk_buff)=>socket_sock_rcv_skb);
int_hook_fn!(fn security_socket_getpeersec_stream(sock:*mut socket,optval:sockptr_t,optlen:sockptr_t,len:c_uint)=>socket_getpeersec_stream);
int_hook_fn!(fn security_socket_getpeersec_dgram(sock:*mut socket,skb:*mut sk_buff,secid:*mut u32)=>socket_getpeersec_dgram);
#[no_mangle] pub unsafe extern "C" fn security_sk_alloc(sk:*mut sock,family:c_int,priority:gfp_t)->c_int{let mut rc=lsm_sock_alloc(sk,priority);if unlikely(rc){return rc;}rc=call_int_hook!(sk_alloc_security,sk,family,priority);if unlikely(rc){security_sk_free(sk);}rc}
#[no_mangle] pub unsafe extern "C" fn security_sk_free(sk:*mut sock){call_void_hook!(sk_free_security,sk);kfree((*sk).sk_security);(*sk).sk_security=core::ptr::null_mut();}
void_hook_fn!(fn security_sk_clone(sk:*const sock,newsk:*mut sock)=>sk_clone_security);
#[no_mangle] pub unsafe extern "C" fn security_sk_classify_flow(sk:*const sock,flic:*mut flowi_common){call_void_hook!(sk_getsecid,sk,&mut (*flic).flowic_secid);}
void_hook_fn!(fn security_req_classify_flow(req:*const request_sock,flic:*mut flowi_common)=>req_classify_flow);
void_hook_fn!(fn security_sock_graft(sk:*mut sock,parent:*mut socket)=>sock_graft);
int_hook_fn!(fn security_inet_conn_request(sk:*const sock,skb:*mut sk_buff,req:*mut request_sock)=>inet_conn_request);
void_hook_fn!(fn security_inet_csk_clone(newsk:*mut sock,req:*const request_sock)=>inet_csk_clone);
void_hook_fn!(fn security_inet_conn_established(sk:*mut sock,skb:*mut sk_buff)=>inet_conn_established);
int_hook_fn!(fn security_secmark_relabel_packet(secid:u32)=>secmark_relabel_packet);
void_hook_fn!(fn security_secmark_refcount_inc()=>secmark_refcount_inc);
void_hook_fn!(fn security_secmark_refcount_dec()=>secmark_refcount_dec);
#[no_mangle] pub unsafe extern "C" fn security_tun_dev_alloc_security(security:*mut *mut c_void)->c_int{let mut rc=lsm_blob_alloc(security,blob_sizes.lbs_tun_dev,GFP_KERNEL);if rc!=0{return rc;}rc=call_int_hook!(tun_dev_alloc_security,*security);if rc!=0{kfree(*security);*security=core::ptr::null_mut();}rc}
#[no_mangle] pub unsafe extern "C" fn security_tun_dev_free_security(security:*mut c_void){kfree(security);}
int_hook_fn!(fn security_tun_dev_create()=>tun_dev_create);
int_hook_fn!(fn security_tun_dev_attach_queue(security:*mut c_void)=>tun_dev_attach_queue);
int_hook_fn!(fn security_tun_dev_attach(sk:*mut sock,security:*mut c_void)=>tun_dev_attach);
int_hook_fn!(fn security_tun_dev_open(security:*mut c_void)=>tun_dev_open);
int_hook_fn!(fn security_sctp_assoc_request(asoc:*mut sctp_association,skb:*mut sk_buff)=>sctp_assoc_request);
int_hook_fn!(fn security_sctp_bind_connect(sk:*mut sock,optname:c_int,address:*mut sockaddr,addrlen:c_int)=>sctp_bind_connect);
void_hook_fn!(fn security_sctp_sk_clone(asoc:*mut sctp_association,sk:*mut sock,newsk:*mut sock)=>sctp_sk_clone);
int_hook_fn!(fn security_sctp_assoc_established(asoc:*mut sctp_association,skb:*mut sk_buff)=>sctp_assoc_established);
int_hook_fn!(fn security_mptcp_add_subflow(sk:*mut sock,ssk:*mut sock)=>mptcp_add_subflow);
int_hook_fn!(fn security_unix_find(path:*const path,other:*mut sock,flags:c_int)=>unix_find);

int_hook_fn!(fn security_ib_pkey_access(sec:*mut c_void,subnet_prefix:u64,pkey:u16)=>ib_pkey_access);
int_hook_fn!(fn security_ib_endport_manage_subnet(sec:*mut c_void,dev_name:*const c_char,port_num:u8)=>ib_endport_manage_subnet);
#[no_mangle] pub unsafe extern "C" fn security_ib_alloc_security(sec:*mut *mut c_void)->c_int{let mut rc=lsm_blob_alloc(sec,blob_sizes.lbs_ib,GFP_KERNEL);if rc!=0{return rc;}rc=call_int_hook!(ib_alloc_security,*sec);if rc!=0{kfree(*sec);*sec=core::ptr::null_mut();}rc}
#[no_mangle] pub unsafe extern "C" fn security_ib_free_security(sec:*mut c_void){kfree(sec);}
int_hook_fn!(fn security_xfrm_policy_alloc(ctxp:*mut *mut xfrm_sec_ctx,sec_ctx:*mut xfrm_user_sec_ctx,gfp:gfp_t)=>xfrm_policy_alloc_security);
int_hook_fn!(fn security_xfrm_policy_clone(old_ctx:*mut xfrm_sec_ctx,new_ctxp:*mut *mut xfrm_sec_ctx)=>xfrm_policy_clone_security);
void_hook_fn!(fn security_xfrm_policy_free(ctx:*mut xfrm_sec_ctx)=>xfrm_policy_free_security);
int_hook_fn!(fn security_xfrm_policy_delete(ctx:*mut xfrm_sec_ctx)=>xfrm_policy_delete_security);
int_hook_fn!(fn security_xfrm_state_alloc(x:*mut xfrm_state,sec_ctx:*mut xfrm_user_sec_ctx)=>xfrm_state_alloc);
int_hook_fn!(fn security_xfrm_state_alloc_acquire(x:*mut xfrm_state,polsec:*mut xfrm_sec_ctx,secid:u32)=>xfrm_state_alloc_acquire);
int_hook_fn!(fn security_xfrm_state_delete(x:*mut xfrm_state)=>xfrm_state_delete_security);
void_hook_fn!(fn security_xfrm_state_free(x:*mut xfrm_state)=>xfrm_state_free_security);
int_hook_fn!(fn security_xfrm_policy_lookup(ctx:*mut xfrm_sec_ctx,fl_secid:u32)=>xfrm_policy_lookup);
int_hook_fn!(fn security_xfrm_state_pol_flow_match(x:*mut xfrm_state,xp:*mut xfrm_policy,flic:*const flowi_common)=>xfrm_state_pol_flow_match);
int_hook_fn!(fn security_xfrm_decode_session(skb:*mut sk_buff,secid:*mut u32)=>xfrm_decode_session);
#[no_mangle] pub unsafe extern "C" fn security_skb_classify_flow(skb:*mut sk_buff,flic:*mut flowi_common){let rc=call_int_hook!(xfrm_decode_session,skb,&mut (*flic).flowic_secid,0);BUG_ON(rc);}

#[no_mangle] pub unsafe extern "C" fn security_key_alloc(key:*mut key,cred:*const cred,flags:c_ulong)->c_int{let mut rc=lsm_key_alloc(key);if unlikely(rc){return rc;}rc=call_int_hook!(key_alloc,key,cred,flags);if unlikely(rc){security_key_free(key);}rc}
#[no_mangle] pub unsafe extern "C" fn security_key_free(key:*mut key){kfree((*key).security);(*key).security=core::ptr::null_mut();}
int_hook_fn!(fn security_key_permission(key_ref:key_ref_t,cred:*const cred,need_perm:c_int)=>key_permission);
#[no_mangle] pub unsafe extern "C" fn security_key_getsecurity(key:*mut key,buffer:*mut *mut c_char)->c_int{*buffer=core::ptr::null_mut();call_int_hook!(key_getsecurity,key,buffer)}
void_hook_fn!(fn security_key_post_create_or_update(keyring:*mut key,key:*mut key,payload:*const c_void,payload_len:size_t,flags:c_ulong,create:bool)=>key_post_create_or_update);

int_hook_fn!(fn security_audit_rule_init(field:u32,op:u32,rulestr:*mut c_char,lsmrule:*mut *mut c_void,gfp:gfp_t)=>audit_rule_init);
int_hook_fn!(fn security_audit_rule_known(krule:*mut audit_krule)=>audit_rule_known);
void_hook_fn!(fn security_audit_rule_free(lsmrule:*mut c_void)=>audit_rule_free);
int_hook_fn!(fn security_audit_rule_match(prop:*mut lsm_prop,field:u32,op:u32,lsmrule:*mut c_void)=>audit_rule_match);

int_hook_fn!(fn security_bpf(cmd:c_int,attr:*mut bpf_attr,size:c_uint,kernel:bool)=>bpf);
int_hook_fn!(fn security_bpf_map(map:*mut bpf_map,fmode:fmode_t)=>bpf_map);
int_hook_fn!(fn security_bpf_prog(prog:*mut bpf_prog)=>bpf_prog);
#[no_mangle] pub unsafe extern "C" fn security_bpf_map_create(map:*mut bpf_map,attr:*mut bpf_attr,token:*mut bpf_token,kernel:bool)->c_int{let mut rc=lsm_bpf_map_alloc(map);if unlikely(rc){return rc;}rc=call_int_hook!(bpf_map_create,map,attr,token,kernel);if unlikely(rc){security_bpf_map_free(map);}rc}
#[no_mangle] pub unsafe extern "C" fn security_bpf_prog_load(prog:*mut bpf_prog,attr:*mut bpf_attr,token:*mut bpf_token,kernel:bool)->c_int{let mut rc=lsm_bpf_prog_alloc(prog);if unlikely(rc){return rc;}rc=call_int_hook!(bpf_prog_load,prog,attr,token,kernel);if unlikely(rc){security_bpf_prog_free(prog);}rc}
#[no_mangle] pub unsafe extern "C" fn security_bpf_token_create(token:*mut bpf_token,attr:*mut bpf_attr,path:*const path)->c_int{let mut rc=lsm_bpf_token_alloc(token);if unlikely(rc){return rc;}rc=call_int_hook!(bpf_token_create,token,attr,path);if unlikely(rc){security_bpf_token_free(token);}rc}
int_hook_fn!(fn security_bpf_token_cmd(token:*const bpf_token,cmd:c_int)=>bpf_token_cmd);
int_hook_fn!(fn security_bpf_token_capable(token:*const bpf_token,cap:c_int)=>bpf_token_capable);
#[no_mangle] pub unsafe extern "C" fn security_bpf_map_free(map:*mut bpf_map){call_void_hook!(bpf_map_free,map);kfree((*map).security);(*map).security=core::ptr::null_mut();}
#[no_mangle] pub unsafe extern "C" fn security_bpf_prog_free(prog:*mut bpf_prog){call_void_hook!(bpf_prog_free,prog);kfree((*(*prog).aux).security);(*(*prog).aux).security=core::ptr::null_mut();}
#[no_mangle] pub unsafe extern "C" fn security_bpf_token_free(token:*mut bpf_token){call_void_hook!(bpf_token_free,token);kfree((*token).security);(*token).security=core::ptr::null_mut();}

int_hook_fn!(fn security_locked_down(what:c_int)=>locked_down);
#[no_mangle] pub unsafe extern "C" fn security_bdev_alloc(bdev:*mut block_device)->c_int{let mut rc=lsm_bdev_alloc(bdev);if unlikely(rc){return rc;}rc=call_int_hook!(bdev_alloc_security,bdev);if unlikely(rc){security_bdev_free(bdev);}rc}
#[no_mangle] pub unsafe extern "C" fn security_bdev_free(bdev:*mut block_device){if (*bdev).bd_security.is_null(){return;}call_void_hook!(bdev_free_security,bdev);kfree((*bdev).bd_security);(*bdev).bd_security=core::ptr::null_mut();}
int_hook_fn!(fn security_bdev_setintegrity(bdev:*mut block_device,type_:c_int,value:*const c_void,size:size_t)=>bdev_setintegrity);

int_hook_fn!(fn security_perf_event_open(type_:c_int)=>perf_event_open);
#[no_mangle] pub unsafe extern "C" fn security_perf_event_alloc(event:*mut perf_event)->c_int{let mut rc=lsm_blob_alloc(&mut (*event).security,blob_sizes.lbs_perf_event,GFP_KERNEL);if rc!=0{return rc;}rc=call_int_hook!(perf_event_alloc,event);if rc!=0{kfree((*event).security);(*event).security=core::ptr::null_mut();}rc}
#[no_mangle] pub unsafe extern "C" fn security_perf_event_free(event:*mut perf_event){kfree((*event).security);(*event).security=core::ptr::null_mut();}
int_hook_fn!(fn security_perf_event_read(event:*mut perf_event)=>perf_event_read);
int_hook_fn!(fn security_perf_event_write(event:*mut perf_event)=>perf_event_write);

int_hook_fn!(fn security_uring_override_creds(new:*const cred)=>uring_override_creds);
int_hook_fn!(fn security_uring_sqpoll()=>uring_sqpoll);
int_hook_fn!(fn security_uring_cmd(ioucmd:*mut io_uring_cmd)=>uring_cmd);
int_hook_fn!(fn security_uring_allowed()=>uring_allowed);
void_hook_fn!(fn security_initramfs_populated()=>initramfs_populated);

/* EXPORT_SYMBOL/EXPORT_SYMBOL_GPL markers from C have no standalone Rust item here. */
/* CONFIG_* preprocessor guards from C are retained as comments; in a full kernel Rust build these items would be gated with matching cfgs. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
