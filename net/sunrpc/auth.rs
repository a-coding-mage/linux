// SPDX-License-Identifier: GPL-2.0-only
/* Generic RPC client authentication API. */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const RPC_CREDCACHE_DEFAULT_HASHBITS: u32 = 4;
const MAX_HASHTABLE_BITS: u32 = 14;
const RPC_AUTH_EXPIRY_MORATORIUM: u64 = 60 * HZ as u64;

#[repr(C)]
pub struct rpc_cred_cache {
    pub hashtable: *mut hlist_head,
    pub hashbits: u32,
    pub lock: spinlock_t,
}

static mut auth_hashbits: u32 = RPC_CREDCACHE_DEFAULT_HASHBITS;
static mut auth_flavors: [*const rpc_authops; RPC_AUTH_MAXFLAVOR as usize] = [core::ptr::null(); RPC_AUTH_MAXFLAVOR as usize];
static mut cred_unused: list_head = LIST_HEAD_INIT;
static mut number_cred_unused: c_ulong = 0;
static mut machine_cred: cred = cred { usage: ATOMIC_INIT(1) };

#[no_mangle]
pub unsafe extern "C" fn rpc_machine_cred() -> *const cred { &machine_cred }

unsafe fn pseudoflavor_to_flavor(flavor: u32) -> u32 {
    if flavor > RPC_AUTH_MAXFLAVOR { RPC_AUTH_GSS } else { flavor }
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_register(ops: *const rpc_authops) -> c_int {
    let flavor = (*ops).au_flavor;
    if flavor >= RPC_AUTH_MAXFLAVOR { return -EINVAL; }
    let old = cmpxchg_auth(&mut auth_flavors[flavor as usize], core::ptr::null(), ops);
    if old.is_null() || old == ops { 0 } else { -EPERM }
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_unregister(ops: *const rpc_authops) -> c_int {
    let flavor = (*ops).au_flavor;
    if flavor >= RPC_AUTH_MAXFLAVOR { return -EINVAL; }
    let old = cmpxchg_auth(&mut auth_flavors[flavor as usize], ops, core::ptr::null());
    if old == ops || old.is_null() { 0 } else { -EPERM }
}

unsafe fn rpcauth_get_authops(flavor: u32) -> *const rpc_authops {
    if flavor >= RPC_AUTH_MAXFLAVOR { return core::ptr::null(); }
    rcu_read_lock();
    let mut ops = rcu_dereference(auth_flavors[flavor as usize]);
    if ops.is_null() {
        rcu_read_unlock();
        request_module(b"rpc-auth-%u\0".as_ptr() as *const c_char, flavor);
        rcu_read_lock();
        ops = rcu_dereference(auth_flavors[flavor as usize]);
        if ops.is_null() { rcu_read_unlock(); return ops; }
    }
    if !try_module_get((*ops).owner) { ops = core::ptr::null(); }
    rcu_read_unlock(); ops
}

unsafe fn rpcauth_put_authops(ops: *const rpc_authops) { module_put((*ops).owner); }

#[no_mangle]
pub unsafe extern "C" fn rpcauth_get_pseudoflavor(flavor: u32, info: *mut rpcsec_gss_info) -> u32 {
    let ops = rpcauth_get_authops(flavor);
    if ops.is_null() { return RPC_AUTH_MAXFLAVOR; }
    let mut pf = flavor;
    if let Some(f) = (*ops).info2flavor { pf = f(info); }
    rpcauth_put_authops(ops); pf
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_get_gssinfo(pseudoflavor: u32, info: *mut rpcsec_gss_info) -> c_int {
    let ops = rpcauth_get_authops(pseudoflavor_to_flavor(pseudoflavor));
    if ops.is_null() { return -ENOENT; }
    let mut result = -ENOENT;
    if let Some(f) = (*ops).flavor2info { result = f(pseudoflavor, info); }
    rpcauth_put_authops(ops); result
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_create(args: *const rpc_auth_create_args, clnt: *mut rpc_clnt) -> *mut rpc_auth {
    let flavor = pseudoflavor_to_flavor((*args).pseudoflavor);
    let ops = rpcauth_get_authops(flavor);
    if ops.is_null() { return ERR_PTR(-EINVAL); }
    let auth = ((*ops).create)(args, clnt);
    rpcauth_put_authops(ops);
    if IS_ERR(auth) { return auth; }
    if !(*clnt).cl_auth.is_null() { rpcauth_release((*clnt).cl_auth); }
    (*clnt).cl_auth = auth; auth
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_release(auth: *mut rpc_auth) {
    if !refcount_dec_and_test(&mut (*auth).au_count) { return; }
    ((*auth).au_ops.unwrap()).destroy(auth);
}

unsafe fn rpcauth_unhash_cred_locked(cred: *mut rpc_cred) -> bool {
    if !test_and_clear_bit(RPCAUTH_CRED_HASHED, &mut (*cred).cr_flags) { return false; }
    hlist_del_rcu(&mut (*cred).cr_hash); true
}
unsafe fn rpcauth_unhash_cred(cred: *mut rpc_cred) -> bool {
    if !test_bit(RPCAUTH_CRED_HASHED, &(*cred).cr_flags) { return false; }
    let lock = &mut (*(*cred).cr_auth).au_credcache.as_mut().unwrap().lock;
    spin_lock(lock); let ret = rpcauth_unhash_cred_locked(cred); spin_unlock(lock); ret
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_init_credcache(auth: *mut rpc_auth) -> c_int {
    let new = kmalloc_obj::<rpc_cred_cache>();
    if new.is_null() { return -ENOMEM; }
    (*new).hashbits = auth_hashbits;
    let size = 1usize << (*new).hashbits;
    (*new).hashtable = kzalloc_hlist(size);
    if (*new).hashtable.is_null() { kfree(new as *mut c_void); return -ENOMEM; }
    spin_lock_init(&mut (*new).lock); (*auth).au_credcache = Some(Box::from_raw(new)); 0
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_stringify_acceptor(cred: *mut rpc_cred) -> *mut c_char {
    match (*(*cred).cr_ops).crstringify_acceptor { Some(f) => f(cred), None => core::ptr::null_mut() }
}

unsafe fn rpcauth_destroy_credlist(head: *mut list_head) {
    while !list_empty(head) {
        let cred = list_entry((*head).next, rpc_cred, cr_lru);
        list_del_init(&mut (*cred).cr_lru); put_rpccred(cred);
    }
}
unsafe fn rpcauth_lru_add_locked(cred: *mut rpc_cred) { if list_empty(&(*cred).cr_lru) { number_cred_unused += 1; list_add_tail(&mut (*cred).cr_lru, &mut cred_unused); } }
unsafe fn rpcauth_lru_remove_locked(cred: *mut rpc_cred) { if !list_empty(&(*cred).cr_lru) { number_cred_unused -= 1; list_del_init(&mut (*cred).cr_lru); } }

#[no_mangle]
pub unsafe extern "C" fn rpcauth_clear_credcache(cache: *mut rpc_cred_cache) {
    let mut free = LIST_HEAD_INIT; let n = 1usize << (*cache).hashbits;
    spin_lock(&mut rpc_credcache_lock); spin_lock(&mut (*cache).lock);
    for i in 0..n { let head = (*cache).hashtable.add(i); while !hlist_empty(head) { let c = hlist_entry((*head).first, rpc_cred, cr_hash); rpcauth_unhash_cred_locked(c); rpcauth_lru_remove_locked(c); list_add_tail(&mut (*c).cr_lru, &mut free); } }
    spin_unlock(&mut (*cache).lock); spin_unlock(&mut rpc_credcache_lock); rpcauth_destroy_credlist(&mut free);
}

#[no_mangle]
pub unsafe extern "C" fn rpcauth_destroy_credcache(auth: *mut rpc_auth) {
    if let Some(cache) = (*auth).au_credcache.take() { let p = Box::into_raw(cache); rpcauth_clear_credcache(p); kfree((*p).hashtable as *mut c_void); kfree(p as *mut c_void); }
}

// The remaining entry points are direct dispatches to the credential and auth operation tables.
#[no_mangle] pub unsafe extern "C" fn rpcauth_marshcred(t:*mut rpc_task,x:*mut xdr_stream)->c_int { ((*(*(*t).tk_rqstp).rq_cred).cr_ops).crmarshal.unwrap()(t,x) }
#[no_mangle] pub unsafe extern "C" fn rpcauth_wrap_req(t:*mut rpc_task,x:*mut xdr_stream)->c_int { ((*(*(*t).tk_rqstp).rq_cred).cr_ops).crwrap_req.unwrap()(t,x) }
#[no_mangle] pub unsafe extern "C" fn rpcauth_checkverf(t:*mut rpc_task,x:*mut xdr_stream)->c_int { ((*(*(*t).tk_rqstp).rq_cred).cr_ops).crvalidate.unwrap()(t,x) }
#[no_mangle] pub unsafe extern "C" fn rpcauth_unwrap_resp(t:*mut rpc_task,x:*mut xdr_stream)->c_int { ((*(*(*t).tk_rqstp).rq_cred).cr_ops).crunwrap_resp.unwrap()(t,x) }
#[no_mangle] pub unsafe extern "C" fn rpcauth_xmit_need_reencode(t:*mut rpc_task)->bool { let c=(*(*t).tk_rqstp).rq_cred; !c.is_null() && (*(*c).cr_ops).crneed_reencode.map_or(false,|f|f(t)) }
#[no_mangle] pub unsafe extern "C" fn rpcauth_invalcred(t:*mut rpc_task) { let c=(*(*t).tk_rqstp).rq_cred; if !c.is_null(){clear_bit(RPCAUTH_CRED_UPTODATE,&mut (*c).cr_flags);} }
#[no_mangle] pub unsafe extern "C" fn rpcauth_uptodatecred(t:*mut rpc_task)->bool { let c=(*(*t).tk_rqstp).rq_cred; c.is_null() || test_bit(RPCAUTH_CRED_UPTODATE,&(*c).cr_flags) }

#[no_mangle]
pub unsafe extern "C" fn rpcauth_lookupcred(auth:*mut rpc_auth, flags:c_int)->*mut rpc_cred {
    let mut a = core::mem::zeroed::<auth_cred>(); a.cred = current_cred();
    ((*(*auth).au_ops).lookup_cred.unwrap())(auth,&mut a,flags)
}
#[no_mangle]
pub unsafe extern "C" fn rpcauth_init_cred(c:*mut rpc_cred,a:*const auth_cred,auth:*mut rpc_auth,ops:*const rpc_credops) {
    INIT_HLIST_NODE(&mut (*c).cr_hash); INIT_LIST_HEAD(&mut (*c).cr_lru); refcount_set(&mut (*c).cr_count,1);
    (*c).cr_auth=auth; (*c).cr_flags=0; (*c).cr_ops=ops; (*c).cr_expire=jiffies; (*c).cr_cred=get_cred((*a).cred);
}
#[no_mangle]
pub unsafe extern "C" fn put_rpccred(c:*mut rpc_cred) { if c.is_null(){return;} if refcount_dec_and_test(&mut (*c).cr_count){((*(*c).cr_ops).crdestroy.unwrap())(c);} }
#[no_mangle]
pub unsafe extern "C" fn rpcauth_wrap_req_encode(t:*mut rpc_task,x:*mut xdr_stream)->c_int { let f=(*(*t).tk_msg.rpc_proc).p_encode.unwrap(); f((*t).tk_rqstp,x,(*t).tk_msg.rpc_argp); 0 }
#[no_mangle]
pub unsafe extern "C" fn rpcauth_unwrap_resp_decode(t:*mut rpc_task,x:*mut xdr_stream)->c_int { let f=(*(*t).tk_msg.rpc_proc).p_decode.unwrap(); f((*t).tk_rqstp,x,(*t).tk_msg.rpc_resp) }
#[no_mangle]
pub unsafe extern "C" fn rpcauth_refreshcred(t:*mut rpc_task)->c_int {
    let mut c=(*(*t).tk_rqstp).rq_cred; let mut err=0;
    if c.is_null(){ err=rpcauth_bindcred(t,(*t).tk_msg.rpc_cred,(*t).tk_flags); if err<0{return err;} c=(*(*t).tk_rqstp).rq_cred; }
    err=((*(*c).cr_ops).crrefresh.unwrap())(t); if err<0{(*t).tk_status=err;} err
}
#[no_mangle]
pub unsafe extern "C" fn rpcauth_init_module()->c_int { let e=rpc_init_authunix(); if e<0{e}else{0} }
#[no_mangle]
pub unsafe extern "C" fn rpcauth_remove_module(){rpc_destroy_authunix();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
