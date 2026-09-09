/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM hw_pressure
// The C header includes the kernel tracepoint definitions and define_trace.h;
// those build-time facilities are supplied by the surrounding translation unit.

#[repr(C)]
pub struct HwPressureUpdateEntry {
    pub hw_pressure: ::core::ffi::c_ulong,
    pub cpu: ::core::ffi::c_int,
}

impl HwPressureUpdateEntry {
    #[inline]
    pub const unsafe fn new(
        cpu: ::core::ffi::c_int,
        hw_pressure: ::core::ffi::c_ulong,
    ) -> Self {
        Self {
            hw_pressure,
            cpu,
        }
    }
}

// TRACE_EVENT(hw_pressure_update,
//     TP_PROTO(int cpu, unsigned long hw_pressure),
//     TP_ARGS(cpu, hw_pressure),
//     TP_STRUCT__entry(
//         __field(unsigned long, hw_pressure)
//         __field(int, cpu)
//     ),
//     TP_fast_assign(
//         __entry->hw_pressure = hw_pressure;
//         __entry->cpu = cpu;
//     ),
//     TP_printk("cpu=%d hw_pressure=%lu", __entry->cpu, __entry->hw_pressure)
// );

extern "C" {
    pub fn trace_hw_pressure_update(
        cpu: ::core::ffi::c_int,
        hw_pressure: ::core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
