// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
 * Copyright 2005-2006 Ian Kent <raven@themaw.net>
 */

// Linux kernel dependencies supplied by the surrounding translation.

pub unsafe fn autofs_new_ino(sbi: *mut autofs_sb_info) -> *mut autofs_info {
    let ino = kzalloc_obj::<autofs_info>();
    if !ino.is_null() {
        INIT_LIST_HEAD(&mut (*ino).active);
        INIT_LIST_HEAD(&mut (*ino).expiring);
        (*ino).last_used = jiffies;
        (*ino).sbi = sbi;
        (*ino).exp_timeout = -1;
        (*ino).count = 1;
    }
    ino
}

pub unsafe fn autofs_clean_ino(ino: *mut autofs_info) {
    (*ino).uid = GLOBAL_ROOT_UID;
    (*ino).gid = GLOBAL_ROOT_GID;
    (*ino).exp_timeout = -1;
    (*ino).last_used = jiffies;
}

pub unsafe fn autofs_free_ino(ino: *mut autofs_info) {
    kfree_rcu(ino, rcu);
}

pub unsafe fn autofs_kill_sb(sb: *mut super_block) {
    let sbi = autofs_sbi(sb);
    if !sbi.is_null() {
        autofs_catatonic_mode(sbi);
        put_pid((*sbi).oz_pgrp);
    }
    pr_debug!("shutting down\n");
    kill_anon_super(sb);
    if !sbi.is_null() { kfree_rcu(sbi, rcu); }
}

unsafe fn autofs_show_options(m: *mut seq_file, root: *mut dentry) -> c_int {
    let sbi = autofs_sbi((*root).d_sb);
    let root_inode = d_inode((*(*root).d_sb).s_root);
    if sbi.is_null() { return 0; }
    seq_printf(m, ",fd=%d", (*sbi).pipefd);
    if !uid_eq((*root_inode).i_uid, GLOBAL_ROOT_UID) {
        seq_printf(m, ",uid=%u", from_kuid_munged(&init_user_ns, (*root_inode).i_uid));
    }
    if !gid_eq((*root_inode).i_gid, GLOBAL_ROOT_GID) {
        seq_printf(m, ",gid=%u", from_kgid_munged(&init_user_ns, (*root_inode).i_gid));
    }
    seq_printf(m, ",pgrp=%d", pid_vnr((*sbi).oz_pgrp));
    seq_printf(m, ",timeout=%lu", (*sbi).exp_timeout / HZ);
    seq_printf(m, ",minproto=%d", (*sbi).min_proto);
    seq_printf(m, ",maxproto=%d", (*sbi).max_proto);
    if autofs_type_offset((*sbi).type) { seq_puts(m, ",offset"); }
    else if autofs_type_direct((*sbi).type) { seq_puts(m, ",direct"); }
    else { seq_puts(m, ",indirect"); }
    if (*sbi).flags & AUTOFS_SBI_STRICTEXPIRE { seq_puts(m, ",strictexpire"); }
    if (*sbi).flags & AUTOFS_SBI_IGNORE { seq_puts(m, ",ignore"); }
    #[cfg(feature = "CONFIG_CHECKPOINT_RESTORE")]
    {
        if !(*sbi).pipe.is_null() { seq_printf(m, ",pipe_ino=%llu", (*file_inode((*sbi).pipe)).i_ino); }
        else { seq_puts(m, ",pipe_ino=-1"); }
    }
    0
}

unsafe fn autofs_evict_inode(inode: *mut inode) {
    clear_inode(inode);
    kfree((*inode).i_private);
}

static AUTOfs_SOPS: super_operations = super_operations {
    statfs: Some(simple_statfs), show_options: Some(autofs_show_options),
    evict_inode: Some(autofs_evict_inode),
};

pub enum autofs_option { Opt_direct, Opt_fd, Opt_gid, Opt_ignore, Opt_indirect, Opt_maxproto, Opt_minproto, Opt_offset, Opt_pgrp, Opt_strictexpire, Opt_uid }

pub const autofs_param_specs: [fs_parameter_spec; 12] = [
    fsparam_flag!("direct", Opt_direct), fsparam_fd!("fd", Opt_fd), fsparam_gid!("gid", Opt_gid),
    fsparam_flag!("ignore", Opt_ignore), fsparam_flag!("indirect", Opt_indirect), fsparam_u32!("maxproto", Opt_maxproto),
    fsparam_u32!("minproto", Opt_minproto), fsparam_flag!("offset", Opt_offset), fsparam_u32!("pgrp", Opt_pgrp),
    fsparam_flag!("strictexpire", Opt_strictexpire), fsparam_uid!("uid", Opt_uid), fs_parameter_spec::default(),
];

#[repr(C)] pub struct autofs_fs_context { pub uid: kuid_t, pub gid: kgid_t, pub pgrp: c_int, pub pgrp_set: bool }

unsafe fn autofs_parse_fd(fc: *mut fs_context, sbi: *mut autofs_sb_info, param: *mut fs_parameter, result: *mut fs_parse_result) -> c_int {
    let pipe = if (*param).type_ == fs_value_is_file { let p = (*param).file; (*param).file = core::ptr::null_mut(); p } else { fget((*result).uint_32) };
    if pipe.is_null() { errorf!(fc, "could not open pipe file descriptor"); return -EBADF; }
    if autofs_check_pipe(pipe) < 0 { errorf!(fc, "Invalid/unusable pipe"); fput(pipe); return -EBADF; }
    autofs_set_packet_pipe_flags(pipe);
    if !(*sbi).pipe.is_null() { fput((*sbi).pipe); }
    (*sbi).pipefd = (*result).uint_32; (*sbi).pipe = pipe; 0
}

unsafe fn autofs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let ctx = (*fc).fs_private as *mut autofs_fs_context; let sbi = (*fc).s_fs_info;
    let mut result = fs_parse_result::default(); let opt = fs_parse(fc, &autofs_param_specs, param, &mut result);
    if opt < 0 { return opt; }
    match opt { Opt_fd => autofs_parse_fd(fc, sbi, param, &mut result), Opt_uid => { (*ctx).uid=result.uid; 0 }, Opt_gid => { (*ctx).gid=result.gid; 0 }, Opt_pgrp => { (*ctx).pgrp=result.uint_32; (*ctx).pgrp_set=true; 0 }, Opt_minproto => { (*sbi).min_proto=result.uint_32; 0 }, Opt_maxproto => { (*sbi).max_proto=result.uint_32; 0 }, Opt_indirect => { set_autofs_type_indirect(&mut (*sbi).type_); 0 }, Opt_direct => { set_autofs_type_direct(&mut (*sbi).type_); 0 }, Opt_offset => { set_autofs_type_offset(&mut (*sbi).type_); 0 }, Opt_strictexpire => { (*sbi).flags |= AUTOFS_SBI_STRICTEXPIRE; 0 }, Opt_ignore => { (*sbi).flags |= AUTOFS_SBI_IGNORE; 0 } }
}

unsafe fn autofs_alloc_sbi() -> *mut autofs_sb_info {
    let sbi = kzalloc_obj::<autofs_sb_info>();
    if sbi.is_null() { return core::ptr::null_mut(); }
    (*sbi).magic = AUTOFS_SBI_MAGIC; (*sbi).flags = AUTOFS_SBI_CATATONIC;
    (*sbi).min_proto = AUTOFS_MIN_PROTO_VERSION; (*sbi).max_proto = AUTOFS_MAX_PROTO_VERSION;
    (*sbi).pipefd = -1; (*sbi).mnt_ns_id = to_ns_common((*current).nsproxy.mnt_ns).ns_id;
    set_autofs_type_indirect(&mut (*sbi).type_); mutex_init(&mut (*sbi).wq_mutex);
    mutex_init(&mut (*sbi).pipe_mutex); spin_lock_init(&mut (*sbi).fs_lock); spin_lock_init(&mut (*sbi).lookup_lock);
    INIT_LIST_HEAD(&mut (*sbi).active_list); INIT_LIST_HEAD(&mut (*sbi).expiring_list); sbi
}

unsafe fn autofs_validate_protocol(fc: *mut fs_context) -> c_int {
    let sbi = (*fc).s_fs_info;
    if (*sbi).max_proto < AUTOFS_MIN_PROTO_VERSION || (*sbi).min_proto > AUTOFS_MAX_PROTO_VERSION {
        errorf!(fc, "kernel does not match daemon version daemon ({}, {}) kernel ({}, {})\n", (*sbi).min_proto, (*sbi).max_proto, AUTOFS_MIN_PROTO_VERSION, AUTOFS_MAX_PROTO_VERSION); return -EINVAL;
    }
    (*sbi).version = if (*sbi).max_proto > AUTOFS_MAX_PROTO_VERSION { AUTOFS_MAX_PROTO_VERSION } else { (*sbi).max_proto };
    (*sbi).sub_version = match (*sbi).version { 4 => 7, 5 => AUTOFS_PROTO_SUBVERSION, _ => 0 }; 0
}

unsafe fn autofs_free_fc(fc: *mut fs_context) {
    let ctx = (*fc).fs_private as *mut autofs_fs_context; let sbi = (*fc).s_fs_info;
    if !sbi.is_null() { if !(*sbi).pipe.is_null() { fput((*sbi).pipe); } kfree(sbi); } kfree(ctx);
}

pub unsafe fn autofs_init_fs_context(fc: *mut fs_context) -> c_int {
    let ctx = kzalloc_obj::<autofs_fs_context>(); if ctx.is_null() { return -ENOMEM; }
    (*ctx).uid = current_uid(); (*ctx).gid = current_gid(); let sbi = autofs_alloc_sbi();
    if sbi.is_null() { kfree(ctx); return -ENOMEM; }
    (*fc).fs_private = ctx as *mut c_void; (*fc).s_fs_info = sbi; (*fc).ops = &autofs_context_ops; 0
}

pub unsafe fn autofs_get_inode(sb: *mut super_block, mode: umode_t) -> *mut inode {
    let inode = new_inode(sb); if inode.is_null() { return core::ptr::null_mut(); }
    (*inode).i_mode = mode;
    if !(*sb).s_root.is_null() { (*inode).i_uid = (*d_inode((*sb).s_root)).i_uid; (*inode).i_gid = (*d_inode((*sb).s_root)).i_gid; }
    simple_inode_init_ts(inode); (*inode).i_ino = get_next_ino();
    if S_ISDIR(mode) { set_nlink(inode, 2); (*inode).i_op = &autofs_dir_inode_operations; (*inode).i_fop = &autofs_dir_operations; }
    else if S_ISLNK(mode) { (*inode).i_op = &autofs_symlink_inode_operations; } else { WARN_ON!(1); }
    inode
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
