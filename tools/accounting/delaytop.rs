// SPDX-License-Identifier: GPL-2.0
/*
 * delaytop.c - system-wide delay monitoring tool.
 *
 * This tool provides real-time monitoring and statistics of
 * system, container, and task-level delays, including CPU,
 * memory, IO, and IRQ. It supports both interactive (top-like),
 * and can output delay information for the whole system, specific
 * containers (cgroups), or individual tasks (PIDs).
 *
 * Key features:
 *	- Collects per-task delay accounting statistics via taskstats.
 *	- Collects system-wide PSI information.
 *	- Supports sorting, filtering.
 *	- Supports both interactive (screen refresh).
 *
 * Copyright (C) Fan Yu, ZTE Corp. 2025
 * Copyright (C) Wang Yaxin, ZTE Corp. 2025
 *
 * Compile with
 *	gcc -I/usr/src/linux/include delaytop.c -o delaytop
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
use std::ptr;

const PSI_PATH: &[u8] = b"/proc/pressure\0";
const PSI_CPU_PATH: &[u8] = b"/proc/pressure/cpu\0";
const PSI_MEMORY_PATH: &[u8] = b"/proc/pressure/memory\0";
const PSI_IO_PATH: &[u8] = b"/proc/pressure/io\0";
const PSI_IRQ_PATH: &[u8] = b"/proc/pressure/irq\0";

const TASK_COMM_LEN: usize = 16;
const MAX_MSG_SIZE: usize = 1024;
const MAX_TASKS: usize = 1000;
const MAX_BUF_LEN: usize = 256;
const MAX_MODE_SIZE: usize = 2;

const MODE_TYPE_ALL: usize = 0xFFFF_FFFF;
const MODE_DEFAULT: usize = 1 << 0;
const MODE_MEMVERBOSE: usize = 1 << 1;

const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const NETLINK_GENERIC: c_int = 16;
const NLM_F_REQUEST: c_ushort = 1;
const GENL_ID_CTRL: c_ushort = 0x10;
const CTRL_CMD_GETFAMILY: u8 = 3;
const CTRL_ATTR_FAMILY_ID: c_ushort = 1;
const CTRL_ATTR_FAMILY_NAME: c_ushort = 2;
const TASKSTATS_CMD_GET: u8 = 1;
const TASKSTATS_CMD_ATTR_PID: c_ushort = 1;
const TASKSTATS_TYPE_AGGR_PID: c_ushort = 4;
const TASKSTATS_TYPE_STATS: c_ushort = 3;
const CGROUPSTATS_CMD_GET: u8 = 3;
const CGROUPSTATS_CMD_ATTR_FD: c_ushort = 1;
const CGROUPSTATS_TYPE_CGROUP_STATS: c_ushort = 1;
const TASKSTATS_GENL_NAME: &[u8] = b"TASKSTATS\0";
const NLMSG_ERROR: c_ushort = 2;
const F_OK: c_int = 0;
const O_RDONLY: c_int = 0;
const STDIN_FILENO: c_int = 0;
const TCSAFLUSH: c_int = 2;
const ICANON: c_uint = 0o0000002;
const ECHO: c_uint = 0o0000010;
const EAGAIN: c_int = 11;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type time_t = c_long;

#[repr(C)]
struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
struct DIR {
	_private: [u8; 0],
}

#[repr(C)]
struct option {
	name: *const c_char,
	has_arg: c_int,
	flag: *mut c_int,
	val: c_int,
}

const no_argument: c_int = 0;
const required_argument: c_int = 1;

#[repr(C)]
struct sockaddr {
	sa_family: c_ushort,
	sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_nl {
	nl_family: c_ushort,
	nl_pad: c_ushort,
	nl_pid: u32,
	nl_groups: u32,
}

#[repr(C)]
struct nlmsghdr {
	nlmsg_len: u32,
	nlmsg_type: u16,
	nlmsg_flags: u16,
	nlmsg_seq: u32,
	nlmsg_pid: u32,
}

#[repr(C)]
struct genlmsghdr {
	cmd: u8,
	version: u8,
	reserved: u16,
}

#[repr(C)]
struct nlattr {
	nla_len: u16,
	nla_type: u16,
}

#[repr(C)]
struct termios {
	c_iflag: c_uint,
	c_oflag: c_uint,
	c_cflag: c_uint,
	c_lflag: c_uint,
	c_line: u8,
	c_cc: [u8; 32],
	c_ispeed: c_uint,
	c_ospeed: c_uint,
}

#[repr(C)]
struct timeval {
	tv_sec: c_long,
	tv_usec: c_long,
}

#[repr(C)]
struct fd_set {
	fds_bits: [c_long; 16],
}

#[repr(C)]
struct tm {
	tm_sec: c_int,
	tm_min: c_int,
	tm_hour: c_int,
	tm_mday: c_int,
	tm_mon: c_int,
	tm_year: c_int,
	tm_wday: c_int,
	tm_yday: c_int,
	tm_isdst: c_int,
}

#[repr(C)]
struct dirent {
	d_ino: u64,
	d_off: i64,
	d_reclen: u16,
	d_type: u8,
	d_name: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct taskstats {
	version: u16,
	ac_exitcode: u32,
	ac_flag: u8,
	ac_nice: u8,
	cpu_count: u64,
	cpu_delay_total: u64,
	blkio_count: u64,
	blkio_delay_total: u64,
	swapin_count: u64,
	swapin_delay_total: u64,
	cpu_run_real_total: u64,
	cpu_run_virtual_total: u64,
	ac_comm: [c_char; 32],
	ac_sched: u8,
	ac_pad: [u8; 3],
	ac_uid: u32,
	ac_gid: u32,
	ac_pid: u32,
	ac_ppid: u32,
	ac_btime: u32,
	ac_etime: u64,
	ac_utime: u64,
	ac_stime: u64,
	ac_minflt: u64,
	ac_majflt: u64,
	coremem: u64,
	virtmem: u64,
	hiwater_rss: u64,
	hiwater_vm: u64,
	read_char: u64,
	write_char: u64,
	read_syscalls: u64,
	write_syscalls: u64,
	read_bytes: u64,
	write_bytes: u64,
	cancelled_write_bytes: u64,
	nvcsw: u64,
	nivcsw: u64,
	ac_utimescaled: u64,
	ac_stimescaled: u64,
	cpu_scaled_run_real_total: u64,
	freepages_count: u64,
	freepages_delay_total: u64,
	thrashing_count: u64,
	thrashing_delay_total: u64,
	ac_btime64: u64,
	compact_count: u64,
	compact_delay_total: u64,
	wpcopy_count: u64,
	wpcopy_delay_total: u64,
	irq_count: u64,
	irq_delay_total: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cgroupstats {
	nr_sleeping: u64,
	nr_running: u64,
	nr_stopped: u64,
	nr_uninterruptible: u64,
	nr_io_wait: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct psi_stats {
	cpu_some_avg10: c_double,
	cpu_some_avg60: c_double,
	cpu_some_avg300: c_double,
	cpu_some_total: u64,
	cpu_full_avg10: c_double,
	cpu_full_avg60: c_double,
	cpu_full_avg300: c_double,
	cpu_full_total: u64,
	memory_some_avg10: c_double,
	memory_some_avg60: c_double,
	memory_some_avg300: c_double,
	memory_some_total: u64,
	memory_full_avg10: c_double,
	memory_full_avg60: c_double,
	memory_full_avg300: c_double,
	memory_full_total: u64,
	io_some_avg10: c_double,
	io_some_avg60: c_double,
	io_some_avg300: c_double,
	io_some_total: u64,
	io_full_avg10: c_double,
	io_full_avg60: c_double,
	io_full_avg300: c_double,
	io_full_total: u64,
	irq_full_avg10: c_double,
	irq_full_avg60: c_double,
	irq_full_avg300: c_double,
	irq_full_total: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct task_info {
	pid: c_int,
	tgid: c_int,
	command: [c_char; TASK_COMM_LEN],
	cpu_count: u64,
	cpu_delay_total: u64,
	blkio_count: u64,
	blkio_delay_total: u64,
	swapin_count: u64,
	swapin_delay_total: u64,
	freepages_count: u64,
	freepages_delay_total: u64,
	thrashing_count: u64,
	thrashing_delay_total: u64,
	compact_count: u64,
	compact_delay_total: u64,
	wpcopy_count: u64,
	wpcopy_delay_total: u64,
	irq_count: u64,
	irq_delay_total: u64,
	mem_count: u64,
	mem_delay_total: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct container_stats {
	nr_sleeping: c_int,
	nr_running: c_int,
	nr_stopped: c_int,
	nr_uninterruptible: c_int,
	nr_io_wait: c_int,
}

#[repr(C)]
struct field_desc {
	name: *const c_char,
	cmd_char: *const c_char,
	total_offset: c_ulong,
	count_offset: c_ulong,
	supported_modes: size_t,
}

#[repr(C)]
struct config {
	delay: c_int,
	iterations: c_int,
	max_processes: c_int,
	output_one_time: c_int,
	monitor_pid: c_int,
	container_path: *mut c_char,
	sort_field: *const field_desc,
	display_mode: size_t,
}

#[repr(C)]
struct nl_msg {
	n: nlmsghdr,
	g: genlmsghdr,
	buf: [c_char; MAX_MSG_SIZE],
}

extern "C" {
	static mut stdout: *mut FILE;
	static mut stderr: *mut FILE;
	static mut errno: c_int;
	static mut optarg: *mut c_char;

	fn printf(fmt: *const c_char, ...) -> c_int;
	fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
	fn perror(s: *const c_char);
	fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
	fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
	fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
	fn fclose(stream: *mut FILE) -> c_int;
	fn exit(status: c_int) -> !;
	fn atoi(nptr: *const c_char) -> c_int;
	fn strdup(s: *const c_char) -> *mut c_char;
	fn free(ptr: *mut c_void);
	fn strlen(s: *const c_char) -> size_t;
	fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
	fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
	fn strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
	fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
	fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
	fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
	fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
	fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn sendto(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int, dest_addr: *const sockaddr, addrlen: socklen_t) -> ssize_t;
	fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
	fn getpid() -> c_int;
	fn access(pathname: *const c_char, mode: c_int) -> c_int;
	fn opendir(name: *const c_char) -> *mut DIR;
	fn readdir(dirp: *mut DIR) -> *mut dirent;
	fn closedir(dirp: *mut DIR) -> c_int;
	fn isdigit(c: c_int) -> c_int;
	fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn time(tloc: *mut time_t) -> time_t;
	fn localtime(timep: *const time_t) -> *mut tm;
	fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *mut timeval) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
	fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
	fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
}

static mut cfg: config = config {
	delay: 0,
	iterations: 0,
	max_processes: 0,
	output_one_time: 0,
	monitor_pid: 0,
	container_path: ptr::null_mut(),
	sort_field: ptr::null(),
	display_mode: 0,
};
static mut psi: psi_stats = unsafe { mem::zeroed() };
static mut tasks: [task_info; MAX_TASKS] = unsafe { mem::zeroed() };
static mut task_count: c_int = 0;
static mut running: c_int = 1;
static mut container_stats: container_stats = container_stats {
	nr_sleeping: 0,
	nr_running: 0,
	nr_stopped: 0,
	nr_uninterruptible: 0,
	nr_io_wait: 0,
};

const CPU: &[u8] = b"cpu\0";
const C_CMD: &[u8] = b"c\0";
const BLKIO: &[u8] = b"blkio\0";
const I_CMD: &[u8] = b"i\0";
const IRQ: &[u8] = b"irq\0";
const Q_CMD: &[u8] = b"q\0";
const MEM: &[u8] = b"mem\0";
const M_CMD: &[u8] = b"m\0";
const SWAPIN: &[u8] = b"swapin\0";
const S_CMD: &[u8] = b"s\0";
const FREEPAGES: &[u8] = b"freepages\0";
const R_CMD: &[u8] = b"r\0";
const THRASHING: &[u8] = b"thrashing\0";
const T_CMD: &[u8] = b"t\0";
const COMPACT: &[u8] = b"compact\0";
const P_CMD: &[u8] = b"p\0";
const WPCOPY: &[u8] = b"wpcopy\0";
const W_CMD: &[u8] = b"w\0";
const UNKNOWN: &[u8] = b"UNKNOWN\0";

const fn offset_of_task_info(field: usize) -> c_ulong {
	field as c_ulong
}

static sort_fields: [field_desc; 10] = [
	field_desc { name: CPU.as_ptr() as *const c_char, cmd_char: C_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(24), count_offset: offset_of_task_info(16), supported_modes: MODE_DEFAULT },
	field_desc { name: BLKIO.as_ptr() as *const c_char, cmd_char: I_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(40), count_offset: offset_of_task_info(32), supported_modes: MODE_DEFAULT },
	field_desc { name: IRQ.as_ptr() as *const c_char, cmd_char: Q_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(136), count_offset: offset_of_task_info(128), supported_modes: MODE_DEFAULT },
	field_desc { name: MEM.as_ptr() as *const c_char, cmd_char: M_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(152), count_offset: offset_of_task_info(144), supported_modes: MODE_DEFAULT | MODE_MEMVERBOSE },
	field_desc { name: SWAPIN.as_ptr() as *const c_char, cmd_char: S_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(56), count_offset: offset_of_task_info(48), supported_modes: MODE_MEMVERBOSE },
	field_desc { name: FREEPAGES.as_ptr() as *const c_char, cmd_char: R_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(72), count_offset: offset_of_task_info(64), supported_modes: MODE_MEMVERBOSE },
	field_desc { name: THRASHING.as_ptr() as *const c_char, cmd_char: T_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(88), count_offset: offset_of_task_info(80), supported_modes: MODE_MEMVERBOSE },
	field_desc { name: COMPACT.as_ptr() as *const c_char, cmd_char: P_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(104), count_offset: offset_of_task_info(96), supported_modes: MODE_MEMVERBOSE },
	field_desc { name: WPCOPY.as_ptr() as *const c_char, cmd_char: W_CMD.as_ptr() as *const c_char, total_offset: offset_of_task_info(120), count_offset: offset_of_task_info(112), supported_modes: MODE_MEMVERBOSE },
	field_desc { name: ptr::null(), cmd_char: ptr::null(), total_offset: 0, count_offset: 0, supported_modes: 0 },
];
static mut sort_selected: c_int = 0;
static mut nl_sd: c_int = -1;
static mut family_id: c_int = 0;
static mut orig_termios: termios = unsafe { mem::zeroed() };

fn c_lit(bytes: &'static [u8]) -> *const c_char {
	bytes.as_ptr() as *const c_char
}

fn NLMSG_ALIGN(len: u32) -> u32 {
	(len + 3) & !3
}

fn NLA_ALIGN(len: u16) -> u32 {
	((len as u32) + 3) & !3
}

fn NLMSG_LENGTH(len: u32) -> u32 {
	(mem::size_of::<nlmsghdr>() as u32) + len
}

unsafe fn NLMSG_DATA(nlh: *const nlmsghdr) -> *mut c_void {
	(nlh as *mut u8).add(NLMSG_LENGTH(0) as usize) as *mut c_void
}

fn NLMSG_PAYLOAD(nlh: *const nlmsghdr, len: u32) -> c_int {
	unsafe { ((*nlh).nlmsg_len - NLMSG_LENGTH(len)) as c_int }
}

unsafe fn NLMSG_OK(nlh: *const nlmsghdr, len: c_int) -> bool {
	len >= mem::size_of::<nlmsghdr>() as c_int
		&& (*nlh).nlmsg_len >= mem::size_of::<nlmsghdr>() as u32
		&& (*nlh).nlmsg_len as c_int <= len
}

unsafe fn NLA_NEXT(na: *mut nlattr) -> *mut nlattr {
	(na as *mut u8).add(NLA_ALIGN((*na).nla_len) as usize) as *mut nlattr
}

unsafe fn NLA_DATA(na: *mut nlattr) -> *mut c_void {
	(na as *mut u8).add(mem::size_of::<nlattr>()) as *mut c_void
}

fn NLA_PAYLOAD(len: u16) -> c_int {
	(len as c_int) - mem::size_of::<nlattr>() as c_int
}

unsafe fn GENLMSG_DATA(glh: *const nlmsghdr) -> *mut c_void {
	(NLMSG_DATA(glh) as *mut u8).add(mem::size_of::<genlmsghdr>()) as *mut c_void
}

fn GENLMSG_PAYLOAD(glh: *const nlmsghdr) -> c_int {
	NLMSG_PAYLOAD(glh, 0) - mem::size_of::<genlmsghdr>() as c_int
}

unsafe fn BOOL_FPRINT(stream: *mut FILE, fmt: *const c_char) -> bool {
	fprintf(stream, fmt) >= 0
}

unsafe fn fd_zero(set: *mut fd_set) {
	for i in 0..(*set).fds_bits.len() {
		(*set).fds_bits[i] = 0;
	}
}

unsafe fn fd_set_fd(fd: c_int, set: *mut fd_set) {
	let bits = 8 * mem::size_of::<c_long>() as c_int;
	(*set).fds_bits[(fd / bits) as usize] |= 1 << (fd % bits);
}

unsafe fn fd_isset(fd: c_int, set: *mut fd_set) -> bool {
	let bits = 8 * mem::size_of::<c_long>() as c_int;
	((*set).fds_bits[(fd / bits) as usize] & (1 << (fd % bits))) != 0
}

unsafe fn enable_raw_mode() {
	let mut raw: termios = mem::zeroed();
	tcgetattr(STDIN_FILENO, &mut orig_termios);
	ptr::copy_nonoverlapping(&orig_termios, &mut raw, 1);
	raw.c_lflag &= !(ICANON | ECHO);
	tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
}

unsafe fn disable_raw_mode() {
	tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios);
}

unsafe fn get_field_by_cmd_char(ch: c_char) -> *const field_desc {
	let mut field = sort_fields.as_ptr();
	while !(*field).name.is_null() {
		if *(*field).cmd_char == ch {
			return field;
		}
		field = field.add(1);
	}
	ptr::null()
}

unsafe fn get_field_by_name(name: *const c_char) -> *const field_desc {
	let mut field = sort_fields.as_ptr();
	let mut field_len: size_t;
	while !(*field).name.is_null() {
		field_len = strlen((*field).name);
		if field_len != strlen(name) {
			field = field.add(1);
			continue;
		}
		if strncmp((*field).name, name, field_len) == 0 {
			return field;
		}
		field = field.add(1);
	}
	ptr::null()
}

unsafe fn get_name_by_field(field: *const field_desc) -> *const c_char {
	if !field.is_null() { (*field).name } else { c_lit(UNKNOWN) }
}

unsafe fn display_available_fields(mode: size_t) {
	let mut field = sort_fields.as_ptr();
	let mut buf = [0 as c_char; MAX_BUF_LEN];
	buf[0] = 0;
	while !(*field).name.is_null() {
		if ((*field).supported_modes & mode) == 0 {
			field = field.add(1);
			continue;
		}
		let remain = MAX_BUF_LEN - strlen(buf.as_ptr()) - 1;
		strncat(buf.as_mut_ptr(), c_lit(b"|\0"), remain);
		let remain = MAX_BUF_LEN - strlen(buf.as_ptr()) - 1;
		strncat(buf.as_mut_ptr(), (*field).name, remain);
		buf[MAX_BUF_LEN - 1] = 0;
		field = field.add(1);
	}
	fprintf(stderr, c_lit(b"Available fields: %s\n\0"), buf.as_ptr());
}

unsafe fn usage() -> ! {
	printf(c_lit(b"Usage: delaytop [Options]\nOptions:\n  -h, --help               Show this help message and exit\n  -d, --delay=SECONDS      Set refresh interval (default: 2 seconds, min: 1)\n  -n, --iterations=COUNT   Set number of updates (default: 0 = infinite)\n  -P, --processes=NUMBER   Set maximum number of processes to show (default: 20, max: 1000)\n  -o, --once               Display once and exit\n  -p, --pid=PID            Monitor only the specified PID\n  -C, --container=PATH     Monitor the container at specified cgroup path\n  -s, --sort=FIELD         Sort by delay field (default: cpu)\n  -M, --memverbose         Display memory detailed information\n\0"));
	exit(0);
}

unsafe fn parse_args(argc: c_int, argv: *mut *mut c_char) {
	let mut field: *const field_desc;
	let long_options = [
		option { name: c_lit(b"help\0"), has_arg: no_argument, flag: ptr::null_mut(), val: 'h' as c_int },
		option { name: c_lit(b"delay\0"), has_arg: required_argument, flag: ptr::null_mut(), val: 'd' as c_int },
		option { name: c_lit(b"iterations\0"), has_arg: required_argument, flag: ptr::null_mut(), val: 'n' as c_int },
		option { name: c_lit(b"pid\0"), has_arg: required_argument, flag: ptr::null_mut(), val: 'p' as c_int },
		option { name: c_lit(b"once\0"), has_arg: no_argument, flag: ptr::null_mut(), val: 'o' as c_int },
		option { name: c_lit(b"processes\0"), has_arg: required_argument, flag: ptr::null_mut(), val: 'P' as c_int },
		option { name: c_lit(b"sort\0"), has_arg: required_argument, flag: ptr::null_mut(), val: 's' as c_int },
		option { name: c_lit(b"container\0"), has_arg: required_argument, flag: ptr::null_mut(), val: 'C' as c_int },
		option { name: c_lit(b"memverbose\0"), has_arg: no_argument, flag: ptr::null_mut(), val: 'M' as c_int },
		option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
	];
	cfg.delay = 2;
	cfg.iterations = 0;
	cfg.max_processes = 20;
	cfg.sort_field = sort_fields.as_ptr();
	cfg.output_one_time = 0;
	cfg.monitor_pid = 0;
	cfg.container_path = ptr::null_mut();
	cfg.display_mode = MODE_DEFAULT;

	loop {
		let mut option_index = 0;
		let c = getopt_long(argc, argv, c_lit(b"hd:n:p:oP:C:s:M\0"), long_options.as_ptr(), &mut option_index);
		if c == -1 {
			break;
		}
		match c {
			x if x == 'h' as c_int => usage(),
			x if x == 'd' as c_int => {
				cfg.delay = atoi(optarg);
				if cfg.delay < 1 {
					fprintf(stderr, c_lit(b"Error: delay must be >= 1.\n\0"));
					exit(1);
				}
			}
			x if x == 'n' as c_int => {
				cfg.iterations = atoi(optarg);
				if cfg.iterations < 0 {
					fprintf(stderr, c_lit(b"Error: iterations must be >= 0.\n\0"));
					exit(1);
				}
			}
			x if x == 'p' as c_int => {
				cfg.monitor_pid = atoi(optarg);
				if cfg.monitor_pid < 1 {
					fprintf(stderr, c_lit(b"Error: pid must be >= 1.\n\0"));
					exit(1);
				}
			}
			x if x == 'o' as c_int => cfg.output_one_time = 1,
			x if x == 'P' as c_int => {
				cfg.max_processes = atoi(optarg);
				if cfg.max_processes < 1 {
					fprintf(stderr, c_lit(b"Error: processes must be >= 1.\n\0"));
					exit(1);
				}
				if cfg.max_processes > MAX_TASKS as c_int {
					fprintf(stderr, c_lit(b"Warning: processes capped to %d.\n\0"), MAX_TASKS as c_int);
					cfg.max_processes = MAX_TASKS as c_int;
				}
			}
			x if x == 'C' as c_int => cfg.container_path = strdup(optarg),
			x if x == 's' as c_int => {
				if strlen(optarg) == 0 {
					fprintf(stderr, c_lit(b"Error: empty sort field\n\0"));
					exit(1);
				}
				field = get_field_by_name(optarg);
				if field.is_null() {
					fprintf(stderr, c_lit(b"Error: invalid sort field '%s'\n\0"), optarg);
					display_available_fields(MODE_TYPE_ALL);
					exit(1);
				}
				cfg.sort_field = field;
			}
			x if x == 'M' as c_int => {
				cfg.display_mode = MODE_MEMVERBOSE;
				cfg.sort_field = get_field_by_name(c_lit(b"mem\0"));
			}
			_ => {
				fprintf(stderr, c_lit(b"Try 'delaytop --help' for more information.\n\0"));
				exit(1);
			}
		}
	}
}

unsafe fn set_mem_delay_total(t: *mut task_info) {
	(*t).mem_delay_total = (*t).swapin_delay_total + (*t).freepages_delay_total + (*t).thrashing_delay_total + (*t).compact_delay_total + (*t).wpcopy_delay_total;
}

unsafe fn set_mem_count(t: *mut task_info) {
	(*t).mem_count = (*t).swapin_count + (*t).freepages_count + (*t).thrashing_count + (*t).compact_count + (*t).wpcopy_count;
}

unsafe fn create_nl_socket() -> c_int {
	let fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
	if fd < 0 {
		return -1;
	}
	let mut local: sockaddr_nl = mem::zeroed();
	local.nl_family = AF_NETLINK as c_ushort;
	if bind(fd, &local as *const _ as *const sockaddr, mem::size_of::<sockaddr_nl>() as socklen_t) < 0 {
		fprintf(stderr, c_lit(b"Failed to bind socket when create nl_socket\n\0"));
		close(fd);
		return -1;
	}
	fd
}

unsafe fn send_cmd(sd: c_int, nlmsg_type: __u16, nlmsg_pid: __u32, genl_cmd: __u8, nla_type: __u16, nla_data: *mut c_void, nla_len: c_int) -> c_int {
	let mut nladdr: sockaddr_nl = mem::zeroed();
	let mut msg: nl_msg = mem::zeroed();
	msg.n.nlmsg_len = NLMSG_LENGTH(mem::size_of::<genlmsghdr>() as u32);
	msg.n.nlmsg_type = nlmsg_type;
	msg.n.nlmsg_flags = NLM_F_REQUEST;
	msg.n.nlmsg_seq = 0;
	msg.n.nlmsg_pid = nlmsg_pid;
	msg.g.cmd = genl_cmd;
	msg.g.version = 0x1;
	let na = GENLMSG_DATA(&msg.n) as *mut nlattr;
	(*na).nla_type = nla_type;
	(*na).nla_len = nla_len as u16 + mem::size_of::<nlattr>() as u16;
	memcpy(NLA_DATA(na), nla_data, nla_len as size_t);
	msg.n.nlmsg_len += NLMSG_ALIGN((*na).nla_len as u32);
	let mut buf = &mut msg as *mut _ as *mut c_char;
	let mut buflen = msg.n.nlmsg_len as c_int;
	nladdr.nl_family = AF_NETLINK as c_ushort;
	while {
		let r = sendto(sd, buf as *const c_void, buflen as size_t, 0, &nladdr as *const _ as *const sockaddr, mem::size_of::<sockaddr_nl>() as socklen_t) as c_int;
		if r < buflen {
			if r > 0 {
				buf = buf.add(r as usize);
				buflen -= r;
				true
			} else if errno != EAGAIN {
				return -1;
			} else {
				true
			}
		} else {
			false
		}
	} {}
	0
}

unsafe fn get_family_id(sd: c_int) -> c_int {
	let mut ans: nl_msg = mem::zeroed();
	let mut id = 0;
	let mut name = [0 as c_char; 100];
	strncpy(name.as_mut_ptr(), c_lit(TASKSTATS_GENL_NAME), mem::size_of_val(&name) - 1);
	name[mem::size_of_val(&name) - 1] = 0;
	let rc = send_cmd(sd, GENL_ID_CTRL, getpid() as u32, CTRL_CMD_GETFAMILY, CTRL_ATTR_FAMILY_NAME, name.as_mut_ptr() as *mut c_void, strlen(c_lit(TASKSTATS_GENL_NAME)) as c_int + 1);
	if rc < 0 {
		fprintf(stderr, c_lit(b"Failed to send cmd for family id\n\0"));
		return 0;
	}
	let rep_len = recv(sd, &mut ans as *mut _ as *mut c_void, mem::size_of_val(&ans), 0) as c_int;
	if ans.n.nlmsg_type == NLMSG_ERROR || rep_len < 0 || !NLMSG_OK(&ans.n, rep_len) {
		fprintf(stderr, c_lit(b"Failed to receive response for family id\n\0"));
		return 0;
	}
	let mut na = GENLMSG_DATA(&ans.n) as *mut nlattr;
	na = (na as *mut u8).add(NLA_ALIGN((*na).nla_len) as usize) as *mut nlattr;
	if (*na).nla_type == CTRL_ATTR_FAMILY_ID {
		id = *(NLA_DATA(na) as *mut __u16) as c_int;
	}
	id
}

unsafe fn read_psi_stats() -> c_int {
	let mut line = [0 as c_char; 256];
	let mut ret = 0;
	let mut error_count = 0;
	if access(c_lit(PSI_PATH), F_OK) != 0 {
		fprintf(stderr, c_lit(b"Error: PSI interface not found at %s\n\0"), c_lit(PSI_PATH));
		fprintf(stderr, c_lit(b"Please ensure your kernel supports PSI (Pressure Stall Information)\n\0"));
		return -1;
	}
	memset(&mut psi as *mut _ as *mut c_void, 0, mem::size_of::<psi_stats>());
	let paths = [
		(c_lit(PSI_CPU_PATH), c_lit(b"CPU\0"), 0),
		(c_lit(PSI_MEMORY_PATH), c_lit(b"Memory\0"), 1),
		(c_lit(PSI_IO_PATH), c_lit(b"IO\0"), 2),
		(c_lit(PSI_IRQ_PATH), c_lit(b"IRQ\0"), 3),
	];
	for &(path, label, kind) in &paths {
		let fp = fopen(path, c_lit(b"r\0"));
		if !fp.is_null() {
			while !fgets(line.as_mut_ptr(), line.len() as c_int, fp).is_null() {
				if strncmp(line.as_ptr(), c_lit(b"some\0"), 4) == 0 {
					if kind == 3 { continue; }
					ret = match kind {
						0 => sscanf(line.as_ptr(), c_lit(b"some avg10=%lf avg60=%lf avg300=%lf total=%llu\0"), &mut psi.cpu_some_avg10, &mut psi.cpu_some_avg60, &mut psi.cpu_some_avg300, &mut psi.cpu_some_total),
						1 => sscanf(line.as_ptr(), c_lit(b"some avg10=%lf avg60=%lf avg300=%lf total=%llu\0"), &mut psi.memory_some_avg10, &mut psi.memory_some_avg60, &mut psi.memory_some_avg300, &mut psi.memory_some_total),
						_ => sscanf(line.as_ptr(), c_lit(b"some avg10=%lf avg60=%lf avg300=%lf total=%llu\0"), &mut psi.io_some_avg10, &mut psi.io_some_avg60, &mut psi.io_some_avg300, &mut psi.io_some_total),
					};
					if ret != 4 {
						fprintf(stderr, c_lit(b"Failed to parse %s some PSI data\n\0"), label);
						error_count += 1;
					}
				} else if strncmp(line.as_ptr(), c_lit(b"full\0"), 4) == 0 {
					ret = match kind {
						0 => sscanf(line.as_ptr(), c_lit(b"full avg10=%lf avg60=%lf avg300=%lf total=%llu\0"), &mut psi.cpu_full_avg10, &mut psi.cpu_full_avg60, &mut psi.cpu_full_avg300, &mut psi.cpu_full_total),
						1 => sscanf(line.as_ptr(), c_lit(b"full avg10=%lf avg60=%lf avg300=%lf total=%llu\0"), &mut psi.memory_full_avg10, &mut psi.memory_full_avg60, &mut psi.memory_full_avg300, &mut psi.memory_full_total),
						2 => sscanf(line.as_ptr(), c_lit(b"full avg10=%lf avg60=%lf avg300=%lf total=%llu\0"), &mut psi.io_full_avg10, &mut psi.io_full_avg60, &mut psi.io_full_avg300, &mut psi.io_full_total),
						_ => sscanf(line.as_ptr(), c_lit(b"full avg10=%lf avg60=%lf avg300=%lf total=%llu\0"), &mut psi.irq_full_avg10, &mut psi.irq_full_avg60, &mut psi.irq_full_avg300, &mut psi.irq_full_total),
					};
					if ret != 4 {
						fprintf(stderr, c_lit(b"Failed to parse %s full PSI data\n\0"), label);
						error_count += 1;
					}
				}
			}
			fclose(fp);
		} else {
			fprintf(stderr, c_lit(b"Warning: Failed to open %s\n\0"), path);
			error_count += 1;
		}
	}
	if error_count > 0 {
		fprintf(stderr, c_lit(b"PSI stats reading completed with %d warnings\n\0"), error_count);
		return error_count;
	}
	0
}

unsafe fn read_comm(pid: c_int, comm_buf: *mut c_char, buf_size: size_t) -> c_int {
	let mut path = [0 as c_char; 64];
	let mut ret = -1;
	snprintf(path.as_mut_ptr(), path.len(), c_lit(b"/proc/%d/comm\0"), pid);
	let fp = fopen(path.as_ptr(), c_lit(b"r\0"));
	if fp.is_null() {
		fprintf(stderr, c_lit(b"Failed to open comm file /proc/%d/comm\n\0"), pid);
		return ret;
	}
	if !fgets(comm_buf, buf_size as c_int, fp).is_null() {
		let len = strlen(comm_buf);
		if len > 0 && *comm_buf.add(len - 1) == '\n' as c_char {
			*comm_buf.add(len - 1) = 0;
		}
		ret = 0;
	}
	fclose(fp);
	ret
}

unsafe fn fetch_and_fill_task_info(pid: c_int, comm: *const c_char) {
	let mut resp: nl_msg = mem::zeroed();
	let mut stats: taskstats = mem::zeroed();
	let mut pid_arg = pid;
	if send_cmd(nl_sd, family_id as u16, getpid() as u32, TASKSTATS_CMD_GET, TASKSTATS_CMD_ATTR_PID, &mut pid_arg as *mut _ as *mut c_void, mem::size_of::<c_int>() as c_int) < 0 {
		fprintf(stderr, c_lit(b"Failed to send request for task stats\n\0"));
		return;
	}
	let rc = recv(nl_sd, &mut resp as *mut _ as *mut c_void, mem::size_of_val(&resp), 0);
	if rc < 0 || resp.n.nlmsg_type == NLMSG_ERROR {
		fprintf(stderr, c_lit(b"Failed to receive response for task stats\n\0"));
		return;
	}
	let mut nl_len = GENLMSG_PAYLOAD(&resp.n);
	let mut na = GENLMSG_DATA(&resp.n) as *mut nlattr;
	while nl_len > 0 {
		if (*na).nla_type == TASKSTATS_TYPE_AGGR_PID {
			let mut nested = NLA_DATA(na) as *mut nlattr;
			let mut nested_len = NLA_PAYLOAD((*na).nla_len);
			while nested_len > 0 {
				if (*nested).nla_type == TASKSTATS_TYPE_STATS {
					memcpy(&mut stats as *mut _ as *mut c_void, NLA_DATA(nested), mem::size_of::<taskstats>());
					if task_count < MAX_TASKS as c_int {
						let t = &mut tasks[task_count as usize];
						t.pid = pid;
						t.tgid = pid;
						strncpy(t.command.as_mut_ptr(), comm, TASK_COMM_LEN - 1);
						t.command[TASK_COMM_LEN - 1] = 0;
						t.cpu_count = stats.cpu_count;
						t.cpu_delay_total = stats.cpu_delay_total;
						t.blkio_count = stats.blkio_count;
						t.blkio_delay_total = stats.blkio_delay_total;
						t.swapin_count = stats.swapin_count;
						t.swapin_delay_total = stats.swapin_delay_total;
						t.freepages_count = stats.freepages_count;
						t.freepages_delay_total = stats.freepages_delay_total;
						t.thrashing_count = stats.thrashing_count;
						t.thrashing_delay_total = stats.thrashing_delay_total;
						t.compact_count = stats.compact_count;
						t.compact_delay_total = stats.compact_delay_total;
						t.wpcopy_count = stats.wpcopy_count;
						t.wpcopy_delay_total = stats.wpcopy_delay_total;
						t.irq_count = stats.irq_count;
						t.irq_delay_total = stats.irq_delay_total;
						set_mem_count(t);
						set_mem_delay_total(t);
						task_count += 1;
					}
					break;
				}
				nested_len -= NLA_ALIGN((*nested).nla_len) as c_int;
				nested = NLA_NEXT(nested);
			}
		}
		nl_len -= NLA_ALIGN((*na).nla_len) as c_int;
		na = NLA_NEXT(na);
	}
}

unsafe fn get_task_delays() {
	let mut comm = [0 as c_char; TASK_COMM_LEN];
	task_count = 0;
	if cfg.monitor_pid > 0 {
		if read_comm(cfg.monitor_pid, comm.as_mut_ptr(), comm.len()) == 0 {
			fetch_and_fill_task_info(cfg.monitor_pid, comm.as_ptr());
		}
		return;
	}
	let dir = opendir(c_lit(b"/proc\0"));
	if dir.is_null() {
		fprintf(stderr, c_lit(b"Error opening /proc directory\n\0"));
		return;
	}
	loop {
		let entry = readdir(dir);
		if entry.is_null() || task_count >= MAX_TASKS as c_int {
			break;
		}
		if isdigit((*entry).d_name[0] as c_int) == 0 {
			continue;
		}
		let pid = atoi((*entry).d_name.as_ptr());
		if pid == 0 {
			continue;
		}
		if read_comm(pid, comm.as_mut_ptr(), comm.len()) != 0 {
			continue;
		}
		fetch_and_fill_task_info(pid, comm.as_ptr());
	}
	closedir(dir);
}

fn average_ms(total: u64, count: u64) -> c_double {
	if count == 0 {
		return 0.0;
	}
	total as c_double / 1000000.0 / count as c_double
}

unsafe extern "C" fn compare_tasks(a: *const c_void, b: *const c_void) -> c_int {
	let t1 = a as *const task_info;
	let t2 = b as *const task_info;
	let total1 = *((t1 as *const u8).add((*cfg.sort_field).total_offset as usize) as *const u64);
	let total2 = *((t2 as *const u8).add((*cfg.sort_field).total_offset as usize) as *const u64);
	let count1 = *((t1 as *const u8).add((*cfg.sort_field).count_offset as usize) as *const u64);
	let count2 = *((t2 as *const u8).add((*cfg.sort_field).count_offset as usize) as *const u64);
	let avg1 = average_ms(total1, count1);
	let avg2 = average_ms(total2, count2);
	if avg1 != avg2 {
		return if avg2 > avg1 { 1 } else { -1 };
	}
	0
}

unsafe fn sort_tasks() {
	if task_count > 0 {
		qsort(tasks.as_mut_ptr() as *mut c_void, task_count as size_t, mem::size_of::<task_info>(), Some(compare_tasks));
	}
}

unsafe fn get_container_stats() {
	if cfg.container_path.is_null() {
		return;
	}
	let cfd = open(cfg.container_path, O_RDONLY);
	if cfd < 0 {
		fprintf(stderr, c_lit(b"Error opening container path: %s\n\0"), cfg.container_path);
		return;
	}
	let mut fd_arg = cfd as __u32;
	if send_cmd(nl_sd, family_id as u16, getpid() as u32, CGROUPSTATS_CMD_GET, CGROUPSTATS_CMD_ATTR_FD, &mut fd_arg as *mut _ as *mut c_void, mem::size_of::<__u32>() as c_int) < 0 {
		fprintf(stderr, c_lit(b"Failed to send request for container stats\n\0"));
		close(cfd);
		return;
	}
	let mut resp: nl_msg = mem::zeroed();
	let rc = recv(nl_sd, &mut resp as *mut _ as *mut c_void, mem::size_of_val(&resp), 0);
	if rc < 0 || resp.n.nlmsg_type == NLMSG_ERROR {
		fprintf(stderr, c_lit(b"Failed to receive response for container stats\n\0"));
		close(cfd);
		return;
	}
	let mut stats: cgroupstats = mem::zeroed();
	let mut nl_len = GENLMSG_PAYLOAD(&resp.n);
	let mut na = GENLMSG_DATA(&resp.n) as *mut nlattr;
	while nl_len > 0 {
		if (*na).nla_type == CGROUPSTATS_TYPE_CGROUP_STATS {
			memcpy(&mut stats as *mut _ as *mut c_void, NLA_DATA(na), mem::size_of::<cgroupstats>());
			container_stats.nr_sleeping = stats.nr_sleeping as c_int;
			container_stats.nr_running = stats.nr_running as c_int;
			container_stats.nr_stopped = stats.nr_stopped as c_int;
			container_stats.nr_uninterruptible = stats.nr_uninterruptible as c_int;
			container_stats.nr_io_wait = stats.nr_io_wait as c_int;
			break;
		}
		nl_len -= NLA_ALIGN((*na).nla_len) as c_int;
		na = (na as *mut u8).add(NLA_ALIGN((*na).nla_len) as usize) as *mut nlattr;
	}
	close(cfd);
}

unsafe fn task_avg(t: task_info, field: &str) -> c_double {
	match field {
		"mem" => average_ms(t.mem_delay_total, t.mem_count),
		"swapin" => average_ms(t.swapin_delay_total, t.swapin_count),
		"freepages" => average_ms(t.freepages_delay_total, t.freepages_count),
		"thrashing" => average_ms(t.thrashing_delay_total, t.thrashing_count),
		"compact" => average_ms(t.compact_delay_total, t.compact_count),
		"wpcopy" => average_ms(t.wpcopy_delay_total, t.wpcopy_count),
		"cpu" => average_ms(t.cpu_delay_total, t.cpu_count),
		"blkio" => average_ms(t.blkio_delay_total, t.blkio_count),
		"irq" => average_ms(t.irq_delay_total, t.irq_count),
		_ => 0.0,
	}
}

unsafe fn display_results(psi_ret: c_int) {
	let mut now = time(ptr::null_mut());
	let _tm_now = localtime(&mut now);
	let out = stdout;
	let _timestamp = [0 as c_char; 32];
	let mut suc = true;
	suc &= BOOL_FPRINT(out, c_lit(b"\x1b[H\x1b[J\0"));
	suc &= BOOL_FPRINT(out, c_lit(b"System Pressure Information: (avg10/avg60/avg300/total)\n\0"));
	if psi_ret != 0 {
		suc &= BOOL_FPRINT(out, c_lit(b"  PSI not found: check if psi=1 enabled in cmdline\n\0"));
	} else {
		let fmt = c_lit(b"%-12s %6.1f%%/%6.1f%%/%6.1f%%/%8llu(ms)\n\0");
		suc &= fprintf(out, fmt, c_lit(b"CPU some:\0"), psi.cpu_some_avg10, psi.cpu_some_avg60, psi.cpu_some_avg300, psi.cpu_some_total / 1000) >= 0;
		suc &= fprintf(out, fmt, c_lit(b"CPU full:\0"), psi.cpu_full_avg10, psi.cpu_full_avg60, psi.cpu_full_avg300, psi.cpu_full_total / 1000) >= 0;
		suc &= fprintf(out, fmt, c_lit(b"Memory full:\0"), psi.memory_full_avg10, psi.memory_full_avg60, psi.memory_full_avg300, psi.memory_full_total / 1000) >= 0;
		suc &= fprintf(out, fmt, c_lit(b"Memory some:\0"), psi.memory_some_avg10, psi.memory_some_avg60, psi.memory_some_avg300, psi.memory_some_total / 1000) >= 0;
		suc &= fprintf(out, fmt, c_lit(b"IO full:\0"), psi.io_full_avg10, psi.io_full_avg60, psi.io_full_avg300, psi.io_full_total / 1000) >= 0;
		suc &= fprintf(out, fmt, c_lit(b"IO some:\0"), psi.io_some_avg10, psi.io_some_avg60, psi.io_some_avg300, psi.io_some_total / 1000) >= 0;
		suc &= fprintf(out, fmt, c_lit(b"IRQ full:\0"), psi.irq_full_avg10, psi.irq_full_avg60, psi.irq_full_avg300, psi.irq_full_total / 1000) >= 0;
	}
	if !cfg.container_path.is_null() {
		suc &= fprintf(out, c_lit(b"Container Information (%s):\n\0"), cfg.container_path) >= 0;
		suc &= fprintf(out, c_lit(b"Processes: running=%d, sleeping=%d, \0"), container_stats.nr_running, container_stats.nr_sleeping) >= 0;
		suc &= fprintf(out, c_lit(b"stopped=%d, uninterruptible=%d, io_wait=%d\n\n\0"), container_stats.nr_stopped, container_stats.nr_uninterruptible, container_stats.nr_io_wait) >= 0;
	}
	suc &= BOOL_FPRINT(out, c_lit(b"[o]sort [M]memverbose [q]quit\n\0"));
	if sort_selected != 0 {
		if cfg.display_mode == MODE_MEMVERBOSE {
			suc &= BOOL_FPRINT(out, c_lit(b"sort selection: [m]MEM [r]RCL [t]THR [p]CMP [w]WP\n\0"));
		} else {
			suc &= BOOL_FPRINT(out, c_lit(b"sort selection: [c]CPU [i]IO [m]MEM [q]IRQ\n\0"));
		}
	}
	suc &= fprintf(out, c_lit(b"Top %d processes (sorted by %s delay):\n\0"), cfg.max_processes, get_name_by_field(cfg.sort_field)) >= 0;
	suc &= fprintf(out, c_lit(b"%8s  %8s  %-17s\0"), c_lit(b"PID\0"), c_lit(b"TGID\0"), c_lit(b"COMMAND\0")) >= 0;
	if cfg.display_mode == MODE_MEMVERBOSE {
		suc &= fprintf(out, c_lit(b"%8s %8s %8s %8s %8s %8s\n\0"), c_lit(b"MEM(ms)\0"), c_lit(b"SWAP(ms)\0"), c_lit(b"RCL(ms)\0"), c_lit(b"THR(ms)\0"), c_lit(b"CMP(ms)\0"), c_lit(b"WP(ms)\0")) >= 0;
		suc &= BOOL_FPRINT(out, c_lit(b"-----------------------\0"));
		suc &= BOOL_FPRINT(out, c_lit(b"-----------------------\0"));
		suc &= BOOL_FPRINT(out, c_lit(b"-----------------------\0"));
		suc &= BOOL_FPRINT(out, c_lit(b"---------------------\n\0"));
	} else {
		suc &= fprintf(out, c_lit(b"%8s %8s %8s %8s\n\0"), c_lit(b"CPU(ms)\0"), c_lit(b"IO(ms)\0"), c_lit(b"IRQ(ms)\0"), c_lit(b"MEM(ms)\0")) >= 0;
		suc &= BOOL_FPRINT(out, c_lit(b"-----------------------\0"));
		suc &= BOOL_FPRINT(out, c_lit(b"-----------------------\0"));
		suc &= BOOL_FPRINT(out, c_lit(b"--------------------------\n\0"));
	}
	let count = if task_count < cfg.max_processes { task_count } else { cfg.max_processes };
	for i in 0..count {
		let t = tasks[i as usize];
		suc &= fprintf(out, c_lit(b"%8d  %8d  %-15s\0"), t.pid, t.tgid, t.command.as_ptr()) >= 0;
		if cfg.display_mode == MODE_MEMVERBOSE {
			suc &= fprintf(out, c_lit(b"%8.2f %8.2f %8.2f %8.2f %8.2f %8.2f\n\0"), task_avg(t, "mem"), task_avg(t, "swapin"), task_avg(t, "freepages"), task_avg(t, "thrashing"), task_avg(t, "compact"), task_avg(t, "wpcopy")) >= 0;
		} else {
			suc &= fprintf(out, c_lit(b"%8.2f %8.2f %8.2f %8.2f\n\0"), task_avg(t, "cpu"), task_avg(t, "blkio"), task_avg(t, "irq"), task_avg(t, "mem")) >= 0;
		}
	}
	suc &= BOOL_FPRINT(out, c_lit(b"\n\0"));
	if !suc {
		perror(c_lit(b"Error writing to output\0"));
	}
}

unsafe fn check_for_keypress() -> c_char {
	let mut tv = timeval { tv_sec: cfg.delay as c_long, tv_usec: 0 };
	let mut readfds: fd_set = mem::zeroed();
	let mut ch: c_char = 0;
	fd_zero(&mut readfds);
	fd_set_fd(STDIN_FILENO, &mut readfds);
	let r = select(STDIN_FILENO + 1, &mut readfds, ptr::null_mut(), ptr::null_mut(), &mut tv);
	if r > 0 && fd_isset(STDIN_FILENO, &mut readfds) {
		read(STDIN_FILENO, &mut ch as *mut _ as *mut c_void, 1);
		return ch;
	}
	0
}

unsafe fn toggle_display_mode() {
	static modes: [size_t; MAX_MODE_SIZE] = [MODE_DEFAULT, MODE_MEMVERBOSE];
	static mut cur_index: size_t = 0;
	cur_index = (cur_index + 1) % MAX_MODE_SIZE;
	cfg.display_mode = modes[cur_index];
}

unsafe fn handle_keypress(ch: c_char, running_ptr: *mut c_int) {
	let mut field: *const field_desc;
	if sort_selected != 0 {
		field = get_field_by_cmd_char(ch);
		if !field.is_null() && ((*field).supported_modes & cfg.display_mode) != 0 {
			cfg.sort_field = field;
		}
		sort_selected = 0;
	} else {
		match ch as u8 as char {
			'o' => sort_selected = 1,
			'M' => {
				toggle_display_mode();
				field = sort_fields.as_ptr();
				while !(*field).name.is_null() {
					if ((*field).supported_modes & cfg.display_mode) != 0 {
						cfg.sort_field = field;
						break;
					}
					field = field.add(1);
				}
			}
			'q' | 'Q' => *running_ptr = 0,
			_ => {}
		}
	}
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let mut iterations = 0;
	let mut psi_ret: c_int;
	parse_args(argc, argv);
	nl_sd = create_nl_socket();
	if nl_sd < 0 {
		fprintf(stderr, c_lit(b"Error creating netlink socket\n\0"));
		exit(1);
	}
	family_id = get_family_id(nl_sd);
	if family_id == 0 {
		fprintf(stderr, c_lit(b"Error getting taskstats family ID\n\0"));
		close(nl_sd);
		exit(1);
	}
	enable_raw_mode();
	while running != 0 {
		if ((*cfg.sort_field).supported_modes & cfg.display_mode) == 0 {
			let mut field = sort_fields.as_ptr();
			while !(*field).name.is_null() {
				if ((*field).supported_modes & cfg.display_mode) != 0 {
					cfg.sort_field = field;
					printf(c_lit(b"Auto-switched sort field to: %s\n\0"), (*field).name);
					break;
				}
				field = field.add(1);
			}
		}
		psi_ret = read_psi_stats();
		if !cfg.container_path.is_null() {
			get_container_stats();
		}
		get_task_delays();
		sort_tasks();
		display_results(psi_ret);
		if cfg.iterations > 0 {
			iterations += 1;
			if iterations >= cfg.iterations {
				break;
			}
		}
		if cfg.output_one_time != 0 {
			break;
		}
		let keypress = check_for_keypress();
		if keypress != 0 {
			handle_keypress(keypress, &mut running);
		}
	}
	disable_raw_mode();
	close(nl_sd);
	if !cfg.container_path.is_null() {
		free(cfg.container_path as *mut c_void);
	}
	0
}

fn main() {
	let args: Vec<CString> = std::env::args()
		.map(|arg| CString::new(arg).unwrap())
		.collect();
	let mut argv: Vec<*mut c_char> = args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
	argv.push(ptr::null_mut());
	unsafe {
		c_main((argv.len() - 1) as c_int, argv.as_mut_ptr());
	}
}
