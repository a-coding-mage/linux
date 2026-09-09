/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022 SiFive
 *
 * Authors:
 *     Vincent Chen <vincent.chen@sifive.com>
 *     Greentime Hu <greentime.hu@sifive.com>
 */

// C dependencies: linux/types.h, asm/vector.h, and asm/kvm_host.h.

#[cfg(CONFIG_RISCV_ISA_V)]
mod riscv_isa_v {
    use super::*;

    // These declarations are supplied by the corresponding kernel bindings.
    extern "C" {
        fn __riscv_v_vstate_save(vector: *mut riscv_v_ext_state, datap: *mut core::ffi::c_void);
        fn __riscv_v_vstate_restore(vector: *mut riscv_v_ext_state, datap: *mut core::ffi::c_void);
        fn has_vector() -> bool;
        pub fn kvm_riscv_vcpu_vector_reset(vcpu: *mut kvm_vcpu);
        pub fn kvm_riscv_vcpu_guest_vector_save(cntx: *mut kvm_cpu_context, isa: *mut c_ulong);
        pub fn kvm_riscv_vcpu_guest_vector_restore(cntx: *mut kvm_cpu_context, isa: *mut c_ulong);
        pub fn kvm_riscv_vcpu_host_vector_save(cntx: *mut kvm_cpu_context);
        pub fn kvm_riscv_vcpu_host_vector_restore(cntx: *mut kvm_cpu_context);
        pub fn kvm_riscv_vcpu_alloc_vector_context(vcpu: *mut kvm_vcpu) -> c_int;
        pub fn kvm_riscv_vcpu_free_vector_context(vcpu: *mut kvm_vcpu);
        pub fn kvm_riscv_register_vctx_callback(func: Option<unsafe extern "C" fn()>);
        pub fn kvm_riscv_unregister_vctx_callback();
        pub fn kvm_riscv_vcpu_flush_vector();
    }

    #[inline(always)]
    pub unsafe fn __kvm_riscv_vector_save(context: *mut kvm_cpu_context) {
        __riscv_v_vstate_save(&mut (*context).vector, (*context).vector.datap);
    }

    #[inline(always)]
    pub unsafe fn __kvm_riscv_vector_restore(context: *mut kvm_cpu_context) {
        __riscv_v_vstate_restore(&mut (*context).vector, (*context).vector.datap);
    }

    #[inline]
    pub unsafe fn kvm_riscv_v_init() {
        if has_vector() {
            kvm_riscv_register_vctx_callback(Some(kvm_riscv_vcpu_flush_vector));
        }
    }

    #[inline]
    pub unsafe fn kvm_riscv_v_exit() {
        if has_vector() {
            kvm_riscv_unregister_vctx_callback();
        }
    }
}

#[cfg(not(CONFIG_RISCV_ISA_V))]
mod riscv_isa_v {
    use super::*;

    #[inline]
    pub unsafe fn kvm_riscv_vcpu_vector_reset(_vcpu: *mut kvm_vcpu) {}

    #[inline]
    pub unsafe fn kvm_riscv_vcpu_guest_vector_save(
        _cntx: *mut kvm_cpu_context,
        _isa: *mut c_ulong,
    ) {}

    #[inline]
    pub unsafe fn kvm_riscv_vcpu_guest_vector_restore(
        _cntx: *mut kvm_cpu_context,
        _isa: *mut c_ulong,
    ) {}

    #[inline]
    pub unsafe fn kvm_riscv_vcpu_host_vector_save(_cntx: *mut kvm_cpu_context) {}

    #[inline]
    pub unsafe fn kvm_riscv_vcpu_host_vector_restore(_cntx: *mut kvm_cpu_context) {}

    #[inline]
    pub unsafe fn kvm_riscv_vcpu_alloc_vector_context(_vcpu: *mut kvm_vcpu) -> c_int {
        0
    }

    #[inline]
    pub unsafe fn kvm_riscv_vcpu_free_vector_context(_vcpu: *mut kvm_vcpu) {}

    #[inline]
    pub unsafe fn kvm_riscv_v_init() {}

    #[inline]
    pub unsafe fn kvm_riscv_v_exit() {}
}

// External types and integer aliases are provided by the translated kernel headers.
pub use riscv_isa_v::*;

extern "C" {
    pub fn kvm_riscv_vcpu_get_reg_vector(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
    ) -> c_int;
    pub fn kvm_riscv_vcpu_set_reg_vector(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
