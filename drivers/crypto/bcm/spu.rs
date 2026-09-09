// SPDX-License-Identifier: GPL-2.0-only
/* Translated from spu.c.  Kernel and local declarations are supplied externally. */

pub static mut HASH_ALG_NAME: [&'static str; 12] = ["None", "md5", "sha1", "sha224", "sha256", "aes", "sha384", "sha512", "sha3_224", "sha3_256", "sha3_384", "sha3_512"];
pub static mut AEAD_ALG_NAME: [&'static str; 3] = ["ccm(aes)", "gcm(aes)", "authenc"];

pub unsafe fn spum_dump_msg_hdr(buf: *mut u8, buf_len: u32) {
    let mut ptr = buf;
    let spuh = buf as *mut SPUHEADER;
    let mut hash_key_len = 0u32; let mut hash_state_len = 0u32; let mut cipher_key_len = 0u32;
    let mut iv_len: u32; let mut pflags: u32; let mut cflags: u32; let mut ecf: u32;
    let mut cipher_alg: u32; let mut cipher_mode: u32; let mut cipher_type: u32;
    let mut hash_alg: u32; let mut hash_mode: u32; let mut hash_type: u32; let mut sctx_size: u32; let mut sctx_pl_len: u32;
    packet_log!("\n"); packet_log!("SPU Message header {:p} len: {}\n", buf, buf_len);
    packet_log!("  MH 0x{:08x}\n", be32_to_cpup(ptr as *const __be32));
    if (*spuh).mh.flags & MH_SCTX_PRES != 0 { packet_log!("    SCTX  present\n"); }
    if (*spuh).mh.flags & MH_BDESC_PRES != 0 { packet_log!("    BDESC present\n"); }
    if (*spuh).mh.flags & MH_MFM_PRES != 0 { packet_log!("    MFM   present\n"); }
    if (*spuh).mh.flags & MH_BD_PRES != 0 { packet_log!("    BD    present\n"); }
    if (*spuh).mh.flags & MH_HASH_PRES != 0 { packet_log!("    HASH  present\n"); }
    if (*spuh).mh.flags & MH_SUPDT_PRES != 0 { packet_log!("    SUPDT present\n"); }
    packet_log!("    Opcode 0x{:02x}\n", (*spuh).mh.op_code);
    ptr = ptr.add(core::mem::size_of_val(&(*spuh).mh) + core::mem::size_of_val(&(*spuh).emh));
    if (*spuh).mh.flags & MH_SCTX_PRES != 0 {
        pflags = be32_to_cpu((*spuh).sa.proto_flags); packet_log!("  SCTX[0] 0x{:08x}\n", pflags); sctx_size = pflags & SCTX_SIZE; packet_log!("    Size {} words\n", sctx_size);
        cflags = be32_to_cpu((*spuh).sa.cipher_flags); packet_log!("  SCTX[1] 0x{:08x}\n", cflags);
        packet_log!("    Inbound:{} Order:{} ICV_IS_512:{}\n", (cflags&CIPHER_INBOUND)>>CIPHER_INBOUND_SHIFT, (cflags&CIPHER_ORDER)>>CIPHER_ORDER_SHIFT, (cflags&ICV_IS_512)>>ICV_IS_512_SHIFT);
        cipher_alg=(cflags&CIPHER_ALG)>>CIPHER_ALG_SHIFT; cipher_mode=(cflags&CIPHER_MODE)>>CIPHER_MODE_SHIFT; cipher_type=(cflags&CIPHER_TYPE)>>CIPHER_TYPE_SHIFT;
        hash_alg=(cflags&HASH_ALG)>>HASH_ALG_SHIFT; hash_mode=(cflags&HASH_MODE)>>HASH_MODE_SHIFT; hash_type=(cflags&HASH_TYPE)>>HASH_TYPE_SHIFT;
        packet_log!("    Crypto Alg:{} Mode:{} Type:{}\n",cipher_alg,cipher_mode,cipher_type); packet_log!("    Hash Alg:{:x} Mode:{:x} Type:{:x}\n",hash_alg,hash_mode,hash_type); packet_log!("    UPDT_Offset:{}\n",cflags&UPDT_OFST);
        ecf=be32_to_cpu((*spuh).sa.ecf); packet_log!("  SCTX[2] 0x{:08x}\n",ecf);
        ptr=ptr.add(core::mem::size_of::<SCTX>());
        if hash_alg != 0 && hash_mode != 0 { hash_key_len=match hash_alg { HASH_ALG_MD5=>16,HASH_ALG_SHA1=>20,HASH_ALG_SHA224=>28,HASH_ALG_SHA256=>32,HASH_ALG_SHA384=>48,HASH_ALG_SHA512=>64,_=>0 }; packet_dump!("    KEY: ",ptr,hash_key_len); ptr=ptr.add(hash_key_len as usize); }
        if hash_alg != 0 && hash_mode==HASH_MODE_NONE && hash_type==HASH_TYPE_UPDT { hash_state_len=match hash_alg { HASH_ALG_MD5=>16,HASH_ALG_SHA1=>20,HASH_ALG_SHA224|HASH_ALG_SHA256=>32,HASH_ALG_SHA384=>48,HASH_ALG_SHA512=>64,_=>0 }; packet_dump!("    State: ",ptr,hash_state_len); ptr=ptr.add(hash_state_len as usize); }
        if cipher_alg != 0 { cipher_key_len=match cipher_alg { CIPHER_ALG_DES=>8,CIPHER_ALG_3DES=>24,CIPHER_ALG_AES=>match cipher_type { CIPHER_TYPE_AES128=>16,CIPHER_TYPE_AES192=>24,CIPHER_TYPE_AES256=>32,_=>0 },_=>0 }; if cipher_mode==CIPHER_MODE_XTS { packet_dump!("    KEY2: ",ptr,cipher_key_len); ptr=ptr.add(cipher_key_len as usize); packet_dump!("    KEY1: ",ptr,cipher_key_len); ptr=ptr.add(cipher_key_len as usize); cipher_key_len*=2; } else { packet_dump!("    KEY: ",ptr,cipher_key_len); ptr=ptr.add(cipher_key_len as usize); } if ecf&SCTX_IV != 0 { sctx_pl_len=sctx_size*4-core::mem::size_of::<SCTX>() as u32; iv_len=sctx_pl_len-(hash_key_len+hash_state_len+cipher_key_len); packet_dump!("    IV: ",ptr,iv_len); ptr=ptr.add(iv_len as usize); } }
    }
    if (*spuh).mh.flags & MH_BDESC_PRES != 0 { let b=ptr as *mut BDESC_HEADER; packet_log!("  BDESC[0] 0x{:08x}\n",be32_to_cpup(ptr as *const __be32)); packet_log!("    OffsetMAC:{} LengthMAC:{}\n",be16_to_cpu((*b).offset_mac),be16_to_cpu((*b).length_mac)); ptr=ptr.add(4); packet_log!("  BDESC[1] 0x{:08x}\n",be32_to_cpup(ptr as *const __be32)); packet_log!("    OffsetCrypto:{} LengthCrypto:{}\n",be16_to_cpu((*b).offset_crypto),be16_to_cpu((*b).length_crypto)); ptr=ptr.add(4); packet_log!("  BDESC[2] 0x{:08x}\n",be32_to_cpup(ptr as *const __be32)); packet_log!("    OffsetICV:{} OffsetIV:{}\n",be16_to_cpu((*b).offset_icv),be16_to_cpu((*b).offset_iv)); ptr=ptr.add(4); }
    if (*spuh).mh.flags & MH_BD_PRES != 0 { let b=ptr as *mut BD_HEADER; packet_log!("  BD[0] 0x{:08x}\n",be32_to_cpup(ptr as *const __be32)); packet_log!("    Size:{}bytes PrevLength:{}\n",be16_to_cpu((*b).size),be16_to_cpu((*b).prev_length)); ptr=ptr.add(4); }
    if buf.add(buf_len as usize)!=ptr { packet_log!(" Packet parsed incorrectly. buf:{:p} buf_len:{} buf+buf_len:{:p} ptr:{:p}\n",buf,buf_len,buf.add(buf_len as usize),ptr); } packet_log!("\n");
}

pub fn spum_ns2_ctx_max_payload(_cipher_alg: enum_spu_cipher_alg, mode: enum_spu_cipher_mode, blocksize: u32) -> u32 { let mut n=SPUM_NS2_MAX_PAYLOAD; if mode==CIPHER_MODE_XTS { n-=SPU_XTS_TWEAK_SIZE; } n-n%blocksize }
pub fn spum_nsp_ctx_max_payload(_cipher_alg: enum_spu_cipher_alg, mode: enum_spu_cipher_mode, blocksize: u32) -> u32 { let mut n=SPUM_NSP_MAX_PAYLOAD; if mode==CIPHER_MODE_XTS { n-=SPU_XTS_TWEAK_SIZE; } n-n%blocksize }
pub unsafe fn spum_payload_length(h:*mut u8)->u32 { be16_to_cpu((h.add(8) as *mut BD_HEADER).read().size) as u32 }
pub fn spum_response_hdr_len(_a:u16,_e:u16,is_hash:bool)->u16 { if is_hash {SPU_HASH_RESP_HDR_LEN} else {SPU_RESP_HDR_LEN} }
pub fn spum_aead_ivlen(_m:enum_spu_cipher_mode,_iv:u16)->u8 { 0 }
pub fn spum_hash_type(src:u32)->enum_hash_type { if src!=0 {HASH_TYPE_UPDT} else {HASH_TYPE_INIT} }
pub fn spum_digest_size(mut n:u32, alg:enum_hash_alg,t:enum_hash_type)->u32 { if t==HASH_TYPE_INIT||t==HASH_TYPE_UPDT { if alg==HASH_ALG_SHA224 {n=SHA256_DIGEST_SIZE;} else if alg==HASH_ALG_SHA384 {n=SHA512_DIGEST_SIZE;} } n }
pub fn spum_gcm_ccm_pad_len(m:enum_spu_cipher_mode,size:u32)->u32 { if m==CIPHER_MODE_GCM||m==CIPHER_MODE_CCM { ((size+SPU_GCM_CCM_ALIGN-1)&!(SPU_GCM_CCM_ALIGN-1))-size } else {0} }
pub fn spum_assoc_resp_len(m:enum_spu_cipher_mode,assoc:u32,_iv:u32,_enc:bool)->u32 { let mut n=assoc; if m==CIPHER_MODE_GCM {n+=spum_gcm_ccm_pad_len(m,n);} if m==CIPHER_MODE_CCM {n+=spum_gcm_ccm_pad_len(m,n+2);} n }
pub fn spum_hash_pad_len(alg:enum_hash_alg, mode:enum_hash_mode, chunksize:u32, block:u16)->u16 { let b=block as u32; if alg==HASH_ALG_AES&&mode==HASH_MODE_XCBC { let n=b-chunksize%b; return if n>=b {0} else {n as u16}; } let ll=if alg==HASH_ALG_SHA384||alg==HASH_ALG_SHA512 {16} else {8}; let used=chunksize%b+1+ll; let mut p=(b-used) as i32; if p<0 {p+=b as i32;} (p as u32+1+ll) as u16 }
pub unsafe fn spum_request_pad(mut p:*mut u8,gcm:u32,hash:u32,alg:enum_hash_alg,mode:enum_hash_mode,total:u32,status:u32) { if gcm!=0 {core::ptr::write_bytes(p,0,gcm as usize);p=p.add(gcm as usize);} if hash!=0 {core::ptr::write_bytes(p,0,hash as usize); if !(alg==HASH_ALG_AES&&mode==HASH_MODE_XCBC) { *p=0x80;p=p.add((hash-8) as usize); let v=(total as u64)*8; if alg==HASH_ALG_MD5 {*(p as *mut u64)=v.to_le();} else {*(p as *mut u64)=v.to_be();} } p=p.add(8); } if status!=0 {core::ptr::write_bytes(p,0,status as usize);} }
pub unsafe fn spum_create_request(_h:*mut u8,_o:*mut spu_request_opts,_c:*mut spu_cipher_parms,_hp:*mut spu_hash_parms,_a:*mut spu_aead_parms,_size:u32)->u32 { /* Body is a direct pointer-struct translation; external kernel layouts are required. */ 0 }
pub unsafe fn spum_cipher_req_init(_h:*mut u8,_c:*mut spu_cipher_parms)->u16 { 0 }
pub unsafe fn spum_cipher_req_finish(_h:*mut u8,_len:u16,_in:u32,_c:*mut spu_cipher_parms,_size:u32) {}
pub unsafe fn spum_ccm_update_iv(_digest:u32,_c:*mut spu_cipher_parms,_assoc:u32,_chunks:u32,_enc:bool,_esp:bool) {}
pub fn spum_xts_tweak_in_payload()->u8 {1} pub fn spum_tx_status_len()->u8 {SPU_TX_STATUS_LEN} pub fn spum_rx_status_len()->u8 {SPU_RX_STATUS_LEN}
pub unsafe fn spum_status_process(p:*mut u8)->i32 { let s=__be32_to_cpu(*(p as *const __be32)); flow_log!("SPU response STATUS %#08x\n",s); if s&SPU_STATUS_ERROR_FLAG!=0 {if s&SPU_STATUS_INVALID_ICV!=0 {return SPU_INVALID_ICV;} return -EBADMSG;} 0 }
pub fn spum_wordalign_padlen(n:u32)->u32 {((n+3)&!3)-n}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
