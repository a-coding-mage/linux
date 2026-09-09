// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 Instituto Nokia de Tecnologia
 *
 * Authors:
 *    Lauro Ramos Venancio <lauro.venancio@openbossa.org>
 *    Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
 */

// Linux kernel headers and "nfc.h" provide the types, constants, globals,
// macros, and external functions referenced below.

pub const VERSION: &str = "0.1";
pub const NFC_CHECK_PRES_FREQ_MS: u32 = 2000;

pub static mut nfc_devlist_generation: i32 = 0;
// DEFINE_MUTEX(nfc_devlist_mutex);
// static DEFINE_IDA(nfc_index_ida);

pub unsafe fn nfc_fw_download(dev: *mut nfc_dev, firmware_name: *const i8) -> i32 {
    let mut rc: i32 = 0;
    pr_debug!("{} do firmware {}\\n", dev_name(&(*dev).dev), firmware_name);
    device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc = -ENODEV; goto_error!(error); }
    if (*dev).dev_up { rc = -EBUSY; goto_error!(error); }
    if (*(*dev).ops).fw_download.is_none() { rc = -EOPNOTSUPP; goto_error!(error); }
    (*dev).fw_download_in_progress = true;
    rc = ((*(*dev).ops).fw_download.unwrap())(dev, firmware_name);
    if rc != 0 { (*dev).fw_download_in_progress = false; }
error: device_unlock(&mut (*dev).dev); rc
}

pub unsafe fn nfc_fw_download_done(dev: *mut nfc_dev, firmware_name: *const i8, result: u32) -> i32 {
    (*dev).fw_download_in_progress = false;
    nfc_genl_fw_download_done(dev, firmware_name, result)
}

pub unsafe fn nfc_dev_up(dev: *mut nfc_dev) -> i32 {
    let mut rc = 0;
    pr_debug!("dev_name={}\\n", dev_name(&(*dev).dev));
    device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc = -ENODEV; goto_error!(error); }
    if !(*dev).rfkill.is_null() && rfkill_blocked((*dev).rfkill) { rc = -ERFKILL; goto_error!(error); }
    if (*dev).fw_download_in_progress { rc = -EBUSY; goto_error!(error); }
    if (*dev).dev_up { rc = -EALREADY; goto_error!(error); }
    if let Some(f) = (*(*dev).ops).dev_up { rc = f(dev); }
    if rc == 0 { (*dev).dev_up = true; }
    if let Some(f) = (*(*dev).ops).discover_se { if f(dev) != 0 { pr_err!("SE discovery failed\\n"); } }
error: device_unlock(&mut (*dev).dev); rc
}

pub unsafe fn nfc_dev_down(dev: *mut nfc_dev) -> i32 {
    let mut rc = 0;
    pr_debug!("dev_name={}\\n", dev_name(&(*dev).dev));
    device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc = -ENODEV; goto_error!(error); }
    if !(*dev).dev_up { rc = -EALREADY; goto_error!(error); }
    if (*dev).polling || !(*dev).active_target.is_null() { rc = -EBUSY; goto_error!(error); }
    if let Some(f) = (*(*dev).ops).dev_down { f(dev); }
    (*dev).dev_up = false;
error: device_unlock(&mut (*dev).dev); rc
}

unsafe fn nfc_rfkill_set_block(data: *mut core::ffi::c_void, blocked: bool) -> i32 {
    let dev = data as *mut nfc_dev;
    pr_debug!("{} blocked {}", dev_name(&(*dev).dev), blocked);
    if !blocked { return 0; }
    nfc_dev_down(dev); 0
}

// static const struct rfkill_ops nfc_rfkill_ops = { .set_block = nfc_rfkill_set_block };

pub unsafe fn nfc_start_poll(dev: *mut nfc_dev, im_protocols: u32, tm_protocols: u32) -> i32 {
    let mut rc;
    pr_debug!("dev_name {} initiator protocols 0x{:x} target protocols 0x{:x}\\n", dev_name(&(*dev).dev), im_protocols, tm_protocols);
    if im_protocols == 0 && tm_protocols == 0 { return -EINVAL; }
    device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc = -ENODEV; goto_error!(error); }
    if !(*dev).dev_up { rc = -ENODEV; goto_error!(error); }
    if (*dev).polling { rc = -EBUSY; goto_error!(error); }
    rc = ((*(*dev).ops).start_poll.unwrap())(dev, im_protocols, tm_protocols);
    if rc == 0 { (*dev).polling = true; (*dev).rf_mode = NFC_RF_NONE; }
error: device_unlock(&mut (*dev).dev); rc
}

pub unsafe fn nfc_stop_poll(dev: *mut nfc_dev) -> i32 {
    let mut rc = 0;
    pr_debug!("dev_name={}\\n", dev_name(&(*dev).dev));
    device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc = -ENODEV; goto_error!(error); }
    if !(*dev).polling { rc = -EINVAL; goto_error!(error); }
    ((*(*dev).ops).stop_poll.unwrap())(dev); (*dev).polling = false; (*dev).rf_mode = NFC_RF_NONE;
error: device_unlock(&mut (*dev).dev); rc
}

unsafe fn nfc_find_target(dev: *mut nfc_dev, target_idx: u32) -> *mut nfc_target {
    for i in 0..(*dev).n_targets { if (*dev).targets.add(i as usize).as_ref().unwrap().idx == target_idx { return (*dev).targets.add(i as usize); } }
    core::ptr::null_mut()
}

pub unsafe fn nfc_dep_link_up(dev: *mut nfc_dev, target_index: i32, comm_mode: u8) -> i32 {
    let mut rc = 0; let mut gb: *mut u8 = core::ptr::null_mut(); let mut gb_len = 0usize;
    pr_debug!("dev_name={} comm {}\\n", dev_name(&(*dev).dev), comm_mode);
    if (*(*dev).ops).dep_link_up.is_none() { return -EOPNOTSUPP; }
    device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc = -ENODEV; goto_error!(error); }
    if (*dev).dep_link_up { rc = -EALREADY; goto_error!(error); }
    gb = nfc_llcp_general_bytes(dev, &mut gb_len);
    if gb_len > NFC_MAX_GT_LEN as usize { rc = -EINVAL; goto_error!(error); }
    let target = nfc_find_target(dev, target_index as u32); if target.is_null() { rc = -ENOTCONN; goto_error!(error); }
    rc = ((*(*dev).ops).dep_link_up.unwrap())(dev, target, comm_mode, gb, gb_len);
    if rc == 0 { (*dev).active_target = target; (*dev).rf_mode = NFC_RF_INITIATOR; }
error: device_unlock(&mut (*dev).dev); rc
}

pub unsafe fn nfc_dep_link_down(dev: *mut nfc_dev) -> i32 {
    let mut rc = 0; pr_debug!("dev_name={}\\n", dev_name(&(*dev).dev));
    if (*(*dev).ops).dep_link_down.is_none() { return -EOPNOTSUPP; }
    device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc = -ENODEV; goto_error!(error); }
    if !(*dev).dep_link_up { rc = -EALREADY; goto_error!(error); }
    rc = ((*(*dev).ops).dep_link_down.unwrap())(dev);
    if rc == 0 { (*dev).dep_link_up = false; (*dev).active_target = core::ptr::null_mut(); (*dev).rf_mode = NFC_RF_NONE; nfc_llcp_mac_is_down(dev); nfc_genl_dep_link_down_event(dev); }
error: device_unlock(&mut (*dev).dev); rc
}

pub unsafe fn nfc_dep_link_is_up(dev: *mut nfc_dev, target_idx: u32, comm_mode: u8, rf_mode: u8) -> i32 {
    (*dev).dep_link_up = true;
    if (*dev).active_target.is_null() && rf_mode == NFC_RF_INITIATOR { let target = nfc_find_target(dev, target_idx); if target.is_null() { return -ENOTCONN; } (*dev).active_target = target; }
    (*dev).polling = false; (*dev).rf_mode = rf_mode;
    nfc_llcp_mac_is_up(dev, target_idx, comm_mode, rf_mode);
    nfc_genl_dep_link_up_event(dev, target_idx, comm_mode, rf_mode)
}

pub unsafe fn nfc_activate_target(dev: *mut nfc_dev, target_idx: u32, protocol: u32) -> i32 {
    let mut rc; pr_debug!("dev_name={} target_idx={} protocol={}\\n", dev_name(&(*dev).dev), target_idx, protocol); device_lock(&mut (*dev).dev);
    if (*dev).shutting_down { rc=-ENODEV; goto_error!(error); } if !(*dev).active_target.is_null() { rc=-EBUSY; goto_error!(error); }
    let target=nfc_find_target(dev,target_idx); if target.is_null(){rc=-ENOTCONN;goto_error!(error);}
    rc=((*(*dev).ops).activate_target.unwrap())(dev,target,protocol); if rc==0 {(*dev).active_target=target;(*dev).rf_mode=NFC_RF_INITIATOR;}
error: device_unlock(&mut (*dev).dev); rc
}

pub unsafe fn nfc_deactivate_target(dev: *mut nfc_dev, target_idx: u32, mode: u8) -> i32 {
    let mut rc=0; pr_debug!("dev_name={} target_idx={}\\n",dev_name(&(*dev).dev),target_idx); device_lock(&mut (*dev).dev);
    if (*dev).shutting_down {rc=-ENODEV;goto_error!(error);} if (*dev).active_target.is_null(){rc=-ENOTCONN;goto_error!(error);} if (*(*dev).active_target).idx!=target_idx{rc=-ENOTCONN;goto_error!(error);}
    if (*(*dev).ops).check_presence.is_some(){timer_delete_sync(&mut (*dev).check_pres_timer);} ((*(*dev).ops).deactivate_target.unwrap())(dev,(*dev).active_target,mode);(*dev).active_target=core::ptr::null_mut();
error: device_unlock(&mut (*dev).dev);rc
}

pub unsafe fn nfc_data_exchange(dev:*mut nfc_dev,target_idx:u32,skb:*mut sk_buff,cb:data_exchange_cb_t,cb_context:*mut core::ffi::c_void)->i32{
 let mut rc; device_lock(&mut (*dev).dev); if (*dev).shutting_down{rc=-ENODEV;kfree_skb(skb);goto_error!(error);}
 if (*dev).rf_mode==NFC_RF_INITIATOR&&!(*dev).active_target.is_null(){if (*(*dev).active_target).idx!=target_idx{rc=-EADDRNOTAVAIL;kfree_skb(skb);goto_error!(error);} if (*dev).ops.check_presence.is_some(){timer_delete_sync(&mut (*dev).check_pres_timer);} rc=((*(*dev).ops).im_transceive.unwrap())(dev,(*dev).active_target,skb,cb,cb_context);
 }else if (*dev).rf_mode==NFC_RF_TARGET&&(*dev).ops.tm_send.is_some(){rc=((*(*dev).ops).tm_send.unwrap())(dev,skb);}else{rc=-ENOTCONN;kfree_skb(skb);goto_error!(error);}
error:device_unlock(&mut (*dev).dev);rc
}

pub unsafe fn nfc_find_se(dev:*mut nfc_dev,se_idx:u32)->*mut nfc_se{let mut se=(*dev).secure_elements.next;while se!=&mut (*dev).secure_elements as *mut _{let p=container_of_nfc_se(se);if (*p).idx==se_idx{return p;}se=(*se).next;}core::ptr::null_mut()}

pub unsafe fn nfc_enable_se(dev:*mut nfc_dev,se_idx:u32)->i32{let mut rc;device_lock(&mut (*dev).dev);if (*dev).shutting_down{rc=-ENODEV;goto_error!(error);}if !(*dev).dev_up{rc=-ENODEV;goto_error!(error);}if (*dev).polling{rc=-EBUSY;goto_error!(error);}if (*dev).ops.enable_se.is_none()||(*dev).ops.disable_se.is_none(){rc=-EOPNOTSUPP;goto_error!(error);}let se=nfc_find_se(dev,se_idx);if se.is_null(){rc=-EINVAL;goto_error!(error);}if (*se).state==NFC_SE_ENABLED{rc=-EALREADY;goto_error!(error);}rc=((*dev).ops.enable_se.unwrap())(dev,se_idx);if rc>=0{(*se).state=NFC_SE_ENABLED;}error:device_unlock(&mut (*dev).dev);rc}

pub unsafe fn nfc_disable_se(dev:*mut nfc_dev,se_idx:u32)->i32{let mut rc;device_lock(&mut (*dev).dev);if (*dev).shutting_down{rc=-ENODEV;goto_error!(error);}if !(*dev).dev_up{rc=-ENODEV;goto_error!(error);}if (*dev).ops.enable_se.is_none()||(*dev).ops.disable_se.is_none(){rc=-EOPNOTSUPP;goto_error!(error);}let se=nfc_find_se(dev,se_idx);if se.is_null(){rc=-EINVAL;goto_error!(error);}if (*se).state==NFC_SE_DISABLED{rc=-EALREADY;goto_error!(error);}rc=((*dev).ops.disable_se.unwrap())(dev,se_idx);if rc>=0{(*se).state=NFC_SE_DISABLED;}error:device_unlock(&mut (*dev).dev);rc}

pub unsafe fn nfc_set_remote_general_bytes(dev:*mut nfc_dev,gb:*const u8,gb_len:u8)->i32{nfc_llcp_set_remote_gb(dev,gb,gb_len)}
pub unsafe fn nfc_get_local_general_bytes(dev:*mut nfc_dev,gb_len:*mut usize)->*mut u8{nfc_llcp_general_bytes(dev,gb_len)}
pub unsafe fn nfc_tm_data_received(dev:*mut nfc_dev,skb:*mut sk_buff)->i32{if !(*dev).dep_link_up{kfree_skb(skb);return -ENOLINK;}nfc_llcp_data_received(dev,skb)}

pub unsafe fn nfc_tm_activated(dev:*mut nfc_dev,protocol:u32,comm_mode:u8,gb:*const u8,gb_len:usize)->i32{let mut rc;device_lock(&mut (*dev).dev);(*dev).polling=false;if !gb.is_null(){rc=nfc_set_remote_general_bytes(dev,gb,gb_len as u8);if rc<0{goto_out!(out);}}(*dev).rf_mode=NFC_RF_TARGET;if protocol==NFC_PROTO_NFC_DEP_MASK{nfc_dep_link_is_up(dev,0,comm_mode,NFC_RF_TARGET);}rc=nfc_genl_tm_activated(dev,protocol);out:device_unlock(&mut (*dev).dev);rc}
pub unsafe fn nfc_tm_deactivated(dev:*mut nfc_dev)->i32{(*dev).dep_link_up=false;(*dev).rf_mode=NFC_RF_NONE;nfc_genl_tm_deactivated(dev)}

pub unsafe fn nfc_alloc_send_skb(dev:*mut nfc_dev,sk:*mut sock,flags:u32,size:u32,err:*mut u32)->*mut sk_buff{let total=size+(*dev).tx_headroom+(*dev).tx_tailroom+NFC_HEADER_SIZE;sock_alloc_send_skb(sk,total,(flags&MSG_DONTWAIT)!=0,err).map_or(core::ptr::null_mut(),|skb|{skb_reserve(skb,(*dev).tx_headroom+NFC_HEADER_SIZE);skb})}
pub unsafe fn nfc_alloc_recv_skb(size:u32,gfp:gfp_t)->*mut sk_buff{let skb=alloc_skb(size+1,gfp);if !skb.is_null(){skb_reserve(skb,1);}skb}

pub unsafe fn nfc_targets_found(dev:*mut nfc_dev,targets:*mut nfc_target,n_targets:i32)->i32{for i in 0..n_targets{(*targets.add(i as usize)).idx=(*dev).target_next_idx;(*dev).target_next_idx+=1;}device_lock(&mut (*dev).dev);if !(*dev).polling{device_unlock(&mut (*dev).dev);return 0;}(*dev).polling=false;(*dev).targets_generation+=1;kfree((*dev).targets as *mut core::ffi::c_void);(*dev).targets=core::ptr::null_mut();if !targets.is_null(){(*dev).targets=kmemdup(targets,(n_targets as usize)*core::mem::size_of::<nfc_target>(),GFP_ATOMIC);if (*dev).targets.is_null(){(*dev).n_targets=0;device_unlock(&mut (*dev).dev);return -ENOMEM;}}(*dev).n_targets=n_targets;device_unlock(&mut (*dev).dev);nfc_genl_targets_found(dev);0}

pub unsafe fn nfc_target_lost(dev:*mut nfc_dev,target_idx:u32)->i32{device_lock(&mut (*dev).dev);let mut i=0;while i<(*dev).n_targets&&(*(*dev).targets.add(i as usize)).idx!=target_idx{i+=1;}if i==(*dev).n_targets{device_unlock(&mut (*dev).dev);return -EINVAL;}(*dev).targets_generation+=1;(*dev).n_targets-=1;(*dev).active_target=core::ptr::null_mut();if (*dev).n_targets>0{core::ptr::copy((*dev).targets.add(i as usize+1),(*dev).targets.add(i as usize),((*dev).n_targets-i) as usize);}else{kfree((*dev).targets as *mut core::ffi::c_void);(*dev).targets=core::ptr::null_mut();}device_unlock(&mut (*dev).dev);nfc_genl_target_lost(dev,target_idx);0}

pub unsafe fn nfc_driver_failure(dev:*mut nfc_dev,_err:i32){nfc_targets_found(dev,core::ptr::null_mut(),0);}

pub unsafe fn nfc_add_se(dev:*mut nfc_dev,se_idx:u32,typ:u16)->i32{if !nfc_find_se(dev,se_idx).is_null(){return -EALREADY;}let se=kzalloc_obj_nfc_se();if se.is_null(){return -ENOMEM;}(*se).idx=se_idx;(*se).type_=typ;(*se).state=NFC_SE_DISABLED;list_add(&mut (*se).list,&mut (*dev).secure_elements);let rc=nfc_genl_se_added(dev,se_idx,typ);if rc<0{list_del(&mut (*se).list);kfree(se as *mut core::ffi::c_void);}rc}
pub unsafe fn nfc_remove_se(dev:*mut nfc_dev,se_idx:u32)->i32{let se=nfc_find_se(dev,se_idx);if se.is_null(){return -EINVAL;}let rc=nfc_genl_se_removed(dev,se_idx);if rc<0{return rc;}list_del(&mut (*se).list);kfree(se as *mut core::ffi::c_void);0}
pub unsafe fn nfc_se_transaction(dev:*mut nfc_dev,se_idx:u8,evt:*mut nfc_evt_transaction)->i32{device_lock(&mut (*dev).dev);let rc=if evt.is_null(){-EPROTO}else{nfc_genl_se_transaction(dev,se_idx,evt)};device_unlock(&mut (*dev).dev);rc}
pub unsafe fn nfc_se_connectivity(dev:*mut nfc_dev,se_idx:u8)->i32{device_lock(&mut (*dev).dev);let rc=nfc_genl_se_connectivity(dev,se_idx);device_unlock(&mut (*dev).dev);rc}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
