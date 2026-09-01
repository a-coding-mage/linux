/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency intent from C header: #include <linux/types.h> */

/*
 * cloning flags:
 */
pub const CSIGNAL: u32 = 0x000000ff; /* signal mask to be sent at exit */
pub const CLONE_VM: u32 = 0x00000100; /* set if VM shared between processes */
pub const CLONE_FS: u32 = 0x00000200; /* set if fs info shared between processes */
pub const CLONE_FILES: u32 = 0x00000400; /* set if open files shared between processes */
pub const CLONE_SIGHAND: u32 = 0x00000800; /* set if signal handlers and blocked signals shared */
pub const CLONE_PIDFD: u32 = 0x00001000; /* set if a pidfd should be placed in parent */
pub const CLONE_PTRACE: u32 = 0x00002000; /* set if we want to let tracing continue on the child too */
pub const CLONE_VFORK: u32 = 0x00004000; /* set if the parent wants the child to wake it up on mm_release */
pub const CLONE_PARENT: u32 = 0x00008000; /* set if we want to have the same parent as the cloner */
pub const CLONE_THREAD: u32 = 0x00010000; /* Same thread group? */
pub const CLONE_NEWNS: u32 = 0x00020000; /* New mount namespace group */
pub const CLONE_SYSVSEM: u32 = 0x00040000; /* share system V SEM_UNDO semantics */
pub const CLONE_SETTLS: u32 = 0x00080000; /* create a new TLS for the child */
pub const CLONE_PARENT_SETTID: u32 = 0x00100000; /* set the TID in the parent */
pub const CLONE_CHILD_CLEARTID: u32 = 0x00200000; /* clear the TID in the child */
pub const CLONE_DETACHED: u32 = 0x00400000; /* Unused, ignored */
pub const CLONE_UNTRACED: u32 = 0x00800000; /* set if the tracing process can't force CLONE_PTRACE on this clone */
pub const CLONE_CHILD_SETTID: u32 = 0x01000000; /* set the TID in the child */
pub const CLONE_NEWCGROUP: u32 = 0x02000000; /* New cgroup namespace */
pub const CLONE_NEWUTS: u32 = 0x04000000; /* New utsname namespace */
pub const CLONE_NEWIPC: u32 = 0x08000000; /* New ipc namespace */
pub const CLONE_NEWUSER: u32 = 0x10000000; /* New user namespace */
pub const CLONE_NEWPID: u32 = 0x20000000; /* New pid namespace */
pub const CLONE_NEWNET: u32 = 0x40000000; /* New network namespace */
pub const CLONE_IO: u32 = 0x80000000; /* Clone io context */

/* Flags for the clone3() syscall. */
pub const CLONE_CLEAR_SIGHAND: u64 = 1u64 << 32; /* Clear any signal handler and reset to SIG_DFL. */
pub const CLONE_INTO_CGROUP: u64 = 1u64 << 33; /* Clone into a specific cgroup given the right permissions. */
pub const CLONE_AUTOREAP: u64 = 1u64 << 34; /* Auto-reap child on exit. */
pub const CLONE_NNP: u64 = 1u64 << 35; /* Set no_new_privs on child. */
pub const CLONE_PIDFD_AUTOKILL: u64 = 1u64 << 36; /* Kill child when clone pidfd closes. */
pub const CLONE_EMPTY_MNTNS: u64 = 1u64 << 37; /* Create an empty mount namespace. */

/*
 * cloning flags intersect with CSIGNAL so can be used with unshare and clone3
 * syscalls only:
 */
pub const CLONE_NEWTIME: u32 = 0x00000080; /* New time namespace */

/*
 * unshare flags share the bit space with clone flags but only apply to the
 * unshare syscall:
 */
pub const UNSHARE_EMPTY_MNTNS: u32 = 0x00100000; /* Unshare an empty mount namespace. */

/**
 * struct clone_args - arguments for the clone3 syscall
 * @flags:        Flags for the new process as listed above.
 *                All flags are valid except for CSIGNAL and
 *                CLONE_DETACHED.
 * @pidfd:        If CLONE_PIDFD is set, a pidfd will be
 *                returned in this argument.
 * @child_tid:    If CLONE_CHILD_SETTID is set, the TID of the
 *                child process will be returned in the child's
 *                memory.
 * @parent_tid:   If CLONE_PARENT_SETTID is set, the TID of
 *                the child process will be returned in the
 *                parent's memory.
 * @exit_signal:  The exit_signal the parent process will be
 *                sent when the child exits.
 * @stack:        Specify the location of the stack for the
 *                child process.
 *                Note, @stack is expected to point to the
 *                lowest address. The stack direction will be
 *                determined by the kernel and set up
 *                appropriately based on @stack_size.
 * @stack_size:   The size of the stack for the child process.
 * @tls:          If CLONE_SETTLS is set, the tls descriptor
 *                is set to tls.
 * @set_tid:      Pointer to an array of type *pid_t. The size
 *                of the array is defined using @set_tid_size.
 *                This array is used to select PIDs/TIDs for
 *                newly created processes. The first element in
 *                this defines the PID in the most nested PID
 *                namespace. Each additional element in the array
 *                defines the PID in the parent PID namespace of
 *                the original PID namespace. If the array has
 *                less entries than the number of currently
 *                nested PID namespaces only the PIDs in the
 *                corresponding namespaces are set.
 * @set_tid_size: This defines the size of the array referenced
 *                in @set_tid. This cannot be larger than the
 *                kernel's limit of nested PID namespaces.
 * @cgroup:       If CLONE_INTO_CGROUP is specified set this to
 *                a file descriptor for the cgroup.
 *
 * The structure is versioned by size and thus extensible.
 * New struct members must go at the end of the struct and
 * must be properly 64bit aligned.
 */
#[repr(C)]
pub struct clone_args {
    pub flags: __aligned_u64,
    pub pidfd: __aligned_u64,
    pub child_tid: __aligned_u64,
    pub parent_tid: __aligned_u64,
    pub exit_signal: __aligned_u64,
    pub stack: __aligned_u64,
    pub stack_size: __aligned_u64,
    pub tls: __aligned_u64,
    pub set_tid: __aligned_u64,
    pub set_tid_size: __aligned_u64,
    pub cgroup: __aligned_u64,
}

pub const CLONE_ARGS_SIZE_VER0: u32 = 64; /* sizeof first published struct */
pub const CLONE_ARGS_SIZE_VER1: u32 = 80; /* sizeof second published struct */
pub const CLONE_ARGS_SIZE_VER2: u32 = 88; /* sizeof third published struct */

/*
 * Scheduling policies
 */
pub const SCHED_NORMAL: u32 = 0;
pub const SCHED_FIFO: u32 = 1;
pub const SCHED_RR: u32 = 2;
pub const SCHED_BATCH: u32 = 3;
/* SCHED_ISO: reserved but not implemented yet */
pub const SCHED_IDLE: u32 = 5;
pub const SCHED_DEADLINE: u32 = 6;
pub const SCHED_EXT: u32 = 7;

/* Can be ORed in to make sure the process is reverted back to SCHED_NORMAL on fork */
pub const SCHED_RESET_ON_FORK: u32 = 0x40000000;

/*
 * For the sched_{set,get}attr() calls
 */
pub const SCHED_FLAG_RESET_ON_FORK: u32 = 0x01;
pub const SCHED_FLAG_RECLAIM: u32 = 0x02;
pub const SCHED_FLAG_DL_OVERRUN: u32 = 0x04;
pub const SCHED_FLAG_KEEP_POLICY: u32 = 0x08;
pub const SCHED_FLAG_KEEP_PARAMS: u32 = 0x10;
pub const SCHED_FLAG_UTIL_CLAMP_MIN: u32 = 0x20;
pub const SCHED_FLAG_UTIL_CLAMP_MAX: u32 = 0x40;

pub const SCHED_FLAG_KEEP_ALL: u32 = SCHED_FLAG_KEEP_POLICY | SCHED_FLAG_KEEP_PARAMS;

pub const SCHED_FLAG_UTIL_CLAMP: u32 = SCHED_FLAG_UTIL_CLAMP_MIN | SCHED_FLAG_UTIL_CLAMP_MAX;

pub const SCHED_FLAG_ALL: u32 = SCHED_FLAG_RESET_ON_FORK
    | SCHED_FLAG_RECLAIM
    | SCHED_FLAG_DL_OVERRUN
    | SCHED_FLAG_KEEP_ALL
    | SCHED_FLAG_UTIL_CLAMP;

/* Only for sched_getattr() own flag param, if task is SCHED_DEADLINE */
pub const SCHED_GETATTR_FLAG_DL_DYNAMIC: u32 = 0x01;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
