/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  KVM guest fault handling.
 *
 *    Copyright IBM Corp. 2025
 *    Author(s): Claudio Imbrenda <imbrenda@linux.ibm.com>
 */

// Dependency intent preserved from <linux/kvm_host.h> and "dat.h".

extern "C" {
    pub fn kvm_s390_faultin_gfn(
        vcpu: *mut crate::kvm_vcpu,
        kvm: *mut crate::kvm,
        f: *mut crate::guest_fault,
    ) -> ::core::ffi::c_int;
    pub fn kvm_s390_get_guest_page(
        kvm: *mut crate::kvm,
        f: *mut crate::guest_fault,
        gfn: crate::gfn_t,
        w: bool,
    ) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn kvm_s390_faultin_gfn_simple(
    vcpu: *mut crate::kvm_vcpu,
    kvm: *mut crate::kvm,
    gfn: crate::gfn_t,
    wr: bool,
) -> ::core::ffi::c_int {
    let mut f = crate::guest_fault {
        gfn,
        write_attempt: wr,
        ..::core::mem::zeroed()
    };
    kvm_s390_faultin_gfn(vcpu, kvm, &mut f)
}

#[inline]
pub unsafe fn kvm_s390_get_guest_page_and_read_gpa(
    kvm: *mut crate::kvm,
    f: *mut crate::guest_fault,
    gaddr: crate::gpa_t,
    val: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let rc = kvm_s390_get_guest_page(kvm, f, crate::gpa_to_gfn(gaddr), false);
    if rc != 0 {
        return rc;
    }
    *val = *(crate::phys_to_virt(
        crate::pfn_to_phys((*f).pfn) | crate::offset_in_page(gaddr),
    ) as *const ::core::ffi::c_ulong);
    0
}

#[inline]
pub unsafe fn kvm_s390_release_multiple(
    kvm: *mut crate::kvm,
    guest_faults: *mut crate::guest_fault,
    n: ::core::ffi::c_int,
    ignore: bool,
) {
    let mut i = 0;
    while i < n {
        let fault = &mut *guest_faults.add(i as usize);
        crate::kvm_release_faultin_page(kvm, fault.page, ignore, fault.write_attempt);
        fault.page = ::core::ptr::null_mut();
        i += 1;
    }
}

#[inline]
pub unsafe fn kvm_s390_multiple_faults_need_retry(
    kvm: *mut crate::kvm,
    seq: ::core::ffi::c_ulong,
    guest_faults: *mut crate::guest_fault,
    n: ::core::ffi::c_int,
    unsafe_: bool,
) -> bool {
    let mut i = 0;
    while i < n {
        let fault = &*guest_faults.add(i as usize);
        if !fault.valid {
            i += 1;
            continue;
        }
        if unsafe_ && crate::mmu_invalidate_retry_gfn_unsafe(kvm, seq, fault.gfn) {
            return true;
        }
        if !unsafe_ && crate::mmu_invalidate_retry_gfn(kvm, seq, fault.gfn) {
            return true;
        }
        i += 1;
    }
    false
}

#[inline]
pub unsafe fn kvm_s390_get_guest_pages(
    kvm: *mut crate::kvm,
    guest_faults: *mut crate::guest_fault,
    start: crate::gfn_t,
    n_pages: ::core::ffi::c_int,
    write_attempt: bool,
) -> ::core::ffi::c_int {
    let mut i = 0;
    let mut rc = 0;
    while i < n_pages {
        rc = kvm_s390_get_guest_page(kvm, guest_faults.add(i as usize), start + i as crate::gfn_t, write_attempt);
        if rc != 0 {
            break;
        }
        i += 1;
    }
    rc
}

#[macro_export]
macro_rules! kvm_s390_release_faultin_array {
    ($kvm:expr, $array:expr, $ignore:expr) => {
        $crate::kvm_s390_release_multiple($kvm, $array.as_mut_ptr(), $array.len() as _, $ignore)
    };
}

#[macro_export]
macro_rules! kvm_s390_array_needs_retry_unsafe {
    ($kvm:expr, $seq:expr, $array:expr) => {
        $crate::kvm_s390_multiple_faults_need_retry($kvm, $seq, $array.as_mut_ptr(), $array.len() as _, true)
    };
}

#[macro_export]
macro_rules! kvm_s390_array_needs_retry_safe {
    ($kvm:expr, $seq:expr, $array:expr) => {
        $crate::kvm_s390_multiple_faults_need_retry($kvm, $seq, $array.as_mut_ptr(), $array.len() as _, false)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
