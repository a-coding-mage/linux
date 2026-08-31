// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/*
 * C includes translated as external dependencies expected from the BPF build:
 * <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>, "bpf_misc.h",
 * <bpf_arena_common.h>, and <bpf_arena_spin_lock.h>.
 */

#[repr(C)]
pub struct ArenaMapDef {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub map_extra: u64,
}

extern "C" {
    static BPF_MAP_TYPE_ARENA: u32;
    static BPF_F_MMAPABLE: u32;
}

/*
 * The C source selects map_extra with:
 *   #ifdef __TARGET_ARCH_arm64
 *       0x1ull << 32
 *   #else
 *       0x1ull << 44
 *   #endif
 * Keep the same target-architecture intent in Rust cfg form.
 */
#[cfg(target_arch = "aarch64")]
const ARENA_MAP_EXTRA: u64 = 0x1u64 << 32;
#[cfg(not(target_arch = "aarch64"))]
const ARENA_MAP_EXTRA: u64 = 0x1u64 << 44;

#[link_section = ".maps"]
#[no_mangle]
pub static mut arena: ArenaMapDef = ArenaMapDef {
    type_: unsafe { BPF_MAP_TYPE_ARENA },
    map_flags: unsafe { BPF_F_MMAPABLE },
    max_entries: 100, /* number of pages */
    map_extra: ARENA_MAP_EXTRA, /* start of mmap() region */
};

#[no_mangle]
pub static mut cs_count: ::core::ffi::c_int = 0;

/*
 * Original condition:
 *   #if defined(ENABLE_ATOMICS_TESTS) && defined(__BPF_FEATURE_ADDR_SPACE_CAST)
 * Rust cannot observe these C preprocessor definitions file-locally; the cfg
 * names below preserve the intended build-time condition for translated code.
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
extern "C" {
    pub type arena_spinlock_t;
    pub type arena_qnode;

    static _Q_MAX_CPUS: usize;
    static _Q_MAX_NODES: usize;

    fn arena_spin_lock_irqsave(
        lock: *mut arena_spinlock_t,
        flags: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    fn arena_spin_unlock_irqrestore(lock: *mut arena_spinlock_t, flags: ::core::ffi::c_ulong);
    fn bpf_repeat(count: ::core::ffi::c_int);
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
extern "C" {
    static EOPNOTSUPP: ::core::ffi::c_int;
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[no_mangle]
pub static mut lock: arena_spinlock_t;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[no_mangle]
pub static mut test_skip: ::core::ffi::c_int = 1;

/*
 * Storage for the queue nodes declared by bpf_arena_spin_lock.h. Each program
 * linking the arena spinlock provides exactly one definition; libarena's lives
 * in libarena/src/common.bpf.c.
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[no_mangle]
#[link_section = ".arena"]
pub static mut qnodes: [[arena_qnode; unsafe { _Q_MAX_NODES }]; unsafe { _Q_MAX_CPUS }];

#[cfg(not(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST)))]
#[no_mangle]
pub static mut test_skip: ::core::ffi::c_int = 2;

#[no_mangle]
pub static mut counter: ::core::ffi::c_int = 0;

#[no_mangle]
pub static mut limit: ::core::ffi::c_int = 0;

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn prog(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -2;

    #[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
    {
        let flags: ::core::ffi::c_ulong;

        ret = arena_spin_lock_irqsave(&mut lock, flags);
        if ret != 0 {
            if ret == -EOPNOTSUPP {
                test_skip = 3;
            }
            return ret;
        }
        if counter != limit {
            counter += 1;
        }
        bpf_repeat(cs_count);
        ret = 0;
        arena_spin_unlock_irqrestore(&mut lock, flags);
    }

    let _ = ctx;
    ret
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
