// Translation of trace/events/siox.h.
//
// The Linux tracepoint macros used by the original header are represented here
// by the corresponding C-layout event payloads.  The tracepoint registration,
// formatting, and dynamic-array plumbing remain provided by the tracepoint
// subsystem.

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct siox_master {
    pub busno: core::ffi::c_int,
    pub buf: *mut u8,
}

#[repr(C)]
pub struct siox_device {
    pub inbytes: usize,
    pub outbytes: usize,
}

#[repr(C)]
pub struct siox_set_data_entry {
    pub busno: core::ffi::c_int,
    pub devno: u32,
    pub inbytes: usize,
    // __dynamic_array(u8, buf, sdevice->inbytes)
    pub buf: *mut u8,
}

#[repr(C)]
pub struct siox_get_data_entry {
    pub busno: core::ffi::c_int,
    pub devno: u32,
    pub status_clean: u8,
    pub outbytes: usize,
    // __dynamic_array(u8, buf, sdevice->outbytes)
    pub buf: *mut u8,
}

// TRACE_EVENT(siox_set_data,
//     TP_PROTO(const struct siox_master *smaster,
//              const struct siox_device *sdevice,
//              unsigned int devno, size_t bufoffset),
//     TP_ARGS(smaster, sdevice, devno, bufoffset),
//     TP_STRUCT__entry(
//         __field(int, busno)
//         __field(unsigned int, devno)
//         __field(size_t, inbytes)
//         __dynamic_array(u8, buf, sdevice->inbytes)),
//     TP_fast_assign(
//         __entry->busno = smaster->busno;
//         __entry->devno = devno;
//         __entry->inbytes = sdevice->inbytes;
//         memcpy(__get_dynamic_array(buf),
//                smaster->buf + bufoffset, sdevice->inbytes)),
//     TP_printk("siox-%d-%u [%*phD]", __entry->busno, __entry->devno,
//               (int)__entry->inbytes, __get_dynamic_array(buf)))

// TRACE_EVENT(siox_get_data,
//     TP_PROTO(const struct siox_master *smaster,
//              const struct siox_device *sdevice,
//              unsigned int devno, u8 status_clean, size_t bufoffset),
//     TP_ARGS(smaster, sdevice, devno, status_clean, bufoffset),
//     TP_STRUCT__entry(
//         __field(int, busno)
//         __field(unsigned int, devno)
//         __field(u8, status_clean)
//         __field(size_t, outbytes)
//         __dynamic_array(u8, buf, sdevice->outbytes)),
//     TP_fast_assign(
//         __entry->busno = smaster->busno;
//         __entry->devno = devno;
//         __entry->status_clean = status_clean;
//         __entry->outbytes = sdevice->outbytes;
//         memcpy(__get_dynamic_array(buf),
//                smaster->buf + bufoffset, sdevice->outbytes)),
//     TP_printk("siox-%d-%u (%02hhx) [%*phD]", __entry->busno,
//               __entry->devno, __entry->status_clean,
//               (int)__entry->outbytes, __get_dynamic_array(buf)))

// `c_void` is retained as the tracepoint subsystem's opaque dependency type.
#[allow(dead_code)]
type TracepointOpaque = c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
