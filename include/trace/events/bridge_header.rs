// Translated from the Linux bridge tracepoint header.
// Required kernel types, constants, helpers, and the trace-event machinery are
// supplied by the surrounding translation unit.

// The original include guard and TRACE_HEADER_MULTI_READ conditional are
// intentionally represented by the surrounding build system.

trace_event! {
    br_fdb_add {
        proto(ndm: *mut ndmsg, dev: *mut net_device, addr: *const u8,
              vid: u16, nlh_flags: u16);
        entry {
            ndm_flags: u8,
            dev: string,
            addr: [u8; ETH_ALEN],
            vid: u16,
            nlh_flags: u16,
        }
        assign {
            assign_str!(dev);
            unsafe { memcpy(entry.addr.as_mut_ptr(), addr, ETH_ALEN); }
            entry.vid = vid;
            entry.nlh_flags = nlh_flags;
            entry.ndm_flags = unsafe { (*ndm).ndm_flags };
        }
        print("dev %s addr %02x:%02x:%02x:%02x:%02x:%02x vid %u nlh_flags %04x ndm_flags %02x",
              dev, entry.addr[0], entry.addr[1], entry.addr[2], entry.addr[3],
              entry.addr[4], entry.addr[5], entry.vid, entry.nlh_flags,
              entry.ndm_flags);
    }

    br_fdb_external_learn_add {
        proto(br: *mut net_bridge, p: *mut net_bridge_port, addr: *const u8,
              vid: u16);
        entry {
            br_dev: string,
            dev: string,
            addr: [u8; ETH_ALEN],
            vid: u16,
        }
        assign {
            assign_str!(br_dev);
            assign_str!(dev);
            unsafe { memcpy(entry.addr.as_mut_ptr(), addr, ETH_ALEN); }
            entry.vid = vid;
        }
        print("br_dev %s port %s addr %02x:%02x:%02x:%02x:%02x:%02x vid %u",
              br_dev, dev, entry.addr[0], entry.addr[1], entry.addr[2],
              entry.addr[3], entry.addr[4], entry.addr[5], entry.vid);
    }

    fdb_delete {
        proto(br: *mut net_bridge, f: *mut net_bridge_fdb_entry);
        entry {
            br_dev: string,
            dev: string,
            addr: [u8; ETH_ALEN],
            vid: u16,
        }
        assign {
            assign_str!(br_dev);
            assign_str!(dev);
            unsafe { memcpy(entry.addr.as_mut_ptr(), (*f).key.addr.addr.as_ptr(), ETH_ALEN); }
            entry.vid = unsafe { (*f).key.vlan_id };
        }
        print("br_dev %s dev %s addr %02x:%02x:%02x:%02x:%02x:%02x vid %u",
              br_dev, dev, entry.addr[0], entry.addr[1], entry.addr[2],
              entry.addr[3], entry.addr[4], entry.addr[5], entry.vid);
    }

    br_fdb_update {
        proto(br: *mut net_bridge, source: *mut net_bridge_port, addr: *const u8,
              vid: u16, flags: c_ulong);
        entry {
            br_dev: string,
            dev: string,
            addr: [u8; ETH_ALEN],
            vid: u16,
            flags: c_ulong,
        }
        assign {
            assign_str!(br_dev);
            assign_str!(dev);
            unsafe { memcpy(entry.addr.as_mut_ptr(), addr, ETH_ALEN); }
            entry.vid = vid;
            entry.flags = flags;
        }
        print("br_dev %s source %s addr %02x:%02x:%02x:%02x:%02x:%02x vid %u flags 0x%lx",
              br_dev, dev, entry.addr[0], entry.addr[1], entry.addr[2],
              entry.addr[3], entry.addr[4], entry.addr[5], entry.vid, entry.flags);
    }

    br_mdb_full {
        proto(dev: *const net_device, group: *const br_ip);
        entry {
            dev: string,
            af: c_int,
            vid: u16,
            src: [u8; 16],
            grp: [u8; 16],
            grpmac: [u8; ETH_ALEN], // For af == 0.
        }
        assign {
            let mut in6: *mut in6_addr;
            assign_str!(dev);
            entry.vid = unsafe { (*group).vid };
            if unsafe { (*group).proto == 0 } {
                entry.af = 0;
                unsafe {
                    memset(entry.src.as_mut_ptr(), 0, entry.src.len());
                    memset(entry.grp.as_mut_ptr(), 0, entry.grp.len());
                    memcpy(entry.grpmac.as_mut_ptr(), (*group).dst.mac_addr.as_ptr(), ETH_ALEN);
                }
            } else if unsafe { (*group).proto == htons(ETH_P_IP) } {
                entry.af = AF_INET;
                in6 = entry.src.as_mut_ptr() as *mut in6_addr;
                unsafe { ipv6_addr_set_v4mapped((*group).src.ip4, in6); }
                in6 = entry.grp.as_mut_ptr() as *mut in6_addr;
                unsafe { ipv6_addr_set_v4mapped((*group).dst.ip4, in6); }
                unsafe { memset(entry.grpmac.as_mut_ptr(), 0, ETH_ALEN); }
            // CONFIG_IPV6 condition from the source is preserved by this branch.
            } else {
                entry.af = AF_INET6;
                in6 = entry.src.as_mut_ptr() as *mut in6_addr;
                unsafe { *in6 = (*group).src.ip6; }
                in6 = entry.grp.as_mut_ptr() as *mut in6_addr;
                unsafe { *in6 = (*group).dst.ip6; }
                unsafe { memset(entry.grpmac.as_mut_ptr(), 0, ETH_ALEN); }
            }
        }
        print("dev %s af %u src %pI6c grp %pI6c/%pM vid %u",
              dev, entry.af, entry.src, entry.grp, entry.grpmac, entry.vid);
    }
}

// define_trace.h is intentionally handled by the surrounding build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
