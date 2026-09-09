/* SPDX-License-Identifier: GPL-2.0 */
/* Simple wrappers around HVM functions */

// C dependencies supplied by the surrounding Xen translation:
// xen/interface/hvm/params.h and asm/xen/hypercall.h

unsafe extern "C" {
    fn HYPERVISOR_hvm_op(op: ::core::ffi::c_int, arg: *mut ::core::ffi::c_void)
        -> ::core::ffi::c_int;
    fn pr_err(fmt: *const ::core::ffi::c_char, ...);
}

static PARAM_NAMES: [Option<&'static [u8]>; 15] = [
    Some(b"CALLBACK_IRQ\0"),
    Some(b"STORE_PFN\0"),
    Some(b"STORE_EVTCHN\0"),
    Some(b"PAE_ENABLED\0"),
    Some(b"IOREQ_PFN\0"),
    Some(b"BUFIOREQ_PFN\0"),
    Some(b"TIMER_MODE\0"),
    Some(b"HPET_ENABLED\0"),
    Some(b"IDENT_PT\0"),
    Some(b"DM_DOMAIN\0"),
    Some(b"ACPI_S_STATE\0"),
    Some(b"VM86_TSS\0"),
    Some(b"VPT_ALIGN\0"),
    Some(b"CONSOLE_PFN\0"),
    Some(b"CONSOLE_EVTCHN\0"),
];

unsafe fn param_name(op: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    if op < 0 || (op as usize) >= PARAM_NAMES.len() {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }

    match PARAM_NAMES[op as usize] {
        Some(name) => name.as_ptr() as *const ::core::ffi::c_char,
        None => b"reserved\0".as_ptr() as *const ::core::ffi::c_char,
    }
}

#[inline]
pub unsafe fn hvm_get_parameter(
    idx: ::core::ffi::c_int,
    value: *mut u64,
) -> ::core::ffi::c_int {
    // `xen_hvm_param` and the constants below are supplied by Xen headers.
    let mut xhv: xen_hvm_param = ::core::mem::zeroed();
    let mut r: ::core::ffi::c_int;

    xhv.domid = DOMID_SELF;
    xhv.index = idx;
    r = HYPERVISOR_hvm_op(HVMOP_get_param, &mut xhv as *mut _ as *mut ::core::ffi::c_void);
    if r < 0 {
        pr_err(
            b"Cannot get hvm parameter %s (%d): %d!\n\0".as_ptr() as *const _,
            param_name(idx),
            idx,
            r,
        );
        return r;
    }
    *value = xhv.value;
    r
}

pub const HVM_CALLBACK_VIA_TYPE_VECTOR: u64 = 0x2;
pub const HVM_CALLBACK_VIA_TYPE_SHIFT: u32 = 56;

#[inline]
pub const fn HVM_CALLBACK_VECTOR(x: u64) -> u64 {
    (HVM_CALLBACK_VIA_TYPE_VECTOR << HVM_CALLBACK_VIA_TYPE_SHIFT) | x
}

unsafe extern "C" {
    pub fn xen_setup_callback_vector();
    pub fn xen_set_upcall_vector(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
