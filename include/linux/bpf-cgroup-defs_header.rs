/* SPDX-License-Identifier: GPL-2.0 */

/* The original declarations are conditional on CONFIG_CGROUP_BPF. */
#[cfg(CONFIG_CGROUP_BPF)]
pub struct bpf_prog_array;

#[cfg(all(CONFIG_CGROUP_BPF, CONFIG_BPF_LSM))]
pub const CGROUP_LSM_NUM: usize = 10;

#[cfg(all(CONFIG_CGROUP_BPF, not(CONFIG_BPF_LSM)))]
pub const CGROUP_LSM_NUM: usize = 0;

#[cfg(CONFIG_CGROUP_BPF)]
#[repr(isize)]
pub enum cgroup_bpf_attach_type {
    CGROUP_BPF_ATTACH_TYPE_INVALID = -1,
    CGROUP_INET_INGRESS = 0,
    CGROUP_INET_EGRESS,
    CGROUP_INET_SOCK_CREATE,
    CGROUP_SOCK_OPS,
    CGROUP_DEVICE,
    CGROUP_INET4_BIND,
    CGROUP_INET6_BIND,
    CGROUP_INET4_CONNECT,
    CGROUP_INET6_CONNECT,
    CGROUP_UNIX_CONNECT,
    CGROUP_INET4_POST_BIND,
    CGROUP_INET6_POST_BIND,
    CGROUP_UDP4_SENDMSG,
    CGROUP_UDP6_SENDMSG,
    CGROUP_UNIX_SENDMSG,
    CGROUP_SYSCTL,
    CGROUP_UDP4_RECVMSG,
    CGROUP_UDP6_RECVMSG,
    CGROUP_UNIX_RECVMSG,
    CGROUP_GETSOCKOPT,
    CGROUP_SETSOCKOPT,
    CGROUP_INET4_GETPEERNAME,
    CGROUP_INET6_GETPEERNAME,
    CGROUP_UNIX_GETPEERNAME,
    CGROUP_INET4_GETSOCKNAME,
    CGROUP_INET6_GETSOCKNAME,
    CGROUP_UNIX_GETSOCKNAME,
    CGROUP_INET_SOCK_RELEASE,
    CGROUP_LSM_START,
    CGROUP_LSM_END = CGROUP_LSM_START as isize + CGROUP_LSM_NUM as isize - 1,
    MAX_CGROUP_BPF_ATTACH_TYPE,
}

#[cfg(CONFIG_CGROUP_BPF)]
#[repr(C)]
pub struct cgroup_bpf {
    /* array of effective progs in this cgroup */
    pub effective: [*mut bpf_prog_array; MAX_CGROUP_BPF_ATTACH_TYPE as usize],

    /* attached progs to this cgroup and attach flags
     * when flags == 0 or BPF_F_ALLOW_OVERRIDE the progs list will
     * have either zero or one element
     * when BPF_F_ALLOW_MULTI the list can have up to BPF_CGROUP_MAX_PROGS
     */
    pub progs: [hlist_head; MAX_CGROUP_BPF_ATTACH_TYPE as usize],
    pub flags: [u8; MAX_CGROUP_BPF_ATTACH_TYPE as usize],
    pub revisions: [u64; MAX_CGROUP_BPF_ATTACH_TYPE as usize],

    /* list of cgroup shared storages */
    pub storages: list_head,

    /* temp storage for effective prog array used by prog_attach/detach */
    pub inactive: *mut bpf_prog_array,

    /* reference counter used to detach bpf programs after cgroup removal */
    pub refcnt: percpu_ref,

    /* cgroup_bpf is released using a work queue */
    pub release_work: work_struct,
}

#[cfg(not(CONFIG_CGROUP_BPF))]
pub struct cgroup_bpf {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
