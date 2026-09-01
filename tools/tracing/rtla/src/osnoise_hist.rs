// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_longlong, c_ulonglong, c_void};

#[repr(C)]
pub struct tracefs_hist {
	_private: [u8; 0],
}

#[repr(C)]
pub struct trace_seq {
	_private: [u8; 0],
}

#[repr(C)]
pub struct trace_instance {
	pub seq: *mut trace_seq,
	pub inst: *mut c_void,
	pub tep: *mut c_void,
}

#[repr(C)]
pub struct hist_params {
	pub entries: c_int,
	pub bucket_size: c_int,
	pub no_header: c_int,
	pub no_index: c_int,
	pub no_summary: c_int,
	pub with_zeros: c_int,
}

#[repr(C)]
pub struct common_params {
	pub output_divisor: c_int,
	pub hist: hist_params,
}

#[repr(C)]
pub struct osnoise_params {
	pub common: common_params,
}

#[repr(C)]
pub struct osnoise_tool {
	pub data: *mut osnoise_hist_data,
	pub params: *mut c_void,
	pub trace: trace_instance,
	pub start_time: c_ulonglong,
}

#[repr(C)]
pub struct tool_ops {
	pub tracer: *const c_char,
	pub comm_prefix: *const c_char,
	pub parse_args: Option<unsafe extern "C" fn() -> c_int>,
	pub init_tool: Option<unsafe extern "C" fn(*mut common_params) -> *mut osnoise_tool>,
	pub apply_config: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
	pub enable: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
	pub main: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
	pub print_stats: Option<unsafe extern "C" fn(*mut osnoise_tool)>,
	pub free: Option<unsafe extern "C" fn(*mut osnoise_tool)>,
}

#[repr(C)]
pub struct osnoise_hist_cpu {
	pub samples: *mut c_int,
	pub count: c_int,

	pub min_sample: c_ulonglong,
	pub sum_sample: c_ulonglong,
	pub max_sample: c_ulonglong,
}

#[repr(C)]
pub struct osnoise_hist_data {
	pub trace_hist: *mut tracefs_hist,
	pub hist: *mut osnoise_hist_cpu,
	pub entries: c_int,
	pub bucket_size: c_int,
}

const TRACEFS_HIST_KEY_NORMAL: c_int = 0;

unsafe extern "C" {
	static nr_cpus: c_int;

	fn calloc(nmemb: usize, size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
	fn strlen(s: *const c_char) -> usize;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

	fn to_osnoise_params(params: *mut c_void) -> *mut osnoise_params;
	fn update_min(current: *mut c_ulonglong, value: *const c_ulonglong);
	fn update_sum(current: *mut c_ulonglong, value: *const c_ulonglong);
	fn update_max(current: *mut c_ulonglong, value: *const c_ulonglong);

	fn tracefs_hist_pause(inst: *mut c_void, hist: *mut tracefs_hist);
	fn tracefs_hist_destroy(inst: *mut c_void, hist: *mut tracefs_hist);
	fn tracefs_hist_alloc(
		tep: *mut c_void,
		system: *const c_char,
		event: *const c_char,
		key: *const c_char,
		flags: c_int,
	) -> *mut tracefs_hist;
	fn tracefs_hist_add_key(hist: *mut tracefs_hist, key: *const c_char, flags: c_int) -> c_int;
	fn tracefs_hist_start(inst: *mut c_void, hist: *mut tracefs_hist) -> c_int;
	fn tracefs_event_file_read(
		inst: *mut c_void,
		system: *const c_char,
		event: *const c_char,
		file: *const c_char,
		psize: *mut c_void,
	) -> *mut c_char;

	fn get_llong_from_str(str_: *const c_char) -> c_longlong;
	fn err_msg(format: *const c_char, ...);
	fn get_duration(start_time: c_ulonglong, duration: *mut c_char, size: usize);
	fn trace_seq_printf(s: *mut trace_seq, format: *const c_char, ...);
	fn trace_seq_do_printf(s: *mut trace_seq);
	fn trace_seq_reset(s: *mut trace_seq);

	fn osnoise_report_missed_events(tool: *mut osnoise_tool);
	fn osnoise_apply_config(tool: *mut osnoise_tool, params: *mut osnoise_params) -> c_int;
	fn osnoise_init_tool(name: *const c_char) -> *mut osnoise_tool;
	fn osnoise_destroy_tool(tool: *mut osnoise_tool);
	fn osnoise_enable(tool: *mut osnoise_tool) -> c_int;
	fn hist_main_loop(tool: *mut osnoise_tool) -> c_int;
	fn osnoise_hist_parse_args() -> c_int;
}

macro_rules! for_each_monitored_cpu {
	($cpu:ident, $common:expr, $body:block) => {{
		/* C source uses for_each_monitored_cpu(cpu, &params->common). */
		let mut $cpu: c_int = 0;
		while $cpu < nr_cpus {
			let _ = $common;
			$body
			$cpu += 1;
		}
	}};
}

/*
 * osnoise_free_histogram - free runtime data
 */
unsafe fn osnoise_free_histogram(data: *mut osnoise_hist_data) {
	let mut cpu: c_int;

	/* one histogram for IRQ and one for thread, per CPU */
	cpu = 0;
	while cpu < nr_cpus {
		if !(*(*data).hist.add(cpu as usize)).samples.is_null() {
			free((*(*data).hist.add(cpu as usize)).samples as *mut c_void);
		}
		cpu += 1;
	}

	/* one set of histograms per CPU */
	if !(*data).hist.is_null() {
		free((*data).hist as *mut c_void);
	}

	free(data as *mut c_void);
}

unsafe extern "C" fn osnoise_free_hist_tool(tool: *mut osnoise_tool) {
	osnoise_free_histogram((*tool).data);
}

/*
 * osnoise_alloc_histogram - alloc runtime data
 */
unsafe fn osnoise_alloc_histogram(entries: c_int, bucket_size: c_int) -> *mut osnoise_hist_data {
	let data: *mut osnoise_hist_data;
	let mut cpu: c_int;

	data = calloc(1, core::mem::size_of::<osnoise_hist_data>()) as *mut osnoise_hist_data;
	if data.is_null() {
		return core::ptr::null_mut();
	}

	(*data).entries = entries;
	(*data).bucket_size = bucket_size;

	(*data).hist = calloc(
		1,
		core::mem::size_of::<osnoise_hist_cpu>() * nr_cpus as usize,
	) as *mut osnoise_hist_cpu;
	if (*data).hist.is_null() {
		osnoise_free_histogram(data);
		return core::ptr::null_mut();
	}

	cpu = 0;
	while cpu < nr_cpus {
		(*(*data).hist.add(cpu as usize)).samples = calloc(
			1,
			core::mem::size_of::<c_int>() * (entries + 1) as usize,
		) as *mut c_int;
		if (*(*data).hist.add(cpu as usize)).samples.is_null() {
			osnoise_free_histogram(data);
			return core::ptr::null_mut();
		}
		cpu += 1;
	}

	/* set the min to max */
	cpu = 0;
	while cpu < nr_cpus {
		(*(*data).hist.add(cpu as usize)).min_sample = !0;
		cpu += 1;
	}

	data
}

unsafe fn osnoise_hist_update_multiple(
	tool: *mut osnoise_tool,
	cpu: c_int,
	mut duration: c_ulonglong,
	count: c_int,
) {
	let params: *mut osnoise_params = to_osnoise_params((*tool).params);
	let data: *mut osnoise_hist_data = (*tool).data;
	let total_duration: c_ulonglong;
	let entries: c_int = (*data).entries;
	let bucket: c_int;
	let hist: *mut c_int;

	if (*params).common.output_divisor != 0 {
		duration = duration / (*params).common.output_divisor as c_ulonglong;
	}

	bucket = (duration / (*data).bucket_size as c_ulonglong) as c_int;

	total_duration = duration * count as c_ulonglong;

	hist = (*(*data).hist.add(cpu as usize)).samples;
	(*(*data).hist.add(cpu as usize)).count += count;
	update_min(&mut (*(*data).hist.add(cpu as usize)).min_sample, &duration);
	update_sum(&mut (*(*data).hist.add(cpu as usize)).sum_sample, &total_duration);
	update_max(&mut (*(*data).hist.add(cpu as usize)).max_sample, &duration);

	if bucket < entries {
		*hist.add(bucket as usize) += count;
	} else {
		*hist.add(entries as usize) += count;
	}
}

/*
 * osnoise_destroy_trace_hist - disable events used to collect histogram
 */
unsafe fn osnoise_destroy_trace_hist(tool: *mut osnoise_tool) {
	let data: *mut osnoise_hist_data = (*tool).data;

	tracefs_hist_pause((*tool).trace.inst, (*data).trace_hist);
	tracefs_hist_destroy((*tool).trace.inst, (*data).trace_hist);
}

/*
 * osnoise_init_trace_hist - enable events used to collect histogram
 */
unsafe fn osnoise_init_trace_hist(tool: *mut osnoise_tool) -> c_int {
	let params: *mut osnoise_params = to_osnoise_params((*tool).params);
	let data: *mut osnoise_hist_data = (*tool).data;
	let bucket_size: c_int;
	let mut buff = [0 as c_char; 128];
	let mut retval: c_int = 0;

	/*
	 * Set the size of the bucket.
	 */
	bucket_size = (*params).common.output_divisor * (*params).common.hist.bucket_size;
	snprintf(
		buff.as_mut_ptr(),
		buff.len(),
		c"duration.buckets=%d".as_ptr(),
		bucket_size,
	);

	(*data).trace_hist = tracefs_hist_alloc(
		(*tool).trace.tep,
		c"osnoise".as_ptr(),
		c"sample_threshold".as_ptr(),
		buff.as_ptr(),
		TRACEFS_HIST_KEY_NORMAL,
	);
	if (*data).trace_hist.is_null() {
		return 1;
	}

	retval = tracefs_hist_add_key((*data).trace_hist, c"cpu".as_ptr(), 0);
	if retval != 0 {
		osnoise_destroy_trace_hist(tool);
		return 1;
	}

	retval = tracefs_hist_start((*tool).trace.inst, (*data).trace_hist);
	if retval != 0 {
		osnoise_destroy_trace_hist(tool);
		return 1;
	}

	0
}

/*
 * osnoise_read_trace_hist - parse histogram file and file osnoise histogram
 */
unsafe fn osnoise_read_trace_hist(tool: *mut osnoise_tool) {
	let data: *mut osnoise_hist_data = (*tool).data;
	let mut cpu: c_longlong;
	let mut counter: c_longlong;
	let mut duration: c_longlong;
	let content: *mut c_char;
	let mut position: *mut c_char;

	tracefs_hist_pause((*tool).trace.inst, (*data).trace_hist);

	content = tracefs_event_file_read(
		(*tool).trace.inst,
		c"osnoise".as_ptr(),
		c"sample_threshold".as_ptr(),
		c"hist".as_ptr(),
		core::ptr::null_mut(),
	);
	if content.is_null() {
		return;
	}

	position = content;
	loop {
		position = strstr(position, c"duration: ~".as_ptr());
		if position.is_null() {
			break;
		}
		position = position.add(strlen(c"duration: ~".as_ptr()));
		duration = get_llong_from_str(position);
		if duration == -1 {
			err_msg(c"error reading duration from histogram\n".as_ptr());
		}

		position = strstr(position, c"cpu:".as_ptr());
		if position.is_null() {
			break;
		}
		position = position.add(strlen(c"cpu: ".as_ptr()));
		cpu = get_llong_from_str(position);
		if cpu == -1 {
			err_msg(c"error reading cpu from histogram\n".as_ptr());
		}

		position = strstr(position, c"hitcount:".as_ptr());
		if position.is_null() {
			break;
		}
		position = position.add(strlen(c"hitcount: ".as_ptr()));
		counter = get_llong_from_str(position);
		if counter == -1 {
			err_msg(c"error reading counter from histogram\n".as_ptr());
		}

		osnoise_hist_update_multiple(tool, cpu as c_int, duration as c_ulonglong, counter as c_int);
	}
	free(content as *mut c_void);
}

/*
 * osnoise_hist_header - print the header of the tracer to the output
 */
unsafe fn osnoise_hist_header(tool: *mut osnoise_tool) {
	let params: *mut osnoise_params = to_osnoise_params((*tool).params);
	let data: *mut osnoise_hist_data = (*tool).data;
	let s: *mut trace_seq = (*tool).trace.seq;
	let mut duration = [0 as c_char; 26];

	if (*params).common.hist.no_header != 0 {
		return;
	}

	get_duration((*tool).start_time, duration.as_mut_ptr(), duration.len());
	trace_seq_printf(s, c"# RTLA osnoise histogram\n".as_ptr());
	trace_seq_printf(
		s,
		c"# Time unit is %s (%s)\n".as_ptr(),
		if (*params).common.output_divisor == 1 {
			c"nanoseconds".as_ptr()
		} else {
			c"microseconds".as_ptr()
		},
		if (*params).common.output_divisor == 1 {
			c"ns".as_ptr()
		} else {
			c"us".as_ptr()
		},
	);

	trace_seq_printf(s, c"# Duration: %s\n".as_ptr(), duration.as_ptr());

	if (*params).common.hist.no_index == 0 {
		trace_seq_printf(s, c"Index".as_ptr());
	}

	for_each_monitored_cpu!(cpu, &mut (*params).common, {
		if (*(*data).hist.add(cpu as usize)).count == 0 {
			continue;
		}

		trace_seq_printf(s, c"   CPU-%03d".as_ptr(), cpu);
	});
	trace_seq_printf(s, c"\n".as_ptr());

	trace_seq_do_printf(s);
	trace_seq_reset(s);
}

/*
 * osnoise_print_summary - print the summary of the hist data to the output
 */
unsafe fn osnoise_print_summary(
	params: *mut osnoise_params,
	trace: *mut trace_instance,
	data: *mut osnoise_hist_data,
) {
	if (*params).common.hist.no_summary != 0 {
		return;
	}

	if (*params).common.hist.no_index == 0 {
		trace_seq_printf((*trace).seq, c"count:".as_ptr());
	}

	for_each_monitored_cpu!(cpu, &mut (*params).common, {
		if (*(*data).hist.add(cpu as usize)).count == 0 {
			continue;
		}

		trace_seq_printf((*trace).seq, c"%9d ".as_ptr(), (*(*data).hist.add(cpu as usize)).count);
	});
	trace_seq_printf((*trace).seq, c"\n".as_ptr());

	if (*params).common.hist.no_index == 0 {
		trace_seq_printf((*trace).seq, c"min:  ".as_ptr());
	}

	for_each_monitored_cpu!(cpu, &mut (*params).common, {
		if (*(*data).hist.add(cpu as usize)).count == 0 {
			continue;
		}

		trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), (*(*data).hist.add(cpu as usize)).min_sample);
	});
	trace_seq_printf((*trace).seq, c"\n".as_ptr());

	if (*params).common.hist.no_index == 0 {
		trace_seq_printf((*trace).seq, c"avg:  ".as_ptr());
	}

	for_each_monitored_cpu!(cpu, &mut (*params).common, {
		if (*(*data).hist.add(cpu as usize)).count == 0 {
			continue;
		}

		if (*(*data).hist.add(cpu as usize)).count != 0 {
			trace_seq_printf(
				(*trace).seq,
				c"%9.2f ".as_ptr(),
				((*(*data).hist.add(cpu as usize)).sum_sample as c_double)
					/ (*(*data).hist.add(cpu as usize)).count as c_double,
			);
		} else {
			trace_seq_printf((*trace).seq, c"        - ".as_ptr());
		}
	});
	trace_seq_printf((*trace).seq, c"\n".as_ptr());

	if (*params).common.hist.no_index == 0 {
		trace_seq_printf((*trace).seq, c"max:  ".as_ptr());
	}

	for_each_monitored_cpu!(cpu, &mut (*params).common, {
		if (*(*data).hist.add(cpu as usize)).count == 0 {
			continue;
		}

		trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), (*(*data).hist.add(cpu as usize)).max_sample);
	});
	trace_seq_printf((*trace).seq, c"\n".as_ptr());
	trace_seq_do_printf((*trace).seq);
	trace_seq_reset((*trace).seq);
}

/*
 * osnoise_print_stats - print data for all CPUs
 */
unsafe fn osnoise_print_stats(tool: *mut osnoise_tool) {
	let params: *mut osnoise_params = to_osnoise_params((*tool).params);
	let data: *mut osnoise_hist_data = (*tool).data;
	let trace: *mut trace_instance = &mut (*tool).trace;
	let mut has_samples: c_int = 0;
	let mut bucket: c_int;
	let mut total: c_int;

	osnoise_hist_header(tool);

	bucket = 0;
	while bucket < (*data).entries {
		total = 0;

		if (*params).common.hist.no_index == 0 {
			trace_seq_printf((*trace).seq, c"%-6d".as_ptr(), bucket * (*data).bucket_size);
		}

		for_each_monitored_cpu!(cpu, &mut (*params).common, {
			if (*(*data).hist.add(cpu as usize)).count == 0 {
				continue;
			}

			total += *(*(*data).hist.add(cpu as usize)).samples.add(bucket as usize);
			trace_seq_printf(
				(*trace).seq,
				c"%9d ".as_ptr(),
				*(*(*data).hist.add(cpu as usize)).samples.add(bucket as usize),
			);
		});

		if total == 0 && (*params).common.hist.with_zeros == 0 {
			trace_seq_reset((*trace).seq);
			bucket += 1;
			continue;
		}

		/* There are samples above the threshold */
		has_samples = 1;
		trace_seq_printf((*trace).seq, c"\n".as_ptr());
		trace_seq_do_printf((*trace).seq);
		trace_seq_reset((*trace).seq);
		bucket += 1;
	}

	/*
	 * If no samples were recorded, skip calculations, print zeroed statistics
	 * and return.
	 */
	if has_samples == 0 {
		trace_seq_reset((*trace).seq);
		trace_seq_printf((*trace).seq, c"over: 0\ncount: 0\nmin: 0\navg: 0\nmax: 0\n".as_ptr());
		trace_seq_do_printf((*trace).seq);
		trace_seq_reset((*trace).seq);
		return;
	}

	if (*params).common.hist.no_index == 0 {
		trace_seq_printf((*trace).seq, c"over: ".as_ptr());
	}

	for_each_monitored_cpu!(cpu, &mut (*params).common, {
		if (*(*data).hist.add(cpu as usize)).count == 0 {
			continue;
		}

		trace_seq_printf(
			(*trace).seq,
			c"%9d ".as_ptr(),
			*(*(*data).hist.add(cpu as usize)).samples.add((*data).entries as usize),
		);
	});
	trace_seq_printf((*trace).seq, c"\n".as_ptr());
	trace_seq_do_printf((*trace).seq);
	trace_seq_reset((*trace).seq);

	osnoise_print_summary(params, trace, data);
	osnoise_report_missed_events(tool);
}

/*
 * osnoise_hist_apply_config - apply the hist configs to the initialized tool
 */
unsafe extern "C" fn osnoise_hist_apply_config(tool: *mut osnoise_tool) -> c_int {
	osnoise_apply_config(tool, to_osnoise_params((*tool).params))
}

/*
 * osnoise_init_hist - initialize a osnoise hist tool with parameters
 */
unsafe extern "C" fn osnoise_init_hist(params: *mut common_params) -> *mut osnoise_tool {
	let tool: *mut osnoise_tool;

	tool = osnoise_init_tool(c"osnoise_hist".as_ptr());
	if tool.is_null() {
		return core::ptr::null_mut();
	}

	(*tool).data = osnoise_alloc_histogram((*params).hist.entries, (*params).hist.bucket_size);
	if (*tool).data.is_null() {
		osnoise_destroy_tool(tool);
		return core::ptr::null_mut();
	}

	tool
}

unsafe extern "C" fn osnoise_hist_enable(tool: *mut osnoise_tool) -> c_int {
	let retval: c_int;

	retval = osnoise_init_trace_hist(tool);
	if retval != 0 {
		return retval;
	}

	osnoise_enable(tool)
}

unsafe extern "C" fn osnoise_hist_main_loop(tool: *mut osnoise_tool) -> c_int {
	let retval: c_int;

	retval = hist_main_loop(tool);
	osnoise_read_trace_hist(tool);

	retval
}

unsafe extern "C" fn osnoise_print_stats_wrapper(tool: *mut osnoise_tool) {
	osnoise_print_stats(tool);
}

#[unsafe(no_mangle)]
pub static mut osnoise_hist_ops: tool_ops = tool_ops {
	tracer: c"osnoise".as_ptr(),
	comm_prefix: c"osnoise/".as_ptr(),
	parse_args: Some(osnoise_hist_parse_args),
	init_tool: Some(osnoise_init_hist),
	apply_config: Some(osnoise_hist_apply_config),
	enable: Some(osnoise_hist_enable),
	main: Some(osnoise_hist_main_loop),
	print_stats: Some(osnoise_print_stats_wrapper),
	free: Some(osnoise_free_hist_tool),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
