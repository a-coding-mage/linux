/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel translation units:
// linux/netfilter/nf_conntrack_zones_common.h
// net/netfilter/nf_conntrack.h

pub unsafe fn nf_ct_zone(ct: *const nf_conn) -> *const nf_conntrack_zone {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_ZONES")]
    {
        &(*ct).zone
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_ZONES"))]
    {
        &nf_ct_zone_dflt
    }
}

pub unsafe fn nf_ct_zone_init(
    zone: *mut nf_conntrack_zone,
    id: u16,
    dir: u8,
    flags: u8,
) -> *const nf_conntrack_zone {
    (*zone).id = id;
    (*zone).flags = flags;
    (*zone).dir = dir;

    zone
}

pub unsafe fn nf_ct_zone_tmpl(
    tmpl: *const nf_conn,
    skb: *const sk_buff,
    tmp: *mut nf_conntrack_zone,
) -> *const nf_conntrack_zone {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_ZONES")]
    {
        if tmpl.is_null() {
            return &nf_ct_zone_dflt;
        }

        if (*tmpl).zone.flags & NF_CT_FLAG_MARK != 0 {
            return nf_ct_zone_init(tmp, (*skb).mark, (*tmpl).zone.dir, 0);
        }
    }
    nf_ct_zone(tmpl)
}

pub unsafe fn nf_ct_zone_add(
    ct: *mut nf_conn,
    zone: *const nf_conntrack_zone,
) {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_ZONES")]
    {
        (*ct).zone = *zone;
    }
}

pub unsafe fn nf_ct_zone_matches_dir(
    zone: *const nf_conntrack_zone,
    dir: ip_conntrack_dir,
) -> bool {
    (*zone).dir & (1u8 << (dir as u32)) != 0
}

pub unsafe fn nf_ct_zone_id(
    zone: *const nf_conntrack_zone,
    dir: ip_conntrack_dir,
) -> u16 {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_ZONES")]
    {
        return if nf_ct_zone_matches_dir(zone, dir) {
            (*zone).id
        } else {
            NF_CT_DEFAULT_ZONE_ID
        };
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_ZONES"))]
    {
        NF_CT_DEFAULT_ZONE_ID
    }
}

pub unsafe fn nf_ct_zone_equal(
    a: *const nf_conn,
    b: *const nf_conntrack_zone,
    dir: ip_conntrack_dir,
) -> bool {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_ZONES")]
    {
        return nf_ct_zone_id(nf_ct_zone(a), dir) == nf_ct_zone_id(b, dir);
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_ZONES"))]
    {
        true
    }
}

pub unsafe fn nf_ct_zone_equal_any(
    a: *const nf_conn,
    b: *const nf_conntrack_zone,
) -> bool {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_ZONES")]
    {
        return (*nf_ct_zone(a)).id == (*b).id;
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_ZONES"))]
    {
        true
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
