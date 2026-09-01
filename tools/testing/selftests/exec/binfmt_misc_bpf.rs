// SPDX-License-Identifier: GPL-2.0
/*
 * Selftest for binfmt_misc bpf-backed ('B') handlers.
 *
 * A handler is a struct binfmt_misc_ops struct_ops map with a sleepable match
 * and a sleepable load program. Attaching it publishes it by name in the
 * caller's user namespace; a 'B' entry referencing it by name in the
 * interpreter field activates it:
 *
 *     echo ':name:B::::<handler>:' > /proc/sys/fs/binfmt_misc/register
 *
 * Five self-contained cases are exercised:
 *
 *   1. bpf_interp: the match program matches a synthetic aarch64 ELF header
 *      from the prefetched bprm->buf and the load program routes it to a
 *      fixed interpreter of its choosing.
 *   2. nix_origin: the match program reads the binary's program headers to
 *      commit only to a "$ORIGIN/..."-relative PT_INTERP and the load program
 *      resolves it to an interpreter co-located with the binary (the
 *      relocatable-loader case the kernel ELF loader cannot express).
 *   3. transparent: the load program sets BPF_BINPRM_TRANSPARENT; the
 *      asserting interpreter (binfmt_transparent_interp) verifies the
 *      identity the kernel constructed (exe link, argv, cmdline, comm,
 *      AT_EXECFD, write denial) from inside the process.
 *   4. loader: the load program sets BPF_BINPRM_LOADER; the payload
 *      (binfmt_loader_payload) runs as the main image with the selected
 *      interpreter substituted for its PT_INTERP and asserts the native
 *      identity from inside.
 *   5. interp_bind: an entry registered disabled with 'D' is given its
 *      interpreters one write at a time, and the load program picks one by
 *      name per exec. Replacing what the path holds afterwards changes
 *      nothing, which is the point of binding a file rather than resolving
 *      a name at exec time. Enabling the entry seals it.
 *
 * The first two route to a test interpreter that prints BPF_INTERP_RAN,
 * proving the program's chosen interpreter actually ran.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;
use libc::{FILE, pid_t, size_t, ssize_t};

const INTERP_PATH: &[u8] = b"/tmp/binfmt_bpf_interp\0";
const AARCH64_PATH: &[u8] = b"/tmp/binfmt_bpf_aarch64\0";
const RELOC_TEMPLATE: &[u8] = b"/tmp/binfmt_relocXXXXXX\0";
const TRANS_INTERP: &[u8] = b"/tmp/binfmt_transparent_interp\0";
const TRANS_PATH: &[u8] = b"/tmp/binfmt_bpf_riscv\0";
const EXPECT: &[u8] = b"BPF_INTERP_RAN\0";
const TRANS_EXPECT: &[u8] = b"TRANSPARENT_OK\0";
const LOADER_INTERP: &[u8] = b"/tmp/binfmt_loader_interp\0";
const LOADER_PATH: &[u8] = b"/tmp/binfmt_bpf_loader.ldrtest\0";
const BIND_FIRST: &[u8] = b"/tmp/binfmt_bind_first\0";
const BIND_SECOND: &[u8] = b"/tmp/binfmt_bind_second\0";
const BIND_ARM_PATH: &[u8] = b"/tmp/binfmt_bind_arm\0";
const BIND_RISCV_PATH: &[u8] = b"/tmp/binfmt_bind_riscv\0";
const BIND_EXPECT: &[u8] = b"BIND_RAN \0";
const BIND_MAX: c_int = 100;
const INTERP_LIMIT: &[u8] = b"/proc/sys/user/max_binfmt_misc_interpreters\0";
/* Exit status of the binding child when it cannot set up a budget of its own. */
const BIND_NO_BUDGET: c_int = 200;

const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EV_CURRENT: u8 = 1;
const ET_EXEC: u8 = 2;
const EI_PAD: usize = 9;
const EM_ARM: c_uint = 40;
const EM_AARCH64: c_uint = 183;
const EM_RISCV: c_uint = 243;
const BTF_KIND_STRUCT: c_uint = 4;

#[repr(C)]
struct btf {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut errno: c_int;

	fn btf__load_vmlinux_btf() -> *mut btf;
	fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_uint) -> c_int;
	fn btf__free(btf: *mut btf);
	fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
	fn libbpf_get_error(ptr: *const c_void) -> isize;
	fn bpf_object__load(obj: *mut bpf_object) -> c_int;
	fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
	fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
	fn bpf_link__destroy(link: *mut bpf_link);
	fn bpf_object__close(obj: *mut bpf_object);

	fn artifact_path(dst: *mut c_char, size: size_t, name: *const c_char) -> c_int;
	fn binfmt_misc_available() -> bool;
	fn binfmt_flag_supported(flag: c_char) -> bool;
	fn copy_file(src: *const c_char, dst: *const c_char) -> c_int;
	fn entry_command(entry: *const c_char, cmd: *const c_char) -> c_int;
	fn entry_shows(entry: *const c_char, what: *const c_char) -> bool;
	fn find_loader(buf: *mut c_char, size: size_t) -> c_int;
	fn patch_file(path: *const c_char, off: size_t, bytes: *const c_char, len: size_t) -> c_int;
	fn run_payload(path: *const c_char) -> c_int;
	fn unregister(entry: *const c_char);
	fn write_denied(path: *const c_char) -> bool;
	fn write_reg(rule: *const c_char) -> c_int;

	fn close(fd: c_int) -> c_int;
	fn execl(path: *const c_char, arg0: *const c_char, ...) -> c_int;
	fn _exit(status: c_int) -> !;
	fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
	fn fork() -> pid_t;
	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn getuid() -> c_uint;
	fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
	fn mkdtemp(template: *mut c_char) -> *mut c_char;
	fn open(path: *const c_char, flags: c_int, ...) -> c_int;
	fn pclose(stream: *mut FILE) -> c_int;
	fn popen(command: *const c_char, mode: *const c_char) -> *mut FILE;
	fn rmdir(path: *const c_char) -> c_int;
	fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
	fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
	fn strlen(s: *const c_char) -> size_t;
	fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
	fn unshare(flags: c_int) -> c_int;
	fn unsetenv(name: *const c_char) -> c_int;
	fn unlink(path: *const c_char) -> c_int;
	fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
	fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

	static mut stderr: *mut FILE;
}

const PATH_MAX: usize = libc::PATH_MAX as usize;
const O_WRONLY: c_int = libc::O_WRONLY;
const O_CREAT: c_int = libc::O_CREAT;
const O_EXCL: c_int = libc::O_EXCL;
const O_CLOEXEC: c_int = libc::O_CLOEXEC;
const CLONE_NEWUSER: c_int = libc::CLONE_NEWUSER;
const ENOEXEC: c_int = libc::ENOEXEC;
const ENOENT: c_int = libc::ENOENT;
const EBUSY: c_int = libc::EBUSY;
const EEXIST: c_int = libc::EEXIST;
const EINVAL: c_int = libc::EINVAL;
const ENOSPC: c_int = libc::ENOSPC;
const BINFMT_DIR: &[u8] = b"/proc/sys/fs/binfmt_misc\0";
const LOADER_MARKER: &[u8] = b"LOADER_MARKER\0";

fn WIFEXITED(status: c_int) -> bool {
	(status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
	(status & 0xff00) >> 8
}

/* A minimal 64-bit little-endian ELF header, padded to the read size. */
unsafe fn create_fake_elf(path: *const c_char, machine: c_uint) -> c_int {
	let mut hdr = [0u8; 256];
	let fd: c_int;

	hdr[0] = 0x7f;
	hdr[1] = b'E';
	hdr[2] = b'L';
	hdr[3] = b'F';
	hdr[4] = ELFCLASS64;
	hdr[5] = ELFDATA2LSB;
	hdr[6] = EV_CURRENT;
	hdr[16] = ET_EXEC;
	hdr[18] = (machine & 0xff) as u8;	/* e_machine, little-endian */
	hdr[19] = (machine >> 8) as u8;
	hdr[20] = EV_CURRENT;

	unlink(path);
	fd = open(path, O_WRONLY | O_CREAT | O_EXCL, 0o755);
	if fd < 0 {
		return -1;
	}
	if write(fd, hdr.as_ptr() as *const c_void, hdr.len()) != hdr.len() as ssize_t {
		close(fd);
		return -1;
	}
	close(fd);
	0
}

/*
 * Register a 'B' entry for @handler. With @flags "D" the entry is created
 * disabled, which is what leaves it open to being given interpreters.
 */
unsafe fn register_entry(name: *const c_char, handler: *const c_char, flags: *const c_char) -> c_int {
	let mut rule = [0 as c_char; PATH_MAX];

	snprintf(
		rule.as_mut_ptr(),
		rule.len(),
		b":%s:B::::%s:%s\0".as_ptr() as *const c_char,
		name,
		handler,
		if !flags.is_null() { flags } else { b"\0".as_ptr() as *const c_char },
	);
	write_reg(rule.as_ptr())
}

unsafe fn check_output(cmd: *const c_char, expected: *const c_char) -> c_int {
	let mut buf = [0 as c_char; 128];
	let fp: *mut FILE;

	fp = popen(cmd, b"r\0".as_ptr() as *const c_char);
	if fp.is_null() {
		return -1;
	}
	if fgets(buf.as_mut_ptr(), buf.len() as c_int, fp).is_null() {
		pclose(fp);
		return -1;
	}
	pclose(fp);
	if strncmp(buf.as_ptr(), expected, strlen(expected)) != 0 { -1 } else { 0 }
}

/* Does the kernel BTF know struct binfmt_misc_ops (CONFIG_BINFMT_MISC_BPF)? */
unsafe fn have_binfmt_misc_ops() -> bool {
	let btf = btf__load_vmlinux_btf();
	let have: bool;

	have = !btf.is_null()
		&& btf__find_by_name_kind(
			btf,
			b"binfmt_misc_ops\0".as_ptr() as *const c_char,
			BTF_KIND_STRUCT,
		) >= 0;
	btf__free(btf);
	have
}

/* The reason bpf handler cases cannot run here, NULL if they can. */
unsafe fn bpf_handler_unsupported() -> *const c_char {
	if getuid() != 0 {
		return b"test must be run as root\0".as_ptr() as *const c_char;
	}
	if !have_binfmt_misc_ops() {
		return b"no struct binfmt_misc_ops in the kernel BTF (CONFIG_BINFMT_MISC_BPF)\0".as_ptr()
			as *const c_char;
	}
	if !binfmt_misc_available() {
		return b"no binfmt_misc\0".as_ptr() as *const c_char;
	}
	ptr::null()
}

/* An attached handler with its 'B' entry activated. */
#[repr(C)]
struct bpf_case {
	obj: *mut bpf_object,
	link: *mut bpf_link,
	entry: *const c_char,
}

/*
 * Load @objfile, attach its struct_ops map @handler (which publishes the
 * handler) and register a 'B' entry named @entry that references it, with
 * @flags as the entry's register-string flags.
 */
unsafe fn bpf_case_start_flags(
	c: *mut bpf_case,
	objfile: *const c_char,
	handler: *const c_char,
	entry: *const c_char,
	flags: *const c_char,
) -> c_int {
	let map: *mut bpf_map;

	(*c).obj = ptr::null_mut();
	(*c).link = ptr::null_mut();
	(*c).entry = entry;

	(*c).obj = bpf_object__open_file(objfile, ptr::null());
	if (*c).obj.is_null() || libbpf_get_error((*c).obj as *const c_void) != 0 {
		fprintf(stderr, b"open %s failed\n\0".as_ptr() as *const c_char, objfile);
		(*c).obj = ptr::null_mut();
		return -1;
	}
	if bpf_object__load((*c).obj) != 0 {
		fprintf(
			stderr,
			b"load %s failed (check dmesg for the verifier log)\n\0".as_ptr() as *const c_char,
			objfile,
		);
		bpf_link__destroy((*c).link);
		bpf_object__close((*c).obj);
		(*c).obj = ptr::null_mut();
		(*c).link = ptr::null_mut();
		return -1;
	}
	map = bpf_object__find_map_by_name((*c).obj, handler);
	if map.is_null() {
		fprintf(
			stderr,
			b"no struct_ops map '%s' in %s\n\0".as_ptr() as *const c_char,
			handler,
			objfile,
		);
		bpf_link__destroy((*c).link);
		bpf_object__close((*c).obj);
		(*c).obj = ptr::null_mut();
		(*c).link = ptr::null_mut();
		return -1;
	}
	(*c).link = bpf_map__attach_struct_ops(map);
	if (*c).link.is_null() || libbpf_get_error((*c).link as *const c_void) != 0 {
		fprintf(stderr, b"attach struct_ops '%s' failed\n\0".as_ptr() as *const c_char, handler);
		(*c).link = ptr::null_mut();
		bpf_link__destroy((*c).link);
		bpf_object__close((*c).obj);
		(*c).obj = ptr::null_mut();
		(*c).link = ptr::null_mut();
		return -1;
	}
	if register_entry(entry, handler, flags) != 0 {
		fprintf(stderr, b"register 'B' entry '%s' failed\n\0".as_ptr() as *const c_char, entry);
		bpf_link__destroy((*c).link);
		bpf_object__close((*c).obj);
		(*c).obj = ptr::null_mut();
		(*c).link = ptr::null_mut();
		return -1;
	}
	0
}

unsafe fn bpf_case_start(
	c: *mut bpf_case,
	objfile: *const c_char,
	handler: *const c_char,
	entry: *const c_char,
) -> c_int {
	bpf_case_start_flags(c, objfile, handler, entry, ptr::null())
}

unsafe fn bpf_case_stop(c: *mut bpf_case) {
	unregister((*c).entry);
	bpf_link__destroy((*c).link);
	bpf_object__close((*c).obj);
}

/* Activate @handler, run @target and check it produced @expect. */
unsafe fn run_case(
	objfile: *const c_char,
	handler: *const c_char,
	entry: *const c_char,
	target: *const c_char,
	expect: *const c_char,
) -> c_int {
	let mut c = bpf_case { obj: ptr::null_mut(), link: ptr::null_mut(), entry: ptr::null() };
	let ret: c_int;

	if bpf_case_start(&mut c, objfile, handler, entry) != 0 {
		return -1;
	}
	ret = check_output(target, expect);
	bpf_case_stop(&mut c);
	ret
}

#[repr(C)]
struct bpf_handler {
	obj: [c_char; PATH_MAX],	/* struct_ops object of the case under test */
}

FIXTURE!(bpf_handler);

FIXTURE_SETUP!(bpf_handler, |self_: *mut bpf_handler| unsafe {
	let mut src = [0 as c_char; PATH_MAX];
	let why = bpf_handler_unsupported();

	if !why.is_null() {
		SKIP!(return, b"%s\0".as_ptr() as *const c_char, why);
	}

	/* Shared test interpreter. */
	ASSERT_EQ!(artifact_path(src.as_mut_ptr(), src.len(), b"binfmt_bpf_interp\0".as_ptr() as *const c_char), 0);
	ASSERT_EQ!(copy_file(src.as_ptr(), INTERP_PATH.as_ptr() as *const c_char), 0);
});

FIXTURE_TEARDOWN!(bpf_handler, |_self: *mut bpf_handler| unsafe {
	unlink(INTERP_PATH.as_ptr() as *const c_char);
});

/* The match program matches a synthetic header, the load program routes it. */
TEST_F!(bpf_handler, fixed_interpreter, |self_: *mut bpf_handler| unsafe {
	ASSERT_EQ!(create_fake_elf(AARCH64_PATH.as_ptr() as *const c_char, EM_AARCH64), 0);
	ASSERT_EQ!(artifact_path((*self_).obj.as_mut_ptr(), (*self_).obj.len(), b"bpf_interp.bpf.o\0".as_ptr() as *const c_char), 0);
	EXPECT_EQ!(run_case((*self_).obj.as_ptr(), b"bpf_interp\0".as_ptr() as *const c_char, b"test_bpf_interp\0".as_ptr() as *const c_char, AARCH64_PATH.as_ptr() as *const c_char, EXPECT.as_ptr() as *const c_char), 0);
	unlink(AARCH64_PATH.as_ptr() as *const c_char);
});

/* A "$ORIGIN/..." PT_INTERP resolved to an interpreter next to the binary. */
TEST_F!(bpf_handler, origin_relative_interpreter, |self_: *mut bpf_handler| unsafe {
	let mut src = [0 as c_char; PATH_MAX];
	let mut app = [0 as c_char; PATH_MAX];
	let mut interp = [0 as c_char; PATH_MAX];
	let mut dir = [0 as c_char; PATH_MAX];
	memcpy(dir.as_mut_ptr() as *mut c_void, RELOC_TEMPLATE.as_ptr() as *const c_void, RELOC_TEMPLATE.len());

	ASSERT_NE!(mkdtemp(dir.as_mut_ptr()), ptr::null_mut());
	snprintf(app.as_mut_ptr(), app.len(), b"%s/app\0".as_ptr() as *const c_char, dir.as_ptr());
	snprintf(interp.as_mut_ptr(), interp.len(), b"%s/binfmt_bpf_interp\0".as_ptr() as *const c_char, dir.as_ptr());
	ASSERT_EQ!(artifact_path(src.as_mut_ptr(), src.len(), b"binfmt_bpf_app\0".as_ptr() as *const c_char), 0);
	ASSERT_EQ!(copy_file(src.as_ptr(), app.as_ptr()), 0);
	ASSERT_EQ!(copy_file(INTERP_PATH.as_ptr() as *const c_char, interp.as_ptr()), 0);

	ASSERT_EQ!(artifact_path((*self_).obj.as_mut_ptr(), (*self_).obj.len(), b"nix_origin.bpf.o\0".as_ptr() as *const c_char), 0);
	EXPECT_EQ!(run_case((*self_).obj.as_ptr(), b"nix_origin\0".as_ptr() as *const c_char, b"test_bpf_origin\0".as_ptr() as *const c_char, app.as_ptr(), EXPECT.as_ptr() as *const c_char), 0);

	unlink(app.as_ptr());
	unlink(interp.as_ptr());
	rmdir(dir.as_ptr());
});

/* A transparent dispatch: the process presents as the binary, not the interp. */
TEST_F!(bpf_handler, transparent_dispatch, |self_: *mut bpf_handler| unsafe {
	let mut src = [0 as c_char; PATH_MAX];
	let mut cmd = [0 as c_char; PATH_MAX + 16];

	/* Probe for transparent-mode support via its static counterpart. */
	if !binfmt_flag_supported(b'T' as c_char) {
		SKIP!(return, b"kernel without transparent mode\0".as_ptr() as *const c_char);
	}

	ASSERT_EQ!(artifact_path(src.as_mut_ptr(), src.len(), b"binfmt_transparent_interp\0".as_ptr() as *const c_char), 0);
	ASSERT_EQ!(copy_file(src.as_ptr(), TRANS_INTERP.as_ptr() as *const c_char), 0);
	ASSERT_EQ!(create_fake_elf(TRANS_PATH.as_ptr() as *const c_char, EM_RISCV), 0);

	setenv(b"BINFMT_TEST_BINARY\0".as_ptr() as *const c_char, TRANS_PATH.as_ptr() as *const c_char, 1);
	snprintf(cmd.as_mut_ptr(), cmd.len(), b"%s argone argtwo\0".as_ptr() as *const c_char, TRANS_PATH.as_ptr() as *const c_char);
	ASSERT_EQ!(artifact_path((*self_).obj.as_mut_ptr(), (*self_).obj.len(), b"transparent.bpf.o\0".as_ptr() as *const c_char), 0);
	EXPECT_EQ!(run_case((*self_).obj.as_ptr(), b"transparent\0".as_ptr() as *const c_char, b"test_bpf_transparent\0".as_ptr() as *const c_char, cmd.as_ptr(), TRANS_EXPECT.as_ptr() as *const c_char), 0);

	unlink(TRANS_PATH.as_ptr() as *const c_char);
	unlink(TRANS_INTERP.as_ptr() as *const c_char);
});

/* A per-exec loader substitution: the payload runs as a native exec. */
TEST_F!(bpf_handler, loader_substitution, |self_: *mut bpf_handler| unsafe {
	let mut src = [0 as c_char; PATH_MAX];
	let mut loader = [0 as c_char; PATH_MAX];
	let mut c = bpf_case { obj: ptr::null_mut(), link: ptr::null_mut(), entry: ptr::null() };
	let status: c_int;

	if find_loader(loader.as_mut_ptr(), loader.len()) != 0 {
		SKIP!(return, b"cannot determine own PT_INTERP\0".as_ptr() as *const c_char);
	}

	ASSERT_EQ!(copy_file(loader.as_ptr(), LOADER_INTERP.as_ptr() as *const c_char), 0);
	ASSERT_EQ!(artifact_path(src.as_mut_ptr(), src.len(), b"binfmt_loader_payload\0".as_ptr() as *const c_char), 0);
	ASSERT_EQ!(copy_file(src.as_ptr(), LOADER_PATH.as_ptr() as *const c_char), 0);
	ASSERT_EQ!(patch_file(LOADER_PATH.as_ptr() as *const c_char, EI_PAD, LOADER_MARKER.as_ptr() as *const c_char, strlen(LOADER_MARKER.as_ptr() as *const c_char)), 0);
	ASSERT_EQ!(artifact_path((*self_).obj.as_mut_ptr(), (*self_).obj.len(), b"loader.bpf.o\0".as_ptr() as *const c_char), 0);

	setenv(b"BINFMT_TEST_BINARY\0".as_ptr() as *const c_char, LOADER_PATH.as_ptr() as *const c_char, 1);
	setenv(b"BINFMT_TEST_INTERP\0".as_ptr() as *const c_char, LOADER_INTERP.as_ptr() as *const c_char, 1);

	ASSERT_EQ!(bpf_case_start(&mut c, (*self_).obj.as_ptr(), b"loader\0".as_ptr() as *const c_char, b"test_bpf_loader\0".as_ptr() as *const c_char), 0);
	status = run_payload(LOADER_PATH.as_ptr() as *const c_char);
	bpf_case_stop(&mut c);
	EXPECT_EQ!(status, 0);

	unsetenv(b"BINFMT_TEST_INTERP\0".as_ptr() as *const c_char);
	unlink(LOADER_PATH.as_ptr() as *const c_char);
	unlink(LOADER_INTERP.as_ptr() as *const c_char);
});

/* The errno an exec of @path fails with, 0 if it succeeded. */
unsafe fn exec_errno(path: *const c_char) -> c_int {
	let mut status: c_int = 0;
	let pid: pid_t;

	pid = fork();
	if pid == 0 {
		execl(path, path, ptr::null::<c_char>());
		_exit(errno);
	}
	if pid < 0 || waitpid(pid, &mut status, 0) != pid || !WIFEXITED(status) {
		return -1;
	}
	WEXITSTATUS(status)
}

/* Install a copy of the bound-interpreter test binary at @path. */
unsafe fn install_interp(path: *const c_char) -> c_int {
	let mut src = [0 as c_char; PATH_MAX];

	if artifact_path(src.as_mut_ptr(), src.len(), b"binfmt_bind_interp\0".as_ptr() as *const c_char) != 0 {
		return -1;
	}
	copy_file(src.as_ptr(), path)
}

/* Bind @path to @entry under @name, the '+' command of a disabled entry. */
unsafe fn entry_bind(entry: *const c_char, name: *const c_char, path: *const c_char) -> c_int {
	let mut cmd = [0 as c_char; PATH_MAX];

	snprintf(cmd.as_mut_ptr(), cmd.len(), b"+%s %s\n\0".as_ptr() as *const c_char, name, path);
	entry_command(entry, cmd.as_ptr())
}

/* Set the interpreter budget of this namespace. */
unsafe fn write_interp_limit(val: *const c_char) -> c_int {
	let n: ssize_t;
	let fd: c_int;

	fd = open(INTERP_LIMIT.as_ptr() as *const c_char, O_WRONLY | O_CLOEXEC);
	if fd < 0 {
		return -1;
	}
	n = write(fd, val as *const c_void, strlen(val));
	close(fd);
	if n < 0 { -1 } else { 0 }
}

/*
 * The errno a bind is refused with when the writer is a child that has spent
 * the budget of a user namespace of its own, 0 if it succeeded and -1 if the
 * child could not set itself up. The fd is opened here and inherited, so the
 * interpreter is still opened with this process's credentials.
 */
unsafe fn bind_out_of_budget(entry: *const c_char, name: *const c_char, path: *const c_char) -> c_int {
	let mut cmd = [0 as c_char; PATH_MAX];
	let mut file = [0 as c_char; PATH_MAX];
	let fd: c_int;
	let mut status: c_int = 0;
	let retval: c_int;
	let pid: pid_t;

	snprintf(file.as_mut_ptr(), file.len(), b"%s/%s\0".as_ptr() as *const c_char, BINFMT_DIR.as_ptr() as *const c_char, entry);
	snprintf(cmd.as_mut_ptr(), cmd.len(), b"+%s %s\n\0".as_ptr() as *const c_char, name, path);

	fd = open(file.as_ptr(), O_WRONLY | O_CLOEXEC);
	if fd < 0 {
		return -1;
	}

	pid = fork();
	if pid == 0 {
		let n: ssize_t;

		/* A namespace of its own, with nothing left in it to spend. */
		if unshare(CLONE_NEWUSER) != 0 || write_interp_limit(b"0\0".as_ptr() as *const c_char) != 0 {
			_exit(BIND_NO_BUDGET);
		}
		n = write(fd, cmd.as_ptr() as *const c_void, strlen(cmd.as_ptr()));
		_exit(if n < 0 { errno } else { 0 });
	}
	close(fd);
	if pid < 0 || waitpid(pid, &mut status, 0) != pid || !WIFEXITED(status) {
		return -1;
	}
	retval = WEXITSTATUS(status);
	if retval == BIND_NO_BUDGET { -1 } else { retval }
}

#[repr(C)]
struct bound_interp {
	obj: [c_char; PATH_MAX],
	c: bpf_case,
	started: bool,
}

FIXTURE!(bound_interp);

FIXTURE_SETUP!(bound_interp, |self_: *mut bound_interp| unsafe {
	let why = bpf_handler_unsupported();

	if !why.is_null() {
		SKIP!(return, b"%s\0".as_ptr() as *const c_char, why);
	}
	if !binfmt_flag_supported(b'D' as c_char) {
		ASSERT_EQ!(errno, EINVAL);
		SKIP!(return, b"kernel without the 'D' flag\0".as_ptr() as *const c_char);
	}

	ASSERT_EQ!(install_interp(BIND_FIRST.as_ptr() as *const c_char), 0);
	ASSERT_EQ!(install_interp(BIND_SECOND.as_ptr() as *const c_char), 0);

	ASSERT_EQ!(artifact_path((*self_).obj.as_mut_ptr(), (*self_).obj.len(), b"interp_bind.bpf.o\0".as_ptr() as *const c_char), 0);

	/*
	 * Registered disabled, so it cannot be matched yet and can still be
	 * given interpreters. Each path is resolved once, by its write(2);
	 * from here on the entry holds the files themselves.
	 */
	ASSERT_EQ!(bpf_case_start_flags(&mut (*self_).c, (*self_).obj.as_ptr(), b"interp_bind\0".as_ptr() as *const c_char, b"test_interp_bind\0".as_ptr() as *const c_char, b"D\0".as_ptr() as *const c_char), 0);
	(*self_).started = true;

	ASSERT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"first\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char), 0);
	ASSERT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"second\0".as_ptr() as *const c_char, BIND_SECOND.as_ptr() as *const c_char), 0);
});

FIXTURE_TEARDOWN!(bound_interp, |self_: *mut bound_interp| unsafe {
	if (*self_).started {
		bpf_case_stop(&mut (*self_).c);
	}
	unlink(BIND_FIRST.as_ptr() as *const c_char);
	unlink(BIND_SECOND.as_ptr() as *const c_char);
	unlink(AARCH64_PATH.as_ptr() as *const c_char);
	unlink(BIND_RISCV_PATH.as_ptr() as *const c_char);
	unlink(BIND_ARM_PATH.as_ptr() as *const c_char);
});

/* Enabling is what makes the configured entry matchable. */
unsafe fn activate(entry: *const c_char) -> c_int {
	entry_command(entry, b"1\n\0".as_ptr() as *const c_char)
}

/* One entry, one interpreter per guest architecture, picked per exec. */
TEST_F!(bound_interp, selects_by_name, |_self: *mut bound_interp| unsafe {
	ASSERT_EQ!(create_fake_elf(AARCH64_PATH.as_ptr() as *const c_char, EM_AARCH64), 0);
	ASSERT_EQ!(create_fake_elf(BIND_RISCV_PATH.as_ptr() as *const c_char, EM_RISCV), 0);

	/* Disabled, so it does not match and no format claims the binary. */
	EXPECT_EQ!(exec_errno(AARCH64_PATH.as_ptr() as *const c_char), ENOEXEC);

	ASSERT_EQ!(activate(b"test_interp_bind\0".as_ptr() as *const c_char), 0);
	EXPECT_EQ!(check_output(AARCH64_PATH.as_ptr() as *const c_char, b"BIND_RAN /tmp/binfmt_bind_first\0".as_ptr() as *const c_char), 0);
	EXPECT_EQ!(check_output(BIND_RISCV_PATH.as_ptr() as *const c_char, b"BIND_RAN /tmp/binfmt_bind_second\0".as_ptr() as *const c_char), 0);
});

/* What was bound is what runs, whatever the path holds afterwards. */
TEST_F!(bound_interp, path_no_longer_decides, |_self: *mut bound_interp| unsafe {
	let mut other = [0 as c_char; PATH_MAX];

	ASSERT_EQ!(create_fake_elf(AARCH64_PATH.as_ptr() as *const c_char, EM_AARCH64), 0);
	ASSERT_EQ!(activate(b"test_interp_bind\0".as_ptr() as *const c_char), 0);

	/* Bound interpreters are pinned against writes, exactly like 'F'. */
	EXPECT_TRUE!(write_denied(BIND_FIRST.as_ptr() as *const c_char));

	/* Replace the path with a different binary: a new file, new inode. */
	ASSERT_EQ!(artifact_path(other.as_mut_ptr(), other.len(), b"binfmt_bpf_interp\0".as_ptr() as *const c_char), 0);
	ASSERT_EQ!(unlink(BIND_FIRST.as_ptr() as *const c_char), 0);
	ASSERT_EQ!(copy_file(other.as_ptr(), BIND_FIRST.as_ptr() as *const c_char), 0);

	EXPECT_EQ!(check_output(AARCH64_PATH.as_ptr() as *const c_char, b"BIND_RAN /tmp/binfmt_bind_first\0".as_ptr() as *const c_char), 0);
});

/* The entry reports what it bound, under the names it bound them as. */
TEST_F!(bound_interp, entry_reports_bindings, |_self: *mut bound_interp| unsafe {
	EXPECT_TRUE!(entry_shows(b"test_interp_bind\0".as_ptr() as *const c_char, b"bpf-interpreter first /tmp/binfmt_bind_first\0".as_ptr() as *const c_char));
	EXPECT_TRUE!(entry_shows(b"test_interp_bind\0".as_ptr() as *const c_char, b"bpf-interpreter second /tmp/binfmt_bind_second\0".as_ptr() as *const c_char));
});

/* Selecting a name the entry did not bind fails the exec. */
TEST_F!(bound_interp, unbound_name_fails, |_self: *mut bound_interp| unsafe {
	ASSERT_EQ!(create_fake_elf(BIND_ARM_PATH.as_ptr() as *const c_char, EM_ARM), 0);
	ASSERT_EQ!(activate(b"test_interp_bind\0".as_ptr() as *const c_char), 0);

	EXPECT_EQ!(exec_errno(BIND_ARM_PATH.as_ptr() as *const c_char), ENOENT);
});

/* Activating seals it: what can be matched cannot be changed. */
TEST_F!(bound_interp, sealed_once_active, |_self: *mut bound_interp| unsafe {
	ASSERT_EQ!(activate(b"test_interp_bind\0".as_ptr() as *const c_char), 0);

	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"third\0".as_ptr() as *const c_char, BIND_SECOND.as_ptr() as *const c_char), -EBUSY);
	EXPECT_FALSE!(entry_shows(b"test_interp_bind\0".as_ptr() as *const c_char, b"bpf-interpreter third /tmp/binfmt_bind_second\0".as_ptr() as *const c_char));
});

/* The seal is for good: disabling the entry again reopens nothing. */
TEST_F!(bound_interp, disable_does_not_unseal, |_self: *mut bound_interp| unsafe {
	ASSERT_EQ!(activate(b"test_interp_bind\0".as_ptr() as *const c_char), 0);
	ASSERT_EQ!(entry_command(b"test_interp_bind\0".as_ptr() as *const c_char, b"0\n\0".as_ptr() as *const c_char), 0);

	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"third\0".as_ptr() as *const c_char, BIND_SECOND.as_ptr() as *const c_char), -EBUSY);
});

/* An entry registered without 'D' is sealed from the start. */
TEST_F!(bound_interp, born_sealed, |_self: *mut bound_interp| unsafe {
	/* A second entry for the handler the fixture already published. */
	ASSERT_EQ!(register_entry(b"test_born_sealed\0".as_ptr() as *const c_char, b"interp_bind\0".as_ptr() as *const c_char, ptr::null()), 0);

	EXPECT_EQ!(entry_bind(b"test_born_sealed\0".as_ptr() as *const c_char, b"first\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char), -EBUSY);
	unregister(b"test_born_sealed\0".as_ptr() as *const c_char);
});

/* A name is bound once; a second use of it is refused. */
TEST_F!(bound_interp, duplicate_name_refused, |_self: *mut bound_interp| unsafe {
	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"first\0".as_ptr() as *const c_char, BIND_SECOND.as_ptr() as *const c_char), -EEXIST);
});

/* A name is a printable word: the entry file reports 'name path' lines. */
TEST_F!(bound_interp, name_must_be_printable, |_self: *mut bound_interp| unsafe {
	/* A control character would forge a line into the entry file. */
	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"a\tb\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char), -EINVAL);
	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"a\nb\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char), -EINVAL);

	/* A space cannot even be spelled: the path starts after the first one. */
	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"a b\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char), -EINVAL);
});

/* The command ends at the write: bytes past an embedded nul are refused. */
TEST_F!(bound_interp, trailing_bytes_refused, |_self: *mut bound_interp| unsafe {
	let mut cmd = [0 as c_char; PATH_MAX];
	let mut len: size_t;
	let fd: c_int;

	/* entry_command() cannot spell a nul, so write the buffer raw. */
	snprintf(cmd.as_mut_ptr(), cmd.len(), b"+nul %s\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char);
	len = strlen(cmd.as_ptr()) + 1;
	memcpy(cmd.as_mut_ptr().add(len) as *mut c_void, b"junk\0".as_ptr() as *const c_void, b"junk\0".len());
	len += b"junk\0".len();

	fd = open(b"/proc/sys/fs/binfmt_misc/test_interp_bind\0".as_ptr() as *const c_char, O_WRONLY | O_CLOEXEC);
	ASSERT_GE!(fd, 0);
	EXPECT_EQ!(write(fd, cmd.as_ptr() as *const c_void, len), -1);
	EXPECT_EQ!(errno, EINVAL);
	close(fd);

	EXPECT_FALSE!(entry_shows(b"test_interp_bind\0".as_ptr() as *const c_char, b"bpf-interpreter nul /tmp/binfmt_bind_first\0".as_ptr() as *const c_char));
});

/* An entry binds at most BIND_MAX interpreters. */
TEST_F!(bound_interp, capped_bindings, |_self: *mut bound_interp| unsafe {
	let mut name = [0 as c_char; 16];
	let mut i: c_int;

	/* The fixture bound "first" and "second" already. */
	i = 2;
	while i < BIND_MAX {
		snprintf(name.as_mut_ptr(), name.len(), b"n%d\0".as_ptr() as *const c_char, i);
		ASSERT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, name.as_ptr(), BIND_FIRST.as_ptr() as *const c_char), 0);
		i += 1;
	}
	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"over\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char), -ENOSPC);
});

/* A binding pins a file: it is charged, and refused once the budget is out. */
TEST_F!(bound_interp, bindings_are_charged, |_self: *mut bound_interp| unsafe {
	let err = bind_out_of_budget(b"test_interp_bind\0".as_ptr() as *const c_char, b"third\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char);

	if err < 0 {
		SKIP!(return, b"no user namespaces or no /proc/sys/user/max_binfmt_misc_interpreters\0".as_ptr() as *const c_char);
	}

	/* The charge follows the writer, not the entry file it writes to. */
	EXPECT_EQ!(err, ENOSPC);

	/* The budget was the only thing in the way. */
	EXPECT_EQ!(entry_bind(b"test_interp_bind\0".as_ptr() as *const c_char, b"third\0".as_ptr() as *const c_char, BIND_FIRST.as_ptr() as *const c_char), 0);
});

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
