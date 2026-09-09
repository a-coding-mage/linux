/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the translated Linux headers:
// xfrm_address_t, xfrm_selector, hlist_head, AF_INET, AF_INET6, ntohl,
// htonl, jhash2, and jhash_2words.

#[inline]
pub unsafe fn __xfrm4_addr_hash(addr: *const xfrm_address_t) -> u32 {
    ntohl((*addr).a4)
}

#[inline]
pub unsafe fn __xfrm6_addr_hash(addr: *const xfrm_address_t) -> u32 {
    jhash2((*addr).a6.as_ptr() as *const u32, 4, 0)
}

#[inline]
pub unsafe fn __xfrm4_daddr_saddr_hash(
    daddr: *const xfrm_address_t,
    saddr: *const xfrm_address_t,
) -> u32 {
    let sum = (*daddr).a4 as u32 + (*saddr).a4 as u32;
    ntohl(sum as u32)
}

#[inline]
pub unsafe fn __xfrm6_daddr_saddr_hash(
    daddr: *const xfrm_address_t,
    saddr: *const xfrm_address_t,
) -> u32 {
    __xfrm6_addr_hash(daddr) ^ __xfrm6_addr_hash(saddr)
}

#[inline]
pub fn __bits2mask32(bits: u8) -> u32 {
    let mut mask32 = 0xffff_ffffu32;
    if bits == 0 {
        mask32 = 0;
    } else if bits < 32 {
        mask32 <<= 32 - bits;
    }
    mask32
}

#[inline]
pub unsafe fn __xfrm4_dpref_spref_hash(
    daddr: *const xfrm_address_t,
    saddr: *const xfrm_address_t,
    dbits: u8,
    sbits: u8,
) -> u32 {
    jhash_2words(
        ntohl((*daddr).a4) & __bits2mask32(dbits),
        ntohl((*saddr).a4) & __bits2mask32(sbits),
        0,
    )
}

#[inline]
pub unsafe fn __xfrm6_pref_hash(addr: *const xfrm_address_t, prefixlen: u8) -> u32 {
    let pdw = (prefixlen >> 5) as usize;
    let pbi = prefixlen & 0x1f;
    let mut initval = 0u32;
    if pbi != 0 {
        let mask = htonl(0xffff_ffffu32 << (32 - pbi));
        initval = ((*addr).a6[pdw] & mask) as u32;
    }
    jhash2((*addr).a6.as_ptr() as *const u32, pdw as u32, initval)
}

#[inline]
pub unsafe fn __xfrm6_dpref_spref_hash(
    daddr: *const xfrm_address_t,
    saddr: *const xfrm_address_t,
    dbits: u8,
    sbits: u8,
) -> u32 {
    __xfrm6_pref_hash(daddr, dbits) ^ __xfrm6_pref_hash(saddr, sbits)
}

#[inline]
pub unsafe fn __xfrm_dst_hash(
    daddr: *const xfrm_address_t, saddr: *const xfrm_address_t,
    reqid: u32, family: u16, hmask: u32,
) -> u32 {
    let mut h = family as u32 ^ reqid;
    match family as i32 {
        AF_INET => h ^= __xfrm4_daddr_saddr_hash(daddr, saddr),
        AF_INET6 => h ^= __xfrm6_daddr_saddr_hash(daddr, saddr),
        _ => {}
    }
    (h ^ (h >> 16)) & hmask
}

#[inline]
pub unsafe fn __xfrm_src_hash(
    daddr: *const xfrm_address_t, saddr: *const xfrm_address_t,
    family: u16, hmask: u32,
) -> u32 {
    let mut h = family as u32;
    match family as i32 {
        AF_INET => h ^= __xfrm4_daddr_saddr_hash(daddr, saddr),
        AF_INET6 => h ^= __xfrm6_daddr_saddr_hash(daddr, saddr),
        _ => {}
    }
    (h ^ (h >> 16)) & hmask
}

#[inline]
pub unsafe fn __xfrm_spi_hash(
    daddr: *const xfrm_address_t, spi: u32, proto: u8,
    family: u16, hmask: u32,
) -> u32 {
    let mut h = spi ^ proto as u32;
    match family as i32 {
        AF_INET => h ^= __xfrm4_addr_hash(daddr),
        AF_INET6 => h ^= __xfrm6_addr_hash(daddr),
        _ => {}
    }
    (h ^ (h >> 10) ^ (h >> 20)) & hmask
}

#[inline]
pub fn __xfrm_seq_hash(seq: u32, hmask: u32) -> u32 {
    let h = seq;
    (h ^ (h >> 10) ^ (h >> 20)) & hmask
}

#[inline]
pub fn __idx_hash(index: u32, hmask: u32) -> u32 {
    (index ^ (index >> 8)) & hmask
}

#[inline]
pub unsafe fn __sel_hash(
    sel: *const xfrm_selector, family: u16, hmask: u32, dbits: u8, sbits: u8,
) -> u32 {
    let daddr = &(*sel).daddr as *const xfrm_address_t;
    let saddr = &(*sel).saddr as *const xfrm_address_t;
    let mut h = 0;
    match family as i32 {
        AF_INET if (*sel).prefixlen_d < dbits || (*sel).prefixlen_s < sbits => return hmask + 1,
        AF_INET => h = __xfrm4_dpref_spref_hash(daddr, saddr, dbits, sbits),
        AF_INET6 if (*sel).prefixlen_d < dbits || (*sel).prefixlen_s < sbits => return hmask + 1,
        AF_INET6 => h = __xfrm6_dpref_spref_hash(daddr, saddr, dbits, sbits),
        _ => {}
    }
    (h ^ (h >> 16)) & hmask
}

#[inline]
pub unsafe fn __addr_hash(
    daddr: *const xfrm_address_t, saddr: *const xfrm_address_t,
    family: u16, hmask: u32, dbits: u8, sbits: u8,
) -> u32 {
    let mut h = match family as i32 {
        AF_INET => __xfrm4_dpref_spref_hash(daddr, saddr, dbits, sbits),
        AF_INET6 => __xfrm6_dpref_spref_hash(daddr, saddr, dbits, sbits),
        _ => 0,
    };
    h ^= h >> 16;
    h & hmask
}

extern "C" {
    pub fn xfrm_hash_alloc(sz: u32) -> *mut hlist_head;
    pub fn xfrm_hash_free(n: *mut hlist_head, sz: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
