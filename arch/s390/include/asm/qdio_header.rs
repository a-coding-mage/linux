/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of qdio.h. External kernel types are supplied by dependencies. */

// Includes omitted; required external names: ccw_device, dma64_t, PAGE_SIZE.

pub const QDIO_MAX_QUEUES_PER_IRQ: usize = 4;
pub const QDIO_MAX_BUFFERS_PER_Q: usize = 128;
pub const QDIO_MAX_BUFFERS_MASK: usize = QDIO_MAX_BUFFERS_PER_Q - 1;
#[inline]
pub const fn QDIO_BUFNR(num: usize) -> usize { num & QDIO_MAX_BUFFERS_MASK }
pub const QDIO_MAX_ELEMENTS_PER_BUFFER: usize = 16;

pub const QDIO_QETH_QFMT: u32 = 0;
pub const QDIO_ZFCP_QFMT: u32 = 1;
pub const QDIO_IQDIO_QFMT: u32 = 2;

#[repr(C, packed)]
pub struct qdesfmt0 {
    pub sliba: u64,
    pub sla: u64,
    pub slsba: u64,
    pub reserved0: u32,
    // C bitfields: akey:4, bkey:4, ckey:4, dkey:4, reserved:16.
    pub keys: u32,
}

pub const QDR_AC_MULTI_BUFFER_ENABLE: u32 = 0x01;

#[repr(C, packed, align(4096))]
pub struct qdr {
    // C bitfields: qfmt:8, reserved:16, ac:8, reserved:8,
    // iqdcnt:8, reserved:8, oqdcnt:8, reserved:8,
    // iqdsz:8, reserved:8, oqdsz:8, reserved:8.
    pub format_counts: [u32; 3],
    pub res: [u32; 9],
    pub qiba: u64,
    pub reserved0: u32,
    // C bitfields: qkey:4, reserved:28.
    pub qkey: u32,
    pub qdf0: [qdesfmt0; 126],
}

pub const QIB_AC_OUTBOUND_PCI_SUPPORTED: u8 = 0x40;
pub const QIB_RFLAGS_ENABLE_QEBSM: u8 = 0x80;
pub const QIB_RFLAGS_ENABLE_DATA_DIV: u8 = 0x02;

#[repr(C, packed, align(256))]
pub struct qib {
    // C bitfields: qfmt:8, pfmt:8, rflags:8, ac:8.
    pub qfmt_pfmt_rflags_ac: u32,
    pub reserved0: u64,
    pub isliba: u64,
    pub osliba: u64,
    pub reserved1: [u32; 2],
    pub ebcnam: [u8; 8],
    pub res: [u8; 88],
    pub parm: [u8; 128],
}

#[repr(C)]
pub struct slibe { pub parms: u64 }

#[repr(C, packed, align(256))]
pub struct qaob {
    pub res0: [u64; 6],
    pub res1: u8,
    pub res2: u8,
    pub res3: u8,
    pub aorc: u8,
    pub flags: u8,
    pub cbtbs: u16,
    pub sb_count: u8,
    pub sba: [u64; QDIO_MAX_ELEMENTS_PER_BUFFER],
    pub dcount: [u16; QDIO_MAX_ELEMENTS_PER_BUFFER],
    pub user0: u64,
    pub res4: [u64; 2],
    pub user1: [u8; 16],
}

#[repr(C, packed, align(2048))]
pub struct slib {
    pub nsliba: u64,
    pub sla: u64,
    pub slsba: u64,
    pub res: [u8; 1000],
    pub slibe: [slibe; QDIO_MAX_BUFFERS_PER_Q],
}

pub const SBAL_EFLAGS_LAST_ENTRY: u8 = 0x40;
pub const SBAL_EFLAGS_CONTIGUOUS: u8 = 0x20;
pub const SBAL_EFLAGS_FIRST_FRAG: u8 = 0x04;
pub const SBAL_EFLAGS_MIDDLE_FRAG: u8 = 0x08;
pub const SBAL_EFLAGS_LAST_FRAG: u8 = 0x0c;
pub const SBAL_EFLAGS_MASK: u8 = 0x6f;
pub const SBAL_SFLAGS0_PCI_REQ: u8 = 0x40;
pub const SBAL_SFLAGS0_DATA_CONTINUATION: u8 = 0x20;
pub const SBAL_SFLAGS0_TYPE_STATUS: u8 = 0x00;
pub const SBAL_SFLAGS0_TYPE_WRITE: u8 = 0x08;
pub const SBAL_SFLAGS0_TYPE_READ: u8 = 0x10;
pub const SBAL_SFLAGS0_TYPE_WRITE_READ: u8 = 0x18;
pub const SBAL_SFLAGS0_MORE_SBALS: u8 = 0x04;
pub const SBAL_SFLAGS0_COMMAND: u8 = 0x02;
pub const SBAL_SFLAGS0_LAST_SBAL: u8 = 0x00;
pub const SBAL_SFLAGS0_ONLY_SBAL: u8 = SBAL_SFLAGS0_COMMAND;
pub const SBAL_SFLAGS0_MIDDLE_SBAL: u8 = SBAL_SFLAGS0_MORE_SBALS;
pub const SBAL_SFLAGS0_FIRST_SBAL: u8 = SBAL_SFLAGS0_MORE_SBALS | SBAL_SFLAGS0_COMMAND;

#[repr(C, packed, align(16))]
pub struct qdio_buffer_element {
    pub eflags: u8,
    pub res1: u8,
    pub scount: u8,
    pub sflags: u8,
    pub length: u32,
    pub addr: u64,
}

#[repr(C, packed, align(256))]
pub struct qdio_buffer { pub element: [qdio_buffer_element; QDIO_MAX_ELEMENTS_PER_BUFFER] }

#[repr(C, packed)]
pub struct sl_element { pub sbal: u64 }

#[repr(C, packed, align(1024))]
pub struct sl { pub element: [sl_element; QDIO_MAX_BUFFERS_PER_Q] }

#[repr(C, packed, align(256))]
pub struct slsb { pub val: [u8; QDIO_MAX_BUFFERS_PER_Q] }

pub const CHSC_AC1_INITIATE_INPUTQ: u8 = 0x80;
pub const AC1_SIGA_INPUT_NEEDED: u8 = 0x40;
pub const AC1_SIGA_OUTPUT_NEEDED: u8 = 0x20;
pub const AC1_SIGA_SYNC_NEEDED: u8 = 0x10;
pub const AC1_AUTOMATIC_SYNC_ON_THININT: u8 = 0x08;
pub const AC1_AUTOMATIC_SYNC_ON_OUT_PCI: u8 = 0x04;
pub const AC1_SC_QEBSM_AVAILABLE: u8 = 0x02;
pub const AC1_SC_QEBSM_ENABLED: u8 = 0x01;
pub const CHSC_AC2_MULTI_BUFFER_AVAILABLE: u16 = 0x0080;
pub const CHSC_AC2_MULTI_BUFFER_ENABLED: u16 = 0x0040;
pub const CHSC_AC2_DATA_DIV_AVAILABLE: u16 = 0x0010;
pub const CHSC_AC2_SNIFFER_AVAILABLE: u16 = 0x0008;
pub const CHSC_AC2_DATA_DIV_ENABLED: u16 = 0x0002;
pub const CHSC_AC3_FORMAT2_CQ_AVAILABLE: u16 = 0x8000;

#[repr(C, packed)]
pub struct qdio_ssqd_desc {
    pub flags: u8, pub reserved0: u8, pub sch: u16, pub qfmt: u8, pub parm: u8,
    pub qdioac1: u8, pub sch_class: u8, pub pcnt: u8, pub icnt: u8, pub reserved1: u8,
    pub ocnt: u8, pub reserved2: u8, pub mbccnt: u8, pub qdioac2: u16,
    pub sch_token: u64, pub mro: u8, pub mri: u8, pub qdioac3: u16,
    pub reserved3: u16, pub reserved4: u8, pub mmwc: u8,
}

pub type qdio_handler_t = unsafe extern "C" fn(*mut ccw_device, u32, i32, i32, i32, u64);

pub const QDIO_ERROR_ACTIVATE: u32 = 0x0001;
pub const QDIO_ERROR_GET_BUF_STATE: u32 = 0x0002;
pub const QDIO_ERROR_SET_BUF_STATE: u32 = 0x0004;
pub const QDIO_ERROR_SLSB_STATE: u32 = 0x0100;
pub const QDIO_ERROR_SLSB_PENDING: u32 = 0x0200;
pub const QDIO_FLAG_CLEANUP_USING_CLEAR: u32 = 0x01;
pub const QDIO_FLAG_CLEANUP_USING_HALT: u32 = 0x02;

#[repr(C)]
pub struct qdio_initialize {
    pub q_format: u8, pub qdr_ac: u8, pub qib_param_field_format: u32,
    pub qib_param_field: *mut u8, pub qib_rflags: u8, pub no_input_qs: u32,
    pub no_output_qs: u32, pub input_handler: Option<qdio_handler_t>,
    pub output_handler: Option<qdio_handler_t>,
    pub irq_poll: Option<unsafe extern "C" fn(*mut ccw_device, u64)>,
    pub int_parm: u64,
    pub input_sbal_addr_array: *mut *mut *mut qdio_buffer,
    pub output_sbal_addr_array: *mut *mut *mut qdio_buffer,
}

extern "C" {
    pub fn qdio_alloc_buffers(buf: *mut *mut qdio_buffer, count: u32) -> i32;
    pub fn qdio_free_buffers(buf: *mut *mut qdio_buffer, count: u32);
    pub fn qdio_reset_buffers(buf: *mut *mut qdio_buffer, count: u32);
    pub fn qdio_allocate(cdev: *mut ccw_device, no_input_qs: u32, no_output_qs: u32) -> i32;
    pub fn qdio_establish(cdev: *mut ccw_device, init_data: *mut qdio_initialize) -> i32;
    pub fn qdio_activate(cdev: *mut ccw_device) -> i32;
    pub fn qdio_start_irq(cdev: *mut ccw_device) -> i32;
    pub fn qdio_stop_irq(cdev: *mut ccw_device) -> i32;
    pub fn qdio_inspect_input_queue(cdev: *mut ccw_device, nr: u32, bufnr: *mut u32, error: *mut u32) -> i32;
    pub fn qdio_inspect_output_queue(cdev: *mut ccw_device, nr: u32, bufnr: *mut u32, error: *mut u32) -> i32;
    pub fn qdio_add_bufs_to_input_queue(cdev: *mut ccw_device, q_nr: u32, bufnr: u32, count: u32) -> i32;
    pub fn qdio_add_bufs_to_output_queue(cdev: *mut ccw_device, q_nr: u32, bufnr: u32, count: u32, aob: *mut qaob) -> i32;
    pub fn qdio_shutdown(cdev: *mut ccw_device, how: i32) -> i32;
    pub fn qdio_free(cdev: *mut ccw_device) -> i32;
    pub fn qdio_get_ssqd_desc(cdev: *mut ccw_device, desc: *mut qdio_ssqd_desc) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
