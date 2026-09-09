// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/ipc/util.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

#[repr(C)]
pub struct ipc_proc_iface {
    pub path: *const core::ffi::c_char,
    pub header: *const core::ffi::c_char,
    pub ids: i32,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}

extern "C" {
    fn proc_mkdir(name: *const core::ffi::c_char, parent: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn sem_init(); fn msg_init(); fn shm_init();
    fn init_rwsem(x: *mut rw_semaphore); fn rhashtable_init(x: *mut rhashtable, p: *const rhashtable_params) -> i32;
    fn idr_init(x: *mut idr);
    fn rhashtable_lookup_fast(ht: *mut rhashtable, key: *const key_t, p: rhashtable_params) -> *mut kern_ipc_perm;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn ipc_lock_object(x: *mut kern_ipc_perm); fn ipc_unlock(x: *mut kern_ipc_perm);
    fn idr_alloc_cyclic(x: *mut idr, ptr: *mut core::ffi::c_void, s: i32, e: i32, g: u32) -> i32;
    fn idr_alloc(x: *mut idr, ptr: *mut kern_ipc_perm, s: i32, e: i32, g: u32) -> i32;
    fn idr_replace(x: *mut idr, ptr: *mut kern_ipc_perm, id: i32); fn idr_remove(x: *mut idr, id: i32) -> *mut kern_ipc_perm;
    fn ipcid_seq_max() -> i32; fn ipcid_to_seqx(id: i32) -> i32; fn ipcid_to_idx(id: i32) -> i32; fn ipcmni_seq_shift() -> i32;
    fn refcount_set(x: *mut refcount_t, v: i32); fn spin_lock_init(x: *mut spinlock_t); fn spin_lock(x: *mut spinlock_t); fn spin_unlock(x: *mut spinlock_t);
    fn current_euid_egid(u: *mut kuid_t, g: *mut kgid_t); fn idr_preload(g: u32); fn idr_preload_end();
    fn rhashtable_insert_fast(ht: *mut rhashtable, n: *mut rhash_head, p: rhashtable_params) -> i32;
    fn rhashtable_remove_fast(ht: *mut rhashtable, n: *mut rhash_head, p: rhashtable_params) -> i32;
    fn down_write(x: *mut rw_semaphore); fn up_write(x: *mut rw_semaphore); fn down_read(x: *mut rw_semaphore); fn up_read(x: *mut rw_semaphore);
    fn ipcperms(ns: *mut ipc_namespace, p: *mut kern_ipc_perm, f: i16) -> i32;
    fn audit_ipc_obj(p: *mut kern_ipc_perm); fn audit_ipc_set_perm(e: i32, u: u32, g: u32, m: i32);
    fn security_ipc_permission(p: *mut kern_ipc_perm, f: i16) -> i32; fn current_euid() -> kuid_t;
    fn uid_eq(a: kuid_t,b: kuid_t)->bool; fn in_group_p(g: kgid_t)->bool; fn ns_capable(u:*mut user_namespace,c:i32)->bool;
    fn current_user_ns()->*mut user_namespace; fn from_kuid_munged(u:*mut user_namespace,k:kuid_t)->u32; fn from_kgid_munged(u:*mut user_namespace,k:kgid_t)->u32;
    fn make_kuid(u:*mut user_namespace,v:u32)->kuid_t; fn make_kgid(u:*mut user_namespace,v:u32)->kgid_t; fn uid_valid(k:kuid_t)->bool; fn gid_valid(k:kgid_t)->bool;
    fn refcount_inc_not_zero(x:*mut refcount_t)->bool; fn refcount_dec_and_test(x:*mut refcount_t)->bool; fn call_rcu(h:*mut rcu_head,f:unsafe extern "C" fn(*mut rcu_head));
    fn idr_find(x:*mut idr,id:i32)->*mut kern_ipc_perm; fn ipc_checkid(p:*mut kern_ipc_perm,id:i32)->bool;
    fn PTR_ERR(p:*mut kern_ipc_perm)->i32; fn IS_ERR(p:*mut kern_ipc_perm)->bool;
}

pub static mut ipc_kht_params: rhashtable_params = rhashtable_params {
    head_offset: 0, key_offset: 0, key_len: 0, automatic_shrinking: true,
};

// Kernel structures and constants are supplied by the translated headers.
pub unsafe fn ipc_init() -> i32 { proc_mkdir(b"sysvipc\0".as_ptr() as _, core::ptr::null_mut()); sem_init(); msg_init(); shm_init(); 0 }

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn ipc_init_proc_interface(path:*const core::ffi::c_char,header:*const core::ffi::c_char,ids:i32,show:Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void)->i32>){
    let iface=kmalloc_obj::<ipc_proc_iface>(); if iface.is_null(){return;} (*iface).path=path;(*iface).header=header;(*iface).ids=ids;(*iface).show=show;
    let pde=proc_create_data(path,S_IRUGO,core::ptr::null_mut(),core::ptr::null(),iface as *mut _);if pde.is_null(){kfree(iface as *mut core::ffi::c_void);}
}

pub unsafe fn ipc_init_ids(ids: *mut ipc_ids) {
    (*ids).in_use=0; (*ids).seq=0; init_rwsem(&mut (*ids).rwsem); rhashtable_init(&mut (*ids).key_ht, &ipc_kht_params); idr_init(&mut (*ids).ipcs_idr); (*ids).max_idx=-1; (*ids).last_idx=-1;
    #[cfg(CONFIG_CHECKPOINT_RESTORE)] { (*ids).next_id=-1; }
}

pub unsafe fn ipc_addid(ids:*mut ipc_ids,new:*mut kern_ipc_perm,limit:i32)->i32 {
    let mut e=kuid_t::default(); let mut g=kgid_t::default(); refcount_set(&mut (*new).refcount,1);
    let limit=if limit>ipc_mni {ipc_mni} else {limit}; if (*ids).in_use>=limit{return -ENOSPC;}
    idr_preload(GFP_KERNEL); spin_lock_init(&mut (*new).lock); rcu_read_lock(); spin_lock(&mut (*new).lock); current_euid_egid(&mut e,&mut g); (*new).cuid=e;(*new).uid=e;(*new).gid=g;(*new).cgid=g;(*new).deleted=false;
    let mut idx=ipc_idr_alloc(ids,new); idr_preload_end(); if idx>=0 && (*new).key!=IPC_PRIVATE { let err=rhashtable_insert_fast(&mut (*ids).key_ht,&mut (*new).khtnode,ipc_kht_params); if err<0 {idr_remove(&mut (*ids).ipcs_idr,idx);idx=err;} }
    if idx<0 {(*new).deleted=true;spin_unlock(&mut (*new).lock);rcu_read_unlock();return idx;} (*ids).in_use+=1;if idx>(*ids).max_idx{(*ids).max_idx=idx;}idx
}

unsafe fn ipc_idr_alloc(ids:*mut ipc_ids,new:*mut kern_ipc_perm)->i32 { let mut idx; let mut next_id=-1;
    #[cfg(CONFIG_CHECKPOINT_RESTORE)] {next_id=(*ids).next_id;(*ids).next_id=-1;}
    if next_id<0 {let mut max_idx=((*ids).in_use*3/2).max(ipc_min_cycle);max_idx=max_idx.min(ipc_mni);idx=idr_alloc_cyclic(&mut (*ids).ipcs_idr,core::ptr::null_mut(),0,max_idx,GFP_NOWAIT);if idx>=0{if idx<=(*ids).last_idx{(*ids).seq+=1;if (*ids).seq>=ipcid_seq_max(){(*ids).seq=0;}}(*ids).last_idx=idx;(*new).seq=(*ids).seq;idr_replace(&mut (*ids).ipcs_idr,new,idx);}} else {(*new).seq=ipcid_to_seqx(next_id);idx=idr_alloc(&mut (*ids).ipcs_idr,new,ipcid_to_idx(next_id),ipc_mni,GFP_NOWAIT);}if idx>=0{(*new).id=((*new).seq<<ipcmni_seq_shift())+idx;}idx }

unsafe fn ipc_findkey(ids:*mut ipc_ids,key:key_t)->*mut kern_ipc_perm {let p=rhashtable_lookup_fast(&mut (*ids).key_ht,&key,ipc_kht_params);if p.is_null(){return core::ptr::null_mut();}rcu_read_lock();ipc_lock_object(p);p}
unsafe fn ipcget_new(ns:*mut ipc_namespace,ids:*mut ipc_ids,ops:*const ipc_ops,params:*mut ipc_params)->i32{down_write(&mut (*ids).rwsem);let e=((*ops).getnew.unwrap())(ns,params);up_write(&mut (*ids).rwsem);e}
unsafe fn ipc_check_perms(ns:*mut ipc_namespace,p:*mut kern_ipc_perm,ops:*const ipc_ops,pa:*mut ipc_params)->i32{if ipcperms(ns,p,(*pa).flg)!=0{-EACCES}else{let mut e=((*ops).associate.unwrap())(p,(*pa).flg);if e==0{e=(*p).id;}e}}
unsafe fn ipcget_public(ns:*mut ipc_namespace,ids:*mut ipc_ids,ops:*const ipc_ops,pa:*mut ipc_params)->i32{let flg=(*pa).flg;down_write(&mut (*ids).rwsem);let p=ipc_findkey(ids,(*pa).key);let e=if p.is_null(){if flg&IPC_CREAT==0{-ENOENT}else{((*ops).getnew.unwrap())(ns,pa)}}else{let mut e=if flg&IPC_CREAT!=0&&flg&IPC_EXCL!=0{-EEXIST}else{0};if e==0{if let Some(f)=(*ops).more_checks{e=f(p,pa);}if e==0{e=ipc_check_perms(ns,p,ops,pa);}}ipc_unlock(p);e};up_write(&mut (*ids).rwsem);e}
pub unsafe fn ipcget(ns:*mut ipc_namespace,ids:*mut ipc_ids,ops:*const ipc_ops,p:*mut ipc_params)->i32{if (*p).key==IPC_PRIVATE{ipcget_new(ns,ids,ops,p)}else{ipcget_public(ns,ids,ops,p)}}

unsafe fn ipc_kht_remove(ids:*mut ipc_ids,p:*mut kern_ipc_perm){if (*p).key!=IPC_PRIVATE{rhashtable_remove_fast(&mut (*ids).key_ht,&mut (*p).khtnode,ipc_kht_params);}}
unsafe fn ipc_search_maxidx(ids:*mut ipc_ids,limit:i32)->i32{let mut r=0;let mut i=ilog2(limit+1);while i>=0{let mut t=r|(1<<i);t-=1;if !idr_find(&mut (*ids).ipcs_idr,t).is_null(){r|=1<<i;}i-=1;}r-1}
pub unsafe fn ipc_rmid(ids:*mut ipc_ids,p:*mut kern_ipc_perm){let mut idx=ipcid_to_idx((*p).id);idr_remove(&mut (*ids).ipcs_idr,idx);ipc_kht_remove(ids,p);(*ids).in_use-=1;(*p).deleted=true;if idx==(*ids).max_idx{idx-=1;(*ids).max_idx=if idx>=0{ipc_search_maxidx(ids,idx)}else{idx};}}
pub unsafe fn ipc_set_key_private(ids:*mut ipc_ids,p:*mut kern_ipc_perm){ipc_kht_remove(ids,p);(*p).key=IPC_PRIVATE;}
pub unsafe fn ipc_rcu_getref(p:*mut kern_ipc_perm)->bool{refcount_inc_not_zero(&mut (*p).refcount)}
pub unsafe fn ipc_rcu_putref(p:*mut kern_ipc_perm,f:unsafe extern "C" fn(*mut rcu_head)){if refcount_dec_and_test(&mut (*p).refcount){call_rcu(&mut (*p).rcu,f);}}

pub unsafe fn ipcperms(ns:*mut ipc_namespace,p:*mut kern_ipc_perm,flag:i16)->i32{audit_ipc_obj(p);let requested=((flag>>6)|(flag>>3)|flag) as i32;let mut granted=(*p).mode;if uid_eq(current_euid(),(*p).cuid)||uid_eq(current_euid(),(*p).uid){granted>>=6}else if in_group_p((*p).cgid)||in_group_p((*p).gid){granted>>=3}if requested&!granted&0o7!=0&&!ns_capable((*ns).user_ns,CAP_IPC_OWNER){return -1;}security_ipc_permission(p,flag)}

pub unsafe fn kernel_to_ipc64_perm(input:*mut kern_ipc_perm,out:*mut ipc64_perm){(*out).key=(*input).key;(*out).uid=from_kuid_munged(current_user_ns(),(*input).uid);(*out).gid=from_kgid_munged(current_user_ns(),(*input).gid);(*out).cuid=from_kuid_munged(current_user_ns(),(*input).cuid);(*out).cgid=from_kgid_munged(current_user_ns(),(*input).cgid);(*out).mode=(*input).mode;(*out).seq=(*input).seq;}
pub unsafe fn ipc64_perm_to_ipc_perm(input:*mut ipc64_perm,out:*mut ipc_perm){(*out).key=(*input).key;(*out).uid=(*input).uid;(*out).gid=(*input).gid;(*out).cuid=(*input).cuid;(*out).cgid=(*input).cgid;(*out).mode=(*input).mode;(*out).seq=(*input).seq;}
pub unsafe fn ipc_update_perm(input:*mut ipc64_perm,out:*mut kern_ipc_perm)->i32{let u=make_kuid(current_user_ns(),(*input).uid);let g=make_kgid(current_user_ns(),(*input).gid);if !uid_valid(u)||!gid_valid(g){return -EINVAL;}(*out).uid=u;(*out).gid=g;(*out).mode=((*out).mode&!S_IRWXUGO)|((*input).mode&S_IRWXUGO);0}

#[cfg(CONFIG_ARCH_WANT_IPC_PARSE_VERSION)]
pub unsafe fn ipc_parse_version(cmd:*mut i32)->i32{if *cmd&IPC_64!=0{*cmd^=IPC_64;IPC_64}else{IPC_OLD}}

#[cfg(CONFIG_PROC_FS)]
#[repr(C)] pub struct ipc_proc_iter{pub ns:*mut ipc_namespace,pub pid_ns:*mut pid_namespace,pub iface:*mut ipc_proc_iface}
#[cfg(CONFIG_PROC_FS)] pub unsafe fn ipc_seq_pid_ns(s:*mut seq_file)->*mut pid_namespace{(*( (*s).private as *mut ipc_proc_iter)).pid_ns}
#[cfg(CONFIG_PROC_FS)] unsafe fn sysvipc_find_ipc(ids:*mut ipc_ids,pos:*mut i64)->*mut kern_ipc_perm{let mut i=(*pos-1) as i32;let p=idr_find(&mut (*ids).ipcs_idr,i);if !p.is_null(){rcu_read_lock();ipc_lock_object(p);*pos=(i+1) as i64;}p}
#[cfg(CONFIG_PROC_FS)] unsafe fn sysvipc_proc_next(s:*mut seq_file,it:*mut core::ffi::c_void,pos:*mut i64)->*mut core::ffi::c_void{let iter=(*s).private as *mut ipc_proc_iter;if !it.is_null(){ipc_unlock(it as *mut kern_ipc_perm);}*pos+=1;sysvipc_find_ipc(&mut (*(*iter).ns).ids[(*(*iter).iface).ids as usize],pos) as *mut _}
#[cfg(CONFIG_PROC_FS)] unsafe fn sysvipc_proc_start(s:*mut seq_file,pos:*mut i64)->*mut core::ffi::c_void{let iter=(*s).private as *mut ipc_proc_iter;let ids=&mut (*(*iter).ns).ids[(*(*iter).iface).ids as usize];down_read(&mut ids.rwsem);if *pos<0{core::ptr::null_mut()}else if *pos==0{SEQ_START_TOKEN}else{sysvipc_find_ipc(ids,pos) as *mut _}}
#[cfg(CONFIG_PROC_FS)] unsafe fn sysvipc_proc_stop(s:*mut seq_file,it:*mut core::ffi::c_void){let iter=(*s).private as *mut ipc_proc_iter;if !it.is_null(){ipc_unlock(it as *mut kern_ipc_perm);}let ids=&mut (*(*iter).ns).ids[(*(*iter).iface).ids as usize];up_read(&mut ids.rwsem);}
#[cfg(CONFIG_PROC_FS)] unsafe fn sysvipc_proc_show(s:*mut seq_file,it:*mut core::ffi::c_void)->i32{let iter=(*s).private as *mut ipc_proc_iter;if it==SEQ_START_TOKEN{seq_puts(s,(*(*iter).iface).header);0}else{((*(*iter).iface).show.unwrap())(s,it)}}

pub unsafe fn ipc_obtain_object_idr(ids:*mut ipc_ids,id:i32)->*mut kern_ipc_perm{let p=idr_find(&mut (*ids).ipcs_idr,ipcid_to_idx(id));if p.is_null(){ERR_PTR(-EINVAL)}else{p}}
pub unsafe fn ipc_obtain_object_check(ids:*mut ipc_ids,id:i32)->*mut kern_ipc_perm{let p=ipc_obtain_object_idr(ids,id);if IS_ERR(p){p}else if ipc_checkid(p,id){ERR_PTR(-EINVAL)}else{p}}
pub unsafe fn ipcctl_obtain_check(ns:*mut ipc_namespace,ids:*mut ipc_ids,id:i32,cmd:i32,perm:*mut ipc64_perm,extra:i32)->*mut kern_ipc_perm{let p=ipc_obtain_object_check(ids,id);if IS_ERR(p){return p;}audit_ipc_obj(p);if cmd==IPC_SET{audit_ipc_set_perm(extra,(*perm).uid,(*perm).gid,(*perm).mode);}let e=current_euid();if uid_eq(e,(*p).cuid)||uid_eq(e,(*p).uid)||ns_capable((*ns).user_ns,CAP_SYS_ADMIN){p}else{ERR_PTR(-EPERM)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
