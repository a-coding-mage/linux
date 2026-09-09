// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel/KVM translation.

unsafe fn kvm_irq_deliver(vcpu: *mut kvm_vcpu, mask: c_ulong) {
    let mut irq: c_ulong;
    let mut old: c_ulong;
    let mut new: c_ulong;

    if mask & CPU_AVEC != 0 {
        dmsintc_inject_irq(vcpu);
    }

    irq = mask & KVM_ESTAT_INTI_MASK;
    if irq != 0 {
        old = kvm_read_hw_gcsr(LOONGARCH_CSR_TVAL);
        set_gcsr_estat(irq);
        new = kvm_read_hw_gcsr(LOONGARCH_CSR_TVAL);

        /* Inject TI if TVAL inverted */
        if new > old {
            set_gcsr_estat(CPU_TIMER);
        }
    }

    irq = (mask >> VIP_DELTA) & KVM_GINTC_IRQ_MASK;
    if irq != 0 {
        set_csr_gintc(irq);
    }
}

unsafe fn kvm_irq_clear(vcpu: *mut kvm_vcpu, mask: c_ulong) {
    let mut irq: c_ulong;
    let mut old: c_ulong;
    let mut new: c_ulong;

    irq = mask & KVM_ESTAT_INTI_MASK;
    if irq != 0 {
        old = kvm_read_hw_gcsr(LOONGARCH_CSR_TVAL);
        clear_gcsr_estat(irq);
        new = kvm_read_hw_gcsr(LOONGARCH_CSR_TVAL);

        /* Inject TI if TVAL inverted */
        if new > old {
            set_gcsr_estat(CPU_TIMER);
        }
    }

    irq = (mask >> VIP_DELTA) & KVM_GINTC_IRQ_MASK;
    if irq != 0 {
        clear_csr_gintc(irq);
    }
}

pub unsafe fn kvm_deliver_intr(vcpu: *mut kvm_vcpu) {
    let mut mask: c_ulong;

    mask = READ_ONCE((*vcpu).arch.irq_clear);
    if mask != 0 {
        mask = xchg_relaxed(&mut (*vcpu).arch.irq_clear, 0);
        kvm_irq_clear(vcpu, mask);
    }

    mask = READ_ONCE((*vcpu).arch.irq_pending);
    if mask != 0 {
        mask = xchg_relaxed(&mut (*vcpu).arch.irq_pending, 0);
        kvm_irq_deliver(vcpu, mask);
    }
}

pub unsafe fn kvm_pending_timer(vcpu: *mut kvm_vcpu) -> c_int {
    test_bit(INT_TI, &(*vcpu).arch.irq_pending)
}

/*
 * Only support illegal instruction or illegal Address Error exception,
 * Other exceptions are injected by hardware in kvm mode
 */
unsafe fn _kvm_deliver_exception(
    vcpu: *mut kvm_vcpu,
    code: c_uint,
    subcode: c_uint,
) {
    let mut val: c_ulong;
    let mut vec_size: c_ulong;

    /*
     * BADV is added for EXCCODE_ADE exception
     *  Use PC register (GVA address) if it is instruction exeception
     *  Else use BADV from host side (GPA address) for data exeception
     */
    if code == EXCCODE_ADE {
        if subcode == EXSUBCODE_ADEF {
            val = (*vcpu).arch.pc;
        } else {
            val = (*vcpu).arch.badv;
        }
        kvm_write_hw_gcsr(LOONGARCH_CSR_BADV, val);
    }

    /* Set exception instruction */
    kvm_write_hw_gcsr(LOONGARCH_CSR_BADI, (*vcpu).arch.badi);

    /*
     * Save CRMD in PRMD
     * Set IRQ disabled and PLV0 with CRMD
     */
    val = kvm_read_hw_gcsr(LOONGARCH_CSR_CRMD);
    kvm_write_hw_gcsr(LOONGARCH_CSR_PRMD, val);
    val &= !(CSR_CRMD_PLV | CSR_CRMD_IE);
    kvm_write_hw_gcsr(LOONGARCH_CSR_CRMD, val);

    /* Set exception PC address */
    kvm_write_hw_gcsr(LOONGARCH_CSR_ERA, (*vcpu).arch.pc);

    /*
     * Set exception code
     * Exception and interrupt can be inject at the same time
     * Hardware will handle exception first and then extern interrupt
     * Exception code is Ecode in ESTAT[16:21]
     * Interrupt code in ESTAT[0:12]
     */
    val = kvm_read_hw_gcsr(LOONGARCH_CSR_ESTAT);
    val = (val & !CSR_ESTAT_EXC) | code as c_ulong;
    kvm_write_hw_gcsr(LOONGARCH_CSR_ESTAT, val);

    /* Calculate expcetion entry address */
    val = kvm_read_hw_gcsr(LOONGARCH_CSR_ECFG);
    vec_size = (val & CSR_ECFG_VS) >> CSR_ECFG_VS_SHIFT;
    if vec_size != 0 {
        vec_size = (1u64 << vec_size) * 4;
    }
    val = kvm_read_hw_gcsr(LOONGARCH_CSR_EENTRY);
    (*vcpu).arch.pc = val + code as c_ulong * vec_size;
}

pub unsafe fn kvm_deliver_exception(vcpu: *mut kvm_vcpu) {
    let pending: *mut c_ulong = &mut (*vcpu).arch.exception_pending;

    if *pending != 0 {
        let code = __ffs(*pending);
        _kvm_deliver_exception(vcpu, code, (*vcpu).arch.esubcode);
        *pending = 0;
        (*vcpu).arch.esubcode = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
