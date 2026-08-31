// SPDX-License-Identifier: GPL-2.0-only
/*
 * kvm asynchronous fault support
 *
 * Copyright 2010 Red Hat, Inc.
 *
 * Author:
 *      Gleb Natapov <gleb@redhat.com>
 */

// C header guard omitted in Rust.

// Original C conditional:
// #ifdef CONFIG_KVM_ASYNC_PF

extern "C" {
    pub fn kvm_async_pf_init() -> core::ffi::c_int;
    pub fn kvm_async_pf_deinit();
    pub fn kvm_async_pf_vcpu_init(vcpu: *mut kvm_vcpu);
}

// #else
// #define kvm_async_pf_init() (0)
// #define kvm_async_pf_deinit() do {} while (0)
// #define kvm_async_pf_vcpu_init(C) do {} while (0)
//
// If CONFIG_KVM_ASYNC_PF is disabled, the C header provides no-op macro
// fallbacks equivalent to:
// pub const fn kvm_async_pf_init() -> core::ffi::c_int { 0 }
// pub const fn kvm_async_pf_deinit() {}
// pub const fn kvm_async_pf_vcpu_init<C>(_c: C) {}
// #endif
