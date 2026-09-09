// SPDX-License-Identifier: GPL-2.0
// Translated from ics-rtas.c. Kernel includes and external symbols are
// supplied by the surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    static mut xics_default_server: i32;
    static mut icp_ops: *mut icp_ops;

    fn irqd_to_hwirq(d: *mut irq_data) -> u64;
    fn irq_data_get_affinity_mask(d: *mut irq_data) -> *const cpumask;
    fn xics_get_irq_server(irq: u32, mask: *const cpumask, strict_check: i32) -> i32;
    fn rtas_call(token: i32, nargs: i32, nret: i32, outputs: *mut i32, ... ) -> i32;
    fn printk(fmt: *const u8, ...);
    fn pr_devel(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
    fn of_device_is_compatible(node: *mut device_node, compatible: *const u8) -> bool;
    fn rtas_function_token(token: i32) -> i32;
    fn xics_set_irq_type(d: *mut irq_data, flow_type: u32) -> i32;
    fn xics_retrigger(d: *mut irq_data) -> i32;
    fn xics_register_ics(ics: *mut ics);
}

#[repr(C)]
pub struct irq_data { pub irq: u32 }
#[repr(C)]
pub struct cpumask;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct ics {
    pub check: Option<unsafe extern "C" fn(*mut ics, u32) -> i32>,
    pub mask_unknown: Option<unsafe extern "C" fn(*mut ics, u64)>,
    pub get_server: Option<unsafe extern "C" fn(*mut ics, u64) -> i64>,
    pub host_match: Option<unsafe extern "C" fn(*mut ics, *mut device_node) -> i32>,
    pub chip: *mut irq_chip,
}
#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data) -> u32>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_set_affinity: Option<unsafe extern "C" fn(*mut irq_data, *const cpumask, bool) -> i32>,
    pub irq_set_type: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
    pub irq_retrigger: Option<unsafe extern "C" fn(*mut irq_data) -> i32>,
}
#[repr(C)]
pub struct icp_ops { pub eoi: Option<unsafe extern "C" fn(*mut irq_data)> }

static mut ibm_get_xive: i32 = 0;
static mut ibm_set_xive: i32 = 0;
static mut ibm_int_on: i32 = 0;
static mut ibm_int_off: i32 = 0;

unsafe extern "C" fn ics_rtas_unmask_irq(d: *mut irq_data) {
    let hw_irq = irqd_to_hwirq(d) as u32;
    if hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS { return; }
    let server = xics_get_irq_server((*d).irq, irq_data_get_affinity_mask(d), 0);
    let call_status = rtas_call(ibm_set_xive, 3, 1, core::ptr::null_mut(), hw_irq, server, DEFAULT_PRIORITY);
    if call_status != 0 { return; }
    let call_status = rtas_call(ibm_int_on, 1, 1, core::ptr::null_mut(), hw_irq);
    if call_status != 0 { return; }
}

unsafe extern "C" fn ics_rtas_startup(d: *mut irq_data) -> u32 { ics_rtas_unmask_irq(d); 0 }

unsafe extern "C" fn ics_rtas_mask_real_irq(hw_irq: u32) {
    if hw_irq == XICS_IPI { return; }
    if rtas_call(ibm_int_off, 1, 1, core::ptr::null_mut(), hw_irq) != 0 { return; }
    if rtas_call(ibm_set_xive, 3, 1, core::ptr::null_mut(), hw_irq, xics_default_server, 0xff) != 0 { return; }
}

unsafe extern "C" fn ics_rtas_mask_irq(d: *mut irq_data) {
    let hw_irq = irqd_to_hwirq(d) as u32;
    if hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS { return; }
    ics_rtas_mask_real_irq(hw_irq);
}

unsafe extern "C" fn ics_rtas_set_affinity(d: *mut irq_data, cpumask: *const cpumask, _force: bool) -> i32 {
    let hw_irq = irqd_to_hwirq(d) as u32;
    if hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS { return -1; }
    let mut xics_status = [0i32; 2];
    if rtas_call(ibm_get_xive, 1, 3, xics_status.as_mut_ptr(), hw_irq) != 0 { return -1; }
    let irq_server = xics_get_irq_server((*d).irq, cpumask, 1);
    if irq_server == -1 { return -1; }
    if rtas_call(ibm_set_xive, 3, 1, core::ptr::null_mut(), hw_irq, irq_server, xics_status[1]) != 0 { return -1; }
    IRQ_SET_MASK_OK
}

static mut ics_rtas_irq_chip: irq_chip = irq_chip {
    name: b"XICS\0".as_ptr(), irq_startup: Some(ics_rtas_startup), irq_mask: Some(ics_rtas_mask_irq),
    irq_unmask: Some(ics_rtas_unmask_irq), irq_eoi: None, irq_set_affinity: Some(ics_rtas_set_affinity),
    irq_set_type: Some(xics_set_irq_type), irq_retrigger: Some(xics_retrigger),
};

unsafe extern "C" fn ics_rtas_check(_ics: *mut ics, hw_irq: u32) -> i32 {
    if hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS { return -22; }
    let mut status = [0i32; 2];
    if rtas_call(ibm_get_xive, 1, 3, status.as_mut_ptr(), hw_irq) != 0 { return -6; }
    0
}
unsafe extern "C" fn ics_rtas_mask_unknown(_ics: *mut ics, vec: u64) { ics_rtas_mask_real_irq(vec as u32); }
unsafe extern "C" fn ics_rtas_get_server(_ics: *mut ics, vec: u64) -> i64 {
    let mut status = [0i32; 2];
    if rtas_call(ibm_get_xive, 1, 3, status.as_mut_ptr(), vec) != 0 { return -1; }
    status[0] as i64
}
unsafe extern "C" fn ics_rtas_host_match(_ics: *mut ics, node: *mut device_node) -> i32 {
    (!of_device_is_compatible(node, b"chrp,iic\0".as_ptr())) as i32
}

static mut ics_rtas: ics = ics { check: Some(ics_rtas_check), mask_unknown: Some(ics_rtas_mask_unknown), get_server: Some(ics_rtas_get_server), host_match: Some(ics_rtas_host_match), chip: core::ptr::addr_of_mut!(ics_rtas_irq_chip) };

#[allow(non_snake_case)]
pub unsafe extern "C" fn ics_rtas_init() -> i32 {
    ibm_get_xive = rtas_function_token(RTAS_FN_IBM_GET_XIVE); ibm_set_xive = rtas_function_token(RTAS_FN_IBM_SET_XIVE);
    ibm_int_on = rtas_function_token(RTAS_FN_IBM_INT_ON); ibm_int_off = rtas_function_token(RTAS_FN_IBM_INT_OFF);
    if ibm_get_xive == RTAS_UNKNOWN_SERVICE || ibm_set_xive == RTAS_UNKNOWN_SERVICE { return -19; }
    ics_rtas_irq_chip.irq_eoi = (*icp_ops).eoi;
    xics_register_ics(core::ptr::addr_of_mut!(ics_rtas)); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
