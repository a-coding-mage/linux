/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from linux/userfaultfd.h. */

// Dependency supplied externally: linux types and ioctl encoding helpers.

pub const USERFAULTFD_IOC: u32 = 0xAA;
pub const USERFAULTFD_IOC_NEW: _ = _IO(USERFAULTFD_IOC, 0x00);

pub const UFFD_API: __u64 = 0xAA;
pub const UFFD_API_REGISTER_MODES: __u64 = UFFDIO_REGISTER_MODE_MISSING
    | UFFDIO_REGISTER_MODE_WP
    | UFFDIO_REGISTER_MODE_MINOR
    | UFFDIO_REGISTER_MODE_RWP;
pub const UFFD_API_FEATURES: __u64 = UFFD_FEATURE_PAGEFAULT_FLAG_WP
    | UFFD_FEATURE_EVENT_FORK
    | UFFD_FEATURE_EVENT_REMAP
    | UFFD_FEATURE_EVENT_REMOVE
    | UFFD_FEATURE_EVENT_UNMAP
    | UFFD_FEATURE_MISSING_HUGETLBFS
    | UFFD_FEATURE_MISSING_SHMEM
    | UFFD_FEATURE_SIGBUS
    | UFFD_FEATURE_THREAD_ID
    | UFFD_FEATURE_MINOR_HUGETLBFS
    | UFFD_FEATURE_MINOR_SHMEM
    | UFFD_FEATURE_EXACT_ADDRESS
    | UFFD_FEATURE_WP_HUGETLBFS_SHMEM
    | UFFD_FEATURE_WP_UNPOPULATED
    | UFFD_FEATURE_POISON
    | UFFD_FEATURE_WP_ASYNC
    | UFFD_FEATURE_MOVE
    | UFFD_FEATURE_RWP
    | UFFD_FEATURE_RWP_ASYNC;
pub const UFFD_API_IOCTLS: __u64 = (1u64 << _UFFDIO_REGISTER)
    | (1u64 << _UFFDIO_UNREGISTER)
    | (1u64 << _UFFDIO_SET_MODE)
    | (1u64 << _UFFDIO_API);
pub const UFFD_API_RANGE_IOCTLS: __u64 = (1u64 << _UFFDIO_WAKE)
    | (1u64 << _UFFDIO_COPY)
    | (1u64 << _UFFDIO_ZEROPAGE)
    | (1u64 << _UFFDIO_MOVE)
    | (1u64 << _UFFDIO_WRITEPROTECT)
    | (1u64 << _UFFDIO_CONTINUE)
    | (1u64 << _UFFDIO_POISON)
    | (1u64 << _UFFDIO_RWPROTECT);
pub const UFFD_API_RANGE_IOCTLS_BASIC: __u64 = (1u64 << _UFFDIO_WAKE)
    | (1u64 << _UFFDIO_COPY)
    | (1u64 << _UFFDIO_WRITEPROTECT)
    | (1u64 << _UFFDIO_CONTINUE)
    | (1u64 << _UFFDIO_POISON)
    | (1u64 << _UFFDIO_RWPROTECT);

pub const _UFFDIO_REGISTER: u32 = 0x00;
pub const _UFFDIO_UNREGISTER: u32 = 0x01;
pub const _UFFDIO_WAKE: u32 = 0x02;
pub const _UFFDIO_COPY: u32 = 0x03;
pub const _UFFDIO_ZEROPAGE: u32 = 0x04;
pub const _UFFDIO_MOVE: u32 = 0x05;
pub const _UFFDIO_WRITEPROTECT: u32 = 0x06;
pub const _UFFDIO_CONTINUE: u32 = 0x07;
pub const _UFFDIO_POISON: u32 = 0x08;
pub const _UFFDIO_RWPROTECT: u32 = 0x09;
pub const _UFFDIO_SET_MODE: u32 = 0x0A;
pub const _UFFDIO_API: u32 = 0x3F;

pub const UFFDIO: u32 = 0xAA;
pub const UFFDIO_API: _ = _IOWR(UFFDIO, _UFFDIO_API, uffdio_api);
pub const UFFDIO_REGISTER: _ = _IOWR(UFFDIO, _UFFDIO_REGISTER, uffdio_register);
pub const UFFDIO_UNREGISTER: _ = _IOR(UFFDIO, _UFFDIO_UNREGISTER, uffdio_range);
pub const UFFDIO_WAKE: _ = _IOR(UFFDIO, _UFFDIO_WAKE, uffdio_range);
pub const UFFDIO_COPY: _ = _IOWR(UFFDIO, _UFFDIO_COPY, uffdio_copy);
pub const UFFDIO_ZEROPAGE: _ = _IOWR(UFFDIO, _UFFDIO_ZEROPAGE, uffdio_zeropage);
pub const UFFDIO_MOVE: _ = _IOWR(UFFDIO, _UFFDIO_MOVE, uffdio_move);
pub const UFFDIO_WRITEPROTECT: _ = _IOWR(UFFDIO, _UFFDIO_WRITEPROTECT, uffdio_writeprotect);
pub const UFFDIO_CONTINUE: _ = _IOWR(UFFDIO, _UFFDIO_CONTINUE, uffdio_continue);
pub const UFFDIO_POISON: _ = _IOWR(UFFDIO, _UFFDIO_POISON, uffdio_poison);
pub const UFFDIO_RWPROTECT: _ = _IOWR(UFFDIO, _UFFDIO_RWPROTECT, uffdio_rwprotect);
pub const UFFDIO_SET_MODE: _ = _IOW(UFFDIO, _UFFDIO_SET_MODE, uffdio_set_mode);

#[repr(C, packed)]
pub struct uffd_msg {
    pub event: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub arg: uffd_msg__bindgen_ty_1,
}
#[repr(C)]
pub union uffd_msg__bindgen_ty_1 {
    pub pagefault: uffd_msg__bindgen_ty_1__bindgen_ty_1,
    pub fork: uffd_msg__bindgen_ty_1__bindgen_ty_2,
    pub remap: uffd_msg__bindgen_ty_1__bindgen_ty_3,
    pub remove: uffd_msg__bindgen_ty_1__bindgen_ty_4,
    pub reserved: uffd_msg__bindgen_ty_1__bindgen_ty_5,
}
#[repr(C)]
pub struct uffd_msg__bindgen_ty_1__bindgen_ty_1 { pub flags: __u64, pub address: __u64, pub feat: uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 }
#[repr(C)]
pub union uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 { pub ptid: __u32 }
#[repr(C)] pub struct uffd_msg__bindgen_ty_1__bindgen_ty_2 { pub ufd: __u32 }
#[repr(C)] pub struct uffd_msg__bindgen_ty_1__bindgen_ty_3 { pub from: __u64, pub to: __u64, pub len: __u64 }
#[repr(C)] pub struct uffd_msg__bindgen_ty_1__bindgen_ty_4 { pub start: __u64, pub end: __u64 }
#[repr(C)] pub struct uffd_msg__bindgen_ty_1__bindgen_ty_5 { pub reserved1: __u64, pub reserved2: __u64, pub reserved3: __u64 }

pub const UFFD_EVENT_PAGEFAULT: u8 = 0x12;
pub const UFFD_EVENT_FORK: u8 = 0x13;
pub const UFFD_EVENT_REMAP: u8 = 0x14;
pub const UFFD_EVENT_REMOVE: u8 = 0x15;
pub const UFFD_EVENT_UNMAP: u8 = 0x16;
pub const UFFD_PAGEFAULT_FLAG_WRITE: __u64 = 1 << 0;
pub const UFFD_PAGEFAULT_FLAG_WP: __u64 = 1 << 1;
pub const UFFD_PAGEFAULT_FLAG_MINOR: __u64 = 1 << 2;
pub const UFFD_PAGEFAULT_FLAG_RWP: __u64 = 1 << 3;

pub const UFFD_FEATURE_PAGEFAULT_FLAG_WP: __u64 = 1 << 0;
pub const UFFD_FEATURE_EVENT_FORK: __u64 = 1 << 1;
pub const UFFD_FEATURE_EVENT_REMAP: __u64 = 1 << 2;
pub const UFFD_FEATURE_EVENT_REMOVE: __u64 = 1 << 3;
pub const UFFD_FEATURE_MISSING_HUGETLBFS: __u64 = 1 << 4;
pub const UFFD_FEATURE_MISSING_SHMEM: __u64 = 1 << 5;
pub const UFFD_FEATURE_EVENT_UNMAP: __u64 = 1 << 6;
pub const UFFD_FEATURE_SIGBUS: __u64 = 1 << 7;
pub const UFFD_FEATURE_THREAD_ID: __u64 = 1 << 8;
pub const UFFD_FEATURE_MINOR_HUGETLBFS: __u64 = 1 << 9;
pub const UFFD_FEATURE_MINOR_SHMEM: __u64 = 1 << 10;
pub const UFFD_FEATURE_EXACT_ADDRESS: __u64 = 1 << 11;
pub const UFFD_FEATURE_WP_HUGETLBFS_SHMEM: __u64 = 1 << 12;
pub const UFFD_FEATURE_WP_UNPOPULATED: __u64 = 1 << 13;
pub const UFFD_FEATURE_POISON: __u64 = 1 << 14;
pub const UFFD_FEATURE_WP_ASYNC: __u64 = 1 << 15;
pub const UFFD_FEATURE_MOVE: __u64 = 1 << 16;
pub const UFFD_FEATURE_RWP: __u64 = 1 << 17;
pub const UFFD_FEATURE_RWP_ASYNC: __u64 = 1 << 18;

#[repr(C)] pub struct uffdio_api { pub api: __u64, pub features: __u64, pub ioctls: __u64 }
#[repr(C)] pub struct uffdio_range { pub start: __u64, pub len: __u64 }
pub const UFFDIO_REGISTER_MODE_MISSING: __u64 = 1 << 0;
pub const UFFDIO_REGISTER_MODE_WP: __u64 = 1 << 1;
pub const UFFDIO_REGISTER_MODE_MINOR: __u64 = 1 << 2;
pub const UFFDIO_REGISTER_MODE_RWP: __u64 = 1 << 3;
#[repr(C)] pub struct uffdio_register { pub range: uffdio_range, pub mode: __u64, pub ioctls: __u64 }
pub const UFFDIO_COPY_MODE_DONTWAKE: __u64 = 1 << 0;
pub const UFFDIO_COPY_MODE_WP: __u64 = 1 << 1;
#[repr(C)] pub struct uffdio_copy { pub dst: __u64, pub src: __u64, pub len: __u64, pub mode: __u64, pub copy: __s64 }
pub const UFFDIO_ZEROPAGE_MODE_DONTWAKE: __u64 = 1 << 0;
#[repr(C)] pub struct uffdio_zeropage { pub range: uffdio_range, pub mode: __u64, pub zeropage: __s64 }
pub const UFFDIO_WRITEPROTECT_MODE_WP: __u64 = 1 << 0;
pub const UFFDIO_WRITEPROTECT_MODE_DONTWAKE: __u64 = 1 << 1;
#[repr(C)] pub struct uffdio_writeprotect { pub range: uffdio_range, pub mode: __u64 }
pub const UFFDIO_CONTINUE_MODE_DONTWAKE: __u64 = 1 << 0;
pub const UFFDIO_CONTINUE_MODE_WP: __u64 = 1 << 1;
#[repr(C)] pub struct uffdio_continue { pub range: uffdio_range, pub mode: __u64, pub mapped: __s64 }
pub const UFFDIO_POISON_MODE_DONTWAKE: __u64 = 1 << 0;
#[repr(C)] pub struct uffdio_poison { pub range: uffdio_range, pub mode: __u64, pub updated: __s64 }
pub const UFFDIO_RWPROTECT_MODE_RWP: __u64 = 1 << 0;
pub const UFFDIO_RWPROTECT_MODE_DONTWAKE: __u64 = 1 << 1;
#[repr(C)] pub struct uffdio_rwprotect { pub range: uffdio_range, pub mode: __u64 }
pub const UFFDIO_MOVE_MODE_DONTWAKE: __u64 = 1 << 0;
pub const UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES: __u64 = 1 << 1;
#[repr(C)] pub struct uffdio_move { pub dst: __u64, pub src: __u64, pub len: __u64, pub mode: __u64, pub r#move: __s64 }
#[repr(C)] pub struct uffdio_set_mode { pub enable: __u64, pub disable: __u64 }

pub const UFFD_USER_MODE_ONLY: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
