// SPDX-License-Identifier: GPL-2.0
/*
 * User-space helper to sort the output of /sys/kernel/debug/page_owner
 *
 * Example use:
 * cat /sys/kernel/debug/page_owner > page_owner_full.txt
 * ./page_owner_sort page_owner_full.txt sorted_page_owner.txt
 * Or sort by total memory:
 * ./page_owner_sort -m page_owner_full.txt sorted_page_owner.txt
 *
 * See Documentation/mm/page_owner.rst
*/

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type SizeT = usize;
type PidT = c_int;
type U64 = u64;

const TASK_COMM_LEN: usize = 16;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct RegexT {
	pub re_nsub: SizeT,
	_private: [usize; 8],
}

#[repr(C)]
pub struct RegmatchT {
	pub rm_so: isize,
	pub rm_eo: isize,
}

#[repr(C)]
pub struct Stat {
	_private: [u8; 144],
	pub st_size: c_long,
}

#[repr(C)]
pub struct Option {
	pub name: *const c_char,
	pub has_arg: c_int,
	pub flag: *mut c_int,
	pub val: c_int,
}

#[repr(C)]
struct block_list {
	txt: *mut c_char,
	comm: *mut c_char, // task command name
	stacktrace: *mut c_char,
	ts_nsec: U64,
	len: c_int,
	num: c_int,
	page_num: c_int,
	pid: PidT,
	tgid: PidT,
	allocator: c_int,
}

const FILTER_PID: c_int = 1 << 1;
const FILTER_TGID: c_int = 1 << 2;
const FILTER_COMM: c_int = 1 << 3;

const FILTER_ERROR: c_int = 0;
const FILTER_SKIP: c_int = 1;
const FILTER_MATCH: c_int = 2;

const CULL_PID: c_int = 1 << 1;
const CULL_TGID: c_int = 1 << 2;
const CULL_COMM: c_int = 1 << 3;
const CULL_STACKTRACE: c_int = 1 << 4;
const CULL_ALLOCATOR: c_int = 1 << 5;

const ALLOCATOR_CMA: c_int = 1 << 1;
const ALLOCATOR_SLAB: c_int = 1 << 2;
const ALLOCATOR_VMALLOC: c_int = 1 << 3;
const ALLOCATOR_OTHERS: c_int = 1 << 4;

const ARG_TXT: c_int = 0;
const ARG_COMM: c_int = 1;
const ARG_STACKTRACE: c_int = 2;
const ARG_ALLOC_TS: c_int = 3;
const ARG_CULL_TIME: c_int = 4;
const ARG_PAGE_NUM: c_int = 5;
const ARG_PID: c_int = 6;
const ARG_TGID: c_int = 7;
const ARG_UNKNOWN: c_int = 8;
const ARG_ALLOCATOR: c_int = 9;

const SORT_ASC: c_int = 1;
const SORT_DESC: c_int = -1;

const COMP_NO_FLAG: c_int = 0;
const COMP_ALLOC: c_int = 1 << 0;
const COMP_PAGE_NUM: c_int = 1 << 1;
const COMP_PID: c_int = 1 << 2;
const COMP_STACK: c_int = 1 << 3;
const COMP_NUM: c_int = 1 << 4;
const COMP_TGID: c_int = 1 << 5;
const COMP_COMM: c_int = 1 << 6;

#[repr(C)]
struct filter_condition {
	pids: *mut PidT,
	tgids: *mut PidT,
	comms: *mut *mut c_char,
	pids_size: c_int,
	tgids_size: c_int,
	comms_size: c_int,
}

#[repr(C)]
struct sort_condition {
	cmps: *mut Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
	signs: *mut c_int,
	size: c_int,
}

const REG_EXTENDED: c_int = 1;
const REG_NEWLINE: c_int = 4;
const REG_NOTBOL: c_int = 1;
const REQUIRED_ARGUMENT: c_int = 1;
const NO_ARGUMENT: c_int = 0;
const FIELD_BUFF: usize = 25;
const BUF_SIZE: c_int = 128 * 1024;

unsafe extern "C" {
	static mut stderr: *mut FILE;
	static mut stdout: *mut FILE;
	static mut errno: c_int;
	static mut optarg: *mut c_char;
	static mut optind: c_int;

	fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
	fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
	fn strlen(s: *const c_char) -> SizeT;
	fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
	fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
	fn malloc(size: SizeT) -> *mut c_void;
	fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
	fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;
	fn fflush(stream: *mut FILE) -> c_int;
	fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
	fn perror(s: *const c_char);
	fn fileno(stream: *mut FILE) -> c_int;
	fn fstat(fd: c_int, statbuf: *mut Stat) -> c_int;
	fn qsort(
		base: *mut c_void,
		nmemb: SizeT,
		size: SizeT,
		compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
	);
	fn exit(status: c_int) -> !;
	fn getopt_long(
		argc: c_int,
		argv: *mut *mut c_char,
		optstring: *const c_char,
		longopts: *const Option,
		longindex: *mut c_int,
	) -> c_int;
	fn regexec(
		preg: *const RegexT,
		string: *const c_char,
		nmatch: SizeT,
		pmatch: *mut RegmatchT,
		eflags: c_int,
	) -> c_int;
	fn regcomp(preg: *mut RegexT, regex: *const c_char, cflags: c_int) -> c_int;
	fn regfree(preg: *mut RegexT);
}

static mut FC: filter_condition = filter_condition {
	pids: core::ptr::null_mut(),
	tgids: core::ptr::null_mut(),
	comms: core::ptr::null_mut(),
	pids_size: 0,
	tgids_size: 0,
	comms_size: 0,
};
static mut SC: sort_condition = sort_condition {
	cmps: core::ptr::null_mut(),
	signs: core::ptr::null_mut(),
	size: 0,
};
static mut ORDER_PATTERN: RegexT = RegexT { re_nsub: 0, _private: [0; 8] };
static mut PID_PATTERN: RegexT = RegexT { re_nsub: 0, _private: [0; 8] };
static mut TGID_PATTERN: RegexT = RegexT { re_nsub: 0, _private: [0; 8] };
static mut COMM_PATTERN: RegexT = RegexT { re_nsub: 0, _private: [0; 8] };
static mut TS_NSEC_PATTERN: RegexT = RegexT { re_nsub: 0, _private: [0; 8] };
static mut LIST: *mut block_list = core::ptr::null_mut();
static mut LIST_SIZE: c_int = 0;
static mut MAX_SIZE: c_int = 0;
static mut CULL: c_int = 0;
static mut FILTER: c_int = 0;
static mut DEBUG_ON: bool = false;

unsafe extern "C" fn read_block(
	buf: *mut c_char,
	ext_buf: *mut c_char,
	buf_size: c_int,
	fin: *mut FILE,
) -> c_int {
	let mut curr = buf;
	let buf_end = buf.add(buf_size as usize);

	while buf_end.offset_from(curr) > 1
		&& !fgets(curr, buf_end.offset_from(curr) as c_int, fin).is_null()
	{
		if *curr == b'\n' as c_char {
			/* empty line */
			return curr.offset_from(buf) as c_int;
		}
		if strncmp(curr, c"PFN".as_ptr(), 3) == 0 {
			strcpy(ext_buf, curr);
			continue;
		}
		curr = curr.add(strlen(curr));
	}

	-1 /* EOF or no space left in buf. */
}

unsafe extern "C" fn compare_txt(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	strcmp((*l1).txt, (*l2).txt)
}

unsafe extern "C" fn compare_stacktrace(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	strcmp((*l1).stacktrace, (*l2).stacktrace)
}

unsafe extern "C" fn compare_num(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	(*l1).num - (*l2).num
}

unsafe extern "C" fn compare_page_num(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	(*l1).page_num - (*l2).page_num
}

unsafe extern "C" fn compare_pid(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	(*l1).pid - (*l2).pid
}

unsafe extern "C" fn compare_tgid(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	(*l1).tgid - (*l2).tgid
}

unsafe extern "C" fn compare_allocator(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	(*l1).allocator - (*l2).allocator
}

unsafe extern "C" fn compare_comm(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;
	strcmp((*l1).comm, (*l2).comm)
}

unsafe extern "C" fn compare_ts(p1: *const c_void, p2: *const c_void) -> c_int {
	let l1 = p1 as *const block_list;
	let l2 = p2 as *const block_list;

	if (*l1).ts_nsec < (*l2).ts_nsec {
		return -1;
	}
	if (*l1).ts_nsec > (*l2).ts_nsec {
		return 1;
	}
	0
}

unsafe extern "C" fn compare_cull_condition(p1: *const c_void, p2: *const c_void) -> c_int {
	if CULL == 0 {
		return compare_txt(p1, p2);
	}
	if (CULL & CULL_STACKTRACE) != 0 && compare_stacktrace(p1, p2) != 0 {
		return compare_stacktrace(p1, p2);
	}
	if (CULL & CULL_PID) != 0 && compare_pid(p1, p2) != 0 {
		return compare_pid(p1, p2);
	}
	if (CULL & CULL_TGID) != 0 && compare_tgid(p1, p2) != 0 {
		return compare_tgid(p1, p2);
	}
	if (CULL & CULL_COMM) != 0 && compare_comm(p1, p2) != 0 {
		return compare_comm(p1, p2);
	}
	if (CULL & CULL_ALLOCATOR) != 0 && compare_allocator(p1, p2) != 0 {
		return compare_allocator(p1, p2);
	}
	0
}

unsafe extern "C" fn compare_sort_condition(p1: *const c_void, p2: *const c_void) -> c_int {
	let mut cmp = 0;

	for i in 0..SC.size {
		if cmp == 0 {
			let f = (*SC.cmps.add(i as usize)).unwrap();
			cmp = *SC.signs.add(i as usize) * f(p1, p2);
		}
	}
	cmp
}

unsafe fn remove_pattern(pattern: *mut RegexT, buf: *mut c_char, len: c_int) -> c_int {
	let mut pmatch = [
		RegmatchT { rm_so: 0, rm_eo: 0 },
		RegmatchT { rm_so: 0, rm_eo: 0 },
	];
	let err = regexec(pattern, buf, 2, pmatch.as_mut_ptr(), REG_NOTBOL);
	if err != 0 || pmatch[1].rm_so == -1 {
		return len;
	}

	memcpy(
		buf.add(pmatch[1].rm_so as usize) as *mut c_void,
		buf.add(pmatch[1].rm_eo as usize) as *const c_void,
		(len as isize - pmatch[1].rm_eo) as SizeT,
	);

	len - (pmatch[1].rm_eo - pmatch[1].rm_so) as c_int
}

unsafe fn search_pattern(
	pattern: *mut RegexT,
	pattern_str: *mut c_char,
	pattern_str_size: SizeT,
	buf: *mut c_char,
) -> c_int {
	let mut pmatch = [
		RegmatchT { rm_so: 0, rm_eo: 0 },
		RegmatchT { rm_so: 0, rm_eo: 0 },
	];

	let err = regexec(pattern, buf, 2, pmatch.as_mut_ptr(), REG_NOTBOL);
	if err != 0 || pmatch[1].rm_so == -1 {
		if DEBUG_ON {
			fprintf(stderr, c"no matching pattern in %s\n".as_ptr(), buf);
		}
		return -1;
	}
	let val_len = pmatch[1].rm_eo - pmatch[1].rm_so;
	if val_len as SizeT >= pattern_str_size {
		if DEBUG_ON {
			fprintf(stderr, c"pattern too long in %s\n".as_ptr(), buf);
		}
		return -1;
	}

	memcpy(
		pattern_str as *mut c_void,
		buf.add(pmatch[1].rm_so as usize) as *const c_void,
		val_len as SizeT,
	);
	*pattern_str.add(val_len as usize) = 0;

	0
}

unsafe fn check_regcomp(pattern: *mut RegexT, regex: *const c_char) -> bool {
	let err = regcomp(pattern, regex, REG_EXTENDED | REG_NEWLINE);
	if err != 0 || (*pattern).re_nsub != 1 {
		fprintf(stderr, c"Invalid pattern %s code %d\n".as_ptr(), regex, err);
		return false;
	}
	true
}

unsafe fn explode(sep: c_char, str_: *const c_char, size: *mut c_int) -> *mut *mut c_char {
	let mut count = 0;
	let len = strlen(str_) as c_int;
	let mut lastindex = -1;
	let mut j = 0;

	for i in 0..len {
		if *str_.add(i as usize) == sep {
			count += 1;
		}
	}
	count += 1;
	let ret = calloc(count as SizeT, core::mem::size_of::<*mut c_char>()) as *mut *mut c_char;

	for i in 0..len {
		if *str_.add(i as usize) == sep {
			*ret.add(j as usize) = calloc((i - lastindex) as SizeT, core::mem::size_of::<c_char>())
				as *mut c_char;
			memcpy(
				*ret.add(j as usize) as *mut c_void,
				str_.add((lastindex + 1) as usize) as *const c_void,
				(i - lastindex - 1) as SizeT,
			);
			j += 1;
			lastindex = i;
		}
	}
	if lastindex <= len - 1 {
		*ret.add(j as usize) =
			calloc((len - lastindex) as SizeT, core::mem::size_of::<c_char>()) as *mut c_char;
		memcpy(
			*ret.add(j as usize) as *mut c_void,
			str_.add((lastindex + 1) as usize) as *const c_void,
			(strlen(str_) as c_int - 1 - lastindex) as SizeT,
		);
		j += 1;
	}
	*size = j;
	ret
}

unsafe fn free_explode(arr: *mut *mut c_char, size: c_int) {
	for i in 0..size {
		free(*arr.add(i as usize) as *mut c_void);
	}
	free(arr as *mut c_void);
}

unsafe fn get_page_num(buf: *mut c_char) -> c_int {
	let mut order_str = [0 as c_char; FIELD_BUFF];
	let mut endptr: *mut c_char = core::ptr::null_mut();

	if search_pattern(&raw mut ORDER_PATTERN, order_str.as_mut_ptr(), order_str.len(), buf) < 0 {
		return 0;
	}
	errno = 0;
	let order_val = strtol(order_str.as_ptr(), &mut endptr, 10) as c_int;
	if order_val > 64 || errno != 0 || endptr == order_str.as_mut_ptr() || *endptr != 0 {
		if DEBUG_ON {
			fprintf(stderr, c"wrong order in follow buf:\n%s\n".as_ptr(), buf);
		}
		return 0;
	}

	1_i32 << order_val
}

unsafe fn get_pid(buf: *mut c_char) -> PidT {
	let mut pid_str = [0 as c_char; FIELD_BUFF];
	let mut endptr: *mut c_char = core::ptr::null_mut();

	if search_pattern(&raw mut PID_PATTERN, pid_str.as_mut_ptr(), pid_str.len(), buf) < 0 {
		return -1;
	}
	errno = 0;
	let pid = strtol(pid_str.as_ptr(), &mut endptr, 10) as PidT;
	if errno != 0 || endptr == pid_str.as_mut_ptr() || *endptr != 0 {
		if DEBUG_ON {
			fprintf(stderr, c"wrong/invalid pid in follow buf:\n%s\n".as_ptr(), buf);
		}
		return -1;
	}

	pid
}

unsafe fn get_tgid(buf: *mut c_char) -> PidT {
	let mut tgid_str = [0 as c_char; FIELD_BUFF];
	let mut endptr: *mut c_char = core::ptr::null_mut();

	if search_pattern(&raw mut TGID_PATTERN, tgid_str.as_mut_ptr(), tgid_str.len(), buf) < 0 {
		return -1;
	}
	errno = 0;
	let tgid = strtol(tgid_str.as_ptr(), &mut endptr, 10) as PidT;
	if errno != 0 || endptr == tgid_str.as_mut_ptr() || *endptr != 0 {
		if DEBUG_ON {
			fprintf(stderr, c"wrong/invalid tgid in follow buf:\n%s\n".as_ptr(), buf);
		}
		return -1;
	}

	tgid
}

unsafe fn get_ts_nsec(buf: *mut c_char) -> U64 {
	let mut ts_nsec_str = [0 as c_char; FIELD_BUFF];
	let mut endptr: *mut c_char = core::ptr::null_mut();

	if search_pattern(
		&raw mut TS_NSEC_PATTERN,
		ts_nsec_str.as_mut_ptr(),
		ts_nsec_str.len(),
		buf,
	) < 0
	{
		return -1_i64 as U64;
	}
	errno = 0;
	let ts_nsec = strtoull(ts_nsec_str.as_ptr(), &mut endptr, 10) as U64;
	if errno != 0 || endptr == ts_nsec_str.as_mut_ptr() || *endptr != 0 {
		if DEBUG_ON {
			fprintf(stderr, c"wrong ts_nsec in follow buf:\n%s\n".as_ptr(), buf);
		}
		return -1_i64 as U64;
	}

	ts_nsec
}

unsafe fn get_comm(buf: *mut c_char) -> *mut c_char {
	let comm_str = malloc(TASK_COMM_LEN) as *mut c_char;

	if comm_str.is_null() {
		return core::ptr::null_mut();
	}

	memset(comm_str as *mut c_void, 0, TASK_COMM_LEN);

	if search_pattern(&raw mut COMM_PATTERN, comm_str, TASK_COMM_LEN, buf) < 0 {
		free(comm_str as *mut c_void);
		return core::ptr::null_mut();
	}
	errno = 0;
	if errno != 0 {
		if DEBUG_ON {
			fprintf(stderr, c"wrong comm in follow buf:\n%s\n".as_ptr(), buf);
		}
		free(comm_str as *mut c_void);
		return core::ptr::null_mut();
	}

	comm_str
}

unsafe fn free_block_list(block: *mut block_list) {
	free((*block).comm as *mut c_void);
	free((*block).txt as *mut c_void);
}

unsafe fn get_arg_type(arg: *const c_char) -> c_int {
	if strcmp(arg, c"pid".as_ptr()) == 0 || strcmp(arg, c"p".as_ptr()) == 0 {
		ARG_PID
	} else if strcmp(arg, c"tgid".as_ptr()) == 0 || strcmp(arg, c"tg".as_ptr()) == 0 {
		ARG_TGID
	} else if strcmp(arg, c"name".as_ptr()) == 0 || strcmp(arg, c"n".as_ptr()) == 0 {
		ARG_COMM
	} else if strcmp(arg, c"stacktrace".as_ptr()) == 0 || strcmp(arg, c"st".as_ptr()) == 0 {
		ARG_STACKTRACE
	} else if strcmp(arg, c"txt".as_ptr()) == 0 || strcmp(arg, c"T".as_ptr()) == 0 {
		ARG_TXT
	} else if strcmp(arg, c"alloc_ts".as_ptr()) == 0 || strcmp(arg, c"at".as_ptr()) == 0 {
		ARG_ALLOC_TS
	} else if strcmp(arg, c"allocator".as_ptr()) == 0 || strcmp(arg, c"ator".as_ptr()) == 0 {
		ARG_ALLOCATOR
	} else {
		ARG_UNKNOWN
	}
}

unsafe fn get_allocator(buf: *const c_char, migrate_info: *const c_char) -> c_int {
	let mut allocator = 0;

	if !strstr(migrate_info, c"CMA".as_ptr()).is_null() {
		allocator |= ALLOCATOR_CMA;
	}
	if !strstr(migrate_info, c"slab".as_ptr()).is_null() {
		allocator |= ALLOCATOR_SLAB;
	}
	let mut tmp = strstr(buf, c"__vmalloc_node_range".as_ptr());
	if !tmp.is_null() {
		let second_line = tmp;
		while *tmp != b'\n' as c_char {
			tmp = tmp.sub(1);
		}
		tmp = tmp.sub(1);
		while *tmp != b'\n' as c_char {
			tmp = tmp.sub(1);
		}
		tmp = tmp.add(1);
		let first_line = tmp;
		tmp = strstr(tmp, c"alloc_pages".as_ptr());
		if !tmp.is_null() && first_line <= tmp && tmp < second_line {
			allocator |= ALLOCATOR_VMALLOC;
		}
	}
	if allocator == 0 {
		allocator = ALLOCATOR_OTHERS;
	}
	allocator
}

unsafe fn match_num_list(num: c_int, list: *mut c_int, list_size: c_int) -> bool {
	for i in 0..list_size {
		if *list.add(i as usize) == num {
			return true;
		}
	}
	false
}

unsafe fn match_str_list(str_: *const c_char, list: *mut *mut c_char, list_size: c_int) -> bool {
	for i in 0..list_size {
		if strcmp(*list.add(i as usize), str_) == 0 {
			return true;
		}
	}
	false
}

unsafe fn filter_record(buf: *mut c_char) -> c_int {
	let comm: *mut c_char;

	if (FILTER & FILTER_PID) != 0 && !match_num_list(get_pid(buf), FC.pids, FC.pids_size) {
		return FILTER_SKIP;
	}
	if (FILTER & FILTER_TGID) != 0 && !match_num_list(get_tgid(buf), FC.tgids, FC.tgids_size) {
		return FILTER_SKIP;
	}
	if (FILTER & FILTER_COMM) == 0 {
		return FILTER_MATCH;
	}

	comm = get_comm(buf);
	if comm.is_null() {
		return FILTER_ERROR;
	}

	if !match_str_list(comm, FC.comms, FC.comms_size) {
		free(comm as *mut c_void);
		return FILTER_SKIP;
	}
	free(comm as *mut c_void);
	FILTER_MATCH
}

unsafe fn add_list(buf: *mut c_char, mut len: c_int, ext_buf: *mut c_char) -> bool {
	let filter_result: c_int;

	if LIST_SIZE == MAX_SIZE {
		fprintf(stderr, c"max_size too small??\n".as_ptr());
		return false;
	}
	filter_result = filter_record(buf);
	if filter_result == FILTER_ERROR {
		fprintf(stderr, c"Out of memory\n".as_ptr());
		return false;
	}
	if filter_result == FILTER_SKIP {
		return true;
	}
	let item = LIST.add(LIST_SIZE as usize);
	(*item).pid = get_pid(buf);
	(*item).tgid = get_tgid(buf);
	(*item).comm = get_comm(buf);
	if (*item).comm.is_null() {
		fprintf(stderr, c"Out of memory\n".as_ptr());
		return false;
	}
	(*item).txt = malloc((len + 1) as SizeT) as *mut c_char;
	if (*item).txt.is_null() {
		fprintf(stderr, c"Out of memory\n".as_ptr());
		free((*item).comm as *mut c_void);
		return false;
	}
	memcpy((*item).txt as *mut c_void, buf as *const c_void, len as SizeT);
	if *SC.cmps != Some(compare_ts) {
		len = remove_pattern(&raw mut TS_NSEC_PATTERN, (*item).txt, len);
	}
	*(*item).txt.add(len as usize) = 0;
	(*item).len = len;
	(*item).num = 1;
	(*item).page_num = get_page_num(buf);

	(*item).stacktrace = strchr((*item).txt, b'\n' as c_int);
	if (*item).stacktrace.is_null() {
		(*item).stacktrace = c"".as_ptr() as *mut c_char;
	}
	if *(*item).stacktrace == b'\n' as c_char {
		(*item).stacktrace = (*item).stacktrace.add(1);
	}
	(*item).ts_nsec = get_ts_nsec(buf);
	(*item).allocator = get_allocator(buf, ext_buf);
	LIST_SIZE += 1;
	if LIST_SIZE % 1000 == 0 {
		printf(c"loaded %d\r".as_ptr(), LIST_SIZE);
		fflush(stdout);
	}
	true
}

unsafe fn parse_cull_args(arg_str: *const c_char) -> bool {
	let mut size = 0;
	let args = explode(b',' as c_char, arg_str, &mut size);

	for i in 0..size {
		let arg_type = get_arg_type(*args.add(i as usize));

		if arg_type == ARG_PID {
			CULL |= CULL_PID;
		} else if arg_type == ARG_TGID {
			CULL |= CULL_TGID;
		} else if arg_type == ARG_COMM {
			CULL |= CULL_COMM;
		} else if arg_type == ARG_STACKTRACE {
			CULL |= CULL_STACKTRACE;
		} else if arg_type == ARG_ALLOCATOR {
			CULL |= CULL_ALLOCATOR;
		} else {
			free_explode(args, size);
			return false;
		}
	}
	free_explode(args, size);
	if SC.size == 0 {
		set_single_cmp(Some(compare_num), SORT_DESC);
	}
	true
}

unsafe fn set_single_cmp(
	cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
	sign: c_int,
) {
	if SC.signs.is_null() || SC.size < 1 {
		SC.signs = calloc(1, core::mem::size_of::<c_int>()) as *mut c_int;
	}
	*SC.signs = sign;
	if SC.cmps.is_null() || SC.size < 1 {
		SC.cmps = calloc(
			1,
			core::mem::size_of::<Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>>(),
		) as *mut Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>;
	}
	*SC.cmps = cmp;
	SC.size = 1;
}

unsafe fn parse_sort_args(arg_str: *const c_char) -> bool {
	let mut size = 0;

	if SC.size != 0 {
		/* reset sort_condition */
		free(SC.signs as *mut c_void);
		free(SC.cmps as *mut c_void);
		size = 0;
	}

	let args = explode(b',' as c_char, arg_str, &mut size);

	SC.signs = calloc(size as SizeT, core::mem::size_of::<c_int>()) as *mut c_int;
	SC.cmps = calloc(
		size as SizeT,
		core::mem::size_of::<Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>>(),
	) as *mut Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>;
	for i in 0..size {
		let mut offset = 0;

		*SC.signs.add(i as usize) = SORT_ASC;
		if **args.add(i as usize) == b'-' as c_char || **args.add(i as usize) == b'+' as c_char {
			if **args.add(i as usize) == b'-' as c_char {
				*SC.signs.add(i as usize) = SORT_DESC;
			}
			offset = 1;
		}

		let arg_type = get_arg_type((*args.add(i as usize)).add(offset));

		if arg_type == ARG_PID {
			*SC.cmps.add(i as usize) = Some(compare_pid);
		} else if arg_type == ARG_TGID {
			*SC.cmps.add(i as usize) = Some(compare_tgid);
		} else if arg_type == ARG_COMM {
			*SC.cmps.add(i as usize) = Some(compare_comm);
		} else if arg_type == ARG_STACKTRACE {
			*SC.cmps.add(i as usize) = Some(compare_stacktrace);
		} else if arg_type == ARG_ALLOC_TS {
			*SC.cmps.add(i as usize) = Some(compare_ts);
		} else if arg_type == ARG_TXT {
			*SC.cmps.add(i as usize) = Some(compare_txt);
		} else if arg_type == ARG_ALLOCATOR {
			*SC.cmps.add(i as usize) = Some(compare_allocator);
		} else {
			free_explode(args, size);
			SC.size = 0;
			return false;
		}
	}
	SC.size = size;
	free_explode(args, size);
	true
}

unsafe fn parse_nums_list(arg_str: *mut c_char, list_size: *mut c_int) -> *mut c_int {
	let mut size = 0;
	let args = explode(b',' as c_char, arg_str, &mut size);
	let list = calloc(size as SizeT, core::mem::size_of::<c_int>()) as *mut c_int;

	errno = 0;
	for i in 0..size {
		let mut endptr: *mut c_char = core::ptr::null_mut();

		*list.add(i as usize) = strtol(*args.add(i as usize), &mut endptr, 10) as c_int;
		if errno != 0 || endptr == *args.add(i as usize) || *endptr != 0 {
			free(list as *mut c_void);
			return core::ptr::null_mut();
		}
	}
	*list_size = size;
	free_explode(args, size);
	list
}

unsafe fn print_allocator(out: *mut FILE, allocator: c_int) {
	fprintf(out, c"allocated by ".as_ptr());
	if (allocator & ALLOCATOR_CMA) != 0 {
		fprintf(out, c"CMA ".as_ptr());
	}
	if (allocator & ALLOCATOR_SLAB) != 0 {
		fprintf(out, c"SLAB ".as_ptr());
	}
	if (allocator & ALLOCATOR_VMALLOC) != 0 {
		fprintf(out, c"VMALLOC ".as_ptr());
	}
	if (allocator & ALLOCATOR_OTHERS) != 0 {
		fprintf(out, c"OTHERS ".as_ptr());
	}
}

unsafe fn usage() {
	printf(
		c"Usage: ./page_owner_sort [OPTIONS] <input> <output>\n-a\t\t\tSort by memory allocation time.\n-m\t\t\tSort by total memory.\n-n\t\t\tSort by task command name.\n-p\t\t\tSort by pid.\n-P\t\t\tSort by tgid.\n-s\t\t\tSort by the stacktrace.\n-t\t\t\tSort by number of times record is seen (default).\n\n--pid <pidlist>\t\tSelect by pid. This selects the information of\n\t\t\tblocks whose process ID numbers appear in <pidlist>.\n--tgid <tgidlist>\tSelect by tgid. This selects the information of\n\t\t\tblocks whose Thread Group ID numbers appear in <tgidlist>.\n--name <cmdlist>\tSelect by command name. This selects the information\n\t\t\tof blocks whose command name appears in <cmdlist>.\n--cull <rules>\t\tCull by user-defined rules. <rules> is a single\n\t\t\targument in the form of a comma-separated list with some\n\t\t\tcommon fields predefined (pid, tgid, comm, stacktrace, allocator)\n--sort <order>\t\tSpecify sort order as: [+|-]key[,[+|-]key[,...]]\n".as_ptr(),
	);
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let mut fin: *mut FILE;
	let mut fout: *mut FILE;
	let mut buf: *mut c_char = core::ptr::null_mut();
	let mut ext_buf: *mut c_char = core::ptr::null_mut();
	let mut i: c_int;
	let mut count: c_int;
	let mut st: Stat = core::mem::zeroed();
	let mut opt: c_int;
	let mut compare_flag: c_int;
	let longopts = [
		Option { name: c"pid".as_ptr(), has_arg: REQUIRED_ARGUMENT, flag: core::ptr::null_mut(), val: 1 },
		Option { name: c"tgid".as_ptr(), has_arg: REQUIRED_ARGUMENT, flag: core::ptr::null_mut(), val: 2 },
		Option { name: c"name".as_ptr(), has_arg: REQUIRED_ARGUMENT, flag: core::ptr::null_mut(), val: 3 },
		Option { name: c"cull".as_ptr(), has_arg: REQUIRED_ARGUMENT, flag: core::ptr::null_mut(), val: 4 },
		Option { name: c"sort".as_ptr(), has_arg: REQUIRED_ARGUMENT, flag: core::ptr::null_mut(), val: 5 },
		Option { name: c"help".as_ptr(), has_arg: NO_ARGUMENT, flag: core::ptr::null_mut(), val: b'h' as c_int },
		Option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
	];

	compare_flag = COMP_NO_FLAG;

	loop {
		opt = getopt_long(argc, argv, c"admnpstPh".as_ptr(), longopts.as_ptr(), core::ptr::null_mut());
		if opt == -1 {
			break;
		}
		match opt {
			x if x == b'a' as c_int => compare_flag |= COMP_ALLOC,
			x if x == b'd' as c_int => DEBUG_ON = true,
			x if x == b'm' as c_int => compare_flag |= COMP_PAGE_NUM,
			x if x == b'p' as c_int => compare_flag |= COMP_PID,
			x if x == b's' as c_int => compare_flag |= COMP_STACK,
			x if x == b't' as c_int => compare_flag |= COMP_NUM,
			x if x == b'P' as c_int => compare_flag |= COMP_TGID,
			x if x == b'n' as c_int => compare_flag |= COMP_COMM,
			x if x == b'h' as c_int => {
				usage();
				exit(0);
			}
			1 => {
				FILTER |= FILTER_PID;
				FC.pids = parse_nums_list(optarg, &raw mut FC.pids_size);
				if FC.pids.is_null() {
					fprintf(stderr, c"wrong/invalid pid in from the command line:%s\n".as_ptr(), optarg);
					exit(1);
				}
			}
			2 => {
				FILTER |= FILTER_TGID;
				FC.tgids = parse_nums_list(optarg, &raw mut FC.tgids_size);
				if FC.tgids.is_null() {
					fprintf(stderr, c"wrong/invalid tgid in from the command line:%s\n".as_ptr(), optarg);
					exit(1);
				}
			}
			3 => {
				FILTER |= FILTER_COMM;
				FC.comms = explode(b',' as c_char, optarg, &raw mut FC.comms_size);
			}
			4 => {
				if !parse_cull_args(optarg) {
					fprintf(stderr, c"wrong argument after --cull option:%s\n".as_ptr(), optarg);
					exit(1);
				}
			}
			5 => {
				if !parse_sort_args(optarg) {
					fprintf(stderr, c"wrong argument after --sort option:%s\n".as_ptr(), optarg);
					exit(1);
				}
			}
			_ => {
				usage();
				exit(1);
			}
		}
	}

	if optind >= argc - 1 {
		usage();
		exit(1);
	}

	/* Only one compare option is allowed, yet we also want handle the
	 * default case were no option is provided, but we still want to
	 * match the behavior of the -t option (compare by number of times
	 * a record is seen
	 */
	match compare_flag {
		COMP_ALLOC => set_single_cmp(Some(compare_ts), SORT_ASC),
		COMP_PAGE_NUM => set_single_cmp(Some(compare_page_num), SORT_DESC),
		COMP_PID => set_single_cmp(Some(compare_pid), SORT_ASC),
		COMP_STACK => set_single_cmp(Some(compare_stacktrace), SORT_ASC),
		COMP_NO_FLAG | COMP_NUM => set_single_cmp(Some(compare_num), SORT_DESC),
		COMP_TGID => set_single_cmp(Some(compare_tgid), SORT_ASC),
		COMP_COMM => set_single_cmp(Some(compare_comm), SORT_ASC),
		_ => {
			usage();
			exit(1);
		}
	}

	fin = fopen(*argv.add(optind as usize), c"r".as_ptr());
	if fin.is_null() {
		usage();
		perror(c"open: ".as_ptr());
		exit(1);
	}

	if !check_regcomp(&raw mut ORDER_PATTERN, c"order\\s*([0-9]*),".as_ptr()) {
		goto_out_order();
		return 0;
	}
	if !check_regcomp(&raw mut PID_PATTERN, c"pid\\s*([0-9]*),".as_ptr()) {
		goto_out_pid();
		return 0;
	}
	if !check_regcomp(&raw mut TGID_PATTERN, c"tgid\\s*([0-9]*) ".as_ptr()) {
		goto_out_tgid();
		return 0;
	}
	if !check_regcomp(&raw mut COMM_PATTERN, c"tgid\\s*[0-9]*\\s*\\((.*)\\),\\s*ts".as_ptr()) {
		goto_out_comm();
		return 0;
	}
	if !check_regcomp(&raw mut TS_NSEC_PATTERN, c"ts\\s*([0-9]*)\\s*ns".as_ptr()) {
		goto_out_ts();
		return 0;
	}

	fstat(fileno(fin), &mut st);
	MAX_SIZE = (st.st_size / 100) as c_int; /* hack ... */

	LIST = malloc(MAX_SIZE as SizeT * core::mem::size_of::<block_list>()) as *mut block_list;
	buf = malloc(BUF_SIZE as SizeT) as *mut c_char;
	ext_buf = malloc(BUF_SIZE as SizeT) as *mut c_char;
	if LIST.is_null() || buf.is_null() || ext_buf.is_null() {
		fprintf(stderr, c"Out of memory\n".as_ptr());
		goto_out_free(ext_buf, buf);
		return 0;
	}

	loop {
		let buf_len = read_block(buf, ext_buf, BUF_SIZE, fin);

		if buf_len < 0 {
			break;
		}
		if !add_list(buf, buf_len, ext_buf) {
			goto_out_free(ext_buf, buf);
			return 0;
		}
	}

	fout = fopen(*argv.add((optind + 1) as usize), c"w".as_ptr());
	if fout.is_null() {
		usage();
		perror(c"open: ".as_ptr());
		exit(1);
	}

	printf(c"loaded %d\n".as_ptr(), LIST_SIZE);

	printf(c"sorting ....\n".as_ptr());

	qsort(
		LIST as *mut c_void,
		LIST_SIZE as SizeT,
		core::mem::size_of::<block_list>(),
		Some(compare_cull_condition),
	);

	printf(c"culling\n".as_ptr());

	i = 0;
	count = 0;
	while i < LIST_SIZE {
		if count == 0
			|| compare_cull_condition(
				LIST.add((count - 1) as usize) as *const c_void,
				LIST.add(i as usize) as *const c_void,
			) != 0
		{
			core::ptr::copy_nonoverlapping(LIST.add(i as usize), LIST.add(count as usize), 1);
			count += 1;
		} else {
			(*LIST.add((count - 1) as usize)).num += (*LIST.add(i as usize)).num;
			(*LIST.add((count - 1) as usize)).page_num += (*LIST.add(i as usize)).page_num;
			free_block_list(LIST.add(i as usize));
		}
		i += 1;
	}
	LIST_SIZE = count;

	qsort(
		LIST as *mut c_void,
		count as SizeT,
		core::mem::size_of::<block_list>(),
		Some(compare_sort_condition),
	);

	i = 0;
	while i < count {
		let item = LIST.add(i as usize);
		if CULL == 0 {
			fprintf(fout, c"%d times, %d pages, ".as_ptr(), (*item).num, (*item).page_num);
			print_allocator(fout, (*item).allocator);
			fprintf(fout, c":\n%s\n".as_ptr(), (*item).txt);
		} else {
			fprintf(fout, c"%d times, %d pages".as_ptr(), (*item).num, (*item).page_num);
			if (CULL & CULL_PID) != 0 || (FILTER & FILTER_PID) != 0 {
				fprintf(fout, c", PID %d".as_ptr(), (*item).pid);
			}
			if (CULL & CULL_TGID) != 0 || (FILTER & FILTER_TGID) != 0 {
				fprintf(fout, c", TGID %d".as_ptr(), (*item).tgid);
			}
			if (CULL & CULL_COMM) != 0 || (FILTER & FILTER_COMM) != 0 {
				fprintf(fout, c", task_comm_name: %s".as_ptr(), (*item).comm);
			}
			if (CULL & CULL_ALLOCATOR) != 0 {
				fprintf(fout, c", ".as_ptr());
				print_allocator(fout, (*item).allocator);
			}
			if (CULL & CULL_STACKTRACE) != 0 {
				fprintf(fout, c":\n%s".as_ptr(), (*item).stacktrace);
			}
			fprintf(fout, c"\n".as_ptr());
		}
		i += 1;
	}

	goto_out_free(ext_buf, buf);
	0
}

unsafe fn goto_out_free(ext_buf: *mut c_char, buf: *mut c_char) {
	if !ext_buf.is_null() {
		free(ext_buf as *mut c_void);
	}
	if !buf.is_null() {
		free(buf as *mut c_void);
	}
	if !LIST.is_null() {
		let mut i = 0;
		while i < LIST_SIZE {
			free_block_list(LIST.add(i as usize));
			i += 1;
		}
		free(LIST as *mut c_void);
	}
	goto_out_ts();
}

unsafe fn goto_out_ts() {
	regfree(&raw mut TS_NSEC_PATTERN);
	goto_out_comm();
}

unsafe fn goto_out_comm() {
	regfree(&raw mut COMM_PATTERN);
	goto_out_tgid();
}

unsafe fn goto_out_tgid() {
	regfree(&raw mut TGID_PATTERN);
	goto_out_pid();
}

unsafe fn goto_out_pid() {
	regfree(&raw mut PID_PATTERN);
	goto_out_order();
}

unsafe fn goto_out_order() {
	regfree(&raw mut ORDER_PATTERN);
}

fn main() {
	unsafe {
		unsafe extern "C" {
			static mut __libc_argc: c_int;
			static mut __libc_argv: *mut *mut c_char;
		}
		let _ = main_impl(__libc_argc, __libc_argv);
	}
}
