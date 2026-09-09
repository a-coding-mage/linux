/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Some day this will be a full-fledged user tracking system..
 */
#[repr(C)]
pub struct user_struct {
	pub __count: refcount_t, /* reference count */
	#[cfg(feature = "CONFIG_EPOLL")]
	pub epoll_watches: percpu_counter, /* The number of file descriptors currently watched */
	pub unix_inflight: ::core::ffi::c_ulong, /* How many files in flight in unix sockets */
	pub pipe_bufs: atomic_long_t, /* how many pages are allocated in pipe buffers */

	/* Hash table maintenance information */
	pub uidhash_node: hlist_node,
	pub uid: kuid_t,

	/* CONFIG_PERF_EVENTS || CONFIG_BPF_SYSCALL || CONFIG_NET ||
	 * CONFIG_IO_URING || CONFIG_VFIO_PCI_ZDEV_KVM || IS_ENABLED(CONFIG_IOMMUFD) */
	#[cfg(any(
		feature = "CONFIG_PERF_EVENTS",
		feature = "CONFIG_BPF_SYSCALL",
		feature = "CONFIG_NET",
		feature = "CONFIG_IO_URING",
		feature = "CONFIG_VFIO_PCI_ZDEV_KVM",
		feature = "CONFIG_IOMMUFD"
	))]
	pub locked_vm: atomic_long_t,
	#[cfg(feature = "CONFIG_WATCH_QUEUE")]
	pub nr_watches: atomic_t, /* The number of watches this user currently has */

	/* Miscellaneous per-user rate limit */
	pub ratelimit: ratelimit_state,
}

extern "C" {
	pub fn uids_sysfs_init() -> ::core::ffi::c_int;

	pub fn find_user(uid: kuid_t) -> *mut user_struct;

	pub static mut root_user: user_struct;

	/* per-UID process charging. */
	pub fn alloc_uid(uid: kuid_t) -> *mut user_struct;
	pub fn free_uid(user: *mut user_struct);
}

#[inline]
pub unsafe fn get_uid(u: *mut user_struct) -> *mut user_struct {
	refcount_inc(&mut (*u).__count);
	u
}

/* #define INIT_USER (&root_user) */
#[inline]
pub unsafe fn INIT_USER() -> *mut user_struct {
	&raw mut root_user
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
