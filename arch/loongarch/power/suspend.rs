// SPDX-License-Identifier: GPL-2.0
/*
 * loongson-specific suspend support
 *
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Linux and LoongArch dependencies supplied by other translation units.

pub static mut loongarch_suspend_addr: u64 = 0;

#[repr(C)]
pub struct saved_registers {
    pub ecfg: u32,
    pub euen: u32,
    pub pwctl0: u32,
    pub pwctl1: u32,
    pub pgd: usize,
    pub kpgd: usize,
    pub pcpu_base: usize,
}

static mut saved_regs: saved_registers = saved_registers {
    ecfg: 0,
    euen: 0,
    pwctl0: 0,
    pwctl1: 0,
    pgd: 0,
    kpgd: 0,
    pcpu_base: 0,
};

#[repr(C)]
pub struct loongson_sysconf_type {
    pub suspend_addr: u64,
}

unsafe extern "C" {
    pub static mut loongson_sysconf: loongson_sysconf_type;

    fn save_counter();
    fn sync_counter();
    fn csr_read(register: usize) -> usize;
    fn csr_read32(register: usize) -> u32;
    fn csr_write(value: usize, register: usize);
    fn csr_write32(value: u32, register: usize);
    fn local_flush_tlb_all();
    fn enable_gpe_wakeup();
    fn enable_pci_wakeup();
    fn loongarch_suspend_enter();
}

// These constants are supplied by the LoongArch headers.
unsafe extern "C" {
    static eentry: usize;
    static tlbrentry: usize;
}

pub unsafe fn loongarch_common_suspend() {
    save_counter();
    saved_regs.pgd = csr_read(LOONGARCH_CSR_PGDL);
    saved_regs.kpgd = csr_read(LOONGARCH_CSR_PGDH);
    saved_regs.pwctl0 = csr_read32(LOONGARCH_CSR_PWCTL0);
    saved_regs.pwctl1 = csr_read32(LOONGARCH_CSR_PWCTL1);
    saved_regs.ecfg = csr_read32(LOONGARCH_CSR_ECFG);
    saved_regs.euen = csr_read32(LOONGARCH_CSR_EUEN);
    saved_regs.pcpu_base = csr_read(PERCPU_BASE_KS);

    loongarch_suspend_addr = loongson_sysconf.suspend_addr;
}

pub unsafe fn loongarch_common_resume() {
    sync_counter();
    local_flush_tlb_all();
    csr_write(eentry, LOONGARCH_CSR_EENTRY);
    csr_write(eentry, LOONGARCH_CSR_MERRENTRY);
    csr_write(tlbrentry, LOONGARCH_CSR_TLBRENTRY);

    csr_write(saved_regs.pgd, LOONGARCH_CSR_PGDL);
    csr_write(saved_regs.kpgd, LOONGARCH_CSR_PGDH);
    csr_write32(saved_regs.pwctl0, LOONGARCH_CSR_PWCTL0);
    csr_write32(saved_regs.pwctl1, LOONGARCH_CSR_PWCTL1);
    csr_write32(saved_regs.ecfg, LOONGARCH_CSR_ECFG);
    csr_write32(saved_regs.euen, LOONGARCH_CSR_EUEN);
    csr_write(saved_regs.pcpu_base, PERCPU_BASE_KS);
}

pub unsafe fn loongarch_acpi_suspend() -> i32 {
    enable_gpe_wakeup();
    enable_pci_wakeup();

    loongarch_common_suspend();

    /* processor specific suspend */
    loongarch_suspend_enter();

    loongarch_common_resume();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
