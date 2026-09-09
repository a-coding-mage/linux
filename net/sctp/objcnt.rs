// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright IBM Corp. 2001, 2004
 *
 * This file is part of the SCTP kernel implementation
 *
 * Support for memory object debugging.  This allows one to monitor the
 * object allocations/deallocations for types instrumented for this
 * via the proc fs.
 *
 * Please send any bug reports or fixes you make to the
 * email address(es):
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *    Jon Grimm             <jgrimm@us.ibm.com>
 */

// `pr_fmt` and the declarations supplied by the Linux SCTP headers are
// external dependencies of this translation unit.

/*
 * Global counters to count raw object allocation counts.
 * To add new counters, choose a unique suffix for the variable
 * name as the helper macros key off this suffix to make
 * life easier for the programmer.
 */

SCTP_DBG_OBJCNT!(sock);
SCTP_DBG_OBJCNT!(ep);
SCTP_DBG_OBJCNT!(transport);
SCTP_DBG_OBJCNT!(assoc);
SCTP_DBG_OBJCNT!(bind_addr);
SCTP_DBG_OBJCNT!(bind_bucket);
SCTP_DBG_OBJCNT!(chunk);
SCTP_DBG_OBJCNT!(addr);
SCTP_DBG_OBJCNT!(datamsg);
SCTP_DBG_OBJCNT!(keys);

/* An array to make it easy to pretty print the debug information
 * to the proc fs.
 */
static mut SCTP_DBG_OBJCNT: [sctp_dbg_objcnt_entry; 10] = [
	SCTP_DBG_OBJCNT_ENTRY!(sock),
	SCTP_DBG_OBJCNT_ENTRY!(ep),
	SCTP_DBG_OBJCNT_ENTRY!(assoc),
	SCTP_DBG_OBJCNT_ENTRY!(transport),
	SCTP_DBG_OBJCNT_ENTRY!(chunk),
	SCTP_DBG_OBJCNT_ENTRY!(bind_addr),
	SCTP_DBG_OBJCNT_ENTRY!(bind_bucket),
	SCTP_DBG_OBJCNT_ENTRY!(addr),
	SCTP_DBG_OBJCNT_ENTRY!(datamsg),
	SCTP_DBG_OBJCNT_ENTRY!(keys),
];

/* Callback from procfs to read out objcount information.
 * Walk through the entries in the sctp_dbg_objcnt array, dumping
 * the raw object counts for each monitored type.
 */
unsafe fn sctp_objcnt_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
	let i: usize = *(v as *mut loff_t) as usize;
	seq_setwidth(seq, 127);
	seq_printf(
		seq,
		c"%s: %d",
		SCTP_DBG_OBJCNT[i].label,
		atomic_read(SCTP_DBG_OBJCNT[i].counter),
	);
	seq_pad(seq, b'\n' as i32);
	0
}

unsafe fn sctp_objcnt_seq_start(
	_seq: *mut seq_file,
	pos: *mut loff_t,
) -> *mut core::ffi::c_void {
	if *pos >= SCTP_DBG_OBJCNT.len() as loff_t {
		core::ptr::null_mut()
	} else {
		pos as *mut core::ffi::c_void
	}
}

unsafe fn sctp_objcnt_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) {}

unsafe fn sctp_objcnt_seq_next(
	_seq: *mut seq_file,
	v: *mut core::ffi::c_void,
	pos: *mut loff_t,
) -> *mut core::ffi::c_void {
	let _ = v;
	*pos += 1;
	if *pos >= SCTP_DBG_OBJCNT.len() as loff_t {
		core::ptr::null_mut()
	} else {
		pos as *mut core::ffi::c_void
	}
}

static sctp_objcnt_seq_ops: seq_operations = seq_operations {
	start: Some(sctp_objcnt_seq_start),
	next: Some(sctp_objcnt_seq_next),
	stop: Some(sctp_objcnt_seq_stop),
	show: Some(sctp_objcnt_seq_show),
};

/* Initialize the objcount in the proc filesystem.  */
unsafe fn sctp_dbg_objcnt_init(net: *mut net) {
	let ent = proc_create_seq(
		c"sctp_dbg_objcnt",
		0,
		(*net).sctp.proc_net_sctp,
		&sctp_objcnt_seq_ops,
	);
	if ent.is_null() {
		pr_warn!(c"sctp_dbg_objcnt: Unable to create /proc entry.\n");
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
