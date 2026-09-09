// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (C) 2018, LG Electronics.
 *   Copyright (c) 2025 Stefan Metzmacher
 */

// Dependencies supplied by the surrounding kernel/RDMA translation unit.

unsafe fn smbdirect_ib_device_rdma_capable_node_type(
    ib_dev: *mut ib_device,
) -> u8 {
    if !smbdirect_frwr_is_supported(unsafe { &(*ib_dev).attrs }) {
        return RDMA_NODE_UNSPECIFIED;
    }

    match unsafe { (*ib_dev).node_type } {
        RDMA_NODE_IB_CA | RDMA_NODE_RNIC => unsafe { (*ib_dev).node_type },
        _ => RDMA_NODE_UNSPECIFIED,
    }
}

unsafe fn smbdirect_ib_client_add(ib_dev: *mut ib_device) -> i32 {
    let mut node_type = smbdirect_ib_device_rdma_capable_node_type(ib_dev);
    let mut sdev: *mut smbdirect_device;
    let (node_str, action): (&str, &str);
    let mut pidx: u32;

    match node_type {
        RDMA_NODE_IB_CA => {
            node_str = "IB_CA";
            action = "added";
        }
        RDMA_NODE_RNIC => {
            node_str = "RNIC";
            action = "added";
        }
        RDMA_NODE_UNSPECIFIED => {
            node_str = "UNSPECIFIED";
            action = "ignored";
        }
        _ => {
            node_str = "UNKNOWN";
            action = "ignored";
            node_type = RDMA_NODE_UNSPECIFIED;
        }
    }

    pr_info!(
        "ib_dev[%.*s]: {}: {} {}={} {}=0x{:x} {}=0x{:x} {}=0x{:x}\n",
        IB_DEVICE_NAME_MAX, (*ib_dev).name, action, node_str,
        "max_fast_reg_page_list_len", (*ib_dev).attrs.max_fast_reg_page_list_len,
        "device_cap_flags", (*ib_dev).attrs.device_cap_flags,
        "kernel_cap_flags", (*ib_dev).attrs.kernel_cap_flags,
        "page_size_cap", (*ib_dev).attrs.page_size_cap
    );

    if node_type == RDMA_NODE_UNSPECIFIED {
        return 0;
    }

    pr_info!(
        "ib_dev[%.*s]: {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}\n",
        IB_DEVICE_NAME_MAX, (*ib_dev).name,
        "num_ports", rdma_end_port(ib_dev),
        "max_qp_rd_atom", (*ib_dev).attrs.max_qp_rd_atom,
        "max_qp_init_rd_atom", (*ib_dev).attrs.max_qp_init_rd_atom,
        "max_sgl_rd", (*ib_dev).attrs.max_sgl_rd,
        "max_sge_rd", (*ib_dev).attrs.max_sge_rd,
        "max_cqe", (*ib_dev).attrs.max_cqe,
        "max_qp_wr", (*ib_dev).attrs.max_qp_wr,
        "max_send_sge", (*ib_dev).attrs.max_send_sge,
        "max_recv_sge", (*ib_dev).attrs.max_recv_sge
    );

    rdma_for_each_port!(ib_dev, pidx, {
        let ib_pi = ib_port_immutable_read(ib_dev, pidx);
        let core_cap_flags: u32 = if !ib_pi.is_null() {
            (*ib_pi).core_cap_flags
        } else {
            0
        };
        pr_info!(
            "ib_dev[%.*s]PORT[{}]: {}={} {}={} {}={} {}={} {}={} {}=0x{:x}\n",
            IB_DEVICE_NAME_MAX, (*ib_dev).name, pidx,
            "iwarp", rdma_protocol_iwarp(ib_dev, pidx),
            "ib", rdma_protocol_ib(ib_dev, pidx),
            "roce", rdma_protocol_roce(ib_dev, pidx),
            "v1", rdma_protocol_roce_eth_encap(ib_dev, pidx),
            "v2", rdma_protocol_roce_udp_encap(ib_dev, pidx),
            "core_cap_flags", core_cap_flags
        );
    });

    sdev = kzalloc_obj!(*sdev);
    if sdev.is_null() {
        return -ENOMEM;
    }
    (*sdev).ib_dev = ib_dev;
    snprintf!((*sdev).ib_name, ARRAY_SIZE!((*sdev).ib_name), "%.*s",
        IB_DEVICE_NAME_MAX, (*ib_dev).name);

    write_lock!(&mut smbdirect_globals.devices.lock);
    list_add!(&mut (*sdev).list, &mut smbdirect_globals.devices.list);
    write_unlock!(&mut smbdirect_globals.devices.lock);

    0
}

unsafe fn smbdirect_ib_client_remove(ib_dev: *mut ib_device, _client_data: *mut core::ffi::c_void) {
    let mut sdev: *mut smbdirect_device;
    let mut tmp: *mut smbdirect_device;

    write_lock!(&mut smbdirect_globals.devices.lock);
    list_for_each_entry_safe!(sdev, tmp, &mut smbdirect_globals.devices.list, list, {
        if (*sdev).ib_dev == ib_dev {
            list_del!(&mut (*sdev).list);
            pr_info!("ib_dev[%.*s] removed\n", IB_DEVICE_NAME_MAX, (*sdev).ib_name);
            kfree!(sdev);
            break;
        }
    });
    write_unlock!(&mut smbdirect_globals.devices.lock);
}

unsafe fn smbdirect_ib_client_rename(ib_dev: *mut ib_device, _client_data: *mut core::ffi::c_void) {
    let mut sdev: *mut smbdirect_device;

    write_lock!(&mut smbdirect_globals.devices.lock);
    list_for_each_entry!(sdev, &mut smbdirect_globals.devices.list, list, {
        if (*sdev).ib_dev == ib_dev {
            pr_info!("ib_dev[%.*s] renamed to [%.*s]\n",
                IB_DEVICE_NAME_MAX, (*sdev).ib_name,
                IB_DEVICE_NAME_MAX, (*ib_dev).name);
            snprintf!((*sdev).ib_name, ARRAY_SIZE!((*sdev).ib_name), "%.*s",
                IB_DEVICE_NAME_MAX, (*ib_dev).name);
            break;
        }
    });
    write_unlock!(&mut smbdirect_globals.devices.lock);
}

static mut smbdirect_ib_client: ib_client = ib_client {
    name: "smbdirect_ib_client",
    add: Some(smbdirect_ib_client_add),
    remove: Some(smbdirect_ib_client_remove),
    rename: Some(smbdirect_ib_client_rename),
};

unsafe fn smbdirect_netdev_find_rdma_capable_node_type(netdev: *mut net_device) -> u8 {
    let mut sdev: *mut smbdirect_device;
    let mut node_type = RDMA_NODE_UNSPECIFIED;

    read_lock!(&smbdirect_globals.devices.lock);
    list_for_each_entry!(sdev, &smbdirect_globals.devices.list, list, {
        let mut pi: u32;
        rdma_for_each_port!((*sdev).ib_dev, pi, {
            let ndev = ib_device_get_netdev((*sdev).ib_dev, pi);
            if ndev.is_null() { continue; }
            if ndev == netdev {
                dev_put(ndev);
                node_type = (*(*sdev).ib_dev).node_type;
                goto_out!();
            }
            dev_put(ndev);
        });
    });
    goto_out_label!();
    read_unlock!(&smbdirect_globals.devices.lock);

    if node_type == RDMA_NODE_UNSPECIFIED {
        let ibdev = ib_device_get_by_netdev(netdev, RDMA_DRIVER_UNKNOWN);
        if !ibdev.is_null() {
            node_type = smbdirect_ib_device_rdma_capable_node_type(ibdev);
            ib_device_put(ibdev);
        }
    }
    node_type
}

/* Returns RDMA_NODE_UNSPECIFIED when the netdev has no support for smbdirect capable rdma. */
pub unsafe fn smbdirect_netdev_rdma_capable_node_type(netdev: *mut net_device) -> u8 {
    let mut lower_dev: *mut net_device;
    let mut iter: *mut list_head;
    let mut node_type = smbdirect_netdev_find_rdma_capable_node_type(netdev);
    if node_type != RDMA_NODE_UNSPECIFIED { return node_type; }

    /* check if netdev is bridge or VLAN */
    if netif_is_bridge_master(netdev) || ((*netdev).priv_flags & IFF_802_1Q_VLAN) != 0 {
        netdev_for_each_lower_dev!(netdev, lower_dev, iter, {
            node_type = smbdirect_netdev_find_rdma_capable_node_type(lower_dev);
            if node_type != RDMA_NODE_UNSPECIFIED { return node_type; }
        });
    }

    /* check if netdev is IPoIB safely without layer violation */
    if (*netdev).type_ == ARPHRD_INFINIBAND { return RDMA_NODE_IB_CA; }
    RDMA_NODE_UNSPECIFIED
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
