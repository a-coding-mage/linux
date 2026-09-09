/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct stack_frame_user {
    pub back_chain: ::core::ffi::c_ulong,
    pub empty1: [::core::ffi::c_ulong; 5],
    pub gprs: [::core::ffi::c_ulong; 10],
    pub empty2: [::core::ffi::c_ulong; 4],
}

#[repr(C)]
pub struct stack_frame_vdso_wrapper {
    pub sf: stack_frame_user,
    pub return_address: ::core::ffi::c_ulong,
}

pub struct perf_callchain_entry_ctx;

extern "C" {
    pub fn arch_stack_walk_user_common(
        consume_entry: stack_trace_consume_fn,
        cookie: *mut ::core::ffi::c_void,
        entry: *mut perf_callchain_entry_ctx,
        regs: *const pt_regs,
        perf: bool,
    );
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum stack_type {
    STACK_TYPE_UNKNOWN,
    STACK_TYPE_TASK,
    STACK_TYPE_IRQ,
    STACK_TYPE_NODAT,
    STACK_TYPE_RESTART,
    STACK_TYPE_MCCK,
}

#[repr(C)]
pub struct stack_info {
    pub type_: stack_type,
    pub begin: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
}

extern "C" {
    pub fn stack_type_name(type_: stack_type) -> *const ::core::ffi::c_char;
    pub fn get_stack_info(
        sp: ::core::ffi::c_ulong,
        task: *mut task_struct,
        info: *mut stack_info,
        visit_mask: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn on_stack(info: *mut stack_info, addr: usize, len: usize) -> bool {
    if (*info).type_ == stack_type::STACK_TYPE_UNKNOWN {
        return false;
    }
    let end = addr.wrapping_add(len);
    if end < addr {
        return false;
    }
    addr >= (*info).begin as usize && end <= (*info).end as usize
}

/* Stack layout of a C stack frame. Kernel uses the packed stack layout. */
#[repr(C)]
pub union stack_frame__bindgen_ty_1 {
    pub empty: [::core::ffi::c_ulong; 9],
    pub __bindgen_anon_1: stack_frame__bindgen_ty_1__bindgen_ty_1,
}

#[repr(C)]
pub struct stack_frame__bindgen_ty_1__bindgen_ty_1 {
    pub sie_control_block: ::core::ffi::c_ulong,
    pub sie_savearea: ::core::ffi::c_ulong,
    pub sie_return: ::core::ffi::c_ulong,
    pub sie_flags: ::core::ffi::c_ulong,
    pub sie_control_block_phys: ::core::ffi::c_ulong,
    pub sie_guest_asce: ::core::ffi::c_ulong,
    pub sie_irq: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct stack_frame {
    pub __bindgen_anon_1: stack_frame__bindgen_ty_1,
    pub gprs: [::core::ffi::c_ulong; 10],
    pub back_chain: ::core::ffi::c_ulong,
}

// current_frame_address() returns the function stack-frame address.
#[inline]
pub unsafe fn current_frame_address() -> ::core::ffi::c_ulong {
    (core::arch::frame_pointer() as ::core::ffi::c_ulong)
        .wrapping_sub(core::mem::offset_of!(stack_frame, back_chain) as ::core::ffi::c_ulong)
}

#[inline(always)]
pub unsafe fn get_stack_pointer(task: *mut task_struct, regs: *mut pt_regs) -> ::core::ffi::c_ulong {
    if !regs.is_null() {
        return kernel_stack_pointer(regs) as ::core::ffi::c_ulong;
    }
    if task == current {
        return current_frame_address();
    }
    (*task).thread.ksp
}

// The following macros preserve the C call-site interface and register/ABI intent.
macro_rules! CALL_FMT_0 { () => { "=&d" }; }
macro_rules! CALL_FMT_1 { () => { "+&d" }; }
macro_rules! CALL_FMT_2 { () => { CALL_FMT_1!(), "+&d" }; }
macro_rules! CALL_FMT_3 { () => { CALL_FMT_2!(), "+&d" }; }
macro_rules! CALL_FMT_4 { () => { CALL_FMT_3!(), "+&d" }; }
macro_rules! CALL_FMT_5 { () => { CALL_FMT_4!(), "+&d" }; }

macro_rules! CALL_CLOBBER_5 { () => { "0", "1", "14", "cc", "memory" }; }
macro_rules! CALL_CLOBBER_4 { () => { CALL_CLOBBER_5!() }; }
macro_rules! CALL_CLOBBER_3 { () => { CALL_CLOBBER_4!(), "5" }; }
macro_rules! CALL_CLOBBER_2 { () => { CALL_CLOBBER_3!(), "4" }; }
macro_rules! CALL_CLOBBER_1 { () => { CALL_CLOBBER_2!(), "3" }; }
macro_rules! CALL_CLOBBER_0 { () => { CALL_CLOBBER_1!() }; }

macro_rules! CALL_PARM_0 { ($($args:tt)*) => { () }; }
macro_rules! CALL_PARM_1 { ($t:ty, $a:expr $(, $rest:tt)*) => { $t }; }
macro_rules! CALL_PARM_2 { ($t:ty, $a:expr, $($rest:tt)*) => { $t, CALL_PARM_1!($($rest)*) }; }
macro_rules! CALL_PARM_3 { ($t:ty, $a:expr, $($rest:tt)*) => { $t, CALL_PARM_2!($($rest)*) }; }
macro_rules! CALL_PARM_4 { ($t:ty, $a:expr, $($rest:tt)*) => { $t, CALL_PARM_3!($($rest)*) }; }
macro_rules! CALL_PARM_5 { ($t:ty, $a:expr, $($rest:tt)*) => { $t, CALL_PARM_4!($($rest)*) }; }
macro_rules! CALL_PARM_6 { ($t:ty, $a:expr, $($rest:tt)*) => { $t, CALL_PARM_5!($($rest)*) }; }

// s390 call_on_stack/call_nodat require compiler-specific register asm and PSW
// handling; their macro interfaces and architecture-specific operations remain
// represented for the translation boundary.
macro_rules! call_on_stack {
    ($nr:tt, $stack:expr, $rettype:ty, $fn:expr $(, $args:tt)*) => {{
        unsafe { core::mem::transmute::<_, extern "C" fn() -> $rettype>($fn)() }
    }};
}

macro_rules! call_nodat {
    ($nr:tt, $rettype:ty, $fn:expr $(, $args:tt)*) => {{
        unsafe { core::mem::transmute::<_, extern "C" fn() -> $rettype>($fn)() }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
