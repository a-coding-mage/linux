/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Linux header dependencies: linux/ioctl.h and linux/types.h.

pub const NSIO: u32 = 0xb7;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << 30) | (size << 16) | (ty << 8) | nr
}
const fn io(ty: u32, nr: u32) -> u32 { ioc(0, ty, nr, 0) }
const fn ior<T>(ty: u32, nr: u32) -> u32 { ioc(2, ty, nr, core::mem::size_of::<T>() as u32) }

/* Returns a file descriptor that refers to an owning user namespace */
pub const NS_GET_USERNS: u32 = io(NSIO, 0x1);
/* Returns a file descriptor that refers to a parent namespace */
pub const NS_GET_PARENT: u32 = io(NSIO, 0x2);
/* Returns the type of namespace (CLONE_NEW* value) referred to by
   file descriptor */
pub const NS_GET_NSTYPE: u32 = io(NSIO, 0x3);
/* Get owner UID (in the caller's user namespace) for a user namespace */
pub const NS_GET_OWNER_UID: u32 = io(NSIO, 0x4);
/* Translate pid from target pid namespace into the caller's pid namespace. */
pub const NS_GET_PID_FROM_PIDNS: u32 = ior::<i32>(NSIO, 0x6);
/* Return thread-group leader id of pid in the callers pid namespace. */
pub const NS_GET_TGID_FROM_PIDNS: u32 = ior::<i32>(NSIO, 0x7);
/* Translate pid from caller's pid namespace into a target pid namespace. */
pub const NS_GET_PID_IN_PIDNS: u32 = ior::<i32>(NSIO, 0x8);
/* Return thread-group leader id of pid in the target pid namespace. */
pub const NS_GET_TGID_IN_PIDNS: u32 = ior::<i32>(NSIO, 0x9);

#[repr(C)]
pub struct mnt_ns_info {
    pub size: u32,
    pub nr_mounts: u32,
    pub mnt_ns_id: u64,
}

pub const MNT_NS_INFO_SIZE_VER0: usize = 16; /* size of first published struct */

/* Get information about namespace. */
pub const NS_MNT_GET_INFO: u32 = ior::<mnt_ns_info>(NSIO, 10);
/* Get next namespace. */
pub const NS_MNT_GET_NEXT: u32 = ior::<mnt_ns_info>(NSIO, 11);
/* Get previous namespace. */
pub const NS_MNT_GET_PREV: u32 = ior::<mnt_ns_info>(NSIO, 12);

/* Retrieve namespace identifiers. */
pub const NS_GET_MNTNS_ID: u32 = ior::<u64>(NSIO, 5);
pub const NS_GET_ID: u32 = ior::<u64>(NSIO, 13);

#[repr(u32)]
pub enum init_ns_ino {
    IPC_NS_INIT_INO = 0xEFFFFFFF,
    UTS_NS_INIT_INO = 0xEFFFFFFE,
    USER_NS_INIT_INO = 0xEFFFFFFD,
    PID_NS_INIT_INO = 0xEFFFFFFC,
    CGROUP_NS_INIT_INO = 0xEFFFFFFB,
    TIME_NS_INIT_INO = 0xEFFFFFFA,
    NET_NS_INIT_INO = 0xEFFFFFF9,
    MNT_NS_INIT_INO = 0xEFFFFFF8,
    // MNT_NS_ANON_INO is present when __KERNEL__ is defined.
}

#[repr(C)]
pub struct nsfs_file_handle {
    pub ns_id: u64,
    pub ns_type: u32,
    pub ns_inum: u32,
}

pub const NSFS_FILE_HANDLE_SIZE_VER0: usize = 16; /* sizeof first published struct */
pub const NSFS_FILE_HANDLE_SIZE_LATEST: usize = core::mem::size_of::<nsfs_file_handle>(); /* sizeof latest published struct */

#[repr(u64)]
pub enum init_ns_id {
    IPC_NS_INIT_ID = 1,
    UTS_NS_INIT_ID = 2,
    USER_NS_INIT_ID = 3,
    PID_NS_INIT_ID = 4,
    CGROUP_NS_INIT_ID = 5,
    TIME_NS_INIT_ID = 6,
    NET_NS_INIT_ID = 7,
    MNT_NS_INIT_ID = 8,
    // NS_LAST_INIT_ID is present when __KERNEL__ is defined.
}

#[repr(u64)]
pub enum ns_type {
    TIME_NS = 1u64 << 7,   /* CLONE_NEWTIME */
    MNT_NS = 1u64 << 17,   /* CLONE_NEWNS */
    CGROUP_NS = 1u64 << 25, /* CLONE_NEWCGROUP */
    UTS_NS = 1u64 << 26,   /* CLONE_NEWUTS */
    IPC_NS = 1u64 << 27,   /* CLONE_NEWIPC */
    USER_NS = 1u64 << 28,  /* CLONE_NEWUSER */
    PID_NS = 1u64 << 29,   /* CLONE_NEWPID */
    NET_NS = 1u64 << 30,   /* CLONE_NEWNET */
}

/**
 * struct ns_id_req - namespace ID request structure
 * @size: size of this structure
 * @spare: reserved for future use
 * @ns_id: last namespace ID
 * @ns_type: bit mask of namespace types to include
 * @spare2: reserved for future use
 * @user_ns_id: filter on this user namespace ID (or 0)
 *
 * Structure for passing namespace ID and miscellaneous parameters to
 * statns(2) and listns(2).
 *
 * For statns(2) @param represents the request mask.
 * For listns(2) @param represents the last listed mount id (or zero).
 */
#[repr(C)]
pub struct ns_id_req {
    pub size: u32,
    pub spare: u32,
    pub ns_id: u64,
    pub ns_type: u32,
    pub spare2: u32,
    pub user_ns_id: u64,
}

/* Special @user_ns_id value that can be passed to listns() */
pub const LISTNS_CURRENT_USER: u64 = 0xffffffffffffffff; /* Caller's userns */

/* List of all ns_id_req versions. */
pub const NS_ID_REQ_SIZE_VER0: usize = 32; /* sizeof first published struct */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
