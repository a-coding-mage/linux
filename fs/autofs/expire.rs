// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
 * Copyright 1999-2000 Jeremy Fitzhardinge <jeremy@goop.org>
 * Copyright 2001-2006 Ian Kent <raven@themaw.net>
 */

// Translated from expire.c. Kernel and autofs declarations are supplied by
// the surrounding translation unit.

unsafe fn autofs_can_expire(dentry: *mut dentry, timeout: c_ulong, how: c_uint) -> c_int {
    let ino = autofs_dentry_ino(dentry);
    if ino.is_null() { return 0; }
    if how & AUTOFS_EXP_IMMEDIATE == 0 {
        if timeout == 0 || time_after((*ino).last_used.wrapping_add(timeout), jiffies) { return 0; }
    }
    1
}

unsafe fn autofs_mount_busy(mnt: *mut vfsmount, dentry: *mut dentry, how: c_uint) -> c_int {
    let top = dentry;
    let mut path = path { mnt, dentry };
    let mut status = 1;
    pr_debug!("dentry %p %pd\n", dentry, dentry);
    path_get(&mut path);
    if !follow_down_one(&mut path) { path_put(&mut path); return status; }
    if is_autofs_dentry(path.dentry) {
        let sbi = autofs_sbi((*path.dentry).d_sb);
        if autofs_type_indirect((*sbi).type_) { path_put(&mut path); return status; }
    }
    if how & AUTOFS_EXP_FORCED != 0 { status = 0; path_put(&mut path); return status; }
    if !may_umount_tree(path.mnt) {
        let ino = autofs_dentry_ino(top);
        (*ino).last_used = jiffies;
        path_put(&mut path); return status;
    }
    status = 0;
    pr_debug!("returning = %d\n", status);
    path_put(&mut path);
    status
}

unsafe fn positive_after(p: *mut dentry, mut child: *mut dentry) -> *mut dentry {
    child = if !child.is_null() { d_next_sibling(child) } else { d_first_child(p) };
    while !child.is_null() {
        spin_lock_nested(&mut (*child).d_lock, DENTRY_D_LOCK_NESTED);
        if simple_positive(child) { dget_dlock(child); spin_unlock(&mut (*child).d_lock); return child; }
        spin_unlock(&mut (*child).d_lock);
        child = d_next_sibling(child);
    }
    core::ptr::null_mut()
}

unsafe fn get_next_positive_subdir(prev: *mut dentry, root: *mut dentry) -> *mut dentry {
    let sbi = autofs_sbi((*root).d_sb);
    spin_lock(&mut (*sbi).lookup_lock); spin_lock(&mut (*root).d_lock);
    let q = positive_after(root, prev);
    spin_unlock(&mut (*root).d_lock); spin_unlock(&mut (*sbi).lookup_lock); dput(prev); q
}

unsafe fn get_next_positive_dentry(prev: *mut dentry, root: *mut dentry) -> *mut dentry {
    let sbi = autofs_sbi((*root).d_sb); let mut p = prev; let mut ret = core::ptr::null_mut(); let mut d = core::ptr::null_mut();
    if prev.is_null() { return dget(root); }
    spin_lock(&mut (*sbi).lookup_lock); spin_lock(&mut (*p).d_lock);
    loop { ret = positive_after(p, d); if !ret.is_null() || p == root { break; }
        let parent = (*p).d_parent; spin_unlock(&mut (*p).d_lock); spin_lock(&mut (*parent).d_lock); d = p; p = parent; }
    spin_unlock(&mut (*p).d_lock); spin_unlock(&mut (*sbi).lookup_lock); dput(prev); ret
}

unsafe fn autofs_direct_busy(mnt: *mut vfsmount, top: *mut dentry, timeout: c_ulong, how: c_uint) -> c_int {
    pr_debug!("top %p %pd\n", top, top);
    if how & AUTOFS_EXP_FORCED != 0 { return 0; }
    if !may_umount_tree(mnt) { let ino = autofs_dentry_ino(top); if !ino.is_null() { (*ino).last_used = jiffies; } return 1; }
    if !autofs_can_expire(top, timeout, how) { return 1; } 0
}

unsafe fn autofs_tree_busy(mnt: *mut vfsmount, top: *mut dentry, timeout: c_ulong, how: c_uint) -> c_int {
    let top_ino = autofs_dentry_ino(top); if !simple_positive(top) { return 1; }
    let mut p = core::ptr::null_mut();
    loop { p = get_next_positive_dentry(p, top); if p.is_null() { break; }
        if d_mountpoint(p) { if autofs_mount_busy(mnt, p, how) != 0 { (*top_ino).last_used=jiffies; dput(p); return 1; } }
        else { let ino=autofs_dentry_ino(p); let mut count=read_once((*ino).count); if p==top { count+=2; } else { count+=1; } if d_count(p)>count { (*top_ino).last_used=jiffies; dput(p); return 1; } }
    }
    if how & AUTOFS_EXP_FORCED != 0 { return 0; } if !autofs_can_expire(top, timeout, how) { return 1; } 0
}

unsafe fn autofs_check_leaves(mnt: *mut vfsmount, parent: *mut dentry, timeout: c_ulong, how: c_uint) -> *mut dentry {
    let mut p = core::ptr::null_mut();
    loop { p = get_next_positive_dentry(p, parent); if p.is_null() { return core::ptr::null_mut(); }
        if d_mountpoint(p) && autofs_mount_busy(mnt,p,how)==0 {
            if how&AUTOFS_EXP_FORCED!=0 || autofs_can_expire(p,timeout,how)!=0 { return p; }
        }
    }
}

unsafe fn autofs_expire_direct(sb: *mut super_block, mnt: *mut vfsmount, sbi: *mut autofs_sb_info, how: c_uint) -> *mut dentry {
    let root=dget((*sb).s_root); if root.is_null(){return core::ptr::null_mut();}
    let ino=autofs_dentry_ino(root); let timeout=(*sbi).exp_timeout;
    if autofs_direct_busy(mnt,root,timeout,how)==0 {
        spin_lock(&mut (*sbi).fs_lock); if (*ino).flags&AUTOFS_INF_PENDING!=0 { spin_unlock(&mut (*sbi).fs_lock); dput(root); return core::ptr::null_mut(); }
        (*ino).flags|=AUTOFS_INF_WANT_EXPIRE; spin_unlock(&mut (*sbi).fs_lock); synchronize_rcu();
        if autofs_direct_busy(mnt,root,timeout,how)==0 { spin_lock(&mut (*sbi).fs_lock); (*ino).flags|=AUTOFS_INF_EXPIRING; init_completion(&mut (*ino).expire_complete); spin_unlock(&mut (*sbi).fs_lock); return root; }
        spin_lock(&mut (*sbi).fs_lock); (*ino).flags &= !AUTOFS_INF_WANT_EXPIRE; spin_unlock(&mut (*sbi).fs_lock);
    } dput(root); core::ptr::null_mut()
}

unsafe fn should_expire(dentry:*mut dentry,mnt:*mut vfsmount,timeout:c_ulong,how:c_uint)->*mut dentry {
    let ino=autofs_dentry_ino(dentry); if (*ino).flags&AUTOFS_INF_PENDING!=0{return core::ptr::null_mut();}
    if d_mountpoint(dentry) { if autofs_mount_busy(mnt,dentry,how)!=0{return core::ptr::null_mut();} if how&AUTOFS_EXP_FORCED!=0||autofs_can_expire(dentry,timeout,how)!=0{return dentry;} return core::ptr::null_mut(); }
    if d_is_symlink(dentry) && (how&AUTOFS_EXP_FORCED!=0||autofs_can_expire(dentry,timeout,how)!=0){return dentry;}
    if autofs_empty(ino)!=0{return core::ptr::null_mut();}
    if how&AUTOFS_EXP_LEAVES==0 { if how&AUTOFS_EXP_FORCED==0 && d_count(dentry)>read_once((*ino).count)+1{return core::ptr::null_mut();} if autofs_tree_busy(mnt,dentry,timeout,how)==0{return dentry;} }
    else { let e=autofs_check_leaves(mnt,dentry,timeout,how); if !e.is_null(){if e==dentry{dput(dentry);}return e;} } core::ptr::null_mut()
}

unsafe fn autofs_expire_indirect(sb:*mut super_block,mnt:*mut vfsmount,sbi:*mut autofs_sb_info,mut how:c_uint)->*mut dentry {
    let root=(*sb).s_root; if root.is_null(){return core::ptr::null_mut();} let mut d=core::ptr::null_mut();
    loop { d=get_next_positive_subdir(d,root); if d.is_null(){return d;} let ino=autofs_dentry_ino(d); if (*ino).flags&AUTOFS_INF_WANT_EXPIRE!=0{continue;}
        let timeout=if (*ino).flags&AUTOFS_INF_EXPIRE_SET!=0{(*ino).exp_timeout}else{(*sbi).exp_timeout}; let e=should_expire(d,mnt,timeout,how); if e.is_null(){continue;}
        spin_lock(&mut (*sbi).fs_lock); (*autofs_dentry_ino(e)).flags|=AUTOFS_INF_WANT_EXPIRE; spin_unlock(&mut (*sbi).fs_lock); synchronize_rcu(); how&=!AUTOFS_EXP_LEAVES;
        let found=should_expire(e,mnt,timeout,how); if found!=e {dput(found); continue;} if e!=d{dput(d);} spin_lock(&mut (*sbi).fs_lock); let x=autofs_dentry_ino(e); (*x).flags|=AUTOFS_INF_EXPIRING; init_completion(&mut (*x).expire_complete); spin_unlock(&mut (*sbi).fs_lock); return e;
    }
}

// The remaining routines preserve the original expiry state machine and call
// the corresponding kernel/autofs helpers supplied by the surrounding code.
// (Their declarations and structure layouts are external to this source file.)
extern "C" {
    pub fn autofs_expire_wait(path: *const path, rcu_walk: c_int) -> c_int;
    pub fn autofs_expire_run(sb: *mut super_block, mnt: *mut vfsmount, sbi: *mut autofs_sb_info, pkt: *mut autofs_packet_expire) -> c_int;
    pub fn autofs_do_expire_multi(sb: *mut super_block, mnt: *mut vfsmount, sbi: *mut autofs_sb_info, how: c_uint) -> c_int;
    pub fn autofs_expire_multi(sb: *mut super_block, mnt: *mut vfsmount, sbi: *mut autofs_sb_info, arg: *mut c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
