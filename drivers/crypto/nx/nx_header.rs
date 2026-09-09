/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies are supplied by other translated files.

pub const NX_NAME: &str = "nx-crypto";
pub const NX_STRING: &str = "IBM Power7+ Nest Accelerator Crypto Driver";
pub const NX_VERSION: &str = "1.0";

/* a scatterlist in the format PHYP is expecting */
#[repr(C, packed)]
pub struct nx_sg {
    pub addr: u64,
    pub rsvd: u32,
    pub len: u32,
}

pub const NX_PAGE_SIZE: usize = 4096;
pub const NX_MAX_SG_ENTRIES: usize = NX_PAGE_SIZE / core::mem::size_of::<nx_sg>();

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nx_status {
    NX_DISABLED,
    NX_WAITING,
    NX_OKAY,
}

/* msc_triplet and max_sync_cop are used only to assist in parsing the
 * openFirmware property */
#[repr(C, packed)]
pub struct msc_triplet {
    pub keybitlen: u32,
    pub databytelen: u32,
    pub sglen: u32,
}

#[repr(C, packed)]
pub struct max_sync_cop {
    pub fc: u32,
    pub mode: u32,
    pub triplets: u32,
    pub trip: [msc_triplet; 0],
}

#[repr(C)]
pub struct alg_props {
    pub databytelen: u32,
    pub sglen: u32,
}

pub const NX_OF_FLAG_MAXSGLEN_SET: u32 = 1;
pub const NX_OF_FLAG_STATUS_SET: u32 = 2;
pub const NX_OF_FLAG_MAXSYNCCOP_SET: u32 = 4;
pub const NX_OF_FLAG_MASK_READY: u32 = NX_OF_FLAG_MAXSGLEN_SET
    | NX_OF_FLAG_STATUS_SET
    | NX_OF_FLAG_MAXSYNCCOP_SET;

#[repr(C)]
pub struct nx_of {
    pub flags: u32,
    pub max_sg_len: u32,
    pub status: nx_status,
    pub ap: [[[alg_props; 3]; NX_MAX_MODE]; NX_MAX_FC],
}

#[repr(C)]
pub struct nx_stats {
    pub aes_ops: atomic_t,
    pub aes_bytes: atomic64_t,
    pub sha256_ops: atomic_t,
    pub sha256_bytes: atomic64_t,
    pub sha512_ops: atomic_t,
    pub sha512_bytes: atomic64_t,
    pub sync_ops: atomic_t,
    pub errors: atomic_t,
    pub last_error: atomic_t,
    pub last_error_pid: atomic_t,
}

#[repr(C)]
pub struct nx_crypto_driver {
    pub stats: nx_stats,
    pub of: nx_of,
    pub viodev: *mut vio_dev,
    pub viodriver: vio_driver,
    pub dfs_root: *mut dentry,
}

pub const NX_GCM4106_NONCE_LEN: usize = 4;
pub const NX_GCM_CTR_OFFSET: usize = 12;

#[repr(C)]
pub struct nx_gcm_rctx { pub iv: [u8; 16] }

#[repr(C)]
pub struct nx_gcm_priv {
    pub iauth_tag: [u8; 16],
    pub nonce: [u8; NX_GCM4106_NONCE_LEN],
}

pub const NX_CCM_AES_KEY_LEN: usize = 16;
pub const NX_CCM4309_AES_KEY_LEN: usize = 19;
pub const NX_CCM4309_NONCE_LEN: usize = 3;

#[repr(C)]
pub struct nx_ccm_rctx { pub iv: [u8; 16] }

#[repr(C)]
pub struct nx_ccm_priv {
    pub b0: [u8; 16],
    pub iauth_tag: [u8; 16],
    pub oauth_tag: [u8; 16],
    pub nonce: [u8; NX_CCM4309_NONCE_LEN],
}

#[repr(C)]
pub struct nx_xcbc_priv { pub key: [u8; 16] }

#[repr(C)]
pub struct nx_ctr_priv { pub nonce: [u8; CTR_RFC3686_NONCE_SIZE] }

#[repr(C)]
pub union nx_crypto_ctx_priv {
    pub gcm: nx_gcm_priv,
    pub ccm: nx_ccm_priv,
    pub xcbc: nx_xcbc_priv,
    pub ctr: nx_ctr_priv,
}

#[repr(C)]
pub struct nx_crypto_ctx {
    pub lock: spinlock_t,       /* synchronize access to the context */
    pub kmem: *mut core::ffi::c_void, /* unaligned, kmalloc'd buffer */
    pub kmem_len: usize,        /* length of kmem */
    pub csbcpb: *mut nx_csbcpb, /* aligned page given to phyp @ hcall time */
    pub op: vio_pfo_op,         /* operation struct with hcall parameters */
    pub csbcpb_aead: *mut nx_csbcpb, /* secondary csbcpb used by AEAD algs */
    pub op_aead: vio_pfo_op,    /* operation struct for csbcpb_aead */
    pub in_sg: *mut nx_sg,      /* aligned pointer into kmem to an sg list */
    pub out_sg: *mut nx_sg,     /* aligned pointer into kmem to an sg list */
    pub ap: *mut alg_props,     /* pointer into props based on our key size */
    pub props: [alg_props; 3],  /* openFirmware properties for requests */
    pub stats: *mut nx_stats,   /* pointer into an nx_crypto_driver for stats
                                   reporting */
    pub priv_: nx_crypto_ctx_priv,
}

#[repr(C)]
pub struct scatterlist;

unsafe extern "C" {
    pub fn nx_crypto_ctx_aes_ccm_init(tfm: *mut crypto_aead) -> i32;
    pub fn nx_crypto_ctx_aes_gcm_init(tfm: *mut crypto_aead) -> i32;
    pub fn nx_crypto_ctx_aes_xcbc_init(tfm: *mut crypto_shash) -> i32;
    pub fn nx_crypto_ctx_aes_ctr_init(tfm: *mut crypto_skcipher) -> i32;
    pub fn nx_crypto_ctx_aes_cbc_init(tfm: *mut crypto_skcipher) -> i32;
    pub fn nx_crypto_ctx_aes_ecb_init(tfm: *mut crypto_skcipher) -> i32;
    pub fn nx_crypto_ctx_sha_init(tfm: *mut crypto_shash) -> i32;
    pub fn nx_crypto_ctx_exit(nx_ctx: *mut nx_crypto_ctx);
    pub fn nx_crypto_ctx_skcipher_exit(tfm: *mut crypto_skcipher);
    pub fn nx_crypto_ctx_aead_exit(tfm: *mut crypto_aead);
    pub fn nx_crypto_ctx_shash_exit(tfm: *mut crypto_shash);
    pub fn nx_ctx_init(nx_ctx: *mut nx_crypto_ctx, function: core::ffi::c_uint);
    pub fn nx_hcall_sync(ctx: *mut nx_crypto_ctx, op: *mut vio_pfo_op, may_sleep: u32) -> i32;
    pub fn nx_build_sg_list(sg: *mut nx_sg, ptr: *mut u8, nbytes: *mut core::ffi::c_uint, max_sg_len: u32) -> *mut nx_sg;
    pub fn nx_build_sg_lists(nx_ctx: *mut nx_crypto_ctx, iv: *const u8, dst: *mut scatterlist, src: *mut scatterlist, nbytes: *mut core::ffi::c_uint, offset: core::ffi::c_uint, oiv: *mut u8) -> i32;
    pub fn nx_walk_and_build(sg: *mut nx_sg, sgmax: core::ffi::c_uint, sglist: *mut scatterlist, offset: core::ffi::c_uint, nbytes: *mut core::ffi::c_uint) -> *mut nx_sg;

    #[cfg(CONFIG_DEBUG_FS)]
    pub fn nx_debugfs_init(drv: *mut nx_crypto_driver);
    #[cfg(CONFIG_DEBUG_FS)]
    pub fn nx_debugfs_fini(drv: *mut nx_crypto_driver);

    pub static mut nx_cbc_aes_alg: skcipher_alg;
    pub static mut nx_ecb_aes_alg: skcipher_alg;
    pub static mut nx_gcm_aes_alg: aead_alg;
    pub static mut nx_gcm4106_aes_alg: aead_alg;
    pub static mut nx_ctr3686_aes_alg: skcipher_alg;
    pub static mut nx_ccm_aes_alg: aead_alg;
    pub static mut nx_ccm4309_aes_alg: aead_alg;
    pub static mut nx_shash_aes_xcbc_alg: shash_alg;
    pub static mut nx_shash_sha512_alg: shash_alg;
    pub static mut nx_shash_sha256_alg: shash_alg;
    pub static mut nx_driver: nx_crypto_driver;
}

#[inline]
pub const fn nx_page_num(x: u64) -> u64 { x & 0xfffffffffffff000u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
