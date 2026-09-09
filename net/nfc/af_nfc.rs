// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 Instituto Nokia de Tecnologia
 *
 * Authors:
 *    Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
 *    Lauro Ramos Venancio <lauro.venancio@openbossa.org>
 */

// C dependencies: <linux/nfc.h>, <linux/module.h>, and "nfc.h".
// Build-time kernel definitions and synchronization primitives are supplied externally.

static mut proto_tab_lock: RwLock = DEFINE_RWLOCK!();
static mut proto_tab: [*const nfc_protocol; NFC_SOCKPROTO_MAX as usize] =
    [core::ptr::null(); NFC_SOCKPROTO_MAX as usize];

unsafe fn nfc_sock_create(
    net: *mut net,
    sock: *mut socket,
    proto: c_int,
    kern: c_int,
) -> c_int {
    let mut rc: c_int = -EPROTONOSUPPORT;

    if net != &raw mut init_net {
        return -EAFNOSUPPORT;
    }

    if proto < 0 || proto >= NFC_SOCKPROTO_MAX {
        return -EINVAL;
    }

    read_lock(&raw mut proto_tab_lock);
    let nfc_proto = proto_tab[proto as usize];
    if !nfc_proto.is_null() && try_module_get((*nfc_proto).owner) {
        rc = ((*nfc_proto).create)(net, sock, nfc_proto, kern);
        module_put((*nfc_proto).owner);
    }
    read_unlock(&raw mut proto_tab_lock);

    rc
}

static nfc_sock_family_ops: net_proto_family = net_proto_family {
    owner: THIS_MODULE,
    family: PF_NFC,
    create: nfc_sock_create,
};

unsafe fn nfc_proto_register(nfc_proto: *const nfc_protocol) -> c_int {
    let mut rc: c_int;

    if (*nfc_proto).id < 0 || (*nfc_proto).id >= NFC_SOCKPROTO_MAX {
        return -EINVAL;
    }

    rc = proto_register((*nfc_proto).proto, 0);
    if rc != 0 {
        return rc;
    }

    write_lock(&raw mut proto_tab_lock);
    let id = (*nfc_proto).id as usize;
    if !proto_tab[id].is_null() {
        rc = -EBUSY;
    } else {
        proto_tab[id] = nfc_proto;
    }
    write_unlock(&raw mut proto_tab_lock);

    if rc != 0 {
        proto_unregister((*nfc_proto).proto);
    }

    rc
}

// EXPORT_SYMBOL(nfc_proto_register);

unsafe fn nfc_proto_unregister(nfc_proto: *const nfc_protocol) {
    write_lock(&raw mut proto_tab_lock);
    proto_tab[(*nfc_proto).id as usize] = core::ptr::null();
    write_unlock(&raw mut proto_tab_lock);

    proto_unregister((*nfc_proto).proto);
}

// EXPORT_SYMBOL(nfc_proto_unregister);

unsafe fn af_nfc_init() -> c_int {
    sock_register(&nfc_sock_family_ops)
}

unsafe fn af_nfc_exit() {
    sock_unregister(PF_NFC);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
