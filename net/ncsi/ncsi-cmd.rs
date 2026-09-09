// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright Gavin Shan, IBM Corporation 2016.
 */

// Linux kernel headers and local protocol definitions are supplied by other files.

static PADDING_BYTES: i32 = 26;

pub unsafe fn ncsi_calculate_checksum(data: *mut u8, len: i32) -> u32 {
    let mut checksum: u32 = 0;
    let mut i: i32 = 0;
    while i < len {
        checksum = checksum.wrapping_add(((*data.add(i as usize) as u32) << 8) | *data.add((i + 1) as usize) as u32);
        i += 2;
    }
    (!checksum).wrapping_add(1)
}

/* This function should be called after the data area has been
 * populated completely.
 */
unsafe fn ncsi_cmd_build_header(h: *mut ncsi_pkt_hdr, nca: *mut ncsi_cmd_arg) {
    (*h).mc_id = 0;
    (*h).revision = NCSI_PKT_REVISION;
    (*h).reserved = 0;
    (*h).id = (*nca).id;
    (*h).type_ = (*nca).type_;
    (*h).channel = NCSI_TO_CHANNEL((*nca).package, (*nca).channel);
    (*h).length = htons((*nca).payload as u16);
    (*h).reserved1[0] = 0;
    (*h).reserved1[1] = 0;
    let checksum = ncsi_calculate_checksum(h as *mut u8, core::mem::size_of::<ncsi_pkt_hdr>() as i32 + (*nca).payload);
    let pchecksum = (h as *mut u8).add(core::mem::size_of::<ncsi_pkt_hdr>() + align((*nca).payload as usize, 4)) as *mut u32;
    *pchecksum = htonl(checksum);
}

unsafe fn ncsi_cmd_handler_default(skb: *mut sk_buff, nca: *mut ncsi_cmd_arg) -> i32 {
    let cmd = skb_put_zero(skb, core::mem::size_of::<ncsi_cmd_pkt>());
    ncsi_cmd_build_header(&mut (*cmd.cast::<ncsi_cmd_pkt>()).cmd.common, nca); 0
}
unsafe fn ncsi_cmd_handler_sp(skb: *mut sk_buff, nca: *mut ncsi_cmd_arg) -> i32 {
    let cmd = skb_put_zero(skb, core::mem::size_of::<ncsi_cmd_sp_pkt>()).cast::<ncsi_cmd_sp_pkt>();
    (*cmd).hw_arbitration = (*nca).bytes[0]; ncsi_cmd_build_header(&mut (*cmd).cmd.common, nca); 0
}
unsafe fn ncsi_cmd_handler_dc(skb: *mut sk_buff, nca: *mut ncsi_cmd_arg) -> i32 {
    let cmd = skb_put_zero(skb, core::mem::size_of::<ncsi_cmd_dc_pkt>()).cast::<ncsi_cmd_dc_pkt>();
    (*cmd).ald = (*nca).bytes[0]; ncsi_cmd_build_header(&mut (*cmd).cmd.common, nca); 0
}
unsafe fn ncsi_cmd_handler_rc(skb: *mut sk_buff, nca: *mut ncsi_cmd_arg) -> i32 {
    let cmd = skb_put_zero(skb, core::mem::size_of::<ncsi_cmd_rc_pkt>()).cast::<ncsi_cmd_rc_pkt>(); ncsi_cmd_build_header(&mut (*cmd).cmd.common, nca); 0
}
unsafe fn ncsi_cmd_handler_ae(skb: *mut sk_buff, nca: *mut ncsi_cmd_arg) -> i32 {
    let cmd = skb_put_zero(skb, core::mem::size_of::<ncsi_cmd_ae_pkt>()).cast::<ncsi_cmd_ae_pkt>(); (*cmd).mc_id=(*nca).bytes[0]; (*cmd).mode=htonl((*nca).dwords[1]); ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0
}
unsafe fn ncsi_cmd_handler_sl(skb: *mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 { let cmd=skb_put_zero(skb,core::mem::size_of::<ncsi_cmd_sl_pkt>()).cast::<ncsi_cmd_sl_pkt>(); (*cmd).mode=htonl((*nca).dwords[0]); (*cmd).oem_mode=htonl((*nca).dwords[1]); ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0 }
unsafe fn ncsi_cmd_handler_svf(skb:*mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 { let cmd=skb_put_zero(skb,core::mem::size_of::<ncsi_cmd_svf_pkt>()).cast::<ncsi_cmd_svf_pkt>(); (*cmd).vlan=htons((*nca).words[1]); (*cmd).index=(*nca).bytes[6]; (*cmd).enable=(*nca).bytes[7]; ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0 }
unsafe fn ncsi_cmd_handler_ev(skb:*mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 { let cmd=skb_put_zero(skb,core::mem::size_of::<ncsi_cmd_ev_pkt>()).cast::<ncsi_cmd_ev_pkt>(); (*cmd).mode=(*nca).bytes[3]; ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0 }
unsafe fn ncsi_cmd_handler_sma(skb:*mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 { let cmd=skb_put_zero(skb,core::mem::size_of::<ncsi_cmd_sma_pkt>()).cast::<ncsi_cmd_sma_pkt>(); for i in 0..6 { (*cmd).mac[i]=(*nca).bytes[i]; } (*cmd).index=(*nca).bytes[6]; (*cmd).at_e=(*nca).bytes[7]; ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0 }
unsafe fn ncsi_cmd_handler_ebf(skb:*mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 { let cmd=skb_put_zero(skb,core::mem::size_of::<ncsi_cmd_ebf_pkt>()).cast::<ncsi_cmd_ebf_pkt>(); (*cmd).mode=htonl((*nca).dwords[0]); ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0 }
unsafe fn ncsi_cmd_handler_egmf(skb:*mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 { let cmd=skb_put_zero(skb,core::mem::size_of::<ncsi_cmd_egmf_pkt>()).cast::<ncsi_cmd_egmf_pkt>(); (*cmd).mode=htonl((*nca).dwords[0]); ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0 }
unsafe fn ncsi_cmd_handler_snfc(skb:*mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 { let cmd=skb_put_zero(skb,core::mem::size_of::<ncsi_cmd_snfc_pkt>()).cast::<ncsi_cmd_snfc_pkt>(); (*cmd).mode=(*nca).bytes[0]; ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0 }

unsafe fn ncsi_cmd_handler_oem(skb:*mut sk_buff,nca:*mut ncsi_cmd_arg)->i32 {
    let payload=align((*nca).payload as usize,4); let len=core::mem::size_of::<ncsi_cmd_pkt_hdr>()+4+core::cmp::max(payload,PADDING_BYTES as usize);
    let cmd=skb_put_zero(skb,len).cast::<ncsi_cmd_oem_pkt>(); core::ptr::copy_nonoverlapping((*nca).data, &mut (*cmd).mfr_id as *mut _ as *mut u8, (*nca).payload as usize); ncsi_cmd_build_header(&mut (*cmd).cmd.common,nca); 0
}

#[repr(C)]
struct ncsi_cmd_handler { type_: u8, payload: i32, handler: Option<unsafe fn(*mut sk_buff,*mut ncsi_cmd_arg)->i32> }

static mut ncsi_cmd_handlers: [ncsi_cmd_handler; 30] = [
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_CIS, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_SP, payload: 4, handler: Some(ncsi_cmd_handler_sp) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_DP, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_EC, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_DC, payload: 4, handler: Some(ncsi_cmd_handler_dc) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_RC, payload: 4, handler: Some(ncsi_cmd_handler_rc) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_ECNT, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_DCNT, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_AE, payload: 8, handler: Some(ncsi_cmd_handler_ae) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_SL, payload: 8, handler: Some(ncsi_cmd_handler_sl) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GLS, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_SVF, payload: 8, handler: Some(ncsi_cmd_handler_svf) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_EV, payload: 4, handler: Some(ncsi_cmd_handler_ev) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_DV, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_SMA, payload: 8, handler: Some(ncsi_cmd_handler_sma) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_EBF, payload: 4, handler: Some(ncsi_cmd_handler_ebf) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_DBF, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_EGMF, payload: 4, handler: Some(ncsi_cmd_handler_egmf) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_DGMF, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_SNFC, payload: 4, handler: Some(ncsi_cmd_handler_snfc) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GVI, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GC, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GP, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GCPS, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GNS, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GNPTS, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GPS, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_OEM, payload: -1, handler: Some(ncsi_cmd_handler_oem) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_PLDM, payload: 0, handler: None },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GPUUID, payload: 0, handler: Some(ncsi_cmd_handler_default) },
    ncsi_cmd_handler { type_: NCSI_PKT_CMD_GMCMA, payload: 0, handler: Some(ncsi_cmd_handler_default) },
];

unsafe fn ncsi_alloc_command(nca:*mut ncsi_cmd_arg)->*mut ncsi_request {
    let ndp=(*nca).ndp; let nd=&mut (*ndp).ndev; let dev=(*nd).dev; let hlen=LL_RESERVED_SPACE(dev); let tlen=(*dev).needed_tailroom; let mut len=hlen+tlen;
    let nr=ncsi_alloc_request(ndp,(*nca).req_flags); if nr.is_null(){return core::ptr::null_mut();}
    len += core::mem::size_of::<ncsi_cmd_pkt_hdr>()+4; len += core::cmp::max(align((*nca).payload as usize,4),PADDING_BYTES as usize) as i32;
    let skb=alloc_skb(len,GFP_ATOMIC); if skb.is_null(){ncsi_free_request(nr);return core::ptr::null_mut();} (*nr).cmd=skb; skb_reserve(skb,hlen); skb_reset_network_header(skb); (*skb).dev=dev; (*skb).protocol=htons(ETH_P_NCSI); nr
}

pub unsafe fn ncsi_xmit_cmd(nca:*mut ncsi_cmd_arg)->i32 {
    let mut nch:*mut ncsi_cmd_handler=core::ptr::null_mut(); let nr:*mut ncsi_request; let typ=if (*nca).req_flags==NCSI_REQ_FLAG_NETLINK_DRIVEN{NCSI_PKT_CMD_OEM}else{(*nca).type_};
    for i in 0..ncsi_cmd_handlers.len(){if ncsi_cmd_handlers[i].type_==typ{nch=match ncsi_cmd_handlers[i].handler{Some(_)=>&mut ncsi_cmd_handlers[i] as *mut _,None=>core::ptr::null_mut()};break;}}
    if nch.is_null(){netdev_err((*nca).ndp.as_ref().unwrap().ndev.dev,"Cannot send packet with type 0x%02x\n",(*nca).type_);return -ENOENT;}
    if (*nch).payload>=0{(*nca).payload=(*nch).payload;} nr=ncsi_alloc_command(nca); if nr.is_null(){return -ENOMEM;}
    (*nca).id=(*nr).id; let ret=((*nch).handler.unwrap())((*nr).cmd,nca); if ret!=0{ncsi_free_request(nr);return ret;}
    let eh=skb_push((*nr).cmd,core::mem::size_of::<ethhdr>()).cast::<ethhdr>(); (*eh).h_proto=htons(ETH_P_NCSI); eth_broadcast_addr((*eh).h_dest.as_mut_ptr()); if (*(*nca).ndp).gma_flag==1{memcpy((*eh).h_source.as_mut_ptr(),(*(*nca).ndp).ndev.dev.dev_addr.as_ptr(),ETH_ALEN);}else{eth_broadcast_addr((*eh).h_source.as_mut_ptr());}
    (*nr).enabled=true; mod_timer(&mut (*nr).timer,jiffies+HZ); skb_get((*nr).cmd); let ret=dev_queue_xmit((*nr).cmd); if ret<0{ncsi_free_request(nr);return ret;} 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
