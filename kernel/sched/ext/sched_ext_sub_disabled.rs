// SPDX-License-Identifier: GPL-2.0
// Native BTF ABI adapters call the existing disabled-config Rust fallbacks;
// their original signatures and bodies in sub.rs remain unchanged.

/// Bridge the disabled grant fallback to the native BPF symbol wrapper.
///
/// # Safety
/// Native wrapper supplies ABI-correct arguments; no pointer is dereferenced.
#[export_name = "lupos_scx_sub_grant"]
pub unsafe extern "C" fn scx_sub_disabled_grant(id: u64, caps: u64,
    cmask: *const scx_cmask, denied: *mut scx_cmask, aux: *const bpf_prog_aux) -> c_int
{
    // SAFETY: The preserved fallback ignores every argument.
    unsafe { scx_bpf_sub_grant(id, caps, cmask, denied, aux) }
}

/// Bridge the disabled revoke fallback to the native BPF symbol wrapper.
///
/// # Safety
/// Native wrapper supplies ABI-correct arguments; no pointer is dereferenced.
#[export_name = "lupos_scx_sub_revoke"]
pub unsafe extern "C" fn scx_sub_disabled_revoke(id: u64, caps: u64,
    cmask: *const scx_cmask, aux: *const bpf_prog_aux)
{
    // SAFETY: The preserved fallback ignores every argument.
    unsafe { scx_bpf_sub_revoke(id, caps, cmask, aux) };
}

/// Bridge the disabled cap-read fallback to the native BPF symbol wrapper.
///
/// # Safety
/// Native wrapper supplies ABI-correct arguments; no pointer is dereferenced.
#[export_name = "lupos_scx_sub_caps"]
pub unsafe extern "C" fn scx_sub_disabled_caps(id: u64, caps: u64,
    out: *mut scx_cmask, aux: *const bpf_prog_aux) -> c_int
{
    // SAFETY: The preserved fallback ignores every argument.
    unsafe { scx_bpf_sub_caps(id, caps, out, aux) }
}

/// Bridge the disabled kill fallback to the native BPF symbol wrapper.
///
/// # Safety
/// Native wrapper supplies ABI-correct arguments; no pointer is dereferenced.
#[export_name = "lupos_scx_sub_kill_bstr"]
pub unsafe extern "C" fn scx_sub_disabled_kill(id: u64, fmt: *mut c_char,
    data: *mut kernel::ffi::c_ulonglong, size: u32, aux: *const bpf_prog_aux) -> c_int
{
    // SAFETY: Native c_ulonglong and u64 denote the same unsigned 64-bit ABI;
    // the preserved fallback ignores every argument without dereferencing data.
    unsafe { scx_bpf_sub_kill_bstr(id, fmt, data, size, aux) }
}
