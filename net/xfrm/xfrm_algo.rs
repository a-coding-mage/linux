// SPDX-License-Identifier: GPL-2.0-or-later
/* xfrm algorithm interface */

// The kernel headers and crypto providers are supplied by the surrounding translation.

static mut AEAD_LIST: [xfrm_algo_desc; 8] = [
    xfrm_algo_desc { name: cstr!("rfc4106(gcm(aes))"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 64), pfkey_supported: 1, desc: xfrm_algo_desc_info { sadb_alg_id: SADB_X_EALG_AES_GCM_ICV8, sadb_alg_ivlen: 8, sadb_alg_minbits: 128, sadb_alg_maxbits: 256 }, available: 0 },
    xfrm_algo_desc { name: cstr!("rfc4106(gcm(aes))"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 96), pfkey_supported: 1, desc: xfrm_algo_desc_info { sadb_alg_id: SADB_X_EALG_AES_GCM_ICV12, sadb_alg_ivlen: 8, sadb_alg_minbits: 128, sadb_alg_maxbits: 256 }, available: 0 },
    xfrm_algo_desc { name: cstr!("rfc4106(gcm(aes))"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 128), pfkey_supported: 1, desc: xfrm_algo_desc_info { sadb_alg_id: SADB_X_EALG_AES_GCM_ICV16, sadb_alg_ivlen: 8, sadb_alg_minbits: 128, sadb_alg_maxbits: 256 }, available: 0 },
    xfrm_algo_desc { name: cstr!("rfc4309(ccm(aes))"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 64), pfkey_supported: 1, desc: xfrm_algo_desc_info { sadb_alg_id: SADB_X_EALG_AES_CCM_ICV8, sadb_alg_ivlen: 8, sadb_alg_minbits: 128, sadb_alg_maxbits: 256 }, available: 0 },
    xfrm_algo_desc { name: cstr!("rfc4309(ccm(aes))"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 96), pfkey_supported: 1, desc: xfrm_algo_desc_info { sadb_alg_id: SADB_X_EALG_AES_CCM_ICV12, sadb_alg_ivlen: 8, sadb_alg_minbits: 128, sadb_alg_maxbits: 256 }, available: 0 },
    xfrm_algo_desc { name: cstr!("rfc4309(ccm(aes))"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 128), pfkey_supported: 1, desc: xfrm_algo_desc_info { sadb_alg_id: SADB_X_EALG_AES_CCM_ICV16, sadb_alg_ivlen: 8, sadb_alg_minbits: 128, sadb_alg_maxbits: 256 }, available: 0 },
    xfrm_algo_desc { name: cstr!("rfc4543(gcm(aes))"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 128), pfkey_supported: 1, desc: xfrm_algo_desc_info { sadb_alg_id: SADB_X_EALG_NULL_AES_GMAC, sadb_alg_ivlen: 8, sadb_alg_minbits: 128, sadb_alg_maxbits: 256 }, available: 0 },
    xfrm_algo_desc { name: cstr!("rfc7539esp(chacha20,poly1305)"), compat: core::ptr::null(), uinfo: xfrm_algo_uinfo::aead("seqiv", 128), pfkey_supported: 0, desc: xfrm_algo_desc_info::default(), available: 0 },
];

static mut AALG_LIST: [xfrm_algo_desc; 9] = [
    xfrm_algo_desc::auth("digest_null", core::ptr::null(), 0, 0, 1, SADB_X_AALG_NULL, 0, 0, 0),
    xfrm_algo_desc::auth("hmac(md5)", cstr!("md5"), 96, 128, 1, SADB_AALG_MD5HMAC, 0, 128, 128),
    xfrm_algo_desc::auth("hmac(sha1)", cstr!("sha1"), 96, 160, 1, SADB_AALG_SHA1HMAC, 0, 160, 160),
    xfrm_algo_desc::auth("hmac(sha256)", cstr!("sha256"), 96, 256, 1, SADB_X_AALG_SHA2_256HMAC, 0, 256, 256),
    xfrm_algo_desc::auth("hmac(sha384)", core::ptr::null(), 192, 384, 1, SADB_X_AALG_SHA2_384HMAC, 0, 384, 384),
    xfrm_algo_desc::auth("hmac(sha512)", core::ptr::null(), 256, 512, 1, SADB_X_AALG_SHA2_512HMAC, 0, 512, 512),
    xfrm_algo_desc::auth("xcbc(aes)", core::ptr::null(), 96, 128, 1, SADB_X_AALG_AES_XCBC_MAC, 0, 128, 128),
    xfrm_algo_desc::auth("cmac(aes)", core::ptr::null(), 96, 128, 0, 0, 0, 0, 0),
    xfrm_algo_desc::auth("hmac(sm3)", cstr!("sm3"), 256, 256, 1, SADB_X_AALG_SM3_256HMAC, 0, 256, 256),
];

static mut EALG_LIST: [xfrm_algo_desc; 12] = [
    xfrm_algo_desc::encr("ecb(cipher_null)", cstr!("cipher_null"), 8, 0, 1, SADB_EALG_NULL, 0, 0, 0),
    xfrm_algo_desc::encr_geniv("cbc(des)", cstr!("des"), "echainiv", 64, 64, 1, SADB_EALG_DESCBC, 8, 64, 64),
    xfrm_algo_desc::encr_geniv("cbc(des3_ede)", cstr!("des3_ede"), "echainiv", 64, 192, 1, SADB_EALG_3DESCBC, 8, 192, 192),
    xfrm_algo_desc::encr_geniv("cbc(cast5)", cstr!("cast5"), "echainiv", 64, 128, 1, SADB_X_EALG_CASTCBC, 8, 40, 128),
    xfrm_algo_desc::encr_geniv("cbc(blowfish)", cstr!("blowfish"), "echainiv", 64, 128, 1, SADB_X_EALG_BLOWFISHCBC, 8, 40, 448),
    xfrm_algo_desc::encr_geniv("cbc(aes)", cstr!("aes"), "echainiv", 128, 128, 1, SADB_X_EALG_AESCBC, 8, 128, 256),
    xfrm_algo_desc::encr_geniv("cbc(serpent)", cstr!("serpent"), "echainiv", 128, 128, 1, SADB_X_EALG_SERPENTCBC, 8, 128, 256),
    xfrm_algo_desc::encr_geniv("cbc(camellia)", cstr!("camellia"), "echainiv", 128, 128, 1, SADB_X_EALG_CAMELLIACBC, 8, 128, 256),
    xfrm_algo_desc::encr_geniv("cbc(twofish)", cstr!("twofish"), "echainiv", 128, 128, 1, SADB_X_EALG_TWOFISHCBC, 8, 128, 256),
    xfrm_algo_desc::encr_geniv("rfc3686(ctr(aes))", core::ptr::null(), "seqiv", 128, 160, 1, SADB_X_EALG_AESCTR, 8, 160, 288),
    xfrm_algo_desc::encr_geniv("cbc(sm4)", cstr!("sm4"), "echainiv", 128, 128, 1, SADB_X_EALG_SM4CBC, 16, 128, 256),
];

static mut CALG_LIST: [xfrm_algo_desc; 3] = [
    xfrm_algo_desc::comp("deflate", 90, 1, SADB_X_CALG_DEFLATE),
    xfrm_algo_desc::comp("lzs", 90, 1, SADB_X_CALG_LZS),
    xfrm_algo_desc::comp("lzjh", 50, 1, SADB_X_CALG_LZJH),
];

#[inline]
fn aalg_entries() -> i32 { AALG_LIST.len() as i32 }
#[inline]
fn ealg_entries() -> i32 { EALG_LIST.len() as i32 }
#[inline]
fn calg_entries() -> i32 { CALG_LIST.len() as i32 }

#[repr(C)]
struct xfrm_algo_list {
    find: unsafe extern "C" fn(*const c_char, u32, u32) -> i32,
    algs: *mut xfrm_algo_desc,
    entries: i32,
}

static XFRM_AEAD_LIST: xfrm_algo_list = xfrm_algo_list { find: crypto_has_aead, algs: unsafe { AEAD_LIST.as_mut_ptr() }, entries: 8 };
static XFRM_AALG_LIST: xfrm_algo_list = xfrm_algo_list { find: crypto_has_ahash, algs: unsafe { AALG_LIST.as_mut_ptr() }, entries: 9 };
static XFRM_EALG_LIST: xfrm_algo_list = xfrm_algo_list { find: crypto_has_skcipher, algs: unsafe { EALG_LIST.as_mut_ptr() }, entries: 11 };
static XFRM_CALG_LIST: xfrm_algo_list = xfrm_algo_list { find: crypto_has_acomp, algs: unsafe { CALG_LIST.as_mut_ptr() }, entries: 3 };

unsafe fn xfrm_find_algo(list: *const xfrm_algo_list, matcher: unsafe fn(*const xfrm_algo_desc, *const c_void) -> i32, data: *const c_void, probe: i32) -> *mut xfrm_algo_desc {
    let l = &*list;
    for i in 0..l.entries {
        let entry = l.algs.add(i as usize);
        if matcher(entry, data) == 0 { continue; }
        if (*entry).available != 0 { return entry; }
        if probe == 0 { break; }
        let status = (l.find)((*entry).name, 0, 0);
        if status == 0 { break; }
        (*entry).available = status;
        return entry;
    }
    core::ptr::null_mut()
}

unsafe fn xfrm_alg_id_match(entry: *const xfrm_algo_desc, data: *const c_void) -> i32 { ((*entry).desc.sadb_alg_id == data as usize) as i32 }
pub unsafe fn xfrm_aalg_get_byid(id: i32) -> *mut xfrm_algo_desc { xfrm_find_algo(&XFRM_AALG_LIST, xfrm_alg_id_match, id as usize as *const c_void, 1) }
pub unsafe fn xfrm_ealg_get_byid(id: i32) -> *mut xfrm_algo_desc { xfrm_find_algo(&XFRM_EALG_LIST, xfrm_alg_id_match, id as usize as *const c_void, 1) }
pub unsafe fn xfrm_calg_get_byid(id: i32) -> *mut xfrm_algo_desc { xfrm_find_algo(&XFRM_CALG_LIST, xfrm_alg_id_match, id as usize as *const c_void, 1) }

unsafe fn xfrm_alg_name_match(entry: *const xfrm_algo_desc, data: *const c_void) -> i32 { (!data.is_null() && (strcmp(data as *const c_char, (*entry).name) == 0 || (!(*entry).compat.is_null() && strcmp(data as *const c_char, (*entry).compat) == 0))) as i32 }
pub unsafe fn xfrm_aalg_get_byname(name: *const c_char, probe: i32) -> *mut xfrm_algo_desc { xfrm_find_algo(&XFRM_AALG_LIST, xfrm_alg_name_match, name as *const c_void, probe) }
pub unsafe fn xfrm_ealg_get_byname(name: *const c_char, probe: i32) -> *mut xfrm_algo_desc { xfrm_find_algo(&XFRM_EALG_LIST, xfrm_alg_name_match, name as *const c_void, probe) }
pub unsafe fn xfrm_calg_get_byname(name: *const c_char, probe: i32) -> *mut xfrm_algo_desc { xfrm_find_algo(&XFRM_CALG_LIST, xfrm_alg_name_match, name as *const c_void, probe) }

#[repr(C)] struct xfrm_aead_name { name: *const c_char, icvbits: i32 }
unsafe fn xfrm_aead_name_match(entry: *const xfrm_algo_desc, data: *const c_void) -> i32 { let a = &*(data as *const xfrm_aead_name); (a.icvbits == (*entry).uinfo.aead.icv_truncbits && !a.name.is_null() && strcmp(a.name, (*entry).name) == 0) as i32 }
pub unsafe fn xfrm_aead_get_byname(name: *const c_char, icv_len: i32, probe: i32) -> *mut xfrm_algo_desc { let data = xfrm_aead_name { name, icvbits: icv_len }; xfrm_find_algo(&XFRM_AEAD_LIST, xfrm_aead_name_match, &data as *const _ as *const c_void, probe) }
pub unsafe fn xfrm_aalg_get_byidx(idx: u32) -> *mut xfrm_algo_desc { if idx >= AALG_LIST.len() as u32 { core::ptr::null_mut() } else { AALG_LIST.as_mut_ptr().add(idx as usize) } }
pub unsafe fn xfrm_ealg_get_byidx(idx: u32) -> *mut xfrm_algo_desc { if idx >= EALG_LIST.len() as u32 { core::ptr::null_mut() } else { EALG_LIST.as_mut_ptr().add(idx as usize) } }

pub unsafe fn xfrm_probe_algs() {
    BUG_ON(in_softirq());
    for i in 0..AALG_LIST.len() { let status = crypto_has_ahash(AALG_LIST[i].name, 0, 0); AALG_LIST[i].available = status; }
    for i in 0..EALG_LIST.len() { let status = crypto_has_skcipher(EALG_LIST[i].name, 0, 0); EALG_LIST[i].available = status; }
    for i in 0..CALG_LIST.len() { let status = crypto_has_acomp(CALG_LIST[i].name, 0, 0); CALG_LIST[i].available = status; }
}
pub unsafe fn xfrm_count_pfkey_auth_supported() -> i32 { AALG_LIST.iter().filter(|x| x.available != 0 && x.pfkey_supported != 0).count() as i32 }
pub unsafe fn xfrm_count_pfkey_enc_supported() -> i32 { EALG_LIST.iter().filter(|x| x.available != 0 && x.pfkey_supported != 0).count() as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
