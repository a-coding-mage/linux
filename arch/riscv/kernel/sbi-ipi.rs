// SPDX-License-Identifier: GPL-2.0-only
/*
 * Multiplex several IPIs over a single HW IPI.
 *
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// C headers supply the following kernel declarations and constants.

#[allow(non_upper_case_globals)]
pub static mut riscv_sbi_for_rfence: bool = false;

static mut sbi_ipi_virq: i32 = 0;

extern "C" {
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
    fn chained_irq_enter(chip: *mut irq_chip, desc: *mut irq_desc);
    fn csr_clear(csr: usize, value: usize);
    fn ipi_mux_process();
    fn chained_irq_exit(chip: *mut irq_chip, desc: *mut irq_desc);
    fn enable_percpu_irq(irq: i32, irq_type: u32);
    fn irq_get_trigger_type(irq: i32) -> u32;
    fn riscv_ipi_have_virq_range() -> bool;
    fn riscv_get_intc_hwnode() -> *mut fwnode_handle;
    fn irq_find_matching_fwnode(node: *mut fwnode_handle, bus_token: u32)
        -> *mut irq_domain;
    fn irq_create_mapping(domain: *mut irq_domain, hwirq: u32) -> i32;
    fn ipi_mux_create(nr_ipi: usize, send_ipi: unsafe extern "C" fn(u32, u32)) -> i32;
    fn sbi_send_ipi(mask: u32, hartid: u32);
    fn irq_dispose_mapping(virq: i32);
    fn irq_set_chained_handler(virq: i32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn cpuhp_setup_state(
        state: u32,
        name: *const core::ffi::c_char,
        startup: unsafe extern "C" fn(u32) -> i32,
        teardown: Option<unsafe extern "C" fn(u32) -> i32>,
    ) -> i32;
    fn riscv_ipi_set_virq_range(virq: i32, nr_ipi: usize);
    fn static_branch_enable(key: *mut bool);
}

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

const CSR_IP: usize = 0;
const IE_SIE: usize = 0;
const RV_IRQ_SOFT: u32 = 1;
const DOMAIN_BUS_ANY: u32 = 0;
const CPUHP_AP_IRQ_RISCV_SBI_IPI_STARTING: u32 = 0;
const BITS_PER_BYTE: usize = 8;

unsafe extern "C" fn sbi_ipi_handle(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);

    chained_irq_enter(chip, desc);

    csr_clear(CSR_IP, IE_SIE);
    ipi_mux_process();

    chained_irq_exit(chip, desc);
}

unsafe extern "C" fn sbi_ipi_starting_cpu(cpu: u32) -> i32 {
    enable_percpu_irq(sbi_ipi_virq, irq_get_trigger_type(sbi_ipi_virq));
    0
}

pub unsafe extern "C" fn sbi_ipi_init() {
    let virq: i32;
    let domain: *mut irq_domain;

    if riscv_ipi_have_virq_range() {
        return;
    }

    domain = irq_find_matching_fwnode(riscv_get_intc_hwnode(), DOMAIN_BUS_ANY);
    if domain.is_null() {
        // pr_err("unable to find INTC IRQ domain\\n");
        return;
    }

    sbi_ipi_virq = irq_create_mapping(domain, RV_IRQ_SOFT);
    if sbi_ipi_virq == 0 {
        // pr_err("unable to create INTC IRQ mapping\\n");
        return;
    }

    virq = ipi_mux_create(BITS_PER_BYTE, sbi_send_ipi);
    if virq <= 0 {
        // pr_err("unable to create muxed IPIs\\n");
        irq_dispose_mapping(sbi_ipi_virq);
        return;
    }

    irq_set_chained_handler(sbi_ipi_virq, sbi_ipi_handle);

    /*
     * Don't disable IPI when CPU goes offline because
     * the masking/unmasking of virtual IPIs is done
     * via generic IPI-Mux
     */
    cpuhp_setup_state(
        CPUHP_AP_IRQ_RISCV_SBI_IPI_STARTING,
        b"irqchip/sbi-ipi:starting\\0".as_ptr() as *const core::ffi::c_char,
        sbi_ipi_starting_cpu,
        None,
    );

    riscv_ipi_set_virq_range(virq, BITS_PER_BYTE);
    // pr_info("providing IPIs using SBI IPI extension\\n");

    /*
     * Use the SBI remote fence extension to avoid
     * the extra context switch needed to handle IPIs.
     */
    static_branch_enable(&raw mut riscv_sbi_for_rfence);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
