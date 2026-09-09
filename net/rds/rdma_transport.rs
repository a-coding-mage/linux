/*
 * Copyright (c) 2009, 2018 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available under a choice of one of two licenses: the GNU
 * General Public License (GPL) Version 2, or the OpenIB.org BSD license.
 */

// Dependencies supplied by the surrounding kernel/RDMA implementation are
// intentionally referenced here rather than reimplemented.

static mut rds_rdma_listen_id: *mut rdma_cm_id = core::ptr::null_mut();
#[cfg(feature = "ipv6")]
static mut rds6_rdma_listen_id: *mut rdma_cm_id = core::ptr::null_mut();

#[inline]
const fn tos_to_sl(tos: u8) -> u8 {
    tos & 0xF
}

unsafe fn rds_rdma_cm_event_handler_cmn(
    cm_id: *mut rdma_cm_id,
    event: *mut rdma_cm_event,
    isv6: bool,
) -> i32 {
    // this can be null in the listening path
    let conn = (*cm_id).context as *mut rds_connection;
    let mut trans: *mut rds_transport = core::ptr::null_mut();
    let mut ret: i32 = 0;
    let mut err: *mut i32;
    let mut len: u8 = 0;

    rdsdebug!("conn %p id %p handling event %u (%s)\n", conn, cm_id,
              (*event).event, rdma_event_msg((*event).event));

    if (*(*cm_id).device).node_type == RDMA_NODE_IB_CA {
        trans = &raw mut rds_ib_transport;
    }

    // Prevent shutdown from tearing down the connection while executing.
    if !conn.is_null() {
        mutex_lock(&raw mut (*conn).c_cm_lock);

        // If the connection is being shut down, bail out immediately.
        if rds_conn_state(conn) == RDS_CONN_DISCONNECTING {
            if (*event).event == RDMA_CM_EVENT_CONNECT_REQUEST {
                ret = 1;
            }
            goto_out!();
        }
    }

    match (*event).event {
        RDMA_CM_EVENT_CONNECT_REQUEST => {
            ret = ((*trans).cm_handle_connect)(cm_id, event, isv6);
        }
        RDMA_CM_EVENT_ADDR_RESOLVED => {
            if !conn.is_null() {
                rdma_set_service_type(cm_id, (*conn).c_tos);
                rdma_set_min_rnr_timer(cm_id, IB_RNR_TIMER_000_32);
                ret = rdma_resolve_route(cm_id, RDS_RDMA_RESOLVE_TIMEOUT_MS);
            }
        }
        RDMA_CM_EVENT_ROUTE_RESOLVED => {
            if !conn.is_null() {
                let ibic = (*conn).c_transport_data as *mut rds_ib_connection;
                if !ibic.is_null() && (*ibic).i_cm_id == cm_id {
                    (*cm_id).route.path_rec[0].sl = tos_to_sl((*conn).c_tos);
                    ret = ((*trans).cm_initiate_connect)(cm_id, isv6);
                } else {
                    rds_conn_drop(conn);
                }
            }
        }
        RDMA_CM_EVENT_ESTABLISHED => {
            if !conn.is_null() {
                ((*trans).cm_connect_complete)(conn, event);
            }
        }
        RDMA_CM_EVENT_REJECTED => {
            if conn.is_null() {
                goto_out!();
            }
            err = rdma_consumer_reject_data(cm_id, event, &mut len) as *mut i32;
            if err.is_null() || (len as usize >= core::mem::size_of::<i32>() && *err <= RDS_RDMA_REJ_INCOMPAT) {
                pr_warn!("RDS/RDMA: conn rejected, dropping connection\n");
                if (*conn).c_tos == 0 {
                    (*conn).c_proposed_version = RDS_PROTOCOL_COMPAT_VERSION;
                }
                rds_conn_drop(conn);
            }
            rdsdebug!("Connection rejected: %s\n", rdma_reject_msg(cm_id, (*event).status));
        }
        RDMA_CM_EVENT_ADDR_ERROR | RDMA_CM_EVENT_ROUTE_ERROR |
        RDMA_CM_EVENT_CONNECT_ERROR | RDMA_CM_EVENT_UNREACHABLE |
        RDMA_CM_EVENT_DEVICE_REMOVAL | RDMA_CM_EVENT_ADDR_CHANGE => {
            if !conn.is_null() { rds_conn_drop(conn); }
        }
        RDMA_CM_EVENT_DISCONNECTED => {
            if !conn.is_null() {
                rdsdebug!("DISCONNECT event - dropping connection\n");
                rds_conn_drop(conn);
            }
        }
        RDMA_CM_EVENT_TIMEWAIT_EXIT => {
            if !conn.is_null() {
                pr_info!("RDS: RDMA_CM_EVENT_TIMEWAIT_EXIT event: dropping connection\n");
                rds_conn_drop(conn);
            }
        }
        _ => {
            printk!(KERN_ERR, "RDS: unknown event %u (%s)!\n", (*event).event,
                    rdma_event_msg((*event).event));
        }
    }

    if !conn.is_null() { mutex_unlock(&raw mut (*conn).c_cm_lock); }
    rdsdebug!("id %p event %u (%s) handling ret %d\n", cm_id, (*event).event,
              rdma_event_msg((*event).event), ret);
    ret
}

pub unsafe extern "C" fn rds_rdma_cm_event_handler(cm_id: *mut rdma_cm_id, event: *mut rdma_cm_event) -> i32 {
    rds_rdma_cm_event_handler_cmn(cm_id, event, false)
}

#[cfg(feature = "ipv6")]
pub unsafe extern "C" fn rds6_rdma_cm_event_handler(cm_id: *mut rdma_cm_id, event: *mut rdma_cm_event) -> i32 {
    rds_rdma_cm_event_handler_cmn(cm_id, event, true)
}

unsafe fn rds_rdma_listen_init_common(
    handler: rdma_cm_event_handler,
    sa: *mut sockaddr,
    ret_cm_id: *mut *mut rdma_cm_id,
) -> i32 {
    let mut cm_id = rdma_create_id(&raw mut init_net, handler, core::ptr::null_mut(), RDMA_PS_TCP, IB_QPT_RC);
    if IS_ERR(cm_id) {
        let ret = PTR_ERR(cm_id);
        printk!(KERN_ERR, "RDS/RDMA: failed to setup listener, rdma_create_id() returned %d\n", ret);
        return ret;
    }
    let mut ret = rdma_bind_addr(cm_id, sa);
    if ret != 0 { printk!(KERN_ERR, "RDS/RDMA: failed to setup listener, rdma_bind_addr() returned %d\n", ret); }
    if ret == 0 { ret = rdma_listen(cm_id, 128); }
    if ret != 0 {
        if !cm_id.is_null() { rdma_destroy_id(cm_id); }
        return ret;
    }
    rdsdebug!("cm %p listening on port %u\n", cm_id, RDS_PORT);
    *ret_cm_id = cm_id;
    cm_id = core::ptr::null_mut();
    ret
}

unsafe fn rds_rdma_listen_init() -> i32 {
    let mut sin: sockaddr_in = core::mem::zeroed();
    sin.sin_family = PF_INET;
    sin.sin_addr.s_addr = htonl(INADDR_ANY);
    sin.sin_port = htons(RDS_PORT);
    let ret = rds_rdma_listen_init_common(rds_rdma_cm_event_handler, &mut sin as *mut _ as *mut sockaddr, &raw mut rds_rdma_listen_id);
    if ret != 0 { return ret; }
    0
}

unsafe fn rds_rdma_listen_stop() {
    if !rds_rdma_listen_id.is_null() {
        rdma_destroy_id(rds_rdma_listen_id);
        rds_rdma_listen_id = core::ptr::null_mut();
    }
    #[cfg(feature = "ipv6")]
    if !rds6_rdma_listen_id.is_null() {
        rdma_destroy_id(rds6_rdma_listen_id);
        rds6_rdma_listen_id = core::ptr::null_mut();
    }
}

unsafe fn rds_rdma_init() -> i32 {
    let ret = rds_ib_init();
    if ret != 0 { return ret; }
    let ret = rds_rdma_listen_init();
    if ret != 0 { rds_ib_exit(); }
    ret
}

unsafe fn rds_rdma_exit() {
    // stop listening first to ensure no new connections are attempted
    rds_rdma_listen_stop();
    rds_ib_exit();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
