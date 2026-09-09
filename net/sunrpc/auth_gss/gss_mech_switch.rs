// SPDX-License-Identifier: BSD-3-Clause
/*
 *  linux/net/sunrpc/gss_mech_switch.c
 *
 *  Copyright (c) 2001 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  J. Bruce Fields   <bfields@umich.edu>
 */

// C includes are supplied by the surrounding kernel translation.

#[cfg(CONFIG_SUNRPC_DEBUG)]
// RPCDBG_FACILITY = RPCDBG_AUTH

static mut registered_mechs: list_head = LIST_HEAD_INIT;
static mut registered_mechs_lock: spinlock_t = __SPIN_LOCK_UNLOCKED;

unsafe fn gss_mech_free(gm: *mut gss_api_mech) {
    let mut i: c_int = 0;
    while i < (*gm).gm_pf_num {
        let pf = &mut *(*gm).gm_pfs.add(i as usize);
        if !pf.domain.is_null() { auth_domain_put(pf.domain); }
        kfree(pf.auth_domain_name as *mut c_void);
        pf.auth_domain_name = core::ptr::null_mut();
        i += 1;
    }
}

unsafe fn make_auth_domain_name(name: *mut c_char) -> *mut c_char {
    let prefix = b"gss/\0".as_ptr() as *mut c_char;
    let new = kmalloc(strlen(name) + strlen(prefix) + 1, GFP_KERNEL) as *mut c_char;
    if !new.is_null() { strcpy(new, prefix); strcat(new, name); }
    new
}

unsafe fn gss_mech_svc_setup(gm: *mut gss_api_mech) -> c_int {
    let mut i: c_int = 0;
    while i < (*gm).gm_pf_num {
        let pf = &mut *(*gm).gm_pfs.add(i as usize);
        pf.auth_domain_name = make_auth_domain_name(pf.name);
        let mut status = -ENOMEM;
        if pf.auth_domain_name.is_null() { gss_mech_free(gm); return status; }
        let dom = svcauth_gss_register_pseudoflavor(pf.pseudoflavor, pf.auth_domain_name);
        if IS_ERR(dom) { status = PTR_ERR(dom); gss_mech_free(gm); return status; }
        pf.domain = dom;
        i += 1;
    }
    0
}

pub unsafe fn gss_mech_register(gm: *mut gss_api_mech) -> c_int {
    let status = gss_mech_svc_setup(gm);
    if status != 0 { return status; }
    spin_lock(&mut registered_mechs_lock);
    list_add_rcu(&mut (*gm).gm_list, &mut registered_mechs);
    spin_unlock(&mut registered_mechs_lock);
    dprintk!("RPC:       registered gss mechanism %s\n", (*gm).gm_name);
    0
}

pub unsafe fn gss_mech_unregister(gm: *mut gss_api_mech) {
    spin_lock(&mut registered_mechs_lock);
    list_del_rcu(&mut (*gm).gm_list);
    spin_unlock(&mut registered_mechs_lock);
    dprintk!("RPC:       unregistered gss mechanism %s\n", (*gm).gm_name);
    gss_mech_free(gm);
}

pub unsafe fn gss_mech_get(gm: *mut gss_api_mech) -> *mut gss_api_mech {
    __module_get((*gm).gm_owner); gm
}

unsafe fn _gss_mech_get_by_name(name: *const c_char) -> *mut gss_api_mech {
    let mut gm = core::ptr::null_mut();
    rcu_read_lock();
    let mut pos: *mut gss_api_mech = core::ptr::null_mut();
    list_for_each_entry_rcu!(pos, registered_mechs, gm_list, {
        if strcmp(name, (*pos).gm_name) == 0 {
            if try_module_get((*pos).gm_owner) { gm = pos; }
            break;
        }
    });
    rcu_read_unlock(); gm
}

pub unsafe fn gss_mech_get_by_name(name: *const c_char) -> *mut gss_api_mech {
    let mut gm = _gss_mech_get_by_name(name);
    if gm.is_null() { request_module!("rpc-auth-gss-%s", name); gm = _gss_mech_get_by_name(name); }
    gm
}

pub unsafe fn gss_mech_get_by_OID(obj: *mut rpcsec_gss_oid) -> *mut gss_api_mech {
    let mut buf = [0 as c_char; 32];
    if sprint_oid((*obj).data, (*obj).len, buf.as_mut_ptr(), buf.len()) < 0 { return core::ptr::null_mut(); }
    request_module!("rpc-auth-gss-%s", buf.as_ptr());
    let mut gm = core::ptr::null_mut();
    rcu_read_lock();
    let mut pos: *mut gss_api_mech = core::ptr::null_mut();
    list_for_each_entry_rcu!(pos, registered_mechs, gm_list, {
        if (*obj).len == (*pos).gm_oid.len && memcmp((*obj).data, (*pos).gm_oid.data, (*obj).len) == 0 {
            if try_module_get((*pos).gm_owner) { gm = pos; } break;
        }
    });
    rcu_read_unlock();
    if gm.is_null() { trace_rpcgss_oid_to_mech(buf.as_ptr()); }
    gm
}

unsafe fn mech_supports_pseudoflavor(gm: *mut gss_api_mech, pseudoflavor: u32) -> c_int {
    let mut i = 0; while i < (*gm).gm_pf_num { if (*gm).gm_pfs.add(i as usize).as_ref().unwrap().pseudoflavor == pseudoflavor { return 1; } i += 1; } 0
}

unsafe fn _gss_mech_get_by_pseudoflavor(pseudoflavor: u32) -> *mut gss_api_mech {
    let mut gm = core::ptr::null_mut(); rcu_read_lock(); let mut pos: *mut gss_api_mech = core::ptr::null_mut();
    list_for_each_entry_rcu!(pos, registered_mechs, gm_list, { if mech_supports_pseudoflavor(pos, pseudoflavor) == 0 { continue; } if try_module_get((*pos).gm_owner) { gm = pos; } break; }); rcu_read_unlock(); gm
}

pub unsafe fn gss_mech_get_by_pseudoflavor(pseudoflavor: u32) -> *mut gss_api_mech {
    let mut gm = _gss_mech_get_by_pseudoflavor(pseudoflavor); if gm.is_null() { request_module!("rpc-auth-gss-%u", pseudoflavor); gm = _gss_mech_get_by_pseudoflavor(pseudoflavor); } gm
}

pub unsafe fn gss_svc_to_pseudoflavor(gm: *mut gss_api_mech, qop: u32, service: u32) -> rpc_authflavor_t { let mut i=0; while i<(*gm).gm_pf_num { let pf=&*(*gm).gm_pfs.add(i as usize); if pf.qop==qop && pf.service==service { return pf.pseudoflavor; } i+=1; } RPC_AUTH_MAXFLAVOR }

pub unsafe fn gss_mech_info2flavor(info: *mut rpcsec_gss_info) -> rpc_authflavor_t { let gm=gss_mech_get_by_OID(&mut (*info).oid); if gm.is_null(){return RPC_AUTH_MAXFLAVOR;} let p=gss_svc_to_pseudoflavor(gm,(*info).qop,(*info).service); gss_mech_put(gm); p }

pub unsafe fn gss_mech_flavor2info(pseudoflavor: rpc_authflavor_t, info: *mut rpcsec_gss_info) -> c_int { let gm=gss_mech_get_by_pseudoflavor(pseudoflavor); if gm.is_null(){return -ENOENT;} let mut i=0; while i<(*gm).gm_pf_num { let pf=&*(*gm).gm_pfs.add(i as usize); if pf.pseudoflavor==pseudoflavor { memcpy((*info).oid.data,(*gm).gm_oid.data,(*gm).gm_oid.len); (*info).oid.len=(*gm).gm_oid.len; (*info).qop=pf.qop; (*info).service=pf.service; gss_mech_put(gm); return 0;} i+=1;} gss_mech_put(gm); -ENOENT }

pub unsafe fn gss_pseudoflavor_to_service(gm:*mut gss_api_mech,p:u32)->u32 { let mut i=0; while i<(*gm).gm_pf_num {let pf=&*(*gm).gm_pfs.add(i as usize);if pf.pseudoflavor==p{return pf.service;}i+=1;}0 }
pub unsafe fn gss_pseudoflavor_to_datatouch(gm:*mut gss_api_mech,p:u32)->bool {let mut i=0;while i<(*gm).gm_pf_num{let pf=&*(*gm).gm_pfs.add(i as usize);if pf.pseudoflavor==p{return pf.datatouch;}i+=1;}false}
pub unsafe fn gss_service_to_auth_domain_name(gm:*mut gss_api_mech,s:u32)->*mut c_char{let mut i=0;while i<(*gm).gm_pf_num{let pf=&*(*gm).gm_pfs.add(i as usize);if pf.service==s{return pf.auth_domain_name;}i+=1;}core::ptr::null_mut()}
pub unsafe fn gss_mech_put(gm:*mut gss_api_mech){if !gm.is_null(){module_put((*gm).gm_owner);}}

pub unsafe fn gss_import_sec_context(input_token:*const c_void,bufsize:usize,mech:*mut gss_api_mech,ctx_id:*mut *mut gss_ctx,endtime:*mut time64_t,gfp_mask:gfp_t)->u32{*ctx_id=kzalloc_obj::<gss_ctx>(gfp_mask);if (*ctx_id).is_null(){return -ENOMEM as u32;}(**ctx_id).mech_type=gss_mech_get(mech);((*mech).gm_ops).gss_import_sec_context(input_token,bufsize,*ctx_id,endtime,gfp_mask)}
pub unsafe fn gss_get_mic(c:*mut gss_ctx,m:*mut xdr_buf,t:*mut xdr_netobj)->u32{((*(*c).mech_type).gm_ops).gss_get_mic(c,m,t)}
pub unsafe fn gss_verify_mic(c:*mut gss_ctx,m:*mut xdr_buf,t:*mut xdr_netobj)->u32{((*(*c).mech_type).gm_ops).gss_verify_mic(c,m,t)}
/* The client and server provide RPC_MAX_AUTH_SIZE slack in both head and tail. */
pub unsafe fn gss_wrap(c:*mut gss_ctx,o:c_int,b:*mut xdr_buf,p:*mut *mut page)->u32{((*(*c).mech_type).gm_ops).gss_wrap(c,o,b,p)}
pub unsafe fn gss_unwrap(c:*mut gss_ctx,o:c_int,l:c_int,b:*mut xdr_buf)->u32{((*(*c).mech_type).gm_ops).gss_unwrap(c,o,l,b)}

pub unsafe fn gss_delete_sec_context(c:*mut *mut gss_ctx)->u32{dprintk!("RPC:       gss_delete_sec_context deleting %p\n",*c);if (*c).is_null(){return GSS_S_NO_CONTEXT;}if !(**c).internal_ctx_id.is_null(){((*(*(*c).mech_type).gm_ops).gss_delete_sec_context)((**c).internal_ctx_id);}gss_mech_put((**c).mech_type);kfree(*c as *mut c_void);*c=core::ptr::null_mut();GSS_S_COMPLETE}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
