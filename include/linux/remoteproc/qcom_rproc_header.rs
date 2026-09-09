// Forward declaration from the C header.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

/**
 * enum qcom_ssr_notify_type - Startup/Shutdown events related to a remoteproc
 * processor.
 *
 * @QCOM_SSR_BEFORE_POWERUP: Remoteproc about to start (prepare stage)
 * @QCOM_SSR_AFTER_POWERUP: Remoteproc is running (start stage)
 * @QCOM_SSR_BEFORE_SHUTDOWN: Remoteproc crashed or shutting down (stop stage)
 * @QCOM_SSR_AFTER_SHUTDOWN: Remoteproc is down (unprepare stage)
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum qcom_ssr_notify_type {
    QCOM_SSR_BEFORE_POWERUP,
    QCOM_SSR_AFTER_POWERUP,
    QCOM_SSR_BEFORE_SHUTDOWN,
    QCOM_SSR_AFTER_SHUTDOWN,
}

#[repr(C)]
pub struct qcom_ssr_notify_data {
    pub name: *const core::ffi::c_char,
    pub crashed: bool,
}

// CONFIG_QCOM_RPROC_COMMON build-time condition from the C header.
#[cfg(feature = "CONFIG_QCOM_RPROC_COMMON")]
unsafe extern "C" {
    pub fn qcom_register_ssr_notifier(
        name: *const core::ffi::c_char,
        nb: *mut notifier_block,
    ) -> *mut core::ffi::c_void;

    pub fn qcom_unregister_ssr_notifier(
        notify: *mut core::ffi::c_void,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_QCOM_RPROC_COMMON"))]
pub unsafe fn qcom_register_ssr_notifier(
    _name: *const core::ffi::c_char,
    _nb: *mut notifier_block,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_QCOM_RPROC_COMMON"))]
pub unsafe fn qcom_unregister_ssr_notifier(
    _notify: *mut core::ffi::c_void,
    _nb: *mut notifier_block,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
