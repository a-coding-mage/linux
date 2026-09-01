// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/cgroup/test_kmem.c.
// C includes removed; external declarations below correspond to kselftest.h,
// cgroup_util.h, libc, and pthread interfaces used by the original file.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type SsizeT = isize;
type PthreadT = c_ulong;

const O_RDONLY: c_int = 0;
const PATH_MAX: usize = 4096;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;
const _SC_PAGESIZE: c_int = 30;
const DEFAULT_WAIT_INTERVAL_US: c_int = 100000;
const KMEM_DEAD_WAIT_RETRIES: c_int = 80;

#[repr(C)]
struct Stat {
	_data: [u8; 256],
}

#[repr(C)]
struct KmemTest {
	fn_: unsafe extern "C" fn(*const c_char) -> c_int,
	name: *const c_char,
}

unsafe extern "C" {
	fn sysconf(name: c_int) -> c_long;
	fn get_nprocs() -> c_int;
	fn getpid() -> c_int;
	fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
	fn stat(pathname: *const c_char, statbuf: *mut Stat) -> c_int;
	fn sleep(seconds: u32) -> u32;
	fn calloc(nmemb: usize, size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn labs(j: c_long) -> c_long;
	fn printf(format: *const c_char, ...) -> c_int;
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> SsizeT;
	fn close(fd: c_int) -> c_int;
	fn pthread_create(
		thread: *mut PthreadT,
		attr: *const c_void,
		start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: PthreadT, retval: *mut *mut c_void) -> c_int;

	fn ksft_print_header();
	fn ksft_set_plan(plan: c_int);
	fn ksft_exit_skip(msg: *const c_char, ...) -> !;
	fn ksft_test_result_pass(msg: *const c_char, ...);
	fn ksft_test_result_skip(msg: *const c_char, ...);
	fn ksft_test_result_fail(msg: *const c_char, ...);
	fn ksft_finished() -> !;

	fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
	fn cg_name_indexed(root: *const c_char, name: *const c_char, index: c_int) -> *mut c_char;
	fn cg_create(cgroup: *const c_char) -> c_int;
	fn cg_destroy(cgroup: *const c_char) -> c_int;
	fn cg_run(
		cgroup: *const c_char,
		fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
		arg: *mut c_void,
	) -> c_int;
	fn cg_write(cgroup: *const c_char, control: *const c_char, value: *const c_char) -> c_int;
	fn cg_read_key_long(cgroup: *const c_char, control: *const c_char, key: *const c_char) -> c_long;
	fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long;
	fn cg_read_key_long_poll(
		cgroup: *const c_char,
		control: *const c_char,
		key: *const c_char,
		expected: c_long,
		retries: c_int,
		wait_us: c_int,
	) -> c_long;
	fn cg_find_unified_root(root: *mut c_char, len: usize, mount: *const c_char) -> c_int;
	fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char) -> c_int;
}

unsafe fn max_vmstat_error() -> c_long {
	sysconf(_SC_PAGESIZE) * 64 * get_nprocs() as c_long
}

unsafe extern "C" fn alloc_dcache(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
	let mut i: c_ulong;
	let mut st: Stat = mem::zeroed();
	let mut buf = [0 as c_char; 128];

	i = 0;
	while i < arg as c_ulong {
		snprintf(
			buf.as_mut_ptr(),
			buf.len(),
			c"/something-non-existent-with-a-long-name-%64lu-%d".as_ptr(),
			i,
			getpid(),
		);
		stat(buf.as_ptr(), &mut st);
		i += 1;
	}

	0
}

/*
 * This test allocates 100000 of negative dentries with long names.
 * Then it checks that "slab" in memory.stat is larger than 1M.
 * Then it sets memory.high to 1M and checks that at least 1/2
 * of slab memory has been reclaimed.
 */
unsafe extern "C" fn test_kmem_basic(root: *const c_char) -> c_int {
	let mut ret = KSFT_FAIL;
	let mut cg: *mut c_char = ptr::null_mut();
	let slab0: c_long;
	let slab1: c_long;
	let current: c_long;

	cg = cg_name(root, c"kmem_basic_test".as_ptr());
	if cg.is_null() {
		goto_cleanup_basic(cg);
		return ret;
	}

	if cg_create(cg) != 0 {
		goto_cleanup_basic(cg);
		return ret;
	}

	if cg_run(cg, alloc_dcache, 100000usize as *mut c_void) != 0 {
		goto_cleanup_basic(cg);
		return ret;
	}

	slab0 = cg_read_key_long(cg, c"memory.stat".as_ptr(), c"slab ".as_ptr());
	if slab0 < (1 << 20) {
		goto_cleanup_basic(cg);
		return ret;
	}

	cg_write(cg, c"memory.high".as_ptr(), c"1M".as_ptr());

	/* wait for RCU freeing */
	sleep(1);

	slab1 = cg_read_key_long(cg, c"memory.stat".as_ptr(), c"slab ".as_ptr());
	if slab1 < 0 {
		goto_cleanup_basic(cg);
		return ret;
	}

	current = cg_read_long(cg, c"memory.current".as_ptr());
	if current < 0 {
		goto_cleanup_basic(cg);
		return ret;
	}

	if slab1 < slab0 / 2 && current < slab0 / 2 {
		ret = KSFT_PASS;
	}
	goto_cleanup_basic(cg);
	ret
}

unsafe fn goto_cleanup_basic(cg: *mut c_char) {
	cg_destroy(cg);
	free(cg as *mut c_void);
}

unsafe extern "C" fn alloc_kmem_fn(_arg: *mut c_void) -> *mut c_void {
	alloc_dcache(ptr::null(), 100usize as *mut c_void);
	ptr::null_mut()
}

unsafe extern "C" fn alloc_kmem_smp(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
	let nr_threads: c_int = 2 * get_nprocs();
	let tinfo: *mut PthreadT;
	let mut i: c_ulong;
	let mut ret: c_int = -1;

	tinfo = calloc(nr_threads as usize, mem::size_of::<PthreadT>()) as *mut PthreadT;
	if tinfo.is_null() {
		return -1;
	}

	i = 0;
	while i < nr_threads as c_ulong {
		if pthread_create(tinfo.add(i as usize), ptr::null(), Some(alloc_kmem_fn), i as *mut c_void) != 0 {
			free(tinfo as *mut c_void);
			return -1;
		}
		i += 1;
	}

	i = 0;
	while i < nr_threads as c_ulong {
		ret = pthread_join(*tinfo.add(i as usize), ptr::null_mut());
		if ret != 0 {
			break;
		}
		i += 1;
	}

	free(tinfo as *mut c_void);
	ret
}

unsafe fn cg_run_in_subcgroups(
	parent: *const c_char,
	fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
	_arg: *mut c_void,
	times: c_int,
) -> c_int {
	let mut child: *mut c_char;
	let mut i: c_int;

	i = 0;
	while i < times {
		child = cg_name_indexed(parent, c"child".as_ptr(), i);
		if child.is_null() {
			return -1;
		}

		if cg_create(child) != 0 {
			cg_destroy(child);
			free(child as *mut c_void);
			return -1;
		}

		if cg_run(child, fn_, ptr::null_mut()) != 0 {
			cg_destroy(child);
			free(child as *mut c_void);
			return -1;
		}

		cg_destroy(child);
		free(child as *mut c_void);
		i += 1;
	}

	0
}

/*
 * The test creates and destroys a large number of cgroups. In each cgroup it
 * allocates some slab memory (mostly negative dentries) using 2 * NR_CPUS
 * threads. Then it checks the sanity of numbers on the parent level:
 * the total size of the cgroups should be roughly equal to
 * anon + file + kernel + sock.
 */
unsafe extern "C" fn test_kmem_memcg_deletion(root: *const c_char) -> c_int {
	let current: c_long;
	let anon: c_long;
	let file: c_long;
	let kernel: c_long;
	let sock: c_long;
	let sum: c_long;
	let mut ret = KSFT_FAIL;
	let parent: *mut c_char;

	parent = cg_name(root, c"kmem_memcg_deletion_test".as_ptr());
	if parent.is_null() {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_create(parent) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+memory".as_ptr()) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_run_in_subcgroups(parent, alloc_kmem_smp, ptr::null_mut(), 100) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	current = cg_read_long(parent, c"memory.current".as_ptr());
	anon = cg_read_key_long(parent, c"memory.stat".as_ptr(), c"anon ".as_ptr());
	file = cg_read_key_long(parent, c"memory.stat".as_ptr(), c"file ".as_ptr());
	kernel = cg_read_key_long(parent, c"memory.stat".as_ptr(), c"kernel ".as_ptr());
	sock = cg_read_key_long(parent, c"memory.stat".as_ptr(), c"sock ".as_ptr());
	if current < 0 || anon < 0 || file < 0 || kernel < 0 || sock < 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	sum = anon + file + kernel + sock;
	if labs(sum - current) < max_vmstat_error() {
		ret = KSFT_PASS;
	} else {
		printf(c"memory.current = %ld\n".as_ptr(), current);
		printf(c"anon + file + kernel + sock = %ld\n".as_ptr(), sum);
		printf(c"anon = %ld\n".as_ptr(), anon);
		printf(c"file = %ld\n".as_ptr(), file);
		printf(c"kernel = %ld\n".as_ptr(), kernel);
		printf(c"sock = %ld\n".as_ptr(), sock);
	}

	goto_cleanup_parent(parent);
	ret
}

unsafe fn goto_cleanup_parent(parent: *mut c_char) {
	cg_destroy(parent);
	free(parent as *mut c_void);
}

/*
 * The test reads the entire /proc/kpagecgroup. If the operation went
 * successfully (and the kernel didn't panic), the test is treated as passed.
 */
unsafe extern "C" fn test_kmem_proc_kpagecgroup(_root: *const c_char) -> c_int {
	let mut buf = [0 as c_ulong; 128];
	let mut ret = KSFT_FAIL;
	let mut len: SsizeT;
	let fd: c_int;

	fd = open(c"/proc/kpagecgroup".as_ptr(), O_RDONLY);
	if fd < 0 {
		return ret;
	}

	loop {
		len = read(fd, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf));
		if len <= 0 {
			break;
		}
	}

	if len == 0 {
		ret = KSFT_PASS;
	}

	close(fd);
	ret
}

unsafe extern "C" fn pthread_wait_fn(_arg: *mut c_void) -> *mut c_void {
	sleep(100);
	ptr::null_mut()
}

unsafe extern "C" fn spawn_1000_threads(cgroup: *const c_char, _arg: *mut c_void) -> c_int {
	let nr_threads: c_int = 1000;
	let tinfo: *mut PthreadT;
	let mut i: c_ulong;
	let stack: c_long;
	let mut ret: c_int = -1;

	tinfo = calloc(nr_threads as usize, mem::size_of::<PthreadT>()) as *mut PthreadT;
	if tinfo.is_null() {
		return -1;
	}

	i = 0;
	while i < nr_threads as c_ulong {
		if pthread_create(tinfo.add(i as usize), ptr::null(), Some(pthread_wait_fn), i as *mut c_void) != 0 {
			free(tinfo as *mut c_void);
			return -1;
		}
		i += 1;
	}

	stack = cg_read_key_long(cgroup, c"memory.stat".as_ptr(), c"kernel_stack ".as_ptr());
	if stack >= 4096 * 1000 {
		ret = 0;
	}

	free(tinfo as *mut c_void);
	ret
}

/*
 * The test spawns a process, which spawns 1000 threads. Then it checks
 * that memory.stat's kernel_stack is at least 1000 pages large.
 */
unsafe extern "C" fn test_kmem_kernel_stacks(root: *const c_char) -> c_int {
	let mut ret = KSFT_FAIL;
	let mut cg: *mut c_char = ptr::null_mut();

	cg = cg_name(root, c"kmem_kernel_stacks_test".as_ptr());
	if cg.is_null() {
		goto_cleanup_basic(cg);
		return ret;
	}

	if cg_create(cg) != 0 {
		goto_cleanup_basic(cg);
		return ret;
	}

	if cg_run(cg, spawn_1000_threads, ptr::null_mut()) != 0 {
		goto_cleanup_basic(cg);
		return ret;
	}

	ret = KSFT_PASS;
	goto_cleanup_basic(cg);
	ret
}

/*
 * This test sequentionally creates 30 child cgroups, allocates some
 * kernel memory in each of them, and deletes them. Then it checks
 * that the number of dying cgroups on the parent level is 0.
 */
unsafe extern "C" fn test_kmem_dead_cgroups(root: *const c_char) -> c_int {
	let mut ret = KSFT_FAIL;
	let parent: *mut c_char;
	let mut dead: c_long = -1;

	parent = cg_name(root, c"kmem_dead_cgroups_test".as_ptr());
	if parent.is_null() {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_create(parent) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+memory".as_ptr()) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_run_in_subcgroups(parent, alloc_dcache, 100usize as *mut c_void, 30) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	/*
	 * Allow up to ~8s for reclaim of dying descendants to complete.
	 * This is a generous upper bound derived from stress testing, not
	 * from a specific kernel constant, and can be adjusted if reclaim
	 * behavior changes in the future.
	 */
	dead = cg_read_key_long_poll(
		parent,
		c"cgroup.stat".as_ptr(),
		c"nr_dying_descendants ".as_ptr(),
		0,
		KMEM_DEAD_WAIT_RETRIES,
		DEFAULT_WAIT_INTERVAL_US,
	);
	if dead != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	ret = KSFT_PASS;

	goto_cleanup_parent(parent);
	ret
}

/*
 * This test creates a sub-tree with 1000 memory cgroups.
 * Then it checks that the memory.current on the parent level
 * is greater than 0 and approximates matches the percpu value
 * from memory.stat.
 */
unsafe extern "C" fn test_percpu_basic(root: *const c_char) -> c_int {
	let mut ret = KSFT_FAIL;
	let parent: *mut c_char;
	let mut child: *mut c_char;
	let current: c_long;
	let percpu: c_long;
	let slab: c_long;
	let mut i: c_int;

	parent = cg_name(root, c"percpu_basic_test".as_ptr());
	if parent.is_null() {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_create(parent) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+memory".as_ptr()) != 0 {
		goto_cleanup_parent(parent);
		return ret;
	}

	i = 0;
	while i < 1000 {
		child = cg_name_indexed(parent, c"child".as_ptr(), i);
		if child.is_null() {
			ret = -1;
			break;
		}

		if cg_create(child) != 0 {
			free(child as *mut c_void);
			break;
		}

		free(child as *mut c_void);
		i += 1;
	}

	if i == 1000 {
		current = cg_read_long(parent, c"memory.current".as_ptr());
		percpu = cg_read_key_long(parent, c"memory.stat".as_ptr(), c"percpu ".as_ptr());
		slab = cg_read_key_long(parent, c"memory.stat".as_ptr(), c"slab ".as_ptr());

		if current > 0
			&& percpu > 0
			&& slab >= 0
			&& labs(current - (percpu + slab)) < max_vmstat_error()
		{
			ret = KSFT_PASS;
		} else {
			printf(
				c"memory.current %ld\npercpu %ld\nslab %ld\ndelta %ld\n".as_ptr(),
				current,
				percpu,
				slab,
				current - (percpu + slab),
			);
		}
	}

	i = 0;
	while i < 1000 {
		child = cg_name_indexed(parent, c"child".as_ptr(), i);
		cg_destroy(child);
		free(child as *mut c_void);
		i += 1;
	}

	goto_cleanup_parent(parent);
	ret
}

static TESTS: [KmemTest; 6] = [
	KmemTest {
		fn_: test_kmem_basic,
		name: c"test_kmem_basic".as_ptr(),
	},
	KmemTest {
		fn_: test_kmem_memcg_deletion,
		name: c"test_kmem_memcg_deletion".as_ptr(),
	},
	KmemTest {
		fn_: test_kmem_proc_kpagecgroup,
		name: c"test_kmem_proc_kpagecgroup".as_ptr(),
	},
	KmemTest {
		fn_: test_kmem_kernel_stacks,
		name: c"test_kmem_kernel_stacks".as_ptr(),
	},
	KmemTest {
		fn_: test_kmem_dead_cgroups,
		name: c"test_kmem_dead_cgroups".as_ptr(),
	},
	KmemTest {
		fn_: test_percpu_basic,
		name: c"test_percpu_basic".as_ptr(),
	},
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
	let mut root = [0 as c_char; PATH_MAX];
	let mut i: usize;

	ksft_print_header();
	ksft_set_plan(TESTS.len() as c_int);
	if cg_find_unified_root(root.as_mut_ptr(), root.len(), ptr::null()) != 0 {
		ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
	}

	/*
	 * Check that memory controller is available:
	 * memory is listed in cgroup.controllers
	 */
	if cg_read_strstr(root.as_ptr(), c"cgroup.controllers".as_ptr(), c"memory".as_ptr()) != 0 {
		ksft_exit_skip(c"memory controller isn't available\n".as_ptr());
	}

	if cg_read_strstr(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"memory".as_ptr()) != 0 {
		if cg_write(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"+memory".as_ptr()) != 0 {
			ksft_exit_skip(c"Failed to set memory controller\n".as_ptr());
		}
	}

	i = 0;
	while i < TESTS.len() {
		match (TESTS[i].fn_)(root.as_ptr()) {
			KSFT_PASS => {
				ksft_test_result_pass(c"%s\n".as_ptr(), TESTS[i].name);
			}
			KSFT_SKIP => {
				ksft_test_result_skip(c"%s\n".as_ptr(), TESTS[i].name);
			}
			_ => {
				ksft_test_result_fail(c"%s\n".as_ptr(), TESTS[i].name);
			}
		}
		i += 1;
	}

	ksft_finished();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
