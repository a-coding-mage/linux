// SPDX-License-Identifier: GPL-2.0-or-later
/* /proc/net/ support for AF_RXRPC
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel headers and symbols referenced by this translation are supplied by other files.

const RXRPC_PROC_ADDRBUF_SIZE: usize = "[xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:255.255.255.255]".len()
    + ":12345".len();

static RXRPC_CONN_STATES: [&'static str; RXRPC_CONN__NR_STATES] = [
    [RXRPC_CONN_UNUSED] = "Unused  ",
    [RXRPC_CONN_CLIENT_UNSECURED] = "ClUnsec ",
    [RXRPC_CONN_CLIENT] = "Client  ",
    [RXRPC_CONN_SERVICE_PREALLOC] = "SvPrealc",
    [RXRPC_CONN_SERVICE_UNSECURED] = "SvUnsec ",
    [RXRPC_CONN_SERVICE_CHALLENGING] = "SvChall ",
    [RXRPC_CONN_SERVICE] = "SvSecure",
    [RXRPC_CONN_ABORTED] = "Aborted ",
];

/* generate a list of extant and dead calls in /proc/net/rxrpc_calls */
unsafe fn rxrpc_call_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let rxnet = rxrpc_net(seq_file_net(seq));
    rcu_read_lock();
    seq_list_start_head_rcu(&mut (*rxnet).calls, *pos)
}

unsafe fn rxrpc_call_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let rxnet = rxrpc_net(seq_file_net(seq));
    seq_list_next_rcu(v, &mut (*rxnet).calls, pos)
}

unsafe fn rxrpc_call_seq_stop(_seq: *mut seq_file, _v: *mut c_void) { rcu_read_unlock(); }

unsafe fn rxrpc_call_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let rxnet = rxrpc_net(seq_file_net(seq));
    if v == &mut (*rxnet).calls as *mut _ as *mut c_void {
        seq_puts(seq, "Proto Local                                          Remote                                         SvID ConnID   CallID   End Use State    Abort   DebugId  TxSeq    TW RxSeq    RW RxSerial CW RxTimo\n");
        return 0;
    }
    let call = list_entry(v, rxrpc_call, link);
    let local = (*call).local;
    let mut lbuff = [0i8; RXRPC_PROC_ADDRBUF_SIZE];
    let mut rbuff = [0i8; RXRPC_PROC_ADDRBUF_SIZE];
    if !local.is_null() { scnprintf(lbuff.as_mut_ptr(), lbuff.len(), "%pISpc", &(*local).srx.transport); }
    else { strcpy(lbuff.as_mut_ptr(), "no_local"); }
    scnprintf(rbuff.as_mut_ptr(), rbuff.len(), "%pISpc", &(*call).dest_srx.transport);
    let state = rxrpc_call_state(call);
    let mut timeout: c_long = 0;
    if state != RXRPC_CALL_SERVER_PREALLOC { timeout = ktime_ms_delta(READ_ONCE((*call).expect_rx_by), ktime_get_real()); }
    let tx_bottom = READ_ONCE((*call).tx_bottom);
    seq_printf(seq, "UDP   %-47.47s %-47.47s %4x %08x %08x %s %3u %-8.8s %08x %08x %08x %02x %08x %02x %06lx\n", lbuff.as_ptr(), rbuff.as_ptr(), (*call).dest_srx.srx_service, (*call).cid, (*call).call_id, if rxrpc_is_service_call(call) { "Svc" } else { "Clt" }, refcount_read(&(*call).ref_), rxrpc_call_states[state], (*call).abort_code, (*call).debug_id, tx_bottom, READ_ONCE((*call).tx_top) - tx_bottom, (*call).ackr_window, (*call).ackr_wtop - (*call).ackr_window, (*call).rx_serial, (*call).cong_cwnd, timeout);
    0
}

pub static RXRPC_CALL_SEQ_OPS: seq_operations = seq_operations { start: Some(rxrpc_call_seq_start), next: Some(rxrpc_call_seq_next), stop: Some(rxrpc_call_seq_stop), show: Some(rxrpc_call_seq_show) };

/* generate a list of extant virtual connections in /proc/net/rxrpc_conns */
unsafe fn rxrpc_connection_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void { let n = rxrpc_net(seq_file_net(seq)); read_lock(&mut (*n).conn_lock); seq_list_start_head(&mut (*n).conn_proc_list, *pos) }
unsafe fn rxrpc_connection_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void { let n = rxrpc_net(seq_file_net(seq)); seq_list_next(v, &mut (*n).conn_proc_list, pos) }
unsafe fn rxrpc_connection_seq_stop(seq: *mut seq_file, _v: *mut c_void) { let n = rxrpc_net(seq_file_net(seq)); read_unlock(&mut (*n).conn_lock); }
unsafe fn rxrpc_connection_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int { let n = rxrpc_net(seq_file_net(seq)); if v == &mut (*n).conn_proc_list as *mut _ as *mut c_void { seq_puts(seq, "Proto Local                                          Remote                                         SvID ConnID   End Ref Act State    Key     Serial   ISerial  CallId0  CallId1  CallId2  CallId3\n"); return 0; } let c = list_entry(v, rxrpc_connection, proc_link); let mut l=[0i8;RXRPC_PROC_ADDRBUF_SIZE]; let mut r=[0i8;RXRPC_PROC_ADDRBUF_SIZE]; if (*c).state==RXRPC_CONN_SERVICE_PREALLOC { strcpy(l.as_mut_ptr(),"no_local"); strcpy(r.as_mut_ptr(),"no_connection"); } else { scnprintf(l.as_mut_ptr(),l.len(),"%pISpc",&(*c).local.srx.transport); scnprintf(r.as_mut_ptr(),r.len(),"%pISpc",&(*c).peer.srx.transport); } let s=if rxrpc_is_conn_aborted(c){rxrpc_call_completions[(*c).completion]}else{RXRPC_CONN_STATES[(*c).state]}; seq_printf(seq,"UDP   %-47.47s %-47.47s %4x %08x %s %3u %3d %s %08x %08x %08x %08x %08x %08x %08x\n",l.as_ptr(),r.as_ptr(),(*c).service_id,(*c).proto.cid,if rxrpc_conn_is_service(c){"Svc"}else{"Clt"},refcount_read(&(*c).ref_),atomic_read(&(*c).active),s,key_serial((*c).key),(*c).tx_serial,(*c).hi_serial,(*c).channels[0].call_id,(*c).channels[1].call_id,(*c).channels[2].call_id,(*c).channels[3].call_id); 0 }
pub static RXRPC_CONNECTION_SEQ_OPS: seq_operations = seq_operations { start: Some(rxrpc_connection_seq_start), next: Some(rxrpc_connection_seq_next), stop: Some(rxrpc_connection_seq_stop), show: Some(rxrpc_connection_seq_show) };

/* generate a list of extant virtual bundles in /proc/net/rxrpc/bundles */
unsafe fn rxrpc_bundle_seq_start(seq:*mut seq_file,pos:*mut loff_t)->*mut c_void{let n=rxrpc_net(seq_file_net(seq));read_lock(&mut(*n).conn_lock);seq_list_start_head(&mut(*n).bundle_proc_list,*pos)}
unsafe fn rxrpc_bundle_seq_next(seq:*mut seq_file,v:*mut c_void,pos:*mut loff_t)->*mut c_void{let n=rxrpc_net(seq_file_net(seq));seq_list_next(v,&mut(*n).bundle_proc_list,pos)}
unsafe fn rxrpc_bundle_seq_stop(seq:*mut seq_file,_:*mut c_void){let n=rxrpc_net(seq_file_net(seq));read_unlock(&mut(*n).conn_lock)}
unsafe fn rxrpc_bundle_seq_show(seq:*mut seq_file,v:*mut c_void)->c_int{let n=rxrpc_net(seq_file_net(seq));if v==&mut(*n).bundle_proc_list as*mut _ as*mut c_void{seq_puts(seq,"Proto Local                                          Remote                                         SvID Ref Act Flg Key      | Bundle   Conn_0   Conn_1   Conn_2   Conn_3\n");return 0;}let b=list_entry(v,rxrpc_bundle,proc_link);let mut l=[0i8;RXRPC_PROC_ADDRBUF_SIZE];let mut r=[0i8;RXRPC_PROC_ADDRBUF_SIZE];scnprintf(l.as_mut_ptr(),l.len(),"%pISpc",&(*b).local.srx.transport);scnprintf(r.as_mut_ptr(),r.len(),"%pISpc",&(*b).peer.srx.transport);seq_printf(seq,"UDP   %-47.47s %-47.47s %4x %3u %3d %c%c%c %08x | %08x %08x %08x %08x %08x\n",l.as_ptr(),r.as_ptr(),(*b).service_id,refcount_read(&(*b).ref_),atomic_read(&(*b).active),if(*b).try_upgrade{'U'}else{'-'},if(*b).exclusive{'e'}else{'-'},if(*b).upgrade{'u'}else{'-'},key_serial((*b).key),(*b).debug_id,(*b).conn_ids[0],(*b).conn_ids[1],(*b).conn_ids[2],(*b).conn_ids[3]);0}
pub static RXRPC_BUNDLE_SEQ_OPS:seq_operations=seq_operations{start:Some(rxrpc_bundle_seq_start),next:Some(rxrpc_bundle_seq_next),stop:Some(rxrpc_bundle_seq_stop),show:Some(rxrpc_bundle_seq_show)};

/* Generate a list of extant virtual local endpoints in /proc/net/rxrpc/locals */
unsafe fn rxrpc_local_seq_show(seq:*mut seq_file,v:*mut c_void)->c_int{if v==SEQ_START_TOKEN{seq_puts(seq,"Proto Local                                          Use Act RxQ\n");return 0;}let l=hlist_entry(v,rxrpc_local,link);let mut b=[0i8;RXRPC_PROC_ADDRBUF_SIZE];scnprintf(b.as_mut_ptr(),b.len(),"%pISpc",&(*l).srx.transport);seq_printf(seq,"UDP   %-47.47s %3u %3u %3u\n",b.as_ptr(),refcount_read(&(*l).ref_),atomic_read(&(*l).active_users),(*l).rx_queue.qlen);0}
unsafe fn rxrpc_local_seq_start(seq:*mut seq_file,p:*mut loff_t)->*mut c_void{let n=rxrpc_net(seq_file_net(seq));rcu_read_lock();if *p>=UINT_MAX{return core::ptr::null_mut()}if*p==0{return SEQ_START_TOKEN}seq_hlist_start_rcu(&mut(*n).local_endpoints,*p-1)}
unsafe fn rxrpc_local_seq_next(seq:*mut seq_file,v:*mut c_void,p:*mut loff_t)->*mut c_void{let n=rxrpc_net(seq_file_net(seq));if*p>=UINT_MAX{core::ptr::null_mut()}else{seq_hlist_next_rcu(v,&mut(*n).local_endpoints,p)}}
unsafe fn rxrpc_local_seq_stop(_: *mut seq_file,_:*mut c_void){rcu_read_unlock()}
pub static RXRPC_LOCAL_SEQ_OPS:seq_operations=seq_operations{start:Some(rxrpc_local_seq_start),next:Some(rxrpc_local_seq_next),stop:Some(rxrpc_local_seq_stop),show:Some(rxrpc_local_seq_show)};

/* Display stats in /proc/net/rxrpc/stats. */
pub unsafe fn rxrpc_stats_show(seq:*mut seq_file,_:*mut c_void)->c_int{let n=rxrpc_net(seq_file_single_net(seq));seq_printf(seq,"Data     : send=%u sendf=%u fail=%u emsz=%u\n",atomic_read(&(*n).stat_tx_data_send),atomic_read(&(*n).stat_tx_data_send_frag),atomic_read(&(*n).stat_tx_data_send_fail),atomic_read(&(*n).stat_tx_data_send_msgsize));seq_printf(seq,"Data-Tx  : nr=%u retrans=%u uf=%u cwr=%u\n",atomic_read(&(*n).stat_tx_data),atomic_read(&(*n).stat_tx_data_retrans),atomic_read(&(*n).stat_tx_data_underflow),atomic_read(&(*n).stat_tx_data_cwnd_reset));seq_printf(seq,"Data-Rx  : nr=%u reqack=%u jumbo=%u\n",atomic_read(&(*n).stat_rx_data),atomic_read(&(*n).stat_rx_data_reqack),atomic_read(&(*n).stat_rx_data_jumbo));seq_printf(seq,"Ack      : fill=%u send=%u skip=%u\n",atomic_read(&(*n).stat_tx_ack_fill),atomic_read(&(*n).stat_tx_ack_send),atomic_read(&(*n).stat_tx_ack_skip));0}

/* Clear stats if /proc/net/rxrpc/stats is written to. */
pub unsafe fn rxrpc_stats_clear(file:*mut file,buf:*mut c_char,size:usize)->isize{let m=(*file).private_data;let n=rxrpc_net(seq_file_single_net(m));if size>1||(size==1&&*buf as u8!=b'\n'){return -EINVAL as isize;}atomic_set(&mut(*n).stat_tx_data,0);atomic_set(&mut(*n).stat_tx_data_retrans,0);atomic_set(&mut(*n).stat_tx_data_underflow,0);atomic_set(&mut(*n).stat_tx_data_cwnd_reset,0);atomic_set(&mut(*n).stat_tx_data_send,0);atomic_set(&mut(*n).stat_tx_data_send_frag,0);atomic_set(&mut(*n).stat_tx_data_send_fail,0);atomic_set(&mut(*n).stat_rx_data,0);atomic_set(&mut(*n).stat_rx_data_reqack,0);atomic_set(&mut(*n).stat_rx_data_jumbo,0);atomic_set(&mut(*n).stat_tx_ack_fill,0);atomic_set(&mut(*n).stat_tx_ack_send,0);atomic_set(&mut(*n).stat_tx_ack_skip,0);memset(&mut(*n).stat_tx_acks as*mut _,0,size_of_val(&(*n).stat_tx_acks));memset(&mut(*n).stat_rx_acks as*mut _,0,size_of_val(&(*n).stat_rx_acks));memset(&mut(*n).stat_tx_jumbo as*mut _,0,size_of_val(&(*n).stat_tx_jumbo));memset(&mut(*n).stat_rx_jumbo as*mut _,0,size_of_val(&(*n).stat_rx_jumbo));memset(&mut(*n).stat_why_req_ack as*mut _,0,size_of_val(&(*n).stat_why_req_ack));atomic_set(&mut(*n).stat_io_loop,0);size as isize}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
