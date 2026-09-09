// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation. Direct Rust translation of sm_statetable.c. */

// C headers and build-time symbols are supplied by the surrounding SCTP code.

macro_rules! ent { ($f:ident) => { sctp_sm_table_entry { r#fn: $f, name: stringify!($f) } }; }
macro_rules! row { ($($f:ident),* $(,)?) => { [$(ent!($f)),*] }; }

static BUG: sctp_sm_table_entry = sctp_sm_table_entry { r#fn: sctp_sf_bug, name: "sctp_sf_bug" };

static CHUNK_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_BASE_CHUNK_TYPES] = [
    row!(sctp_sf_ootb,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_eat_data_6_2,sctp_sf_eat_data_6_2,sctp_sf_eat_data_fast_4_4,sctp_sf_discard_chunk,sctp_sf_discard_chunk),
    row!(sctp_sf_do_5_1B_init,sctp_sf_do_5_2_1_siminit,sctp_sf_do_5_2_1_siminit,sctp_sf_do_5_2_2_dupinit,sctp_sf_do_5_2_2_dupinit,sctp_sf_do_5_2_2_dupinit,sctp_sf_do_5_2_2_dupinit,sctp_sf_do_9_2_reshutack),
    row!(sctp_sf_do_5_2_3_initack,sctp_sf_do_5_1C_ack,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk),
    row!(sctp_sf_ootb,sctp_sf_discard_chunk,sctp_sf_eat_sack_6_2,sctp_sf_eat_sack_6_2,sctp_sf_eat_sack_6_2,sctp_sf_discard_chunk,sctp_sf_eat_sack_6_2,sctp_sf_discard_chunk),
    row!(sctp_sf_ootb,sctp_sf_discard_chunk,sctp_sf_beat_8_3,sctp_sf_beat_8_3,sctp_sf_beat_8_3,sctp_sf_beat_8_3,sctp_sf_beat_8_3,sctp_sf_beat_8_3),
    row!(sctp_sf_ootb,sctp_sf_violation,sctp_sf_discard_chunk,sctp_sf_backbeat_8_3,sctp_sf_backbeat_8_3,sctp_sf_backbeat_8_3,sctp_sf_backbeat_8_3,sctp_sf_discard_chunk),
    row!(sctp_sf_pdiscard,sctp_sf_cookie_wait_abort,sctp_sf_cookie_echoed_abort,sctp_sf_do_9_1_abort,sctp_sf_shutdown_pending_abort,sctp_sf_shutdown_sent_abort,sctp_sf_do_9_1_abort,sctp_sf_shutdown_ack_sent_abort),
    row!(sctp_sf_ootb,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_9_2_shutdown,sctp_sf_do_9_2_shutdown,sctp_sf_do_9_2_shutdown_ack,sctp_sf_do_9_2_shut_ctsn,sctp_sf_discard_chunk),
    row!(sctp_sf_ootb,sctp_sf_do_8_5_1_E_sa,sctp_sf_do_8_5_1_E_sa,sctp_sf_violation,sctp_sf_violation,sctp_sf_do_9_2_final,sctp_sf_violation,sctp_sf_do_9_2_final),
    row!(sctp_sf_ootb,sctp_sf_discard_chunk,sctp_sf_cookie_echoed_err,sctp_sf_operr_notify,sctp_sf_operr_notify,sctp_sf_discard_chunk,sctp_sf_operr_notify,sctp_sf_discard_chunk),
    row!(sctp_sf_do_5_1D_ce,sctp_sf_do_5_2_4_dupcook,sctp_sf_do_5_2_4_dupcook,sctp_sf_do_5_2_4_dupcook,sctp_sf_do_5_2_4_dupcook,sctp_sf_do_5_2_4_dupcook,sctp_sf_do_5_2_4_dupcook,sctp_sf_do_5_2_4_dupcook),
    row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_5_1E_ca,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk),
    row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_ecne,sctp_sf_do_ecne,sctp_sf_do_ecne,sctp_sf_do_ecne,sctp_sf_do_ecne,sctp_sf_discard_chunk),
    row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_ecn_cwr,sctp_sf_do_ecn_cwr,sctp_sf_do_ecn_cwr,sctp_sf_discard_chunk,sctp_sf_discard_chunk),
    row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_4_C),
];

static ADDIP_CHUNK_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_ADDIP_CHUNK_TYPES] = [
    row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_asconf,sctp_sf_do_asconf,sctp_sf_do_asconf,sctp_sf_do_asconf,sctp_sf_discard_chunk),
    row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_asconf_ack,sctp_sf_do_asconf_ack,sctp_sf_do_asconf_ack,sctp_sf_do_asconf_ack,sctp_sf_discard_chunk),
];
static PRSCTP_CHUNK_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_PRSCTP_CHUNK_TYPES] = [row!(sctp_sf_ootb,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_eat_fwd_tsn,sctp_sf_eat_fwd_tsn,sctp_sf_eat_fwd_tsn_fast,sctp_sf_discard_chunk,sctp_sf_discard_chunk)];
static RECONF_CHUNK_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_RECONF_CHUNK_TYPES] = [row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_do_reconf,sctp_sf_do_reconf,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk)];
static AUTH_CHUNK_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_AUTH_CHUNK_TYPES] = [row!(sctp_sf_ootb,sctp_sf_discard_chunk,sctp_sf_eat_auth,sctp_sf_eat_auth,sctp_sf_eat_auth,sctp_sf_eat_auth,sctp_sf_eat_auth,sctp_sf_eat_auth)];
static PAD_CHUNK_EVENT_TABLE: [sctp_sm_table_entry; SCTP_STATE_NUM_STATES] = row!(sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk,sctp_sf_discard_chunk);
static CHUNK_EVENT_TABLE_UNKNOWN: [sctp_sm_table_entry; SCTP_STATE_NUM_STATES] = row!(sctp_sf_ootb,sctp_sf_unk_chunk,sctp_sf_unk_chunk,sctp_sf_unk_chunk,sctp_sf_unk_chunk,sctp_sf_unk_chunk,sctp_sf_unk_chunk,sctp_sf_unk_chunk);

static PRIMITIVE_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_PRIMITIVE_TYPES] = [
 row!(sctp_sf_do_prm_asoc,sctp_sf_not_impl,sctp_sf_not_impl,sctp_sf_not_impl,sctp_sf_not_impl,sctp_sf_not_impl,sctp_sf_not_impl,sctp_sf_not_impl),
 row!(sctp_sf_error_closed,sctp_sf_cookie_wait_prm_shutdown,sctp_sf_cookie_echoed_prm_shutdown,sctp_sf_do_9_2_prm_shutdown,sctp_sf_ignore_primitive,sctp_sf_ignore_primitive,sctp_sf_ignore_primitive,sctp_sf_ignore_primitive),
 row!(sctp_sf_error_closed,sctp_sf_cookie_wait_prm_abort,sctp_sf_cookie_echoed_prm_abort,sctp_sf_do_9_1_prm_abort,sctp_sf_shutdown_pending_prm_abort,sctp_sf_shutdown_sent_prm_abort,sctp_sf_do_9_1_prm_abort,sctp_sf_shutdown_ack_sent_prm_abort),
 row!(sctp_sf_error_closed,sctp_sf_do_prm_send,sctp_sf_do_prm_send,sctp_sf_do_prm_send,sctp_sf_error_shutdown,sctp_sf_error_shutdown,sctp_sf_error_shutdown,sctp_sf_error_shutdown),
 row!(sctp_sf_error_closed,sctp_sf_do_prm_requestheartbeat,sctp_sf_do_prm_requestheartbeat,sctp_sf_do_prm_requestheartbeat,sctp_sf_do_prm_requestheartbeat,sctp_sf_do_prm_requestheartbeat,sctp_sf_do_prm_requestheartbeat,sctp_sf_do_prm_requestheartbeat),
 row!(sctp_sf_error_closed,sctp_sf_error_closed,sctp_sf_error_closed,sctp_sf_do_prm_asconf,sctp_sf_do_prm_asconf,sctp_sf_do_prm_asconf,sctp_sf_do_prm_asconf,sctp_sf_error_shutdown),
 row!(sctp_sf_error_closed,sctp_sf_error_closed,sctp_sf_error_closed,sctp_sf_do_prm_reconf,sctp_sf_do_prm_reconf,sctp_sf_do_prm_reconf,sctp_sf_do_prm_reconf,sctp_sf_error_shutdown),
];

static OTHER_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_OTHER_TYPES] = [
 row!(sctp_sf_ignore_other,sctp_sf_ignore_other,sctp_sf_ignore_other,sctp_sf_do_no_pending_tsn,sctp_sf_do_9_2_start_shutdown,sctp_sf_ignore_other,sctp_sf_do_9_2_shutdown_ack,sctp_sf_ignore_other),
 row!(sctp_sf_ignore_other,sctp_sf_cookie_wait_icmp_abort,sctp_sf_ignore_other,sctp_sf_ignore_other,sctp_sf_ignore_other,sctp_sf_ignore_other,sctp_sf_ignore_other,sctp_sf_ignore_other),
];

static TIMEOUT_EVENT_TABLE: [[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]; SCTP_NUM_TIMEOUT_TYPES] = [
 row!(sctp_sf_bug,sctp_sf_bug,sctp_sf_bug,sctp_sf_bug,sctp_sf_bug,sctp_sf_bug,sctp_sf_bug,sctp_sf_bug),
 row!(sctp_sf_timer_ignore,sctp_sf_bug,sctp_sf_t1_cookie_timer_expire,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_t1_init_timer_expire,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_t2_timer_expire,sctp_sf_timer_ignore,sctp_sf_t2_timer_expire),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_do_6_3_3_rtx,sctp_sf_do_6_3_3_rtx,sctp_sf_do_6_3_3_rtx,sctp_sf_timer_ignore,sctp_sf_do_6_3_3_rtx,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_t4_timer_expire,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_t5_timer_expire,sctp_sf_t5_timer_expire,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_sendbeat_8_3,sctp_sf_sendbeat_8_3,sctp_sf_timer_ignore,sctp_sf_sendbeat_8_3,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_send_reconf,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_send_probe,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_do_6_2_sack,sctp_sf_do_6_2_sack,sctp_sf_do_6_2_sack,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
 row!(sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_autoclose_timer_expire,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore,sctp_sf_timer_ignore),
];

pub unsafe fn sctp_sm_lookup_event(net: *mut net, event_type: sctp_event_type, state: sctp_state, event_subtype: sctp_subtype) -> *const sctp_sm_table_entry {
    match event_type {
        SCTP_EVENT_T_CHUNK => sctp_chunk_event_lookup(net, event_subtype.chunk, state),
        SCTP_EVENT_T_TIMEOUT => lookup(&TIMEOUT_EVENT_TABLE, event_subtype.timeout, state, SCTP_EVENT_TIMEOUT_MAX),
        SCTP_EVENT_T_OTHER => lookup(&OTHER_EVENT_TABLE, event_subtype.other, state, SCTP_EVENT_OTHER_MAX),
        SCTP_EVENT_T_PRIMITIVE => lookup(&PRIMITIVE_EVENT_TABLE, event_subtype.primitive, state, SCTP_EVENT_PRIMITIVE_MAX),
        _ => &BUG,
    }
}

unsafe fn lookup<T>(table: &T, index: usize, state: sctp_state, max: usize) -> *const sctp_sm_table_entry
where T: AsRef<[[sctp_sm_table_entry; SCTP_STATE_NUM_STATES]]> {
    if index > max { &BUG } else { &table.as_ref()[index][state as usize] }
}

unsafe fn sctp_chunk_event_lookup(_net: *mut net, mut cid: sctp_cid, state: sctp_state) -> *const sctp_sm_table_entry {
    if state > SCTP_STATE_MAX { return &BUG; }
    if cid == SCTP_CID_I_DATA { cid = SCTP_CID_DATA; }
    let s = state as usize;
    if cid <= SCTP_CID_BASE_MAX { return &CHUNK_EVENT_TABLE[cid as usize][s]; }
    match cid as u16 {
        SCTP_CID_FWD_TSN | SCTP_CID_I_FWD_TSN => &PRSCTP_CHUNK_EVENT_TABLE[0][s],
        SCTP_CID_ASCONF => &ADDIP_CHUNK_EVENT_TABLE[0][s],
        SCTP_CID_ASCONF_ACK => &ADDIP_CHUNK_EVENT_TABLE[1][s],
        SCTP_CID_RECONF => &RECONF_CHUNK_EVENT_TABLE[0][s],
        SCTP_CID_AUTH => &AUTH_CHUNK_EVENT_TABLE[0][s],
        SCTP_CID_PAD => &PAD_CHUNK_EVENT_TABLE[s],
        _ => &CHUNK_EVENT_TABLE_UNKNOWN[s],
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
