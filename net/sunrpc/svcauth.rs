// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/net/sunrpc/svcauth.c
 *
 * The generic interface for RPC authentication on the server side.
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 *
 * CHANGES
 * 19-Apr-2000 Chris Evans      - Security fix
 */

// Dependencies supplied by the surrounding kernel/RPC translation.

const RPCDBG_FACILITY: u32 = RPCDBG_AUTH;

/* Table of authenticators */
extern "C" {
    static mut svcauth_null: auth_ops;
    static mut svcauth_unix: auth_ops;
    static mut svcauth_tls: auth_ops;
}

static mut authtab: [*mut auth_ops; RPC_AUTH_MAXFLAVOR as usize] = {
    let mut table = [core::ptr::null_mut(); RPC_AUTH_MAXFLAVOR as usize];
    table[RPC_AUTH_NULL as usize] = core::ptr::addr_of_mut!(svcauth_null);
    table[RPC_AUTH_UNIX as usize] = core::ptr::addr_of_mut!(svcauth_unix);
    table[RPC_AUTH_TLS as usize] = core::ptr::addr_of_mut!(svcauth_tls);
    table
};

unsafe fn svc_get_auth_ops(flavor: rpc_authflavor_t) -> *mut auth_ops {
    let mut aops: *mut auth_ops;

    if flavor >= RPC_AUTH_MAXFLAVOR {
        return core::ptr::null_mut();
    }
    rcu_read_lock();
    aops = rcu_dereference(authtab[flavor as usize]);
    if !aops.is_null() && !try_module_get((*aops).owner) {
        aops = core::ptr::null_mut();
    }
    rcu_read_unlock();
    aops
}

unsafe fn svc_put_auth_ops(aops: *mut auth_ops) {
    module_put((*aops).owner);
}

/**
 * svc_authenticate - Initialize an outgoing credential
 * @rqstp: RPC execution context
 *
 * Return values:
 *   %SVC_OK: XDR encoding of the result can begin
 *   %SVC_DENIED: Credential or verifier is not valid
 *   %SVC_GARBAGE: Failed to decode credential or verifier
 *   %SVC_COMPLETE: GSS context lifetime event; no further action
 *   %SVC_DROP: Drop this request; no further action
 *   %SVC_CLOSE: Like drop, but also close transport connection
 */
pub unsafe extern "C" fn svc_authenticate(rqstp: *mut svc_rqst) -> svc_auth_status {
    let mut flavor: u32 = 0;

    (*rqstp).rq_auth_stat = rpc_auth_ok;
    if xdr_stream_decode_u32(&mut (*rqstp).rq_arg_stream, &mut flavor) < 0 {
        return SVC_GARBAGE;
    }

    let aops = svc_get_auth_ops(flavor);
    if aops.is_null() {
        (*rqstp).rq_auth_stat = rpc_autherr_badcred;
        return SVC_DENIED;
    }

    (*rqstp).rq_auth_slack = 0;
    init_svc_cred(&mut (*rqstp).rq_cred);
    (*rqstp).rq_authop = aops;
    ((*aops).accept)(rqstp)
}

/** svc_set_client - Assign an appropriate auth_domain as the client */
pub unsafe extern "C" fn svc_set_client(rqstp: *mut svc_rqst) -> svc_auth_status {
    (*rqstp).rq_client = core::ptr::null_mut();
    ((*(*rqstp).rq_authop).set_client)(rqstp)
}

pub unsafe extern "C" fn svc_authorise(rqstp: *mut svc_rqst) -> i32 {
    let aops = (*rqstp).rq_authop;
    let mut rv: i32 = 0;
    (*rqstp).rq_authop = core::ptr::null_mut();
    if !aops.is_null() {
        rv = ((*aops).release)(rqstp);
        svc_put_auth_ops(aops);
    }
    rv
}

pub unsafe extern "C" fn svc_auth_register(
    flavor: rpc_authflavor_t,
    aops: *mut auth_ops,
) -> i32 {
    let mut rv = -EINVAL;
    if flavor < RPC_AUTH_MAXFLAVOR {
        let old = cmpxchg(&mut authtab[flavor as usize], core::ptr::null_mut(), aops);
        if old.is_null() || old == aops {
            rv = 0;
        }
    }
    rv
}

pub unsafe extern "C" fn svc_auth_unregister(flavor: rpc_authflavor_t) {
    if flavor < RPC_AUTH_MAXFLAVOR {
        rcu_assign_pointer(&mut authtab[flavor as usize], core::ptr::null_mut());
    }
}

pub unsafe extern "C" fn svc_auth_flavor(rqstp: *mut svc_rqst) -> rpc_authflavor_t {
    let aops = (*rqstp).rq_authop;
    if (*aops).pseudoflavor.is_none() {
        (*aops).flavour
    } else {
        ((*aops).pseudoflavor.unwrap())(rqstp)
    }
}

pub unsafe extern "C" fn svcauth_map_clnt_to_svc_cred_local(
    clnt: *mut rpc_clnt,
    cred: *const cred,
    svc: *mut svc_cred,
) {
    let userns = if !(*clnt).cl_cred.is_null() {
        (*(*clnt).cl_cred).user_ns
    } else {
        &mut init_user_ns
    };
    core::ptr::write_bytes(svc, 0, 1);
    (*svc).cr_uid = KUIDT_INIT(from_kuid_munged(userns, (*cred).fsuid));
    (*svc).cr_gid = KGIDT_INIT(from_kgid_munged(userns, (*cred).fsgid));
    (*svc).cr_flavor = (*(*clnt).cl_auth).au_flavor;
    if !(*cred).group_info.is_null() {
        (*svc).cr_group_info = get_group_info((*cred).group_info);
    }
    (*svc).cr_principal = core::ptr::null_mut();
    (*svc).cr_gss_mech = core::ptr::null_mut();
}

const DN_HASHBITS: usize = 6;
const DN_HASHMAX: usize = 1 << DN_HASHBITS;

static mut auth_domain_table: [hlist_head; DN_HASHMAX] = [hlist_head::new(); DN_HASHMAX];
static mut auth_domain_lock: spinlock_t = DEFINE_SPINLOCK();

unsafe fn auth_domain_release(kref: *mut kref) {
    let dom = container_of!(kref, auth_domain, ref_);
    hlist_del_rcu(&mut (*dom).hash);
    ((*(*dom).flavour).domain_release)(dom);
    spin_unlock(&mut auth_domain_lock);
}

pub unsafe extern "C" fn auth_domain_put(dom: *mut auth_domain) {
    kref_put_lock(&mut (*dom).ref_, auth_domain_release, &mut auth_domain_lock);
}

pub unsafe extern "C" fn auth_domain_lookup(
    name: *mut c_char,
    new: *mut auth_domain,
) -> *mut auth_domain {
    let head = &mut auth_domain_table[hash_str(name, DN_HASHBITS) as usize];
    spin_lock(&mut auth_domain_lock);
    hlist_for_each_entry!(hp, head, hash, {
        if strcmp((*hp).name, name) == 0 {
            kref_get(&mut (*hp).ref_);
            spin_unlock(&mut auth_domain_lock);
            return hp;
        }
    });
    if !new.is_null() {
        hlist_add_head_rcu(&mut (*new).hash, head);
    }
    spin_unlock(&mut auth_domain_lock);
    new
}

pub unsafe extern "C" fn auth_domain_find(name: *mut c_char) -> *mut auth_domain {
    let head = &mut auth_domain_table[hash_str(name, DN_HASHBITS) as usize];
    rcu_read_lock();
    hlist_for_each_entry_rcu!(hp, head, hash, {
        if strcmp((*hp).name, name) == 0 {
            if !kref_get_unless_zero(&mut (*hp).ref_) {
                hp = core::ptr::null_mut();
            }
            rcu_read_unlock();
            return hp;
        }
    });
    rcu_read_unlock();
    core::ptr::null_mut()
}

pub unsafe extern "C" fn auth_domain_cleanup() {
    for h in 0..DN_HASHMAX {
        hlist_for_each_entry!(hp, &mut auth_domain_table[h], hash, {
            pr_warn!("svc: domain %s still present at module unload.\n", (*hp).name);
        });
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
