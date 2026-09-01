/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from C header firewire/cmp.h.
 *
 * Dependency intent from removed C includes:
 * - linux/mutex.h provides struct mutex.
 * - linux/types.h provides bool and __be32.
 * - iso-resources.h provides struct fw_iso_resources.
 */

/* Forward declaration from the C header: struct fw_unit; */
pub enum fw_unit {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cmp_direction {
    CMP_INPUT = 0,
    CMP_OUTPUT,
}

/**
 * struct cmp_connection - manages an isochronous connection to a device
 * @speed: the connection's actual speed
 *
 * This structure manages (using CMP) an isochronous stream between the local
 * computer and a device's input plug (iPCR) and output plug (oPCR).
 *
 * There is no corresponding oPCR created on the local computer, so it is not
 * possible to overlay connections on top of this one.
 */
#[repr(C)]
pub struct cmp_connection {
    pub speed: ::core::ffi::c_int,
    /* private: */
    pub connected: bool,
    pub mutex: mutex,
    pub resources: fw_iso_resources,
    pub last_pcr_value: __be32,
    pub pcr_index: ::core::ffi::c_uint,
    pub max_speed: ::core::ffi::c_uint,
    pub direction: cmp_direction,
}

unsafe extern "C" {
    pub fn cmp_connection_init(
        connection: *mut cmp_connection,
        unit: *mut fw_unit,
        direction: cmp_direction,
        pcr_index: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn cmp_connection_check_used(
        connection: *mut cmp_connection,
        used: *mut bool,
    ) -> ::core::ffi::c_int;

    pub fn cmp_connection_destroy(connection: *mut cmp_connection);

    pub fn cmp_connection_reserve(
        connection: *mut cmp_connection,
        max_payload: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn cmp_connection_release(connection: *mut cmp_connection);

    pub fn cmp_connection_establish(connection: *mut cmp_connection) -> ::core::ffi::c_int;

    pub fn cmp_connection_break(connection: *mut cmp_connection);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
