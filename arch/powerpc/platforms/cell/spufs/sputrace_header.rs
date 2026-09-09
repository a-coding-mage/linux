/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the Linux tracepoint header sputrace.h.
// The C tracepoint-definition machinery is supplied by the kernel tracing
// headers and is intentionally represented here as declarations/comments.

// TRACE_SYSTEM spufs

#[repr(C)]
pub struct spu_context {
    pub tid: ::core::ffi::c_int,
}

#[repr(C)]
pub struct spu {
    pub number: ::core::ffi::c_int,
}

/// External tracepoint emitted by TRACE_EVENT(spufs_context).
///
/// C signature:
/// `trace_spufs_context(struct spu_context *ctx, struct spu *spu,
///                      const char *name)`
unsafe extern "C" {
    pub fn trace_spufs_context(
        ctx: *mut spu_context,
        spu: *mut spu,
        name: *const ::core::ffi::c_char,
    );
}

// TRACE_EVENT(spufs_context,
//     TP_PROTO(struct spu_context *ctx, struct spu *spu, const char *name),
//     TP_ARGS(ctx, spu, name),
//     TP_STRUCT__entry(
//         __field(const char *, name)
//         __field(int, owner_tid)
//         __field(int, number)
//     ),
//     TP_fast_assign(
//         __entry->name = name;
//         __entry->owner_tid = ctx->tid;
//         __entry->number = spu ? spu->number : -1;
//     ),
//     TP_printk("%s (ctxthread = %d, spu = %d)",
//         __entry->name, __entry->owner_tid, __entry->number)
// );

// Equivalent to the C preprocessor's stringify operation for an identifier.
#[macro_export]
macro_rules! __stringify {
    ($name:ident) => {
        concat!(stringify!($name), "\0")
    };
}

/// `spu_context_trace(name, ctx, spu)`.
#[macro_export]
macro_rules! spu_context_trace {
    ($name:ident, $ctx:expr, $spu:expr) => {{
        let __name = __stringify!($name).as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            trace_spufs_context($ctx, $spu, __name);
        }
    }};
}

/// `spu_context_nospu_trace(name, ctx)`.
#[macro_export]
macro_rules! spu_context_nospu_trace {
    ($name:ident, $ctx:expr) => {{
        let __name = __stringify!($name).as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            trace_spufs_context($ctx, ::core::ptr::null_mut(), __name);
        }
    }};
}

// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE sputrace
// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
