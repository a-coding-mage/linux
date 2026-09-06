// SPDX-License-Identifier: GPL-2.0-only
/*
 * Netlink message type permission tables, for user generated messages.
 *
 * Author: James Morris <jmorris@redhat.com>
 *
 * Copyright (C) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
 */

use core::mem::size_of;

use crate::*;

#[repr(C)]
struct nlmsg_perm {
    nlmsg_type: u16,
    perm: u32,
}

static nlmsg_route_perms: &[nlmsg_perm] = &[
    nlmsg_perm { nlmsg_type: RTM_NEWLINK as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELLINK as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETLINK as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_SETLINK as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_NEWADDR as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELADDR as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETADDR as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWROUTE as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELROUTE as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETROUTE as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWNEIGH as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELNEIGH as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETNEIGH as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWRULE as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELRULE as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETRULE as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWQDISC as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELQDISC as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETQDISC as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWTCLASS as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELTCLASS as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETTCLASS as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWTFILTER as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELTFILTER as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETTFILTER as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWACTION as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELACTION as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETACTION as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWPREFIX as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETMULTICAST as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_GETANYCAST as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_GETNEIGHTBL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_SETNEIGHTBL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_NEWADDRLABEL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELADDRLABEL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETADDRLABEL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_GETDCB as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_SETDCB as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_NEWNETCONF as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELNETCONF as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETNETCONF as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWMDB as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELMDB as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETMDB as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWNSID as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELNSID as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_GETNSID as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWSTATS as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_GETSTATS as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_SETSTATS as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_NEWCACHEREPORT as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWCHAIN as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELCHAIN as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETCHAIN as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWNEXTHOP as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELNEXTHOP as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETNEXTHOP as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWLINKPROP as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELLINKPROP as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_NEWVLAN as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELVLAN as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETVLAN as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWNEXTHOPBUCKET as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELNEXTHOPBUCKET as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETNEXTHOPBUCKET as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: RTM_NEWTUNNEL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_DELTUNNEL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: RTM_GETTUNNEL as u16, perm: NETLINK_ROUTE_SOCKET__NLMSG_READ },
];

static nlmsg_tcpdiag_perms: &[nlmsg_perm] = &[
    nlmsg_perm { nlmsg_type: TCPDIAG_GETSOCK as u16, perm: NETLINK_TCPDIAG_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: SOCK_DIAG_BY_FAMILY as u16, perm: NETLINK_TCPDIAG_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: SOCK_DESTROY as u16, perm: NETLINK_TCPDIAG_SOCKET__NLMSG_WRITE },
];

static nlmsg_xfrm_perms: &[nlmsg_perm] = &[
    nlmsg_perm { nlmsg_type: XFRM_MSG_NEWSA as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_DELSA as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_GETSA as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_NEWPOLICY as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_DELPOLICY as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_GETPOLICY as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_ALLOCSPI as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_ACQUIRE as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_EXPIRE as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_UPDPOLICY as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_UPDSA as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_POLEXPIRE as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_FLUSHSA as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_FLUSHPOLICY as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_NEWAE as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_GETAE as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_REPORT as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_MIGRATE as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_NEWSADINFO as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_GETSADINFO as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_NEWSPDINFO as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_GETSPDINFO as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_MAPPING as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_SETDEFAULT as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: XFRM_MSG_GETDEFAULT as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: XFRM_MSG_MIGRATE_STATE as u16, perm: NETLINK_XFRM_SOCKET__NLMSG_WRITE },
];

static nlmsg_audit_perms: &[nlmsg_perm] = &[
    nlmsg_perm { nlmsg_type: AUDIT_GET as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: AUDIT_SET as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: AUDIT_LIST as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_READPRIV },
    nlmsg_perm { nlmsg_type: AUDIT_ADD as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: AUDIT_DEL as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: AUDIT_LIST_RULES as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_READPRIV },
    nlmsg_perm { nlmsg_type: AUDIT_ADD_RULE as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: AUDIT_DEL_RULE as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: AUDIT_USER as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_RELAY },
    nlmsg_perm { nlmsg_type: AUDIT_SIGNAL_INFO as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: AUDIT_TRIM as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: AUDIT_MAKE_EQUIV as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
    nlmsg_perm { nlmsg_type: AUDIT_TTY_GET as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: AUDIT_TTY_SET as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_TTY_AUDIT },
    nlmsg_perm { nlmsg_type: AUDIT_GET_FEATURE as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_READ },
    nlmsg_perm { nlmsg_type: AUDIT_SET_FEATURE as u16, perm: NETLINK_AUDIT_SOCKET__NLMSG_WRITE },
];

fn nlmsg_perm(nlmsg_type: u16, perm: *mut u32, tab: *const nlmsg_perm, tabsize: usize) -> i32 {
    let mut i: u32;
    let mut err: i32 = -EINVAL;

    i = 0;
    while (i as usize) < tabsize / size_of::<nlmsg_perm>() {
        unsafe {
            if nlmsg_type == (*tab.add(i as usize)).nlmsg_type {
                *perm = (*tab.add(i as usize)).perm;
                err = 0;
                break;
            }
        }
        i += 1;
    }

    err
}

pub fn selinux_nlmsg_lookup(sclass: u16, nlmsg_type: u16, perm: *mut u32) -> i32 {
    /* While it is possible to add a similar permission to other netlink
     * classes, note that the extended permission value is matched against
     * the nlmsg_type field. Notably, SECCLASS_NETLINK_GENERIC_SOCKET uses
     * dynamic values for this field, which means that it cannot be added
     * as-is.
     */

    match sclass {
        SECCLASS_NETLINK_ROUTE_SOCKET => {
            /* RTM_MAX always points to RTM_SETxxxx, ie RTM_NEWxxx + 3.
             * If the BUILD_BUG_ON() below fails you must update the
             * structures at the top of this file with the new mappings
             * before updating the BUILD_BUG_ON() macro!
             */
            // BUILD_BUG_ON(RTM_MAX != (RTM_NEWTUNNEL + 3));

            if selinux_policycap_netlink_xperm() {
                unsafe {
                    *perm = NETLINK_ROUTE_SOCKET__NLMSG;
                }
                return 0;
            }
            return nlmsg_perm(
                nlmsg_type,
                perm,
                nlmsg_route_perms.as_ptr(),
                core::mem::size_of_val(nlmsg_route_perms),
            );
        }
        SECCLASS_NETLINK_TCPDIAG_SOCKET => {
            if selinux_policycap_netlink_xperm() {
                unsafe {
                    *perm = NETLINK_TCPDIAG_SOCKET__NLMSG;
                }
                return 0;
            }
            return nlmsg_perm(
                nlmsg_type,
                perm,
                nlmsg_tcpdiag_perms.as_ptr(),
                core::mem::size_of_val(nlmsg_tcpdiag_perms),
            );
        }
        SECCLASS_NETLINK_XFRM_SOCKET => {
            /* If the BUILD_BUG_ON() below fails you must update the
             * structures at the top of this file with the new mappings
             * before updating the BUILD_BUG_ON() macro!
             */
            // BUILD_BUG_ON(XFRM_MSG_MAX != XFRM_MSG_MIGRATE_STATE);

            if selinux_policycap_netlink_xperm() {
                unsafe {
                    *perm = NETLINK_XFRM_SOCKET__NLMSG;
                }
                return 0;
            }
            return nlmsg_perm(
                nlmsg_type,
                perm,
                nlmsg_xfrm_perms.as_ptr(),
                core::mem::size_of_val(nlmsg_xfrm_perms),
            );
        }
        SECCLASS_NETLINK_AUDIT_SOCKET => {
            if selinux_policycap_netlink_xperm() {
                unsafe {
                    *perm = NETLINK_AUDIT_SOCKET__NLMSG;
                }
                return 0;
            } else if (nlmsg_type >= AUDIT_FIRST_USER_MSG as u16
                && nlmsg_type <= AUDIT_LAST_USER_MSG as u16)
                || (nlmsg_type >= AUDIT_FIRST_USER_MSG2 as u16
                    && nlmsg_type <= AUDIT_LAST_USER_MSG2 as u16)
            {
                unsafe {
                    *perm = NETLINK_AUDIT_SOCKET__NLMSG_RELAY;
                }
                return 0;
            }
            return nlmsg_perm(
                nlmsg_type,
                perm,
                nlmsg_audit_perms.as_ptr(),
                core::mem::size_of_val(nlmsg_audit_perms),
            );
        }
        _ => {}
    }

    /* No messaging from userspace, or class unknown/unhandled */
    -ENOENT
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
