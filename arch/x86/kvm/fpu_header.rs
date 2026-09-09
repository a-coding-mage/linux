/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

/* Swap (qemu) user FPU context for the guest FPU context. */
#[inline]
pub unsafe fn kvm_load_guest_fpu(vcpu: *mut kvm_vcpu) {
    if KVM_BUG_ON((*(*vcpu).arch.guest_fpu.fpstate).in_use, (*vcpu).kvm) {
        return;
    }

    /* Exclude PKRU, it's restored separately immediately after VM-Exit. */
    fpu_swap_kvm_fpstate(&mut (*vcpu).arch.guest_fpu, true);
    trace_kvm_fpu(1);
}

/* When vcpu_run ends, restore user space FPU context. */
#[inline]
pub unsafe fn kvm_put_guest_fpu(vcpu: *mut kvm_vcpu) {
    if KVM_BUG_ON(!(*(*vcpu).arch.guest_fpu.fpstate).in_use, (*vcpu).kvm) {
        return;
    }

    fpu_swap_kvm_fpstate(&mut (*vcpu).arch.guest_fpu, false);
    (*vcpu).stat.fpu_reload += 1;
    trace_kvm_fpu(0);
}

#[repr(simd)]
pub struct sse128_t(pub u32, pub u32, pub u32, pub u32);

#[repr(C)]
pub union __sse128_u {
    pub vec: sse128_t,
    pub as_u64: [u64; 2],
    pub as_u32: [u32; 4],
}

#[macro_export]
macro_rules! sse128_lo { ($x:expr) => {{ let t = __sse128_u { vec: $x }; unsafe { t.as_u64[0] } }}; }
#[macro_export]
macro_rules! sse128_hi { ($x:expr) => {{ let t = __sse128_u { vec: $x }; unsafe { t.as_u64[1] } }}; }
#[macro_export]
macro_rules! sse128_l0 { ($x:expr) => {{ let t = __sse128_u { vec: $x }; unsafe { t.as_u32[0] } }}; }
#[macro_export]
macro_rules! sse128_l1 { ($x:expr) => {{ let t = __sse128_u { vec: $x }; unsafe { t.as_u32[1] } }}; }
#[macro_export]
macro_rules! sse128_l2 { ($x:expr) => {{ let t = __sse128_u { vec: $x }; unsafe { t.as_u32[2] } }}; }
#[macro_export]
macro_rules! sse128_l3 { ($x:expr) => {{ let t = __sse128_u { vec: $x }; unsafe { t.as_u32[3] } }}; }
#[macro_export]
macro_rules! sse128 { ($lo:expr, $hi:expr) => {{ let mut t = __sse128_u { as_u64: [0; 2] }; unsafe { t.as_u64[0] = $lo; t.as_u64[1] = $hi; t.vec } }}; }

#[repr(simd)]
pub struct avx256_t(pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32);

#[inline]
pub unsafe fn _kvm_read_avx_reg(reg: i32, data: *mut avx256_t) {
    match reg {
        0..=15 => core::arch::asm!("vmovdqa ymm{0}, [{1}]", const reg, in(reg) data, options(nostack)),
        _ => BUG(),
    }
}

#[inline]
pub unsafe fn _kvm_write_avx_reg(reg: i32, data: *const avx256_t) {
    match reg {
        0..=15 => core::arch::asm!("vmovdqa [{1}], ymm{0}", const reg, in(reg) data, options(nostack)),
        _ => BUG(),
    }
}

#[inline]
pub unsafe fn _kvm_read_sse_reg(reg: i32, data: *mut sse128_t) {
    match reg {
        0..=15 => core::arch::asm!("movdqa xmm{0}, [{1}]", const reg, in(reg) data, options(nostack)),
        _ => BUG(),
    }
}

#[inline]
pub unsafe fn _kvm_write_sse_reg(reg: i32, data: *const sse128_t) {
    match reg {
        0..=15 => core::arch::asm!("movdqa [{1}], xmm{0}", const reg, in(reg) data, options(nostack)),
        _ => BUG(),
    }
}

#[inline]
pub unsafe fn _kvm_read_mmx_reg(reg: i32, data: *mut u64) {
    match reg {
        0..=7 => core::arch::asm!("movq mm{0}, [{1}]", const reg, in(reg) data, options(nostack)),
        _ => BUG(),
    }
}

#[inline]
pub unsafe fn _kvm_write_mmx_reg(reg: i32, data: *const u64) {
    match reg {
        0..=7 => core::arch::asm!("movq [{1}], mm{0}", const reg, in(reg) data, options(nostack)),
        _ => BUG(),
    }
}

#[inline]
pub unsafe fn kvm_fpu_get() {
    fpregs_lock();
    fpregs_assert_state_consistent();
    if test_thread_flag(TIF_NEED_FPU_LOAD) != 0 {
        switch_fpu_return();
    }
}

#[inline]
pub unsafe fn kvm_fpu_put() { fpregs_unlock(); }

#[inline]
pub unsafe fn kvm_read_avx_reg(reg: i32, data: *mut avx256_t) { kvm_fpu_get(); _kvm_read_avx_reg(reg, data); kvm_fpu_put(); }
#[inline]
pub unsafe fn kvm_write_avx_reg(reg: i32, data: *const avx256_t) { kvm_fpu_get(); _kvm_write_avx_reg(reg, data); kvm_fpu_put(); }
#[inline]
pub unsafe fn kvm_read_sse_reg(reg: i32, data: *mut sse128_t) { kvm_fpu_get(); _kvm_read_sse_reg(reg, data); kvm_fpu_put(); }
#[inline]
pub unsafe fn kvm_write_sse_reg(reg: i32, data: *const sse128_t) { kvm_fpu_get(); _kvm_write_sse_reg(reg, data); kvm_fpu_put(); }
#[inline]
pub unsafe fn kvm_read_mmx_reg(reg: i32, data: *mut u64) { kvm_fpu_get(); _kvm_read_mmx_reg(reg, data); kvm_fpu_put(); }
#[inline]
pub unsafe fn kvm_write_mmx_reg(reg: i32, data: *const u64) { kvm_fpu_get(); _kvm_write_mmx_reg(reg, data); kvm_fpu_put(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
