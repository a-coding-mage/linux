/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from nitrox_req.h. Kernel-provided types/functions are external dependencies. */

pub const PENDING_SIG: u64 = 0xffff_ffff_ffff_ffff;
pub const PRIO: i32 = 4001;
pub type SereqCompletionT = unsafe extern "C" fn(*mut core::ffi::c_void, i32);

#[repr(C)] pub struct Gphdr { pub param0: u16, pub param1: u16, pub param2: u16, pub param3: u16 }

#[repr(C)] pub union SeReqCtrl { pub value: u64, pub s: SeReqCtrlBits }
#[repr(C)] pub struct SeReqCtrlBits { pub bits: u64 }
pub const MAX_IV_LEN: usize = 16;

#[repr(C)] pub struct SeCryptoRequest {
    pub opcode: u8, pub gfp: GfpT, pub flags: u32, pub ctx_handle: u64,
    pub gph: Gphdr, pub ctrl: SeReqCtrl, pub orh: *mut u64, pub comp: *mut u64,
    pub src: *mut Scatterlist, pub dst: *mut Scatterlist,
}

pub const FLEXI_CRYPTO_ENCRYPT_HMAC: u8 = 0x33;
pub const ENCRYPT: u8 = 0; pub const DECRYPT: u8 = 1;
pub const IV_FROM_CTX: u8 = 0; pub const IV_FROM_DPTR: u8 = 1;

#[repr(C)] pub enum FlexiCipher { CIPHER_NULL = 0, CIPHER_3DES_CBC, CIPHER_3DES_ECB, CIPHER_AES_CBC, CIPHER_AES_ECB, CIPHER_AES_CFB, CIPHER_AES_CTR, CIPHER_AES_GCM, CIPHER_AES_XTS, CIPHER_AES_CCM, CIPHER_AES_CBC_CTS, CIPHER_AES_ECB_CTS, CIPHER_INVALID }
#[repr(C)] pub enum FlexiAuth { AUTH_NULL = 0, AUTH_MD5, AUTH_SHA1, AUTH_SHA2_SHA224, AUTH_SHA2_SHA256, AUTH_SHA2_SHA384, AUTH_SHA2_SHA512, AUTH_GMAC, AUTH_INVALID }

#[repr(C)] pub union CryptoKeysU { pub key: [u8; 32], pub key1: [u8; 32] }
#[repr(C)] pub struct CryptoKeys { pub u: CryptoKeysU, pub iv: [u8; 16] }
#[repr(C)] pub union AuthKeysU { pub ipad: [u8; 64], pub key2: [u8; 64] }
#[repr(C)] pub struct AuthKeys { pub u: AuthKeysU, pub opad: [u8; 64] }
#[repr(C)] pub union FcCtxFlags { pub f: u64, pub fu: u64, pub w0: FcCtxFlagsBits }
#[repr(C)] pub struct FcCtxFlagsBits { pub bits: u64 }
#[repr(C)] pub struct FlexiCryptoContext { pub flags: FcCtxFlags, pub crypto: CryptoKeys, pub auth: AuthKeys }

#[repr(C)] pub struct CryptoCtxHdr { pub pool: *mut DmaPool, pub dma: DmaAddrT, pub vaddr: *mut core::ffi::c_void }
#[repr(C)] pub union NitroxCryptoCtxU { pub ctx_handle: u64, pub fctx: *mut FlexiCryptoContext }
#[repr(C)] pub struct NitroxCryptoCtx { pub ndev: *mut NitroxDevice, pub u: NitroxCryptoCtxU, pub chdr: *mut CryptoCtxHdr, pub callback: SereqCompletionT }
#[repr(C)] pub struct NitroxKcryptRequest { pub creq: SeCryptoRequest, pub src: *mut u8, pub dst: *mut u8, pub iv_out: *mut u8 }
#[repr(C)] pub struct NitroxAeadRctx { pub nkreq: NitroxKcryptRequest, pub cryptlen: u32, pub assoclen: u32, pub srclen: u32, pub dstlen: u32, pub iv: *mut u8, pub ivsize: i32, pub flags: u32, pub ctx_handle: u64, pub src: *mut Scatterlist, pub dst: *mut Scatterlist, pub ctrl_arg: u8 }
#[repr(C)] pub struct NitroxRfc4106Rctx { pub base: NitroxAeadRctx, pub src: [Scatterlist; 3], pub dst: [Scatterlist; 3], pub assoc: [u8; 20] }

#[repr(C)] pub union PktInstrHdr { pub bev: u64, pub value: u64, pub s: PktInstrHdrBits }
#[repr(C)] pub struct PktInstrHdrBits { pub bits: u64 }
#[repr(C)] pub union PktHdr { pub bev: [u64; 2], pub value: [u64; 2], pub s: PktHdrBits }
#[repr(C)] pub struct PktHdrBits { pub word0: u64, pub ctxp: u64 }
#[repr(C)] pub union SlcStoreInfo { pub bev: [u64; 2], pub value: [u64; 2], pub s: SlcStoreInfoBits }
#[repr(C)] pub struct SlcStoreInfoBits { pub word0: u64, pub rptr: u64 }
#[repr(C)] pub struct NpsPktInstr { pub dptr0: u64, pub ih: PktInstrHdr, pub irh: PktHdr, pub slc: SlcStoreInfo, pub fdata: [u64; 2] }
#[repr(C)] pub union AQMqCommandWord { pub word3: u64, pub bits: u64 }
#[repr(C)] pub struct AQMqCommandS { pub opcode: u16, pub param1: u16, pub param2: u16, pub dlen: u16, pub dptr: u64, pub rptr: u64, pub word3: AQMqCommandWord }
#[repr(C)] pub struct CtxHdr { pub pool: *mut DmaPool, pub dma: DmaAddrT, pub ctx_dma: DmaAddrT }
#[repr(C)] pub struct NitroxSgcomp { pub len: [u16; 4], pub dma: [u64; 4] }
#[repr(C)] pub struct NitroxSgtable { pub sgmap_cnt: u8, pub total_bytes: u16, pub sgcomp_len: u32, pub sgcomp_dma: DmaAddrT, pub sg: *mut Scatterlist, pub sgcomp: *mut NitroxSgcomp }
pub const ORH_HLEN: usize = 8; pub const COMP_HLEN: usize = 8;
#[repr(C)] pub struct RespHdr { pub orh: *mut u64, pub completion: *mut u64 }
pub type CompletionT = unsafe extern "C" fn(*mut core::ffi::c_void, i32);
#[repr(C)] pub struct NitroxSoftreq { pub response: ListHead, pub backlog: ListHead, pub flags: u32, pub gfp: GfpT, pub status: AtomicT, pub ndev: *mut NitroxDevice, pub cmdq: *mut NitroxCmdq, pub instr: NpsPktInstr, pub resp: RespHdr, pub input: NitroxSgtable, pub output: NitroxSgtable, pub tstamp: usize, pub callback: CompletionT, pub cb_arg: *mut core::ffi::c_void }

pub const AES_KEYSIZE_128: i32 = 16; pub const AES_KEYSIZE_192: i32 = 24; pub const AES_KEYSIZE_256: i32 = 32;
pub unsafe fn flexi_aes_keylen(keylen: i32) -> i32 { match keylen { AES_KEYSIZE_128 => 1, AES_KEYSIZE_192 => 2, AES_KEYSIZE_256 => 3, _ => -22 } }
pub unsafe fn alloc_req_buf(_nents: i32, _extralen: i32, _gfp: GfpT) -> *mut core::ffi::c_void { extern "C" { fn kzalloc(size: usize, flags: GfpT) -> *mut core::ffi::c_void; } kzalloc(core::mem::size_of::<Scatterlist>() * _nents as usize + _extralen as usize, _gfp) }
pub unsafe fn create_single_sg(mut sg: *mut Scatterlist, buf: *mut core::ffi::c_void, buflen: i32) -> *mut Scatterlist { extern "C" { fn sg_set_buf(sg: *mut Scatterlist, buf: *mut core::ffi::c_void, len: usize); } sg_set_buf(sg, buf, buflen as usize); sg = sg.add(1); sg }
pub unsafe fn create_multi_sg(mut to_sg: *mut Scatterlist, mut from_sg: *mut Scatterlist, mut buflen: i32) -> *mut Scatterlist { extern "C" { fn sg_set_buf(sg: *mut Scatterlist, buf: *mut core::ffi::c_void, len: usize); fn sg_virt(sg: *mut Scatterlist) -> *mut core::ffi::c_void; fn sg_next(sg: *mut Scatterlist) -> *mut Scatterlist; } while buflen != 0 && !from_sg.is_null() { let mut sglen = (*from_sg).length as i32; if sglen > buflen { sglen = buflen; } sg_set_buf(to_sg, sg_virt(from_sg), sglen as usize); from_sg = sg_next(from_sg); to_sg = to_sg.add(1); buflen -= sglen; } to_sg }
pub unsafe fn set_orh_value(orh: *mut u64) { core::ptr::write_volatile(orh, PENDING_SIG); }
pub unsafe fn set_comp_value(comp: *mut u64) { core::ptr::write_volatile(comp, PENDING_SIG); }

pub unsafe fn alloc_src_req_buf(nkreq: *mut NitroxKcryptRequest, nents: i32, ivsize: i32) -> i32 { let p = alloc_req_buf(nents, ivsize, (*nkreq).creq.gfp) as *mut u8; (*nkreq).src = p; if p.is_null() { -12 } else { 0 } }
pub unsafe fn nitrox_creq_copy_iv(dst: *mut i8, src: *mut i8, size: i32) { core::ptr::copy_nonoverlapping(src, dst, size as usize); }
pub unsafe fn nitrox_creq_src_sg(iv: *mut i8, ivsize: i32) -> *mut Scatterlist { (iv.offset(ivsize as isize)) as *mut Scatterlist }
pub unsafe fn nitrox_creq_set_src_sg(nkreq: *mut NitroxKcryptRequest, nents: i32, ivsize: i32, src: *mut Scatterlist, buflen: i32) { let iv = (*nkreq).src as *mut i8; let creq = &mut (*nkreq).creq; creq.src = nitrox_creq_src_sg(iv, ivsize); extern "C" { fn sg_init_table(sg: *mut Scatterlist, nents: usize); } sg_init_table(creq.src, nents as usize); let sg = create_single_sg(creq.src, iv as *mut _, ivsize); let _ = create_multi_sg(sg, src, buflen); }
pub unsafe fn alloc_dst_req_buf(nkreq: *mut NitroxKcryptRequest, nents: i32) -> i32 { let p = alloc_req_buf(nents, (ORH_HLEN + COMP_HLEN) as i32, (*nkreq).creq.gfp) as *mut u8; (*nkreq).dst = p; if p.is_null() { -12 } else { 0 } }
pub unsafe fn nitrox_creq_set_orh(nkreq: *mut NitroxKcryptRequest) { (*nkreq).creq.orh = (*nkreq).dst as *mut u64; set_orh_value((*nkreq).creq.orh); }
pub unsafe fn nitrox_creq_set_comp(nkreq: *mut NitroxKcryptRequest) { (*nkreq).creq.comp = (*nkreq).dst.add(ORH_HLEN) as *mut u64; set_comp_value((*nkreq).creq.comp); }
pub unsafe fn nitrox_creq_dst_sg(dst: *mut i8) -> *mut Scatterlist { dst.add(ORH_HLEN + COMP_HLEN) as *mut Scatterlist }
pub unsafe fn nitrox_creq_set_dst_sg(nkreq: *mut NitroxKcryptRequest, nents: i32, ivsize: i32, dst: *mut Scatterlist, buflen: i32) { let creq = &mut (*nkreq).creq; extern "C" { fn sg_init_table(sg: *mut Scatterlist, nents: usize); } creq.dst = nitrox_creq_dst_sg((*nkreq).dst as *mut i8); sg_init_table(creq.dst, nents as usize); let mut sg = create_single_sg(creq.dst, creq.orh as *mut _, ORH_HLEN as i32); sg = create_single_sg(sg, (*nkreq).src as *mut _, ivsize); sg = create_multi_sg(sg, dst, buflen); let _ = create_single_sg(sg, creq.comp as *mut _, COMP_HLEN as i32); }

/* External kernel types and helpers used by the remaining inline request helpers. */
pub type GfpT = usize; pub type DmaAddrT = u64; pub type AtomicT = i32;
#[repr(C)] pub struct DmaPool { _private: [u8; 0] } #[repr(C)] pub struct NitroxDevice { _private: [u8; 0] } #[repr(C)] pub struct NitroxCmdq { _private: [u8; 0] } #[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct Scatterlist { pub length: u32, _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
