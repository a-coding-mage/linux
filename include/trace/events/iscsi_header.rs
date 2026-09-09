// Translation of trace/events/iscsi.h.
//
// The C preprocessor tracepoint machinery is supplied by the surrounding
// kernel environment and has no direct file-local Rust equivalent.

/// Maximum debug message length.
pub const ISCSI_MSG_MAX: usize = 256;

/// Opaque external kernel type.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/// Opaque external kernel type.
#[repr(C)]
pub struct va_format {
    _private: [u8; 0],
}

/// Declare tracepoint helper function.
unsafe extern "C" {
    pub fn iscsi_dbg_trace(
        trace: Option<unsafe extern "C" fn(*mut device, *const va_format, ... )>,
        dev: *mut device,
        fmt: *const core::ffi::c_char,
        ...,
    );
}

// DECLARE_EVENT_CLASS(iscsi_log_msg,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf),
//     TP_STRUCT__entry(
//         __string(dname, dev_name(dev))
//         __vstring(msg, vaf->fmt, vaf->va)
//     ),
//     TP_fast_assign(
//         __assign_str(dname);
//         __assign_vstr(msg, vaf->fmt, vaf->va);
//     ),
//     TP_printk("%s: %s", __get_str(dname), __get_str(msg))
// );

// Define event to capture iscsi connection debug messages.
// DEFINE_EVENT(iscsi_log_msg, iscsi_dbg_conn,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf));

// Define event to capture iscsi session debug messages.
// DEFINE_EVENT(iscsi_log_msg, iscsi_dbg_session,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf));

// Define event to capture iscsi error handling debug messages.
// DEFINE_EVENT(iscsi_log_msg, iscsi_dbg_eh,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf));

// Define event to capture iscsi tcp debug messages.
// DEFINE_EVENT(iscsi_log_msg, iscsi_dbg_tcp,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf));

// Define event to capture iscsi sw tcp debug messages.
// DEFINE_EVENT(iscsi_log_msg, iscsi_dbg_sw_tcp,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf));

// Define event to capture iscsi transport session debug messages.
// DEFINE_EVENT(iscsi_log_msg, iscsi_dbg_trans_session,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf));

// Define event to capture iscsi transport connection debug messages.
// DEFINE_EVENT(iscsi_log_msg, iscsi_dbg_trans_conn,
//     TP_PROTO(struct device *dev, struct va_format *vaf),
//     TP_ARGS(dev, vaf));

// The C tracepoint definitions are emitted by <trace/define_trace.h> outside
// the header guard; that build-time expansion is intentionally preserved here
// as a dependency on the surrounding tracepoint implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
