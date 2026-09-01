// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */
/* Translated from testing/selftests/bpf/prog_tests/token.c. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type uid_t = u32;
type gid_t = u32;

#[repr(C)]
pub struct btf {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_insn {
	pub code: u8,
	pub dst_src: u8,
	pub off: i16,
	pub imm: i32,
}

#[repr(C)]
pub struct bpf_map_create_opts {
	pub sz: size_t,
	pub map_flags: __u32,
	pub token_fd: __u32,
}

#[repr(C)]
pub struct bpf_btf_load_opts {
	pub sz: size_t,
	pub btf_flags: __u32,
	pub token_fd: __u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
	pub sz: size_t,
	pub prog_flags: __u32,
	pub expected_attach_type: __u32,
	pub token_fd: __u32,
}

#[repr(C)]
pub struct bpf_object_open_opts {
	pub sz: size_t,
	pub bpf_token_path: *const c_char,
}

#[repr(C)]
pub struct bpf_token_info {
	pub allowed_cmds: __u64,
	pub allowed_maps: __u64,
	pub allowed_progs: __u64,
	pub allowed_attachs: __u64,
}

#[repr(C)]
pub struct token_lsm_bss {
	pub my_pid: pid_t,
	pub reject_capable: bool,
	pub reject_cmd: bool,
}

#[repr(C)]
pub struct token_lsm {
	pub bss: *mut token_lsm_bss,
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
pub struct priv_prog_progs {
	pub xdp_prog1: *mut bpf_program,
}

#[repr(C)]
pub struct priv_prog {
	pub progs: priv_prog_progs,
}

#[repr(C)]
pub struct priv_freplace_prog_progs {
	pub new_xdp_prog2: *mut bpf_program,
}

#[repr(C)]
pub struct priv_freplace_prog {
	pub obj: *mut bpf_object,
	pub progs: priv_freplace_prog_progs,
}

#[repr(C)]
pub struct priv_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dummy_st_ops_success {
	_private: [u8; 0],
}

#[repr(C)]
pub struct token_kallsyms {
	_private: [u8; 0],
}

#[repr(C)]
pub struct ksym {
	pub name: *const c_char,
}

#[repr(C)]
pub struct ksyms {
	pub sym_cnt: c_int,
	pub syms: *mut ksym,
}

#[repr(C)]
pub struct iovec {
	pub iov_base: *mut c_void,
	pub iov_len: size_t,
}

#[repr(C)]
pub struct msghdr {
	pub msg_name: *mut c_void,
	pub msg_namelen: c_uint,
	pub msg_iov: *mut iovec,
	pub msg_iovlen: size_t,
	pub msg_control: *mut c_void,
	pub msg_controllen: size_t,
	pub msg_flags: c_int,
}

#[repr(C)]
pub struct cmsghdr {
	pub cmsg_len: size_t,
	pub cmsg_level: c_int,
	pub cmsg_type: c_int,
}

#[repr(C)]
union cmsg_buf {
	pub buf: [c_char; 64],
	pub align: cmsghdr,
}

#[repr(C)]
struct bpffs_opts {
	cmds: __u64,
	maps: __u64,
	progs: __u64,
	attachs: __u64,
	cmds_str: *const c_char,
	maps_str: *const c_char,
	progs_str: *const c_char,
	attachs_str: *const c_char,
}

type child_callback_fn = unsafe extern "C" fn(c_int, *mut token_lsm) -> c_int;

const TOKEN_ENVVAR: *const c_char = b"LIBBPF_BPF_TOKEN_PATH\0".as_ptr() as *const c_char;

unsafe extern "C" {
	fn syscall(num: c_long, ...) -> c_long;
	fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
	fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
	fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
	fn strlen(s: *const c_char) -> size_t;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
	fn getenv(name: *const c_char) -> *mut c_char;
	fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
	fn unsetenv(name: *const c_char) -> c_int;
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
	fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
	fn unshare(flags: c_int) -> c_int;
	fn getuid() -> uid_t;
	fn getgid() -> gid_t;
	fn getpid() -> pid_t;
	fn setuid(uid: uid_t) -> c_int;
	fn setgid(gid: gid_t) -> c_int;
	fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
	fn fork() -> pid_t;
	fn exit(status: c_int) -> !;
	fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
	fn kill(pid: pid_t, sig: c_int) -> c_int;
	fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
	fn rmdir(pathname: *const c_char) -> c_int;
	fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
	fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
	fn __errno_location() -> *mut c_int;

	fn cap_disable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
	fn cap_enable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
	fn bpf_token_create(bpffs_fd: c_int, opts: *const c_void) -> c_int;
	fn bpf_map_create(t: c_int, name: *const c_char, key_size: c_int, value_size: c_int, max_entries: c_int, opts: *const bpf_map_create_opts) -> c_int;
	fn bpf_btf_load(data: *const c_void, size: __u32, opts: *const bpf_btf_load_opts) -> c_int;
	fn bpf_prog_load(t: c_int, name: *const c_char, license: *const c_char, insns: *const bpf_insn, insn_cnt: size_t, opts: *const bpf_prog_load_opts) -> c_int;
	fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut bpf_token_info, len: *mut __u32) -> c_int;
	fn btf__new_empty() -> *mut btf;
	fn btf__add_int(btf: *mut btf, name: *const c_char, byte_sz: c_int, encoding: c_int) -> c_int;
	fn btf__raw_data(btf: *mut btf, size: *mut __u32) -> *const c_void;
	fn btf__free(btf: *mut btf);
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_object__prepare(obj: *mut bpf_object) -> c_int;
	fn bpf_program__set_attach_target(prog: *mut bpf_program, fd: c_int, name: *const c_char) -> c_int;
	fn test__start_subtest(name: *const c_char) -> bool;
	fn sysctl_set_or_fail(path: *const c_char, old: *mut c_char, new: *const c_char) -> c_int;

	fn token_lsm__open_and_load() -> *mut token_lsm;
	fn token_lsm__attach(skel: *mut token_lsm) -> c_int;
	fn token_lsm__destroy(skel: *mut token_lsm);
	fn priv_map__open_and_load() -> *mut priv_map;
	fn priv_map__open_opts(opts: *const bpf_object_open_opts) -> *mut priv_map;
	fn priv_map__load(skel: *mut priv_map) -> c_int;
	fn priv_map__destroy(skel: *mut priv_map);
	fn priv_prog__open_and_load() -> *mut priv_prog;
	fn priv_prog__open_opts(opts: *const bpf_object_open_opts) -> *mut priv_prog;
	fn priv_prog__load(skel: *mut priv_prog) -> c_int;
	fn priv_prog__destroy(skel: *mut priv_prog);
	fn priv_freplace_prog__open_opts(opts: *const bpf_object_open_opts) -> *mut priv_freplace_prog;
	fn priv_freplace_prog__load(skel: *mut priv_freplace_prog) -> c_int;
	fn priv_freplace_prog__destroy(skel: *mut priv_freplace_prog);
	fn dummy_st_ops_success__open_and_load() -> *mut dummy_st_ops_success;
	fn dummy_st_ops_success__open_opts(opts: *const bpf_object_open_opts) -> *mut dummy_st_ops_success;
	fn dummy_st_ops_success__load(skel: *mut dummy_st_ops_success) -> c_int;
	fn dummy_st_ops_success__destroy(skel: *mut dummy_st_ops_success);
	fn token_kallsyms__open_opts(opts: *const bpf_object_open_opts) -> *mut token_kallsyms;
	fn token_kallsyms__load(skel: *mut token_kallsyms) -> c_int;
	fn token_kallsyms__destroy(skel: *mut token_kallsyms);
	fn load_kallsyms_local() -> *mut ksyms;
	fn free_kallsyms_local(ksyms: *mut ksyms);
}

macro_rules! ASSERT_OK { ($expr:expr, $name:expr) => { $expr == 0 }; }
macro_rules! ASSERT_ERR { ($expr:expr, $name:expr) => { $expr != 0 }; }
macro_rules! ASSERT_GE { ($a:expr, $b:expr, $name:expr) => { $a >= $b }; }
macro_rules! ASSERT_GT { ($a:expr, $b:expr, $name:expr) => { $a > $b }; }
macro_rules! ASSERT_LT { ($a:expr, $b:expr, $name:expr) => { $a < $b }; }
macro_rules! ASSERT_EQ { ($a:expr, $b:expr, $name:expr) => { $a == $b }; }
macro_rules! ASSERT_TRUE { ($expr:expr, $name:expr) => { $expr }; }
macro_rules! ASSERT_OK_PTR { ($ptr:expr, $name:expr) => { !$ptr.is_null() }; }
macro_rules! ASSERT_ERR_PTR { ($ptr:expr, $name:expr) => { $ptr.is_null() }; }
macro_rules! ASSERT_OK_FD { ($fd:expr, $name:expr) => { $fd >= 0 }; }

const CAP_BPF: c_int = 39;
const CAP_PERFMON: c_int = 38;
const CAP_NET_ADMIN: c_int = 12;
const CAP_SYS_ADMIN: c_int = 21;
const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWNS: c_int = 0x00020000;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;
const FSPICK_EMPTY_PATH: c_uint = 0x00000008;
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;
const AT_FDCWD: c_int = -100;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 1;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_CLOEXEC: c_int = 0o2000000;
const O_NOCTTY: c_int = 0o0000400;
const O_NOFOLLOW: c_int = 0o400000;
const EINTR: c_int = 4;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EINVAL: c_int = 22;
const SIGKILL: c_int = 9;
const BPF_F_TOKEN_FD: __u32 = 1 << 16;
const BPF_REG_0: c_int = 0;
const BPF_JMP: u8 = 0x05;
const BPF_CALL: u8 = 0x80;
const BPF_FUNC_JIFFIES64: i32 = 118;
const BPF_FUNC_GET_CURRENT_TASK: i32 = 35;
const BPF_MAP_CREATE: c_int = 0;
const BPF_PROG_LOAD: c_int = 5;
const BPF_BTF_LOAD: c_int = 18;
const BPF_BTF_GET_FD_BY_ID: c_int = 19;
const BPF_MAP_TYPE_STACK: c_int = 23;
const BPF_MAP_TYPE_QUEUE: c_int = 22;
const BPF_MAP_TYPE_STRUCT_OPS: c_int = 26;
const BPF_PROG_TYPE_XDP: c_int = 6;
const BPF_PROG_TYPE_EXT: c_int = 28;
const BPF_PROG_TYPE_STRUCT_OPS: c_int = 27;
const BPF_XDP: __u32 = 37;
const __NR_MOUNT: c_long = 165;
const __NR_FSOPEN: c_long = 430;
const __NR_FSPICK: c_long = 433;
const __NR_FSCONFIG: c_long = 431;
const __NR_FSMOUNT: c_long = 432;
const __NR_MOVE_MOUNT: c_long = 429;

unsafe fn errno() -> c_int {
	*__errno_location()
}

unsafe fn CMSG_SPACE(len: size_t) -> size_t {
	((len + size_of::<size_t>() - 1) & !(size_of::<size_t>() - 1))
		+ ((size_of::<cmsghdr>() + size_of::<size_t>() - 1) & !(size_of::<size_t>() - 1))
}

unsafe fn CMSG_LEN(len: size_t) -> size_t {
	((size_of::<cmsghdr>() + size_of::<size_t>() - 1) & !(size_of::<size_t>() - 1)) + len
}

unsafe fn CMSG_FIRSTHDR(msg: *mut msghdr) -> *mut cmsghdr {
	if (*msg).msg_controllen >= size_of::<cmsghdr>() {
		(*msg).msg_control as *mut cmsghdr
	} else {
		null_mut()
	}
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_uchar {
	(cmsg as *mut u8).add(CMSG_LEN(0)) as *mut c_uchar
}

type c_uchar = u8;

unsafe fn WIFEXITED(status: c_int) -> bool {
	(status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
	(status & 0xff00) >> 8
}

unsafe fn zclose(fd: &mut c_int) {
	if *fd >= 0 {
		close(*fd);
	}
	*fd = -1;
}

unsafe fn bpf_raw_insn(code: u8, dst: u8, src: u8, off: i16, imm: i32) -> bpf_insn {
	bpf_insn { code, dst_src: (dst & 0x0f) | ((src & 0x0f) << 4), off, imm }
}

unsafe fn bpf_mov64_imm(dst: u8, imm: i32) -> bpf_insn {
	bpf_raw_insn(0xb7, dst, 0, 0, imm)
}

unsafe fn bpf_exit_insn() -> bpf_insn {
	bpf_raw_insn(0x95, 0, 0, 0, 0)
}

unsafe fn sys_mount(dev_name: *const c_char, dir_name: *const c_char, type_: *const c_char, flags: c_ulong, data: *const c_void) -> c_int {
	syscall(__NR_MOUNT, dev_name, dir_name, type_, flags, data) as c_int
}

unsafe fn sys_fsopen(fsname: *const c_char, flags: c_uint) -> c_int {
	syscall(__NR_FSOPEN, fsname, flags) as c_int
}

unsafe fn sys_fspick(dfd: c_int, path: *const c_char, flags: c_uint) -> c_int {
	syscall(__NR_FSPICK, dfd, path, flags) as c_int
}

unsafe fn sys_fsconfig(fs_fd: c_int, cmd: c_uint, key: *const c_char, val: *const c_void, aux: c_int) -> c_int {
	syscall(__NR_FSCONFIG, fs_fd, cmd, key, val, aux) as c_int
}

unsafe fn sys_fsmount(fs_fd: c_int, flags: c_uint, ms_flags: c_uint) -> c_int {
	syscall(__NR_FSMOUNT, fs_fd, flags, ms_flags) as c_int
}

unsafe fn sys_move_mount(from_dfd: c_int, from_path: *const c_char, to_dfd: c_int, to_path: *const c_char, flags: c_uint) -> c_int {
	syscall(__NR_MOVE_MOUNT, from_dfd, from_path, to_dfd, to_path, flags) as c_int
}

unsafe fn drop_priv_caps(old_caps: *mut __u64) -> c_int {
	cap_disable_effective((1u64 << CAP_BPF) | (1u64 << CAP_PERFMON) | (1u64 << CAP_NET_ADMIN) | (1u64 << CAP_SYS_ADMIN), old_caps)
}

unsafe fn restore_priv_caps(old_caps: __u64) -> c_int {
	cap_enable_effective(old_caps, null_mut())
}

unsafe fn set_delegate_mask(fs_fd: c_int, key: *const c_char, mask: __u64, mut mask_str: *const c_char) -> c_int {
	let mut buf = [0 as c_char; 32];
	let mut err: c_int;

	if mask_str.is_null() {
		if mask == !0u64 {
			mask_str = b"any\0".as_ptr() as *const c_char;
		} else {
			snprintf(buf.as_mut_ptr(), buf.len(), b"0x%llx\0".as_ptr() as *const c_char, mask as c_ulong);
			mask_str = buf.as_ptr();
		}
	}

	err = sys_fsconfig(fs_fd, FSCONFIG_SET_STRING, key, mask_str as *const c_void, 0);
	if err < 0 {
		err = -errno();
	}
	err
}

unsafe fn create_bpffs_fd() -> c_int {
	let fs_fd = sys_fsopen(b"bpf\0".as_ptr() as *const c_char, 0);
	ASSERT_GE!(fs_fd, 0, "fs_fd");
	fs_fd
}

unsafe fn materialize_bpffs_fd(fs_fd: c_int, opts: *mut bpffs_opts) -> c_int {
	let mut err: c_int;

	err = set_delegate_mask(fs_fd, b"delegate_cmds\0".as_ptr() as *const c_char, (*opts).cmds, (*opts).cmds_str);
	if !ASSERT_OK!(err, "fs_cfg_cmds") { return err; }
	err = set_delegate_mask(fs_fd, b"delegate_maps\0".as_ptr() as *const c_char, (*opts).maps, (*opts).maps_str);
	if !ASSERT_OK!(err, "fs_cfg_maps") { return err; }
	err = set_delegate_mask(fs_fd, b"delegate_progs\0".as_ptr() as *const c_char, (*opts).progs, (*opts).progs_str);
	if !ASSERT_OK!(err, "fs_cfg_progs") { return err; }
	err = set_delegate_mask(fs_fd, b"delegate_attachs\0".as_ptr() as *const c_char, (*opts).attachs, (*opts).attachs_str);
	if !ASSERT_OK!(err, "fs_cfg_attachs") { return err; }

	err = sys_fsconfig(fs_fd, FSCONFIG_CMD_CREATE, null(), null(), 0);
	if err < 0 { return -errno(); }
	0
}

/* send FD over Unix domain (AF_UNIX) socket */
unsafe fn sendfd(sockfd: c_int, fd: c_int) -> c_int {
	let mut msg: msghdr = zeroed();
	let mut fds = [fd];
	let mut iobuf = [0 as c_char; 1];
	let mut io = iovec { iov_base: iobuf.as_mut_ptr() as *mut c_void, iov_len: size_of_val(&iobuf) };
	let mut u = cmsg_buf { buf: [0; 64] };

	msg.msg_iov = &mut io;
	msg.msg_iovlen = 1;
	msg.msg_control = u.buf.as_mut_ptr() as *mut c_void;
	msg.msg_controllen = CMSG_SPACE(size_of_val(&fds));
	let cmsg = CMSG_FIRSTHDR(&mut msg);
	(*cmsg).cmsg_level = SOL_SOCKET;
	(*cmsg).cmsg_type = SCM_RIGHTS;
	(*cmsg).cmsg_len = CMSG_LEN(size_of_val(&fds));
	memcpy(CMSG_DATA(cmsg) as *mut c_void, fds.as_ptr() as *const c_void, size_of_val(&fds));

	let mut err = sendmsg(sockfd, &msg, 0) as c_int;
	if err < 0 { err = -errno(); }
	if !ASSERT_EQ!(err, 1, "sendmsg") { return -EINVAL; }
	0
}

/* receive FD over Unix domain (AF_UNIX) socket */
unsafe fn recvfd(sockfd: c_int, fd: *mut c_int) -> c_int {
	let mut msg: msghdr = zeroed();
	let cmsg: *mut cmsghdr;
	let mut fds = [0 as c_int; 1];
	let mut iobuf = [0 as c_char; 1];
	let mut io = iovec { iov_base: iobuf.as_mut_ptr() as *mut c_void, iov_len: size_of_val(&iobuf) };
	let mut u = cmsg_buf { buf: [0; 64] };

	msg.msg_iov = &mut io;
	msg.msg_iovlen = 1;
	msg.msg_control = u.buf.as_mut_ptr() as *mut c_void;
	msg.msg_controllen = CMSG_SPACE(size_of_val(&fds));

	let mut err = recvmsg(sockfd, &mut msg, 0) as c_int;
	if err < 0 { err = -errno(); }
	if !ASSERT_EQ!(err, 1, "recvmsg") { return -EINVAL; }

	cmsg = CMSG_FIRSTHDR(&mut msg);
	if !ASSERT_OK_PTR!(cmsg, "cmsg_null")
		|| !ASSERT_EQ!((*cmsg).cmsg_len, CMSG_LEN(size_of_val(&fds)), "cmsg_len")
		|| !ASSERT_EQ!((*cmsg).cmsg_level, SOL_SOCKET, "cmsg_level")
		|| !ASSERT_EQ!((*cmsg).cmsg_type, SCM_RIGHTS, "cmsg_type") {
		return -EINVAL;
	}

	memcpy(fds.as_mut_ptr() as *mut c_void, CMSG_DATA(cmsg) as *const c_void, size_of_val(&fds));
	*fd = fds[0];
	0
}

unsafe fn write_nointr(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
	let mut ret: ssize_t;
	loop {
		ret = write(fd, buf, count);
		if !(ret < 0 && errno() == EINTR) { break; }
	}
	ret
}

unsafe fn write_file(path: *const c_char, buf: *const c_void, count: size_t) -> c_int {
	let fd = open(path, O_WRONLY | O_CLOEXEC | O_NOCTTY | O_NOFOLLOW);
	if fd < 0 { return -1; }
	let ret = write_nointr(fd, buf, count);
	close(fd);
	if ret < 0 || ret as size_t != count { return -1; }
	0
}

unsafe fn create_and_enter_userns() -> c_int {
	let uid = getuid();
	let gid = getgid();
	let mut map = [0 as c_char; 100];

	if unshare(CLONE_NEWUSER) != 0 { return -1; }
	if write_file(b"/proc/self/setgroups\0".as_ptr() as *const c_char, b"deny".as_ptr() as *const c_void, 4) != 0 && errno() != ENOENT { return -1; }
	snprintf(map.as_mut_ptr(), map.len(), b"0 %d 1\0".as_ptr() as *const c_char, uid);
	if write_file(b"/proc/self/uid_map\0".as_ptr() as *const c_char, map.as_ptr() as *const c_void, strlen(map.as_ptr())) != 0 { return -1; }
	snprintf(map.as_mut_ptr(), map.len(), b"0 %d 1\0".as_ptr() as *const c_char, gid);
	if write_file(b"/proc/self/gid_map\0".as_ptr() as *const c_char, map.as_ptr() as *const c_void, strlen(map.as_ptr())) != 0 { return -1; }
	if setgid(0) != 0 { return -1; }
	if setuid(0) != 0 { return -1; }
	0
}

unsafe extern "C" fn child(sock_fd: c_int, opts: *mut bpffs_opts, callback: child_callback_fn) {
	let mut mnt_fd = -1;
	let mut fs_fd = -1;
	let mut err = 0;
	let mut bpffs_fd = -1;
	let mut token_fd = -1;
	let mut lsm_skel: *mut token_lsm = null_mut();
	let mut one = 0 as c_char;

	lsm_skel = token_lsm__open_and_load();
	if !ASSERT_OK_PTR!(lsm_skel, "lsm_skel_load") { err = -EINVAL; goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	(*(*lsm_skel).bss).my_pid = getpid();
	err = token_lsm__attach(lsm_skel);
	if !ASSERT_OK!(err, "lsm_skel_attach") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	err = create_and_enter_userns();
	if !ASSERT_OK!(err, "create_and_enter_userns") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	err = unshare(CLONE_NEWNS);
	if !ASSERT_OK!(err, "create_mountns") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	err = sys_mount(null(), b"/\0".as_ptr() as *const c_char, null(), MS_REC | MS_PRIVATE, null());
	if !ASSERT_OK!(err, "remount_root") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }

	fs_fd = create_bpffs_fd();
	if !ASSERT_GE!(fs_fd, 0, "create_bpffs_fd") { err = -EINVAL; goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	err = set_delegate_mask(fs_fd, b"delegate_cmds\0".as_ptr() as *const c_char, 0x1, null());
	ASSERT_EQ!(err, -EPERM, "delegate_cmd_eperm");
	err = set_delegate_mask(fs_fd, b"delegate_maps\0".as_ptr() as *const c_char, 0x1, null());
	ASSERT_EQ!(err, -EPERM, "delegate_maps_eperm");
	err = set_delegate_mask(fs_fd, b"delegate_progs\0".as_ptr() as *const c_char, 0x1, null());
	ASSERT_EQ!(err, -EPERM, "delegate_progs_eperm");
	err = set_delegate_mask(fs_fd, b"delegate_attachs\0".as_ptr() as *const c_char, 0x1, null());
	ASSERT_EQ!(err, -EPERM, "delegate_attachs_eperm");
	err = sendfd(sock_fd, fs_fd);
	if !ASSERT_OK!(err, "send_fs_fd") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	err = read(sock_fd, &mut one as *mut _ as *mut c_void, size_of_val(&one)) as c_int;
	if !ASSERT_GE!(err, 0, "read_one") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	mnt_fd = sys_fsmount(fs_fd, 0, 0);
	if !ASSERT_OK_FD!(mnt_fd, "mnt_fd") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	fs_fd = sys_fspick(mnt_fd, b"\0".as_ptr() as *const c_char, FSPICK_EMPTY_PATH);
	if !ASSERT_GE!(fs_fd, 0, "bpffs_fspick") { err = -EINVAL; goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	for key in [b"delegate_cmds\0".as_ptr(), b"delegate_maps\0".as_ptr(), b"delegate_progs\0".as_ptr(), b"delegate_attachs\0".as_ptr()] {
		err = set_delegate_mask(fs_fd, key as *const c_char, 0, b"any\0".as_ptr() as *const c_char);
		if !ASSERT_EQ!(err, -EPERM, "delegate_eperm_reconfig") { err = -EINVAL; goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	}
	zclose(&mut fs_fd);
	bpffs_fd = openat(mnt_fd, b".\0".as_ptr() as *const c_char, 0, O_RDWR);
	if !ASSERT_GE!(bpffs_fd, 0, "bpffs_open") { err = -EINVAL; goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	token_fd = bpf_token_create(bpffs_fd, null());
	if !ASSERT_GT!(token_fd, 0, "child_token_create") { err = -EINVAL; goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	err = sendfd(sock_fd, token_fd);
	if !ASSERT_OK!(err, "send_token_fd") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	zclose(&mut token_fd);
	err = callback(bpffs_fd, lsm_skel);
	if !ASSERT_OK!(err, "test_callback") { goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err); }
	err = 0;
	goto_cleanup_child(sock_fd, mnt_fd, fs_fd, bpffs_fd, token_fd, lsm_skel, err);
}

unsafe fn goto_cleanup_child(mut sock_fd: c_int, mut mnt_fd: c_int, mut fs_fd: c_int, mut bpffs_fd: c_int, mut token_fd: c_int, lsm_skel: *mut token_lsm, err: c_int) -> ! {
	zclose(&mut sock_fd);
	zclose(&mut mnt_fd);
	zclose(&mut fs_fd);
	zclose(&mut bpffs_fd);
	zclose(&mut token_fd);
	if !lsm_skel.is_null() {
		(*(*lsm_skel).bss).my_pid = 0;
		token_lsm__destroy(lsm_skel);
	}
	exit(-err);
}

unsafe fn wait_for_pid(pid: pid_t) -> c_int {
	let mut status = 0;
	loop {
		let ret = waitpid(pid, &mut status, 0);
		if ret == -1 {
			if errno() == EINTR { continue; }
			return -1;
		}
		break;
	}
	if !WIFEXITED(status) { return -1; }
	WEXITSTATUS(status)
}

unsafe fn parent(child_pid: c_int, bpffs_opts: *mut bpffs_opts, sock_fd: c_int) {
	let mut fs_fd = -1;
	let mut token_fd = -1;
	let mut one: c_char = 1;
	let mut err = recvfd(sock_fd, &mut fs_fd);
	if !ASSERT_OK!(err, "recv_bpffs_fd") { goto_cleanup_parent(child_pid, sock_fd, fs_fd, token_fd); }
	err = materialize_bpffs_fd(fs_fd, bpffs_opts);
	if !ASSERT_GE!(err, 0, "materialize_bpffs_fd") { goto_cleanup_parent(child_pid, sock_fd, fs_fd, token_fd); }
	err = write(sock_fd, &mut one as *mut _ as *const c_void, size_of_val(&one)) as c_int;
	if !ASSERT_EQ!(err as usize, size_of_val(&one), "send_one") { goto_cleanup_parent(child_pid, sock_fd, fs_fd, token_fd); }
	zclose(&mut fs_fd);
	err = recvfd(sock_fd, &mut token_fd);
	if !ASSERT_OK!(err, "recv_token_fd") { goto_cleanup_parent(child_pid, sock_fd, fs_fd, token_fd); }
	err = wait_for_pid(child_pid);
	ASSERT_OK!(err, "waitpid_child");
	goto_cleanup_parent(child_pid, sock_fd, fs_fd, token_fd);
}

unsafe fn goto_cleanup_parent(child_pid: c_int, mut sock_fd: c_int, mut fs_fd: c_int, mut token_fd: c_int) {
	zclose(&mut sock_fd);
	zclose(&mut fs_fd);
	zclose(&mut token_fd);
	if child_pid > 0 { kill(child_pid, SIGKILL); }
}

unsafe fn subtest_userns(bpffs_opts: *mut bpffs_opts, child_cb: child_callback_fn) {
	let mut sock_fds = [-1, -1];
	let mut child_pid = 0;
	let err = socketpair(AF_UNIX, SOCK_STREAM, 0, sock_fds.as_mut_ptr());
	if !ASSERT_OK!(err, "socketpair") { cleanup_subtest(sock_fds, child_pid); return; }
	child_pid = fork();
	if !ASSERT_GE!(child_pid, 0, "fork") { cleanup_subtest(sock_fds, child_pid); return; }
	if child_pid == 0 {
		zclose(&mut sock_fds[0]);
		child(sock_fds[1], bpffs_opts, child_cb);
	} else {
		zclose(&mut sock_fds[1]);
		parent(child_pid, bpffs_opts, sock_fds[0]);
	}
}

unsafe fn cleanup_subtest(mut sock_fds: [c_int; 2], child_pid: c_int) {
	zclose(&mut sock_fds[0]);
	zclose(&mut sock_fds[1]);
	if child_pid > 0 { kill(child_pid, SIGKILL); }
}

unsafe extern "C" fn userns_map_create(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut map_opts: bpf_map_create_opts = zeroed(); map_opts.sz = size_of::<bpf_map_create_opts>();
	let mut err: c_int = 0;
	let token_fd = bpf_token_create(mnt_fd, null());
	let mut map_fd = -1;
	let mut old_caps = 0;
	if !ASSERT_GT!(token_fd, 0, "token_create") { return -EINVAL; }
	err = drop_priv_caps(&mut old_caps);
	if !ASSERT_OK!(err, "drop_caps") { zclose(&mut (token_fd.clone())); return err; }
	map_opts.map_flags = 0; map_opts.token_fd = 0;
	map_fd = bpf_map_create(BPF_MAP_TYPE_STACK, b"wo_token_wo_bpf\0".as_ptr() as *const c_char, 0, 8, 1, &map_opts);
	if !ASSERT_LT!(map_fd, 0, "stack_map_wo_token_wo_cap_bpf_should_fail") { err = -EINVAL; }
	if err == 0 {
		map_opts.map_flags = BPF_F_TOKEN_FD; map_opts.token_fd = token_fd as __u32;
		map_fd = bpf_map_create(BPF_MAP_TYPE_STACK, b"w_token_wo_bpf\0".as_ptr() as *const c_char, 0, 8, 1, &map_opts);
		if !ASSERT_LT!(map_fd, 0, "stack_map_w_token_wo_cap_bpf_should_fail") { err = -EINVAL; }
	}
	if err == 0 {
		err = restore_priv_caps(old_caps);
		if ASSERT_OK!(err, "restore_caps") {
			map_opts.map_flags = 0; map_opts.token_fd = 0;
			map_fd = bpf_map_create(BPF_MAP_TYPE_STACK, b"wo_token_w_bpf\0".as_ptr() as *const c_char, 0, 8, 1, &map_opts);
			if !ASSERT_LT!(map_fd, 0, "stack_map_wo_token_w_cap_bpf_should_fail") { err = -EINVAL; }
			map_opts.map_flags = BPF_F_TOKEN_FD; map_opts.token_fd = token_fd as __u32;
			map_fd = bpf_map_create(BPF_MAP_TYPE_STACK, b"w_token_w_bpf\0".as_ptr() as *const c_char, 0, 8, 1, &map_opts);
			if !ASSERT_GT!(map_fd, 0, "stack_map_w_token_w_cap_bpf") { err = -EINVAL; }
		}
	}
	let mut tfd = token_fd; zclose(&mut tfd); zclose(&mut map_fd); err
}

unsafe extern "C" fn userns_btf_load(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut btf_opts: bpf_btf_load_opts = zeroed(); btf_opts.sz = size_of::<bpf_btf_load_opts>();
	let mut err = 0;
	let mut btf_fd = -1;
	let mut raw_btf_size: __u32 = 0;
	let mut old_caps = 0;
	let token_fd = bpf_token_create(mnt_fd, null());
	if !ASSERT_GT!(token_fd, 0, "token_create") { return -EINVAL; }
	err = drop_priv_caps(&mut old_caps);
	if !ASSERT_OK!(err, "drop_caps") { return err; }
	let btf = btf__new_empty();
	if !ASSERT_OK_PTR!(btf, "empty_btf") { return err; }
	ASSERT_GT!(btf__add_int(btf, b"int\0".as_ptr() as *const c_char, 4, 0), 0, "int_type");
	let raw_btf_data = btf__raw_data(btf, &mut raw_btf_size);
	if ASSERT_OK_PTR!(raw_btf_data, "raw_btf_data") {
		btf_opts.btf_flags = 0; btf_opts.token_fd = 0;
		btf_fd = bpf_btf_load(raw_btf_data, raw_btf_size, &btf_opts);
		if !ASSERT_LT!(btf_fd, 0, "no_token_no_cap_should_fail") { err = -EINVAL; }
		btf_opts.btf_flags = BPF_F_TOKEN_FD; btf_opts.token_fd = token_fd as __u32;
		btf_fd = bpf_btf_load(raw_btf_data, raw_btf_size, &btf_opts);
		if !ASSERT_LT!(btf_fd, 0, "token_no_cap_should_fail") { err = -EINVAL; }
		if err == 0 {
			err = restore_priv_caps(old_caps);
			if ASSERT_OK!(err, "restore_caps") {
				btf_fd = bpf_btf_load(raw_btf_data, raw_btf_size, &btf_opts);
				if !ASSERT_GT!(btf_fd, 0, "token_and_cap_success") { err = -EINVAL; }
			}
		}
	}
	btf__free(btf); zclose(&mut btf_fd); let mut tfd = token_fd; zclose(&mut tfd); err
}

unsafe extern "C" fn userns_prog_load(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut prog_opts: bpf_prog_load_opts = zeroed(); prog_opts.sz = size_of::<bpf_prog_load_opts>();
	let insns = [
		bpf_raw_insn(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_JIFFIES64),
		bpf_raw_insn(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_GET_CURRENT_TASK),
		bpf_mov64_imm(BPF_REG_0 as u8, 0),
		bpf_exit_insn(),
	];
	let mut err = 0;
	let token_fd = bpf_token_create(mnt_fd, null());
	let mut prog_fd = -1;
	let mut old_caps = 0;
	if !ASSERT_GT!(token_fd, 0, "token_create") { return -EINVAL; }
	prog_opts.prog_flags = BPF_F_TOKEN_FD; prog_opts.token_fd = token_fd as __u32; prog_opts.expected_attach_type = BPF_XDP;
	prog_fd = bpf_prog_load(BPF_PROG_TYPE_XDP, b"token_prog\0".as_ptr() as *const c_char, b"GPL\0".as_ptr() as *const c_char, insns.as_ptr(), insns.len(), &prog_opts);
	if !ASSERT_GT!(prog_fd, 0, "prog_fd") { err = -EPERM; }
	prog_opts.prog_flags = 0; prog_opts.token_fd = 0;
	if err == 0 {
		prog_fd = bpf_prog_load(BPF_PROG_TYPE_XDP, b"token_prog\0".as_ptr() as *const c_char, b"GPL\0".as_ptr() as *const c_char, insns.as_ptr(), insns.len(), &prog_opts);
		if !ASSERT_EQ!(prog_fd, -EPERM, "prog_fd_eperm") { err = -EPERM; }
	}
	if err == 0 {
		err = drop_priv_caps(&mut old_caps);
		if ASSERT_OK!(err, "drop_caps") {
			prog_opts.prog_flags = BPF_F_TOKEN_FD; prog_opts.token_fd = token_fd as __u32;
			prog_fd = bpf_prog_load(BPF_PROG_TYPE_XDP, b"token_prog\0".as_ptr() as *const c_char, b"GPL\0".as_ptr() as *const c_char, insns.as_ptr(), insns.len(), &prog_opts);
			if !ASSERT_EQ!(prog_fd, -EPERM, "prog_fd_eperm") { err = -EPERM; }
			prog_opts.prog_flags = 0; prog_opts.token_fd = 0;
			prog_fd = bpf_prog_load(BPF_PROG_TYPE_XDP, b"token_prog\0".as_ptr() as *const c_char, b"GPL\0".as_ptr() as *const c_char, insns.as_ptr(), insns.len(), &prog_opts);
			if !ASSERT_EQ!(prog_fd, -EPERM, "prog_fd_eperm") { err = -EPERM; }
		}
	}
	zclose(&mut prog_fd); let mut tfd = token_fd; zclose(&mut tfd); err
}

unsafe extern "C" fn userns_obj_priv_map(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut opts: bpf_object_open_opts = zeroed(); opts.sz = size_of::<bpf_object_open_opts>();
	let mut buf = [0 as c_char; 256];
	let mut skel = priv_map__open_and_load();
	if !ASSERT_ERR_PTR!(skel, "obj_tokenless_load") { priv_map__destroy(skel); return -EINVAL; }
	snprintf(buf.as_mut_ptr(), buf.len(), b"/proc/self/fd/%d\0".as_ptr() as *const c_char, mnt_fd);
	opts.bpf_token_path = buf.as_ptr();
	skel = priv_map__open_opts(&opts);
	if !ASSERT_OK_PTR!(skel, "obj_token_path_open") { return -EINVAL; }
	let err = priv_map__load(skel);
	priv_map__destroy(skel);
	if !ASSERT_OK!(err, "obj_token_path_load") { return -EINVAL; }
	0
}

unsafe extern "C" fn userns_obj_priv_prog(mnt_fd: c_int, lsm_skel: *mut token_lsm) -> c_int {
	let mut opts: bpf_object_open_opts = zeroed(); opts.sz = size_of::<bpf_object_open_opts>();
	let mut buf = [0 as c_char; 256];
	let mut skel = priv_prog__open_and_load();
	if !ASSERT_ERR_PTR!(skel, "obj_tokenless_load") { priv_prog__destroy(skel); return -EINVAL; }
	snprintf(buf.as_mut_ptr(), buf.len(), b"/proc/self/fd/%d\0".as_ptr() as *const c_char, mnt_fd);
	opts.bpf_token_path = buf.as_ptr();
	skel = priv_prog__open_opts(&opts);
	if !ASSERT_OK_PTR!(skel, "obj_token_path_open") { return -EINVAL; }
	let mut err = priv_prog__load(skel); priv_prog__destroy(skel);
	if !ASSERT_OK!(err, "obj_token_path_load") { return -EINVAL; }
	(*(*lsm_skel).bss).reject_capable = true; (*(*lsm_skel).bss).reject_cmd = false;
	skel = priv_prog__open_opts(&opts); if !ASSERT_OK_PTR!(skel, "obj_token_lsm_reject_cap_open") { return -EINVAL; }
	err = priv_prog__load(skel); priv_prog__destroy(skel); if !ASSERT_ERR!(err, "obj_token_lsm_reject_cap_load") { return -EINVAL; }
	(*(*lsm_skel).bss).reject_capable = false; (*(*lsm_skel).bss).reject_cmd = true;
	skel = priv_prog__open_opts(&opts); if !ASSERT_OK_PTR!(skel, "obj_token_lsm_reject_cmd_open") { return -EINVAL; }
	err = priv_prog__load(skel); priv_prog__destroy(skel); if !ASSERT_ERR!(err, "obj_token_lsm_reject_cmd_load") { return -EINVAL; }
	0
}

unsafe fn userns_obj_priv_freplace_setup(mnt_fd: c_int, fr_skel: *mut *mut priv_freplace_prog, skel: *mut *mut priv_prog, tgt_fd: *mut c_int) -> c_int {
	let mut opts: bpf_object_open_opts = zeroed(); opts.sz = size_of::<bpf_object_open_opts>();
	let mut buf = [0 as c_char; 256];
	snprintf(buf.as_mut_ptr(), buf.len(), b"/proc/self/fd/%d\0".as_ptr() as *const c_char, mnt_fd);
	opts.bpf_token_path = buf.as_ptr();
	*skel = priv_prog__open_opts(&opts);
	if !ASSERT_OK_PTR!(*skel, "priv_prog__open_opts") { return -EINVAL; }
	let err = priv_prog__load(*skel);
	if !ASSERT_OK!(err, "priv_prog__load") { return -EINVAL; }
	*fr_skel = priv_freplace_prog__open_opts(&opts);
	if !ASSERT_OK_PTR!(*skel, "priv_freplace_prog__open_opts") { return -EINVAL; }
	*tgt_fd = bpf_program__fd((**skel).progs.xdp_prog1);
	0
}

unsafe extern "C" fn userns_obj_priv_freplace_prog(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut fr_skel: *mut priv_freplace_prog = null_mut();
	let mut skel: *mut priv_prog = null_mut();
	let mut tgt_fd = 0;
	let mut err = userns_obj_priv_freplace_setup(mnt_fd, &mut fr_skel, &mut skel, &mut tgt_fd);
	if ASSERT_OK!(err, "setup") {
		err = bpf_object__prepare((*fr_skel).obj);
		if ASSERT_OK!(err, "freplace__prepare") {
			err = bpf_program__set_attach_target((*fr_skel).progs.new_xdp_prog2, tgt_fd, b"xdp_prog1\0".as_ptr() as *const c_char);
			if ASSERT_OK!(err, "set_attach_target") {
				err = priv_freplace_prog__load(fr_skel);
				ASSERT_OK!(err, "priv_freplace_prog__load");
			}
		}
	}
	priv_freplace_prog__destroy(fr_skel); priv_prog__destroy(skel); err
}

unsafe extern "C" fn userns_obj_priv_freplace_prog_fail(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut fr_skel: *mut priv_freplace_prog = null_mut();
	let mut skel: *mut priv_prog = null_mut();
	let mut tgt_fd = 0;
	let mut err = userns_obj_priv_freplace_setup(mnt_fd, &mut fr_skel, &mut skel, &mut tgt_fd);
	if ASSERT_OK!(err, "setup") {
		err = bpf_program__set_attach_target((*fr_skel).progs.new_xdp_prog2, tgt_fd, b"xdp_prog1\0".as_ptr() as *const c_char);
		err = if ASSERT_ERR!(err, "attach fails") { 0 } else { -EINVAL };
	}
	priv_freplace_prog__destroy(fr_skel); priv_prog__destroy(skel); err
}

unsafe fn validate_struct_ops_load(mnt_fd: c_int, expect_success: bool) -> c_int {
	let mut opts: bpf_object_open_opts = zeroed(); opts.sz = size_of::<bpf_object_open_opts>();
	let mut buf = [0 as c_char; 256];
	snprintf(buf.as_mut_ptr(), buf.len(), b"/proc/self/fd/%d\0".as_ptr() as *const c_char, mnt_fd);
	opts.bpf_token_path = buf.as_ptr();
	let skel = dummy_st_ops_success__open_opts(&opts);
	if !ASSERT_OK_PTR!(skel, "obj_token_path_open") { return -EINVAL; }
	let err = dummy_st_ops_success__load(skel);
	dummy_st_ops_success__destroy(skel);
	if expect_success {
		if !ASSERT_OK!(err, "obj_token_path_load") { return -EINVAL; }
	} else if !ASSERT_ERR!(err, "obj_token_path_load") { return -EINVAL; }
	0
}

unsafe extern "C" fn userns_obj_priv_btf_fail(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int { validate_struct_ops_load(mnt_fd, false) }
unsafe extern "C" fn userns_obj_priv_btf_success(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int { validate_struct_ops_load(mnt_fd, true) }

unsafe fn token_bpffs_custom_dir() -> *const c_char {
	let v = getenv(b"BPF_SELFTESTS_BPF_TOKEN_DIR\0".as_ptr() as *const c_char);
	if !v.is_null() { v } else { b"/tmp/bpf-token-fs\0".as_ptr() as *const c_char }
}

unsafe extern "C" fn userns_obj_priv_implicit_token(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut opts: bpf_object_open_opts = zeroed(); opts.sz = size_of::<bpf_object_open_opts>();
	let mut skel = dummy_st_ops_success__open_and_load();
	if !ASSERT_ERR_PTR!(skel, "obj_tokenless_load") { dummy_st_ops_success__destroy(skel); return -EINVAL; }
	let mut err = sys_move_mount(mnt_fd, b"\0".as_ptr() as *const c_char, AT_FDCWD, b"/sys/fs/bpf\0".as_ptr() as *const c_char, MOVE_MOUNT_F_EMPTY_PATH);
	if !ASSERT_OK!(err, "move_mount_bpffs") { return -EINVAL; }
	err = setenv(TOKEN_ENVVAR, b"\0".as_ptr() as *const c_char, 1);
	if !ASSERT_OK!(err, "setenv_token_path") { return -EINVAL; }
	skel = dummy_st_ops_success__open_and_load();
	if !ASSERT_ERR_PTR!(skel, "obj_token_envvar_disabled_load") { unsetenv(TOKEN_ENVVAR); dummy_st_ops_success__destroy(skel); return -EINVAL; }
	unsetenv(TOKEN_ENVVAR);
	skel = dummy_st_ops_success__open_and_load();
	if !ASSERT_OK_PTR!(skel, "obj_implicit_token_load") { return -EINVAL; }
	dummy_st_ops_success__destroy(skel);
	opts.bpf_token_path = b"\0".as_ptr() as *const c_char;
	skel = dummy_st_ops_success__open_opts(&opts);
	if !ASSERT_OK_PTR!(skel, "obj_empty_token_path_open") { return -EINVAL; }
	err = dummy_st_ops_success__load(skel); dummy_st_ops_success__destroy(skel);
	if !ASSERT_ERR!(err, "obj_empty_token_path_load") { return -EINVAL; }
	0
}

unsafe extern "C" fn userns_obj_priv_implicit_token_envvar(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let custom_dir = token_bpffs_custom_dir();
	let mut opts: bpf_object_open_opts = zeroed(); opts.sz = size_of::<bpf_object_open_opts>();
	let mut skel = dummy_st_ops_success__open_and_load();
	if !ASSERT_ERR_PTR!(skel, "obj_tokenless_load") { dummy_st_ops_success__destroy(skel); return -EINVAL; }
	rmdir(custom_dir);
	if !ASSERT_OK!(mkdir(custom_dir, 0o777), "mkdir_bpffs_custom") { rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); return -EINVAL; }
	let mut err = sys_move_mount(mnt_fd, b"\0".as_ptr() as *const c_char, AT_FDCWD, custom_dir, MOVE_MOUNT_F_EMPTY_PATH);
	if !ASSERT_OK!(err, "move_mount_bpffs") { rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); return -EINVAL; }
	skel = dummy_st_ops_success__open_and_load();
	if !ASSERT_ERR_PTR!(skel, "obj_tokenless_load2") { dummy_st_ops_success__destroy(skel); rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); return -EINVAL; }
	err = setenv(TOKEN_ENVVAR, custom_dir, 1);
	if !ASSERT_OK!(err, "setenv_token_path") { rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); return -EINVAL; }
	skel = dummy_st_ops_success__open_and_load();
	if !ASSERT_OK_PTR!(skel, "obj_implicit_token_load") { rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); return -EINVAL; }
	dummy_st_ops_success__destroy(skel);
	opts.bpf_token_path = b"\0".as_ptr() as *const c_char;
	skel = dummy_st_ops_success__open_opts(&opts);
	if !ASSERT_OK_PTR!(skel, "obj_empty_token_path_open") { rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); return -EINVAL; }
	err = dummy_st_ops_success__load(skel); dummy_st_ops_success__destroy(skel);
	if !ASSERT_ERR!(err, "obj_empty_token_path_load") { rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); return -EINVAL; }
	rmdir(custom_dir); unsetenv(TOKEN_ENVVAR); 0
}

unsafe fn kallsyms_has_bpf_func(ksyms: *mut ksyms, func_name: *const c_char) -> bool {
	let mut name = [0 as c_char; 256];
	for i in 0..(*ksyms).sym_cnt {
		let sym = (*ksyms).syms.add(i as usize);
		if sscanf((*sym).name, b"bpf_prog_%*[^_]_%255s\0".as_ptr() as *const c_char, name.as_mut_ptr()) == 1
			&& strcmp(name.as_ptr(), func_name) == 0 {
			return true;
		}
	}
	false
}

const fn bit(n: c_int) -> __u64 { 1u64 << n }

unsafe extern "C" fn userns_obj_priv_prog_kallsyms(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let func_names = [b"xdp_main\0".as_ptr() as *const c_char, b"token_ksym_subprog\0".as_ptr() as *const c_char];
	let mut opts: bpf_object_open_opts = zeroed(); opts.sz = size_of::<bpf_object_open_opts>();
	let mut buf = [0 as c_char; 256];
	snprintf(buf.as_mut_ptr(), buf.len(), b"/proc/self/fd/%d\0".as_ptr() as *const c_char, mnt_fd);
	opts.bpf_token_path = buf.as_ptr();
	let skel = token_kallsyms__open_opts(&opts);
	if !ASSERT_OK_PTR!(skel, "token_kallsyms__open_opts") { return -EINVAL; }
	let mut err = token_kallsyms__load(skel);
	let mut ksyms_ptr: *mut ksyms = null_mut();
	if ASSERT_OK!(err, "token_kallsyms__load") {
		ksyms_ptr = load_kallsyms_local();
		if !ASSERT_OK_PTR!(ksyms_ptr, "load_kallsyms_local") { err = -EINVAL; }
		for f in func_names {
			if err == 0 && !ASSERT_TRUE!(kallsyms_has_bpf_func(ksyms_ptr, f), f) { err = -EINVAL; break; }
		}
	}
	free_kallsyms_local(ksyms_ptr); token_kallsyms__destroy(skel); err
}

unsafe extern "C" fn userns_bpf_token_info(mnt_fd: c_int, _lsm_skel: *mut token_lsm) -> c_int {
	let mut err: c_int;
	let mut token_fd = bpf_token_create(mnt_fd, null());
	let mut info: bpf_token_info = zeroed();
	let mut len = size_of::<bpf_token_info>() as __u32;
	if !ASSERT_GT!(token_fd, 0, "token_create") { return -EINVAL; }
	memset(&mut info as *mut _ as *mut c_void, 0, len as size_t);
	err = bpf_obj_get_info_by_fd(token_fd, &mut info, &mut len);
	if !ASSERT_ERR!(err, "bpf_obj_get_token_info") { zclose(&mut token_fd); return err; }
	if !ASSERT_EQ!(info.allowed_cmds, bit(BPF_MAP_CREATE), "token_info_cmds_map_create") { err = -EINVAL; }
	if err == 0 && !ASSERT_EQ!(info.allowed_progs, bit(BPF_PROG_TYPE_XDP), "token_info_progs_xdp") { err = -EINVAL; }
	if ASSERT_EQ!(info.allowed_progs, bit(BPF_PROG_TYPE_EXT), "token_info_progs_ext") { err = -EINVAL; }
	zclose(&mut token_fd); err
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_token() {
	if test__start_subtest(b"map_token\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: 0, maps: 0, progs: 0, attachs: 0, cmds_str: b"map_create\0".as_ptr() as *const c_char, maps_str: b"stack\0".as_ptr() as *const c_char, progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_map_create);
	}
	if test__start_subtest(b"btf_token\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: 1u64 << BPF_BTF_LOAD, maps: 0, progs: 0, attachs: 0, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_btf_load);
	}
	if test__start_subtest(b"prog_token\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: 0, maps: 0, progs: 0, attachs: 0, cmds_str: b"PROG_LOAD\0".as_ptr() as *const c_char, maps_str: null(), progs_str: b"XDP\0".as_ptr() as *const c_char, attachs_str: b"xdp\0".as_ptr() as *const c_char };
		subtest_userns(&mut opts, userns_prog_load);
	}
	if test__start_subtest(b"obj_priv_map\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_MAP_CREATE), maps: bit(BPF_MAP_TYPE_QUEUE), progs: 0, attachs: 0, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_map);
	}
	if test__start_subtest(b"obj_priv_prog\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_PROG_LOAD), maps: 0, progs: bit(BPF_PROG_TYPE_XDP), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_prog);
	}
	if test__start_subtest(b"obj_priv_freplace_prog\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_BTF_LOAD) | bit(BPF_PROG_LOAD) | bit(BPF_BTF_GET_FD_BY_ID), maps: 0, progs: bit(BPF_PROG_TYPE_EXT) | bit(BPF_PROG_TYPE_XDP), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_freplace_prog);
	}
	if test__start_subtest(b"obj_priv_freplace_prog_fail\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_BTF_LOAD) | bit(BPF_PROG_LOAD) | bit(BPF_BTF_GET_FD_BY_ID), maps: 0, progs: bit(BPF_PROG_TYPE_EXT) | bit(BPF_PROG_TYPE_XDP), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_freplace_prog_fail);
	}
	if test__start_subtest(b"obj_priv_btf_fail\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_MAP_CREATE) | bit(BPF_PROG_LOAD), maps: bit(BPF_MAP_TYPE_STRUCT_OPS), progs: bit(BPF_PROG_TYPE_STRUCT_OPS), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_btf_fail);
	}
	if test__start_subtest(b"obj_priv_btf_success\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_BTF_LOAD) | bit(BPF_MAP_CREATE) | bit(BPF_PROG_LOAD), maps: bit(BPF_MAP_TYPE_STRUCT_OPS), progs: bit(BPF_PROG_TYPE_STRUCT_OPS), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_btf_success);
	}
	if test__start_subtest(b"obj_priv_implicit_token\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_BTF_LOAD) | bit(BPF_MAP_CREATE) | bit(BPF_PROG_LOAD), maps: bit(BPF_MAP_TYPE_STRUCT_OPS), progs: bit(BPF_PROG_TYPE_STRUCT_OPS), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_implicit_token);
	}
	if test__start_subtest(b"obj_priv_implicit_token_envvar\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_BTF_LOAD) | bit(BPF_MAP_CREATE) | bit(BPF_PROG_LOAD), maps: bit(BPF_MAP_TYPE_STRUCT_OPS), progs: bit(BPF_PROG_TYPE_STRUCT_OPS), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_obj_priv_implicit_token_envvar);
	}
	if test__start_subtest(b"bpf_token_info\0".as_ptr() as *const c_char) {
		let mut opts = bpffs_opts { cmds: bit(BPF_MAP_CREATE), maps: 0, progs: bit(BPF_PROG_TYPE_XDP), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		subtest_userns(&mut opts, userns_bpf_token_info);
	}
	if test__start_subtest(b"obj_priv_prog_kallsyms\0".as_ptr() as *const c_char) {
		let mut perf_paranoid_orig = [0 as c_char; 32];
		let mut kptr_restrict_orig = [0 as c_char; 32];
		let mut opts = bpffs_opts { cmds: bit(BPF_BTF_LOAD) | bit(BPF_PROG_LOAD), maps: 0, progs: bit(BPF_PROG_TYPE_XDP), attachs: !0u64, cmds_str: null(), maps_str: null(), progs_str: null(), attachs_str: null() };
		if sysctl_set_or_fail(b"/proc/sys/kernel/perf_event_paranoid\0".as_ptr() as *const c_char, perf_paranoid_orig.as_mut_ptr(), b"0\0".as_ptr() as *const c_char) == 0
			&& sysctl_set_or_fail(b"/proc/sys/kernel/kptr_restrict\0".as_ptr() as *const c_char, kptr_restrict_orig.as_mut_ptr(), b"0\0".as_ptr() as *const c_char) == 0 {
			subtest_userns(&mut opts, userns_obj_priv_prog_kallsyms);
		}
		if perf_paranoid_orig[0] != 0 {
			sysctl_set_or_fail(b"/proc/sys/kernel/perf_event_paranoid\0".as_ptr() as *const c_char, null_mut(), perf_paranoid_orig.as_ptr());
		}
		if kptr_restrict_orig[0] != 0 {
			sysctl_set_or_fail(b"/proc/sys/kernel/kptr_restrict\0".as_ptr() as *const c_char, null_mut(), kptr_restrict_orig.as_ptr());
		}
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
