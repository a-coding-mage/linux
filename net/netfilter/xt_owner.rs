// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel module to match various things tied to sockets associated with
 * locally generated outgoing packets.
 *
 * (C) 2000 Marc Boucher <marc@mbsi.ca>
 *
 * Copyright © CC Computer Consultants GmbH, 2007 - 2008
 */

unsafe fn owner_check(par: *const xt_mtchk_param) -> c_int {
    let info: *mut xt_owner_match_info = (*par).matchinfo as *mut xt_owner_match_info;
    let net: *mut net = (*par).net;

    if (*info).match_ & !XT_OWNER_MASK != 0 {
        return -EINVAL;
    }

    /* Only allow the common case where the userns of the writer
     * matches the userns of the network namespace.
     */
    if ((*info).match_ & (XT_OWNER_UID | XT_OWNER_GID)) != 0
        && current_user_ns() != (*net).user_ns
    {
        return -EINVAL;
    }

    /* Ensure the uids are valid */
    if (*info).match_ & XT_OWNER_UID != 0 {
        let uid_min: kuid_t = make_kuid((*net).user_ns, (*info).uid_min);
        let uid_max: kuid_t = make_kuid((*net).user_ns, (*info).uid_max);

        if !uid_valid(uid_min)
            || !uid_valid(uid_max)
            || (*info).uid_max < (*info).uid_min
            || uid_lt(uid_max, uid_min)
        {
            return -EINVAL;
        }
    }

    /* Ensure the gids are valid */
    if (*info).match_ & XT_OWNER_GID != 0 {
        let gid_min: kgid_t = make_kgid((*net).user_ns, (*info).gid_min);
        let gid_max: kgid_t = make_kgid((*net).user_ns, (*info).gid_max);

        if !gid_valid(gid_min)
            || !gid_valid(gid_max)
            || (*info).gid_max < (*info).gid_min
            || gid_lt(gid_max, gid_min)
        {
            return -EINVAL;
        }
    }

    0
}

unsafe fn owner_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info: *const xt_owner_match_info = (*par).matchinfo as *const xt_owner_match_info;
    let sk: *mut sock = skb_to_full_sk(skb);
    let net: *mut net = xt_net(par);
    let mut sock: *const socket;
    let filp: *const file;

    if sk.is_null() || READ_ONCE((*sk).sk_socket).is_null() || !net_eq(net, sock_net(sk)) {
        return ((*info).match_ ^ (*info).invert) == 0;
    } else if ((*info).match_ & (*info).invert & XT_OWNER_SOCKET) != 0 {
        /*
         * Socket exists but user wanted ! --socket-exists.
         * (Single ampersands intended.)
         */
        return false;
    }

    /* The sk pointer remains valid as long as the skb is. The sk_socket and
     * file pointer may become NULL if the socket is closed. Both structures
     * (including file->cred) are RCU freed which means they can be accessed
     * within a RCU read section.
     */
    sock = READ_ONCE((*sk).sk_socket);
    filp = if !sock.is_null() { READ_ONCE((*sock).file) } else { core::ptr::null() };
    if filp.is_null() {
        return (((*info).match_ ^ (*info).invert) & (XT_OWNER_UID | XT_OWNER_GID)) == 0;
    }

    if (*info).match_ & XT_OWNER_UID != 0 {
        let uid_min: kuid_t = make_kuid((*net).user_ns, (*info).uid_min);
        let uid_max: kuid_t = make_kuid((*net).user_ns, (*info).uid_max);

        if (uid_gte((*(*filp).f_cred).fsuid, uid_min)
            && uid_lte((*(*filp).f_cred).fsuid, uid_max))
            ^ ((*info).invert & XT_OWNER_UID == 0)
        {
            return false;
        }
    }

    if (*info).match_ & XT_OWNER_GID != 0 {
        let mut i: c_uint;
        let mut matched = false;
        let gid_min: kgid_t = make_kgid((*net).user_ns, (*info).gid_min);
        let gid_max: kgid_t = make_kgid((*net).user_ns, (*info).gid_max);
        let gi: *mut group_info = (*(*filp).f_cred).group_info;

        if gid_gte((*(*filp).f_cred).fsgid, gid_min)
            && gid_lte((*(*filp).f_cred).fsgid, gid_max)
        {
            matched = true;
        }

        if !matched && ((*info).match_ & XT_OWNER_SUPPL_GROUPS != 0) && !gi.is_null() {
            i = 0;
            while i < (*gi).ngroups {
                let group: kgid_t = *(*gi).gid.add(i as usize);

                if gid_gte(group, gid_min) && gid_lte(group, gid_max) {
                    matched = true;
                    break;
                }
                i += 1;
            }
        }

        if matched ^ ((*info).invert & XT_OWNER_GID == 0) {
            return false;
        }
    }

    true
}

static mut owner_mt_reg: [xt_match; 2] = [
    xt_match {
        name: "owner",
        revision: 1,
        family: NFPROTO_IPV4,
        checkentry: Some(owner_check),
        r#match: Some(owner_mt),
        matchsize: core::mem::size_of::<xt_owner_match_info>(),
        hooks: (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_POST_ROUTING),
        me: THIS_MODULE,
    },
    xt_match {
        name: "owner",
        revision: 1,
        family: NFPROTO_IPV6,
        checkentry: Some(owner_check),
        r#match: Some(owner_mt),
        matchsize: core::mem::size_of::<xt_owner_match_info>(),
        hooks: (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_POST_ROUTING),
        me: THIS_MODULE,
    },
];

unsafe fn owner_mt_init() -> c_int {
    xt_register_matches(owner_mt_reg.as_mut_ptr(), owner_mt_reg.len())
}

unsafe fn owner_mt_exit() {
    xt_unregister_matches(owner_mt_reg.as_mut_ptr(), owner_mt_reg.len());
}

module_init!(owner_mt_init);
module_exit!(owner_mt_exit);
module_author!("Jan Engelhardt <jengelh@medozas.de>");
module_description!("Xtables: socket owner matching");
module_license!("GPL");
module_alias!("ipt_owner");
module_alias!("ip6t_owner");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
