/* SPDX-License-Identifier: GPL-2.0-or-later */
/* CRC folding constants translated from crc-pclmul-consts.h. */

#[repr(C, align(64))]
pub struct Crc16Msb0x8bb7Consts {
    pub bswap_mask: [u8; 16],
    pub fold_across_2048_bits_consts: [u64; 2],
    pub fold_across_1024_bits_consts: [u64; 2],
    pub fold_across_512_bits_consts: [u64; 2],
    pub fold_across_256_bits_consts: [u64; 2],
    pub fold_across_128_bits_consts: [u64; 2],
    pub shuf_table: [i8; 48],
    pub barrett_reduction_consts: [u64; 2],
}

pub static crc16_msb_0x8bb7_consts: Crc16Msb0x8bb7Consts = Crc16Msb0x8bb7Consts {
    bswap_mask: [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    fold_across_2048_bits_consts: [0xdccf000000000000, 0x4b0b000000000000],
    fold_across_1024_bits_consts: [0x9d9d000000000000, 0x7cf5000000000000],
    fold_across_512_bits_consts: [0x044c000000000000, 0xe658000000000000],
    fold_across_256_bits_consts: [0x6ee3000000000000, 0xe7b5000000000000],
    fold_across_128_bits_consts: [0x2d56000000000000, 0x06df000000000000],
    shuf_table: [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    barrett_reduction_consts: [0x8bb7000000000000, 0xf65a57f81d33a48a],
};

#[repr(C, align(64))]
pub struct Crc32Consts {
    pub fold_across_2048_bits_consts: [u64; 2],
    pub fold_across_1024_bits_consts: [u64; 2],
    pub fold_across_512_bits_consts: [u64; 2],
    pub fold_across_256_bits_consts: [u64; 2],
    pub fold_across_128_bits_consts: [u64; 2],
    pub shuf_table: [i8; 48],
    pub barrett_reduction_consts: [u64; 2],
}

const SHUF_TABLE: [i8; 48] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

pub static crc32_lsb_0xedb88320_consts: Crc32Consts = Crc32Consts {
    fold_across_2048_bits_consts: [0x00000000ce3371cb, 0x00000000e95c1271],
    fold_across_1024_bits_consts: [0x0000000033fff533, 0x00000000910eeec1],
    fold_across_512_bits_consts: [0x000000008f352d95, 0x000000001d9513d7],
    fold_across_256_bits_consts: [0x00000000f1da05aa, 0x0000000081256527],
    fold_across_128_bits_consts: [0x00000000ae689191, 0x00000000ccaa009e],
    shuf_table: SHUF_TABLE,
    barrett_reduction_consts: [0xb4e5b025f7011641, 0x00000001db710640],
};

pub static crc32_lsb_0x82f63b78_consts: Crc32Consts = Crc32Consts {
    fold_across_2048_bits_consts: [0x00000000dcb17aa4, 0x00000000b9e02b86],
    fold_across_1024_bits_consts: [0x000000006992cea2, 0x000000000d3b6092],
    fold_across_512_bits_consts: [0x00000000740eef02, 0x000000009e4addf8],
    fold_across_256_bits_consts: [0x000000003da6d0cb, 0x00000000ba4fc28e],
    fold_across_128_bits_consts: [0x00000000f20c0dfe, 0x00000000493c7d27],
    shuf_table: SHUF_TABLE,
    barrett_reduction_consts: [0x4869ec38dea713f1, 0x0000000105ec76f0],
};

#[repr(C, align(64))]
pub struct Crc64MsbConsts {
    pub bswap_mask: [u8; 16],
    pub fold_across_2048_bits_consts: [u64; 2],
    pub fold_across_1024_bits_consts: [u64; 2],
    pub fold_across_512_bits_consts: [u64; 2],
    pub fold_across_256_bits_consts: [u64; 2],
    pub fold_across_128_bits_consts: [u64; 2],
    pub shuf_table: [i8; 48],
    pub barrett_reduction_consts: [u64; 2],
}

pub static crc64_msb_0x42f0e1eba9ea3693_consts: Crc64MsbConsts = Crc64MsbConsts {
    bswap_mask: [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    fold_across_2048_bits_consts: [0x7f52691a60ddc70d, 0x7036b0389f6a0c82],
    fold_across_1024_bits_consts: [0x05cf79dea9ac37d6, 0x001067e571d7d5c2],
    fold_across_512_bits_consts: [0x5f6843ca540df020, 0xddf4b6981205b83f],
    fold_across_256_bits_consts: [0x571bee0a227ef92b, 0x44bef2a201b5200c],
    fold_across_128_bits_consts: [0x05f5c3c7eb52fab6, 0x4eb938a7d257740e],
    shuf_table: SHUF_TABLE,
    barrett_reduction_consts: [0x42f0e1eba9ea3693, 0x578d29d06cc4f872],
};

#[repr(C, align(64))]
pub struct Crc64LsbConsts {
    pub fold_across_2048_bits_consts: [u64; 2],
    pub fold_across_1024_bits_consts: [u64; 2],
    pub fold_across_512_bits_consts: [u64; 2],
    pub fold_across_256_bits_consts: [u64; 2],
    pub fold_across_128_bits_consts: [u64; 2],
    pub shuf_table: [i8; 48],
    pub barrett_reduction_consts: [u64; 2],
}

pub static crc64_lsb_0x9a6c9329ac4bc9b5_consts: Crc64LsbConsts = Crc64LsbConsts {
    fold_across_2048_bits_consts: [0x37ccd3e14069cabc, 0xa043808c0f782663],
    fold_across_1024_bits_consts: [0xa1ca681e733f9c40, 0x5f852fb61e8d92dc],
    fold_across_512_bits_consts: [0x0c32cdb31e18a84a, 0x62242240ace5045a],
    fold_across_256_bits_consts: [0xb0bc2e589204f500, 0xe1e0bb9d45d7a44c],
    fold_across_128_bits_consts: [0xeadc41fd2ba3d420, 0x21e9761e252621ac],
    shuf_table: SHUF_TABLE,
    barrett_reduction_consts: [0x27ecfa329aef9f77, 0x34d926535897936a],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
