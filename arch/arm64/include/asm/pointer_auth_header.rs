/* SPDX-License-Identifier: GPL-2.0 */

// The EL0/EL1 pointer bits used by a pointer authentication code.
// This is dependent on TBI0/TBI1 being enabled, or bits 63:56 would also apply.
#[inline]
pub fn ptrauth_user_pac_mask() -> u64 {
    GENMASK_ULL(54, vabits_actual)
}

#[inline]
pub fn ptrauth_kernel_pac_mask() -> u64 {
    GENMASK_ULL(63, vabits_actual)
}

pub const PR_PAC_ENABLED_KEYS_MASK: u64 =
    PR_PAC_APIAKEY | PR_PAC_APIBKEY | PR_PAC_APDAKEY | PR_PAC_APDBKEY;

#[cfg(CONFIG_ARM64_PTR_AUTH)]
#[repr(C)]
pub struct ptrauth_key {
    pub lo: c_ulong,
    pub hi: c_ulong,
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
#[repr(C)]
pub struct ptrauth_keys_user {
    pub apia: ptrauth_key,
    pub apib: ptrauth_key,
    pub apda: ptrauth_key,
    pub apdb: ptrauth_key,
    pub apga: ptrauth_key,
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
macro_rules! __ptrauth_key_install_nosync {
    ($k:ident, $v:expr) => {{
        let __pki_v: ptrauth_key = $v;
        write_sysreg_s(__pki_v.lo, concat!(stringify!($k), "KEYLO_EL1"));
        write_sysreg_s(__pki_v.hi, concat!(stringify!($k), "KEYHI_EL1"));
    }};
}

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
#[repr(C)]
pub struct ptrauth_keys_kernel {
    pub apia: ptrauth_key,
}

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
#[inline(always)]
pub unsafe fn ptrauth_keys_init_kernel(keys: *mut ptrauth_keys_kernel) {
    if system_supports_address_auth() {
        get_random_bytes(
            core::ptr::addr_of_mut!((*keys).apia) as *mut c_void,
            core::mem::size_of::<ptrauth_key>(),
        );
    }
}

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
#[inline(always)]
pub unsafe fn ptrauth_keys_switch_kernel(keys: *mut ptrauth_keys_kernel) {
    if !system_supports_address_auth() {
        return;
    }
    __ptrauth_key_install_nosync!(APIA, (*keys).apia);
    isb();
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
pub unsafe fn ptrauth_keys_install_user(keys: *mut ptrauth_keys_user) {
    if system_supports_address_auth() {
        __ptrauth_key_install_nosync!(APIB, (*keys).apib);
        __ptrauth_key_install_nosync!(APDA, (*keys).apda);
        __ptrauth_key_install_nosync!(APDB, (*keys).apdb);
    }
    if system_supports_generic_auth() {
        __ptrauth_key_install_nosync!(APGA, (*keys).apga);
    }
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
pub unsafe fn ptrauth_keys_init_user(keys: *mut ptrauth_keys_user) {
    if system_supports_address_auth() {
        get_random_bytes(core::ptr::addr_of_mut!((*keys).apia) as *mut c_void, core::mem::size_of::<ptrauth_key>());
        get_random_bytes(core::ptr::addr_of_mut!((*keys).apib) as *mut c_void, core::mem::size_of::<ptrauth_key>());
        get_random_bytes(core::ptr::addr_of_mut!((*keys).apda) as *mut c_void, core::mem::size_of::<ptrauth_key>());
        get_random_bytes(core::ptr::addr_of_mut!((*keys).apdb) as *mut c_void, core::mem::size_of::<ptrauth_key>());
    }
    if system_supports_generic_auth() {
        get_random_bytes(core::ptr::addr_of_mut!((*keys).apga) as *mut c_void, core::mem::size_of::<ptrauth_key>());
    }
    ptrauth_keys_install_user(keys);
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
extern "C" {
    pub fn ptrauth_prctl_reset_keys(tsk: *mut task_struct, arg: c_ulong) -> c_int;
    pub fn ptrauth_set_enabled_keys(tsk: *mut task_struct, keys: c_ulong, enabled: c_ulong) -> c_int;
    pub fn ptrauth_get_enabled_keys(tsk: *mut task_struct) -> c_int;
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
#[inline(always)]
pub unsafe fn ptrauth_enable() {
    if !system_supports_address_auth() { return; }
    sysreg_clear_set(sctlr_el1, 0, SCTLR_ELx_ENIA | SCTLR_ELx_ENIB | SCTLR_ELx_ENDA | SCTLR_ELx_ENDB);
    isb();
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
#[macro_export]
macro_rules! ptrauth_suspend_exit {
    () => { ptrauth_keys_install_user(core::ptr::addr_of_mut!(current.thread.keys_user)); };
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
#[macro_export]
macro_rules! ptrauth_thread_init_user {
    () => {{
        ptrauth_keys_init_user(core::ptr::addr_of_mut!(current.thread.keys_user));
        if system_supports_address_auth() {
            ptrauth_set_enabled_keys(current, PR_PAC_ENABLED_KEYS_MASK, PR_PAC_ENABLED_KEYS_MASK);
        }
    }};
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
#[macro_export]
macro_rules! ptrauth_thread_switch_user {
    ($tsk:expr) => { ptrauth_keys_install_user(core::ptr::addr_of_mut!((*$tsk).thread.keys_user)); };
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
pub unsafe fn ptrauth_prctl_reset_keys(_: *mut task_struct, _: c_ulong) -> c_int { -EINVAL }
#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
pub unsafe fn ptrauth_set_enabled_keys(_: *mut task_struct, _: c_ulong, _: c_ulong) -> c_int { -EINVAL }
#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
pub unsafe fn ptrauth_get_enabled_keys(_: *mut task_struct) -> c_int { -EINVAL }

#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
#[macro_export]
macro_rules! ptrauth_enable { () => {}; }
#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
#[macro_export]
macro_rules! ptrauth_suspend_exit { () => {}; }
#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
#[macro_export]
macro_rules! ptrauth_thread_init_user { () => {}; }
#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
#[macro_export]
macro_rules! ptrauth_thread_switch_user { ($tsk:expr) => {}; }

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
#[macro_export]
macro_rules! ptrauth_thread_init_kernel {
    ($tsk:expr) => { ptrauth_keys_init_kernel(core::ptr::addr_of_mut!((*$tsk).thread.keys_kernel)); };
}
#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
#[macro_export]
macro_rules! ptrauth_thread_switch_kernel {
    ($tsk:expr) => { ptrauth_keys_switch_kernel(core::ptr::addr_of_mut!((*$tsk).thread.keys_kernel)); };
}
#[cfg(not(CONFIG_ARM64_PTR_AUTH_KERNEL))]
#[macro_export]
macro_rules! ptrauth_thread_init_kernel { ($tsk:expr) => {}; }
#[cfg(not(CONFIG_ARM64_PTR_AUTH_KERNEL))]
#[macro_export]
macro_rules! ptrauth_thread_switch_kernel { ($tsk:expr) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
