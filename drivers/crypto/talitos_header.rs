/* SPDX-License-Identifier: BSD-3-Clause */
/* Freescale SEC (talitos) device register and descriptor header defines */

pub const TALITOS_TIMEOUT: u32 = 100000;
pub const TALITOS1_MAX_DATA_LEN: u32 = 32768;
pub const TALITOS2_MAX_DATA_LEN: u32 = 65535;

#[inline]
pub unsafe fn DESC_TYPE(desc_hdr: u32) -> u32 { (be32_to_cpu(desc_hdr) >> 3) & 0x1f }
#[inline]
pub unsafe fn PRIMARY_EU(desc_hdr: u32) -> u32 { (be32_to_cpu(desc_hdr) >> 28) & 0xf }
#[inline]
pub unsafe fn SECONDARY_EU(desc_hdr: u32) -> u32 { (be32_to_cpu(desc_hdr) >> 16) & 0xf }

#[repr(C)]
pub union talitos_ptr_inner {
    pub sec2: talitos_ptr_sec2,
    pub sec1: talitos_ptr_sec1,
}
#[repr(C)]
pub struct talitos_ptr_sec2 { pub len: __be16, pub j_extent: u8, pub eptr: u8 }
#[repr(C)]
pub struct talitos_ptr_sec1 { pub res: __be16, pub len1: __be16 }
#[repr(C)]
pub struct talitos_ptr { pub u: talitos_ptr_inner, pub ptr: __be32 }

#[repr(C)]
pub union talitos_desc_hdr { pub hdr_lo: __be32, pub hdr1: __be32 }
#[repr(C)]
pub struct talitos_desc {
    pub hdr: __be32,
    pub hdr_union: talitos_desc_hdr,
    pub ptr: [talitos_ptr; 7],
    pub next_desc: __be32,
}

pub const TALITOS_DESC_SIZE: usize = core::mem::size_of::<talitos_desc>() - core::mem::size_of::<__be32>();

#[repr(C)]
pub struct talitos_edesc {
    pub bufsl: [scatterlist; 2], pub src: *mut scatterlist,
    pub first: core::ffi::c_int, pub last: core::ffi::c_int,
    pub src_nents: core::ffi::c_int, pub dst_nents: core::ffi::c_int,
    pub iv_dma: dma_addr_t, pub dma_len: core::ffi::c_int,
    pub dma_link_tbl: dma_addr_t, pub next_desc: *mut talitos_edesc,
    pub desc: talitos_desc,
    /* Flexible array member; select link_tbl or buf as appropriate. */
    pub link_tbl: [talitos_ptr; 0],
}

#[repr(C)]
pub struct talitos_request {
    pub desc: *mut talitos_desc,
    pub dma_desc: dma_addr_t,
    pub callback: Option<unsafe extern "C" fn(*mut device, *mut talitos_desc, *mut core::ffi::c_void, core::ffi::c_int)>,
    pub context: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct talitos_channel {
    pub reg: *mut core::ffi::c_void, pub fifo: *mut talitos_request,
    pub submit_count: atomic_t, pub head_lock: spinlock_t, pub head: core::ffi::c_int,
    pub tail_lock: spinlock_t, pub tail: core::ffi::c_int,
}

#[repr(C)]
pub struct talitos_private {
    pub dev: *mut device, pub ofdev: *mut platform_device,
    pub reg: *mut core::ffi::c_void, pub reg_deu: *mut core::ffi::c_void,
    pub reg_aesu: *mut core::ffi::c_void, pub reg_mdeu: *mut core::ffi::c_void,
    pub reg_afeu: *mut core::ffi::c_void, pub reg_rngu: *mut core::ffi::c_void,
    pub reg_pkeu: *mut core::ffi::c_void, pub reg_keu: *mut core::ffi::c_void,
    pub reg_crcu: *mut core::ffi::c_void, pub irq: [core::ffi::c_int; 2],
    pub reg_lock: spinlock_t, pub num_channels: u32, pub chfifo_len: u32,
    pub exec_units: u32, pub desc_types: u32, pub features: c_ulong,
    pub fifo_len: u32, pub last_chan: atomic_t, pub done_task: [tasklet_struct; 2],
    pub alg_list: list_head, pub rng: hwrng, pub rng_registered: bool,
    pub chan: [talitos_channel; 0],
}

pub const TALITOS_FTR_SRC_LINK_TBL_LEN_INCLUDES_EXTENT: c_ulong = 0x1;
pub const TALITOS_FTR_HW_AUTH_CHECK: c_ulong = 0x2;
pub const TALITOS_FTR_SHA224_HWINIT: c_ulong = 0x4;
pub const TALITOS_FTR_HMAC_OK: c_ulong = 0x8;
pub const TALITOS_FTR_SEC1: c_ulong = 0x10;

/* Build-time CONFIG_CRYPTO_DEV_TALITOS1/2 conditions are supplied externally. */
#[inline]
pub unsafe fn has_ftr_sec1(priv_: *mut talitos_private) -> bool {
    #[cfg(all(feature = "CONFIG_CRYPTO_DEV_TALITOS1", feature = "CONFIG_CRYPTO_DEV_TALITOS2"))]
    { return (*priv_).features & TALITOS_FTR_SEC1 != 0; }
    #[cfg(feature = "CONFIG_CRYPTO_DEV_TALITOS1")]
    { true }
    #[cfg(not(feature = "CONFIG_CRYPTO_DEV_TALITOS1"))]
    { false }
}

#[inline] pub const fn ISR1_FORMAT(x: u32) -> u32 { (x << 28) | (x << 16) }
#[inline] pub const fn ISR2_FORMAT(x: u32) -> u32 { (x << 4) | x }

pub const TALITOS_MCR:u32=0x1030; pub const TALITOS_MCR_RCA0:u32=1<<15; pub const TALITOS_MCR_RCA1:u32=1<<14; pub const TALITOS_MCR_RCA2:u32=1<<13; pub const TALITOS_MCR_RCA3:u32=1<<12; pub const TALITOS1_MCR_SWR:u32=0x1000000; pub const TALITOS2_MCR_SWR:u32=1; pub const TALITOS_MCR_LO:u32=0x1034; pub const TALITOS_IMR:u32=0x1008;
pub const TALITOS1_IMR_INIT:u32=ISR1_FORMAT(0xf); pub const TALITOS1_IMR_DONE:u32=ISR1_FORMAT(5); pub const TALITOS2_IMR_INIT:u32=ISR2_FORMAT(0xf)|0x10000; pub const TALITOS2_IMR_DONE:u32=ISR1_FORMAT(5); pub const TALITOS_IMR_LO:u32=0x100c; pub const TALITOS1_IMR_LO_INIT:u32=0x2000000; pub const TALITOS2_IMR_LO_INIT:u32=0x20000; pub const TALITOS_ISR:u32=0x1010; pub const TALITOS_ISR_LO:u32=0x1014; pub const TALITOS_ICR:u32=0x1018; pub const TALITOS_ICR_LO:u32=0x101c;
pub const TALITOS_CH_BASE_OFFSET:u32=0x1000; pub const TALITOS1_CH_STRIDE:u32=0x1000; pub const TALITOS2_CH_STRIDE:u32=0x100; pub const TALITOS_CCCR:u32=8; pub const TALITOS2_CCCR_CONT:u32=2; pub const TALITOS2_CCCR_RESET:u32=1; pub const TALITOS_CCCR_LO:u32=0xc; pub const TALITOS_CCCR_LO_IWSE:u32=0x80; pub const TALITOS_CCCR_LO_EAE:u32=0x20; pub const TALITOS_CCCR_LO_CDWE:u32=0x10; pub const TALITOS_CCCR_LO_NE:u32=8; pub const TALITOS_CCCR_LO_NT:u32=4; pub const TALITOS_CCCR_LO_CDIE:u32=2; pub const TALITOS1_CCCR_LO_RESET:u32=1;
pub const TALITOS_CCPSR:u32=0x10; pub const TALITOS_CCPSR_LO:u32=0x14; pub const TALITOS_FF:u32=0x48; pub const TALITOS_FF_LO:u32=0x4c; pub const TALITOS_CDPR:u32=0x40; pub const TALITOS_CDPR_LO:u32=0x44; pub const TALITOS_DESCBUF:u32=0x80; pub const TALITOS_DESCBUF_LO:u32=0x84; pub const TALITOS_GATHER:u32=0xc0; pub const TALITOS_GATHER_LO:u32=0xc4; pub const TALITOS_SCATTER:u32=0xe0; pub const TALITOS_SCATTER_LO:u32=0xe4;
pub const TALITOS2_DEU:u32=0x2000; pub const TALITOS2_AESU:u32=0x4000; pub const TALITOS2_MDEU:u32=0x6000; pub const TALITOS2_AFEU:u32=0x8000; pub const TALITOS2_RNGU:u32=0xa000; pub const TALITOS2_PKEU:u32=0xc000; pub const TALITOS2_KEU:u32=0xe000; pub const TALITOS2_CRCU:u32=0xf000; pub const TALITOS12_AESU:u32=0x4000; pub const TALITOS12_DEU:u32=0x5000; pub const TALITOS12_MDEU:u32=0x6000; pub const TALITOS10_AFEU:u32=0x8000; pub const TALITOS10_DEU:u32=0xa000; pub const TALITOS10_MDEU:u32=0xc000; pub const TALITOS10_RNGU:u32=0xe000; pub const TALITOS10_PKEU:u32=0x10000; pub const TALITOS10_AESU:u32=0x12000;
pub const TALITOS_EUDSR:u32=0x10; pub const TALITOS_EUDSR_LO:u32=0x14; pub const TALITOS_EURCR:u32=0x18; pub const TALITOS_EURCR_LO:u32=0x1c; pub const TALITOS_EUSR:u32=0x28; pub const TALITOS_EUSR_LO:u32=0x2c; pub const TALITOS_EUISR:u32=0x30; pub const TALITOS_EUISR_LO:u32=0x34; pub const TALITOS_EUICR:u32=0x38; pub const TALITOS_EUICR_LO:u32=0x3c; pub const TALITOS_EU_FIFO:u32=0x800; pub const TALITOS_EU_FIFO_LO:u32=0x804;
pub const TALITOS1_DEUICR_KPE:u32=0x00200000; pub const TALITOS_MDEUICR_LO_ICE:u32=0x4000; pub const TALITOS_RNGUSR_LO_RD:u32=1; pub const TALITOS_RNGUSR_LO_OFL:u32=0xff0000; pub const TALITOS_RNGURCR_LO_SR:u32=1; pub const TALITOS_MDEU_CONTEXT_SIZE_MD5_SHA1_SHA256:u32=0x28; pub const TALITOS_MDEU_CONTEXT_SIZE_SHA384_SHA512:u32=0x48;
pub const DESC_PTR_LNKTBL_JUMP:u8=0x80; pub const DESC_PTR_LNKTBL_RET:u8=2; pub const DESC_PTR_LNKTBL_NEXT:u8=1;

/* Descriptor header values retain their original big-endian conversion intent. */
#[inline] pub const fn cpu_to_be32(v:u32)->u32 { v.to_be() }
pub const DESC_HDR_DONE:u32=0xff000000; pub const DESC_HDR_LO_ICCR1_MASK:u32=0x00180000; pub const DESC_HDR_LO_ICCR1_PASS:u32=0x00080000; pub const DESC_HDR_LO_ICCR1_FAIL:u32=0x00100000;
pub const DESC_HDR_SEL0_MASK:u32=0xf0000000; pub const DESC_HDR_SEL0_AFEU:u32=0x10000000; pub const DESC_HDR_SEL0_DEU:u32=0x20000000; pub const DESC_HDR_SEL0_MDEUA:u32=0x30000000; pub const DESC_HDR_SEL0_MDEUB:u32=0xb0000000; pub const DESC_HDR_SEL0_RNG:u32=0x40000000; pub const DESC_HDR_SEL0_PKEU:u32=0x50000000; pub const DESC_HDR_SEL0_AESU:u32=0x60000000; pub const DESC_HDR_SEL0_KEU:u32=0x70000000; pub const DESC_HDR_SEL0_CRCU:u32=0x80000000;
pub const DESC_HDR_MODE0_ENCRYPT:u32=0x00100000; pub const DESC_HDR_MODE0_AESU_MASK:u32=0x00600000; pub const DESC_HDR_MODE0_AESU_CBC:u32=0x00200000; pub const DESC_HDR_MODE0_AESU_CTR:u32=0x00600000; pub const DESC_HDR_MODE0_DEU_CBC:u32=0x00400000; pub const DESC_HDR_MODE0_DEU_3DES:u32=0x00200000; pub const DESC_HDR_MODE0_MDEU_CONT:u32=0x08000000; pub const DESC_HDR_MODE0_MDEU_INIT:u32=0x01000000; pub const DESC_HDR_MODE0_MDEU_HMAC:u32=0x00800000; pub const DESC_HDR_MODE0_MDEU_PAD:u32=0x00400000; pub const DESC_HDR_MODE0_MDEU_SHA224:u32=0x00300000; pub const DESC_HDR_MODE0_MDEU_MD5:u32=0x00200000; pub const DESC_HDR_MODE0_MDEU_SHA256:u32=0x00100000; pub const DESC_HDR_MODE0_MDEU_SHA1:u32=0; pub const DESC_HDR_MODE0_MDEUB_SHA384:u32=0; pub const DESC_HDR_MODE0_MDEUB_SHA512:u32=0x00200000;
pub const DESC_HDR_MODE0_MDEU_MD5_HMAC:u32=0x00a00000; pub const DESC_HDR_MODE0_MDEU_SHA256_HMAC:u32=0x00900000; pub const DESC_HDR_MODE0_MDEU_SHA1_HMAC:u32=0x00800000; pub const DESC_HDR_SEL1_MASK:u32=0x000f0000; pub const DESC_HDR_SEL1_MDEUA:u32=0x00030000; pub const DESC_HDR_SEL1_MDEUB:u32=0x000b0000; pub const DESC_HDR_SEL1_CRCU:u32=0x00080000;
pub const DESC_HDR_MODE1_MDEU_CICV:u32=0x4000; pub const DESC_HDR_MODE1_MDEU_INIT:u32=0x1000; pub const DESC_HDR_MODE1_MDEU_HMAC:u32=0x800; pub const DESC_HDR_MODE1_MDEU_PAD:u32=0x400; pub const DESC_HDR_MODE1_MDEU_SHA224:u32=0x300; pub const DESC_HDR_MODE1_MDEU_MD5:u32=0x200; pub const DESC_HDR_MODE1_MDEU_SHA256:u32=0x100; pub const DESC_HDR_MODE1_MDEU_SHA1:u32=0; pub const DESC_HDR_MODE1_MDEUB_SHA384:u32=0; pub const DESC_HDR_MODE1_MDEUB_SHA512:u32=0x200; pub const DESC_HDR_DIR_INBOUND:u32=2; pub const DESC_HDR_DONE_NOTIFY:u32=1; pub const DESC_HDR_TYPE_AESU_CTR_NONSNOOP:u32=0; pub const DESC_HDR_TYPE_IPSEC_ESP:u32=8; pub const DESC_HDR_TYPE_COMMON_NONSNOOP_NO_AFEU:u32=16; pub const DESC_HDR_TYPE_HMAC_SNOOP_NO_AFEU:u32=32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
