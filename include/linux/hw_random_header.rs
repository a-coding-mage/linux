/*
    Hardware Random Number Generator

    Please read Documentation/admin-guide/hw_random.rst for details on use.

    ----------------------------------------------------------
    This software may be used and distributed according to the terms
    of the GNU General Public License, incorporated herein by reference.
*/

// Dependencies supplied by the corresponding Linux/Rust bindings:
// completion, kref, types, workqueue_types, list_head, kref, work_struct,
// completion, and device.

#[repr(C)]
pub struct hwrng {
    pub name: *const ::std::os::raw::c_char,
    pub init: Option<unsafe extern "C" fn(rng: *mut hwrng) -> ::std::os::raw::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(rng: *mut hwrng)>,
    pub data_present:
        Option<unsafe extern "C" fn(rng: *mut hwrng, wait: ::std::os::raw::c_int) -> ::std::os::raw::c_int>,
    pub data_read:
        Option<unsafe extern "C" fn(rng: *mut hwrng, data: *mut u32) -> ::std::os::raw::c_int>,
    pub read: Option<unsafe extern "C" fn(
        rng: *mut hwrng,
        data: *mut ::std::ffi::c_void,
        max: usize,
        wait: bool,
    ) -> ::std::os::raw::c_int>,
    pub priv_: ::std::os::raw::c_ulong,
    pub quality: ::std::os::raw::c_ushort,

    // private: internal.
    pub list: list_head,
    pub ref_: kref,
    pub cleanup_work: work_struct,
    pub cleanup_done: completion,
    pub dying: completion,
}

#[repr(C)]
pub struct device {
    _bindgen_opaque_blob: [u8; 0],
}

extern "C" {
    /** Register a new Hardware Random Number Generator driver. */
    pub fn hwrng_register(rng: *mut hwrng) -> ::std::os::raw::c_int;
    pub fn devm_hwrng_register(
        dev: *mut device,
        rng: *mut hwrng,
    ) -> ::std::os::raw::c_int;

    /** Unregister a Hardware Random Number Generator driver. */
    pub fn hwrng_unregister(rng: *mut hwrng);
    pub fn devm_hwrng_unregister(dev: *mut device, rng: *mut hwrng);

    pub fn hwrng_msleep(
        rng: *mut hwrng,
        msecs: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
    pub fn hwrng_yield(rng: *mut hwrng) -> ::std::os::raw::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
