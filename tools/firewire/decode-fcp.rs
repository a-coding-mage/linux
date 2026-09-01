// SPDX-License-Identifier: GPL-2.0
// Translated from firewire/decode-fcp.c.
// C includes referenced linux/firewire-constants.h, list.h, and nosy-dump.h.

use std::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const CSR_FCP_COMMAND: c_ulonglong = 0xfffff0000b00u64;
const CSR_FCP_RESPONSE: c_ulonglong = 0xfffff0000d00u64;

unsafe extern "C" {
    static TCODE_WRITE_BLOCK_REQUEST: c_uint;

    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct link_transaction {
    pub request: *mut request,
}

#[repr(C)]
pub struct request {
    pub packet: packet,
}

#[repr(C)]
pub union packet {
    pub common: packet_common,
    pub write_block: packet_write_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_common {
    pub tcode: c_uint,
    pub offset_high: c_uint,
    pub offset_low: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_write_block {
    pub common: packet_common,
    pub data: *mut c_void,
}

static CTYPE_NAMES: [&[u8]; 16] = [
    b"control\0",
    b"status\0",
    b"specific inquiry\0",
    b"notify\0",
    b"general inquiry\0",
    b"(reserved 0x05)\0",
    b"(reserved 0x06)\0",
    b"(reserved 0x07)\0",
    b"not implemented\0",
    b"accepted\0",
    b"rejected\0",
    b"in transition\0",
    b"stable\0",
    b"changed\0",
    b"(reserved 0x0e)\0",
    b"interim\0",
];

static SUBUNIT_TYPE_NAMES: [&[u8]; 32] = [
    b"monitor\0",
    b"audio\0",
    b"printer\0",
    b"disc\0",
    b"tape recorder/player\0",
    b"tuner\0",
    b"ca\0",
    b"camera\0",
    b"(reserved 0x08)\0",
    b"panel\0",
    b"bulletin board\0",
    b"camera storage\0",
    b"(reserved 0x0c)\0",
    b"(reserved 0x0d)\0",
    b"(reserved 0x0e)\0",
    b"(reserved 0x0f)\0",
    b"(reserved 0x10)\0",
    b"(reserved 0x11)\0",
    b"(reserved 0x12)\0",
    b"(reserved 0x13)\0",
    b"(reserved 0x14)\0",
    b"(reserved 0x15)\0",
    b"(reserved 0x16)\0",
    b"(reserved 0x17)\0",
    b"(reserved 0x18)\0",
    b"(reserved 0x19)\0",
    b"(reserved 0x1a)\0",
    b"(reserved 0x1b)\0",
    b"vendor unique\0",
    b"all subunit types\0",
    b"subunit_type extended to next byte\0",
    b"unit\0",
];

#[repr(C)]
pub struct avc_enum {
    pub value: c_int,
    pub name: *const c_char,
}

unsafe impl Sync for avc_enum {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avc_field {
    pub name: *const c_char, /* Short name for field. */
    pub offset: c_int,       /* Location of field, specified in bits; */
                             /* negative means from end of packet.    */
    pub width: c_int,        /* Width of field, 0 means use data_length. */
    pub names: *mut avc_enum,
}

unsafe impl Sync for avc_field {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avc_opcode_info {
    pub name: *const c_char,
    pub fields: [avc_field; 8],
}

unsafe impl Sync for avc_opcode_info {}

const NULL_FIELD: avc_field = avc_field {
    name: std::ptr::null(),
    offset: 0,
    width: 0,
    names: std::ptr::null_mut(),
};

const NULL_OPCODE_INFO: avc_opcode_info = avc_opcode_info {
    name: std::ptr::null(),
    fields: [NULL_FIELD; 8],
};

#[unsafe(no_mangle)]
pub static mut power_field_names: [avc_enum; 3] = [
    avc_enum {
        value: 0x70,
        name: c"on".as_ptr(),
    },
    avc_enum {
        value: 0x60,
        name: c"off".as_ptr(),
    },
    avc_enum {
        value: 0,
        name: std::ptr::null(),
    },
];

static OPCODE_INFO: [avc_opcode_info; 256] = {
    let mut a = [NULL_OPCODE_INFO; 256];

    /*
     * TA Document 1999026
     * AV/C Digital Interface Command Set General Specification 4.0
     */
    a[0xb2] = avc_opcode_info {
        name: c"power".as_ptr(),
        fields: [
            avc_field {
                name: c"state".as_ptr(),
                offset: 0,
                width: 8,
                names: &raw mut power_field_names as *mut avc_enum,
            },
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
        ],
    };
    a[0x30] = avc_opcode_info {
        name: c"unit info".as_ptr(),
        fields: [
            avc_field {
                name: c"foo".as_ptr(),
                offset: 0,
                width: 8,
                names: std::ptr::null_mut(),
            },
            avc_field {
                name: c"unit_type".as_ptr(),
                offset: 8,
                width: 5,
                names: std::ptr::null_mut(),
            },
            avc_field {
                name: c"unit".as_ptr(),
                offset: 13,
                width: 3,
                names: std::ptr::null_mut(),
            },
            avc_field {
                name: c"company id".as_ptr(),
                offset: 16,
                width: 24,
                names: std::ptr::null_mut(),
            },
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
        ],
    };
    a[0x31] = avc_opcode_info { name: c"subunit info".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x01] = avc_opcode_info { name: c"reserve".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0xb0] = avc_opcode_info { name: c"version".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x00] = avc_opcode_info { name: c"vendor dependent".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x02] = avc_opcode_info { name: c"plug info".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x12] = avc_opcode_info { name: c"channel usage".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x24] = avc_opcode_info { name: c"connect".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x20] = avc_opcode_info { name: c"connect av".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x22] = avc_opcode_info { name: c"connections".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x11] = avc_opcode_info { name: c"digital input".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x10] = avc_opcode_info { name: c"digital output".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x25] = avc_opcode_info { name: c"disconnect".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x21] = avc_opcode_info { name: c"disconnect av".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x19] = avc_opcode_info { name: c"input plug signal format".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x18] = avc_opcode_info { name: c"output plug signal format".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x1f] = avc_opcode_info { name: c"general bus setup".as_ptr(), fields: [NULL_FIELD; 8] };

    /*
     * TA Document 1999025
     * AV/C Descriptor Mechanism Specification Version 1.0
     */
    a[0x0c] = avc_opcode_info { name: c"create descriptor".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x08] = avc_opcode_info { name: c"open descriptor".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x09] = avc_opcode_info { name: c"read descriptor".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x0a] = avc_opcode_info { name: c"write descriptor".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x05] = avc_opcode_info { name: c"open info block".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x06] = avc_opcode_info { name: c"read info block".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x07] = avc_opcode_info { name: c"write info block".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x0b] = avc_opcode_info { name: c"search descriptor".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x0d] = avc_opcode_info { name: c"object number select".as_ptr(), fields: [NULL_FIELD; 8] };

    /*
     * TA Document 1999015
     * AV/C Command Set for Rate Control of Isochronous Data Flow 1.0
     */
    a[0xb3] = avc_opcode_info {
        name: c"rate".as_ptr(),
        fields: [
            avc_field {
                name: c"subfunction".as_ptr(),
                offset: 0,
                width: 8,
                names: std::ptr::null_mut(),
            },
            avc_field {
                name: c"result".as_ptr(),
                offset: 8,
                width: 8,
                names: std::ptr::null_mut(),
            },
            avc_field {
                name: c"plug_type".as_ptr(),
                offset: 16,
                width: 8,
                names: std::ptr::null_mut(),
            },
            avc_field {
                name: c"plug_id".as_ptr(),
                offset: 16,
                width: 8,
                names: std::ptr::null_mut(),
            },
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
            NULL_FIELD,
        ],
    };

    /*
     * TA Document 1999008
     * AV/C Audio Subunit Specification 1.0
     */
    a[0xb8] = avc_opcode_info { name: c"function block".as_ptr(), fields: [NULL_FIELD; 8] };

    /*
     * TA Document 2001001
     * AV/C Panel Subunit Specification 1.1
     */
    a[0x7d] = avc_opcode_info { name: c"gui update".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x7e] = avc_opcode_info { name: c"push gui data".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x7f] = avc_opcode_info { name: c"user action".as_ptr(), fields: [NULL_FIELD; 8] };
    a[0x7c] = avc_opcode_info { name: c"pass through".as_ptr(), fields: [NULL_FIELD; 8] };

    /* */
    a[0x26] = avc_opcode_info { name: c"asynchronous connection".as_ptr(), fields: [NULL_FIELD; 8] };

    a
};

#[repr(C)]
pub struct avc_frame {
    bits: u32,
}

impl avc_frame {
    unsafe fn operand0(&self) -> u32 {
        self.bits & 0xff
    }

    unsafe fn opcode(&self) -> u32 {
        (self.bits >> 8) & 0xff
    }

    unsafe fn subunit_id(&self) -> u32 {
        (self.bits >> 16) & 0x7
    }

    unsafe fn subunit_type(&self) -> u32 {
        (self.bits >> 19) & 0x1f
    }

    unsafe fn ctype(&self) -> u32 {
        (self.bits >> 24) & 0xf
    }

    unsafe fn cts(&self) -> u32 {
        (self.bits >> 28) & 0xf
    }
}

unsafe fn decode_avc(t: *mut link_transaction) {
    let frame = (*(*t).request).packet.write_block.data as *mut avc_frame;
    let info: *const avc_opcode_info;
    let name: *const c_char;
    let mut buffer = [0 as c_char; 32];
    let mut i: c_int;

    info = &OPCODE_INFO[(*frame).opcode() as usize];
    if (*info).name.is_null() {
        snprintf(
            buffer.as_mut_ptr(),
            buffer.len(),
            c"(unknown opcode 0x%02x)".as_ptr(),
            (*frame).opcode(),
        );
        name = buffer.as_ptr();
    } else {
        name = (*info).name;
    }

    printf(
        c"av/c %s, subunit_type=%s, subunit_id=%u, opcode=%s".as_ptr(),
        CTYPE_NAMES[(*frame).ctype() as usize].as_ptr() as *const c_char,
        SUBUNIT_TYPE_NAMES[(*frame).subunit_type() as usize].as_ptr() as *const c_char,
        (*frame).subunit_id(),
        name,
    );

    i = 0;
    while !(*info).fields[i as usize].name.is_null() {
        printf(c", %s".as_ptr(), (*info).fields[i as usize].name);
        i += 1;
    }

    printf(c"\n".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_fcp(t: *mut link_transaction) -> c_int {
    let frame = (*(*t).request).packet.write_block.data as *mut avc_frame;
    let offset =
        (((*(*t).request).packet.common.offset_high as c_ulonglong) << 32)
            | (*(*t).request).packet.common.offset_low as c_ulonglong;

    if (*(*t).request).packet.common.tcode != TCODE_WRITE_BLOCK_REQUEST {
        return 0;
    }

    if offset == CSR_FCP_COMMAND || offset == CSR_FCP_RESPONSE {
        match (*frame).cts() {
            0x00 => {
                decode_avc(t);
            }
            0x01 => {
                printf(c"cal fcp frame (cts=0x01)\n".as_ptr());
            }
            0x02 => {
                printf(c"ehs fcp frame (cts=0x02)\n".as_ptr());
            }
            0x03 => {
                printf(c"havi fcp frame (cts=0x03)\n".as_ptr());
            }
            0x0e => {
                printf(c"vendor specific fcp frame (cts=0x0e)\n".as_ptr());
            }
            0x0f => {
                printf(c"extended cts\n".as_ptr());
            }
            _ => {
                printf(c"reserved fcp frame (ctx=0x%02x)\n".as_ptr(), (*frame).cts());
            }
        }
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
