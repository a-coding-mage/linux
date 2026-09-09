// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 */

// Linux and architecture declarations are supplied by the surrounding crate.

extern "C" {
    fn kvm_riscv_vcpu_sbi_forward_handler(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        retdata: *mut kvm_vcpu_sbi_return,
    ) -> i32;
    fn kvm_riscv_vcpu_timer_next_event(vcpu: *mut kvm_vcpu, next_cycle: u64) -> i32;
    fn kvm_riscv_vcpu_unset_interrupt(vcpu: *mut kvm_vcpu, irq: i32) -> i32;
    fn kvm_riscv_vcpu_unpriv_read(
        vcpu: *mut kvm_vcpu,
        write: bool,
        addr: c_ulong,
        utrap: *mut kvm_cpu_trap,
    ) -> c_ulong;
    fn kvm_get_vcpu_by_id(kvm: *mut kvm, id: c_ulong) -> *mut kvm_vcpu;
    fn kvm_riscv_vcpu_set_interrupt(vcpu: *mut kvm_vcpu, irq: i32) -> i32;
    fn kvm_riscv_vcpu_sbi_system_reset(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        event: i32,
        flags: i32,
    );
    fn kvm_riscv_fence_i(kvm: *mut kvm, hbase: c_ulong, hmask: c_ulong);
    fn kvm_riscv_hfence_vvma_all(kvm: *mut kvm, hbase: c_ulong, hmask: c_ulong, vmid: c_ulong);
    fn kvm_riscv_hfence_vvma_gva(
        kvm: *mut kvm, hbase: c_ulong, hmask: c_ulong, start: c_ulong,
        size: c_ulong, page_shift: i32, vmid: c_ulong,
    );
    fn kvm_riscv_hfence_vvma_asid_all(
        kvm: *mut kvm, hbase: c_ulong, hmask: c_ulong, asid: c_ulong, vmid: c_ulong,
    );
    fn kvm_riscv_hfence_vvma_asid_gva(
        kvm: *mut kvm, hbase: c_ulong, hmask: c_ulong, start: c_ulong,
        size: c_ulong, page_shift: i32, asid: c_ulong, vmid: c_ulong,
    );
}

unsafe fn kvm_sbi_ext_v01_handler(
    vcpu: *mut kvm_vcpu,
    run: *mut kvm_run,
    retdata: *mut kvm_vcpu_sbi_return,
) -> i32 {
    let mut hbase: c_ulong = 0;
    let mut hmask: c_ulong;
    let mut i: c_ulong;
    let mut ret: i32 = 0;
    let mut next_cycle: u64;
    let mut rvcpu: *mut kvm_vcpu;
    let kvm = (*vcpu).kvm;
    let cp = &mut (*vcpu).arch.guest_context as *mut kvm_cpu_context;
    let utrap = (*retdata).utrap;
    let mut vmid: c_ulong;

    match (*cp).a7 {
        SBI_EXT_0_1_CONSOLE_GETCHAR | SBI_EXT_0_1_CONSOLE_PUTCHAR => {
            ret = kvm_riscv_vcpu_sbi_forward_handler(vcpu, run, retdata);
        }
        SBI_EXT_0_1_SET_TIMER => {
            next_cycle = (*cp).a0 as u64;
            ret = kvm_riscv_vcpu_timer_next_event(vcpu, next_cycle);
        }
        SBI_EXT_0_1_CLEAR_IPI => {
            ret = kvm_riscv_vcpu_unset_interrupt(vcpu, IRQ_VS_SOFT);
        }
        SBI_EXT_0_1_SEND_IPI => {
            if (*cp).a0 != 0 {
                hmask = kvm_riscv_vcpu_unpriv_read(vcpu, false, (*cp).a0, utrap);
                if (*utrap).scause != 0 { return ret; }
                i = 0;
                while i < BITS_PER_LONG {
                    if (hmask & (1 as c_ulong).wrapping_shl(i as u32)) != 0 {
                        rvcpu = kvm_get_vcpu_by_id((*vcpu).kvm, i);
                        if !rvcpu.is_null() {
                            ret = kvm_riscv_vcpu_set_interrupt(rvcpu, IRQ_VS_SOFT);
                            if ret < 0 { break; }
                        }
                    }
                    i += 1;
                }
            } else {
                // Equivalent of kvm_for_each_vcpu(i, rvcpu, kvm), supplied by the KVM layer.
                i = 0;
                while i < (*kvm).created_vcpus {
                    rvcpu = kvm_get_vcpu_by_id(kvm, i);
                    if !rvcpu.is_null() {
                        ret = kvm_riscv_vcpu_set_interrupt(rvcpu, IRQ_VS_SOFT);
                        if ret < 0 { break; }
                    }
                    i += 1;
                }
            }
        }
        SBI_EXT_0_1_SHUTDOWN => {
            kvm_riscv_vcpu_sbi_system_reset(vcpu, run, KVM_SYSTEM_EVENT_SHUTDOWN, 0);
            (*retdata).uexit = true;
        }
        SBI_EXT_0_1_REMOTE_FENCE_I | SBI_EXT_0_1_REMOTE_SFENCE_VMA |
        SBI_EXT_0_1_REMOTE_SFENCE_VMA_ASID => {
            if (*cp).a0 != 0 { hmask = kvm_riscv_vcpu_unpriv_read(vcpu, false, (*cp).a0, utrap); }
            else { hbase = !0; hmask = 0; }
            if (*utrap).scause != 0 { return ret; }
            if (*cp).a7 == SBI_EXT_0_1_REMOTE_FENCE_I {
                kvm_riscv_fence_i((*vcpu).kvm, hbase, hmask);
            } else if (*cp).a7 == SBI_EXT_0_1_REMOTE_SFENCE_VMA {
                vmid = (*vcpu).kvm.arch.vmid.vmid;
                if (*cp).a1 == 0 && (*cp).a2 == 0 { kvm_riscv_hfence_vvma_all((*vcpu).kvm, hbase, hmask, vmid); }
                else { kvm_riscv_hfence_vvma_gva((*vcpu).kvm, hbase, hmask, (*cp).a1, (*cp).a2, PAGE_SHIFT, vmid); }
            } else {
                vmid = (*vcpu).kvm.arch.vmid.vmid;
                if (*cp).a1 == 0 && (*cp).a2 == 0 { kvm_riscv_hfence_vvma_asid_all((*vcpu).kvm, hbase, hmask, (*cp).a3, vmid); }
                else { kvm_riscv_hfence_vvma_asid_gva((*vcpu).kvm, hbase, hmask, (*cp).a1, (*cp).a2, PAGE_SHIFT, (*cp).a3, vmid); }
            }
        }
        _ => { (*retdata).err_val = SBI_ERR_NOT_SUPPORTED; }
    }
    ret
}

#[no_mangle]
pub static vcpu_sbi_ext_v01: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_0_1_SET_TIMER,
    extid_end: SBI_EXT_0_1_SHUTDOWN,
    handler: kvm_sbi_ext_v01_handler,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
