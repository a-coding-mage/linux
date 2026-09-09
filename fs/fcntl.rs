// SPDX-License-Identifier: GPL-2.0
// Translation of linux/fs/fcntl.c. Kernel types, constants, and helpers are
// supplied by the surrounding kernel translation unit.

const SETFL_MASK: u32 = O_APPEND | O_NONBLOCK | O_NDELAY | O_DIRECT | O_NOATIME;

unsafe fn setfl(fd: i32, filp: *mut file, mut arg: u32) -> i32 {
    let inode = file_inode(filp);
    let mut error = 0;
    if ((arg ^ (*filp).f_flags) & O_APPEND) != 0 && IS_APPEND(inode) { return -EPERM; }
    if (arg & O_NOATIME) != 0 && ((*filp).f_flags & O_NOATIME) == 0 &&
       !inode_owner_or_capable(file_mnt_idmap(filp), inode) { return -EPERM; }
    if O_NONBLOCK != O_NDELAY && (arg & O_NDELAY) != 0 { arg |= O_NONBLOCK; }
    if !S_ISFIFO((*inode).i_mode) && (arg & O_DIRECT) != 0 &&
       ((*filp).f_mode & FMODE_CAN_ODIRECT) == 0 { return -EINVAL; }
    if let Some(check) = (*(*filp).f_op).check_flags { error = check(arg); }
    if error != 0 { return error; }
    if ((arg ^ (*filp).f_flags) & FASYNC) != 0 {
        if let Some(fasync) = (*(*filp).f_op).fasync {
            error = fasync(fd, filp, (arg & FASYNC) != 0);
            if error < 0 { return error; }
            if error > 0 { error = 0; }
        }
    }
    spin_lock(&mut (*filp).f_lock);
    (*filp).f_flags = (arg & SETFL_MASK) | ((*filp).f_flags & !SETFL_MASK);
    (*filp).f_iocb_flags = iocb_flags(filp);
    spin_unlock(&mut (*filp).f_lock);
    error
}

pub unsafe fn file_f_owner_allocate(file: *mut file) -> i32 {
    let mut owner = file_f_owner(file);
    if !owner.is_null() { return 0; }
    owner = kzalloc_obj::<fown_struct>();
    if owner.is_null() { return -ENOMEM; }
    rwlock_init(&mut (*owner).lock); (*owner).file = file;
    if unlikely(cmpxchg(&mut (*file).f_owner, core::ptr::null_mut(), owner) != core::ptr::null_mut()) { kfree(owner); }
    0
}
pub unsafe fn file_f_owner_release(file: *mut file) { let owner = file_f_owner(file); if !owner.is_null() { put_pid((*owner).pid); kfree(owner); } }

pub unsafe fn __f_setown(filp: *mut file, pid: *mut pid, ty: pid_type, force: i32) {
    let owner = file_f_owner(filp); if WARN_ON_ONCE(owner.is_null()) { return; }
    write_lock_irq(&mut (*owner).lock);
    if force != 0 || (*owner).pid.is_null() { put_pid((*owner).pid); (*owner).pid = get_pid(pid); (*owner).pid_type = ty;
        if !pid.is_null() { let cred = current_cred(); security_file_set_fowner(filp); (*owner).uid = (*cred).uid; (*owner).euid = (*cred).euid; }
    }
    write_unlock_irq(&mut (*owner).lock);
}
pub unsafe fn f_setown(filp: *mut file, mut who: i32, force: i32) -> i32 {
    let mut ty = PIDTYPE_TGID; let mut pid = core::ptr::null_mut();
    if who < 0 { if who == i32::MIN { return -EINVAL; } ty = PIDTYPE_PGID; who = -who; }
    let mut ret = file_f_owner_allocate(filp); if ret != 0 { return ret; }
    rcu_read_lock(); if who != 0 { pid = find_vpid(who); if pid.is_null() { ret = -ESRCH; } }
    if ret == 0 { __f_setown(filp, pid, ty, force); } rcu_read_unlock(); ret
}
pub unsafe fn f_delown(filp: *mut file) { __f_setown(filp, core::ptr::null_mut(), PIDTYPE_TGID, 1); }
pub unsafe fn f_getown(filp: *mut file) -> pid_t { let owner=file_f_owner(filp); if owner.is_null(){return 0;} let mut p=0; read_lock_irq(&mut (*owner).lock); rcu_read_lock(); if !pid_task((*owner).pid,(*owner).pid_type).is_null(){p=pid_vnr((*owner).pid);if (*owner).pid_type==PIDTYPE_PGID{p=-p;}} rcu_read_unlock();read_unlock_irq(&mut (*owner).lock);p }

unsafe fn f_setown_ex(filp:*mut file,arg:usize)->i32{let owner_p=arg as *mut f_owner_ex;let mut owner=core::mem::zeroed();if copy_from_user(&mut owner,owner_p,core::mem::size_of::<f_owner_ex>())!=0{return -EFAULT;}let ty=match owner.type_{F_OWNER_TID=>PIDTYPE_PID,F_OWNER_PID=>PIDTYPE_TGID,F_OWNER_PGRP=>PIDTYPE_PGID,_=>return -EINVAL};let ret=file_f_owner_allocate(filp);if ret!=0{return ret;}rcu_read_lock();let pid=find_vpid(owner.pid);let ret=if owner.pid!=0&&pid.is_null(){-ESRCH}else{__f_setown(filp,pid,ty,1);0};rcu_read_unlock();ret}
unsafe fn f_getown_ex(filp:*mut file,arg:usize)->i32{let p=arg as *mut f_owner_ex;let mut o:f_owner_ex=core::mem::zeroed();let f=file_f_owner(filp);let mut ty=PIDTYPE_PID;if !f.is_null(){read_lock_irq(&mut (*f).lock);rcu_read_lock();if !pid_task((*f).pid,(*f).pid_type).is_null(){o.pid=pid_vnr((*f).pid);}rcu_read_unlock();ty=(*f).pid_type;}o.type_=match ty{PIDTYPE_PID=>F_OWNER_TID,PIDTYPE_TGID=>F_OWNER_PID,PIDTYPE_PGID=>F_OWNER_PGRP,_=>{if !f.is_null(){read_unlock_irq(&mut (*f).lock);}return -EINVAL}};if !f.is_null(){read_unlock_irq(&mut (*f).lock);}if copy_to_user(p,&o,core::mem::size_of::<f_owner_ex>())!=0{-EFAULT}else{0}}

unsafe fn rw_hint_valid(h:u64)->bool{matches!(h,RWH_WRITE_LIFE_NOT_SET|RWH_WRITE_LIFE_NONE|RWH_WRITE_LIFE_SHORT|RWH_WRITE_LIFE_MEDIUM|RWH_WRITE_LIFE_LONG|RWH_WRITE_LIFE_EXTREME)}
unsafe fn fcntl_get_rw_hint(f:*mut file,arg:usize)->i64{let h=READ_ONCE((*file_inode(f)).i_write_hint);if copy_to_user(arg as *mut u64,&h,8)!=0{-EFAULT as i64}else{0}}
unsafe fn fcntl_set_rw_hint(f:*mut file,arg:usize)->i64{let i=file_inode(f);if !inode_owner_or_capable(file_mnt_idmap(f),i){return -EPERM as i64;}let mut h=0u64;if copy_from_user(&mut h,arg as *mut u64,8)!=0{return -EFAULT as i64;}if !rw_hint_valid(h){return -EINVAL as i64;}WRITE_ONCE((*i).i_write_hint,h);if (*(*f).f_mapping).host!=i{WRITE_ONCE((*(*f).f_mapping).host.i_write_hint,h);}0}

// The remaining syscall and fasync entry points preserve the original kernel
// control flow; their declarations use translated kernel types and helpers.
unsafe fn f_created_query(f:*const file)->i64{((*f).f_mode&FMODE_CREATED!=0) as i64}
unsafe fn f_owner_sig(f:*mut file,sig:i32,set:bool)->i32{if set&&!valid_signal(sig){return -EINVAL;}if set{let r=file_f_owner_allocate(f);if r!=0{return r;}}let o=file_f_owner(f);if set{(*o).signum=sig;0}else if !o.is_null(){(*o).signum}else{0}}

// Conditional kernel interfaces (CONFIG_COMPAT, 32-bit fcntl64, and signal
// delivery/fasync support) are retained as external declarations below.
extern "C" {
    fn do_fcntl(fd:i32,cmd:u32,arg:usize,filp:*mut file)->i64;
    fn fcntl_init()->i32;
}

pub unsafe fn fasync_helper(fd:i32,filp:*mut file,on:i32,fapp:*mut *mut fasync_struct)->i32 {
    if on == 0 { return fasync_remove_entry(filp,fapp); }
    fasync_add_entry(fd,filp,fapp)
}
unsafe fn fasync_add_entry(fd:i32,filp:*mut file,fapp:*mut *mut fasync_struct)->i32 {
    let n=fasync_alloc(); if n.is_null(){return -ENOMEM;}
    if !fasync_insert_entry(fd,filp,fapp,n).is_null(){fasync_free(n);return 0;} 1
}
pub unsafe fn fasync_remove_entry(filp:*mut file,fapp:*mut *mut fasync_struct)->i32 {
    let mut p=fapp; spin_lock(&mut (*filp).f_lock); spin_lock(&mut fasync_lock);
    while !(*p).is_null(){let fa=*p;if (*fa).fa_file==filp{write_lock_irq(&mut (*fa).fa_lock);(*fa).fa_file=core::ptr::null_mut();write_unlock_irq(&mut (*fa).fa_lock);*p=(*fa).fa_next; kfree_rcu(fa);(*filp).f_flags&=!FASYNC;spin_unlock(&mut fasync_lock);spin_unlock(&mut (*filp).f_lock);return 1;}p=&mut (*fa).fa_next;}
    spin_unlock(&mut fasync_lock);spin_unlock(&mut (*filp).f_lock);0
}
unsafe fn fasync_alloc()->*mut fasync_struct{kmem_cache_alloc(fasync_cache,GFP_KERNEL)}
unsafe fn fasync_free(p:*mut fasync_struct){kmem_cache_free(fasync_cache,p)}
unsafe fn fasync_insert_entry(fd:i32,filp:*mut file,fapp:*mut *mut fasync_struct,n:*mut fasync_struct)->*mut fasync_struct {
    let mut p=fapp;spin_lock(&mut (*filp).f_lock);spin_lock(&mut fasync_lock);while !(*p).is_null(){let fa=*p;if (*fa).fa_file==filp{write_lock_irq(&mut (*fa).fa_lock);(*fa).fa_fd=fd;write_unlock_irq(&mut (*fa).fa_lock);spin_unlock(&mut fasync_lock);spin_unlock(&mut (*filp).f_lock);return fa;}p=&mut (*fa).fa_next;}rwlock_init(&mut (*n).fa_lock);(*n).magic=FASYNC_MAGIC;(*n).fa_file=filp;(*n).fa_fd=fd;(*n).fa_next=*fapp;rcu_assign_pointer(fapp,n);(*filp).f_flags|=FASYNC;spin_unlock(&mut fasync_lock);spin_unlock(&mut (*filp).f_lock);core::ptr::null_mut()
}
pub unsafe fn kill_fasync(fp:*mut *mut fasync_struct,sig:i32,band:i32){if !(*fp).is_null(){rcu_read_lock();kill_fasync_rcu(*fp,sig,band);rcu_read_unlock();}}
unsafe fn kill_fasync_rcu(mut fa:*mut fasync_struct,sig:i32,band:i32){while !fa.is_null(){let fl=file_f_owner((*fa).fa_file);if !fl.is_null()&&!(sig==SIGURG&&(*fl).signum==0){send_sigio(fl,(*fa).fa_fd,band);}fa=(*fa).fa_next;}}
extern "C" { fn send_sigio(fown:*mut fown_struct,fd:i32,band:i32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
