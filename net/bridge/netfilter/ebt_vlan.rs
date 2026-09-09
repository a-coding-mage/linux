// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Description: EBTables 802.1Q match extension kernelspace module.
 * Authors: Nick Fedchik <nick@fedchik.org.ua>
 *          Bart De Schuymer <bdschuym@pandora.be>
 */

// Kernel headers and build-time declarations are supplied by the surrounding
// translation unit.

const MODULE_VERS: &[u8] = b"0.6\0";

// MODULE_AUTHOR("Nick Fedchik <nick@fedchik.org.ua>");
// MODULE_DESCRIPTION("Ebtables: 802.1Q VLAN tag match");
// MODULE_LICENSE("GPL");

static unsafe fn ebt_vlan_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info: *const ebt_vlan_info = (*par).matchinfo as *const ebt_vlan_info;

    let mut tci: u16; // Whole TCI, given from parsed frame
    let mut id: u16; // VLAN ID, given from frame TCI
    let mut prio: u8; // user_priority, given from frame TCI
    // VLAN encapsulated Type/Length field, given from orig frame
    let mut encap: __be16;

    if skb_vlan_tag_present(skb) {
        tci = skb_vlan_tag_get(skb);
        encap = (*skb).protocol;
    } else {
        let mut frame: vlan_hdr = core::mem::zeroed();
        let fp: *const vlan_hdr = skb_header_pointer(
            skb,
            0,
            core::mem::size_of::<vlan_hdr>(),
            &mut frame as *mut vlan_hdr as *mut core::ffi::c_void,
        );
        if fp.is_null() {
            return false;
        }

        tci = ntohs((*fp).h_vlan_TCI);
        encap = (*fp).h_vlan_encapsulated_proto;
    }

    /* Tag Control Information (TCI) consists of the following elements:
     * - User_priority. The user_priority field is three bits in length,
     * interpreted as a binary number.
     * - Canonical Format Indicator (CFI). The Canonical Format Indicator
     * (CFI) is a single bit flag value. Currently ignored.
     * - VLAN Identifier (VID). The VID is encoded as
     * an unsigned binary number.
     */
    id = tci & VLAN_VID_MASK;
    prio = ((tci >> 13) & 0x7) as u8;

    /* Checking VLAN Identifier (VID) */
    if (*info).bitmask & EBT_VLAN_ID != 0 {
        if !(((*info).id == id) ^ ((*info).invflags & EBT_VLAN_ID != 0)) {
            return false;
        }
    }

    /* Checking user_priority */
    if (*info).bitmask & EBT_VLAN_PRIO != 0 {
        if !(((*info).prio == prio) ^ ((*info).invflags & EBT_VLAN_PRIO != 0)) {
            return false;
        }
    }

    /* Checking Encapsulated Proto (Length/Type) field */
    if *info.bitmask & EBT_VLAN_ENCAP != 0 {
        if !(((*info).encap == encap) ^ ((*info).invflags & EBT_VLAN_ENCAP != 0)) {
            return false;
        }
    }

    true
}

static unsafe fn ebt_vlan_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info: *mut ebt_vlan_info = (*par).matchinfo as *mut ebt_vlan_info;
    let e: *const ebt_entry = (*par).entryinfo;

    /* Is it 802.1Q frame checked? */
    if (*e).ethproto != htons(ETH_P_8021Q) {
        pr_debug!("passed entry proto %2.4X is not 802.1Q (8100)\n", ntohs((*e).ethproto));
        return -EINVAL;
    }

    /* Check for bitmask range */
    if (*info).bitmask & !EBT_VLAN_MASK != 0 {
        pr_debug!("bitmask %2X is out of mask (%2X)\n", (*info).bitmask, EBT_VLAN_MASK);
        return -EINVAL;
    }

    /* Check for inversion flags range */
    if (*info).invflags & !EBT_VLAN_MASK != 0 {
        pr_debug!("inversion flags %2X is out of mask (%2X)\n", (*info).invflags, EBT_VLAN_MASK);
        return -EINVAL;
    }

    /* Reserved VLAN ID (VID) values */
    if (*info).bitmask & EBT_VLAN_ID != 0 {
        if (*info).id != 0 {
            if (*info).id > VLAN_N_VID {
                pr_debug!("id %d is out of range (1-4096)\n", (*info).id);
                return -EINVAL;
            }
            (*info).bitmask &= !EBT_VLAN_PRIO;
        }
    }

    if (*info).bitmask & EBT_VLAN_PRIO != 0 {
        if (*info).prio as u8 > 7 {
            pr_debug!("prio %d is out of range (0-7)\n", (*info).prio);
            return -EINVAL;
        }
    }

    if (*info).bitmask & EBT_VLAN_ENCAP != 0 {
        if ntohs((*info).encap) < ETH_ZLEN {
            pr_debug!("encap frame length %d is less than minimal\n", ntohs((*info).encap));
            return -EINVAL;
        }
    }

    0
}

static mut ebt_vlan_mt_reg: xt_match = xt_match {
    name: b"vlan\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_vlan_mt),
    checkentry: Some(ebt_vlan_mt_check),
    matchsize: core::mem::size_of::<ebt_vlan_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_vlan_init() -> c_int {
    pr_debug!("ebtables 802.1Q extension module v{}\n", core::str::from_utf8_unchecked(&MODULE_VERS[..MODULE_VERS.len() - 1]));
    xt_register_match(&mut ebt_vlan_mt_reg)
}

unsafe fn ebt_vlan_fini() {
    xt_unregister_match(&mut ebt_vlan_mt_reg);
}

// module_init(ebt_vlan_init);
// module_exit(ebt_vlan_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
