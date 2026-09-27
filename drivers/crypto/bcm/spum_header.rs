/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2016 Broadcom
 */

/*
 * This file contains SPU message definitions specific to SPU-M.
 */

pub const SPU_CRYPTO_OPERATION_GENERIC: u32 = 0x1;

/* Length of STATUS field in tx and rx packets */
pub const SPU_TX_STATUS_LEN: u32 = 4;

/* SPU-M error codes */
pub const SPU_STATUS_MASK: u32 = 0x0000FF00;
pub const SPU_STATUS_SUCCESS: u32 = 0x00000000;
pub const SPU_STATUS_INVALID_ICV: u32 = 0x00000100;

pub const SPU_STATUS_ERROR_FLAG: u32 = 0x00020000;

/* Request message. MH + EMH + BDESC + BD header */
pub const SPU_REQ_FIXED_LEN: u32 = 24;

/*
 * Max length of a SPU message header. Used to allocate a buffer where
 * the SPU message header is constructed. Can be used for either a SPU-M
 * header or a SPU2 header.
 * For SPU-M, sum of the following:
 *    MH - 4 bytes
 *    EMH - 4
 *    SCTX - 3 +
 *      max auth key len - 64
 *      max cipher key len - 264 (RC4)
 *      max IV len - 16
 *    BDESC - 12
 *    BD header - 4
 * Total:  371
 *
 * For SPU2, FMD_SIZE (32) plus lengths of hash and cipher keys,
 * hash and cipher IVs. If SPU2 does not support RC4, then
 */
pub const SPU_HEADER_ALLOC_LEN: u32 = SPU_REQ_FIXED_LEN + MAX_KEY_SIZE + MAX_KEY_SIZE + MAX_IV_SIZE;

/*
 * Response message header length. Normally MH, EMH, BD header, but when
 * BD_SUPPRESS is used for hash requests, there is no BD header.
 */
pub const SPU_RESP_HDR_LEN: u32 = 12;
pub const SPU_HASH_RESP_HDR_LEN: u32 = 8;

/*
 * Max value that can be represented in the Payload Length field of the BD
 * header. This is a 16-bit field.
 */
pub const SPUM_NS2_MAX_PAYLOAD: kernel::ffi::c_ulong = BIT(16) - 1;

/* Buffer Descriptor Header [BDESC]. SPU in big-endian mode. */
#[repr(C)]
pub struct BDESC_HEADER {
    pub offset_mac: __be16,
    pub length_mac: __be16,
    pub offset_crypto: __be16,
    pub length_crypto: __be16,
    pub offset_icv: __be16,
    pub offset_iv: __be16,
}

/* Buffer Data Header [BD]. SPU in big-endian mode. */
#[repr(C)]
pub struct BD_HEADER {
    pub size: __be16,
    pub prev_length: __be16,
}

/* Command Context Header. SPU-M in big endian mode. */
#[repr(C)]
pub struct MHEADER {
    pub flags: u8,
    pub op_code: u8,
    pub reserved: u16,
}

/* MH header flags bits */
pub const MH_SUPDT_PRES: kernel::ffi::c_ulong = BIT(0);
pub const MH_HASH_PRES: kernel::ffi::c_ulong = BIT(2);
pub const MH_BD_PRES: kernel::ffi::c_ulong = BIT(3);
pub const MH_MFM_PRES: kernel::ffi::c_ulong = BIT(4);
pub const MH_BDESC_PRES: kernel::ffi::c_ulong = BIT(5);
pub const MH_SCTX_PRES: kernel::ffi::c_ulong = BIT(7);

/* SCTX word 0 bit offsets and fields masks */
pub const SCTX_SIZE: u32 = 0x000000FF;

/* SCTX word 1 bit shifts and field masks */
pub const UPDT_OFST: u32 = 0x000000FF;
pub const HASH_TYPE: u32 = 0x00000300;
pub const HASH_TYPE_SHIFT: u32 = 8;
pub const HASH_MODE: u32 = 0x00001C00;
pub const HASH_MODE_SHIFT: u32 = 10;
pub const HASH_ALG: u32 = 0x0000E000;
pub const HASH_ALG_SHIFT: u32 = 13;
pub const CIPHER_TYPE: u32 = 0x00030000;
pub const CIPHER_TYPE_SHIFT: u32 = 16;
pub const CIPHER_MODE: u32 = 0x001C0000;
pub const CIPHER_MODE_SHIFT: u32 = 18;
pub const CIPHER_ALG: u32 = 0x00E00000;
pub const CIPHER_ALG_SHIFT: u32 = 21;
pub const ICV_IS_512: kernel::ffi::c_ulong = BIT(27);
pub const ICV_IS_512_SHIFT: u32 = 27;
pub const CIPHER_ORDER: kernel::ffi::c_ulong = BIT(30);
pub const CIPHER_ORDER_SHIFT: u32 = 30;
pub const CIPHER_INBOUND: kernel::ffi::c_ulong = BIT(31);
pub const CIPHER_INBOUND_SHIFT: u32 = 31;

/* SCTX word 2 bit shifts and field masks */
pub const EXP_IV_SIZE: u32 = 0x7;
pub const IV_OFFSET: kernel::ffi::c_ulong = BIT(3);
pub const IV_OFFSET_SHIFT: u32 = 3;
pub const GEN_IV: kernel::ffi::c_ulong = BIT(5);
pub const GEN_IV_SHIFT: u32 = 5;
pub const EXPLICIT_IV: kernel::ffi::c_ulong = BIT(6);
pub const EXPLICIT_IV_SHIFT: u32 = 6;
pub const SCTX_IV: kernel::ffi::c_ulong = BIT(7);
pub const SCTX_IV_SHIFT: u32 = 7;
pub const ICV_SIZE: u32 = 0x0F00;
pub const ICV_SIZE_SHIFT: u32 = 8;
pub const CHECK_ICV: kernel::ffi::c_ulong = BIT(12);
pub const CHECK_ICV_SHIFT: u32 = 12;
pub const INSERT_ICV: kernel::ffi::c_ulong = BIT(13);
pub const INSERT_ICV_SHIFT: u32 = 13;
pub const BD_SUPPRESS: kernel::ffi::c_ulong = BIT(19);
pub const BD_SUPPRESS_SHIFT: u32 = 19;

/* Generic Mode Security Context Structure [SCTX] */
#[repr(C)]
pub struct SCTX {
    /* word 0: protocol flags */
    pub proto_flags: __be32,

    /* word 1: cipher flags */
    pub cipher_flags: __be32,

    /* word 2: Extended cipher flags */
    pub ecf: __be32,
}

#[repr(C)]
pub struct SPUHEADER {
    pub mh: MHEADER,
    pub emh: u32,
    pub sa: SCTX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
