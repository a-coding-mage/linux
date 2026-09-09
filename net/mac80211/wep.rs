// SPDX-License-Identifier: GPL-2.0-only
/*
 * Software WEP encryption implementation
 * Copyright 2002, Jouni Malinen <jkmaline@cc.hut.fi>
 * Copyright 2003, Instant802 Networks, Inc.
 * Copyright (C) 2023 Intel Corporation
 */

// Linux/mac80211 declarations supplied by the surrounding translation unit.

#[repr(C)]
pub struct ieee80211_local {
    pub wep_iv: u32,
    pub wep_tx_ctx: arc4_ctx,
    pub wep_rx_ctx: arc4_ctx,
}
#[repr(C)] pub struct arc4_ctx { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize }
#[repr(C)] pub struct ieee80211_key_conf { pub keyidx: u8, pub keylen: u8, pub key: [u8; 32] }
#[repr(C)] pub struct ieee80211_key { pub conf: ieee80211_key_conf }
#[repr(C)] pub struct ieee80211_hdr { pub frame_control: u16 }

pub const IEEE80211_WEP_IV_LEN: usize = 4;
pub const IEEE80211_WEP_ICV_LEN: usize = 4;
pub const WLAN_KEY_LEN_WEP104: usize = 13;
pub const IEEE80211_FCTL_PROTECTED: u16 = 0x4000;

extern "C" {
    fn get_random_bytes(buf: *mut u8, len: usize);
    fn ieee80211_hdrlen(fc: u16) -> usize;
    fn skb_headroom(skb: *const sk_buff) -> usize;
    fn skb_tailroom(skb: *const sk_buff) -> usize;
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_pull(skb: *mut sk_buff, len: usize);
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_trim(skb: *mut sk_buff, len: usize);
    fn arc4_setkey(ctx: *mut arc4_ctx, key: *const u8, len: usize);
    fn arc4_crypt(ctx: *mut arc4_ctx, dst: *mut u8, src: *const u8, len: usize);
    fn crc32_le(crc: u32, data: *const u8, len: usize) -> u32;
    fn memzero_explicit(ptr: *mut u8, len: usize);
    fn memmove(dst: *mut u8, src: *const u8, len: usize) -> *mut u8;
    fn memcmp(a: *const u8, b: *const u8, len: usize) -> i32;
}

#[inline]
pub unsafe fn ieee80211_wep_init(local: *mut ieee80211_local) {
    // start WEP IV from a random value
    get_random_bytes(&mut (*local).wep_iv as *mut u32 as *mut u8, IEEE80211_WEP_IV_LEN);
}

#[inline]
unsafe fn ieee80211_wep_weak_iv(iv: u32, keylen: i32) -> bool {
    if (iv & 0xff00) == 0xff00 {
        let b = ((iv >> 16) & 0xff) as i32;
        if b >= 3 && b < 3 + keylen { return true; }
    }
    false
}

unsafe fn ieee80211_wep_get_iv(local: *mut ieee80211_local, keylen: i32, keyidx: i32, iv: *mut u8) {
    (*local).wep_iv = (*local).wep_iv.wrapping_add(1);
    if ieee80211_wep_weak_iv((*local).wep_iv, keylen) { (*local).wep_iv = (*local).wep_iv.wrapping_add(0x0100); }
    if iv.is_null() { return; }
    *iv = ((*local).wep_iv >> 16) as u8; let iv = iv.add(1);
    *iv = ((*local).wep_iv >> 8) as u8; let iv = iv.add(1);
    *iv = (*local).wep_iv as u8; *iv.add(1) = (keyidx << 6) as u8;
}

unsafe fn ieee80211_wep_add_iv(local: *mut ieee80211_local, skb: *mut sk_buff, keylen: i32, keyidx: i32) -> *mut u8 {
    let hdr = (*skb).data as *mut ieee80211_hdr;
    (*hdr).frame_control |= IEEE80211_FCTL_PROTECTED.to_le();
    if skb_headroom(skb) < IEEE80211_WEP_IV_LEN { return core::ptr::null_mut(); }
    let hdrlen = ieee80211_hdrlen((*hdr).frame_control);
    let newhdr = skb_push(skb, IEEE80211_WEP_IV_LEN);
    memmove(newhdr, newhdr.add(IEEE80211_WEP_IV_LEN), hdrlen);
    ieee80211_wep_get_iv(local, keylen, keyidx, newhdr.add(hdrlen));
    newhdr.add(hdrlen)
}

unsafe fn ieee80211_wep_remove_iv(_local: *mut ieee80211_local, skb: *mut sk_buff, _key: *mut ieee80211_key) {
    let hdrlen = ieee80211_hdrlen((*((*skb).data as *mut ieee80211_hdr)).frame_control);
    memmove((*skb).data.add(IEEE80211_WEP_IV_LEN), (*skb).data, hdrlen);
    skb_pull(skb, IEEE80211_WEP_IV_LEN);
}

pub unsafe fn ieee80211_wep_encrypt_data(ctx: *mut arc4_ctx, rc4key: *mut u8, klen: usize, data: *mut u8, data_len: usize) -> i32 {
    let icv = (!crc32_le(!0, data, data_len)).to_le_bytes();
    core::ptr::copy_nonoverlapping(icv.as_ptr(), data.add(data_len), 4);
    arc4_setkey(ctx, rc4key, klen); arc4_crypt(ctx, data, data, data_len + IEEE80211_WEP_ICV_LEN);
    memzero_explicit(ctx as *mut u8, core::mem::size_of::<arc4_ctx>()); 0
}

pub unsafe fn ieee80211_wep_encrypt(local: *mut ieee80211_local, skb: *mut sk_buff, key: *const u8, keylen: i32, keyidx: i32) -> i32 {
    if skb_tailroom(skb) < IEEE80211_WEP_ICV_LEN { return -1; }
    let iv = ieee80211_wep_add_iv(local, skb, keylen, keyidx); if iv.is_null() { return -1; }
    let len = (*skb).len - (iv.add(IEEE80211_WEP_IV_LEN) as usize - (*skb).data as usize);
    let mut rc4key = [0u8; 3 + WLAN_KEY_LEN_WEP104];
    core::ptr::copy_nonoverlapping(iv, rc4key.as_mut_ptr(), 3);
    core::ptr::copy_nonoverlapping(key, rc4key.as_mut_ptr().add(3), keylen as usize);
    skb_put(skb, IEEE80211_WEP_ICV_LEN);
    ieee80211_wep_encrypt_data(&mut (*local).wep_tx_ctx, rc4key.as_mut_ptr(), keylen as usize + 3, iv.add(IEEE80211_WEP_IV_LEN), len)
}

pub unsafe fn ieee80211_wep_decrypt_data(ctx: *mut arc4_ctx, rc4key: *mut u8, klen: usize, data: *mut u8, data_len: usize) -> i32 {
    arc4_setkey(ctx, rc4key, klen); arc4_crypt(ctx, data, data, data_len + IEEE80211_WEP_ICV_LEN);
    memzero_explicit(ctx as *mut u8, core::mem::size_of::<arc4_ctx>());
    let crc = (!crc32_le(!0, data, data_len)).to_le_bytes();
    if memcmp(crc.as_ptr(), data.add(data_len), IEEE80211_WEP_ICV_LEN) != 0 { return -1; } 0
}

unsafe fn ieee80211_wep_decrypt(local: *mut ieee80211_local, skb: *mut sk_buff, key: *mut ieee80211_key) -> i32 {
    let hdr = (*skb).data as *mut ieee80211_hdr; if ((*hdr).frame_control & IEEE80211_FCTL_PROTECTED) == 0 { return -1; }
    let hdrlen = ieee80211_hdrlen((*hdr).frame_control);
    if (*skb).len < hdrlen + IEEE80211_WEP_IV_LEN + IEEE80211_WEP_ICV_LEN { return -1; }
    let len = (*skb).len - hdrlen - IEEE80211_WEP_IV_LEN - IEEE80211_WEP_ICV_LEN;
    let keyidx = *(*skb).data.add(hdrlen + 3) >> 6;
    if key.is_null() || keyidx != (*key).conf.keyidx { return -1; }
    let klen = 3 + (*key).conf.keylen as usize; let mut rc4key = [0u8; 3 + WLAN_KEY_LEN_WEP104];
    core::ptr::copy_nonoverlapping((*skb).data.add(hdrlen), rc4key.as_mut_ptr(), 3);
    core::ptr::copy_nonoverlapping((*key).conf.key.as_ptr(), rc4key.as_mut_ptr().add(3), (*key).conf.keylen as usize);
    let ret = ieee80211_wep_decrypt_data(&mut (*local).wep_rx_ctx, rc4key.as_mut_ptr(), klen, (*skb).data.add(hdrlen + IEEE80211_WEP_IV_LEN), len);
    skb_trim(skb, (*skb).len - IEEE80211_WEP_ICV_LEN);
    memmove((*skb).data.add(IEEE80211_WEP_IV_LEN), (*skb).data, hdrlen); skb_pull(skb, IEEE80211_WEP_IV_LEN); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
