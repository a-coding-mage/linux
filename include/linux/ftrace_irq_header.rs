/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_HWLAT_TRACER controls the following declarations and calls.
#[cfg(feature = "CONFIG_HWLAT_TRACER")]
unsafe extern "C" {
    pub static mut trace_hwlat_callback_enabled: bool;
    pub fn trace_hwlat_callback(enter: bool);
}

// CONFIG_OSNOISE_TRACER controls the following declarations and calls.
#[cfg(feature = "CONFIG_OSNOISE_TRACER")]
unsafe extern "C" {
    pub static mut trace_osnoise_callback_enabled: bool;
    pub fn trace_osnoise_callback(enter: bool);
}

#[inline]
pub unsafe fn ftrace_nmi_enter() {
    #[cfg(feature = "CONFIG_HWLAT_TRACER")]
    {
        if trace_hwlat_callback_enabled {
            trace_hwlat_callback(true);
        }
    }

    #[cfg(feature = "CONFIG_OSNOISE_TRACER")]
    {
        if trace_osnoise_callback_enabled {
            trace_osnoise_callback(true);
        }
    }
}

#[inline]
pub unsafe fn ftrace_nmi_exit() {
    #[cfg(feature = "CONFIG_HWLAT_TRACER")]
    {
        if trace_hwlat_callback_enabled {
            trace_hwlat_callback(false);
        }
    }

    #[cfg(feature = "CONFIG_OSNOISE_TRACER")]
    {
        if trace_osnoise_callback_enabled {
            trace_osnoise_callback(false);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
