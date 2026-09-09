// SPDX-License-Identifier: GPL-2.0
/* hvapi.c: Hypervisor API management.
 *
 * Copyright (C) 2007 David S. Miller <davem@davemloft.net>
 */

/* Linux kernel and SPARC hypervisor dependencies are supplied externally. */

#[repr(C)]
struct ApiInfo {
    group: libc::c_ulong,
    major: libc::c_ulong,
    minor: libc::c_ulong,
    refcnt: libc::c_uint,
    flags: libc::c_uint,
}

const FLAG_PRE_API: libc::c_uint = 0x00000001;

static mut API_TABLE: [ApiInfo; 26] = [
    ApiInfo { group: HV_GRP_SUN4V, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_CORE, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_INTR, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_SOFT_STATE, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_TM, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_PCI, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_LDOM, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_SVC_CHAN, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_NCS, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_RNG, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_PBOOT, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_TPM, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_SDIO, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_SDIO_ERR, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_REBOOT_DATA, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_ATU, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_DAX, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_NIAG_PERF, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_FIRE_PERF, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_N2_CPU, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_NIU, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_VF_CPU, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_KT_CPU, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_VT_CPU, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_T5_CPU, flags: 0, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_DIAG, flags: FLAG_PRE_API, major: 0, minor: 0, refcnt: 0 },
    ApiInfo { group: HV_GRP_M7_PERF, flags: 0, major: 0, minor: 0, refcnt: 0 },
];

static mut HVAPI_LOCK: Spinlock = DEFINE_SPINLOCK!();

unsafe fn __get_info(group: libc::c_ulong) -> *mut ApiInfo {
    let mut i = 0;
    while i < API_TABLE.len() {
        if API_TABLE[i].group == group { return &mut API_TABLE[i]; }
        i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn __get_ref(p: *mut ApiInfo) { (*p).refcnt += 1; }

unsafe fn __put_ref(p: *mut ApiInfo) {
    (*p).refcnt -= 1;
    if (*p).refcnt == 0 {
        let mut ignore = 0;
        sun4v_set_version((*p).group, 0, 0, &mut ignore);
        (*p).major = 0;
        (*p).minor = 0;
    }
}

pub unsafe fn sun4v_hvapi_register(group: libc::c_ulong, major: libc::c_ulong, minor: *mut libc::c_ulong) -> libc::c_int {
    let mut flags = 0;
    spin_lock_irqsave(&mut HVAPI_LOCK, &mut flags);
    let p = __get_info(group);
    let mut ret = -EINVAL;
    if !p.is_null() {
        if (*p).refcnt != 0 {
            if (*p).major == major { *minor = (*p).minor; ret = 0; }
        } else {
            let mut actual_minor = 0;
            let hv_ret = sun4v_set_version(group, major, *minor, &mut actual_minor);
            if hv_ret == HV_EOK { *minor = actual_minor; (*p).major = major; (*p).minor = actual_minor; ret = 0; }
            else if (hv_ret == HV_EBADTRAP || hv_ret == HV_ENOTSUPPORTED) && ((*p).flags & FLAG_PRE_API) != 0 && major == 1 {
                (*p).major = 1; (*p).minor = 0; *minor = 0; ret = 0;
            }
        }
        if ret == 0 { __get_ref(p); }
    }
    spin_unlock_irqrestore(&mut HVAPI_LOCK, flags);
    ret
}

pub unsafe fn sun4v_hvapi_unregister(group: libc::c_ulong) {
    let mut flags = 0;
    spin_lock_irqsave(&mut HVAPI_LOCK, &mut flags);
    let p = __get_info(group);
    if !p.is_null() { __put_ref(p); }
    spin_unlock_irqrestore(&mut HVAPI_LOCK, flags);
}

pub unsafe fn sun4v_hvapi_get(group: libc::c_ulong, major: *mut libc::c_ulong, minor: *mut libc::c_ulong) -> libc::c_int {
    let mut flags = 0;
    spin_lock_irqsave(&mut HVAPI_LOCK, &mut flags);
    let p = __get_info(group);
    let mut ret = -EINVAL;
    if !p.is_null() && (*p).refcnt != 0 { *major = (*p).major; *minor = (*p).minor; ret = 0; }
    spin_unlock_irqrestore(&mut HVAPI_LOCK, flags);
    ret
}

pub unsafe fn sun4v_hvapi_init() {
    let mut group = HV_GRP_SUN4V;
    let mut major = 1;
    let mut minor = 0;
    if sun4v_hvapi_register(group, major, &mut minor) != 0 { goto_bad(group, major, minor); return; }
    group = HV_GRP_CORE; major = 1; minor = 6;
    if sun4v_hvapi_register(group, major, &mut minor) != 0 { goto_bad(group, major, minor); }
}

unsafe fn goto_bad(group: libc::c_ulong, major: libc::c_ulong, minor: libc::c_ulong) -> ! {
    prom_printf(b"HVAPI: Cannot register API group %lx with major(%lu) minor(%lu)\n\0".as_ptr(), group, major, minor);
    prom_halt();
    core::hint::unreachable_unchecked()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
