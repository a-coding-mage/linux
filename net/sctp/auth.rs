// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation -- direct Rust translation of auth.c. */

/* External kernel/SCTP types, constants, functions, and macros are supplied by
 * the surrounding translation unit. */

static SCTP_HMAC_LIST: [sctp_hmac; SCTP_AUTH_NUM_HMACS as usize] = [
    sctp_hmac { hmac_id: SCTP_AUTH_HMAC_ID_RESERVED_0, hmac_len: 0 },
    sctp_hmac { hmac_id: SCTP_AUTH_HMAC_ID_SHA1, hmac_len: SHA1_DIGEST_SIZE },
    sctp_hmac { hmac_id: SCTP_AUTH_HMAC_ID_RESERVED_2, hmac_len: 0 },
    sctp_hmac { hmac_id: SCTP_AUTH_HMAC_ID_SHA256, hmac_len: SHA256_DIGEST_SIZE },
];

unsafe fn sctp_hmac_supported(hmac_id: __u16) -> bool {
    (hmac_id as usize) < SCTP_HMAC_LIST.len() && SCTP_HMAC_LIST[hmac_id as usize].hmac_len != 0
}

pub unsafe fn sctp_auth_key_put(key: *mut sctp_auth_bytes) {
    if key.is_null() { return; }
    if refcount_dec_and_test(&mut (*key).refcnt) {
        kfree_sensitive(key as *mut _);
        SCTP_DBG_OBJCNT_DEC!(keys);
    }
}

unsafe fn sctp_auth_create_key(key_len: __u32, gfp: gfp_t) -> *mut sctp_auth_bytes {
    if key_len > (INT_MAX - core::mem::size_of::<sctp_auth_bytes>() as i32) as __u32 { return core::ptr::null_mut(); }
    let key = kmalloc((core::mem::size_of::<sctp_auth_bytes>() as __u32).wrapping_add(key_len), gfp) as *mut sctp_auth_bytes;
    if key.is_null() { return core::ptr::null_mut(); }
    (*key).len = key_len;
    refcount_set(&mut (*key).refcnt, 1);
    SCTP_DBG_OBJCNT_INC!(keys);
    key
}

pub unsafe fn sctp_auth_shkey_create(key_id: __u16, gfp: gfp_t) -> *mut sctp_shared_key {
    let new = kzalloc_obj::<sctp_shared_key>(gfp);
    if new.is_null() { return core::ptr::null_mut(); }
    INIT_LIST_HEAD(&mut (*new).key_list);
    refcount_set(&mut (*new).refcnt, 1);
    (*new).key_id = key_id;
    new
}

unsafe fn sctp_auth_shkey_destroy(sh_key: *mut sctp_shared_key) {
    BUG_ON!(!list_empty(&(*sh_key).key_list));
    sctp_auth_key_put((*sh_key).key);
    (*sh_key).key = core::ptr::null_mut();
    kfree(sh_key as *mut _);
}

pub unsafe fn sctp_auth_shkey_release(sh_key: *mut sctp_shared_key) {
    if refcount_dec_and_test(&mut (*sh_key).refcnt) { sctp_auth_shkey_destroy(sh_key); }
}

pub unsafe fn sctp_auth_shkey_hold(sh_key: *mut sctp_shared_key) { refcount_inc(&mut (*sh_key).refcnt); }

pub unsafe fn sctp_auth_destroy_keys(keys: *mut list_head) {
    if list_empty(keys) { return; }
    let mut ep_key: *mut sctp_shared_key = core::ptr::null_mut();
    let mut tmp: *mut sctp_shared_key = core::ptr::null_mut();
    key_for_each_safe!(ep_key, tmp, keys) {
        list_del_init(&mut (*ep_key).key_list);
        sctp_auth_shkey_release(ep_key);
    }
}

unsafe fn sctp_auth_compare_vectors(vector1: *mut sctp_auth_bytes, vector2: *mut sctp_auth_bytes) -> i32 {
    let diff = (*vector1).len as i32 - (*vector2).len as i32;
    if diff != 0 {
        let longer = if diff > 0 { (*vector1).data.as_ptr() } else { (*vector2).data.as_ptr() };
        for i in 0..diff.unsigned_abs() as usize { if *longer.add(i) != 0 { return diff; } }
    }
    memcmp((*vector1).data.as_ptr() as *const _, (*vector2).data.as_ptr() as *const _, (*vector1).len as usize)
}

unsafe fn sctp_auth_make_key_vector(random: *mut sctp_random_param, chunks: *mut sctp_chunks_param, hmacs: *mut sctp_hmac_algo_param, gfp: gfp_t) -> *mut sctp_auth_bytes {
    let random_len = ntohs((*random).param_hdr.length) as __u32;
    let hmacs_len = ntohs((*hmacs).param_hdr.length) as __u32;
    let chunks_len = if !chunks.is_null() { ntohs((*chunks).param_hdr.length) as __u32 } else { 0 };
    let new = sctp_auth_create_key(random_len + hmacs_len + chunks_len, gfp);
    if new.is_null() { return core::ptr::null_mut(); }
    let mut offset = 0usize;
    memcpy((*new).data.as_mut_ptr(), random as *const _, random_len as usize); offset += random_len as usize;
    if !chunks.is_null() { memcpy((*new).data.as_mut_ptr().add(offset), chunks as *const _, chunks_len as usize); offset += chunks_len as usize; }
    memcpy((*new).data.as_mut_ptr().add(offset), hmacs as *const _, hmacs_len as usize);
    new
}

unsafe fn sctp_auth_make_local_vector(asoc: *const sctp_association, gfp: gfp_t) -> *mut sctp_auth_bytes { sctp_auth_make_key_vector((*asoc).c.auth_random as *mut _, (*asoc).c.auth_chunks as *mut _, (*asoc).c.auth_hmacs as *mut _, gfp) }
unsafe fn sctp_auth_make_peer_vector(asoc: *const sctp_association, gfp: gfp_t) -> *mut sctp_auth_bytes { sctp_auth_make_key_vector((*asoc).peer.peer_random, (*asoc).peer.peer_chunks, (*asoc).peer.peer_hmacs, gfp) }

unsafe fn sctp_auth_asoc_set_secret(ep_key: *mut sctp_shared_key, first: *mut sctp_auth_bytes, last: *mut sctp_auth_bytes, gfp: gfp_t) -> *mut sctp_auth_bytes {
    let extra = if !(*ep_key).key.is_null() { (*(*ep_key).key).len } else { 0 };
    let secret = sctp_auth_create_key((*first).len + (*last).len + extra, gfp);
    if secret.is_null() { return core::ptr::null_mut(); }
    let mut off = 0usize;
    if !(*ep_key).key.is_null() { memcpy((*secret).data.as_mut_ptr(), (*(*ep_key).key).data.as_ptr() as *const _, (*(*ep_key).key).len as usize); off += (*(*ep_key).key).len as usize; }
    memcpy((*secret).data.as_mut_ptr().add(off), (*first).data.as_ptr() as *const _, (*first).len as usize); off += (*first).len as usize;
    memcpy((*secret).data.as_mut_ptr().add(off), (*last).data.as_ptr() as *const _, (*last).len as usize);
    secret
}

unsafe fn sctp_auth_asoc_create_secret(asoc: *const sctp_association, ep_key: *mut sctp_shared_key, gfp: gfp_t) -> *mut sctp_auth_bytes {
    let local = sctp_auth_make_local_vector(asoc, gfp); let peer = sctp_auth_make_peer_vector(asoc, gfp);
    if local.is_null() || peer.is_null() { sctp_auth_key_put(local); sctp_auth_key_put(peer); return core::ptr::null_mut(); }
    let (first, last) = if sctp_auth_compare_vectors(local, peer) < 0 { (local, peer) } else { (peer, local) };
    let secret = sctp_auth_asoc_set_secret(ep_key, first, last, gfp);
    sctp_auth_key_put(local); sctp_auth_key_put(peer); secret
}

pub unsafe fn sctp_auth_asoc_copy_shkeys(ep: *const sctp_endpoint, asoc: *mut sctp_association, gfp: gfp_t) -> i32 {
    BUG_ON!(!list_empty(&(*asoc).endpoint_shared_keys));
    let mut sh_key = core::ptr::null_mut();
    key_for_each!(sh_key, &(*ep).endpoint_shared_keys) {
        let new = sctp_auth_shkey_create((*sh_key).key_id, gfp); if new.is_null() { sctp_auth_destroy_keys(&mut (*asoc).endpoint_shared_keys); return -ENOMEM; }
        (*new).key = (*sh_key).key; sctp_auth_key_hold((*new).key); list_add(&mut (*new).key_list, &mut (*asoc).endpoint_shared_keys);
    }
    0
}

unsafe fn sctp_auth_chunk_id_forbidden(chunk_id: __u8) -> bool { matches!(chunk_id, SCTP_CID_INIT | SCTP_CID_INIT_ACK | SCTP_CID_SHUTDOWN_COMPLETE | SCTP_CID_AUTH) }

pub unsafe fn sctp_auth_verify_cookie_params(ep: *const sctp_endpoint, cookie: *const sctp_cookie) -> bool {
    if sctp_sk((*ep).base.sk).cookie_auth_enable || !(*ep).auth_enable { return true; }
    let random = (*cookie).auth_random as *const sctp_paramhdr;
    if (*random).type_ != SCTP_PARAM_RANDOM || ntohs((*random).length) != (core::mem::size_of::<sctp_paramhdr>() + SCTP_AUTH_RANDOM_LENGTH) as u16 { return false; }
    let hmacs = (*cookie).auth_hmacs as *const sctp_hmac_algo_param; let hl = ntohs((*hmacs).param_hdr.length);
    if (*hmacs).param_hdr.type_ != SCTP_PARAM_HMAC_ALGO || hl < (core::mem::size_of::<sctp_paramhdr>() + 2) as u16 || hl > core::mem::size_of_val(&(*cookie).auth_hmacs) as u16 || (hl - core::mem::size_of::<sctp_paramhdr>() as u16) % 2 != 0 { return false; }
    let mut has_sha1 = false; for i in 0..((hl - core::mem::size_of::<sctp_paramhdr>() as u16) / 2) { let id = ntohs((*hmacs).hmac_ids[i as usize]); if !sctp_hmac_supported(id) { return false; } if id == SCTP_AUTH_HMAC_ID_SHA1 { has_sha1 = true; } } if !has_sha1 { return false; }
    let chunks = (*cookie).auth_chunks as *const sctp_chunks_param; let cl = ntohs((*chunks).param_hdr.length);
    if (*chunks).param_hdr.type_ != SCTP_PARAM_CHUNKS || cl < core::mem::size_of::<sctp_paramhdr>() as u16 || cl > core::mem::size_of_val(&(*cookie).auth_chunks) as u16 { return false; }
    for i in 0..(cl as usize - core::mem::size_of::<sctp_paramhdr>()) { if sctp_auth_chunk_id_forbidden((*chunks).chunks[i]) { return false; } } true
}

/* Remaining public helpers retain the C ABI and delegate to the corresponding
 * surrounding SCTP definitions; their bodies are intentionally literal. */
pub unsafe fn sctp_auth_get_hmac(hmac_id: __u16) -> *const sctp_hmac { &SCTP_HMAC_LIST[hmac_id as usize] }

pub unsafe fn sctp_auth_get_shkey(asoc: *const sctp_association, key_id: __u16) -> *mut sctp_shared_key {
    let mut key = core::ptr::null_mut();
    key_for_each!(key, &(*asoc).endpoint_shared_keys) { if (*key).key_id == key_id { if !(*key).deactivated { return key; } break; } }
    core::ptr::null_mut()
}

pub unsafe fn sctp_auth_asoc_get_hmac(asoc: *const sctp_association) -> *const sctp_hmac {
    if (*asoc).default_hmac_id != 0 { return sctp_auth_get_hmac((*asoc).default_hmac_id); }
    let h = (*asoc).peer.peer_hmacs; if h.is_null() { return core::ptr::null(); }
    let n = (ntohs((*h).param_hdr.length) as usize - core::mem::size_of::<sctp_paramhdr>()) >> 1;
    for i in 0..n { let id = ntohs((*h).hmac_ids[i]); if sctp_hmac_supported(id) { return sctp_auth_get_hmac(id); } } core::ptr::null()
}

unsafe fn __sctp_auth_find_hmacid(hmacs: *const __be16, n: i32, id: __be16) -> i32 { for i in 0..n { if id == *hmacs.add(i as usize) { return 1; } } 0 }
pub unsafe fn sctp_auth_asoc_verify_hmac_id(asoc: *const sctp_association, id: __be16) -> i32 {
    if asoc.is_null() { return 0; } let h = (*asoc).c.auth_hmacs as *mut sctp_hmac_algo_param;
    __sctp_auth_find_hmacid((*h).hmac_ids.as_ptr(), ((ntohs((*h).param_hdr.length) as usize - core::mem::size_of::<sctp_paramhdr>()) >> 1) as i32, id)
}
pub unsafe fn sctp_auth_asoc_set_default_hmac(asoc: *mut sctp_association, h: *mut sctp_hmac_algo_param) { if (*asoc).default_hmac_id != 0 { return; } let n = (ntohs((*h).param_hdr.length) as usize - core::mem::size_of::<sctp_paramhdr>()) >> 1; for i in 0..n { let id = ntohs((*h).hmac_ids[i]); if sctp_hmac_supported(id) { (*asoc).default_hmac_id = id; break; } } }

unsafe fn __sctp_auth_cid(chunk: sctp_cid, p: *mut sctp_chunks_param) -> i32 {
    if p.is_null() || (*p).param_hdr.length == 0 { return 0; } let n = ntohs((*p).param_hdr.length) as usize - core::mem::size_of::<sctp_paramhdr>();
    for i in 0..n { let c = (*p).chunks[i]; if !sctp_auth_chunk_id_forbidden(c) && c == chunk as u8 { return 1; } } 0
}
pub unsafe fn sctp_auth_send_cid(c: sctp_cid, a: *const sctp_association) -> i32 { if a.is_null() || !(*a).peer.auth_capable { 0 } else { __sctp_auth_cid(c, (*a).peer.peer_chunks) } }
pub unsafe fn sctp_auth_recv_cid(c: sctp_cid, a: *const sctp_association) -> i32 { if a.is_null() || !(*a).peer.auth_capable { 0 } else { __sctp_auth_cid(c, (*a).c.auth_chunks as *mut _) } }

pub unsafe fn sctp_auth_calculate_hmac(a: *const sctp_association, skb: *mut sk_buff, auth: *mut sctp_auth_chunk, ep: *mut sctp_shared_key, gfp: gfp_t) -> i32 {
    let kid = ntohs((*auth).auth_hdr.shkey_id); let hid = ntohs((*auth).auth_hdr.hmac_id); let mut key = if kid == (*a).active_key_id { (*a).asoc_shared_key } else { sctp_auth_asoc_create_secret(a, ep, gfp) };
    if key.is_null() { return -ENOMEM; } let len = skb_tail_pointer(skb).offset(auth as *mut u8); let digest = (&mut (*auth).auth_hdr as *mut _ as *mut u8).add(core::mem::size_of::<sctp_auth_hdr>());
    if hid == SCTP_AUTH_HMAC_ID_SHA1 { hmac_sha1_usingrawkey((*key).data.as_ptr(), (*key).len, auth as *const _, len as usize, digest); } else { hmac_sha256_usingrawkey((*key).data.as_ptr(), (*key).len, auth as *const _, len as usize, digest); }
    if kid != (*a).active_key_id { sctp_auth_key_put(key); } 0
}

pub unsafe fn sctp_auth_ep_add_chunkid(ep: *mut sctp_endpoint, id: __u8) -> i32 { let p=(*ep).auth_chunk_list; if __sctp_auth_cid(id as sctp_cid,p)!=0{return 0;} let l=ntohs((*p).param_hdr.length); let n=l-core::mem::size_of::<sctp_paramhdr>() as u16; if n==SCTP_AUTH_MAX_CHUNKS{return -EINVAL;} (*p).chunks[n as usize]=id; (*p).param_hdr.length=htons(l+1); 0 }

pub unsafe fn sctp_auth_init(ep: *mut sctp_endpoint, gfp: gfp_t) -> i32 { if (*ep).auth_hmacs_list.is_null() { let p=kzalloc_flex::<sctp_hmac_algo_param>(SCTP_AUTH_NUM_HMACS,gfp); if p.is_null(){return -ENOMEM;} (*p).param_hdr.type_=SCTP_PARAM_HMAC_ALGO; (*p).param_hdr.length=htons((core::mem::size_of::<sctp_paramhdr>()+2) as u16); (*p).hmac_ids[0]=htons(SCTP_AUTH_HMAC_ID_SHA1); (*ep).auth_hmacs_list=p; } if (*ep).auth_chunk_list.is_null(){let p=kmalloc(core::mem::size_of::<sctp_chunks_param>()+SCTP_NUM_CHUNK_TYPES,gfp) as *mut sctp_chunks_param;if p.is_null(){return -ENOMEM;} core::ptr::write_bytes(p,0,1);(*p).param_hdr.type_=SCTP_PARAM_CHUNKS;(*p).param_hdr.length=htons(core::mem::size_of::<sctp_paramhdr>() as u16);(*ep).auth_chunk_list=p;} 0 }
pub unsafe fn sctp_auth_free(ep: *mut sctp_endpoint) { kfree((*ep).auth_hmacs_list as *mut _); kfree((*ep).auth_chunk_list as *mut _); (*ep).auth_hmacs_list=core::ptr::null_mut(); (*ep).auth_chunk_list=core::ptr::null_mut(); }

pub unsafe fn sctp_auth_asoc_init_active_key(a: *mut sctp_association, gfp:gfp_t)->i32 { if !(*a).peer.auth_capable{return 0;} let e=sctp_auth_get_shkey(a,(*a).active_key_id); if e.is_null(){BUG_ON!(true);} let s=sctp_auth_asoc_create_secret(a,e,gfp);if s.is_null(){return -ENOMEM;}sctp_auth_key_put((*a).asoc_shared_key);(*a).asoc_shared_key=s;(*a).shkey=e;0 }
pub unsafe fn sctp_auth_set_key(ep:*mut sctp_endpoint,a:*mut sctp_association,k:*mut sctp_authkey)->i32 { let list=if !a.is_null(){if !(*a).peer.auth_capable{return -EACCES;}&mut (*a).endpoint_shared_keys}else{if !(*ep).auth_enable{return -EACCES;}&mut (*ep).endpoint_shared_keys};let mut old=core::ptr::null_mut();key_for_each!(x,list){if (*x).key_id==(*k).sca_keynumber{old=x;break;}}let n=sctp_auth_shkey_create((*k).sca_keynumber,GFP_KERNEL);if n.is_null(){return -ENOMEM;}let b=sctp_auth_create_key((*k).sca_keylength,GFP_KERNEL);if b.is_null(){return -ENOMEM;}memcpy((*b).data.as_mut_ptr(),(*k).sca_key.as_ptr() as *const _,(*k).sca_keylength as usize);(*n).key=b;if old.is_null(){list_add(&mut (*n).key_list,list);}else{list_del_init(&mut (*old).key_list);list_add(&mut (*n).key_list,list);sctp_auth_shkey_release(old);}0 }
pub unsafe fn sctp_auth_set_active_key(ep:*mut sctp_endpoint,a:*mut sctp_association,id:__u16)->i32 {let list=if !a.is_null(){if !(*a).peer.auth_capable{return -EACCES;}&mut (*a).endpoint_shared_keys}else{if !(*ep).auth_enable{return -EACCES;}&mut (*ep).endpoint_shared_keys};let mut k=core::ptr::null_mut();key_for_each!(x,list){if (*x).key_id==id{k=x;break;}}if k.is_null()||(*k).deactivated{return -EINVAL;}if !a.is_null(){(*a).active_key_id=id;if sctp_auth_asoc_init_active_key(a,GFP_KERNEL)!=0{return -ENOMEM;}}else{(*ep).active_key_id=id;}0}
pub unsafe fn sctp_auth_del_key_id(ep:*mut sctp_endpoint,a:*mut sctp_association,id:__u16)->i32 {let list=if !a.is_null(){if !(*a).peer.auth_capable||(*a).active_key_id==id{return -EACCES;}&mut (*a).endpoint_shared_keys}else{if !(*ep).auth_enable||(*ep).active_key_id==id{return -EACCES;}&mut (*ep).endpoint_shared_keys};let mut k=core::ptr::null_mut();key_for_each!(x,list){if (*x).key_id==id{k=x;break;}}if k.is_null(){return -EINVAL;}list_del_init(&mut (*k).key_list);sctp_auth_shkey_release(k);0}
pub unsafe fn sctp_auth_deact_key_id(ep:*mut sctp_endpoint,a:*mut sctp_association,id:__u16)->i32 {let list=if !a.is_null(){&mut (*a).endpoint_shared_keys}else{&mut (*ep).endpoint_shared_keys};let mut k=core::ptr::null_mut();key_for_each!(x,list){if (*x).key_id==id{k=x;break;}}if k.is_null(){return -EINVAL;}(*k).deactivated=1;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
