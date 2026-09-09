// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 */

// Dependencies are provided by the surrounding KVM implementation.

pub unsafe fn kvm_mmio_write_buf(buf: *mut core::ffi::c_void, len: u32, data: usize) {
    let mut tmp = [0u8; 8];
    match len {
        1 => tmp[..1].copy_from_slice(&(data as u8).to_ne_bytes()),
        2 => tmp[..2].copy_from_slice(&(data as u16).to_ne_bytes()),
        4 => tmp[..4].copy_from_slice(&(data as u32).to_ne_bytes()),
        8 => tmp[..8].copy_from_slice(&(data as u64).to_ne_bytes()),
        _ => {}
    }
    core::ptr::copy_nonoverlapping(tmp.as_ptr(), buf as *mut u8, len as usize);
}

pub unsafe fn kvm_mmio_read_buf(buf: *const core::ffi::c_void, len: u32) -> usize {
    let mut tmp = [0u8; 8];
    let mut data = 0usize;
    match len {
        1 => data = *(buf as *const u8) as usize,
        2 => {
            core::ptr::copy_nonoverlapping(buf as *const u8, tmp.as_mut_ptr(), len as usize);
            data = u16::from_ne_bytes([tmp[0], tmp[1]]) as usize;
        }
        4 => {
            core::ptr::copy_nonoverlapping(buf as *const u8, tmp.as_mut_ptr(), len as usize);
            data = u32::from_ne_bytes([tmp[0], tmp[1], tmp[2], tmp[3]]) as usize;
        }
        8 => {
            core::ptr::copy_nonoverlapping(buf as *const u8, tmp.as_mut_ptr(), len as usize);
            data = u64::from_ne_bytes(tmp) as usize;
        }
        _ => {}
    }
    data
}

unsafe fn kvm_pending_external_abort(vcpu: *mut kvm_vcpu) -> bool {
    if !vcpu_get_flag(vcpu, PENDING_EXCEPTION) { return false; }
    if vcpu_el1_is_32bit(vcpu) {
        matches!(vcpu_get_flag(vcpu, EXCEPT_MASK),
            unpack_vcpu_flag(EXCEPT_AA32_UND) |
            unpack_vcpu_flag(EXCEPT_AA32_IABT) |
            unpack_vcpu_flag(EXCEPT_AA32_DABT))
    } else {
        matches!(vcpu_get_flag(vcpu, EXCEPT_MASK),
            unpack_vcpu_flag(EXCEPT_AA64_EL1_SYNC) |
            unpack_vcpu_flag(EXCEPT_AA64_EL2_SYNC) |
            unpack_vcpu_flag(EXCEPT_AA64_EL1_SERR) |
            unpack_vcpu_flag(EXCEPT_AA64_EL2_SERR))
    }
}

/// Handle MMIO loads after user space emulation or in-kernel IO emulation.
pub unsafe fn kvm_handle_mmio_return(vcpu: *mut kvm_vcpu) -> i32 {
    let mut data: usize;
    let len: u32;
    let mut mask: i32;
    if !(*vcpu).mmio_needed || kvm_pending_external_abort(vcpu) { return 1; }
    (*vcpu).mmio_needed = 0;
    if !kvm_vcpu_dabt_iswrite(vcpu) {
        let run = (*vcpu).run;
        len = kvm_vcpu_dabt_get_as(vcpu);
        data = kvm_mmio_read_buf((*run).mmio.data.as_ptr() as *const _, len);
        trace_kvm_mmio(KVM_TRACE_MMIO_READ, len, (*run).mmio.phys_addr, &data);
        data = vcpu_data_host_to_guest(vcpu, data, len);
        if kvm_vcpu_dabt_issext(vcpu) && (len as usize) < core::mem::size_of::<usize>() {
            mask = 1i32 << ((len * 8) - 1);
            data = (data ^ mask as usize).wrapping_sub(mask as usize);
        }
        if !kvm_vcpu_dabt_issf(vcpu) { data &= 0xffff_ffff; }
        vcpu_set_reg(vcpu, kvm_vcpu_dabt_get_rd(vcpu), data);
    }
    kvm_incr_pc(vcpu);
    1
}

pub unsafe fn io_mem_abort(vcpu: *mut kvm_vcpu, fault_ipa: phys_addr_t) -> i32 {
    let run = (*vcpu).run;
    let mut data: usize;
    let mut rt: usize;
    let mut ret: i32;
    let is_write: bool;
    let len: i32;
    let mut data_buf = [0u8; 8];
    let esr = kvm_vcpu_get_esr(vcpu);
    if !kvm_vcpu_dabt_isvalid(vcpu) {
        trace_kvm_mmio_nisv(*vcpu_pc(vcpu), esr, kvm_vcpu_get_hfar(vcpu), fault_ipa);
        if vcpu_is_protected(vcpu) { return kvm_inject_sea_dabt(vcpu, kvm_vcpu_get_hfar(vcpu)); }
        if test_bit(KVM_ARCH_FLAG_RETURN_NISV_IO_ABORT_TO_USER, &(*(*vcpu).kvm).arch.flags) {
            (*run).exit_reason = KVM_EXIT_ARM_NISV;
            (*run).arm_nisv.esr_iss = kvm_vcpu_dabt_iss_nisv_sanitized(vcpu);
            (*run).arm_nisv.fault_ipa = fault_ipa;
            return 0;
        }
        return -ENOSYS;
    }
    match kvm_vcpu_trap_get_fault(vcpu) {
        0b000100..=0b001111 | 0b101010..=0b101011 => {
            if FIELD_GET(GENMASK(12, 11), esr) != 0 {
                (*run).exit_reason = KVM_EXIT_ARM_LDST64B;
                (*run).arm_nisv.esr_iss = esr & !(ESR_ELx_FSC as u64);
                (*run).arm_nisv.fault_ipa = fault_ipa;
                return 0;
            }
        }
        _ => {}
    }
    is_write = kvm_vcpu_dabt_iswrite(vcpu);
    len = kvm_vcpu_dabt_get_as(vcpu) as i32;
    rt = kvm_vcpu_dabt_get_rd(vcpu) as usize;
    if is_write {
        data = vcpu_data_guest_to_host(vcpu, vcpu_get_reg(vcpu, rt), len as u32);
        trace_kvm_mmio(KVM_TRACE_MMIO_WRITE, len as u32, fault_ipa, &data);
        kvm_mmio_write_buf(data_buf.as_mut_ptr() as *mut _, len as u32, data);
        ret = kvm_io_bus_write(vcpu, KVM_MMIO_BUS, fault_ipa, len, data_buf.as_mut_ptr() as *mut _);
    } else {
        trace_kvm_mmio(KVM_TRACE_MMIO_READ_UNSATISFIED, len as u32, fault_ipa, core::ptr::null());
        ret = kvm_io_bus_read(vcpu, KVM_MMIO_BUS, fault_ipa, len, data_buf.as_mut_ptr() as *mut _);
    }
    (*run).mmio.is_write = is_write;
    (*run).mmio.phys_addr = fault_ipa;
    (*run).mmio.len = len;
    (*vcpu).mmio_needed = 1;
    if ret == 0 {
        if !is_write { core::ptr::copy_nonoverlapping(data_buf.as_ptr(), (*run).mmio.data.as_mut_ptr(), len as usize); }
        (*vcpu).stat.mmio_exit_kernel += 1;
        kvm_handle_mmio_return(vcpu);
        return 1;
    }
    if is_write { core::ptr::copy_nonoverlapping(data_buf.as_ptr(), (*run).mmio.data.as_mut_ptr(), len as usize); }
    (*vcpu).stat.mmio_exit_user += 1;
    (*run).exit_reason = KVM_EXIT_MMIO;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
