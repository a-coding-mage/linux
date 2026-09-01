// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(C) 2015-2018 Linaro Limited.
 *
 * Author: Tor Jeremiassen <tor@ti.com>
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

/*
 * C dependencies translated as external declarations. Their concrete
 * definitions are supplied by the surrounding translated repository.
 */
use crate::*;

/* use raw logging */
/* C conditional intent:
 * #ifdef CS_DEBUG_RAW
 *   #define CS_LOG_RAW_FRAMES
 *   #define CS_PKT_MON 1
 *   #ifdef CS_RAW_PACKED
 *     #define CS_RAW_DEBUG_FLAGS (OCSD_DFRMTR_UNPACKED_RAW_OUT | OCSD_DFRMTR_PACKED_RAW_OUT)
 *   #else
 *     #define CS_RAW_DEBUG_FLAGS (OCSD_DFRMTR_UNPACKED_RAW_OUT)
 *   #endif
 * #else
 *   #define CS_PKT_MON 0
 * #endif
 */
const CS_PKT_MON: c_int = 0;

/*
 * Assume a maximum of 0.1ns elapsed per instruction. This would be the
 * case with a theoretical 10GHz core executing 1 instruction per cycle.
 * Used to estimate the sample time for synthesized instructions because
 * Coresight only emits a timestamp for a range of instructions rather
 * than per instruction.
 */
pub const INSTR_PER_NS: u32 = 10;

#[repr(C)]
pub struct cs_etm_decoder {
	data: *mut c_void,
	packet_printer: Option<unsafe extern "C" fn(msg: *const c_char, data: *mut c_void)>,
	suppress_printing: bool,
	dcd_tree: dcd_tree_handle_t,
	mem_access: cs_etm_mem_cb_type,
	prev_return: ocsd_datapath_resp_t,
	decoder_name: *const c_char,
}

extern "C" {
	fn ocsd_dt_add_callback_trcid_mem_acc(
		dcd_tree: dcd_tree_handle_t,
		start: u64,
		end: u64,
		mem_space: ocsd_mem_space_acc_t,
		cb: Option<
			unsafe extern "C" fn(
				context: *const c_void,
				address: ocsd_vaddr_t,
				mem_space: ocsd_mem_space_acc_t,
				trace_chan_id: u8,
				req_size: u32,
				buffer: *mut u8,
			) -> u32,
		>,
		context: *mut c_void,
	) -> c_int;
	fn ocsd_dt_process_data(
		dcd_tree: dcd_tree_handle_t,
		op: ocsd_datapath_op_t,
		index: u64,
		data_len: usize,
		p_data: *const u8,
		num_bytes_processed: *mut u32,
	) -> ocsd_datapath_resp_t;
	fn ocsd_def_errlog_init(sev: ocsd_err_severity_t, create_output_logger: c_int) -> c_int;
	fn ocsd_def_errlog_config_output(flags: c_int, logfile: *const c_char) -> c_int;
	fn ocsd_def_errlog_set_strprint_cb(
		dcd_tree: dcd_tree_handle_t,
		context: *mut c_void,
		cb: Option<unsafe extern "C" fn(p_context: *const c_void, msg: *const c_char, str_len: c_int)>,
	) -> c_int;
	fn ocsd_dt_create_decoder(
		dcd_tree: dcd_tree_handle_t,
		decoder_name: *const c_char,
		create_flags: c_int,
		trace_config: *mut c_void,
		csid: *mut u8,
	) -> c_int;
	fn ocsd_dt_set_gen_elem_outfn(
		dcd_tree: dcd_tree_handle_t,
		cb: Option<
			unsafe extern "C" fn(
				context: *const c_void,
				indx: ocsd_trc_index_t,
				trace_chan_id: u8,
				elem: *const ocsd_generic_trace_elem,
			) -> ocsd_datapath_resp_t,
		>,
		context: *mut c_void,
	) -> c_int;
	fn ocsd_dt_set_pkt_protocol_printer(
		dcd_tree: dcd_tree_handle_t,
		csid: u8,
		pkt_monitor: c_int,
	) -> c_int;
	fn ocsd_create_dcd_tree(format: ocsd_dcd_tree_src_t, flags: u32) -> dcd_tree_handle_t;
	fn ocsd_destroy_dcd_tree(dcd_tree: dcd_tree_handle_t);
	fn zalloc(size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);

	fn cs_etm__etmq_set_traceid_queue_timestamp(etmq: *mut cs_etm_queue, trace_chan_id: u8);
	fn cs_etm__etmq_get_packet_queue(
		etmq: *mut cs_etm_queue,
		trace_chan_id: u8,
	) -> *mut cs_etm_packet_queue;
	fn cs_etm__convert_sample_time(etmq: *mut cs_etm_queue, timestamp: u64) -> u64;
	fn cs_etm__etmq_is_timeless(etmq: *mut cs_etm_queue) -> bool;
	fn cs_etm__get_cpu(etmq: *mut cs_etm_queue, trace_chan_id: u8, cpu: *mut c_int) -> c_int;
	fn cs_etm__get_pid_fmt(etmq: *mut cs_etm_queue) -> cs_etm_pid_fmt;
	fn cs_etm__etmq_update_decode_context(
		etmq: *mut cs_etm_queue,
		trace_chan_id: u8,
		exception_level: ocsd_ex_level,
		tid: pid_t,
	) -> c_int;
	fn pr_warning_once(fmt: *const c_char, ...);
	fn pr_err(fmt: *const c_char, ...);
}

unsafe extern "C" fn cs_etm_decoder__mem_access(
	context: *const c_void,
	address: ocsd_vaddr_t,
	mem_space: ocsd_mem_space_acc_t,
	trace_chan_id: u8,
	req_size: u32,
	buffer: *mut u8,
) -> u32 {
	let decoder = context as *mut cs_etm_decoder;

	((*decoder).mem_access)(
		(*decoder).data,
		trace_chan_id,
		address,
		req_size,
		buffer,
		mem_space,
	)
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm_decoder__add_mem_access_cb(
	decoder: *mut cs_etm_decoder,
	start: u64,
	end: u64,
	cb_func: cs_etm_mem_cb_type,
) -> c_int {
	(*decoder).mem_access = cb_func;

	if ocsd_dt_add_callback_trcid_mem_acc(
		(*decoder).dcd_tree,
		start,
		end,
		OCSD_MEM_SPACE_ANY,
		Some(cs_etm_decoder__mem_access),
		decoder as *mut c_void,
	) != 0
	{
		return -1;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm_decoder__reset(decoder: *mut cs_etm_decoder) -> c_int {
	let dp_ret: ocsd_datapath_resp_t;

	(*decoder).prev_return = OCSD_RESP_CONT;
	(*decoder).suppress_printing = true;
	dp_ret = ocsd_dt_process_data((*decoder).dcd_tree, OCSD_OP_RESET, 0, 0, ptr::null(), ptr::null_mut());
	(*decoder).suppress_printing = false;
	if OCSD_DATA_RESP_IS_FATAL(dp_ret) {
		return -1;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm_decoder__get_packet(
	packet_queue: *mut cs_etm_packet_queue,
	packet: *mut cs_etm_packet,
) -> c_int {
	if packet_queue.is_null() || packet.is_null() {
		return -EINVAL;
	}

	/* Nothing to do, might as well just return */
	if (*packet_queue).packet_count == 0 {
		return 0;
	}
	/*
	 * The queueing process in function cs_etm_decoder__buffer_packet()
	 * increments the tail *before* using it.  This is somewhat counter
	 * intuitive but it has the advantage of centralizing tail management
	 * at a single location.  Because of that we need to follow the same
	 * heuristic with the head, i.e we increment it before using its
	 * value.  Otherwise the first element of the packet queue is not
	 * used.
	 */
	(*packet_queue).head = ((*packet_queue).head + 1) & (CS_ETM_PACKET_MAX_BUFFER - 1);

	*packet = (*packet_queue).packet_buffer[(*packet_queue).head as usize];

	(*packet_queue).packet_count -= 1;

	1
}

/*
 * Calculate the number of nanoseconds elapsed.
 *
 * instr_count is updated in place with the remainder of the instructions
 * which didn't make up a whole nanosecond.
 */
unsafe fn cs_etm_decoder__dec_instr_count_to_ns(instr_count: *mut u32) -> u32 {
	let instr_copy: u32 = *instr_count;

	*instr_count %= INSTR_PER_NS;
	instr_copy / INSTR_PER_NS
}

unsafe fn cs_etm_decoder__gen_etmv3_config(
	params: *mut cs_etm_trace_params,
	config: *mut ocsd_etmv3_cfg,
) -> c_int {
	(*config).reg_idr = (*params).etmv3.reg_idr;
	(*config).reg_ctrl = (*params).etmv3.reg_ctrl;
	(*config).reg_ccer = (*params).etmv3.reg_ccer;
	(*config).reg_trc_id = (*params).etmv3.reg_trc_id;
	(*config).arch_ver = ARCH_V7;
	(*config).core_prof = profile_CortexA;

	0
}

const TRCIDR1_TRCARCHMIN_SHIFT: u32 = 4;
const TRCIDR1_TRCARCHMIN_MASK: u32 = GENMASK(7, 4);

fn TRCIDR1_TRCARCHMIN(x: u32) -> u32 {
	(x & TRCIDR1_TRCARCHMIN_MASK) >> TRCIDR1_TRCARCHMIN_SHIFT
}

unsafe fn cs_etm_decoder__get_etmv4_arch_ver(reg_idr1: u32) -> _ocsd_arch_version {
	/*
	 * For ETMv4 if the trace minor version is 4 or more then we can assume
	 * the architecture is ARCH_AA64 rather than just V8.
	 * ARCH_V8 = V8 architecture
	 * ARCH_AA64 = Min v8r3 plus additional AA64 PE features
	 */
	if TRCIDR1_TRCARCHMIN(reg_idr1) >= 4 { ARCH_AA64 } else { ARCH_V8 }
}

unsafe fn cs_etm_decoder__gen_etmv4_config(
	params: *mut cs_etm_trace_params,
	config: *mut ocsd_etmv4_cfg,
) {
	(*config).reg_configr = (*params).etmv4.reg_configr;
	(*config).reg_traceidr = (*params).etmv4.reg_traceidr;
	(*config).reg_idr0 = (*params).etmv4.reg_idr0;
	(*config).reg_idr1 = (*params).etmv4.reg_idr1;
	(*config).reg_idr2 = (*params).etmv4.reg_idr2;
	(*config).reg_idr8 = (*params).etmv4.reg_idr8;
	(*config).reg_idr9 = 0;
	(*config).reg_idr10 = 0;
	(*config).reg_idr11 = 0;
	(*config).reg_idr12 = 0;
	(*config).reg_idr13 = 0;
	(*config).arch_ver = cs_etm_decoder__get_etmv4_arch_ver((*params).etmv4.reg_idr1);
	(*config).core_prof = profile_CortexA;
}

unsafe fn cs_etm_decoder__gen_ete_config(params: *mut cs_etm_trace_params, config: *mut ocsd_ete_cfg) {
	(*config).reg_configr = (*params).ete.reg_configr;
	(*config).reg_traceidr = (*params).ete.reg_traceidr;
	(*config).reg_idr0 = (*params).ete.reg_idr0;
	(*config).reg_idr1 = (*params).ete.reg_idr1;
	(*config).reg_idr2 = (*params).ete.reg_idr2;
	(*config).reg_idr8 = (*params).ete.reg_idr8;
	(*config).reg_devarch = (*params).ete.reg_devarch;
	(*config).arch_ver = ARCH_AA64;
	(*config).core_prof = profile_CortexA;
}

unsafe extern "C" fn cs_etm_decoder__print_str_cb(
	p_context: *const c_void,
	msg: *const c_char,
	str_len: c_int,
) {
	let decoder = p_context as *const cs_etm_decoder;

	if !p_context.is_null() && str_len != 0 && !(*decoder).suppress_printing {
		if let Some(packet_printer) = (*decoder).packet_printer {
			packet_printer(msg, (*decoder).data);
		}
	}
}

unsafe fn cs_etm_decoder__init_def_logger_printing(
	d_params: *mut cs_etm_decoder_params,
	decoder: *mut cs_etm_decoder,
) -> c_int {
	let mut ret: c_int = 0;

	if (*d_params).packet_printer.is_none() {
		return -1;
	}

	(*decoder).packet_printer = (*d_params).packet_printer;

	/*
	 * Set up a library default logger to process any printers
	 * (packet/raw frame) we add later.
	 */
	ret = ocsd_def_errlog_init(OCSD_ERR_SEV_ERROR, 1);
	if ret != 0 {
		return -1;
	}

	/* no stdout / err / file output */
	ret = ocsd_def_errlog_config_output(C_API_MSGLOGOUT_FLG_NONE, ptr::null());
	if ret != 0 {
		return -1;
	}

	/*
	 * Set the string CB for the default logger, passes strings to
	 * perf print logger.
	 */
	ret = ocsd_def_errlog_set_strprint_cb(
		(*decoder).dcd_tree,
		decoder as *mut c_void,
		Some(cs_etm_decoder__print_str_cb),
	);
	if ret != 0 {
		return -1;
	}

	/* CS_LOG_RAW_FRAMES conditional block omitted unless enabled at build time in C. */
	0
}

unsafe fn cs_etm_decoder__do_soft_timestamp(
	etmq: *mut cs_etm_queue,
	packet_queue: *mut cs_etm_packet_queue,
	trace_chan_id: u8,
) -> ocsd_datapath_resp_t {
	let estimated_ts: u64;

	/* No timestamp packet has been received, nothing to do */
	if (*packet_queue).next_cs_timestamp == 0 {
		return OCSD_RESP_CONT;
	}

	estimated_ts = (*packet_queue).cs_timestamp
		+ cs_etm_decoder__dec_instr_count_to_ns(&mut (*packet_queue).instr_count);

	/* Estimated TS can never be higher than the next real one in the trace */
	(*packet_queue).cs_timestamp = core::cmp::min((*packet_queue).next_cs_timestamp, estimated_ts);

	/* Tell the front end which traceid_queue needs attention */
	cs_etm__etmq_set_traceid_queue_timestamp(etmq, trace_chan_id);

	OCSD_RESP_WAIT
}

unsafe fn cs_etm_decoder__do_hard_timestamp(
	etmq: *mut cs_etm_queue,
	elem: *const ocsd_generic_trace_elem,
	trace_chan_id: u8,
	indx: ocsd_trc_index_t,
) -> ocsd_datapath_resp_t {
	let packet_queue: *mut cs_etm_packet_queue;
	let converted_timestamp: u64;
	let estimated_first_ts: u64;

	/* First get the packet queue for this traceID */
	packet_queue = cs_etm__etmq_get_packet_queue(etmq, trace_chan_id);
	if packet_queue.is_null() {
		return OCSD_RESP_FATAL_SYS_ERR;
	}

	/*
	 * Coresight timestamps are raw timer values which need to be scaled to ns. Assume
	 * 0 is a bad value so don't try to convert it.
	 */
	converted_timestamp = if (*elem).timestamp != 0 {
		cs_etm__convert_sample_time(etmq, (*elem).timestamp)
	} else {
		0
	};

	/*
	 * We've seen a timestamp packet before - simply record the new value.
	 * Function do_soft_timestamp() will report the value to the front end,
	 * hence asking the decoder to keep decoding rather than stopping.
	 */
	if (*packet_queue).next_cs_timestamp != 0 {
		/*
		 * What was next is now where new ranges start from, overwriting
		 * any previous estimate in cs_timestamp
		 */
		(*packet_queue).cs_timestamp = (*packet_queue).next_cs_timestamp;
		(*packet_queue).next_cs_timestamp = converted_timestamp;
		return OCSD_RESP_CONT;
	}

	if converted_timestamp == 0 {
		/*
		 * Zero timestamps can be seen due to misconfiguration or hardware bugs.
		 * Warn once, and don't try to subtract instr_count as it would result in an
		 * underflow.
		 */
		(*packet_queue).cs_timestamp = 0;
		if !cs_etm__etmq_is_timeless(etmq) {
			pr_warning_once(
				c"Zero Coresight timestamp found at Idx:%" OCSD_TRC_IDX_STR ". Decoding may be improved by prepending 'Z' to your current --itrace arguments.\n".as_ptr(),
				indx,
			);
		}
	} else if ((*packet_queue).instr_count / INSTR_PER_NS) as u64 > converted_timestamp {
		/*
		 * Sanity check that the elem->timestamp - packet_queue->instr_count would not
		 * result in an underflow. Warn and clamp at 0 if it would.
		 */
		(*packet_queue).cs_timestamp = 0;
		pr_err(c"Timestamp calculation underflow at Idx:%" OCSD_TRC_IDX_STR "\n".as_ptr(), indx);
	} else {
		/*
		 * This is the first timestamp we've seen since the beginning of traces
		 * or a discontinuity.  Since timestamps packets are generated *after*
		 * range packets have been generated, we need to estimate the time at
		 * which instructions started by subtracting the number of instructions
		 * executed to the timestamp. Don't estimate earlier than the last used
		 * timestamp though.
		 */
		estimated_first_ts = converted_timestamp - ((*packet_queue).instr_count / INSTR_PER_NS) as u64;
		(*packet_queue).cs_timestamp = core::cmp::max((*packet_queue).cs_timestamp, estimated_first_ts);
	}
	(*packet_queue).next_cs_timestamp = converted_timestamp;
	(*packet_queue).instr_count = 0;

	/* Tell the front end which traceid_queue needs attention */
	cs_etm__etmq_set_traceid_queue_timestamp(etmq, trace_chan_id);

	/* Halt processing until we are being told to proceed */
	OCSD_RESP_WAIT
}

unsafe fn cs_etm_decoder__reset_timestamp(packet_queue: *mut cs_etm_packet_queue) {
	(*packet_queue).next_cs_timestamp = 0;
	(*packet_queue).instr_count = 0;
}

unsafe fn cs_etm_decoder__buffer_packet(
	etmq: *mut cs_etm_queue,
	packet_queue: *mut cs_etm_packet_queue,
	trace_chan_id: u8,
	sample_type: cs_etm_sample_type,
) -> ocsd_datapath_resp_t {
	let mut et: u32 = 0;
	let mut cpu: c_int = 0;

	if (*packet_queue).packet_count >= CS_ETM_PACKET_MAX_BUFFER - 1 {
		return OCSD_RESP_FATAL_SYS_ERR;
	}

	if cs_etm__get_cpu(etmq, trace_chan_id, &mut cpu) < 0 {
		return OCSD_RESP_FATAL_SYS_ERR;
	}

	et = (*packet_queue).tail;
	et = (et + 1) & (CS_ETM_PACKET_MAX_BUFFER - 1);
	(*packet_queue).tail = et;
	(*packet_queue).packet_count += 1;

	let packet = &mut (*packet_queue).packet_buffer[et as usize];
	packet.sample_type = sample_type;
	packet.isa = CS_ETM_ISA_UNKNOWN;
	packet.cpu = cpu;
	packet.start_addr = CS_ETM_INVAL_ADDR;
	packet.end_addr = CS_ETM_INVAL_ADDR;
	packet.instr_count = 0;
	packet.last_instr_taken_branch = false;
	packet.last_instr_size = 0;
	packet.last_instr_type = 0;
	packet.last_instr_subtype = 0;
	packet.last_instr_cond = 0;
	packet.flags = 0;
	packet.exception_number = UINT32_MAX;
	packet.trace_chan_id = trace_chan_id;
	packet.el = ocsd_EL_unknown;
	packet.tid = -1;

	if (*packet_queue).packet_count == CS_ETM_PACKET_MAX_BUFFER - 1 {
		return OCSD_RESP_WAIT;
	}

	OCSD_RESP_CONT
}

unsafe fn cs_etm_decoder__buffer_range(
	etmq: *mut cs_etm_queue,
	packet_queue: *mut cs_etm_packet_queue,
	elem: *const ocsd_generic_trace_elem,
	trace_chan_id: u8,
) -> ocsd_datapath_resp_t {
	let mut ret: ocsd_datapath_resp_t = OCSD_RESP_CONT;
	let packet: *mut cs_etm_packet;

	ret = cs_etm_decoder__buffer_packet(etmq, packet_queue, trace_chan_id, CS_ETM_RANGE);
	if ret != OCSD_RESP_CONT && ret != OCSD_RESP_WAIT {
		return ret;
	}

	packet = &mut (*packet_queue).packet_buffer[(*packet_queue).tail as usize];

	match (*elem).isa {
		ocsd_isa_aarch64 => (*packet).isa = CS_ETM_ISA_A64,
		ocsd_isa_arm => (*packet).isa = CS_ETM_ISA_A32,
		ocsd_isa_thumb2 => (*packet).isa = CS_ETM_ISA_T32,
		ocsd_isa_tee | ocsd_isa_jazelle | ocsd_isa_custom | ocsd_isa_unknown | _ => {
			(*packet).isa = CS_ETM_ISA_UNKNOWN
		}
	}

	(*packet).start_addr = (*elem).st_addr;
	(*packet).end_addr = (*elem).en_addr;
	(*packet).instr_count = (*elem).num_instr_range;
	(*packet).last_instr_type = (*elem).last_i_type;
	(*packet).last_instr_subtype = (*elem).last_i_subtype;
	(*packet).last_instr_cond = (*elem).last_instr_cond;
	(*packet).el = (*elem).context.exception_level;

	if (*elem).last_i_type == OCSD_INSTR_BR || (*elem).last_i_type == OCSD_INSTR_BR_INDIRECT {
		(*packet).last_instr_taken_branch = (*elem).last_instr_exec;
	} else {
		(*packet).last_instr_taken_branch = false;
	}

	(*packet).last_instr_size = (*elem).last_instr_sz;

	/* per-thread scenario, no need to generate a timestamp */
	if cs_etm__etmq_is_timeless(etmq) {
		return ret;
	}

	/*
	 * The packet queue is full and we haven't seen a timestamp (had we
	 * seen one the packet queue wouldn't be full).  Let the front end
	 * deal with it.
	 */
	if ret == OCSD_RESP_WAIT {
		return ret;
	}

	(*packet_queue).instr_count += (*elem).num_instr_range;
	/* Tell the front end we have a new timestamp to process */
	ret = cs_etm_decoder__do_soft_timestamp(etmq, packet_queue, trace_chan_id);

	ret
}

unsafe fn cs_etm_decoder__buffer_discontinuity(
	etmq: *mut cs_etm_queue,
	queue: *mut cs_etm_packet_queue,
	trace_chan_id: u8,
) -> ocsd_datapath_resp_t {
	/*
	 * Something happened and who knows when we'll get new traces so
	 * reset time statistics.
	 */
	cs_etm_decoder__reset_timestamp(queue);
	cs_etm_decoder__buffer_packet(etmq, queue, trace_chan_id, CS_ETM_DISCONTINUITY)
}

unsafe fn cs_etm_decoder__buffer_exception(
	etmq: *mut cs_etm_queue,
	queue: *mut cs_etm_packet_queue,
	elem: *const ocsd_generic_trace_elem,
	trace_chan_id: u8,
) -> ocsd_datapath_resp_t {
	let mut ret: ocsd_datapath_resp_t = OCSD_RESP_CONT;
	let packet: *mut cs_etm_packet;

	ret = cs_etm_decoder__buffer_packet(etmq, queue, trace_chan_id, CS_ETM_EXCEPTION);
	if ret != OCSD_RESP_CONT && ret != OCSD_RESP_WAIT {
		return ret;
	}

	packet = &mut (*queue).packet_buffer[(*queue).tail as usize];
	(*packet).exception_number = (*elem).exception_number;

	ret
}

unsafe fn cs_etm_decoder__buffer_exception_ret(
	etmq: *mut cs_etm_queue,
	queue: *mut cs_etm_packet_queue,
	trace_chan_id: u8,
) -> ocsd_datapath_resp_t {
	cs_etm_decoder__buffer_packet(etmq, queue, trace_chan_id, CS_ETM_EXCEPTION_RET)
}

unsafe fn cs_etm_decoder__set_tid(
	etmq: *mut cs_etm_queue,
	packet_queue: *mut cs_etm_packet_queue,
	elem: *const ocsd_generic_trace_elem,
	trace_chan_id: u8,
) -> ocsd_datapath_resp_t {
	let packet: *mut cs_etm_packet;
	let mut tid: pid_t = -1;
	let ret: ocsd_datapath_resp_t;

	/*
	 * Process the PE_CONTEXT packets if we have a valid contextID or VMID.
	 * If the kernel is running at EL2, the PID is traced in CONTEXTIDR_EL2
	 * as VMID, Format attribute 'contextid2' is set in this case.
	 */
	match cs_etm__get_pid_fmt(etmq) {
		CS_ETM_PIDFMT_CTXTID => {
			if (*elem).context.ctxt_id_valid {
				tid = (*elem).context.context_id;
			}
		}
		CS_ETM_PIDFMT_CTXTID2 => {
			if (*elem).context.vmid_valid {
				tid = (*elem).context.vmid;
			}
		}
		CS_ETM_PIDFMT_NONE | _ => {}
	}

	if cs_etm__etmq_update_decode_context(
		etmq,
		trace_chan_id,
		(*elem).context.exception_level,
		tid,
	) != 0
	{
		return OCSD_RESP_FATAL_SYS_ERR;
	}

	ret = cs_etm_decoder__buffer_packet(etmq, packet_queue, trace_chan_id, CS_ETM_CONTEXT);
	if ret != OCSD_RESP_CONT && ret != OCSD_RESP_WAIT {
		return ret;
	}

	packet = &mut (*packet_queue).packet_buffer[(*packet_queue).tail as usize];
	(*packet).tid = tid;
	(*packet).el = (*elem).context.exception_level;

	/*
	 * A timestamp is generated after a PE_CONTEXT element so make sure
	 * to rely on that coming one.
	 */
	cs_etm_decoder__reset_timestamp(packet_queue);

	ret
}

unsafe extern "C" fn cs_etm_decoder__gen_trace_elem_printer(
	context: *const c_void,
	indx: ocsd_trc_index_t,
	trace_chan_id: u8,
	elem: *const ocsd_generic_trace_elem,
) -> ocsd_datapath_resp_t {
	let mut resp: ocsd_datapath_resp_t = OCSD_RESP_CONT;
	let type_: ocsd_gen_trc_elem_t;
	let decoder = context as *mut cs_etm_decoder;
	let etmq = (*decoder).data as *mut cs_etm_queue;
	let packet_queue: *mut cs_etm_packet_queue;

	/* First get the packet queue for this traceID */
	packet_queue = cs_etm__etmq_get_packet_queue(etmq, trace_chan_id);
	if packet_queue.is_null() {
		return OCSD_RESP_FATAL_SYS_ERR;
	}

	type_ = (*elem).elem_type;

	if type_ == OCSD_GEN_TRC_ELEM_EO_TRACE
		|| type_ == OCSD_GEN_TRC_ELEM_NO_SYNC
		|| type_ == OCSD_GEN_TRC_ELEM_TRACE_ON
	{
		resp = cs_etm_decoder__buffer_discontinuity(etmq, packet_queue, trace_chan_id);
	} else if type_ == OCSD_GEN_TRC_ELEM_INSTR_RANGE {
		resp = cs_etm_decoder__buffer_range(etmq, packet_queue, elem, trace_chan_id);
	} else if type_ == OCSD_GEN_TRC_ELEM_EXCEPTION {
		resp = cs_etm_decoder__buffer_exception(etmq, packet_queue, elem, trace_chan_id);
	} else if type_ == OCSD_GEN_TRC_ELEM_EXCEPTION_RET {
		resp = cs_etm_decoder__buffer_exception_ret(etmq, packet_queue, trace_chan_id);
	} else if type_ == OCSD_GEN_TRC_ELEM_TIMESTAMP {
		resp = cs_etm_decoder__do_hard_timestamp(etmq, elem, trace_chan_id, indx);
	} else if type_ == OCSD_GEN_TRC_ELEM_PE_CONTEXT {
		resp = cs_etm_decoder__set_tid(etmq, packet_queue, elem, trace_chan_id);
	}

	resp
}

unsafe fn cs_etm_decoder__create_etm_decoder(
	d_params: *mut cs_etm_decoder_params,
	t_params: *mut cs_etm_trace_params,
	decoder: *mut cs_etm_decoder,
) -> c_int {
	let mut config_etmv3: ocsd_etmv3_cfg = core::mem::zeroed();
	let mut trace_config_etmv4: ocsd_etmv4_cfg = core::mem::zeroed();
	let mut trace_config_ete: ocsd_ete_cfg = core::mem::zeroed();
	let trace_config: *mut c_void;
	let mut csid: u8;

	match (*t_params).protocol {
		CS_ETM_PROTO_ETMV3 | CS_ETM_PROTO_PTM => {
			csid = ((*t_params).etmv3.reg_idr & CORESIGHT_TRACE_ID_VAL_MASK) as u8;
			cs_etm_decoder__gen_etmv3_config(t_params, &mut config_etmv3);
			(*decoder).decoder_name = if (*t_params).protocol == CS_ETM_PROTO_ETMV3 {
				OCSD_BUILTIN_DCD_ETMV3
			} else {
				OCSD_BUILTIN_DCD_PTM
			};
			trace_config = &mut config_etmv3 as *mut _ as *mut c_void;
		}
		CS_ETM_PROTO_ETMV4i => {
			csid = ((*t_params).etmv4.reg_traceidr & CORESIGHT_TRACE_ID_VAL_MASK) as u8;
			cs_etm_decoder__gen_etmv4_config(t_params, &mut trace_config_etmv4);
			(*decoder).decoder_name = OCSD_BUILTIN_DCD_ETMV4I;
			trace_config = &mut trace_config_etmv4 as *mut _ as *mut c_void;
		}
		CS_ETM_PROTO_ETE => {
			csid = ((*t_params).ete.reg_traceidr & CORESIGHT_TRACE_ID_VAL_MASK) as u8;
			cs_etm_decoder__gen_ete_config(t_params, &mut trace_config_ete);
			(*decoder).decoder_name = OCSD_BUILTIN_DCD_ETE;
			trace_config = &mut trace_config_ete as *mut _ as *mut c_void;
		}
		_ => return -1,
	}

	if (*d_params).operation == CS_ETM_OPERATION_DECODE {
		let mut decode_flags: c_int = OCSD_CREATE_FLG_FULL_DECODER;
		/* C conditional intent:
		 * #ifdef OCSD_OPFLG_N_UNCOND_DIR_BR_CHK
		 * decode_flags |= OCSD_OPFLG_N_UNCOND_DIR_BR_CHK |
		 *                 OCSD_OPFLG_CHK_RANGE_CONTINUE |
		 *                 ETM4_OPFLG_PKTDEC_AA64_OPCODE_CHK;
		 * #endif
		 */
		if ocsd_dt_create_decoder(
			(*decoder).dcd_tree,
			(*decoder).decoder_name,
			decode_flags,
			trace_config,
			&mut csid,
		) != 0
		{
			return -1;
		}

		if ocsd_dt_set_gen_elem_outfn(
			(*decoder).dcd_tree,
			Some(cs_etm_decoder__gen_trace_elem_printer),
			decoder as *mut c_void,
		) != 0
		{
			return -1;
		}

		return 0;
	} else if (*d_params).operation == CS_ETM_OPERATION_PRINT {
		if ocsd_dt_create_decoder(
			(*decoder).dcd_tree,
			(*decoder).decoder_name,
			OCSD_CREATE_FLG_PACKET_PROC,
			trace_config,
			&mut csid,
		) != 0
		{
			return -1;
		}

		if ocsd_dt_set_pkt_protocol_printer((*decoder).dcd_tree, csid, CS_PKT_MON) != 0 {
			return -1;
		}

		return 0;
	}

	-1
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm_decoder__new(
	decoders: c_int,
	d_params: *mut cs_etm_decoder_params,
	t_params: *mut cs_etm_trace_params,
) -> *mut cs_etm_decoder {
	let decoder: *mut cs_etm_decoder;
	let format: ocsd_dcd_tree_src_t;
	let mut flags: u32;
	let mut i: c_int;
	let mut ret: c_int;

	if t_params.is_null() || d_params.is_null() {
		return ptr::null_mut();
	}

	decoder = zalloc(size_of::<cs_etm_decoder>()) as *mut cs_etm_decoder;

	if decoder.is_null() {
		return ptr::null_mut();
	}

	(*decoder).data = (*d_params).data;
	(*decoder).prev_return = OCSD_RESP_CONT;
	format = if (*d_params).formatted {
		OCSD_TRC_SRC_FRAME_FORMATTED
	} else {
		OCSD_TRC_SRC_SINGLE
	};
	flags = 0;
	flags |= if (*d_params).fsyncs { OCSD_DFRMTR_HAS_FSYNCS } else { 0 };
	flags |= if (*d_params).hsyncs { OCSD_DFRMTR_HAS_HSYNCS } else { 0 };
	flags |= if (*d_params).frame_aligned { OCSD_DFRMTR_FRAME_MEM_ALIGN } else { 0 };

	/*
	 * Drivers may add barrier frames when used with perf, set up to
	 * handle this. Barriers const of FSYNC packet repeated 4 times.
	 */
	flags |= OCSD_DFRMTR_RESET_ON_4X_FSYNC;

	/* Create decode tree for the data source */
	(*decoder).dcd_tree = ocsd_create_dcd_tree(format, flags);

	if (*decoder).dcd_tree == 0 as dcd_tree_handle_t {
		goto_err_free_decoder(decoder);
		return ptr::null_mut();
	}

	/* init library print logging support */
	ret = cs_etm_decoder__init_def_logger_printing(d_params, decoder);
	if ret != 0 {
		goto_err_free_decoder(decoder);
		return ptr::null_mut();
	}

	i = 0;
	while i < decoders {
		ret = cs_etm_decoder__create_etm_decoder(d_params, t_params.add(i as usize), decoder);
		if ret != 0 {
			goto_err_free_decoder(decoder);
			return ptr::null_mut();
		}
		i += 1;
	}

	decoder
}

unsafe fn goto_err_free_decoder(decoder: *mut cs_etm_decoder) {
	cs_etm_decoder__free(decoder);
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm_decoder__process_data_block(
	decoder: *mut cs_etm_decoder,
	indx: u64,
	buf: *const u8,
	len: usize,
	consumed: *mut usize,
) -> c_int {
	let mut ret: c_int = 0;
	let mut cur: ocsd_datapath_resp_t = OCSD_RESP_CONT;
	let mut prev_return: ocsd_datapath_resp_t = (*decoder).prev_return;
	let mut processed: usize = 0;
	let mut count: u32 = 0;

	while processed < len {
		if OCSD_DATA_RESP_IS_WAIT(prev_return) {
			cur = ocsd_dt_process_data(
				(*decoder).dcd_tree,
				OCSD_OP_FLUSH,
				0,
				0,
				ptr::null(),
				ptr::null_mut(),
			);
		} else if OCSD_DATA_RESP_IS_CONT(prev_return) {
			cur = ocsd_dt_process_data(
				(*decoder).dcd_tree,
				OCSD_OP_DATA,
				indx + processed as u64,
				len - processed,
				buf.add(processed),
				&mut count,
			);
			processed += count as usize;
		} else {
			ret = -EINVAL;
			break;
		}

		/*
		 * Return to the input code if the packet buffer is full.
		 * Flushing will get done once the packet buffer has been
		 * processed.
		 */
		if OCSD_DATA_RESP_IS_WAIT(cur) {
			break;
		}

		prev_return = cur;
	}

	(*decoder).prev_return = cur;
	*consumed = processed;

	ret
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm_decoder__free(decoder: *mut cs_etm_decoder) {
	if decoder.is_null() {
		return;
	}

	ocsd_destroy_dcd_tree((*decoder).dcd_tree);
	(*decoder).dcd_tree = ptr::null_mut();
	free(decoder as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm_decoder__get_name(decoder: *mut cs_etm_decoder) -> *const c_char {
	(*decoder).decoder_name
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
