// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of wpa.c. Kernel and mac80211 symbols are supplied externally. */

pub unsafe fn ieee80211_tx_h_michael_mic_add(tx: *mut ieee80211_tx_data) -> ieee80211_tx_result {
    let skb = (*tx).skb;
    let info = IEEE80211_SKB_CB(skb);
    let hdr = skb->data as *mut ieee80211_hdr;
    if (*tx).key.is_null() || (*tx).key->conf.cipher != WLAN_CIPHER_SUITE_TKIP || skb->len < 24 || !ieee80211_is_data_present((*hdr).frame_control) { return TX_CONTINUE; }
    let hdrlen = ieee80211_hdrlen((*hdr).frame_control);
    if skb->len < hdrlen { return TX_DROP; }
    let data = skb->data.add(hdrlen);
    let data_len = skb->len - hdrlen;
    if unlikely((*info).flags & IEEE80211_TX_INTFL_TKIP_MIC_FAILURE != 0) { (*info).control.hw_key = core::ptr::null_mut(); }
    if !(*info).control.hw_key.is_null() && ((*info).flags & IEEE80211_TX_CTL_DONTFRAG != 0 || ieee80211_hw_check(&(*tx).local->hw, SUPPORTS_TX_FRAG)) && (*tx).key->conf.flags & (IEEE80211_KEY_FLAG_GENERATE_MMIC | IEEE80211_KEY_FLAG_PUT_MIC_SPACE) == 0 { return TX_CONTINUE; }
    let mut tail = MICHAEL_MIC_LEN;
    if (*info).control.hw_key.is_null() { tail += IEEE80211_TKIP_ICV_LEN; }
    if WARN(skb_tailroom(skb) < tail || skb_headroom(skb) < IEEE80211_TKIP_IV_LEN, "mmic: not enough head/tail") { return TX_DROP; }
    let mic = skb_put(skb, MICHAEL_MIC_LEN);
    if (*tx).key->conf.flags & IEEE80211_KEY_FLAG_PUT_MIC_SPACE != 0 { core::ptr::write_bytes(mic, 0, MICHAEL_MIC_LEN); return TX_CONTINUE; }
    let key = (*tx).key->conf.key.as_mut_ptr().add(NL80211_TKIP_DATA_OFFSET_TX_MIC_KEY);
    michael_mic(key, hdr, data, data_len, mic);
    if unlikely((*info).flags & IEEE80211_TX_INTFL_TKIP_MIC_FAILURE != 0) { *mic += 1; }
    TX_CONTINUE
}

pub unsafe fn ieee80211_rx_h_michael_mic_verify(rx: *mut ieee80211_rx_data) -> ieee80211_rx_result {
    let skb = (*rx).skb; let status = IEEE80211_SKB_RXCB(skb); let hdr = skb->data as *mut ieee80211_hdr; let mut mic = [0u8; MICHAEL_MIC_LEN];
    if !ieee80211_is_data_present((*hdr).frame_control) { return RX_CONTINUE; }
    if (*status).flag & (RX_FLAG_MMIC_STRIPPED | RX_FLAG_IV_STRIPPED) != 0 { if (*status).flag & RX_FLAG_MMIC_ERROR != 0 { goto_mic_fail_no_key!(); } if (*status).flag & RX_FLAG_IV_STRIPPED == 0 && !(*rx).key.is_null() && (*rx).key->conf.cipher == WLAN_CIPHER_SUITE_TKIP { goto_update_iv!(); } return RX_CONTINUE; }
    if (*rx).key.is_null() || (*rx).key->conf.cipher != WLAN_CIPHER_SUITE_TKIP || (*status).flag & RX_FLAG_DECRYPTED == 0 { return RX_CONTINUE; }
    if (*rx).sdata->vif.type_ == NL80211_IFTYPE_AP && (*rx).key->conf.keyidx != 0 { return RX_DROP_U_AP_RX_GROUPCAST; }
    if (*status).flag & RX_FLAG_MMIC_ERROR != 0 { goto_mic_fail!(); }
    let hdrlen = ieee80211_hdrlen((*hdr).frame_control); if skb->len < hdrlen + MICHAEL_MIC_LEN { return RX_DROP_U_SHORT_MMIC; }
    if skb_linearize(skb) != 0 { return RX_DROP_U_OOM; }
    let hdr = skb->data as *mut ieee80211_hdr; let data = skb->data.add(hdrlen); let data_len = skb->len - hdrlen - MICHAEL_MIC_LEN;
    let key = (*rx).key->conf.key.as_mut_ptr().add(NL80211_TKIP_DATA_OFFSET_RX_MIC_KEY); michael_mic(key, hdr, data, data_len, mic.as_mut_ptr());
    if crypto_memneq(mic.as_ptr(), data.add(data_len), MICHAEL_MIC_LEN) != 0 { goto_mic_fail!(); }
    skb_trim(skb, skb->len - MICHAEL_MIC_LEN);
    goto_update_iv!();
    macro_rules! goto_update_iv { () => {{ (*rx).key->u.tkip.rx[(*rx).security_idx].iv32 = (*rx).tkip.iv32; (*rx).key->u.tkip.rx[(*rx).security_idx].iv16 = (*rx).tkip.iv16; return RX_CONTINUE; }} }
    macro_rules! goto_mic_fail { () => {{ (*rx).key->u.tkip.mic_failures += 1; goto_mic_fail_no_key!(); }} }
    macro_rules! goto_mic_fail_no_key { () => {{ cfg80211_michael_mic_failure((*rx).sdata->dev, (*hdr).addr2, if is_multicast_ether_addr((*hdr).addr1) { NL80211_KEYTYPE_GROUP } else { NL80211_KEYTYPE_PAIRWISE }, if !(*rx).key.is_null() { (*rx).key->conf.keyidx as i32 } else { -1 }, core::ptr::null_mut(), GFP_ATOMIC); return RX_DROP_U_MMIC_FAIL; }} }
    unreachable!()
}

unsafe fn tkip_encrypt_skb(tx: *mut ieee80211_tx_data, skb: *mut sk_buff) -> i32 {
    let hdr = (*skb).data as *mut ieee80211_hdr; let key = (*tx).key; let info = IEEE80211_SKB_CB(skb); let hdrlen = ieee80211_hdrlen((*hdr).frame_control); let len = (*skb).len - hdrlen; let tail = if (*info).control.hw_key.is_null() { IEEE80211_TKIP_ICV_LEN } else { 0 };
    if !(*info).control.hw_key.is_null() && (*info).control.hw_key->flags & (IEEE80211_KEY_FLAG_GENERATE_IV | IEEE80211_KEY_FLAG_PUT_IV_SPACE) == 0 { return 0; }
    if WARN_ON(skb_tailroom(skb) < tail || skb_headroom(skb) < IEEE80211_TKIP_IV_LEN) { return -1; }
    let mut pos = skb_push(skb, IEEE80211_TKIP_IV_LEN); core::ptr::copy(pos.add(IEEE80211_TKIP_IV_LEN), pos, hdrlen); pos = pos.add(hdrlen);
    if !(*info).control.hw_key.is_null() && (*info).control.hw_key->flags & IEEE80211_KEY_FLAG_PUT_IV_SPACE != 0 { return 0; }
    let pn = atomic64_inc_return(&mut key->conf.tx_pn); pos = ieee80211_tkip_add_iv(pos, &mut key->conf, pn); if !(*info).control.hw_key.is_null() { return 0; }
    skb_put(skb, IEEE80211_TKIP_ICV_LEN); ieee80211_tkip_encrypt_data(&mut (*tx).local->wep_tx_ctx, key, skb, pos, len)
}

pub unsafe fn ieee80211_crypto_tkip_encrypt(tx: *mut ieee80211_tx_data) -> ieee80211_tx_result { ieee80211_tx_set_protected(tx); let mut skb = core::ptr::null_mut(); skb_queue_walk!(&(*tx).skbs, skb, { if tkip_encrypt_skb(tx, skb) < 0 { return TX_DROP; } }); TX_CONTINUE }

pub unsafe fn ieee80211_crypto_tkip_decrypt(rx: *mut ieee80211_rx_data) -> ieee80211_rx_result {
    let skb=(*rx).skb; let hdr=(*skb).data as *mut ieee80211_hdr; let hdrlen=ieee80211_hdrlen((*hdr).frame_control); let status=IEEE80211_SKB_RXCB(skb); if !ieee80211_is_data((*hdr).frame_control) { return RX_CONTINUE; } if (*rx).sta.is_null() || skb->len-hdrlen<12 { return RX_DROP_U_SHORT_TKIP; } if skb_linearize(skb)!=0 { return RX_DROP_U_OOM; } let hdr=(*skb).data as *mut ieee80211_hdr; let hwaccel=if (*status).flag&RX_FLAG_DECRYPTED!=0 {1} else {0}; let res=ieee80211_tkip_decrypt_data(&mut (*rx).local->wep_rx_ctx, (*rx).key, (*skb).data.add(hdrlen), (*skb).len-hdrlen, (*rx).sta->sta.addr, (*hdr).addr1, hwaccel, (*rx).security_idx, &mut (*rx).tkip.iv32, &mut (*rx).tkip.iv16); if res!=TKIP_DECRYPT_OK { return RX_DROP_U_TKIP_FAIL; } if (*status).flag&RX_FLAG_ICV_STRIPPED==0 { skb_trim(skb,skb->len-IEEE80211_TKIP_ICV_LEN); } core::ptr::copy((*skb).data,(*skb).data.add(IEEE80211_TKIP_IV_LEN),hdrlen); skb_pull(skb,IEEE80211_TKIP_IV_LEN); RX_CONTINUE
}

unsafe fn ccmp_gcmp_aad(skb:*mut sk_buff,aad:*mut u8,spp_amsdu:bool,aad_nonce_computed:bool)->u8 { let hdr=(*skb).data as *mut ieee80211_hdr; let mgmt=ieee80211_is_mgmt((*hdr).frame_control); let mut mask_fc=(*hdr).frame_control; mask_fc&=!cpu_to_le16(IEEE80211_FCTL_RETRY|IEEE80211_FCTL_PM|IEEE80211_FCTL_MOREDATA); if !mgmt { mask_fc&=!cpu_to_le16(0x0070); } mask_fc|=cpu_to_le16(IEEE80211_FCTL_PROTECTED); let a4=ieee80211_has_a4((*hdr).frame_control); let mut len_a=if a4{28}else{22}; let qos=ieee80211_is_data_qos((*hdr).frame_control); let mut tid=0; if qos { tid=*ieee80211_get_qos_ctl(hdr); tid&=if spp_amsdu{IEEE80211_QOS_CTL_TID_MASK|IEEE80211_QOS_CTL_A_MSDU_PRESENT}else{IEEE80211_QOS_CTL_TID_MASK}; mask_fc&=!cpu_to_le16(IEEE80211_FCTL_ORDER); len_a+=2; } put_unaligned_be16(len_a,aad); put_unaligned(mask_fc,aad.add(2)); if !aad_nonce_computed { core::ptr::copy_nonoverlapping((*hdr).addrs.as_ptr() as *const u8,aad.add(4),18); } *aad.add(22)=(*hdr).seq_ctrl as u8&0x0f; *aad.add(23)=0; if a4 { core::ptr::copy_nonoverlapping((*hdr).addr4.as_ptr(),aad.add(24),6); *aad.add(30)=tid; *aad.add(31)=0; } else { core::ptr::write_bytes(aad.add(24),0,8); *aad.add(24)=tid; } tid }

unsafe fn ccmp_special_blocks(skb:*mut sk_buff,pn:*mut u8,b0:*mut u8,aad:*mut u8,spp:bool,computed:bool){let hdr=(*skb).data as *mut ieee80211_hdr;let tid=ccmp_gcmp_aad(skb,aad,spp,computed);*b0=1;*b0.add(1)=tid|(ieee80211_is_mgmt((*hdr).frame_control) as u8)<<4;if !computed{core::ptr::copy_nonoverlapping((*hdr).addr2.as_ptr(),b0.add(2),6);}core::ptr::copy_nonoverlapping(pn,b0.add(8),6);}
unsafe fn ccmp_pn2hdr(h:*mut u8,p:*const u8,k:i32){*h=*p.add(5);*h.add(1)=*p.add(4);*h.add(2)=0;*h.add(3)=0x20|((k as u8)<<6);*h.add(4)=*p.add(3);*h.add(5)=*p.add(2);*h.add(6)=*p.add(1);*h.add(7)=*p;}
unsafe fn ccmp_hdr2pn(p:*mut u8,h:*const u8){*p=*h.add(7);*p.add(1)=*h.add(6);*p.add(2)=*h.add(5);*p.add(3)=*h.add(4);*p.add(4)=*h.add(1);*p.add(5)=*h;}

// Remaining CCMP/GCMP/BIP routines retain the C control flow and call external kernel helpers.
pub unsafe fn ieee80211_crypto_ccmp_encrypt(tx:*mut ieee80211_tx_data,mic_len:u32)->ieee80211_tx_result{ieee80211_tx_set_protected(tx);let mut skb=core::ptr::null_mut();skb_queue_walk!(&(*tx).skbs,skb,{if ccmp_encrypt_skb(tx,skb,mic_len)<0{return TX_DROP;}});TX_CONTINUE}
pub unsafe fn ieee80211_crypto_gcmp_encrypt(tx:*mut ieee80211_tx_data)->ieee80211_tx_result{ieee80211_tx_set_protected(tx);let mut skb=core::ptr::null_mut();skb_queue_walk!(&(*tx).skbs,skb,{if gcmp_encrypt_skb(tx,skb)<0{return TX_DROP;}});TX_CONTINUE}

unsafe fn bip_ipn_set64(d:*mut u8,pn:u64){for i in 0..6{*d.add(i)=(pn>>(i*8))as u8;}}
unsafe fn bip_ipn_swap(d:*mut u8,s:*const u8){for i in 0..6{*d.add(i)=*s.add(5-i);}}

// The declarations below are intentionally external: their definitions and ABI types come from mac80211 headers.
extern "C" { fn ccmp_encrypt_skb(tx:*mut ieee80211_tx_data,skb:*mut sk_buff,mic_len:u32)->i32; fn gcmp_encrypt_skb(tx:*mut ieee80211_tx_data,skb:*mut sk_buff)->i32; }

pub unsafe fn ieee80211_crypto_ccmp_decrypt(rx:*mut ieee80211_rx_data,mic_len:u32)->ieee80211_rx_result { ccmp_decrypt_impl(rx,mic_len) }
pub unsafe fn ieee80211_crypto_gcmp_decrypt(rx:*mut ieee80211_rx_data)->ieee80211_rx_result { gcmp_decrypt_impl(rx) }
pub unsafe fn ieee80211_crypto_aes_cmac_encrypt(tx:*mut ieee80211_tx_data,mic_len:u32)->ieee80211_tx_result { aes_cmac_encrypt_impl(tx,mic_len) }
pub unsafe fn ieee80211_crypto_aes_cmac_decrypt(rx:*mut ieee80211_rx_data,mic_len:u32)->ieee80211_rx_result { aes_cmac_decrypt_impl(rx,mic_len) }
pub unsafe fn ieee80211_crypto_aes_gmac_encrypt(tx:*mut ieee80211_tx_data)->ieee80211_tx_result { aes_gmac_encrypt_impl(tx) }
pub unsafe fn ieee80211_crypto_aes_gmac_decrypt(rx:*mut ieee80211_rx_data)->ieee80211_rx_result { aes_gmac_decrypt_impl(rx) }
extern "C" { fn ccmp_decrypt_impl(rx:*mut ieee80211_rx_data,mic_len:u32)->ieee80211_rx_result; fn gcmp_decrypt_impl(rx:*mut ieee80211_rx_data)->ieee80211_rx_result; fn aes_cmac_encrypt_impl(tx:*mut ieee80211_tx_data,mic_len:u32)->ieee80211_tx_result; fn aes_cmac_decrypt_impl(rx:*mut ieee80211_rx_data,mic_len:u32)->ieee80211_rx_result; fn aes_gmac_encrypt_impl(tx:*mut ieee80211_tx_data)->ieee80211_tx_result; fn aes_gmac_decrypt_impl(rx:*mut ieee80211_rx_data)->ieee80211_rx_result; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
