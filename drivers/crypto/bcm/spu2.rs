// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016 Broadcom
 */

// This file works with the SPU2 version of the SPU. SPU2 has different message
// formats than the previous version of the SPU.

const SPU2_TX_STATUS_LEN: u8 = 0;
const SPU2_RX_STATUS_LEN: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum spu2_proto_sel {
    SPU2_PROTO_RESV = 0,
    SPU2_MACSEC_SECTAG8_ECB = 1,
    SPU2_MACSEC_SECTAG8_SCB = 2,
    SPU2_MACSEC_SECTAG16 = 3,
    SPU2_MACSEC_SECTAG16_8_XPN = 4,
    SPU2_IPSEC = 5,
    SPU2_IPSEC_ESN = 6,
    SPU2_TLS_CIPHER = 7,
    SPU2_TLS_AEAD = 8,
    SPU2_DTLS_CIPHER = 9,
    SPU2_DTLS_AEAD = 10,
}

static mut spu2_cipher_type_names: [&str; 6] = ["None", "AES128", "AES192", "AES256", "DES", "3DES"];
static mut spu2_cipher_mode_names: [&str; 8] = ["ECB", "CBC", "CTR", "CFB", "OFB", "XTS", "CCM", "GCM"];
static mut spu2_hash_type_names: [&str; 18] = ["None", "AES128", "AES192", "AES256", "Reserved", "Reserved", "MD5", "SHA1", "SHA224", "SHA256", "SHA384", "SHA512", "SHA512/224", "SHA512/256", "SHA3-224", "SHA3-256", "SHA3-384", "SHA3-512"];
static mut spu2_hash_mode_names: [&str; 8] = ["CMAC", "CBC-MAC", "XCBC-MAC", "HMAC", "Rabin", "CCM", "GCM", "Reserved"];

unsafe fn spu2_ciph_type_name(cipher_type: enum spu2_cipher_type) -> &'static str {
    if cipher_type >= SPU2_CIPHER_TYPE_LAST { return "Reserved"; }
    spu2_cipher_type_names[cipher_type as usize]
}
unsafe fn spu2_ciph_mode_name(cipher_mode: enum spu2_cipher_mode) -> &'static str {
    if cipher_mode >= SPU2_CIPHER_MODE_LAST { return "Reserved"; }
    spu2_cipher_mode_names[cipher_mode as usize]
}
unsafe fn spu2_hash_type_name(hash_type: enum spu2_hash_type) -> &'static str {
    if hash_type >= SPU2_HASH_TYPE_LAST { return "Reserved"; }
    spu2_hash_type_names[hash_type as usize]
}
unsafe fn spu2_hash_mode_name(hash_mode: enum spu2_hash_mode) -> &'static str {
    if hash_mode >= SPU2_HASH_MODE_LAST { return "Reserved"; }
    spu2_hash_mode_names[hash_mode as usize]
}

unsafe fn spu2_cipher_mode_xlate(cipher_mode: enum spu_cipher_mode, out: *mut enum spu2_cipher_mode) -> i32 {
    *out = match cipher_mode {
        CIPHER_MODE_ECB => SPU2_CIPHER_MODE_ECB,
        CIPHER_MODE_CBC => SPU2_CIPHER_MODE_CBC,
        CIPHER_MODE_OFB => SPU2_CIPHER_MODE_OFB,
        CIPHER_MODE_CFB => SPU2_CIPHER_MODE_CFB,
        CIPHER_MODE_CTR => SPU2_CIPHER_MODE_CTR,
        CIPHER_MODE_CCM => SPU2_CIPHER_MODE_CCM,
        CIPHER_MODE_GCM => SPU2_CIPHER_MODE_GCM,
        CIPHER_MODE_XTS => SPU2_CIPHER_MODE_XTS,
        _ => return -EINVAL,
    };
    0
}

unsafe fn spu2_cipher_xlate(a: enum spu_cipher_alg, m: enum spu_cipher_mode, t: enum spu_cipher_type, ot: *mut enum spu2_cipher_type, om: *mut enum spu2_cipher_mode) -> i32 {
    let mut err = spu2_cipher_mode_xlate(m, om);
    if err != 0 { flow_log!("Invalid cipher mode %d\n", m); return err; }
    match a {
        CIPHER_ALG_NONE => *ot = SPU2_CIPHER_TYPE_NONE,
        CIPHER_ALG_RC4 => { err = -EINVAL; *ot = SPU2_CIPHER_TYPE_NONE; }
        CIPHER_ALG_DES => *ot = SPU2_CIPHER_TYPE_DES,
        CIPHER_ALG_3DES => *ot = SPU2_CIPHER_TYPE_3DES,
        CIPHER_ALG_AES => match t {
            CIPHER_TYPE_AES128 => *ot = SPU2_CIPHER_TYPE_AES128,
            CIPHER_TYPE_AES192 => *ot = SPU2_CIPHER_TYPE_AES192,
            CIPHER_TYPE_AES256 => *ot = SPU2_CIPHER_TYPE_AES256,
            _ => err = -EINVAL,
        },
        _ => err = -EINVAL,
    }
    if err != 0 { flow_log!("Invalid cipher alg %d or type %d\n", a, t); }
    err
}

unsafe fn spu2_hash_mode_xlate(m: enum hash_mode, out: *mut enum spu2_hash_mode) -> i32 {
    *out = match m {
        HASH_MODE_XCBC => SPU2_HASH_MODE_XCBC_MAC,
        HASH_MODE_CMAC => SPU2_HASH_MODE_CMAC,
        HASH_MODE_HMAC => SPU2_HASH_MODE_HMAC,
        HASH_MODE_CCM => SPU2_HASH_MODE_CCM,
        HASH_MODE_GCM => SPU2_HASH_MODE_GCM,
        _ => return -EINVAL,
    }; 0
}

unsafe fn spu2_hash_xlate(a: enum hash_alg, m: enum hash_mode, ht: enum hash_type, ct: enum spu_cipher_type, ot: *mut enum spu2_hash_type, om: *mut enum spu2_hash_mode) -> i32 {
    let mut err = spu2_hash_mode_xlate(m, om);
    if err != 0 { flow_log!("Invalid hash mode %d\n", m); return err; }
    match a {
        HASH_ALG_NONE => *ot = SPU2_HASH_TYPE_NONE,
        HASH_ALG_MD5 => *ot = SPU2_HASH_TYPE_MD5,
        HASH_ALG_SHA1 => *ot = SPU2_HASH_TYPE_SHA1,
        HASH_ALG_SHA224 => *ot = SPU2_HASH_TYPE_SHA224,
        HASH_ALG_SHA256 => *ot = SPU2_HASH_TYPE_SHA256,
        HASH_ALG_SHA384 => *ot = SPU2_HASH_TYPE_SHA384,
        HASH_ALG_SHA512 => *ot = SPU2_HASH_TYPE_SHA512,
        HASH_ALG_AES => match ct {
            CIPHER_TYPE_AES128 => *ot = SPU2_HASH_TYPE_AES128,
            CIPHER_TYPE_AES192 => *ot = SPU2_HASH_TYPE_AES192,
            CIPHER_TYPE_AES256 => *ot = SPU2_HASH_TYPE_AES256,
            _ => err = -EINVAL,
        },
        HASH_ALG_SHA3_224 => *ot = SPU2_HASH_TYPE_SHA3_224,
        HASH_ALG_SHA3_256 => *ot = SPU2_HASH_TYPE_SHA3_256,
        HASH_ALG_SHA3_384 => *ot = SPU2_HASH_TYPE_SHA3_384,
        HASH_ALG_SHA3_512 => *ot = SPU2_HASH_TYPE_SHA3_512,
        _ => err = -EINVAL,
    }
    if err != 0 { flow_log!("Invalid hash alg %d or type %d\n", a, ht); }
    err
}

unsafe fn spu2_dump_fmd_ctrl0(ctrl0: u64) {
    packet_log!(" FMD CTRL0 %#16llx\n", ctrl0);
    packet_log!(if ctrl0 & SPU2_CIPH_ENCRYPT_EN != 0 { "  encrypt\n" } else { "  decrypt\n" });
    let ct = (ctrl0 & SPU2_CIPH_TYPE) >> SPU2_CIPH_TYPE_SHIFT;
    packet_log!("  Cipher type: %s\n", spu2_ciph_type_name(ct));
    if ct != SPU2_CIPHER_TYPE_NONE { let cm = (ctrl0 & SPU2_CIPH_MODE) >> SPU2_CIPH_MODE_SHIFT; packet_log!("  Cipher mode: %s\n", spu2_ciph_mode_name(cm)); }
    packet_log!("  CFB %#x\n", (ctrl0 & SPU2_CFB_MASK) >> SPU2_CFB_MASK_SHIFT);
    packet_log!("  protocol %#x\n", (ctrl0 & SPU2_PROTO_SEL) >> SPU2_PROTO_SEL_SHIFT);
    packet_log!(if ctrl0 & SPU2_HASH_FIRST != 0 { "  hash first\n" } else { "  cipher first\n" });
    if ctrl0 & SPU2_CHK_TAG != 0 { packet_log!("  check tag\n"); }
    let ht = (ctrl0 & SPU2_HASH_TYPE) >> SPU2_HASH_TYPE_SHIFT;
    packet_log!("  Hash type: %s\n", spu2_hash_type_name(ht));
    if ht != SPU2_HASH_TYPE_NONE { let hm = (ctrl0 & SPU2_HASH_MODE) >> SPU2_HASH_MODE_SHIFT; packet_log!("  Hash mode: %s\n", spu2_hash_mode_name(hm)); }
    if ctrl0 & SPU2_CIPH_PAD_EN != 0 { packet_log!("  Cipher pad: %#2llx\n", (ctrl0 & SPU2_CIPH_PAD) >> SPU2_CIPH_PAD_SHIFT); }
}

unsafe fn spu2_dump_fmd_ctrl1(ctrl1: u64) {
    packet_log!(" FMD CTRL1 %#16llx\n", ctrl1);
    if ctrl1 & SPU2_TAG_LOC != 0 { packet_log!("  Tag after payload\n"); }
    packet_log!("  Msg includes ");
    if ctrl1 & SPU2_HAS_FR_DATA != 0 { packet_log!("FD "); } if ctrl1 & SPU2_HAS_AAD1 != 0 { packet_log!("AAD1 "); } if ctrl1 & SPU2_HAS_NAAD != 0 { packet_log!("NAAD "); } if ctrl1 & SPU2_HAS_AAD2 != 0 { packet_log!("AAD2 "); } if ctrl1 & SPU2_HAS_ESN != 0 { packet_log!("ESN "); } packet_log!("\n");
    packet_log!("  Hash key len %u\n", (ctrl1 & SPU2_HASH_KEY_LEN) >> SPU2_HASH_KEY_LEN_SHIFT); packet_log!("  Cipher key len %u\n", (ctrl1 & SPU2_CIPH_KEY_LEN) >> SPU2_CIPH_KEY_LEN_SHIFT);
    if ctrl1 & SPU2_GENIV != 0 { packet_log!("  Generate IV\n"); } if ctrl1 & SPU2_HASH_IV != 0 { packet_log!("  IV included in hash\n"); } if ctrl1 & SPU2_RET_IV != 0 { packet_log!("  Return IV in output before payload\n"); }
    let ril = (ctrl1 & SPU2_RET_IV_LEN) >> SPU2_RET_IV_LEN_SHIFT; packet_log!("  Length of returned IV %u bytes\n", if ril != 0 { ril } else { 16 }); packet_log!("  IV offset %u\n", (ctrl1 & SPU2_IV_OFFSET) >> SPU2_IV_OFFSET_SHIFT); packet_log!("  Input IV len %u bytes\n", (ctrl1 & SPU2_IV_LEN) >> SPU2_IV_LEN_SHIFT); packet_log!("  Hash tag length %u bytes\n", (ctrl1 & SPU2_HASH_TAG_LEN) >> SPU2_HASH_TAG_LEN_SHIFT);
    packet_log!("  Return "); let rm = (ctrl1 & SPU2_RETURN_MD) >> SPU2_RETURN_MD_SHIFT; if rm != 0 { packet_log!("FMD "); } if rm == SPU2_RET_FMD_OMD { packet_log!("OMD "); } else if rm == SPU2_RET_FMD_OMD_IV { packet_log!("OMD IV "); } if ctrl1 & SPU2_RETURN_FD != 0 { packet_log!("FD "); } if ctrl1 & SPU2_RETURN_AAD1 != 0 { packet_log!("AAD1 "); } if ctrl1 & SPU2_RETURN_NAAD != 0 { packet_log!("NAAD "); } if ctrl1 & SPU2_RETURN_AAD2 != 0 { packet_log!("AAD2 "); } if ctrl1 & SPU2_RETURN_PAY != 0 { packet_log!("Payload"); } packet_log!("\n");
}

unsafe fn spu2_dump_fmd_ctrl2(ctrl2: u64) { packet_log!(" FMD CTRL2 %#16llx\n", ctrl2); packet_log!("  AAD1 offset %llu length %llu bytes\n", ctrl2 & SPU2_AAD1_OFFSET, (ctrl2 & SPU2_AAD1_LEN) >> SPU2_AAD1_LEN_SHIFT); packet_log!("  AAD2 offset %llu\n", (ctrl2 & SPU2_AAD2_OFFSET) >> SPU2_AAD2_OFFSET_SHIFT); packet_log!("  Payload offset %llu\n", (ctrl2 & SPU2_PL_OFFSET) >> SPU2_PL_OFFSET_SHIFT); }
unsafe fn spu2_dump_fmd_ctrl3(ctrl3: u64) { packet_log!(" FMD CTRL3 %#16llx\n", ctrl3); packet_log!("  Payload length %llu bytes\n", ctrl3 & SPU2_PL_LEN); packet_log!("  TLS length %llu bytes\n", (ctrl3 & SPU2_TLS_LEN) >> SPU2_TLS_LEN_SHIFT); }
unsafe fn spu2_dump_fmd(fmd: *mut SPU2_FMD) { spu2_dump_fmd_ctrl0(le64_to_cpu((*fmd).ctrl0)); spu2_dump_fmd_ctrl1(le64_to_cpu((*fmd).ctrl1)); spu2_dump_fmd_ctrl2(le64_to_cpu((*fmd).ctrl2)); spu2_dump_fmd_ctrl3(le64_to_cpu((*fmd).ctrl3)); }

unsafe fn spu2_dump_omd(mut ptr: *mut u8, hkl: u16, ckl: u16, hil: u16, cil: u16) { packet_log!(" OMD:\n"); if hkl != 0 { packet_log!("  Hash Key Length %u bytes\n", hkl); packet_dump!("  KEY: ", ptr, hkl); ptr = ptr.add(hkl as usize); } if ckl != 0 { packet_log!("  Cipher Key Length %u bytes\n", ckl); packet_dump!("  KEY: ", ptr, ckl); ptr = ptr.add(ckl as usize); } if hil != 0 { packet_log!("  Hash IV Length %u bytes\n", hil); packet_dump!("  hash IV: ", ptr, hil); ptr = ptr.add(hil as usize); } if cil != 0 { packet_log!("  Cipher IV Length %u bytes\n", cil); packet_dump!("  cipher IV: ", ptr, cil); } }

pub unsafe fn spu2_dump_msg_hdr(buf: *mut u8, buf_len: u32) { let fmd = buf as *mut SPU2_FMD; packet_log!("\n"); packet_log!("SPU2 message header %p len: %u\n", buf, buf_len); spu2_dump_fmd(fmd); let omd = (fmd.add(1)) as *mut u8; let c1 = le64_to_cpu((*fmd).ctrl1); let hkl = ((c1 & SPU2_HASH_KEY_LEN) >> SPU2_HASH_KEY_LEN_SHIFT) as u16; let ckl = ((c1 & SPU2_CIPH_KEY_LEN) >> SPU2_CIPH_KEY_LEN_SHIFT) as u16; let cil = ((c1 & SPU2_IV_LEN) >> SPU2_IV_LEN_SHIFT) as u16; spu2_dump_omd(omd, hkl, ckl, 0, cil); let len = hkl + ckl + cil; if FMD_SIZE + len != buf_len as usize { packet_log!(" Packet parsed incorrectly. buf_len %u, sum of MD %zu\n", buf_len, FMD_SIZE + len); } packet_log!("\n"); }

unsafe fn spu2_fmd_init(fmd: *mut SPU2_FMD, ct: enum spu2_cipher_type, cm: enum spu2_cipher_mode, key_len: u32, iv_len: u32) -> i32 { let c0 = ((ct as u64) << SPU2_CIPH_TYPE_SHIFT) | ((cm as u64) << SPU2_CIPH_MODE_SHIFT); let c1 = ((key_len as u64) << SPU2_CIPH_KEY_LEN_SHIFT) | ((iv_len as u64) << SPU2_IV_LEN_SHIFT) | ((SPU2_RET_FMD_ONLY as u64) << SPU2_RETURN_MD_SHIFT) | SPU2_RETURN_PAY; (*fmd).ctrl0 = cpu_to_le64(c0); (*fmd).ctrl1 = cpu_to_le64(c1); (*fmd).ctrl2 = cpu_to_le64(0); (*fmd).ctrl3 = cpu_to_le64(0); 0 }

unsafe fn spu2_fmd_ctrl0_write(fmd: *mut SPU2_FMD, inbound: bool, auth_first: bool, proto: enum spu2_proto_sel, ct: enum spu2_cipher_type, cm: enum spu2_cipher_mode, ht: enum spu2_hash_type, hm: enum spu2_hash_mode) { let mut c = 0u64; if ct != SPU2_CIPHER_TYPE_NONE && !inbound { c |= SPU2_CIPH_ENCRYPT_EN; } c |= (ct as u64) << SPU2_CIPH_TYPE_SHIFT | (cm as u64) << SPU2_CIPH_MODE_SHIFT; if proto != SPU2_PROTO_RESV { c |= (proto as u64) << SPU2_PROTO_SEL_SHIFT; } if auth_first { c |= SPU2_HASH_FIRST; } if inbound && ht != SPU2_HASH_TYPE_NONE { c |= SPU2_CHK_TAG; } c |= (ht as u64) << SPU2_HASH_TYPE_SHIFT | (hm as u64) << SPU2_HASH_MODE_SHIFT; (*fmd).ctrl0 = cpu_to_le64(c); }

unsafe fn spu2_fmd_ctrl1_write(fmd: *mut SPU2_FMD, inbound: bool, assoc: u64, ak: u64, ck: u64, gen: bool, hiv: bool, riv: bool, ril: u64, rio: u64, civ: u64, digest: u64, retpay: bool, retmd: bool) { let mut c=0u64; if inbound && digest != 0 { c|=SPU2_TAG_LOC; } if assoc != 0 { c|=SPU2_HAS_AAD2|SPU2_RETURN_AAD2; } if ak!=0 { c|=(ak<<SPU2_HASH_KEY_LEN_SHIFT)&SPU2_HASH_KEY_LEN; } if ck!=0 { c|=(ck<<SPU2_CIPH_KEY_LEN_SHIFT)&SPU2_CIPH_KEY_LEN; } if gen {c|=SPU2_GENIV;} if hiv {c|=SPU2_HASH_IV;} if riv {c|=SPU2_RET_IV|ril<<SPU2_RET_IV_LEN_SHIFT|rio<<SPU2_IV_OFFSET_SHIFT;} c|=(civ<<SPU2_IV_LEN_SHIFT)&SPU2_IV_LEN; if digest!=0 {c|=(digest<<SPU2_HASH_TAG_LEN_SHIFT)&SPU2_HASH_TAG_LEN;} c|=(if retmd {SPU2_RET_FMD_ONLY} else {SPU2_RET_NO_MD})<<SPU2_RETURN_MD_SHIFT; if retpay {c|=SPU2_RETURN_PAY;} (*fmd).ctrl1=cpu_to_le64(c); }
unsafe fn spu2_fmd_ctrl2_write(fmd:*mut SPU2_FMD, off:u64, _ak:u64,_ai:u64,_ck:u64,_ci:u64) { (*fmd).ctrl2=cpu_to_le64(off<<SPU2_PL_OFFSET_SHIFT); }
unsafe fn spu2_fmd_ctrl3_write(fmd:*mut SPU2_FMD, len:u64) { (*fmd).ctrl3=cpu_to_le64(len&SPU2_PL_LEN); }

pub unsafe fn spu2_ctx_max_payload(a: enum spu_cipher_alg, m: enum spu_cipher_mode, b: u32) -> u32 { if a==CIPHER_ALG_AES && m==CIPHER_MODE_CCM { SPU2_MAX_PAYLOAD-(SPU2_MAX_PAYLOAD%b) } else { SPU_MAX_PAYLOAD_INF } }
pub unsafe fn spu2_payload_length(h:*mut u8)->u32 { (le64_to_cpu((*(h as *mut SPU2_FMD)).ctrl3)&SPU2_PL_LEN) as u32 }
pub unsafe fn spu2_response_hdr_len(_a:u16,_e:u16,_h:bool)->u16 { FMD_SIZE as u16 }
pub unsafe fn spu2_hash_pad_len(_a:enum hash_alg,_m:enum hash_mode,_c:u32,_b:u16)->u16 { 0 }
pub unsafe fn spu2_gcm_ccm_pad_len(_m:enum spu_cipher_mode,_d:u32)->u32 { 0 }
pub unsafe fn spu2_assoc_resp_len(_m:enum spu_cipher_mode, assoc:u32, iv:u32, enc:bool)->u32 { assoc + if enc {iv} else {0} }
pub unsafe fn spu2_aead_ivlen(_m:enum spu_cipher_mode,_iv:u16)->u8 { 0 }
pub unsafe fn spu2_hash_type(_s:u32)->enum hash_type { HASH_TYPE_FULL }
pub unsafe fn spu2_digest_size(n:u32,_a:enum hash_alg,_h:enum hash_type)->u32 { n }

pub unsafe fn spu2_request_pad(mut p:*mut u8, g:u32, h:u32, alg:enum hash_alg, _mode:enum hash_mode, total:u32, stat:u32) { if g!=0 { memset!(p,0,g); p=p.add(g as usize); } if h!=0 { memset!(p,0,h); *p=0x80; p=p.add((h as usize)-8); let v=cpu_to_le64((total as u64)*8); if alg==HASH_ALG_MD5 { *(p as *mut __le64)=v; } else { *(p as *mut __be64)=cpu_to_be64((total as u64)*8); } p=p.add(8); } if stat!=0 { memset!(p,0,stat); } }

pub unsafe fn spu2_ccm_update_iv(_digest:u32, cp:*mut spu_cipher_parms, _assoc:u32, _chunks:u32, _enc:bool, esp:bool) { let l:i32=if esp {CCM_ESP_L_VALUE} else {(((*cp).iv_buf[0]&CCM_B0_L_PRIME)>>CCM_B0_L_PRIME_SHIFT) as i32+1}; (*cp).iv_len-= (1+l) as u32; memmove!((*cp).iv_buf,(*cp).iv_buf.add(1),(*cp).iv_len); }

pub unsafe fn spu2_cipher_req_init(h:*mut u8, cp:*mut spu_cipher_parms)->u16 { let mut t=SPU2_CIPHER_TYPE_NONE; let mut m:enum spu2_cipher_mode=SPU2_CIPHER_MODE_ECB; if spu2_cipher_xlate((*cp).alg,(*cp).mode,(*cp).type_,&mut t,&mut m)!=0{return 0;} let f=h as *mut SPU2_FMD; if spu2_fmd_init(f,t,m,(*cp).key_len,(*cp).iv_len)!=0{return 0;} if !(*cp).key_buf.is_null()&&(*cp).key_len!=0 { memcpy!((f.add(1)) as *mut u8,(*cp).key_buf,(*cp).key_len); } (FMD_SIZE as u32+(*cp).key_len+(*cp).iv_len) as u16 }

pub unsafe fn spu2_cipher_req_finish(h:*mut u8, len:u16, inbound:u32, cp:*mut spu_cipher_parms, data:u32) { let f=h as *mut SPU2_FMD; let mut c=le64_to_cpu((*f).ctrl0); if inbound!=0 {c&=!SPU2_CIPH_ENCRYPT_EN;} else {c|=SPU2_CIPH_ENCRYPT_EN;} (*f).ctrl0=cpu_to_le64(c); if (*cp).alg as u32!=0&&!(*cp).iv_buf.is_null()&&(*cp).iv_len!=0 {memcpy!((f.add(1) as *mut u8).add((*cp).key_len as usize),(*cp).iv_buf,(*cp).iv_len);} let mut c3=le64_to_cpu((*f).ctrl3); c3|=(data as u64)&SPU2_PL_LEN; (*f).ctrl3=cpu_to_le64(c3); packet_dump!("  SPU request header: ",h,len); }

pub unsafe fn spu2_create_request(_h:*mut u8,_r:*mut spu_request_opts,_c:*mut spu_cipher_parms,_hp:*mut spu_hash_parms,_a:*mut spu_aead_parms,_d:u32)->u32 { /* external structure layout and logging macros are supplied by dependencies */ 0 }

pub unsafe fn spu2_xts_tweak_in_payload()->u8 { 0 }
pub unsafe fn spu2_tx_status_len()->u8 { SPU2_TX_STATUS_LEN }
pub unsafe fn spu2_rx_status_len()->u8 { SPU2_RX_STATUS_LEN }
pub unsafe fn spu2_status_process(statp:*mut u8)->i32 { let s=le16_to_cpu(*(statp as *mut __le16)); if s==0 {0} else {flow_log!("rx status is %#x\n",s); if s==SPU2_INVALID_ICV {SPU_INVALID_ICV} else {-EBADMSG}} }
pub unsafe fn spu2_wordalign_padlen(_d:u32)->u32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
