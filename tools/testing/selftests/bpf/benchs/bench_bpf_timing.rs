// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_double, c_int, c_uint, c_void};

type __u32 = u32;
type __u64 = u64;

// Dependencies from bench_bpf_timing.h and bpf_util.h.
const BENCH_NR_CPUS: usize = 0;
const BENCH_NR_SAMPLES: usize = 0;

type bpf_bench_run_fn = Option<unsafe extern "C" fn(*mut c_void)>;

#[repr(C)]
pub struct bench_res {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bench_env {
	pub warmup_sec: c_int,
	pub producer_cnt: c_int,
}

unsafe extern "C" {
	static mut env: bench_env;
	static mut stderr: *mut c_void;

	fn bpf_num_possible_cpus() -> c_uint;
	fn bench_force_done();

	fn calloc(nmemb: usize, size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;
	fn qsort(
		base: *mut c_void,
		nmemb: usize,
		size: usize,
		compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
	);
	fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn sqrt(x: c_double) -> c_double;
	fn fabs(x: c_double) -> c_double;
}

#[repr(C)]
pub struct bpf_bench_timing {
	pub warmup_ticks: c_int,
	pub batch_iters: __u32,
	pub target_samples: __u32,
	pub done: bool,
	pub machine_readable: bool,
	pub timing_enabled: *mut __u32,
	pub batch_iters_bss: *mut __u32,
	pub idx: [__u32; BENCH_NR_CPUS],
	pub samples: [[__u64; BENCH_NR_SAMPLES]; BENCH_NR_CPUS],
}

#[repr(C)]
struct timing_stats {
	min: c_double,
	max: c_double,
	median: c_double,
	p99: c_double,
	mean: c_double,
	stddev: c_double,
	count: c_int,
}

unsafe extern "C" fn cmp_double(a: *const c_void, b: *const c_void) -> c_int {
	let da = *(a as *const c_double);
	let db = *(b as *const c_double);

	if da < db {
		return -1;
	}
	if da > db {
		return 1;
	}
	0
}

unsafe fn percentile(sorted: *const c_double, n: c_int, pct: c_double) -> c_double {
	let mut idx = (n as c_double * pct / 100.0) as c_int;

	if idx >= n {
		idx = n - 1;
	}
	*sorted.add(idx as usize)
}

unsafe fn collect_samples(t: *mut bpf_bench_timing, out: *mut c_double, max_out: c_int) -> c_int {
	let mut nr_cpus = bpf_num_possible_cpus();
	let timed_iters: __u32 = (*t).batch_iters;
	let mut total: c_int = 0;

	if nr_cpus > BENCH_NR_CPUS as c_uint {
		nr_cpus = BENCH_NR_CPUS as c_uint;
	}

	for cpu in 0..nr_cpus {
		let mut count: __u32 = (*t).idx[cpu as usize];

		if count > BENCH_NR_SAMPLES as __u32 {
			count = BENCH_NR_SAMPLES as __u32;
		}

		let mut i: __u32 = 0;
		while i < count && total < max_out {
			let sample: __u64 = (*t).samples[cpu as usize][i as usize];

			if sample != 0 {
				*out.add(total as usize) = sample as c_double / timed_iters as c_double;
				total += 1;
			}
			i += 1;
		}
	}

	qsort(
		out as *mut c_void,
		total as usize,
		core::mem::size_of::<c_double>(),
		Some(cmp_double),
	);
	total
}

unsafe fn filter_outliers_iqr(sorted: *mut c_double, n: c_int) -> c_int {
	let q1: c_double;
	let q3: c_double;
	let iqr: c_double;
	let lo: c_double;
	let hi: c_double;
	let mut start: c_int = 0;
	let mut end: c_int = n;

	if n < 8 {
		return n;
	}

	q1 = *sorted.add((n / 4) as usize);
	q3 = *sorted.add((3 * n / 4) as usize);
	iqr = q3 - q1;
	lo = q1 - 1.5 * iqr;
	hi = q3 + 1.5 * iqr;

	while start < end && *sorted.add(start as usize) < lo {
		start += 1;
	}
	while end > start && *sorted.add((end - 1) as usize) > hi {
		end -= 1;
	}

	if start > 0 {
		memmove(
			sorted as *mut c_void,
			sorted.add(start as usize) as *const c_void,
			((end - start) as usize) * core::mem::size_of::<c_double>(),
		);
	}

	end - start
}

unsafe fn compute_stats(sorted: *const c_double, n: c_int, s: *mut timing_stats) {
	let mut sum: c_double = 0.0;
	let mut var_sum: c_double = 0.0;

	memset(
		s as *mut c_void,
		0,
		core::mem::size_of_val(&*s),
	);
	(*s).count = n;

	if n == 0 {
		return;
	}

	(*s).min = *sorted.add(0);
	(*s).max = *sorted.add((n - 1) as usize);
	(*s).median = *sorted.add((n / 2) as usize);
	(*s).p99 = percentile(sorted, n, 99.0);

	for i in 0..n {
		sum += *sorted.add(i as usize);
	}
	(*s).mean = sum / n as c_double;

	for i in 0..n {
		let d = *sorted.add(i as usize) - (*s).mean;

		var_sum += d * d;
	}
	(*s).stddev = if n > 1 {
		sqrt(var_sum / (n - 1) as c_double)
	} else {
		0.0
	};
}

#[no_mangle]
pub unsafe extern "C" fn bpf_bench_timing_measure(
	t: *mut bpf_bench_timing,
	_res: *mut bench_res,
) {
	let mut nr_cpus: c_uint;
	let mut total_samples: __u32;
	let mut i: c_int;

	(*t).warmup_ticks += 1;

	if (*t).warmup_ticks < env.warmup_sec {
		return;
	}

	if (*t).warmup_ticks == env.warmup_sec {
		*(*t).timing_enabled = 1;
		return;
	}

	nr_cpus = bpf_num_possible_cpus();
	if nr_cpus > BENCH_NR_CPUS as c_uint {
		nr_cpus = BENCH_NR_CPUS as c_uint;
	}

	total_samples = 0;
	i = 0;
	while i < nr_cpus as c_int {
		let mut cnt: __u32 = (*t).idx[i as usize];

		if cnt > BENCH_NR_SAMPLES as __u32 {
			cnt = BENCH_NR_SAMPLES as __u32;
		}
		total_samples = total_samples.wrapping_add(cnt);
		i += 1;
	}

	if total_samples >= (env.producer_cnt as __u32).wrapping_mul((*t).target_samples) && !(*t).done {
		(*t).done = true;
		*(*t).timing_enabled = 0;
		bench_force_done();
	}
}

#[no_mangle]
pub unsafe extern "C" fn bpf_bench_timing_report(
	t: *mut bpf_bench_timing,
	name: *const c_char,
	_description: *const c_char,
) {
	let max_out: c_int = (BENCH_NR_CPUS * BENCH_NR_SAMPLES) as c_int;
	let mut s: timing_stats = core::mem::zeroed();
	let all: *mut c_double;
	let mut total: c_int;

	all = calloc(max_out as usize, core::mem::size_of::<c_double>()) as *mut c_double;
	if all.is_null() {
		fprintf(stderr, c"failed to allocate timing buffer\n".as_ptr());
		return;
	}

	total = collect_samples(t, all, max_out);

	if total == 0 {
		printf(c"No timing samples collected.\n".as_ptr());
		free(all as *mut c_void);
		return;
	}

	total = filter_outliers_iqr(all, total);
	compute_stats(all, total, &mut s);

	if (*t).machine_readable {
		printf(
			c"RESULT scenario=%s samples=%d median=%.2f stddev=%.2f cv=%.2f min=%.2f p99=%.2f max=%.2f\n".as_ptr(),
			name,
			total,
			s.median,
			s.stddev,
			if s.mean > 0.0 { s.stddev / s.mean * 100.0 } else { 0.0 },
			s.min,
			s.p99,
			s.max,
		);
	} else {
		printf(
			c"%s: median %.2f ns/op, stddev %.2f, p99 %.2f (%d samples)\n".as_ptr(),
			name,
			s.median,
			s.stddev,
			s.p99,
			total,
		);
	}

	free(all as *mut c_void);
}

const CALIBRATE_SEED_BATCH: __u32 = 100;
const CALIBRATE_MIN_BATCH: __u32 = 100;
const CALIBRATE_MAX_BATCH: __u32 = 10000000;
const CALIBRATE_TARGET_MS: __u64 = 10;
const CALIBRATE_RUNS: c_int = 5;
const PROPORTIONALITY_TOL: c_double = 0.05; /* 5% */

unsafe fn reset_timing(t: *mut bpf_bench_timing) {
	*(*t).timing_enabled = 0;
	memset(
		(*t).samples.as_mut_ptr() as *mut c_void,
		0,
		core::mem::size_of::<__u64>() * BENCH_NR_CPUS * BENCH_NR_SAMPLES,
	);
	memset(
		(*t).idx.as_mut_ptr() as *mut c_void,
		0,
		core::mem::size_of::<__u32>() * BENCH_NR_CPUS,
	);
}

unsafe fn measure_elapsed(
	t: *mut bpf_bench_timing,
	run_fn: bpf_bench_run_fn,
	run_ctx: *mut c_void,
	iters: __u32,
	runs: c_int,
) -> __u64 {
	let mut buf: [__u64; CALIBRATE_RUNS as usize] = [0; CALIBRATE_RUNS as usize];
	let mut n: c_int = 0;
	let mut i: c_int;
	let mut j: c_int;

	reset_timing(t);
	*(*t).batch_iters_bss = iters;
	*(*t).timing_enabled = 1;

	i = 0;
	while i < runs {
		if let Some(run_fn) = run_fn {
			run_fn(run_ctx);
		}
		i += 1;
	}

	*(*t).timing_enabled = 0;

	i = 0;
	while i < BENCH_NR_CPUS as c_int && n < runs {
		let cnt: __u32 = (*t).idx[i as usize];

		j = 0;
		while j < cnt as c_int && n < runs {
			buf[n as usize] = (*t).samples[i as usize][j as usize];
			n += 1;
			j += 1;
		}
		i += 1;
	}

	if n == 0 {
		return 0;
	}

	i = 1;
	while i < n {
		let key: __u64 = buf[i as usize];

		j = i - 1;
		while j >= 0 && buf[j as usize] > key {
			buf[(j + 1) as usize] = buf[j as usize];
			j -= 1;
		}
		buf[(j + 1) as usize] = key;
		i += 1;
	}

	buf[(n / 2) as usize]
}

unsafe fn compute_batch_iters(per_op_ns: __u64) -> __u32 {
	let target_ns: __u64 = CALIBRATE_TARGET_MS * 1000000u64;
	let mut iters: __u32;

	if per_op_ns == 0 {
		return CALIBRATE_MIN_BATCH;
	}

	iters = (target_ns / per_op_ns) as __u32;

	if iters < CALIBRATE_MIN_BATCH {
		iters = CALIBRATE_MIN_BATCH;
	}
	if iters > CALIBRATE_MAX_BATCH {
		iters = CALIBRATE_MAX_BATCH;
	}

	iters
}

#[no_mangle]
pub unsafe extern "C" fn bpf_bench_calibrate(
	t: *mut bpf_bench_timing,
	run_fn: bpf_bench_run_fn,
	run_ctx: *mut c_void,
) {
	let elapsed: __u64;
	let per_op_ns: __u64;
	let time_n: __u64;
	let time_2n: __u64;
	let ratio: c_double;

	elapsed = measure_elapsed(t, run_fn, run_ctx, CALIBRATE_SEED_BATCH, CALIBRATE_RUNS);
	if elapsed == 0 {
		fprintf(stderr, c"calibration: no timing samples, using default\n".as_ptr());
		(*t).batch_iters = 10000;
		*(*t).batch_iters_bss = (*t).batch_iters;
		reset_timing(t);
		return;
	}

	per_op_ns = elapsed / CALIBRATE_SEED_BATCH as __u64;
	(*t).batch_iters = compute_batch_iters(per_op_ns);

	time_n = measure_elapsed(t, run_fn, run_ctx, (*t).batch_iters, CALIBRATE_RUNS);
	time_2n = measure_elapsed(t, run_fn, run_ctx, (*t).batch_iters.wrapping_mul(2), CALIBRATE_RUNS);

	if time_n > 0 && time_2n > 0 {
		ratio = time_2n as c_double / time_n as c_double;

		if fabs(ratio - 2.0) / 2.0 > PROPORTIONALITY_TOL {
			fprintf(
				stderr,
				c"WARNING: proportionality check failed (2N/N ratio=%.3f, expected=2.000, error=%.1f%%)\n  System noise may be affecting results.\n".as_ptr(),
				ratio,
				fabs(ratio - 2.0) / 2.0 * 100.0,
			);
		}
	}

	*(*t).batch_iters_bss = (*t).batch_iters;
	reset_timing(t);
}
