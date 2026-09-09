// SPDX-License-Identifier: GPL-2.0-only

// External kernel dependencies used by this translation are supplied by other files.

pub const NFT_PIPAPO_MAX_FIELDS: usize = NFT_REG32_COUNT as usize;
pub const NFT_PIPAPO_MIN_FIELDS: usize = 2;
pub const NFT_PIPAPO_MAX_BYTES: usize = core::mem::size_of::<In6Addr>();
pub const NFT_PIPAPO_MAX_BITS: usize = NFT_PIPAPO_MAX_BYTES * BITS_PER_BYTE as usize;
pub const NFT_PIPAPO_GROUP_BITS_INIT: usize = NFT_PIPAPO_GROUP_BITS_SMALL_SET;
pub const NFT_PIPAPO_GROUP_BITS_SMALL_SET: usize = 8;
pub const NFT_PIPAPO_GROUP_BITS_LARGE_SET: usize = 4;
pub const NFT_PIPAPO_LT_SIZE_THRESHOLD: usize = 1 << 21;
pub const NFT_PIPAPO_LT_SIZE_HYSTERESIS: usize = 1 << 16;
pub const NFT_PIPAPO_LT_SIZE_HIGH: usize = NFT_PIPAPO_LT_SIZE_THRESHOLD;
pub const NFT_PIPAPO_LT_SIZE_LOW: usize = NFT_PIPAPO_LT_SIZE_THRESHOLD - NFT_PIPAPO_LT_SIZE_HYSTERESIS;
pub const NFT_PIPAPO_ALIGN_HEADROOM: usize = 0;
pub const NFT_PIPAPO_MAP_NBITS: usize = const_ilog2(NFT_PIPAPO_MAX_BITS * 2);
pub const NFT_PIPAPO_MAP_TOBITS: usize = 32;
pub const NFT_PIPAPO_RULE0_MAX: usize = (1usize << (NFT_PIPAPO_MAP_TOBITS - 1)) - (1usize << NFT_PIPAPO_MAP_NBITS);

#[inline]
pub const fn nft_pipapo_groups_per_byte(f: &nft_pipapo_field) -> usize {
    BITS_PER_BYTE as usize / f.bb as usize
}

#[inline]
pub const fn nft_pipapo_buckets(bb: usize) -> usize { 1usize << bb }

#[repr(C)]
pub union nft_pipapo_map_bucket {
    pub parts: MapBucketParts,
    pub e: *mut nft_pipapo_elem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MapBucketParts { pub to: u32, pub n: u32 }

#[repr(C)]
pub struct nft_pipapo_field {
    pub rules: u32,
    pub bsize: u32,
    pub rules_alloc: u32,
    pub groups: u8,
    pub bb: u8,
    pub lt: *mut libc_ulong,
    pub mt: *mut nft_pipapo_map_bucket,
}

#[repr(C)]
pub struct nft_pipapo_scratch {
    pub bh_lock: local_lock_t,
    pub map_index: u8,
    pub __map: [libc_ulong; 0],
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum nft_pipapo_clone_state {
    NFT_PIPAPO_CLONE_NEW,
    NFT_PIPAPO_CLONE_MOD,
    NFT_PIPAPO_CLONE_ERR,
}

#[repr(C)]
pub struct nft_pipapo_match {
    pub field_count: u8,
    pub state: nft_pipapo_clone_state,
    pub bsize_max: u32,
    pub scratch: *mut *mut nft_pipapo_scratch,
    pub rcu: rcu_head,
    pub f: [nft_pipapo_field; 0],
}

#[repr(C)]
pub struct nft_pipapo {
    pub r#match: *mut nft_pipapo_match,
    pub clone: *mut nft_pipapo_match,
    pub width: i32,
    pub last_gc: libc_ulong,
    pub gc_head: list_head,
}

#[repr(C)]
pub struct nft_pipapo_elem {
    pub priv_: nft_elem_priv,
    pub ext: nft_set_ext,
}

extern "C" {
    pub fn pipapo_refill(map: *mut libc_ulong, len: u32, rules: u32, dst: *mut libc_ulong,
                          mt: *const nft_pipapo_map_bucket, match_only: bool) -> i32;
}

#[inline]
pub unsafe fn pipapo_and_field_buckets_4bit(f: *const nft_pipapo_field, dst: *mut libc_ulong, mut data: *const u8) {
    let mut lt = (*f).lt;
    let mut group = 0;
    while group < (*f).groups as i32 {
        let v = *data >> 4;
        __bitmap_and(dst, dst, lt.add(v as usize * (*f).bsize as usize), (*f).bsize as usize * BITS_PER_LONG as usize);
        lt = lt.add((*f).bsize as usize * nft_pipapo_buckets(4));
        let v = *data & 0x0f;
        __bitmap_and(dst, dst, lt.add(v as usize * (*f).bsize as usize), (*f).bsize as usize * BITS_PER_LONG as usize);
        lt = lt.add((*f).bsize as usize * nft_pipapo_buckets(4));
        data = data.add(1);
        group += BITS_PER_BYTE as i32 / 4;
    }
}

#[inline]
pub unsafe fn pipapo_and_field_buckets_8bit(f: *const nft_pipapo_field, dst: *mut libc_ulong, mut data: *const u8) {
    let mut lt = (*f).lt;
    for _ in 0..(*f).groups {
        __bitmap_and(dst, dst, lt.add((*data as usize) * (*f).bsize as usize), (*f).bsize as usize * BITS_PER_LONG as usize);
        lt = lt.add((*f).bsize as usize * nft_pipapo_buckets(8));
        data = data.add(1);
    }
}

pub unsafe fn pipapo_estimate_size(desc: *const nft_set_desc) -> u64 {
    let mut entry_size: usize = 0;
    for i in 0..(*desc).field_count as usize {
        if (*desc).field_len[i] as usize > NFT_PIPAPO_MAX_BYTES { return 0; }
        let rules = ilog2((*desc).field_len[i] as usize * BITS_PER_BYTE as usize) * 2;
        entry_size += rules * nft_pipapo_buckets(NFT_PIPAPO_GROUP_BITS_INIT) / BITS_PER_BYTE as usize;
        entry_size += rules * core::mem::size_of::<nft_pipapo_map_bucket>();
    }
    let mut size = (*desc).size as u64 * entry_size as u64;
    if size != 0 && size / (*desc).size as u64 != entry_size as u64 { return 0; }
    size += (core::mem::size_of::<nft_pipapo>() + core::mem::size_of::<nft_pipapo_match>() * 2) as u64;
    size += (core::mem::size_of::<nft_pipapo_field>() * (*desc).field_count as usize) as u64;
    size
}

#[inline]
pub unsafe fn pipapo_resmap_init(m: *const nft_pipapo_match, res_map: *mut libc_ulong) {
    let f = (*m).f.as_ptr();
    for i in 0..(*f).bsize { *res_map.add(i as usize) = libc_ulong::MAX; }
    for i in (*f).bsize..(*m).bsize_max { *res_map.add(i as usize) = 0; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
