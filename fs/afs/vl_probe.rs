// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS vlserver probing
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/AFS translation.

/*
 * Handle the completion of a set of probes.
 */
unsafe fn afs_finished_vl_probe(server: *mut afs_vlserver) {
	if ((*server).probe.flags & AFS_VLSERVER_PROBE_RESPONDED) == 0 {
		(*server).rtt = UINT_MAX;
	clear_bit(AFS_VLSERVER_FL_RESPONDING, &mut (*server).flags);
	}

	clear_bit_unlock(AFS_VLSERVER_FL_PROBING, &mut (*server).flags);
	wake_up_bit(&mut (*server).flags, AFS_VLSERVER_FL_PROBING);
}

/*
 * Handle the completion of a probe RPC call.
 */
unsafe fn afs_done_one_vl_probe(server: *mut afs_vlserver, mut wake_up: bool) {
	if atomic_dec_and_test(&mut (*server).probe_outstanding) {
		afs_finished_vl_probe(server);
		wake_up = true;
	}

	if wake_up {
		wake_up_all(&mut (*server).probe_wq);
	}
}

/*
 * Process the result of probing a vlserver.  This is called after successful
 * or failed delivery of an VL.GetCapabilities operation.
 */
pub unsafe fn afs_vlserver_probe_result(call: *mut afs_call) {
	let alist = (*call).vl_probe;
	let server = (*call).vlserver;
	let addr = &mut (*alist).addrs[(*call).probe_index as usize];
	let server_index: u32 = (*call).server_index;
	let mut rtt_us: u32 = 0;
	let index: u32 = (*call).probe_index;
	let mut have_result = false;
	let ret: i32 = (*call).error;

	_enter!("%s,%u,%u,%d,%d", (*server).name, server_index, index, ret, (*call).abort_code);

	spin_lock(&mut (*server).probe_lock);

	match ret {
		0 => {
			(*server).probe.error = 0;
		}
		-ECONNABORTED => {
			if ((*server).probe.flags & AFS_VLSERVER_PROBE_RESPONDED) == 0 {
				(*server).probe.abort_code = (*call).abort_code;
				(*server).probe.error = ret;
			}
		}
		-ENOMEM | -ENONET | -EKEYEXPIRED | -EKEYREVOKED | -EKEYREJECTED => {
			(*server).probe.flags |= AFS_VLSERVER_PROBE_LOCAL_FAILURE;
			if (*server).probe.error == 0 {
				(*server).probe.error = ret;
			}
			trace_afs_io_error((*call).debug_id, ret, afs_io_error_vl_probe_fail);
			spin_unlock(&mut (*server).probe_lock);
			trace_afs_vl_probe(server, false, alist, index, (*call).error, (*call).abort_code, rtt_us);
			_debug!("probe [%u][%u] %pISpc rtt=%d ret=%d", server_index, index,
				rxrpc_kernel_remote_addr((*addr).peer), rtt_us, ret);
			afs_done_one_vl_probe(server, have_result);
			return;
		}
		_ => {
			clear_bit(index, &mut (*alist).responded);
			set_bit(index, &mut (*alist).probe_failed);
			if ((*server).probe.flags & AFS_VLSERVER_PROBE_RESPONDED) == 0 &&
				((*server).probe.error == 0 || (*server).probe.error == -ETIMEDOUT ||
				 (*server).probe.error == -ETIME) {
				(*server).probe.error = ret;
			}
			trace_afs_io_error((*call).debug_id, ret, afs_io_error_vl_probe_fail);
			spin_unlock(&mut (*server).probe_lock);
			trace_afs_vl_probe(server, false, alist, index, (*call).error, (*call).abort_code, rtt_us);
			_debug!("probe [%u][%u] %pISpc rtt=%d ret=%d", server_index, index,
				rxrpc_kernel_remote_addr((*addr).peer), rtt_us, ret);
			afs_done_one_vl_probe(server, have_result);
			return;
		}
	}

	set_bit(index, &mut (*alist).responded);
	clear_bit(index, &mut (*alist).probe_failed);

	if (*call).service_id == YFS_VL_SERVICE {
		(*server).probe.flags |= AFS_VLSERVER_PROBE_IS_YFS;
		set_bit(AFS_VLSERVER_FL_IS_YFS, &mut (*server).flags);
		(*server).service_id = (*call).service_id;
	} else {
		(*server).probe.flags |= AFS_VLSERVER_PROBE_NOT_YFS;
		if ((*server).probe.flags & AFS_VLSERVER_PROBE_IS_YFS) == 0 {
			clear_bit(AFS_VLSERVER_FL_IS_YFS, &mut (*server).flags);
			(*server).service_id = (*call).service_id;
		}
	}

	rtt_us = rxrpc_kernel_get_srtt((*addr).peer);
	if rtt_us < (*server).probe.rtt {
		(*server).probe.rtt = rtt_us;
		(*server).rtt = rtt_us;
		(*alist).preferred = index;
	}

	smp_wmb(); /* Set rtt before responded. */
	(*server).probe.flags |= AFS_VLSERVER_PROBE_RESPONDED;
	set_bit(AFS_VLSERVER_FL_PROBED, &mut (*server).flags);
	set_bit(AFS_VLSERVER_FL_RESPONDING, &mut (*server).flags);
	have_result = true;

	spin_unlock(&mut (*server).probe_lock);
	trace_afs_vl_probe(server, false, alist, index, (*call).error, (*call).abort_code, rtt_us);
	_debug!("probe [%u][%u] %pISpc rtt=%d ret=%d", server_index, index,
		rxrpc_kernel_remote_addr((*addr).peer), rtt_us, ret);
	afs_done_one_vl_probe(server, have_result);
}

/*
 * Probe all of a vlserver's addresses to find out the best route and to
 * query its capabilities.
 */
unsafe fn afs_do_probe_vlserver(net: *mut afs_net, server: *mut afs_vlserver,
	key: *mut key, server_index: u32, e: *mut afs_error) -> bool {
	let alist;
	let mut in_progress = false;
	_enter!("%s", (*server).name);
	read_lock(&mut (*server).lock);
	alist = rcu_dereference_protected((*server).addresses, lockdep_is_held(&(*server).lock));
	afs_get_addrlist(alist, afs_alist_trace_get_vlprobe);
	read_unlock(&mut (*server).lock);
	atomic_set(&mut (*server).probe_outstanding, (*alist).nr_addrs);
	memset(&mut (*server).probe, 0, core::mem::size_of_val(&(*server).probe));
	(*server).probe.rtt = UINT_MAX;
	let mut unprobed = (1usize << (*alist).nr_addrs) - 1;
	while unprobed != 0 {
		let mut best_prio = -1;
		let mut index = 0;
		for i in 0..(*alist).nr_addrs {
			if test_bit(i, &unprobed) && (*alist).addrs[i as usize].prio > best_prio {
				index = i;
				best_prio = (*alist).addrs[i as usize].prio;
			}
		}
		__clear_bit(index, &mut unprobed);
		trace_afs_vl_probe(server, true, alist, index, 0, 0, 0);
		let call = afs_vl_get_capabilities(net, alist, index, key, server, server_index);
		if !IS_ERR(call) {
			afs_prioritise_error(e, (*call).error, (*call).abort_code);
			afs_put_call(call);
			in_progress = true;
		} else {
			afs_prioritise_error(e, PTR_ERR(call), 0);
			afs_done_one_vl_probe(server, false);
		}
	}
	afs_put_addrlist(alist, afs_alist_trace_put_vlprobe);
	in_progress
}

/* Send off probes to all unprobed servers. */
pub unsafe fn afs_send_vl_probes(net: *mut afs_net, key: *mut key,
	vllist: *mut afs_vlserver_list) -> i32 {
	let mut e: afs_error = core::mem::zeroed();
	let mut in_progress = false;
	for i in 0..(*vllist).nr_servers {
		let server = (*vllist).servers[i as usize].server;
		if test_bit(AFS_VLSERVER_FL_PROBED, &(*server).flags) { continue; }
		if !test_and_set_bit_lock(AFS_VLSERVER_FL_PROBING, &mut (*server).flags) &&
			afs_do_probe_vlserver(net, server, key, i, &mut e) { in_progress = true; }
	}
	if in_progress { 0 } else { e.error }
}

/* Wait for the first as-yet untried server to respond. */
pub unsafe fn afs_wait_for_vl_probes(vllist: *mut afs_vlserver_list, mut untried: usize) -> i32 {
	let mut rtt = UINT_MAX;
	let mut have_responders = false;
	let mut pref: i32 = -1;
	for i in 0..(*vllist).nr_servers {
		if test_bit(i, &untried) {
			let server = (*vllist).servers[i as usize].server;
			if !test_bit(AFS_VLSERVER_FL_PROBING, &(*server).flags) { __clear_bit(i, &mut untried); }
			if ((*server).probe.flags & AFS_VLSERVER_PROBE_RESPONDED) != 0 { have_responders = true; }
		}
	}
	if have_responders || untried == 0 { return 0; }
	let waits = kmalloc(array_size((*vllist).nr_servers, core::mem::size_of::<wait_queue_entry>()), GFP_KERNEL);
	if waits.is_null() { return -ENOMEM; }
	for i in 0..(*vllist).nr_servers {
		if test_bit(i, &untried) {
			let server = (*vllist).servers[i as usize].server;
			init_waitqueue_entry(&mut (*waits.add(i as usize)), current);
			add_wait_queue(&mut (*server).probe_wq, &mut (*waits.add(i as usize)));
		}
	}
	loop {
		let mut still_probing = false;
		set_current_state(TASK_INTERRUPTIBLE);
		for i in 0..(*vllist).nr_servers {
			if test_bit(i, &untried) {
				let server = (*vllist).servers[i as usize].server;
				if ((*server).probe.flags & AFS_VLSERVER_PROBE_RESPONDED) != 0 { break; }
				if test_bit(AFS_VLSERVER_FL_PROBING, &(*server).flags) { still_probing = true; }
			}
		}
		if !still_probing || signal_pending(current) { break; }
		schedule();
	}
	set_current_state(TASK_RUNNING);
	for i in 0..(*vllist).nr_servers {
		if test_bit(i, &untried) {
			let server = (*vllist).servers[i as usize].server;
			let rtt_s = READ_ONCE((*server).rtt);
			if test_bit(AFS_VLSERVER_FL_RESPONDING, &(*server).flags) && rtt_s < rtt { pref = i as i32; rtt = rtt_s; }
			remove_wait_queue(&mut (*server).probe_wq, &mut (*waits.add(i as usize)));
		}
	}
	kfree(waits as *mut core::ffi::c_void);
	if pref == -1 && signal_pending(current) { return -ERESTARTSYS; }
	if pref >= 0 { (*vllist).preferred = pref as u32; }
	_leave!(" = 0 [%u]", pref);
	0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
