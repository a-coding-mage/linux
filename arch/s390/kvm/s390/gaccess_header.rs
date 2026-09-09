/* SPDX-License-Identifier: GPL-2.0 */
/*
 * access guest memory
 *
 * Copyright IBM Corp. 2008, 2014
 *
 *    Author(s): Carsten Otte <cotte@de.ibm.com>
 */

// Translated from gaccess.h. Dependencies are supplied by the surrounding kernel bindings.

pub unsafe fn _kvm_s390_real_to_abs(prefix: u32, mut gra: libc::c_ulong) -> libc::c_ulong {
    if gra < 2 * PAGE_SIZE {
        gra = gra.wrapping_add(prefix as libc::c_ulong);
    } else if gra >= prefix as libc::c_ulong
        && gra < (prefix as libc::c_ulong).wrapping_add(2 * PAGE_SIZE)
    {
        gra = gra.wrapping_sub(prefix as libc::c_ulong);
    }
    gra
}

pub unsafe fn kvm_s390_real_to_abs(
    vcpu: *mut kvm_vcpu,
    gra: libc::c_ulong,
) -> libc::c_ulong {
    _kvm_s390_real_to_abs(kvm_s390_get_prefix(vcpu), gra)
}

pub unsafe fn _kvm_s390_logical_to_effective(
    psw: *mut psw_t,
    ga: libc::c_ulong,
) -> libc::c_ulong {
    if psw_bits(*psw).eaba == PSW_BITS_AMODE_64BIT {
        return ga;
    }
    if psw_bits(*psw).eaba == PSW_BITS_AMODE_31BIT {
        return ga & ((1 as libc::c_ulong).wrapping_shl(31)).wrapping_sub(1);
    }
    ga & ((1 as libc::c_ulong).wrapping_shl(24)).wrapping_sub(1)
}

pub unsafe fn kvm_s390_logical_to_effective(
    vcpu: *mut kvm_vcpu,
    ga: libc::c_ulong,
) -> libc::c_ulong {
    _kvm_s390_logical_to_effective(&mut (*(*vcpu).arch.sie_block).gpsw, ga)
}

pub unsafe fn put_guest_lc<T>(vcpu: *mut kvm_vcpu, x: T, gra: *const T) -> libc::c_int {
    let mut value = x;
    let gpa = (gra as libc::c_ulong).wrapping_add(kvm_s390_get_prefix(vcpu) as libc::c_ulong);
    kvm_write_guest((*vcpu).kvm, gpa, &mut value as *mut T as *mut libc::c_void,
                    core::mem::size_of::<T>() as libc::c_ulong)
}

pub unsafe fn write_guest_lc(
    vcpu: *mut kvm_vcpu,
    gra: libc::c_ulong,
    data: *mut libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int {
    let gpa = gra.wrapping_add(kvm_s390_get_prefix(vcpu) as libc::c_ulong);
    kvm_write_guest((*vcpu).kvm, gpa, data, len)
}

pub unsafe fn read_guest_lc(
    vcpu: *mut kvm_vcpu,
    gra: libc::c_ulong,
    data: *mut libc::c_void,
    len: libc::c_ulong,
) -> libc::c_int {
    let gpa = gra.wrapping_add(kvm_s390_get_prefix(vcpu) as libc::c_ulong);
    kvm_read_guest((*vcpu).kvm, gpa, data, len)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gacc_mode {
    GACC_FETCH,
    GACC_STORE,
    GACC_IFETCH,
}

extern "C" {
    pub fn guest_translate_address_with_key(vcpu: *mut kvm_vcpu, gva: libc::c_ulong, ar: u8,
        gpa: *mut libc::c_ulong, mode: gacc_mode, access_key: u8) -> libc::c_int;
    pub fn check_gva_range(vcpu: *mut kvm_vcpu, gva: libc::c_ulong, ar: u8,
        length: libc::c_ulong, mode: gacc_mode, access_key: u8) -> libc::c_int;
    pub fn check_gpa_range(kvm: *mut kvm, gpa: libc::c_ulong, length: libc::c_ulong,
        mode: gacc_mode, access_key: u8) -> libc::c_int;
    pub fn access_guest_abs_with_key(kvm: *mut kvm, gpa: gpa_t, data: *mut libc::c_void,
        len: libc::c_ulong, mode: gacc_mode, access_key: u8) -> libc::c_int;
    pub fn access_guest_with_key(vcpu: *mut kvm_vcpu, ga: libc::c_ulong, ar: u8,
        data: *mut libc::c_void, len: libc::c_ulong, mode: gacc_mode, access_key: u8) -> libc::c_int;
    pub fn access_guest_real(vcpu: *mut kvm_vcpu, gra: libc::c_ulong, data: *mut libc::c_void,
        len: libc::c_ulong, mode: gacc_mode) -> libc::c_int;
    pub fn cmpxchg_guest_abs_with_key(kvm: *mut kvm, gpa: gpa_t, len: libc::c_int,
        old: *mut kvm_s390_quad, new: kvm_s390_quad, access_key: u8, success: *mut bool) -> libc::c_int;
}

pub unsafe fn write_guest_with_key(vcpu: *mut kvm_vcpu, ga: libc::c_ulong, ar: u8,
    data: *mut libc::c_void, len: libc::c_ulong, access_key: u8) -> libc::c_int {
    access_guest_with_key(vcpu, ga, ar, data, len, gacc_mode::GACC_STORE, access_key)
}

pub unsafe fn write_guest(vcpu: *mut kvm_vcpu, ga: libc::c_ulong, ar: u8,
    data: *mut libc::c_void, len: libc::c_ulong) -> libc::c_int {
    let access_key = psw_bits((*(*vcpu).arch.sie_block).gpsw).key;
    write_guest_with_key(vcpu, ga, ar, data, len, access_key)
}

pub unsafe fn read_guest_with_key(vcpu: *mut kvm_vcpu, ga: libc::c_ulong, ar: u8,
    data: *mut libc::c_void, len: libc::c_ulong, access_key: u8) -> libc::c_int {
    access_guest_with_key(vcpu, ga, ar, data, len, gacc_mode::GACC_FETCH, access_key)
}

pub unsafe fn read_guest(vcpu: *mut kvm_vcpu, ga: libc::c_ulong, ar: u8,
    data: *mut libc::c_void, len: libc::c_ulong) -> libc::c_int {
    let access_key = psw_bits((*(*vcpu).arch.sie_block).gpsw).key;
    read_guest_with_key(vcpu, ga, ar, data, len, access_key)
}

pub unsafe fn read_guest_instr(vcpu: *mut kvm_vcpu, ga: libc::c_ulong,
    data: *mut libc::c_void, len: libc::c_ulong) -> libc::c_int {
    let access_key = psw_bits((*(*vcpu).arch.sie_block).gpsw).key;
    access_guest_with_key(vcpu, ga, 0, data, len, gacc_mode::GACC_IFETCH, access_key)
}

pub unsafe fn write_guest_abs(vcpu: *mut kvm_vcpu, gpa: libc::c_ulong,
    data: *mut libc::c_void, len: libc::c_ulong) -> libc::c_int {
    kvm_write_guest((*vcpu).kvm, gpa, data, len)
}

pub unsafe fn read_guest_abs(vcpu: *mut kvm_vcpu, gpa: libc::c_ulong,
    data: *mut libc::c_void, len: libc::c_ulong) -> libc::c_int {
    kvm_read_guest((*vcpu).kvm, gpa, data, len)
}

pub unsafe fn write_guest_real(vcpu: *mut kvm_vcpu, gra: libc::c_ulong,
    data: *mut libc::c_void, len: libc::c_ulong) -> libc::c_int {
    access_guest_real(vcpu, gra, data, len, gacc_mode::GACC_STORE)
}

pub unsafe fn read_guest_real(vcpu: *mut kvm_vcpu, gra: libc::c_ulong,
    data: *mut libc::c_void, len: libc::c_ulong) -> libc::c_int {
    access_guest_real(vcpu, gra, data, len, gacc_mode::GACC_FETCH)
}

extern "C" {
    pub fn ipte_lock(kvm: *mut kvm);
    pub fn ipte_unlock(kvm: *mut kvm);
    pub fn ipte_lock_held(kvm: *mut kvm) -> libc::c_int;
    pub fn kvm_s390_check_low_addr_prot_real(vcpu: *mut kvm_vcpu, gra: libc::c_ulong) -> libc::c_int;
}

#[repr(C)]
pub union mvpg_pei {
    pub val: libc::c_ulong,
    pub fields: mvpg_pei_fields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpg_pei_fields {
    pub addr: libc::c_ulong,
    pub not_pte: libc::c_ulong,
    pub dat_prot: libc::c_ulong,
    pub real: libc::c_ulong,
}

extern "C" {
    pub fn gaccess_shadow_fault(vcpu: *mut kvm_vcpu, sg: *mut gmap, saddr: gpa_t,
        datptr: *mut mvpg_pei, wr: bool) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
