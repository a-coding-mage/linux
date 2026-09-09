/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translation units: SGX constants and types,
// warning facilities, and architecture trap definitions.

#[inline]
pub const fn encls_trapnr(r: i32) -> i32 {
    r & !SGX_ENCLS_FAULT_FLAG
}

// Issue a WARN() about an ENCLS function.
#[macro_export]
macro_rules! encls_warn {
    ($r:expr, $name:expr) => {{
        let _r: i32 = $r;
        // Equivalent to WARN_ONCE(_r, "%s returned %d (0x%x)\n", name, _r, _r).
        if _r != 0 {
            warn_once!("{} returned {} (0x{:x})\n", $name, _r, _r);
        }
    }};
}

#[inline]
pub const fn encls_faulted(ret: i32) -> bool {
    (ret & SGX_ENCLS_FAULT_FLAG) != 0
}

#[inline]
pub fn encls_failed(ret: i32) -> bool {
    if encls_faulted(ret) {
        return encls_trapnr(ret) != X86_TRAP_PF;
    }

    ret != 0
}

// The following macros correspond to GCC extended-assembly ENCLS wrappers.
// The exception-table operand (_ASM_EXTABLE_TYPE) has no file-local Rust
// equivalent; the required architecture-specific implementation is external.
#[macro_export]
macro_rules! __encls_ret_n {
    ($rax:expr $(, $input:expr)*) => {{
        // TODO: bind to the architecture-specific ENCLS inline assembly and
        // EX_TYPE_FAULT_SGX exception table supplied by other dependencies.
        let _ = ($rax $(, $input)*);
        0i32
    }};
}

#[macro_export]
macro_rules! __encls_ret_1 { ($rax:expr, $rcx:expr) => { __encls_ret_n!($rax, $rcx) }; }
#[macro_export]
macro_rules! __encls_ret_2 { ($rax:expr, $rbx:expr, $rcx:expr) => { __encls_ret_n!($rax, $rbx, $rcx) }; }
#[macro_export]
macro_rules! __encls_ret_3 { ($rax:expr, $rbx:expr, $rcx:expr, $rdx:expr) => { __encls_ret_n!($rax, $rbx, $rcx, $rdx) }; }

#[macro_export]
macro_rules! __encls_n {
    ($rax:expr, $rbx_out:expr $(, $input:expr)*) => {{
        // TODO: bind to the architecture-specific ENCLS inline assembly and
        // EX_TYPE_FAULT_SGX exception table supplied by other dependencies.
        let _ = ($rax, $rbx_out $(, $input)*);
        0i32
    }};
}

#[macro_export]
macro_rules! __encls_2 { ($rax:expr, $rbx:expr, $rcx:expr) => { __encls_n!($rax, (), $rbx, $rcx) }; }

#[inline]
pub unsafe fn __encls_1_1<T: Copy>(rax: usize, data: &mut T, rcx: *mut core::ffi::c_void) -> i32 {
    let mut rbx_out: usize = 0;
    let ret = __encls_n!(rax, rbx_out, rcx);
    if ret == 0 {
        *data = core::mem::transmute_copy(&rbx_out);
    }
    ret
}

#[inline] pub unsafe fn __ecreate(pginfo: *mut sgx_pageinfo, secs: *mut core::ffi::c_void) -> i32 { __encls_2!(ECREATE, pginfo, secs) }
#[inline] pub unsafe fn __eextend(secs: *mut core::ffi::c_void, addr: *mut core::ffi::c_void) -> i32 { __encls_2!(EEXTEND, secs, addr) }
#[inline] pub unsafe fn __eadd(pginfo: *mut sgx_pageinfo, addr: *mut core::ffi::c_void) -> i32 { __encls_2!(EADD, pginfo, addr) }
#[inline] pub unsafe fn __einit(sigstruct: *mut core::ffi::c_void, token: *mut core::ffi::c_void, secs: *mut core::ffi::c_void) -> i32 { __encls_ret_3!(EINIT, sigstruct, secs, token) }
#[inline] pub unsafe fn __eremove(addr: *mut core::ffi::c_void) -> i32 { __encls_ret_1!(EREMOVE, addr) }
#[inline] pub unsafe fn __edbgwr(addr: *mut core::ffi::c_void, data: *mut usize) -> i32 { __encls_2!(EDGBWR, *data, addr) }
#[inline] pub unsafe fn __edbgrd(addr: *mut core::ffi::c_void, data: *mut usize) -> i32 { __encls_1_1(EDGBRD, &mut *data, addr) }
#[inline] pub unsafe fn __etrack(addr: *mut core::ffi::c_void) -> i32 { __encls_ret_1!(ETRACK, addr) }
#[inline] pub unsafe fn __eldu(pginfo: *mut sgx_pageinfo, addr: *mut core::ffi::c_void, va: *mut core::ffi::c_void) -> i32 { __encls_ret_3!(ELDU, pginfo, addr, va) }
#[inline] pub unsafe fn __eblock(addr: *mut core::ffi::c_void) -> i32 { __encls_ret_1!(EBLOCK, addr) }
#[inline] pub unsafe fn __epa(addr: *mut core::ffi::c_void) -> i32 { let rbx = SGX_PAGE_TYPE_VA; __encls_2!(EPA, rbx, addr) }
#[inline] pub unsafe fn __ewb(pginfo: *mut sgx_pageinfo, addr: *mut core::ffi::c_void, va: *mut core::ffi::c_void) -> i32 { __encls_ret_3!(EWB, pginfo, addr, va) }
#[inline] pub unsafe fn __emodpr(secinfo: *mut sgx_secinfo, addr: *mut core::ffi::c_void) -> i32 { __encls_ret_2!(EMODPR, secinfo, addr) }
#[inline] pub unsafe fn __emodt(secinfo: *mut sgx_secinfo, addr: *mut core::ffi::c_void) -> i32 { __encls_ret_2!(EMODT, secinfo, addr) }
#[inline] pub unsafe fn __eaug(pginfo: *mut sgx_pageinfo, addr: *mut core::ffi::c_void) -> i32 { __encls_2!(EAUG, pginfo, addr) }
#[inline] pub unsafe fn __eupdatesvn() -> i32 { __encls_ret_1!(EUPDATESVN, "") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
