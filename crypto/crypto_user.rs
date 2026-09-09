// SPDX-License-Identifier: GPL-2.0-only
/*
 * Crypto user configuration API.
 *
 * Copyright (C) 2011 secunet Security Networks AG
 * Copyright (C) 2011 Steffen Klassert <steffen.klassert@secunet.com>
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

const fn null_terminated<T>(x: *const T) -> bool {
    // Equivalent to: strnlen(x, sizeof(x)) < sizeof(x).
    unsafe { strnlen(x, core::mem::size_of::<T>()) < core::mem::size_of::<T>() }
}

static mut CRYPTO_CFG_MUTEX: Mutex = Mutex::new();

#[repr(C)]
struct CryptoDumpInfo {
    in_skb: *mut SkBuff,
    out_skb: *mut SkBuff,
    nlmsg_seq: u32,
    nlmsg_flags: u16,
}

unsafe fn crypto_alg_match(p: *mut CryptoUserAlg, exact: i32) -> *mut CryptoAlg {
    let mut q: *mut CryptoAlg;
    let mut alg: *mut CryptoAlg = core::ptr::null_mut();

    down_read(&mut crypto_alg_sem);
    list_for_each_entry!(q, crypto_alg_list, cra_list) {
        let mut matched = 0;
        if crypto_is_larval(q) { continue; }
        if ((*q).cra_flags ^ (*p).cru_type) & (*p).cru_mask != 0 { continue; }
        if strlen((*p).cru_driver_name.as_ptr()) != 0 {
            matched = (!strcmp((*q).cra_driver_name.as_ptr(), (*p).cru_driver_name.as_ptr())) as i32;
        } else if exact == 0 {
            matched = (!strcmp((*q).cra_name.as_ptr(), (*p).cru_name.as_ptr())) as i32;
        }
        if matched == 0 { continue; }
        if !crypto_mod_get(q) { continue; }
        alg = q;
        break;
    }
    up_read(&mut crypto_alg_sem);
    alg
}

unsafe fn crypto_report_cipher(skb: *mut SkBuff, alg: *mut CryptoAlg) -> i32 {
    let mut rcipher = CryptoReportCipher { r#type: *b"cipher\0" as *const u8, ..core::mem::zeroed() };
    rcipher.blocksize = (*alg).cra_blocksize;
    rcipher.min_keysize = (*alg).cra_cipher.cia_min_keysize;
    rcipher.max_keysize = (*alg).cra_cipher.cia_max_keysize;
    nla_put(skb, CRYPTOCFGA_REPORT_CIPHER, core::mem::size_of_val(&rcipher), &rcipher)
}

unsafe fn crypto_report_one(alg: *mut CryptoAlg, ualg: *mut CryptoUserAlg, skb: *mut SkBuff) -> i32 {
    memset(ualg as *mut _, 0, core::mem::size_of::<CryptoUserAlg>());
    strscpy((*ualg).cru_name.as_mut_ptr(), (*alg).cra_name.as_ptr());
    strscpy((*ualg).cru_driver_name.as_mut_ptr(), (*alg).cra_driver_name.as_ptr());
    strscpy((*ualg).cru_module_name.as_mut_ptr(), module_name((*alg).cra_module));
    (*ualg).cru_type = 0;
    (*ualg).cru_mask = 0;
    (*ualg).cru_flags = (*alg).cra_flags;
    (*ualg).cru_refcnt = refcount_read(&(*alg).cra_refcnt);
    if nla_put_u32(skb, CRYPTOCFGA_PRIORITY_VAL, (*alg).cra_priority) != 0 { return -EMSGSIZE; }
    if (*alg).cra_flags & CRYPTO_ALG_LARVAL != 0 {
        let rl = CryptoReportLarval { r#type: *b"larval\0" as *const u8 };
        if nla_put(skb, CRYPTOCFGA_REPORT_LARVAL, core::mem::size_of_val(&rl), &rl) != 0 { return -EMSGSIZE; }
        return 0;
    }
    if !(*alg).cra_type.is_null() && !(*(*alg).cra_type).report.is_none() {
        if ((*(*alg).cra_type).report.unwrap())(skb, alg) != 0 { return -EMSGSIZE; }
        return 0;
    }
    match (*alg).cra_flags & (CRYPTO_ALG_TYPE_MASK | CRYPTO_ALG_LARVAL) {
        CRYPTO_ALG_TYPE_CIPHER => { if crypto_report_cipher(skb, alg) != 0 { return -EMSGSIZE; } }
        _ => {}
    }
    0
}

unsafe fn crypto_report_alg(alg: *mut CryptoAlg, info: *mut CryptoDumpInfo) -> i32 {
    let nlh = nlmsg_put((*info).out_skb, NETLINK_CB((*info).in_skb).portid, (*info).nlmsg_seq, CRYPTO_MSG_GETALG, core::mem::size_of::<CryptoUserAlg>(), (*info).nlmsg_flags);
    if nlh.is_null() { return -EMSGSIZE; }
    let ualg = nlmsg_data(nlh);
    let err = crypto_report_one(alg, ualg, (*info).out_skb);
    if err != 0 { nlmsg_cancel((*info).out_skb, nlh); return err; }
    nlmsg_end((*info).out_skb, nlh);
    0
}

unsafe fn crypto_report(in_skb: *mut SkBuff, in_nlh: *mut Nlmsghdr, _attrs: *mut *mut Nlattr) -> i32 {
    let net = sock_net((*in_skb).sk);
    let p: *mut CryptoUserAlg = nlmsg_data(in_nlh);
    if !null_terminated((*p).cru_name.as_ptr()) || !null_terminated((*p).cru_driver_name.as_ptr()) { return -EINVAL; }
    let alg = crypto_alg_match(p, 0);
    if alg.is_null() { return -ENOENT; }
    let skb = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if skb.is_null() { crypto_mod_put(alg); return -ENOMEM; }
    let mut info = CryptoDumpInfo { in_skb, out_skb: skb, nlmsg_seq: (*in_nlh).nlmsg_seq, nlmsg_flags: 0 };
    let err = crypto_report_alg(alg, &mut info);
    crypto_mod_put(alg);
    if err != 0 { kfree_skb(skb); return err; }
    nlmsg_unicast((*net).crypto_nlsk, skb, NETLINK_CB(in_skb).portid)
}

unsafe fn crypto_dump_report(skb: *mut SkBuff, cb: *mut NetlinkCallback) -> i32 {
    let start_pos = (*cb).args[0] as usize;
    let mut pos = 0usize;
    let mut info = CryptoDumpInfo { in_skb: (*cb).skb, out_skb: skb, nlmsg_seq: (*(*cb).nlh).nlmsg_seq, nlmsg_flags: NLM_F_MULTI };
    let mut res;
    down_read(&mut crypto_alg_sem);
    let mut alg: *mut CryptoAlg;
    list_for_each_entry!(alg, crypto_alg_list, cra_list) {
        if pos >= start_pos { res = crypto_report_alg(alg, &mut info); if res == -EMSGSIZE { break; } if res != 0 { up_read(&mut crypto_alg_sem); return res; } }
        pos += 1;
    }
    (*cb).args[0] = pos as _;
    res = (*skb).len as i32;
    up_read(&mut crypto_alg_sem);
    res
}

unsafe fn crypto_dump_report_done(_cb: *mut NetlinkCallback) -> i32 { 0 }

unsafe fn crypto_update_alg(skb: *mut SkBuff, nlh: *mut Nlmsghdr, attrs: *mut *mut Nlattr) -> i32 {
    if !netlink_capable(skb, CAP_NET_ADMIN) { return -EPERM; }
    let p: *mut CryptoUserAlg = nlmsg_data(nlh);
    if !null_terminated((*p).cru_name.as_ptr()) || !null_terminated((*p).cru_driver_name.as_ptr()) { return -EINVAL; }
    let priority = *attrs.add(CRYPTOCFGA_PRIORITY_VAL as usize);
    if !priority.is_null() && strlen((*p).cru_driver_name.as_ptr()) == 0 { return -EINVAL; }
    let alg = crypto_alg_match(p, 1); if alg.is_null() { return -ENOENT; }
    down_write(&mut crypto_alg_sem);
    let mut list = ListHead::new(); crypto_remove_spawns(alg, &mut list, core::ptr::null_mut());
    if !priority.is_null() { (*alg).cra_priority = nla_get_u32(priority); }
    up_write(&mut crypto_alg_sem); crypto_mod_put(alg); crypto_remove_final(&mut list); 0
}

unsafe fn crypto_del_alg(skb: *mut SkBuff, nlh: *mut Nlmsghdr, _attrs: *mut *mut Nlattr) -> i32 {
    if !netlink_capable(skb, CAP_NET_ADMIN) { return -EPERM; }
    let p: *mut CryptoUserAlg = nlmsg_data(nlh);
    if !null_terminated((*p).cru_name.as_ptr()) || !null_terminated((*p).cru_driver_name.as_ptr()) { return -EINVAL; }
    let alg = crypto_alg_match(p, 1); if alg.is_null() { return -ENOENT; }
    let mut err = -EINVAL;
    if (*alg).cra_flags & CRYPTO_ALG_INSTANCE == 0 { crypto_mod_put(alg); return err; }
    err = -EBUSY; if refcount_read(&(*alg).cra_refcnt) > 2 { crypto_mod_put(alg); return err; }
    crypto_unregister_instance(alg as *mut CryptoInstance); crypto_mod_put(alg); 0
}

unsafe fn crypto_add_alg(skb: *mut SkBuff, nlh: *mut Nlmsghdr, attrs: *mut *mut Nlattr) -> i32 {
    if !netlink_capable(skb, CAP_NET_ADMIN) { return -EPERM; }
    let p: *mut CryptoUserAlg = nlmsg_data(nlh);
    if !null_terminated((*p).cru_name.as_ptr()) || !null_terminated((*p).cru_driver_name.as_ptr()) { return -EINVAL; }
    let exact = (strlen((*p).cru_driver_name.as_ptr()) != 0) as i32;
    let priority = *attrs.add(CRYPTOCFGA_PRIORITY_VAL as usize);
    if !priority.is_null() && exact == 0 { return -EINVAL; }
    let mut alg = crypto_alg_match(p, exact); if !alg.is_null() { crypto_mod_put(alg); return -EEXIST; }
    let name = if exact != 0 { (*p).cru_driver_name.as_ptr() } else { (*p).cru_name.as_ptr() };
    alg = crypto_alg_mod_lookup(name, (*p).cru_type, (*p).cru_mask); if is_err(alg) { return ptr_err(alg); }
    down_write(&mut crypto_alg_sem); if !priority.is_null() { (*alg).cra_priority = nla_get_u32(priority); } up_write(&mut crypto_alg_sem); crypto_mod_put(alg); 0
}

unsafe fn crypto_del_rng(skb: *mut SkBuff, _nlh: *mut Nlmsghdr, _attrs: *mut *mut Nlattr) -> i32 {
    if !netlink_capable(skb, CAP_NET_ADMIN) { return -EPERM; } crypto_del_default_rng()
}
unsafe fn crypto_reportstat(_skb: *mut SkBuff, _nlh: *mut Nlmsghdr, _attrs: *mut *mut Nlattr) -> i32 { -ENOTSUPP }

unsafe fn crypto_netlink_rcv(skb: *mut SkBuff) { mutex_lock(&mut CRYPTO_CFG_MUTEX); netlink_rcv_skb(skb, crypto_user_rcv_msg); mutex_unlock(&mut CRYPTO_CFG_MUTEX); }
unsafe fn crypto_user_rcv_msg(skb: *mut SkBuff, nlh: *mut Nlmsghdr, extack: *mut NetlinkExtAck) -> i32 {
    let typ = (*nlh).nlmsg_type; if typ > CRYPTO_MSG_MAX { return -EINVAL; }
    let idx = typ - CRYPTO_MSG_BASE; let link = &crypto_dispatch[idx as usize];
    if idx == CRYPTO_MSG_GETALG - CRYPTO_MSG_BASE && (*nlh).nlmsg_flags & NLM_F_DUMP != 0 {
        if link.dump.is_none() { return -EINVAL; }
        return netlink_dump_start((*sock_net((*skb).sk)).crypto_nlsk, skb, nlh, &NetlinkDumpControl { dump: link.dump, done: link.done, min_dump_alloc: 0 });
    }
    let mut attrs = [core::ptr::null_mut(); CRYPTOCFGA_MAX + 1];
    let err = nlmsg_parse_deprecated(nlh, crypto_msg_min[idx as usize], attrs.as_mut_ptr(), CRYPTOCFGA_MAX, &crypto_policy, extack); if err < 0 { return err; }
    match link.doit { Some(f) => f(skb, nlh, attrs.as_mut_ptr()), None => -EINVAL }
}

const CRYPTO_MSG_MIN: [usize; CRYPTO_NR_MSGTYPES] = [
    core::mem::size_of::<CryptoUserAlg>(), core::mem::size_of::<CryptoUserAlg>(),
    core::mem::size_of::<CryptoUserAlg>(), core::mem::size_of::<CryptoUserAlg>(), 0,
    core::mem::size_of::<CryptoUserAlg>(),
];

#[repr(C)]
struct CryptoLink { doit: Option<unsafe fn(*mut SkBuff, *mut Nlmsghdr, *mut *mut Nlattr) -> i32>, dump: Option<unsafe fn(*mut SkBuff, *mut NetlinkCallback) -> i32>, done: Option<unsafe fn(*mut NetlinkCallback) -> i32> }
static CRYPTO_DISPATCH: [CryptoLink; CRYPTO_NR_MSGTYPES] = [
    CryptoLink { doit: Some(crypto_add_alg), dump: None, done: None },
    CryptoLink { doit: Some(crypto_del_alg), dump: None, done: None },
    CryptoLink { doit: Some(crypto_update_alg), dump: None, done: None },
    CryptoLink { doit: Some(crypto_report), dump: Some(crypto_dump_report), done: Some(crypto_dump_report_done) },
    CryptoLink { doit: Some(crypto_del_rng), dump: None, done: None },
    CryptoLink { doit: Some(crypto_reportstat), dump: None, done: None },
];

unsafe fn crypto_netlink_init(net: *mut Net) -> i32 { (*net).crypto_nlsk = netlink_kernel_create(net, NETLINK_CRYPTO, &NetlinkKernelCfg { input: Some(crypto_netlink_rcv) }); if (*net).crypto_nlsk.is_null() { -ENOMEM } else { 0 } }
unsafe fn crypto_netlink_exit(net: *mut Net) { netlink_kernel_release((*net).crypto_nlsk); (*net).crypto_nlsk = core::ptr::null_mut(); }
static CRYPTO_NETLINK_NET_OPS: PerNetOperations = PerNetOperations { init: Some(crypto_netlink_init), exit: Some(crypto_netlink_exit) };
unsafe fn crypto_user_init() -> i32 { register_pernet_subsys(&CRYPTO_NETLINK_NET_OPS) }
unsafe fn crypto_user_exit() { unregister_pernet_subsys(&CRYPTO_NETLINK_NET_OPS); }

// Module metadata and init/exit registration are retained as kernel build-system intent.
module_init!(crypto_user_init);
module_exit!(crypto_user_exit);
module_license!("GPL");
module_author!("Steffen Klassert <steffen.klassert@secunet.com>");
module_description!("Crypto userspace configuration API");
module_alias!("net-pf-16-proto-21");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
