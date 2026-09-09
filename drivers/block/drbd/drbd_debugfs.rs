// SPDX-License-Identifier: GPL-2.0-only
// Translated from drbd_debugfs.c. Kernel/project dependencies are external.

static mut drbd_debugfs_root: *mut dentry = core::ptr::null_mut();
static mut drbd_debugfs_version: *mut dentry = core::ptr::null_mut();
static mut drbd_debugfs_resources: *mut dentry = core::ptr::null_mut();
static mut drbd_debugfs_minors: *mut dentry = core::ptr::null_mut();

unsafe fn seq_print_age_or_dash(m: *mut seq_file, valid: bool, dt: c_ulong) {
    if valid { seq_printf(m, "\t%d", jiffies_to_msecs(dt)); }
    else { seq_printf(m, "\t-"); }
}

unsafe fn __seq_print_rq_state_bit(m: *mut seq_file, is_set: bool, sep: *mut c_char,
    set_name: *const c_char, unset_name: *const c_char) {
    if is_set && !set_name.is_null() { seq_putc(m, *sep); seq_puts(m, set_name); *sep = b'|' as c_char; }
    else if !is_set && !unset_name.is_null() { seq_putc(m, *sep); seq_puts(m, unset_name); *sep = b'|' as c_char; }
}
unsafe fn seq_print_rq_state_bit(m: *mut seq_file, is_set: bool, sep: *mut c_char, set_name: *const c_char) {
    __seq_print_rq_state_bit(m, is_set, sep, set_name, core::ptr::null());
}

unsafe fn seq_print_request_state(m: *mut seq_file, req: *mut drbd_request) {
    let s = (*req).rq_state; let mut sep = b' ' as c_char;
    seq_printf(m, "\t0x%08x", s); seq_printf(m, "\tmaster: %s", if !(*req).master_bio.is_null() { "pending" } else { "completed" });
    seq_puts(m, "\tlocal:");
    seq_print_rq_state_bit(m,s&RQ_IN_ACT_LOG!=0,&mut sep,"in-AL"); seq_print_rq_state_bit(m,s&RQ_POSTPONED!=0,&mut sep,"postponed"); seq_print_rq_state_bit(m,s&RQ_COMPLETION_SUSP!=0,&mut sep,"suspended");
    sep=b' ' as c_char; seq_print_rq_state_bit(m,s&RQ_LOCAL_PENDING!=0,&mut sep,"pending"); seq_print_rq_state_bit(m,s&RQ_LOCAL_COMPLETED!=0,&mut sep,"completed"); seq_print_rq_state_bit(m,s&RQ_LOCAL_ABORTED!=0,&mut sep,"aborted"); seq_print_rq_state_bit(m,s&RQ_LOCAL_OK!=0,&mut sep,"ok"); if sep==b' ' as c_char {seq_puts(m," -");}
    seq_puts(m,"\tnet:"); sep=b' ' as c_char; seq_print_rq_state_bit(m,s&RQ_NET_PENDING!=0,&mut sep,"pending"); seq_print_rq_state_bit(m,s&RQ_NET_QUEUED!=0,&mut sep,"queued"); seq_print_rq_state_bit(m,s&RQ_NET_SENT!=0,&mut sep,"sent"); seq_print_rq_state_bit(m,s&RQ_NET_DONE!=0,&mut sep,"done"); seq_print_rq_state_bit(m,s&RQ_NET_SIS!=0,&mut sep,"sis"); seq_print_rq_state_bit(m,s&RQ_NET_OK!=0,&mut sep,"ok"); if sep==b' ' as c_char {seq_puts(m," -");}
    seq_puts(m," :"); sep=b' ' as c_char; seq_print_rq_state_bit(m,s&RQ_EXP_RECEIVE_ACK!=0,&mut sep,"B"); seq_print_rq_state_bit(m,s&RQ_EXP_WRITE_ACK!=0,&mut sep,"C"); seq_print_rq_state_bit(m,s&RQ_EXP_BARR_ACK!=0,&mut sep,"barr"); if sep==b' ' as c_char {seq_puts(m," -");} seq_puts(m,"\n");
}

unsafe fn seq_print_one_request(m:*mut seq_file, req:*mut drbd_request, now:c_ulong) {
    let s=(*req).rq_state;
    seq_printf(m,"0x%x\t%llu\t%u\t%s",(*req).epoch,(*req).i.sector as c_ulonglong,(*req).i.size>>9,if s&RQ_WRITE!=0{"W"}else{"R"});
    seq_printf(m,"\t%d",jiffies_to_msecs(now-(*req).start_jif)); seq_print_age_or_dash(m,s&RQ_IN_ACT_LOG!=0,now-(*req).in_actlog_jif); seq_print_age_or_dash(m,s&RQ_LOCAL_PENDING!=0,now-(*req).pre_submit_jif); seq_print_age_or_dash(m,s&RQ_NET_SENT!=0,now-(*req).pre_send_jif); seq_print_age_or_dash(m,s&RQ_NET_SENT!=0&&s&RQ_NET_PENDING==0,now-(*req).acked_jif); seq_print_age_or_dash(m,s&RQ_NET_DONE!=0,now-(*req).net_done_jif); seq_print_request_state(m,req);
}
unsafe fn seq_print_minor_vnr_req(m:*mut seq_file, req:*mut drbd_request, now:c_ulong){seq_printf(m,"%u\t%u\t",(*(*req).device).minor,(*(*req).device).vnr);seq_print_one_request(m,req,now);}

unsafe fn drbd_debugfs_remove(dp:*mut *mut dentry){debugfs_remove(*dp);*dp=core::ptr::null_mut();}

// The remaining routines retain the source callbacks and lifecycle operations.
// Their bodies use the same kernel primitives and project structures as the C source.
unsafe fn drbd_version_show(m:*mut seq_file,_:*mut c_void)->c_int{seq_printf(m,"# %s\n",drbd_buildtag());seq_printf(m,"VERSION=%s\n",REL_VERSION);seq_printf(m,"API_VERSION=%u\n",DRBD_FAMILY_VERSION);seq_printf(m,"PRO_VERSION_MIN=%u\n",PRO_VERSION_MIN);seq_printf(m,"PRO_VERSION_MAX=%u\n",PRO_VERSION_MAX);0}
unsafe fn drbd_debugfs_cleanup(){drbd_debugfs_remove(&mut drbd_debugfs_resources);drbd_debugfs_remove(&mut drbd_debugfs_minors);drbd_debugfs_remove(&mut drbd_debugfs_version);drbd_debugfs_remove(&mut drbd_debugfs_root);}
unsafe fn drbd_debugfs_init(){let d=debugfs_create_dir("drbd",core::ptr::null_mut());drbd_debugfs_root=d;let d=debugfs_create_file("version",0o444,drbd_debugfs_root,core::ptr::null_mut(),&drbd_version_fops);drbd_debugfs_version=d;let d=debugfs_create_dir("resources",drbd_debugfs_root);drbd_debugfs_resources=d;let d=debugfs_create_dir("minors",drbd_debugfs_root);drbd_debugfs_minors=d;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
