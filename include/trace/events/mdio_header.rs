/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM mdio
//
// The include files and tracepoint machinery used by the C header are
// supplied by the surrounding kernel translation.

#[repr(C)]
pub struct MdioAccessEntry {
    pub busid: [core::ffi::c_char; MII_BUS_ID_SIZE],
    pub read: core::ffi::c_char,
    pub addr: u8,
    pub val: u16,
    pub regnum: core::ffi::c_uint,
}

// TRACE_EVENT_CONDITION(mdio_access, ...)
// The event is emitted only when err >= 0.
// TP_fast_assign copies bus->id into busid and stores read, addr, regnum, and
// val in the entry.  TP_printk formats the entry as:
// "%s %-5s phy:0x%02hhx reg:0x%02x val:0x%04hx"
extern "C" {
    pub fn trace_mdio_access(
        bus: *mut MiiBus,
        read: core::ffi::c_char,
        addr: u8,
        regnum: core::ffi::c_uint,
        val: u16,
        err: core::ffi::c_int,
    );
}

// External declarations supplied by the kernel translation.
#[repr(C)]
pub struct MiiBus {
    _private: [u8; 0],
}

// MII_BUS_ID_SIZE is defined by the external MDIO dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
