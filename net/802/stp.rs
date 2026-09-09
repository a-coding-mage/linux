// SPDX-License-Identifier: GPL-2.0-only
/*
 *	STP SAP demux
 *
 *	Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the corresponding kernel networking interfaces.

/* 01:80:c2:00:00:20 - 01:80:c2:00:00:2F */
const GARP_ADDR_MIN: usize = 0x20;
const GARP_ADDR_MAX: usize = 0x2F;
const GARP_ADDR_RANGE: usize = GARP_ADDR_MAX - GARP_ADDR_MIN;

static mut garp_protos: [*const stp_proto; GARP_ADDR_RANGE + 1] =
    [core::ptr::null(); GARP_ADDR_RANGE + 1];
static mut stp_proto: *const stp_proto = core::ptr::null();

static mut sap: *mut llc_sap = core::ptr::null_mut();
static mut sap_registered: u32 = 0;

// DEFINE_MUTEX(stp_proto_mutex)
extern "C" {
    static mut stp_proto_mutex: mutex;
}

/* Called under rcu_read_lock from LLC */
unsafe fn stp_pdu_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
    _pt: *mut packet_type,
    _orig_dev: *mut net_device,
) -> i32 {
    let eh: *const ethhdr = eth_hdr(skb);
    let pdu: *const llc_pdu_un = llc_pdu_un_hdr(skb);
    let proto: *const stp_proto;

    if (*pdu).ssap != LLC_SAP_BSPAN
        || (*pdu).dsap != LLC_SAP_BSPAN
        || (*pdu).ctrl_1 != LLC_PDU_TYPE_U
    {
        goto_err(skb);
        return 0;
    }

    if (*eh).h_dest[5] >= GARP_ADDR_MIN as u8 && (*eh).h_dest[5] <= GARP_ADDR_MAX as u8 {
        proto = rcu_dereference(garp_protos[(*eh).h_dest[5] as usize - GARP_ADDR_MIN]);
        if !proto.is_null() && !ether_addr_equal((*eh).h_dest.as_ptr(), (*proto).group_address.as_ptr()) {
            goto_err(skb);
            return 0;
        }
    } else {
        proto = rcu_dereference(stp_proto);
    }

    if proto.is_null() {
        goto_err(skb);
        return 0;
    }

    ((*proto).rcv)(proto, skb, dev);
    return 0;
}

unsafe fn goto_err(skb: *mut sk_buff) {
    kfree_skb(skb);
}

unsafe fn stp_proto_register(proto: *const stp_proto) -> i32 {
    let mut err: i32 = 0;

    mutex_lock(&raw mut stp_proto_mutex);
    let was_zero = sap_registered == 0;
    sap_registered = sap_registered.wrapping_add(1);
    if was_zero {
        sap = llc_sap_open(LLC_SAP_BSPAN, stp_pdu_rcv);
        if sap.is_null() {
            err = -ENOMEM;
            mutex_unlock(&raw mut stp_proto_mutex);
            return err;
        }
    }
    if is_zero_ether_addr((*proto).group_address.as_ptr()) {
        rcu_assign_pointer(&raw mut stp_proto, proto);
    } else {
        rcu_assign_pointer(
            &raw mut garp_protos[(*proto).group_address[5] as usize - GARP_ADDR_MIN],
            proto,
        );
    }
    mutex_unlock(&raw mut stp_proto_mutex);
    err
}

unsafe fn stp_proto_unregister(proto: *const stp_proto) {
    mutex_lock(&raw mut stp_proto_mutex);
    if is_zero_ether_addr((*proto).group_address.as_ptr()) {
        RCU_INIT_POINTER(&raw mut stp_proto, core::ptr::null());
    } else {
        RCU_INIT_POINTER(
            &raw mut garp_protos[(*proto).group_address[5] as usize - GARP_ADDR_MIN],
            core::ptr::null(),
        );
    }
    synchronize_rcu();

    sap_registered = sap_registered.wrapping_sub(1);
    if sap_registered == 0 {
        llc_sap_put(sap);
    }
    mutex_unlock(&raw mut stp_proto_mutex);
}

// EXPORT_SYMBOL_GPL(stp_proto_register);
// EXPORT_SYMBOL_GPL(stp_proto_unregister);
// MODULE_DESCRIPTION("SAP demux for IEEE 802.1D Spanning Tree Protocol (STP)");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
