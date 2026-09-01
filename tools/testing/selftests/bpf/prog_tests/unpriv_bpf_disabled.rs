// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022, Oracle and/or its affiliates. */

// Dependencies from the original C includes:
// test_progs.h, bpf/btf.h, test_unpriv_bpf_disabled.skel.h,
// cap_helpers.h, bpf_util.h, and sysctl_helpers.h.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type size_t = usize;

const CAP_BPF: c_int = 39;

/* Using CAP_LAST_CAP is risky here, since it can get pulled in from
 * an old /usr/include/linux/capability.h and be < CAP_BPF; as a result
 * CAP_BPF would not be included in ALL_CAPS.  Instead use CAP_BPF as
 * we know its value is correct since it is explicitly defined in
 * cap_helpers.h.
 */
const ALL_CAPS: __u64 = (2u64 << CAP_BPF) - 1;

const PINPATH: &[u8] = b"/sys/fs/bpf/unpriv_bpf_disabled_\0";
const NUM_MAPS: usize = 7;

const EPERM: c_int = 1;
const BPF_PROG_TYPE_SOCKET_FILTER: c_int = 1;
const BPF_MAP_TYPE_HASH: c_int = 1;
const BPF_MAP_TYPE_ARRAY: c_int = 2;
const BPF_TRACE_FENTRY: c_int = 26;
const BPF_PERF_EVENT: c_int = 7;
const PERF_TYPE_SOFTWARE: __u32 = 1;
const PERF_COUNT_SW_CPU_CLOCK: __u64 = 0;
const PERF_FLAG_FD_CLOEXEC: __u64 = 8;
const __NR_perf_event_open: c_long = 298;
const BPF_REG_0: c_int = 0;

static mut got_perfbuf_val: __u32 = 0;
static mut got_ringbuf_val: __u32 = 0;

#[repr(C)]
struct perf_buffer {
	_private: [u8; 0],
}

#[repr(C)]
struct ring_buffer {
	_private: [u8; 0],
}

#[repr(C)]
struct btf {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_insn {
	code: u8,
	dst_src: u8,
	off: i16,
	imm: i32,
}

#[repr(C)]
struct bpf_prog_load_opts {
	sz: size_t,
}

#[repr(C)]
struct bpf_map_info {
	id: __u32,
}

#[repr(C)]
struct bpf_link_info {
	id: __u32,
}

#[repr(C)]
struct bpf_prog_info {
	id: __u32,
}

#[repr(C)]
struct perf_event_attr {
	type_: __u32,
	size: __u32,
	config: __u64,
	freq: __u64,
	sample_freq: __u64,
}

#[repr(C)]
struct test_unpriv_bpf_disabled_bss {
	perfbuf_val: __u32,
	ringbuf_val: __u32,
	test_pid: c_int,
}

#[repr(C)]
struct test_unpriv_bpf_disabled_maps {
	array: *mut bpf_map,
	percpu_array: *mut bpf_map,
	hash: *mut bpf_map,
	percpu_hash: *mut bpf_map,
	perfbuf: *mut bpf_map,
	ringbuf: *mut bpf_map,
	prog_array: *mut bpf_map,
}

#[repr(C)]
struct test_unpriv_bpf_disabled_progs {
	sys_nanosleep_enter: *mut bpf_program,
	handle_perf_event: *mut bpf_program,
}

#[repr(C)]
struct test_unpriv_bpf_disabled_links {
	sys_nanosleep_enter: *mut bpf_link,
}

#[repr(C)]
struct test_unpriv_bpf_disabled {
	bss: *mut test_unpriv_bpf_disabled_bss,
	maps: test_unpriv_bpf_disabled_maps,
	progs: test_unpriv_bpf_disabled_progs,
	links: test_unpriv_bpf_disabled_links,
}

extern "C" {
	fn bpf_num_possible_cpus() -> c_int;
	fn perf_buffer__new(
		map_fd: c_int,
		page_cnt: size_t,
		sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, __u32)>,
		lost_cb: *mut c_void,
		ctx: *mut c_void,
		opts: *const c_void,
	) -> *mut perf_buffer;
	fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
	fn perf_buffer__free(pb: *mut perf_buffer);
	fn ring_buffer__new(
		map_fd: c_int,
		sample_cb: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, size_t) -> c_int>,
		ctx: *mut c_void,
		opts: *const c_void,
	) -> *mut ring_buffer;
	fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int;
	fn ring_buffer__free(rb: *mut ring_buffer);
	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_link__fd(link: *mut bpf_link) -> c_int;
	fn bpf_obj_get(pathname: *const c_char) -> c_int;
	fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> c_int;
	fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
	fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
	fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
	fn bpf_link_create(prog_fd: c_int, target_fd: c_int, attach_type: c_int, opts: *const c_void) -> c_int;
	fn bpf_prog_load(
		prog_type: c_int,
		prog_name: *const c_char,
		license: *const c_char,
		insns: *const bpf_insn,
		insn_cnt: size_t,
		opts: *const bpf_prog_load_opts,
	) -> c_int;
	fn bpf_map_create(
		map_type: c_int,
		map_name: *const c_char,
		key_size: __u32,
		value_size: __u32,
		max_entries: __u32,
		opts: *const c_void,
	) -> c_int;
	fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
	fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
	fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> c_int;
	fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
	fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
	fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, info_len: *mut __u32) -> c_int;
	fn bpf_link_get_fd_by_id(id: __u32) -> c_int;
	fn bpf_link_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
	fn bpf_prog_query(
		target_fd: c_int,
		attach_type: c_int,
		query_flags: __u32,
		attach_flags: *mut __u32,
		prog_ids: *mut __u32,
		prog_cnt: *mut __u32,
	) -> c_int;
	fn btf__new_empty() -> *mut btf;
	fn btf__add_int(btf: *mut btf, name: *const c_char, size: __u32, encoding: __u32) -> c_int;
	fn btf__raw_data(btf: *mut btf, size: *mut __u32) -> *const c_void;
	fn bpf_btf_load(data: *const c_void, size: __u32, opts: *const c_void) -> c_int;
	fn btf__free(btf: *mut btf);
	fn test_unpriv_bpf_disabled__open_and_load() -> *mut test_unpriv_bpf_disabled;
	fn test_unpriv_bpf_disabled__attach(skel: *mut test_unpriv_bpf_disabled) -> c_int;
	fn test_unpriv_bpf_disabled__destroy(skel: *mut test_unpriv_bpf_disabled);
	fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
	fn sysctl_set(path: *const c_char, oldval: *mut c_char, newval: *const c_char) -> c_int;
	fn cap_disable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
	fn cap_enable_effective(caps: __u64, old_caps: *mut c_void) -> c_int;
	fn test__start_subtest(name: *const c_char) -> bool;
	fn getpid() -> c_int;
	fn usleep(usec: __u32) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn unlink(pathname: *const c_char) -> c_int;
	fn strlen(s: *const c_char) -> size_t;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn syscall(num: c_long, ...) -> c_long;
}

macro_rules! ASSERT_EQ {
	($actual:expr, $expected:expr, $name:expr) => {
		($actual) == ($expected)
	};
}

macro_rules! ASSERT_GT {
	($actual:expr, $expected:expr, $name:expr) => {
		($actual) > ($expected)
	};
}

macro_rules! ASSERT_GE {
	($actual:expr, $expected:expr, $name:expr) => {
		($actual) >= ($expected)
	};
}

macro_rules! ASSERT_OK {
	($actual:expr, $name:expr) => {
		($actual) == 0
	};
}

macro_rules! ASSERT_OK_PTR {
	($actual:expr, $name:expr) => {
		!($actual).is_null()
	};
}

const fn BPF_MOV64_IMM(dst: c_int, imm: i32) -> bpf_insn {
	bpf_insn {
		code: 0xb7,
		dst_src: dst as u8,
		off: 0,
		imm,
	}
}

const fn BPF_EXIT_INSN() -> bpf_insn {
	bpf_insn {
		code: 0x95,
		dst_src: 0,
		off: 0,
		imm: 0,
	}
}

unsafe extern "C" fn process_ringbuf(_ctx: *mut c_void, data: *mut c_void, len: size_t) -> c_int {
	if ASSERT_EQ!(len, size_of::<__u32>(), c"ringbuf_size_valid".as_ptr()) {
		got_ringbuf_val = *(data as *mut __u32);
	}
	0
}

unsafe extern "C" fn process_perfbuf(_ctx: *mut c_void, _cpu: c_int, data: *mut c_void, len: __u32) {
	if ASSERT_EQ!(len as size_t, size_of::<__u32>(), c"perfbuf_size_valid".as_ptr()) {
		got_perfbuf_val = *(data as *mut __u32);
	}
}

unsafe fn test_unpriv_bpf_disabled_positive(
	skel: *mut test_unpriv_bpf_disabled,
	prog_id: __u32,
	prog_fd: c_int,
	perf_fd: c_int,
	map_paths: *mut *mut c_char,
	map_fds: *mut c_int,
) {
	let mut perfbuf: *mut perf_buffer = ptr::null_mut();
	let mut ringbuf: *mut ring_buffer = ptr::null_mut();
	let mut link_fd: c_int = -1;

	let nr_cpus = bpf_num_possible_cpus();

	(*(*skel).bss).perfbuf_val = 1;
	(*(*skel).bss).ringbuf_val = 2;

	/* Positive tests for unprivileged BPF disabled. Verify we can
	 * - retrieve and interact with pinned maps;
	 * - set up and interact with perf buffer;
	 * - set up and interact with ring buffer;
	 * - create a link
	 */
	perfbuf = perf_buffer__new(
		bpf_map__fd((*skel).maps.perfbuf),
		8,
		Some(process_perfbuf),
		ptr::null_mut(),
		ptr::null_mut(),
		ptr::null(),
	);
	if !ASSERT_OK_PTR!(perfbuf, c"perf_buffer__new".as_ptr()) {
		goto_cleanup_positive(perfbuf, ringbuf, link_fd);
		return;
	}

	ringbuf = ring_buffer__new(
		bpf_map__fd((*skel).maps.ringbuf),
		Some(process_ringbuf),
		ptr::null_mut(),
		ptr::null(),
	);
	if !ASSERT_OK_PTR!(ringbuf, c"ring_buffer__new".as_ptr()) {
		goto_cleanup_positive(perfbuf, ringbuf, link_fd);
		return;
	}

	/* trigger & validate perf event, ringbuf output */
	usleep(1);

	ASSERT_GT!(perf_buffer__poll(perfbuf, 100), -1, c"perf_buffer__poll".as_ptr());
	ASSERT_EQ!(got_perfbuf_val, (*(*skel).bss).perfbuf_val, c"check_perfbuf_val".as_ptr());
	ASSERT_EQ!(ring_buffer__consume(ringbuf), 1, c"ring_buffer__consume".as_ptr());
	ASSERT_EQ!(got_ringbuf_val, (*(*skel).bss).ringbuf_val, c"check_ringbuf_val".as_ptr());

	for i in 0..NUM_MAPS {
		*map_fds.add(i) = bpf_obj_get(*map_paths.add(i));
		if !ASSERT_GT!(*map_fds.add(i), -1, c"obj_get".as_ptr()) {
			goto_cleanup_positive(perfbuf, ringbuf, link_fd);
			return;
		}
	}

	for i in 0..NUM_MAPS {
		let prog_array = !strstr(*map_paths.add(i), c"prog_array".as_ptr()).is_null();
		let array = !strstr(*map_paths.add(i), c"array".as_ptr()).is_null();
		let buf = !strstr(*map_paths.add(i), c"buf".as_ptr()).is_null();
		let mut key: __u32 = 0;
		let mut vals = vec![0u32; nr_cpus as usize];
		let mut lookup_vals = vec![0u32; nr_cpus as usize];
		let mut expected_val: __u32 = 1;

		/* skip ringbuf, perfbuf */
		if buf {
			continue;
		}

		for j in 0..nr_cpus as usize {
			vals[j] = expected_val;
		}

		if prog_array {
			/* need valid prog array value */
			vals[0] = prog_fd as __u32;
			/* prog array lookup returns prog id, not fd */
			expected_val = prog_id;
		}
		ASSERT_OK!(
			bpf_map_update_elem(*map_fds.add(i), &mut key as *mut _ as *const c_void, vals.as_ptr() as *const c_void, 0),
			c"map_update_elem".as_ptr()
		);
		ASSERT_OK!(
			bpf_map_lookup_elem(*map_fds.add(i), &mut key as *mut _ as *const c_void, lookup_vals.as_mut_ptr() as *mut c_void),
			c"map_lookup_elem".as_ptr()
		);
		ASSERT_EQ!(lookup_vals[0], expected_val, c"map_lookup_elem_values".as_ptr());
		if !array {
			ASSERT_OK!(
				bpf_map_delete_elem(*map_fds.add(i), &mut key as *mut _ as *const c_void),
				c"map_delete_elem".as_ptr()
			);
		}
	}

	link_fd = bpf_link_create(
		bpf_program__fd((*skel).progs.handle_perf_event),
		perf_fd,
		BPF_PERF_EVENT,
		ptr::null(),
	);
	ASSERT_GT!(link_fd, 0, c"link_create".as_ptr());

	goto_cleanup_positive(perfbuf, ringbuf, link_fd);
}

unsafe fn goto_cleanup_positive(
	perfbuf: *mut perf_buffer,
	ringbuf: *mut ring_buffer,
	link_fd: c_int,
) {
	if link_fd != 0 {
		close(link_fd);
	}
	if !perfbuf.is_null() {
		perf_buffer__free(perfbuf);
	}
	if !ringbuf.is_null() {
		ring_buffer__free(ringbuf);
	}
}

unsafe fn test_unpriv_bpf_disabled_negative(
	skel: *mut test_unpriv_bpf_disabled,
	prog_id: __u32,
	prog_fd: c_int,
	_perf_fd: c_int,
	_map_paths: *mut *mut c_char,
	map_fds: *mut c_int,
) {
	let prog_insns = [
		BPF_MOV64_IMM(BPF_REG_0, 0),
		BPF_EXIT_INSN(),
	];
	let prog_insn_cnt: size_t = prog_insns.len();
	let load_opts = bpf_prog_load_opts {
		sz: size_of::<bpf_prog_load_opts>(),
	};
	let mut map_info: bpf_map_info = core::mem::zeroed();
	let mut map_info_len: __u32 = size_of::<bpf_map_info>() as __u32;
	let mut link_info: bpf_link_info = core::mem::zeroed();
	let mut link_info_len: __u32 = size_of::<bpf_link_info>() as __u32;
	let mut btf: *mut btf = ptr::null_mut();
	let mut attach_flags: __u32 = 0;
	let mut prog_ids: [__u32; 3] = [0; 3];
	let mut prog_cnt: __u32 = 3;
	let mut next: __u32 = 0;

	/* Negative tests for unprivileged BPF disabled.  Verify we cannot
	 * - load BPF programs;
	 * - create BPF maps;
	 * - get a prog/map/link fd by id;
	 * - get next prog/map/link id
	 * - query prog
	 * - BTF load
	 */
	ASSERT_EQ!(
		bpf_prog_load(
			BPF_PROG_TYPE_SOCKET_FILTER,
			c"simple_prog".as_ptr(),
			c"GPL".as_ptr(),
			prog_insns.as_ptr(),
			prog_insn_cnt,
			&load_opts,
		),
		-EPERM,
		c"prog_load_fails".as_ptr()
	);

	/* some map types require particular correct parameters which could be
	 * sanity-checked before enforcing -EPERM, so only validate that
	 * the simple ARRAY and HASH maps are failing with -EPERM
	 */
	for i in BPF_MAP_TYPE_HASH..=BPF_MAP_TYPE_ARRAY {
		ASSERT_EQ!(
			bpf_map_create(i, ptr::null(), size_of::<c_int>() as __u32, size_of::<c_int>() as __u32, 1, ptr::null()),
			-EPERM,
			c"map_create_fails".as_ptr()
		);
	}

	ASSERT_EQ!(bpf_prog_get_fd_by_id(prog_id), -EPERM, c"prog_get_fd_by_id_fails".as_ptr());
	ASSERT_EQ!(bpf_prog_get_next_id(prog_id, &mut next), -EPERM, c"prog_get_next_id_fails".as_ptr());
	ASSERT_EQ!(bpf_prog_get_next_id(0, &mut next), -EPERM, c"prog_get_next_id_fails".as_ptr());

	if ASSERT_OK!(
		bpf_map_get_info_by_fd(*map_fds.add(0), &mut map_info, &mut map_info_len),
		c"obj_get_info_by_fd".as_ptr()
	) {
		ASSERT_EQ!(bpf_map_get_fd_by_id(map_info.id), -EPERM, c"map_get_fd_by_id_fails".as_ptr());
		ASSERT_EQ!(
			bpf_map_get_next_id(map_info.id, &mut next),
			-EPERM,
			c"map_get_next_id_fails".as_ptr()
		);
	}
	ASSERT_EQ!(bpf_map_get_next_id(0, &mut next), -EPERM, c"map_get_next_id_fails".as_ptr());

	if ASSERT_OK!(
		bpf_link_get_info_by_fd(bpf_link__fd((*skel).links.sys_nanosleep_enter), &mut link_info, &mut link_info_len),
		c"obj_get_info_by_fd".as_ptr()
	) {
		ASSERT_EQ!(bpf_link_get_fd_by_id(link_info.id), -EPERM, c"link_get_fd_by_id_fails".as_ptr());
		ASSERT_EQ!(
			bpf_link_get_next_id(link_info.id, &mut next),
			-EPERM,
			c"link_get_next_id_fails".as_ptr()
		);
	}
	ASSERT_EQ!(bpf_link_get_next_id(0, &mut next), -EPERM, c"link_get_next_id_fails".as_ptr());

	ASSERT_EQ!(
		bpf_prog_query(
			prog_fd,
			BPF_TRACE_FENTRY,
			0,
			&mut attach_flags,
			prog_ids.as_mut_ptr(),
			&mut prog_cnt,
		),
		-EPERM,
		c"prog_query_fails".as_ptr()
	);

	btf = btf__new_empty();
	if ASSERT_OK_PTR!(btf, c"empty_btf".as_ptr())
		&& ASSERT_GT!(btf__add_int(btf, c"int".as_ptr(), 4, 0), 0, c"unpriv_int_type".as_ptr())
	{
		let mut raw_btf_size: __u32 = 0;

		let raw_btf_data = btf__raw_data(btf, &mut raw_btf_size);
		if ASSERT_OK_PTR!(raw_btf_data, c"raw_btf_data_good".as_ptr()) {
			ASSERT_EQ!(
				bpf_btf_load(raw_btf_data, raw_btf_size, ptr::null()),
				-EPERM,
				c"bpf_btf_load_fails".as_ptr()
			);
		}
	}
	btf__free(btf);
}

#[no_mangle]
pub unsafe extern "C" fn test_unpriv_bpf_disabled() {
	let mut map_paths: [*mut c_char; NUM_MAPS] = [
		c"/sys/fs/bpf/unpriv_bpf_disabled_array".as_ptr() as *mut c_char,
		c"/sys/fs/bpf/unpriv_bpf_disabled_percpu_array".as_ptr() as *mut c_char,
		c"/sys/fs/bpf/unpriv_bpf_disabled_hash".as_ptr() as *mut c_char,
		c"/sys/fs/bpf/unpriv_bpf_disabled_percpu_hash".as_ptr() as *mut c_char,
		c"/sys/fs/bpf/unpriv_bpf_disabled_perfbuf".as_ptr() as *mut c_char,
		c"/sys/fs/bpf/unpriv_bpf_disabled_ringbuf".as_ptr() as *mut c_char,
		c"/sys/fs/bpf/unpriv_bpf_disabled_prog_array".as_ptr() as *mut c_char,
	];
	let mut map_fds: [c_int; NUM_MAPS] = [0; NUM_MAPS];
	let mut unprivileged_bpf_disabled_orig: [c_char; 32] = [0; 32];
	let mut perf_event_paranoid_orig: [c_char; 32] = [0; 32];
	let mut prog_info: bpf_prog_info = core::mem::zeroed();
	let mut prog_info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
	let mut attr: perf_event_attr = core::mem::zeroed();
	let mut perf_fd: c_int = -1;
	let mut ret: c_int;
	let mut save_caps: __u64 = 0;

	let skel = test_unpriv_bpf_disabled__open_and_load();
	if !ASSERT_OK_PTR!(skel, c"skel_open".as_ptr()) {
		return;
	}

	(*(*skel).bss).test_pid = getpid();

	map_fds[0] = bpf_map__fd((*skel).maps.array);
	map_fds[1] = bpf_map__fd((*skel).maps.percpu_array);
	map_fds[2] = bpf_map__fd((*skel).maps.hash);
	map_fds[3] = bpf_map__fd((*skel).maps.percpu_hash);
	map_fds[4] = bpf_map__fd((*skel).maps.perfbuf);
	map_fds[5] = bpf_map__fd((*skel).maps.ringbuf);
	map_fds[6] = bpf_map__fd((*skel).maps.prog_array);

	for i in 0..NUM_MAPS {
		ASSERT_OK!(bpf_obj_pin(map_fds[i], map_paths[i]), c"pin map_fd".as_ptr());
	}

	/* allow user without caps to use perf events */
	if !ASSERT_OK!(
		sysctl_set(
			c"/proc/sys/kernel/perf_event_paranoid".as_ptr(),
			perf_event_paranoid_orig.as_mut_ptr(),
			c"-1".as_ptr(),
		),
		c"set_perf_event_paranoid".as_ptr()
	) {
		goto_cleanup_main(perf_fd, save_caps, perf_event_paranoid_orig.as_mut_ptr(), unprivileged_bpf_disabled_orig.as_mut_ptr(), map_paths.as_mut_ptr(), skel);
		return;
	}
	/* ensure unprivileged bpf disabled is set */
	ret = sysctl_set(
		c"/proc/sys/kernel/unprivileged_bpf_disabled".as_ptr(),
		unprivileged_bpf_disabled_orig.as_mut_ptr(),
		c"2".as_ptr(),
	);
	if ret == -EPERM {
		/* if unprivileged_bpf_disabled=1, we get -EPERM back; that's okay. */
		if !ASSERT_OK!(
			strcmp(unprivileged_bpf_disabled_orig.as_ptr(), c"1".as_ptr()),
			c"unprivileged_bpf_disabled_on".as_ptr()
		) {
			goto_cleanup_main(perf_fd, save_caps, perf_event_paranoid_orig.as_mut_ptr(), unprivileged_bpf_disabled_orig.as_mut_ptr(), map_paths.as_mut_ptr(), skel);
			return;
		}
	} else if !ASSERT_OK!(ret, c"set unprivileged_bpf_disabled".as_ptr()) {
		goto_cleanup_main(perf_fd, save_caps, perf_event_paranoid_orig.as_mut_ptr(), unprivileged_bpf_disabled_orig.as_mut_ptr(), map_paths.as_mut_ptr(), skel);
		return;
	}

	let prog_fd = bpf_program__fd((*skel).progs.sys_nanosleep_enter);
	ASSERT_OK!(
		bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut prog_info_len),
		c"obj_get_info_by_fd".as_ptr()
	);
	let prog_id = prog_info.id;
	ASSERT_GT!(prog_id, 0, c"valid_prog_id".as_ptr());

	attr.size = size_of::<perf_event_attr>() as __u32;
	attr.type_ = PERF_TYPE_SOFTWARE;
	attr.config = PERF_COUNT_SW_CPU_CLOCK;
	attr.freq = 1;
	attr.sample_freq = 1000;
	perf_fd = syscall(
		__NR_perf_event_open,
		&mut attr as *mut perf_event_attr,
		-1,
		0,
		-1,
		PERF_FLAG_FD_CLOEXEC,
	) as c_int;
	if !ASSERT_GE!(perf_fd, 0, c"perf_fd".as_ptr()) {
		goto_cleanup_main(perf_fd, save_caps, perf_event_paranoid_orig.as_mut_ptr(), unprivileged_bpf_disabled_orig.as_mut_ptr(), map_paths.as_mut_ptr(), skel);
		return;
	}

	if !ASSERT_OK!(test_unpriv_bpf_disabled__attach(skel), c"skel_attach".as_ptr()) {
		goto_cleanup_main(perf_fd, save_caps, perf_event_paranoid_orig.as_mut_ptr(), unprivileged_bpf_disabled_orig.as_mut_ptr(), map_paths.as_mut_ptr(), skel);
		return;
	}

	if !ASSERT_OK!(cap_disable_effective(ALL_CAPS, &mut save_caps), c"disable caps".as_ptr()) {
		goto_cleanup_main(perf_fd, save_caps, perf_event_paranoid_orig.as_mut_ptr(), unprivileged_bpf_disabled_orig.as_mut_ptr(), map_paths.as_mut_ptr(), skel);
		return;
	}

	if test__start_subtest(c"unpriv_bpf_disabled_positive".as_ptr()) {
		test_unpriv_bpf_disabled_positive(
			skel,
			prog_id,
			prog_fd,
			perf_fd,
			map_paths.as_mut_ptr(),
			map_fds.as_mut_ptr(),
		);
	}

	if test__start_subtest(c"unpriv_bpf_disabled_negative".as_ptr()) {
		test_unpriv_bpf_disabled_negative(
			skel,
			prog_id,
			prog_fd,
			perf_fd,
			map_paths.as_mut_ptr(),
			map_fds.as_mut_ptr(),
		);
	}

	goto_cleanup_main(perf_fd, save_caps, perf_event_paranoid_orig.as_mut_ptr(), unprivileged_bpf_disabled_orig.as_mut_ptr(), map_paths.as_mut_ptr(), skel);
}

unsafe fn goto_cleanup_main(
	perf_fd: c_int,
	save_caps: __u64,
	perf_event_paranoid_orig: *mut c_char,
	unprivileged_bpf_disabled_orig: *mut c_char,
	map_paths: *mut *mut c_char,
	skel: *mut test_unpriv_bpf_disabled,
) {
	close(perf_fd);
	if save_caps != 0 {
		cap_enable_effective(save_caps, ptr::null_mut());
	}
	if strlen(perf_event_paranoid_orig) > 0 {
		sysctl_set(
			c"/proc/sys/kernel/perf_event_paranoid".as_ptr(),
			ptr::null_mut(),
			perf_event_paranoid_orig,
		);
	}
	if strlen(unprivileged_bpf_disabled_orig) > 0 {
		sysctl_set(
			c"/proc/sys/kernel/unprivileged_bpf_disabled".as_ptr(),
			ptr::null_mut(),
			unprivileged_bpf_disabled_orig,
		);
	}
	for i in 0..NUM_MAPS {
		unlink(*map_paths.add(i));
	}
	test_unpriv_bpf_disabled__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
