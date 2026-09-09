// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2002-2004, Instant802 Networks, Inc.
 * Copyright 2005, Devicescape Software, Inc.
 * Copyright (C) 2016 Intel Deutschland GmbH
 */
// Linux/mac80211 dependencies are supplied by the surrounding translation unit.

const PHASE1_LOOP_COUNT: i32 = 8;

/*
 * 2-byte by 2-byte subset of the full AES S-box table; second part of this
 * table is identical to first part but byte-swapped
 */
static TKIP_SBOX: [u16; 256] = [
    0xC6A5,0xF884,0xEE99,0xF68D,0xFF0D,0xD6BD,0xDEB1,0x9154,0x6050,0x0203,0xCEA9,0x567D,0xE719,0xB562,0x4DE6,0xEC9A,
    0x8F45,0x1F9D,0x8940,0xFA87,0xEF15,0xB2EB,0x8EC9,0xFB0B,0x41EC,0xB367,0x5FFD,0x45EA,0x23BF,0x53F7,0xE496,0x9B5B,
    0x75C2,0xE11C,0x3DAE,0x4C6A,0x6C5A,0x7E41,0xF502,0x834F,0x685C,0x51F4,0xD134,0xF908,0xE293,0xAB73,0x6253,0x2A3F,
    0x080C,0x9552,0x4665,0x9D5E,0x3028,0x37A1,0x0A0F,0x2FB5,0x0E09,0x2436,0x1B9B,0xDF3D,0xCD26,0x4E69,0x7FCD,0xEA9F,
    0x121B,0x1D9E,0x5874,0x342E,0x362D,0xDCB2,0xB4EE,0x5BFB,0xA4F6,0x764D,0xB761,0x7DCE,0x527B,0xDD3E,0x5E71,0x1397,
    0xA6F5,0xB968,0x0000,0xC12C,0x4060,0xE31F,0x79C8,0xB6ED,0xD4BE,0x8D46,0x67D9,0x724B,0x94DE,0x98D4,0xB0E8,0x854A,
    0xBB6B,0xC52A,0x4FE5,0xED16,0x86C5,0x9AD7,0x6655,0x1194,0x8ACF,0xE910,0x0406,0xFE81,0xA0F0,0x7844,0x25BA,0x4BE3,
    0xA2F3,0x5DFE,0x80C0,0x058A,0x3FAD,0x21BC,0x7048,0xF104,0x63DF,0x77C1,0xAF75,0x4263,0x2030,0xE51A,0xFD0E,0xBF6D,
    0x814C,0x1814,0x2635,0xC32F,0xBEE1,0x35A2,0x88CC,0x2E39,0x9357,0x55F2,0xFC82,0x7A47,0xC8AC,0xBAE7,0x322B,0xE695,
    0xC0A0,0x1998,0x9ED1,0xA37F,0x4466,0x547E,0x3BAB,0x0B83,0x8CCA,0xC729,0x6BD3,0x283C,0xA779,0xBCE2,0x161D,0xAD76,
    0xDB3B,0x6456,0x744E,0x141E,0x92DB,0x0C0A,0x486C,0xB8E4,0x9F5D,0xBD6E,0x43EF,0xC4A6,0x39A8,0x31A4,0xD337,0xF28B,
    0xD532,0x8B43,0x6E59,0xDAB7,0x018C,0xB164,0x9CD2,0x49E0,0xD8B4,0xACFA,0xF307,0xCF25,0xCAAF,0xF48E,0x47E9,0x1018,
    0x6FD5,0xF088,0x4A6F,0x5C72,0x3824,0x57F1,0x73C7,0x9751,0xCB23,0xA17C,0xE89C,0x3E21,0x96DD,0x61DC,0x0D86,0x0F85,
    0xE090,0x7C42,0x71C4,0xCCAA,0x90D8,0x0605,0xF701,0x1C12,0xC2A3,0x6A5F,0xAEF9,0x69D0,0x1791,0x9958,0x3A27,0x27B9,
    0xD938,0xEB13,0x2BB3,0x2233,0xD2BB,0xA970,0x0789,0x33A7,0x2DB6,0x3C22,0x1592,0xC920,0x8749,0xAAFF,0x5078,0xA57A,
    0x038F,0x59F8,0x0980,0x1A17,0x65DA,0xD731,0x84C6,0xD0B8,0x82C3,0x29B0,0x5A77,0x1E11,0x7BCB,0xA8FC,0x6DD6,0x2C3A,
];

unsafe fn tkip_s(val: u16) -> u16 { TKIP_SBOX[(val & 0xff) as usize] ^ swab16(TKIP_SBOX[(val >> 8) as usize]) }

unsafe fn write_tkip_iv(mut pos: *mut u8, iv16: u16) -> *mut u8 {
    *pos = (iv16 >> 8) as u8; pos = pos.add(1);
    *pos = (((iv16 >> 8) | 0x20) & 0x7f) as u8; pos = pos.add(1);
    *pos = iv16 as u8; pos.add(1)
}

unsafe fn tkip_mixing_phase1(tk: *const u8, ctx: *mut tkip_ctx, ta: *const u8, tsc_iv32: u32) {
    let p1k = (*ctx).p1k.as_mut_ptr();
    *p1k.add(0) = tsc_iv32 as u16; *p1k.add(1) = (tsc_iv32 >> 16) as u16;
    *p1k.add(2) = get_unaligned_le16(ta); *p1k.add(3) = get_unaligned_le16(ta.add(2));
    *p1k.add(4) = get_unaligned_le16(ta.add(4));
    for i in 0..PHASE1_LOOP_COUNT { let j = 2 * (i & 1);
        *p1k.add(0) = (*p1k.add(0)).wrapping_add(tkip_s(*p1k.add(4) ^ get_unaligned_le16(tk.add(j as usize))));
        *p1k.add(1) = (*p1k.add(1)).wrapping_add(tkip_s(*p1k.add(0) ^ get_unaligned_le16(tk.add((4+j) as usize))));
        *p1k.add(2) = (*p1k.add(2)).wrapping_add(tkip_s(*p1k.add(1) ^ get_unaligned_le16(tk.add((8+j) as usize))));
        *p1k.add(3) = (*p1k.add(3)).wrapping_add(tkip_s(*p1k.add(2) ^ get_unaligned_le16(tk.add((12+j) as usize))));
        *p1k.add(4) = (*p1k.add(4)).wrapping_add(tkip_s(*p1k.add(3) ^ get_unaligned_le16(tk.add(j as usize))).wrapping_add(i as u16));
    }
    (*ctx).state = TKIP_STATE_PHASE1_DONE; (*ctx).p1k_iv32 = tsc_iv32;
}

unsafe fn tkip_mixing_phase2(tk: *const u8, ctx: *const tkip_ctx, tsc_iv16: u16, rc4key: *mut u8) {
    let p = (*ctx).p1k; let mut ppk = [p[0],p[1],p[2],p[3],p[4],p[4].wrapping_add(tsc_iv16)];
    ppk[0]=ppk[0].wrapping_add(tkip_s(ppk[5]^get_unaligned_le16(tk))); ppk[1]=ppk[1].wrapping_add(tkip_s(ppk[0]^get_unaligned_le16(tk.add(2))));
    ppk[2]=ppk[2].wrapping_add(tkip_s(ppk[1]^get_unaligned_le16(tk.add(4)))); ppk[3]=ppk[3].wrapping_add(tkip_s(ppk[2]^get_unaligned_le16(tk.add(6))));
    ppk[4]=ppk[4].wrapping_add(tkip_s(ppk[3]^get_unaligned_le16(tk.add(8)))); ppk[5]=ppk[5].wrapping_add(tkip_s(ppk[4]^get_unaligned_le16(tk.add(10))));
    ppk[0]=ppk[0].wrapping_add(ror16(ppk[5]^get_unaligned_le16(tk.add(12)),1)); ppk[1]=ppk[1].wrapping_add(ror16(ppk[0]^get_unaligned_le16(tk.add(14)),1));
    ppk[2]=ppk[2].wrapping_add(ror16(ppk[1],1)); ppk[3]=ppk[3].wrapping_add(ror16(ppk[2],1)); ppk[4]=ppk[4].wrapping_add(ror16(ppk[3],1)); ppk[5]=ppk[5].wrapping_add(ror16(ppk[4],1));
    let mut out=write_tkip_iv(rc4key,tsc_iv16); *out=(((ppk[5]^get_unaligned_le16(tk))>>1)&0xff) as u8; out=out.add(1);
    for i in 0..6 { put_unaligned_le16(ppk[i],out.add(2*i)); }
}

pub unsafe fn ieee80211_tkip_add_iv(mut pos:*mut u8,keyconf:*mut ieee80211_key_conf,pn:u64)->*mut u8 { pos=write_tkip_iv(pos,TKIP_PN_TO_IV16(pn)); *pos=((*keyconf).keyidx<<6)|(1<<5); pos=pos.add(1); put_unaligned_le32(TKIP_PN_TO_IV32(pn),pos); pos.add(4) }

unsafe fn ieee80211_compute_tkip_p1k(key:*mut ieee80211_key,iv32:u32) { let sdata=(*key).sdata; let ctx=&mut (*key).u.tkip.tx; let tk=(*key).conf.key.as_ptr().add(NL80211_TKIP_DATA_OFFSET_ENCR_KEY as usize); lockdep_assert_held(&(*key).u.tkip.txlock); if ctx.p1k_iv32!=iv32 || ctx.state==TKIP_STATE_NOT_INIT { tkip_mixing_phase1(tk,ctx,(*sdata).vif.addr.as_ptr(),iv32); } }

pub unsafe fn ieee80211_get_tkip_p1k_iv(k:*mut ieee80211_key_conf,iv32:u32,p1k:*mut u16) { let key=container_of!(k,ieee80211_key,conf); spin_lock_bh(&mut (*key).u.tkip.txlock); ieee80211_compute_tkip_p1k(key,iv32); memcpy(p1k,(*key).u.tkip.tx.p1k.as_ptr(),core::mem::size_of_val(&(*key).u.tkip.tx.p1k)); spin_unlock_bh(&mut (*key).u.tkip.txlock); }

pub unsafe fn ieee80211_get_tkip_rx_p1k(k:*mut ieee80211_key_conf,ta:*const u8,iv32:u32,p1k:*mut u16) { let tk=(*k).key.as_ptr().add(NL80211_TKIP_DATA_OFFSET_ENCR_KEY as usize); let mut ctx:tkip_ctx=core::mem::zeroed(); tkip_mixing_phase1(tk,&mut ctx,ta,iv32); memcpy(p1k,ctx.p1k.as_ptr(),core::mem::size_of_val(&ctx.p1k)); }

pub unsafe fn ieee80211_get_tkip_p2k(k:*mut ieee80211_key_conf,skb:*mut sk_buff,p2k:*mut u8) { let key=container_of!(k,ieee80211_key,conf); let tk=(*key).conf.key.as_ptr().add(NL80211_TKIP_DATA_OFFSET_ENCR_KEY as usize); let ctx=&mut (*key).u.tkip.tx; let hdr=(*skb).data as *mut ieee80211_hdr; let data=(hdr as *mut u8).add(ieee80211_hdrlen((*hdr).frame_control) as usize); let iv32=get_unaligned_le32(data.add(4)); let iv16=*data.add(2) as u16 | ((*data as u16)<<8); spin_lock(&mut (*key).u.tkip.txlock); ieee80211_compute_tkip_p1k(key,iv32); tkip_mixing_phase2(tk,ctx,iv16,p2k); spin_unlock(&mut (*key).u.tkip.txlock); }

pub unsafe fn ieee80211_tkip_encrypt_data(ctx:*mut arc4_ctx,key:*mut ieee80211_key,skb:*mut sk_buff,payload:*mut u8,payload_len:usize)->i32 { let mut rc4key=[0u8;16]; ieee80211_get_tkip_p2k(&mut (*key).conf,skb,rc4key.as_mut_ptr()); ieee80211_wep_encrypt_data(ctx,rc4key.as_mut_ptr(),16,payload,payload_len) }

pub unsafe fn ieee80211_tkip_decrypt_data(ctx:*mut arc4_ctx,key:*mut ieee80211_key,skb:*mut sk_buff,payload:*mut u8,payload_len:usize,ta:*mut u8,ra:*mut u8,only_iv:i32,queue:i32,out_iv32:*mut u32,out_iv16:*mut u16)->i32 {
    let mut rc4key=[0u8;16]; let pos=payload; let tk=(*key).conf.key.as_ptr().add(NL80211_TKIP_DATA_OFFSET_ENCR_KEY as usize); let rx_ctx=&mut (*key).u.tkip.rx[queue as usize]; if payload_len<12{return -1;}
    let iv16=((*pos.add(0) as u16)<<8)|*pos.add(2) as u16; let keyid=*pos.add(3); let iv32=get_unaligned_le32(pos.add(4)); let pos=pos.add(8);
    if keyid&(1<<5)==0{return TKIP_DECRYPT_NO_EXT_IV;} if (keyid>>6)!=(*key).conf.keyidx{return TKIP_DECRYPT_INVALID_KEYIDX;}
    if iv32<rx_ctx.iv32 || (iv32==rx_ctx.iv32 && (iv16<rx_ctx.iv16 || (iv16==rx_ctx.iv16 && (rx_ctx.iv32!=0 || rx_ctx.iv16!=0 || rx_ctx.ctx.state!=TKIP_STATE_NOT_INIT)))) {return TKIP_DECRYPT_REPLAY;}
    let res=if only_iv!=0 {rx_ctx.ctx.state=TKIP_STATE_PHASE1_HW_UPLOADED;TKIP_DECRYPT_OK} else {
        if rx_ctx.ctx.state==TKIP_STATE_NOT_INIT || rx_ctx.iv32!=iv32 { /* IV16 wrapped around - perform TKIP phase 1 */ tkip_mixing_phase1(tk,&mut rx_ctx.ctx,ta,iv32); }
        if (*key).local.ops.update_tkip_key.is_some() && ((*key).flags & KEY_FLAG_UPLOADED_TO_HARDWARE)!=0 && rx_ctx.ctx.state!=TKIP_STATE_PHASE1_HW_UPLOADED {
            let mut sdata=(*key).sdata;
            if (*sdata).vif.type_==NL80211_IFTYPE_AP_VLAN { sdata=container_of!((*key).sdata.bss,ieee80211_sub_if_data,u.ap); }
            drv_update_tkip_key((*key).local,sdata,&mut (*key).conf,(*key).sta,iv32,rx_ctx.ctx.p1k.as_ptr());
            rx_ctx.ctx.state=TKIP_STATE_PHASE1_HW_UPLOADED;
        }
        tkip_mixing_phase2(tk,&rx_ctx.ctx,iv16,rc4key.as_mut_ptr()); ieee80211_wep_decrypt_data(ctx,rc4key.as_mut_ptr(),16,pos,payload_len-12)
    };
    if res==TKIP_DECRYPT_OK {*out_iv32=iv32;*out_iv16=iv16;} res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
