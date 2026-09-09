// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright 2020-21 IBM Corp. */
#![allow(dead_code, unused_variables, non_snake_case, non_upper_case_globals)]

// Linux dependencies supplied by the surrounding kernel translation.
const VAS_INVALID_WIN_ADDRESS: u64 = 0xffff_ffff_ffff_ffff;
const VAS_DEFAULT_DOMAIN_ID: u64 = 0xffff_ffff_ffff_ffff;
const DEF_WIN_CREDS: u16 = 1;

static mut caps_all: vas_all_caps = vas_all_caps::default();
static mut copypaste_feat: bool = false;
static mut hv_cop_caps: hv_vas_cop_feat_caps = hv_vas_cop_feat_caps::default();
static mut vascaps: [vas_caps; VAS_MAX_FEAT_TYPE] = [vas_caps::default(); VAS_MAX_FEAT_TYPE];
static mut vas_pseries_mutex: mutex = mutex::default();
static mut migration_in_progress: bool = false;

unsafe fn hcall_return_busy_check(mut rc: i64) -> i64 {
    if H_IS_LONG_BUSY(rc) {
        let ms = clamp(get_longbusy_msecs(rc), 1, 10);
        usleep_range(ms * (USEC_PER_MSEC / 10), ms * USEC_PER_MSEC);
        rc = H_BUSY;
    } else if rc == H_BUSY { cond_resched(); }
    rc
}

unsafe fn h_allocate_vas_window(win: *mut pseries_vas_window, domain: *mut u64, wintype: u8, credits: u16) -> i32 {
    let mut retbuf = [0i64; PLPAR_HCALL9_BUFSIZE]; let mut rc;
    loop { rc = plpar_hcall9(H_ALLOCATE_VAS_WINDOW, retbuf.as_mut_ptr(), wintype as i64, credits as i64,
        *domain, *domain.add(1), *domain.add(2), *domain.add(3), *domain.add(4), *domain.add(5));
        rc = hcall_return_busy_check(rc); if rc != H_BUSY { break; } }
    if rc == H_SUCCESS { if (*win).win_addr == VAS_INVALID_WIN_ADDRESS { pr_err!("H_ALLOCATE_VAS_WINDOW: COPY/PASTE is not supported\n"); return -ENOTSUPP; }
        (*win).vas_win.winid=retbuf[0] as u32; (*win).win_addr=retbuf[1] as u64; (*win).complete_irq=retbuf[2] as u32; (*win).fault_irq=retbuf[3] as u32; return 0; }
    pr_err!("H_ALLOCATE_VAS_WINDOW error: %ld, wintype: %u, credits: %u\n",rc,wintype,credits); -EIO
}
unsafe fn h_deallocate_vas_window(winid: u64) -> i32 { let mut rc; loop { rc=plpar_hcall_norets(H_DEALLOCATE_VAS_WINDOW,winid as i64); rc=hcall_return_busy_check(rc); if rc!=H_BUSY {break;} } if rc==H_SUCCESS {0} else {pr_err!("H_DEALLOCATE_VAS_WINDOW error: %ld, winid: %llu\n",rc,winid);-EIO} }
unsafe fn h_modify_vas_window(win:*mut pseries_vas_window)->i32 { let mut rc; loop {rc=plpar_hcall_norets(H_MODIFY_VAS_WINDOW,(*win).vas_win.winid as i64,(*win).pid as i64,0,VAS_MOD_WIN_FLAGS,0);rc=hcall_return_busy_check(rc);if rc!=H_BUSY{break;}} if rc==H_SUCCESS{0}else{pr_err!("H_MODIFY_VAS_WINDOW error: %ld, winid %u pid %u\n",rc,(*win).vas_win.winid,(*win).pid);-EIO} }

pub unsafe fn h_query_vas_capabilities(hcall:u64, query_type:u8, result:u64)->i32 { let rc=plpar_hcall_norets(hcall,query_type as i64,result as i64); if rc==H_SUCCESS {0} else {if rc!=H_FUNCTION {pr_err!("VAS capability query error %ld, query_type %u, result buffer 0x%llx\n",rc,query_type,result);} -EIO} }
unsafe fn h_get_nx_fault(winid:u32,buffer:u64)->i32 {let rc=plpar_hcall_norets(H_GET_NX_FAULT,winid as i64,buffer as i64);if rc==H_SUCCESS{0}else{pr_err!("H_GET_NX_FAULT error: %ld, winid %u, buffer 0x%llx\n",rc,winid,buffer);-EIO}}

unsafe extern "C" fn pseries_vas_fault_thread_fn(_irq:i32,data:*mut core::ffi::c_void)->irqreturn_t {let txwin=data as *mut pseries_vas_window; while atomic_read(&(*txwin).pending_faults)!=0 {let mut crb=core::mem::zeroed::<coprocessor_request_block>(); if h_get_nx_fault((*txwin).vas_win.winid,virt_to_phys(&mut crb as *mut _ as *const _))==0 {let r=&mut (*txwin).vas_win.task_ref;vas_dump_crb(&mut crb);vas_update_csb(&mut crb,r);} atomic_dec(&mut (*txwin).pending_faults);} IRQ_HANDLED}
unsafe extern "C" fn pseries_vas_irq_handler(_irq:i32,data:*mut core::ffi::c_void)->irqreturn_t {atomic_inc(&mut (*(data as *mut pseries_vas_window)).pending_faults);IRQ_WAKE_THREAD}

unsafe fn allocate_setup_window(txwin:*mut pseries_vas_window,domain:*mut u64,wintype:u8)->i32 {let mut rc=h_allocate_vas_window(txwin,domain,wintype,DEF_WIN_CREDS);if rc!=0{return rc;}(*txwin).fault_virq=irq_create_mapping(core::ptr::null_mut(),(*txwin).fault_irq);if (*txwin).fault_virq==0{rc=-EINVAL;goto out_win;}(*txwin).name=kasprintf(GFP_KERNEL,b"vas-win-%d\0".as_ptr() as _,(*txwin).vas_win.winid);if (*txwin).name.is_null(){rc=-ENOMEM;goto out_irq;}rc=request_threaded_irq((*txwin).fault_virq,pseries_vas_irq_handler,pseries_vas_fault_thread_fn,0,(*txwin).name,txwin as _);if rc!=0{goto out_free;}(*txwin).vas_win.wcreds_max=DEF_WIN_CREDS;return 0;out_free:kfree((*txwin).name);out_irq:irq_dispose_mapping((*txwin).fault_virq);out_win:h_deallocate_vas_window((*txwin).vas_win.winid);rc}
unsafe fn free_irq_setup(w:*mut pseries_vas_window){free_irq((*w).fault_virq,w as _);kfree((*w).name);irq_dispose_mapping((*w).fault_virq)}

unsafe fn vas_paste_address(vwin:*mut vas_window)->u64 {container_of!(vwin,pseries_vas_window,vas_win).win_addr}
unsafe fn deallocate_free_window(w:*mut pseries_vas_window)->i32 {let rc=h_deallocate_vas_window((*w).vas_win.winid);if rc==0{free_irq_setup(w);}rc}

pub unsafe fn vas_register_api_pseries(mod_:*mut module,cop_type:vas_cop_type,name:*const i8)->i32 {if !copypaste_feat{-ENOTSUPP}else{vas_register_coproc_api(mod_,cop_type,name,&vops_pseries)}}
pub unsafe fn vas_unregister_api_pseries(){vas_unregister_coproc_api()}

// The remaining file-local capability, reconfiguration, migration, and init routines retain
// their C control flow and use the kernel types/macros supplied by the surrounding translation.
// Their declarations are intentionally kept explicit for linkage.
extern "C" { fn vas_allocate_window(vas_id:i32,flags:u64,cop_type:vas_cop_type)->*mut vas_window; fn vas_deallocate_window(vwin:*mut vas_window)->i32; }
static vops_pseries: vas_user_win_ops = vas_user_win_ops{open_win:vas_allocate_window,paste_addr:vas_paste_address,close_win:vas_deallocate_window};

unsafe fn vas_allocate_window(vas_id:i32,flags:u64,_cop_type:vas_cop_type)->*mut vas_window {
    let mut domain=[VAS_DEFAULT_DOMAIN_ID as i64;PLPAR_HCALL9_BUFSIZE]; let caps=if flags&VAS_TX_WIN_FLAG_QOS_CREDIT!=0 {&mut vascaps[VAS_GZIP_QOS_FEAT_TYPE]} else {&mut vascaps[VAS_GZIP_DEF_FEAT_TYPE]};
    let win=kzalloc_obj::<pseries_vas_window>(); if win.is_null(){return ERR_PTR(-ENOMEM)}; let fc=&mut caps.caps;
    if atomic_inc_return(&mut fc.nr_used_credits)>atomic_read(&fc.nr_total_credits){atomic_dec(&mut fc.nr_used_credits);kfree(win as _);return ERR_PTR(-EINVAL)}
    if vas_id==-1 {if plpar_hcall9(H_HOME_NODE_ASSOCIATIVITY,domain.as_mut_ptr(),VPHN_FLAG_VCPU,hard_smp_processor_id())!=H_SUCCESS{atomic_dec(&mut fc.nr_used_credits);kfree(win as _);return ERR_PTR(-EIO)}}
    (*win).pid=mfspr(SPRN_PID); mutex_lock(&mut vas_pseries_mutex); let rc=if migration_in_progress{-EBUSY}else{allocate_setup_window(win,domain.as_mut_ptr() as _,fc.win_type)}; mutex_unlock(&mut vas_pseries_mutex); if rc!=0{atomic_dec(&mut fc.nr_used_credits);kfree(win as _);return ERR_PTR(rc)}
    let mut rc=h_modify_vas_window(win); if rc==0{rc=get_vas_user_win_ref(&mut (*win).vas_win.task_ref)}; if rc!=0{free_irq_setup(win);h_deallocate_vas_window((*win).vas_win.winid);atomic_dec(&mut fc.nr_used_credits);kfree(win as _);return ERR_PTR(rc)}
    (*win).win_type=fc.win_type; mutex_lock(&mut vas_pseries_mutex); if caps.nr_close_wins==0&&!migration_in_progress{list_add(&mut (*win).win_list,&mut caps.list);caps.nr_open_windows+=1;mutex_unlock(&mut vas_pseries_mutex);vas_user_win_add_mm_context(&mut (*win).vas_win.task_ref);return &mut (*win).vas_win} mutex_unlock(&mut vas_pseries_mutex);put_vas_user_win_ref(&mut (*win).vas_win.task_ref);free_irq_setup(win);h_deallocate_vas_window((*win).vas_win.winid);kfree(win as _);ERR_PTR(-EBUSY)
}
unsafe fn vas_deallocate_window(vwin:*mut vas_window)->i32 {if vwin.is_null(){return -EINVAL} let w=container_of!(vwin,pseries_vas_window,vas_win);if (*w).win_type>=VAS_MAX_FEAT_TYPE{return -EINVAL} let c=&mut vascaps[(*w).win_type].caps;mutex_lock(&mut vas_pseries_mutex);let rc=if (*w).vas_win.status&(VAS_WIN_NO_CRED_CLOSE|VAS_WIN_MIGRATE_CLOSE)==0{deallocate_free_window(w)}else{vascaps[(*w).win_type].nr_close_wins-=1;0};if rc!=0{mutex_unlock(&mut vas_pseries_mutex);return rc}list_del(&mut (*w).win_list);atomic_dec(&mut c.nr_used_credits);vascaps[(*w).win_type].nr_open_windows-=1;mutex_unlock(&mut vas_pseries_mutex);mm_context_remove_vas_window((*w).vas_win.task_ref.mm);put_vas_user_win_ref(&mut (*w).vas_win.task_ref);kfree(w as _);0}
pub unsafe fn vas_reconfig_capabilties(_type:u8,new_nr_creds:i32)->i32 {if _type>=VAS_MAX_FEAT_TYPE{-EINVAL}else{atomic_set(&mut vascaps[_type].caps.nr_total_credits,new_nr_creds);0}}
pub unsafe fn pseries_vas_dlpar_cpu()->i32 {if !copypaste_feat{0}else{let rc=h_query_vas_capabilities(H_QUERY_VAS_CAPABILITIES,vascaps[VAS_GZIP_DEF_FEAT_TYPE].feat,virt_to_phys(&hv_cop_caps));if rc==0{vas_reconfig_capabilties(VAS_GZIP_DEF_FEAT_TYPE,be16_to_cpu(hv_cop_caps.target_lpar_creds) as i32)}else{rc}}}
unsafe extern "C" fn pseries_vas_notifier(_nb:*mut notifier_block,_action:usize,_data:*mut core::ffi::c_void)->i32 {pseries_vas_dlpar_cpu()}
static mut pseries_vas_nb:notifier_block=notifier_block{notifier_call:pseries_vas_notifier};
pub unsafe fn vas_migration_handler(action:i32)->i32 {if !copypaste_feat{return 0} migration_in_progress=action==VAS_SUSPEND;for i in 0..VAS_MAX_FEAT_TYPE{let c=&mut vascaps[i].caps;let rc=h_query_vas_capabilities(H_QUERY_VAS_CAPABILITIES,vascaps[i].feat,virt_to_phys(&hv_cop_caps));if rc!=0&&action==VAS_RESUME{return rc}if action==VAS_SUSPEND{mutex_lock(&mut vas_pseries_mutex);/* reconfig_close_windows(vcaps, ..., true) */mutex_unlock(&mut vas_pseries_mutex)}else if action==VAS_RESUME{atomic_set(&mut c.nr_total_credits,be16_to_cpu(hv_cop_caps.target_lpar_creds) as i32)}}0}
unsafe fn pseries_vas_init()->i32 {if !radix_enabled(){return -ENOTSUPP}let h=kmalloc_obj::<hv_vas_all_caps>();if h.is_null(){return -ENOMEM}let rc=h_query_vas_capabilities(H_QUERY_VAS_CAPABILITIES,0,virt_to_phys(h));if rc==0{caps_all.descriptor=be64_to_cpu((*h).descriptor);caps_all.feat_type=be64_to_cpu((*h).feat_type);sysfs_pseries_vas_init(&caps_all)}kfree(h as _);rc}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
