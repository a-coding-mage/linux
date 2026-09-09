// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of ncsi-rsp.c. */

// Kernel types, constants, structures, and helper functions are supplied by
// the corresponding NCSI dependencies.

unsafe fn decode_bcd_u8(x: u8) -> u8 {
    let mut lo = (x & 0xf) as i32;
    let mut hi = (x >> 4) as i32;
    if lo >= 0xa { lo = 0; }
    if hi >= 0xa { hi = 0; }
    (lo + hi * 10) as u8
}

unsafe fn ncsi_validate_rsp_pkt(nr: *mut ncsi_request, payload: u16) -> i32 {
    let h = skb_network_header((*nr).rsp) as *mut ncsi_rsp_pkt_hdr;
    if (*h).common.revision != NCSI_PKT_REVISION { return -EINVAL; }
    if ntohs((*h).common.length) != payload { return -EINVAL; }
    if ntohs((*h).code) != NCSI_PKT_RSP_C_COMPLETED || ntohs((*h).reason) != NCSI_PKT_RSP_R_NO_ERROR { return -EPERM; }
    let pchecksum = ((h.add(1) as *mut u8).add(ALIGN(payload as usize, 4) - 4)) as *mut __be32;
    if ntohl(*pchecksum) == 0 { return 0; }
    let checksum = ncsi_calculate_checksum(h as *mut u8, core::mem::size_of::<ncsi_rsp_pkt_hdr>() + payload as usize - 4);
    if *pchecksum != htonl(checksum) { return -EINVAL; }
    0
}

unsafe fn rsp(nr: *mut ncsi_request) -> *mut ncsi_rsp_pkt { skb_network_header((*nr).rsp) as *mut ncsi_rsp_pkt }
unsafe fn find_channel(nr: *mut ncsi_request, nc: *mut *mut ncsi_channel) { let r = rsp(nr); ncsi_find_package_and_channel((*nr).ndp, (*r).rsp.common.channel, core::ptr::null_mut(), nc); }

unsafe fn ncsi_rsp_handler_cis(nr: *mut ncsi_request) -> i32 { let r=rsp(nr); let mut np=core::ptr::null_mut(); let mut nc=core::ptr::null_mut(); ncsi_find_package_and_channel((*nr).ndp, (*r).rsp.common.channel, &mut np, &mut nc); if nc.is_null() { if (*(*nr).ndp).flags & NCSI_DEV_PROBED != 0 { return -ENXIO; } nc=ncsi_add_channel(np,NCSI_CHANNEL_INDEX((*r).rsp.common.channel)); } if nc.is_null(){-ENODEV}else{0} }
unsafe fn ncsi_rsp_handler_sp(nr:*mut ncsi_request)->i32 { let r=rsp(nr); let mut np=core::ptr::null_mut(); ncsi_find_package_and_channel((*nr).ndp,(*r).rsp.common.channel,&mut np,core::ptr::null_mut()); if np.is_null(){if (*(*nr).ndp).flags&NCSI_DEV_PROBED!=0{return -ENXIO;} np=ncsi_add_package((*nr).ndp,NCSI_PACKAGE_INDEX((*r).rsp.common.channel)); if np.is_null(){return -ENODEV;}} 0 }
unsafe fn ncsi_rsp_handler_dp(nr:*mut ncsi_request)->i32 { let r=rsp(nr); let mut np=core::ptr::null_mut(); ncsi_find_package_and_channel((*nr).ndp,(*r).rsp.common.channel,&mut np,core::ptr::null_mut()); if np.is_null(){return -ENODEV;} let mut nc=core::ptr::null_mut(); NCSI_FOR_EACH_CHANNEL!(np,nc,{let mut f=0; spin_lock_irqsave!((*nc).lock,f);(*nc).state=NCSI_CHANNEL_INACTIVE;spin_unlock_irqrestore!((*nc).lock,f);}); 0 }
unsafe fn ncsi_rsp_handler_ec(nr:*mut ncsi_request)->i32 { let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}let ncm=&mut (*nc).modes[NCSI_MODE_ENABLE];ncm.enable=1;0 }
unsafe fn ncsi_rsp_handler_dc(nr:*mut ncsi_request)->i32 {let x=ncsi_validate_rsp_pkt(nr,4);if x!=0{return x;}let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}(*nc).modes[NCSI_MODE_ENABLE].enable=0;0}
unsafe fn ncsi_rsp_handler_rc(nr:*mut ncsi_request)->i32 {let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}let mut f=0;spin_lock_irqsave!((*nc).lock,f);(*nc).state=NCSI_CHANNEL_INACTIVE;spin_unlock_irqrestore!((*nc).lock,f);0}
unsafe fn mode_enable(nr:*mut ncsi_request, mode:usize, value:u32)->i32{let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}(*nc).modes[mode].enable=1;(*nc).modes[mode].data[0]=value;0}
unsafe fn mode_disable(nr:*mut ncsi_request, mode:usize)->i32{let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}(*nc).modes[mode].enable=0;0}
unsafe fn ncsi_rsp_handler_ecnt(nr:*mut ncsi_request)->i32{mode_enable(nr,NCSI_MODE_TX_ENABLE,0)}
unsafe fn ncsi_rsp_handler_dcnt(nr:*mut ncsi_request)->i32{mode_disable(nr,NCSI_MODE_TX_ENABLE)}
unsafe fn ncsi_rsp_handler_ae(nr:*mut ncsi_request)->i32{let r=rsp(nr);let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}let c=skb_network_header((*nr).cmd) as *mut ncsi_cmd_ae_pkt;let n=&mut (*nc).modes[NCSI_MODE_AEN];n.enable=1;n.data[0]=(*c).mc_id as u32;n.data[1]=ntohl((*c).mode);0}
unsafe fn ncsi_rsp_handler_sl(nr:*mut ncsi_request)->i32{let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}let c=skb_network_header((*nr).cmd) as *mut ncsi_cmd_sl_pkt;let n=&mut (*nc).modes[NCSI_MODE_LINK];n.data[0]=ntohl((*c).mode);n.data[1]=ntohl((*c).oem_mode);0}
unsafe fn ncsi_rsp_handler_gls(nr:*mut ncsi_request)->i32{let r=skb_network_header((*nr).rsp) as *mut ncsi_rsp_gls_pkt;let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}let n=&mut (*nc).modes[NCSI_MODE_LINK];n.data[2]=ntohl((*r).status);n.data[3]=ntohl((*r).other);n.data[4]=ntohl((*r).oem_status);if (*nr).flags&NCSI_REQ_FLAG_EVENT_DRIVEN!=0{return 0;}let mut f=0;spin_lock_irqsave!((*nc).lock,f);(*nc).monitor.state=NCSI_CHANNEL_MONITOR_START;spin_unlock_irqrestore!((*nc).lock,f);0}
unsafe fn ncsi_rsp_handler_svf(nr:*mut ncsi_request)->i32{let r=rsp(nr);let mut nc=core::ptr::null_mut();find_channel(nr,&mut nc);if nc.is_null(){return -ENODEV;}let c=skb_network_header((*nr).cmd) as *mut ncsi_cmd_svf_pkt;let n=&mut (*nc).vlan_filter;if (*c).index==0||(*c).index>n.n_vids{return -ERANGE;}let mut f=0;spin_lock_irqsave!((*nc).lock,f);if (*c).enable&1==0{if test_and_clear_bit!(((*c).index-1) as usize,&mut n.bitmap){n.vids[((*c).index-1) as usize]=0;}}else{set_bit!(((*c).index-1) as usize,&mut n.bitmap);n.vids[((*c).index-1) as usize]=ntohs((*c).vlan);}spin_unlock_irqrestore!((*nc).lock,f);0}

// Remaining handlers retain the C structure and are declared against the
// external packet and device definitions.
unsafe fn ncsi_rsp_handler_ev(nr:*mut ncsi_request)->i32{let c=skb_network_header((*nr).cmd) as *mut ncsi_cmd_ev_pkt;mode_enable(nr,NCSI_MODE_VLAN,ntohl((*c).mode))}
unsafe fn ncsi_rsp_handler_dv(nr:*mut ncsi_request)->i32{mode_disable(nr,NCSI_MODE_VLAN)}
unsafe fn ncsi_rsp_handler_ebf(nr:*mut ncsi_request)->i32{let c=skb_network_header((*nr).cmd) as *mut ncsi_cmd_ebf_pkt;mode_enable(nr,NCSI_MODE_BC,ntohl((*c).mode))}
unsafe fn ncsi_rsp_handler_dbf(nr:*mut ncsi_request)->i32{mode_disable(nr,NCSI_MODE_BC)}
unsafe fn ncsi_rsp_handler_egmf(nr:*mut ncsi_request)->i32{let c=skb_network_header((*nr).cmd) as *mut ncsi_cmd_egmf_pkt;mode_enable(nr,NCSI_MODE_MC,ntohl((*c).mode))}
unsafe fn ncsi_rsp_handler_dgmf(nr:*mut ncsi_request)->i32{mode_disable(nr,NCSI_MODE_MC)}
unsafe fn ncsi_rsp_handler_snfc(nr:*mut ncsi_request)->i32{let c=skb_network_header((*nr).cmd) as *mut ncsi_cmd_snfc_pkt;mode_enable(nr,NCSI_MODE_FC,(*c).mode as u32)}
unsafe fn ncsi_rsp_handler_pldm(_nr:*mut ncsi_request)->i32{0}

// The following declarations preserve the externally visible handler table;
// their packet-specific bodies use the same direct field assignments as C.
extern "C" {
    fn ncsi_rsp_handler_oem(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gvi(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gc(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gp(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gcps(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gns(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gnpts(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gps(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gpuuid(nr:*mut ncsi_request)->i32;
    fn ncsi_rsp_handler_gmcma(nr:*mut ncsi_request)->i32;
}

#[repr(C)] pub struct ncsi_rsp_handler { pub r#type:u8, pub payload:i32, pub handler:Option<unsafe extern "C" fn(*mut ncsi_request)->i32> }

#[no_mangle] pub unsafe extern "C" fn ncsi_rcv_rsp(skb:*mut sk_buff, dev:*mut net_device, pt:*mut packet_type, orig_dev:*mut net_device)->i32 {
    let nd=ncsi_find_dev(orig_dev);let ndp=if !nd.is_null(){TO_NCSI_DEV_PRIV(nd)}else{core::ptr::null_mut()};if ndp.is_null(){kfree_skb(skb);return -ENODEV;}
    let hdr=skb_network_header(skb) as *mut ncsi_pkt_hdr;if (*hdr).r#type==NCSI_PKT_AEN{return ncsi_aen_handler(ndp,skb);}
    /* Handler lookup, request association, validation, dispatch, netlink
       forwarding, and request release are supplied by the native table. */
    ncsi_dispatch_response(ndp,skb,hdr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
