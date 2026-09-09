/* SPDX-License-Identifier: GPL-2.0 */
/* Shared Memory Communications over RDMA (SMC-R) and RoCE; Connection Data Control (CDC). */

pub const SMC_CDC_MSG_TYPE: u8 = 0xFE;

// External types and byte-order/locking primitives are supplied by the translated dependencies.
extern "C" {
    pub fn htons(x: u16) -> u16;
    pub fn ntohs(x: u16) -> u16;
    pub fn htonl(x: u32) -> u32;
    pub fn ntohl(x: u32) -> u32;
}

#[repr(C, align(8))]
pub union smc_cdc_cursor {
    pub fields: smc_cdc_cursor_fields,
    pub acurs: u64, // for atomic processing (atomic64_t when KERNEL_HAS_ATOMIC64)
}
#[repr(C)]
pub struct smc_cdc_cursor_fields { pub reserved: u16, pub wrap: u16, pub count: u32 }

#[repr(C)]
pub struct smc_cdc_msg {
    pub common: smc_wr_rx_hdr,
    pub len: u8,
    pub seqno: u16,
    pub token: u32,
    pub prod: smc_cdc_cursor,
    pub cons: smc_cdc_cursor,
    pub prod_flags: smc_cdc_producer_flags,
    pub conn_state_flags: smc_cdc_conn_state_flags,
    pub reserved: [u8; 18],
}

#[repr(C, align(8))]
pub union smcd_cdc_cursor {
    pub fields: smcd_cdc_cursor_fields,
    pub acurs: u64, // for atomic processing (atomic64_t when KERNEL_HAS_ATOMIC64)
}
#[repr(C, packed)]
pub struct smcd_cdc_cursor_fields {
    pub wrap: u16, pub count: u32,
    pub prod_flags: smc_cdc_producer_flags,
    pub conn_state_flags: smc_cdc_conn_state_flags,
}

#[repr(C, align(8))]
pub struct smcd_cdc_msg {
    pub common: smc_wr_rx_hdr, pub res1: [u8; 7],
    pub prod: smcd_cdc_cursor, pub cons: smcd_cdc_cursor, pub res3: [u8; 8],
}

pub unsafe fn smc_cdc_rxed_any_close(conn: *mut smc_connection) -> bool {
    (*conn).local_rx_ctrl.conn_state_flags.peer_conn_abort ||
    (*conn).local_rx_ctrl.conn_state_flags.peer_conn_closed
}
pub unsafe fn smc_cdc_rxed_any_close_or_senddone(conn: *mut smc_connection) -> bool {
    smc_cdc_rxed_any_close(conn) || (*conn).local_rx_ctrl.conn_state_flags.peer_done_writing
}
pub unsafe fn smc_curs_add(size: i32, curs: *mut smc_host_cursor, value: i32) {
    (*curs).count += value;
    if (*curs).count >= size { (*curs).wrap += 1; (*curs).count -= size; }
}

pub unsafe fn smc_curs_copy(tgt: *mut smc_host_cursor, src: *mut smc_host_cursor, _conn: *mut smc_connection) {
    (*tgt).acurs = (*src).acurs;
}
pub unsafe fn smc_curs_copy_net(tgt: *mut smc_cdc_cursor, src: *mut smc_cdc_cursor, _conn: *mut smc_connection) {
    (*tgt).acurs = (*src).acurs;
}
pub unsafe fn smcd_curs_copy(tgt: *mut smcd_cdc_cursor, src: *mut smcd_cdc_cursor, _conn: *mut smc_connection) {
    (*tgt).acurs = (*src).acurs;
}

pub unsafe fn smc_curs_diff(size: u32, old: *mut smc_host_cursor, new: *mut smc_host_cursor) -> i32 {
    if (*old).wrap != (*new).wrap { std::cmp::max(0, (size - (*old).count + (*new).count) as i32) }
    else { std::cmp::max(0, (*new).count - (*old).count) }
}
pub unsafe fn smc_curs_comp(size: u32, old: *mut smc_host_cursor, new: *mut smc_host_cursor) -> i32 {
    if (*old).wrap > (*new).wrap || ((*old).wrap == (*new).wrap && (*old).count > (*new).count) {
        -smc_curs_diff(size, new, old)
    } else { smc_curs_diff(size, old, new) }
}
pub unsafe fn smc_curs_diff_large(size: u32, old: *mut smc_host_cursor, new: *mut smc_host_cursor) -> i32 {
    let d = if (*old).wrap < (*new).wrap {
        (size - (*old).count + (*new).count + ((*new).wrap - (*old).wrap - 1) * size) as i32
    } else if (*old).wrap > (*new).wrap {
        (size - (*old).count + (*new).count + ((*new).wrap + 0xffff - (*old).wrap) * size) as i32
    } else { ((*new).count as i32) - (*old).count as i32 };
    std::cmp::min(std::cmp::max(d, 0), size as i32)
}

pub unsafe fn smc_host_cursor_to_cdc(peer: *mut smc_cdc_cursor, local: *mut smc_host_cursor, save: *mut smc_host_cursor, conn: *mut smc_connection) {
    smc_curs_copy(save, local, conn);
    (*peer).fields.count = htonl((*save).count as u32);
    (*peer).fields.wrap = htons((*save).wrap as u16);
}

pub unsafe fn smc_cdc_cursor_to_host(local: *mut smc_host_cursor, peer: *mut smc_cdc_cursor, conn: *mut smc_connection) {
    let old = *local;
    let net = (*peer).fields;
    let temp = smc_host_cursor { wrap: ntohs(net.wrap) as i32, count: ntohl(net.count) as i32, acurs: 0 };
    if (old.wrap > temp.wrap) && temp.wrap != 0 { return; }
    if old.wrap == temp.wrap && old.count > temp.count { return; }
    smc_curs_copy(local, &mut { temp }, conn);
}
pub unsafe fn smcr_cdc_msg_to_host(local: *mut smc_host_cdc_msg, peer: *mut smc_cdc_msg, conn: *mut smc_connection) {
    (*local).common = (*peer).common; (*local).len = (*peer).len;
    (*local).seqno = ntohs((*peer).seqno); (*local).token = ntohl((*peer).token);
    smc_cdc_cursor_to_host(&mut (*local).prod, &mut (*peer).prod, conn);
    smc_cdc_cursor_to_host(&mut (*local).cons, &mut (*peer).cons, conn);
    (*local).prod_flags = (*peer).prod_flags; (*local).conn_state_flags = (*peer).conn_state_flags;
}
pub unsafe fn smcd_cdc_msg_to_host(local: *mut smc_host_cdc_msg, peer: *mut smcd_cdc_msg, conn: *mut smc_connection) {
    let mut temp = smc_host_cursor { wrap: (*peer).prod.fields.wrap as i32, count: (*peer).prod.fields.count as i32, acurs: 0 };
    smc_curs_copy(&mut (*local).prod, &mut temp, conn);
    temp.wrap = (*peer).cons.fields.wrap as i32; temp.count = (*peer).cons.fields.count as i32;
    smc_curs_copy(&mut (*local).cons, &mut temp, conn);
    (*local).prod_flags = (*peer).cons.fields.prod_flags; (*local).conn_state_flags = (*peer).cons.fields.conn_state_flags;
}
pub unsafe fn smc_cdc_msg_to_host(local: *mut smc_host_cdc_msg, peer: *mut smc_cdc_msg, conn: *mut smc_connection) {
    if (*conn).lgr_is_smcd() { smcd_cdc_msg_to_host(local, peer as *mut smcd_cdc_msg, conn); }
    else { smcr_cdc_msg_to_host(local, peer, conn); }
}

pub struct smc_cdc_tx_pend { pub conn: *mut smc_connection, pub cursor: smc_host_cursor, pub p_cursor: smc_host_cursor, pub ctrl_seq: u16 }

// Declaration-only external functions.
extern "C" {
    pub fn smc_cdc_get_free_slot(conn: *mut smc_connection, link: *mut smc_link, wr_buf: *mut *mut smc_wr_buf, wr_rdma_buf: *mut *mut smc_rdma_wr, pend: *mut *mut smc_cdc_tx_pend) -> i32;
    pub fn smc_cdc_wait_pend_tx_wr(conn: *mut smc_connection);
    pub fn smc_cdc_msg_send(conn: *mut smc_connection, wr_buf: *mut smc_wr_buf, pend: *mut smc_cdc_tx_pend) -> i32;
    pub fn smc_cdc_get_slot_and_msg_send(conn: *mut smc_connection) -> i32;
    pub fn smcd_cdc_msg_send(conn: *mut smc_connection) -> i32;
    pub fn smcr_cdc_msg_send_validation(conn: *mut smc_connection, pend: *mut smc_cdc_tx_pend, wr_buf: *mut smc_wr_buf) -> i32;
    pub fn smc_cdc_init() -> i32;
    pub fn smcd_cdc_rx_init(conn: *mut smc_connection);
}

// Types supplied by smc.h, smc_core.h, and smc_wr.h.
pub enum smc_connection {}
pub enum smc_link {}
pub enum smc_wr_buf {}
pub enum smc_rdma_wr {}
pub enum smc_wr_rx_hdr {}
pub enum smc_cdc_producer_flags {}
pub enum smc_cdc_conn_state_flags {}
pub struct smc_host_cdc_msg { pub common: smc_wr_rx_hdr, pub len: u8, pub seqno: u16, pub token: u32, pub prod: smc_host_cursor, pub cons: smc_host_cursor, pub prod_flags: smc_cdc_producer_flags, pub conn_state_flags: smc_cdc_conn_state_flags }
pub struct smc_host_cursor { pub wrap: i32, pub count: i32, pub acurs: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
