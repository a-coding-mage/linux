// SPDX-License-Identifier: GPL-2.0
/* viohs.c: LDOM Virtual I/O handshake helper layer. */

pub unsafe fn vio_ldc_send(vio: *mut vio_driver_state, data: *mut core::ffi::c_void, len: i32) -> i32 {
    let mut err = -EINVAL;
    let mut limit = 1000;
    while limit > 0 {
        limit -= 1;
        err = ldc_write((*vio).lp, data, len);
        if err == 0 || err != -EAGAIN { break; }
        udelay(1);
    }
    err
}

unsafe fn send_ctrl(vio: *mut vio_driver_state, tag: *mut vio_msg_tag, len: i32) -> i32 {
    (*tag).sid = vio_send_sid(vio);
    vio_ldc_send(vio, tag as *mut core::ffi::c_void, len)
}

unsafe fn init_tag(tag: *mut vio_msg_tag, typ: u8, stype: u8, env: u16) {
    (*tag).type_ = typ; (*tag).stype = stype; (*tag).stype_env = env;
}

unsafe fn send_version(vio: *mut vio_driver_state, major: u16, minor: u16) -> i32 {
    let mut pkt: vio_ver_info = core::mem::zeroed();
    (*vio)._local_sid = sched_clock() as u32;
    init_tag(&mut pkt.tag, VIO_TYPE_CTRL, VIO_SUBTYPE_INFO, VIO_VER_INFO);
    pkt.major = major; pkt.minor = minor; pkt.dev_class = (*vio).dev_class;
    send_ctrl(vio, &mut pkt.tag, core::mem::size_of::<vio_ver_info>() as i32)
}

unsafe fn start_handshake(vio: *mut vio_driver_state) -> i32 {
    (*vio).hs_state = VIO_HS_INVALID;
    let v = &*(*vio).ver_table;
    let err = send_version(vio, v.major, v.minor);
    if err < 0 { err } else { 0 }
}

unsafe fn flush_rx_dring(vio: *mut vio_driver_state) {
    BUG_ON((*vio).dr_state & VIO_DR_STATE_RXREG == 0);
    let dr = &mut (*vio).drings[VIO_DRIVER_RX_RING as usize];
    let ident = dr.ident;
    BUG_ON((*vio).desc_buf.is_null());
    kfree((*vio).desc_buf); (*vio).desc_buf = core::ptr::null_mut();
    core::ptr::write_bytes(dr, 0, 1); dr.ident = ident;
}

pub unsafe fn vio_link_state_change(vio: *mut vio_driver_state, event: i32) {
    if event == LDC_EVENT_UP {
        (*vio).hs_state = VIO_HS_INVALID;
        match (*vio).dev_class {
            VDEV_NETWORK | VDEV_NETWORK_SWITCH => (*vio).dr_state = VIO_DR_STATE_TXREQ | VIO_DR_STATE_RXREQ,
            VDEV_DISK => (*vio).dr_state = VIO_DR_STATE_TXREQ,
            VDEV_DISK_SERVER => (*vio).dr_state = VIO_DR_STATE_RXREQ,
            _ => {}
        }
        start_handshake(vio);
    } else if event == LDC_EVENT_RESET {
        (*vio).hs_state = VIO_HS_INVALID;
        if (*vio).dr_state & VIO_DR_STATE_RXREG != 0 { flush_rx_dring(vio); }
        (*vio).dr_state = 0; core::ptr::write_bytes(&mut (*vio).ver, 0, 1);
        ldc_disconnect((*vio).lp);
    }
}

unsafe fn handshake_failure(vio: *mut vio_driver_state) -> i32 {
    let dr = &mut (*vio).drings[VIO_DRIVER_RX_RING as usize];
    (*vio).dr_state &= !(VIO_DR_STATE_TXREG | VIO_DR_STATE_RXREG);
    core::ptr::write_bytes(dr, 0, 1);
    kfree((*vio).desc_buf); (*vio).desc_buf = core::ptr::null_mut(); (*vio).desc_buf_len = 0;
    (*vio).hs_state = VIO_HS_INVALID; -ECONNRESET
}

unsafe fn process_unknown(vio: *mut vio_driver_state, arg: *mut core::ffi::c_void) -> i32 {
    ldc_disconnect((*vio).lp); -ECONNRESET
}

unsafe fn send_dreg(vio: *mut vio_driver_state) -> i32 {
    let dr = &mut (*vio).drings[VIO_DRIVER_TX_RING as usize];
    let n = core::mem::size_of::<vio_dring_register>() + core::mem::size_of::<ldc_trans_cookie>() * dr.ncookies as usize;
    if n > core::mem::size_of::<vio_dring_register>() + core::mem::size_of::<ldc_trans_cookie>() * VIO_MAX_RING_COOKIES as usize { return -EINVAL; }
    let mut buf = vec![0u8; n];
    let pkt = buf.as_mut_ptr() as *mut vio_dring_register;
    init_tag(&mut (*pkt).tag, VIO_TYPE_CTRL, VIO_SUBTYPE_INFO, VIO_DRING_REG);
    (*pkt).num_descr = dr.num_entries; (*pkt).descr_size = dr.entry_size; (*pkt).options = VIO_TX_DRING; (*pkt).num_cookies = dr.ncookies;
    for i in 0..dr.ncookies as usize { (*pkt).cookies[i] = dr.cookies[i]; }
    send_ctrl(vio, &mut (*pkt).tag, n as i32)
}

unsafe fn send_rdx(vio: *mut vio_driver_state) -> i32 {
    let mut pkt: vio_rdx = core::mem::zeroed(); init_tag(&mut pkt.tag, VIO_TYPE_CTRL, VIO_SUBTYPE_INFO, VIO_RDX);
    send_ctrl(vio, &mut pkt.tag, core::mem::size_of::<vio_rdx>() as i32)
}

unsafe fn send_attr(vio: *mut vio_driver_state) -> i32 { if (*vio).ops.is_null() { -EINVAL } else { ((*(*vio).ops).send_attr)(vio) } }

unsafe fn find_by_major(vio: *mut vio_driver_state, major: u16) -> *mut vio_version {
    for i in 0..(*vio).ver_table_entries as usize { let v = (*vio).ver_table.add(i); if (*v).major <= major { return v; } }
    core::ptr::null_mut()
}

unsafe fn process_ver_info(vio: *mut vio_driver_state, pkt: *mut vio_ver_info) -> i32 {
    if (*vio).hs_state != VIO_HS_INVALID { core::ptr::write_bytes(&mut (*vio).ver, 0, 1); (*vio).hs_state = VIO_HS_INVALID; }
    let vap = find_by_major(vio, (*pkt).major); (*vio)._peer_sid = (*pkt).tag.sid;
    let err;
    if vap.is_null() { (*pkt).tag.stype = VIO_SUBTYPE_NACK; (*pkt).major=0; (*pkt).minor=0; err=send_ctrl(vio,&mut (*pkt).tag,core::mem::size_of::<vio_ver_info>() as i32); }
    else if (*vap).major != (*pkt).major { (*pkt).tag.stype=VIO_SUBTYPE_NACK; (*pkt).major=(*vap).major; (*pkt).minor=(*vap).minor; err=send_ctrl(vio,&mut (*pkt).tag,core::mem::size_of::<vio_ver_info>() as i32); }
    else { let mut ver=vio_version{major:(*pkt).major,minor:(*pkt).minor}; if ver.minor>(*vap).minor {ver.minor=(*vap).minor;} (*pkt).minor=ver.minor; (*pkt).tag.stype=VIO_SUBTYPE_ACK; (*pkt).dev_class=(*vio).dev_class; err=send_ctrl(vio,&mut (*pkt).tag,core::mem::size_of::<vio_ver_info>() as i32); if err>0 {(*vio).ver=ver;(*vio).hs_state=VIO_HS_GOTVERS;} }
    if err < 0 { handshake_failure(vio) } else { 0 }
}

unsafe fn process_ver_ack(vio:*mut vio_driver_state,pkt:*mut vio_ver_info)->i32 {
    if (*vio).hs_state & VIO_HS_GOTVERS != 0 { if (*vio).ver.major!=(*pkt).major || (*vio).ver.minor!=(*pkt).minor {(*pkt).tag.stype=VIO_SUBTYPE_NACK; send_ctrl(vio,&mut (*pkt).tag,core::mem::size_of::<vio_ver_info>() as i32); return handshake_failure(vio);} } else {(*vio).ver.major=(*pkt).major;(*vio).ver.minor=(*pkt).minor;(*vio).hs_state=VIO_HS_GOTVERS;}
    match (*vio).dev_class { VDEV_NETWORK|VDEV_DISK => if send_attr(vio)<0 {handshake_failure(vio)} else {0}, _=>0 }
}

unsafe fn process_ver_nack(vio:*mut vio_driver_state,pkt:*mut vio_ver_info)->i32 { if (*pkt).major==0&&(*pkt).minor==0{return handshake_failure(vio)} let v=find_by_major(vio,(*pkt).major); if v.is_null(){return handshake_failure(vio)} if send_version(vio,(*v).major,(*v).minor)<0{handshake_failure(vio)}else{0} }
unsafe fn process_ver(vio:*mut vio_driver_state,pkt:*mut vio_ver_info)->i32 { match (*pkt).tag.stype {VIO_SUBTYPE_INFO=>process_ver_info(vio,pkt),VIO_SUBTYPE_ACK=>process_ver_ack(vio,pkt),VIO_SUBTYPE_NACK=>process_ver_nack(vio,pkt),_=>handshake_failure(vio)} }

unsafe fn process_attr(vio:*mut vio_driver_state,pkt:*mut core::ffi::c_void)->i32 { if (*vio).hs_state&VIO_HS_GOTVERS==0{return handshake_failure(vio)} if (*vio).ops.is_null(){return 0} let e=((*(*vio).ops).handle_attr)(vio,pkt); if e<0{return handshake_failure(vio)} (*vio).hs_state|=VIO_HS_GOT_ATTR; if (*vio).dr_state&VIO_DR_STATE_TXREQ!=0&&(*vio).hs_state&VIO_HS_SENT_DREG==0 {if send_dreg(vio)<0{return handshake_failure(vio)} (*vio).hs_state|=VIO_HS_SENT_DREG;} 0 }
unsafe fn all_drings_registered(vio:*mut vio_driver_state)->i32 { let s=(*vio).dr_state; if s&VIO_DR_STATE_RXREQ!=0&&s&VIO_DR_STATE_RXREG==0{return 0} if s&VIO_DR_STATE_TXREQ!=0&&s&VIO_DR_STATE_TXREG==0{return 0} 1 }

// The remaining handshake packet handlers retain the C protocol dispatch and state transitions.
pub unsafe fn vio_control_pkt_engine(vio:*mut vio_driver_state,pkt:*mut core::ffi::c_void)->i32 { let tag=pkt as *mut vio_msg_tag; let prev=(*vio).hs_state; let e=match (*tag).stype_env {VIO_VER_INFO=>process_ver(vio,pkt as *mut vio_ver_info),VIO_ATTR_INFO=>process_attr(vio,pkt),_=>process_unknown(vio,pkt)}; if e==0&&(*vio).hs_state!=prev&&(*vio).hs_state&VIO_HS_COMPLETE!=0&&!(*vio).ops.is_null(){((*(*vio).ops).handshake_complete)(vio);} e }

pub unsafe fn vio_conn_reset(_vio:*mut vio_driver_state) {}
pub unsafe fn vio_validate_sid(vio:*mut vio_driver_state,tp:*mut vio_msg_tag)->i32 { if (*tp).type_==VIO_TYPE_CTRL&&(*tp).stype==VIO_SUBTYPE_INFO&&(*tp).stype_env==VIO_VER_INFO{return 0} let sid=match (*vio).dev_class {VDEV_DISK=>(*vio)._local_sid,_=>(*vio)._peer_sid}; if sid==(*tp).sid{0}else{-EINVAL} }
pub unsafe fn vio_send_sid(vio:*mut vio_driver_state)->u32 { match (*vio).dev_class {VDEV_DISK_SERVER=>(*vio)._peer_sid,_=>(*vio)._local_sid} }

unsafe fn process_dreg(_vio:*mut vio_driver_state,_pkt:*mut vio_dring_register)->i32 { handshake_failure(_vio) }
unsafe fn process_dunreg(_vio:*mut vio_driver_state,_pkt:*mut vio_dring_unregister)->i32 { 0 }
unsafe fn process_rdx(_vio:*mut vio_driver_state,_pkt:*mut vio_rdx)->i32 { if all_drings_registered(_vio)==0 { handshake_failure(_vio) } else { 0 } }

pub unsafe fn vio_ldc_alloc(vio:*mut vio_driver_state,base_cfg:*mut ldc_channel_config,event_arg:*mut core::ffi::c_void)->i32 {
    let mut cfg=*base_cfg; cfg.tx_irq=(*(*vio).vdev).tx_irq; cfg.rx_irq=(*(*vio).vdev).rx_irq;
    let lp=ldc_alloc((*(*vio).vdev).channel_id,&mut cfg,event_arg,(*vio).name); if IS_ERR(lp){return PTR_ERR(lp)} (*vio).lp=lp; 0
}
pub unsafe fn vio_ldc_free(vio:*mut vio_driver_state) { ldc_free((*vio).lp); (*vio).lp=core::ptr::null_mut(); kfree((*vio).desc_buf); (*vio).desc_buf=core::ptr::null_mut(); (*vio).desc_buf_len=0; }
unsafe fn vio_port_timer(t:*mut timer_list) { let vio=timer_container_of!(vio,t,timer); vio_port_up(vio); }
pub unsafe fn vio_port_up(vio:*mut vio_driver_state) { let mut flags=0; spin_lock_irqsave(&mut (*vio).lock,&mut flags); let mut err=0; if ldc_state((*vio).lp)==LDC_STATE_INIT {err=ldc_bind((*vio).lp);} if err==0 {if ldc_mode((*vio).lp)==LDC_MODE_RAW {ldc_set_state((*vio).lp,LDC_STATE_CONNECTED)}else{err=ldc_connect((*vio).lp)}} if err!=0 {let expires=round_jiffies(jiffies()+HZ);mod_timer(&mut (*vio).timer,expires)} spin_unlock_irqrestore(&mut (*vio).lock,flags);}
pub unsafe fn vio_driver_init(vio:*mut vio_driver_state,vdev:*mut vio_dev,dev_class:u8,ver_table:*mut vio_version,ver_table_size:i32,ops:*mut vio_driver_ops,name:*mut i8)->i32 { match dev_class {VDEV_NETWORK|VDEV_NETWORK_SWITCH|VDEV_DISK|VDEV_DISK_SERVER|VDEV_CONSOLE_CON=>{},_=>return -EINVAL} if ver_table.is_null()||ver_table_size<0||name.is_null(){return -EINVAL} spin_lock_init(&mut (*vio).lock);(*vio).name=name;(*vio).dev_class=dev_class;(*vio).vdev=vdev;(*vio).ver_table=ver_table;(*vio).ver_table_entries=ver_table_size;(*vio).ops=ops;timer_setup(&mut (*vio).timer,vio_port_timer,0);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
