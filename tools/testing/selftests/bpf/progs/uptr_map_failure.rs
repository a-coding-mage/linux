// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "uptr_test_common.h"

#[repr(C)]
pub struct large_uptr_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: ::core::marker::PhantomData<i32>,
    pub value: ::core::marker::PhantomData<large_uptr>,
}

#[used]
#[no_mangle]
#[link_section = ".maps"]
pub static large_uptr_map: large_uptr_map_def = large_uptr_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: ::core::marker::PhantomData,
    value: ::core::marker::PhantomData,
};

#[repr(C)]
pub struct empty_uptr_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: ::core::marker::PhantomData<i32>,
    pub value: ::core::marker::PhantomData<empty_uptr>,
}

#[used]
#[no_mangle]
#[link_section = ".maps"]
pub static empty_uptr_map: empty_uptr_map_def = empty_uptr_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: ::core::marker::PhantomData,
    value: ::core::marker::PhantomData,
};

#[repr(C)]
pub struct kstruct_uptr_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: ::core::marker::PhantomData<i32>,
    pub value: ::core::marker::PhantomData<kstruct_uptr>,
}

#[used]
#[no_mangle]
#[link_section = ".maps"]
pub static kstruct_uptr_map: kstruct_uptr_map_def = kstruct_uptr_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: ::core::marker::PhantomData,
    value: ::core::marker::PhantomData,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
