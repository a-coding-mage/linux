/* Translated from cvmx-pko-defs.h. */
/* __BIG_ENDIAN_BITFIELD selects the declared bitfield ordering in the C source. */
pub const CVMX_PKO_MEM_COUNT0: u64 = CVMX_ADD_IO_SEG(0x0001180050001080ull);
pub const CVMX_PKO_MEM_COUNT1: u64 = CVMX_ADD_IO_SEG(0x0001180050001088ull);
pub const CVMX_PKO_MEM_DEBUG0: u64 = CVMX_ADD_IO_SEG(0x0001180050001100ull);
pub const CVMX_PKO_MEM_DEBUG1: u64 = CVMX_ADD_IO_SEG(0x0001180050001108ull);
pub const CVMX_PKO_MEM_DEBUG10: u64 = CVMX_ADD_IO_SEG(0x0001180050001150ull);
pub const CVMX_PKO_MEM_DEBUG11: u64 = CVMX_ADD_IO_SEG(0x0001180050001158ull);
pub const CVMX_PKO_MEM_DEBUG12: u64 = CVMX_ADD_IO_SEG(0x0001180050001160ull);
pub const CVMX_PKO_MEM_DEBUG13: u64 = CVMX_ADD_IO_SEG(0x0001180050001168ull);
pub const CVMX_PKO_MEM_DEBUG14: u64 = CVMX_ADD_IO_SEG(0x0001180050001170ull);
pub const CVMX_PKO_MEM_DEBUG2: u64 = CVMX_ADD_IO_SEG(0x0001180050001110ull);
pub const CVMX_PKO_MEM_DEBUG3: u64 = CVMX_ADD_IO_SEG(0x0001180050001118ull);
pub const CVMX_PKO_MEM_DEBUG4: u64 = CVMX_ADD_IO_SEG(0x0001180050001120ull);
pub const CVMX_PKO_MEM_DEBUG5: u64 = CVMX_ADD_IO_SEG(0x0001180050001128ull);
pub const CVMX_PKO_MEM_DEBUG6: u64 = CVMX_ADD_IO_SEG(0x0001180050001130ull);
pub const CVMX_PKO_MEM_DEBUG7: u64 = CVMX_ADD_IO_SEG(0x0001180050001138ull);
pub const CVMX_PKO_MEM_DEBUG8: u64 = CVMX_ADD_IO_SEG(0x0001180050001140ull);
pub const CVMX_PKO_MEM_DEBUG9: u64 = CVMX_ADD_IO_SEG(0x0001180050001148ull);
pub const CVMX_PKO_MEM_IPORT_PTRS: u64 = CVMX_ADD_IO_SEG(0x0001180050001030ull);
pub const CVMX_PKO_MEM_IPORT_QOS: u64 = CVMX_ADD_IO_SEG(0x0001180050001038ull);
pub const CVMX_PKO_MEM_IQUEUE_PTRS: u64 = CVMX_ADD_IO_SEG(0x0001180050001040ull);
pub const CVMX_PKO_MEM_IQUEUE_QOS: u64 = CVMX_ADD_IO_SEG(0x0001180050001048ull);
pub const CVMX_PKO_MEM_PORT_PTRS: u64 = CVMX_ADD_IO_SEG(0x0001180050001010ull);
pub const CVMX_PKO_MEM_PORT_QOS: u64 = CVMX_ADD_IO_SEG(0x0001180050001018ull);
pub const CVMX_PKO_MEM_PORT_RATE0: u64 = CVMX_ADD_IO_SEG(0x0001180050001020ull);
pub const CVMX_PKO_MEM_PORT_RATE1: u64 = CVMX_ADD_IO_SEG(0x0001180050001028ull);
pub const CVMX_PKO_MEM_QUEUE_PTRS: u64 = CVMX_ADD_IO_SEG(0x0001180050001000ull);
pub const CVMX_PKO_MEM_QUEUE_QOS: u64 = CVMX_ADD_IO_SEG(0x0001180050001008ull);
pub const CVMX_PKO_MEM_THROTTLE_INT: u64 = CVMX_ADD_IO_SEG(0x0001180050001058ull);
pub const CVMX_PKO_MEM_THROTTLE_PIPE: u64 = CVMX_ADD_IO_SEG(0x0001180050001050ull);
pub const CVMX_PKO_REG_BIST_RESULT: u64 = CVMX_ADD_IO_SEG(0x0001180050000080ull);
pub const CVMX_PKO_REG_CMD_BUF: u64 = CVMX_ADD_IO_SEG(0x0001180050000010ull);
#[inline] pub const fn CVMX_PKO_REG_CRC_CTLX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180050000028ull) + (offset & 1) * 8 }
pub const CVMX_PKO_REG_CRC_ENABLE: u64 = CVMX_ADD_IO_SEG(0x0001180050000020ull);
#[inline] pub const fn CVMX_PKO_REG_CRC_IVX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180050000038ull) + (offset & 1) * 8 }
pub const CVMX_PKO_REG_DEBUG0: u64 = CVMX_ADD_IO_SEG(0x0001180050000098ull);
pub const CVMX_PKO_REG_DEBUG1: u64 = CVMX_ADD_IO_SEG(0x00011800500000A0ull);
pub const CVMX_PKO_REG_DEBUG2: u64 = CVMX_ADD_IO_SEG(0x00011800500000A8ull);
pub const CVMX_PKO_REG_DEBUG3: u64 = CVMX_ADD_IO_SEG(0x00011800500000B0ull);
pub const CVMX_PKO_REG_DEBUG4: u64 = CVMX_ADD_IO_SEG(0x00011800500000B8ull);
pub const CVMX_PKO_REG_ENGINE_INFLIGHT: u64 = CVMX_ADD_IO_SEG(0x0001180050000050ull);
pub const CVMX_PKO_REG_ENGINE_INFLIGHT1: u64 = CVMX_ADD_IO_SEG(0x0001180050000318ull);
#[inline] pub const fn CVMX_PKO_REG_ENGINE_STORAGEX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180050000300ull) + (offset & 1) * 8 }
pub const CVMX_PKO_REG_ENGINE_THRESH: u64 = CVMX_ADD_IO_SEG(0x0001180050000058ull);
pub const CVMX_PKO_REG_ERROR: u64 = CVMX_ADD_IO_SEG(0x0001180050000088ull);
pub const CVMX_PKO_REG_FLAGS: u64 = CVMX_ADD_IO_SEG(0x0001180050000000ull);
pub const CVMX_PKO_REG_GMX_PORT_MODE: u64 = CVMX_ADD_IO_SEG(0x0001180050000018ull);
pub const CVMX_PKO_REG_INT_MASK: u64 = CVMX_ADD_IO_SEG(0x0001180050000090ull);
pub const CVMX_PKO_REG_LOOPBACK_BPID: u64 = CVMX_ADD_IO_SEG(0x0001180050000118ull);
pub const CVMX_PKO_REG_LOOPBACK_PKIND: u64 = CVMX_ADD_IO_SEG(0x0001180050000068ull);
pub const CVMX_PKO_REG_MIN_PKT: u64 = CVMX_ADD_IO_SEG(0x0001180050000070ull);
pub const CVMX_PKO_REG_PREEMPT: u64 = CVMX_ADD_IO_SEG(0x0001180050000110ull);
pub const CVMX_PKO_REG_QUEUE_MODE: u64 = CVMX_ADD_IO_SEG(0x0001180050000048ull);
pub const CVMX_PKO_REG_QUEUE_PREEMPT: u64 = CVMX_ADD_IO_SEG(0x0001180050000108ull);
pub const CVMX_PKO_REG_QUEUE_PTRS1: u64 = CVMX_ADD_IO_SEG(0x0001180050000100ull);
pub const CVMX_PKO_REG_READ_IDX: u64 = CVMX_ADD_IO_SEG(0x0001180050000008ull);
pub const CVMX_PKO_REG_THROTTLE: u64 = CVMX_ADD_IO_SEG(0x0001180050000078ull);
pub const CVMX_PKO_REG_TIMESTAMP: u64 = CVMX_ADD_IO_SEG(0x0001180050000060ull);

#[repr(C)]
pub union cvmx_pko_mem_count0 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_count0_s: cvmx_pko_mem_count0_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_count0_s {
    pub reserved_32_63: u64, // C bitfield width: 32
    pub count: u64, // C bitfield width: 32
}

#[repr(C)]
pub union cvmx_pko_mem_count1 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_count1_s: cvmx_pko_mem_count1_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_count1_s {
    pub reserved_48_63: u64, // C bitfield width: 16
    pub count: u64, // C bitfield width: 48
}

#[repr(C)]
pub union cvmx_pko_mem_debug0 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug0_s: cvmx_pko_mem_debug0_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug0_s {
    pub fau: u64, // C bitfield width: 28
    pub cmd: u64, // C bitfield width: 14
    pub segs: u64, // C bitfield width: 6
    pub size: u64, // C bitfield width: 16
}

#[repr(C)]
pub union cvmx_pko_mem_debug1 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug1_s: cvmx_pko_mem_debug1_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug1_s {
    pub i: u64, // C bitfield width: 1
    pub back: u64, // C bitfield width: 4
    pub pool: u64, // C bitfield width: 3
    pub size: u64, // C bitfield width: 16
    pub ptr: u64, // C bitfield width: 40
}

#[repr(C)]
pub union cvmx_pko_mem_debug10 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug10_s: cvmx_pko_mem_debug10_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug10_cn30xx: cvmx_pko_mem_debug10_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug10_cn50xx: cvmx_pko_mem_debug10_cn50xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug10_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug10_cn30xx {
    pub fau: u64, // C bitfield width: 28
    pub cmd: u64, // C bitfield width: 14
    pub segs: u64, // C bitfield width: 6
    pub size: u64, // C bitfield width: 16
}

#[repr(C)]
pub struct cvmx_pko_mem_debug10_cn50xx {
    pub reserved_49_63: u64, // C bitfield width: 15
    pub ptrs1: u64, // C bitfield width: 17
    pub reserved_17_31: u64, // C bitfield width: 15
    pub ptrs2: u64, // C bitfield width: 17
}

#[repr(C)]
pub union cvmx_pko_mem_debug11 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug11_s: cvmx_pko_mem_debug11_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug11_cn30xx: cvmx_pko_mem_debug11_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug11_cn50xx: cvmx_pko_mem_debug11_cn50xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug11_s {
    pub i: u64, // C bitfield width: 1
    pub back: u64, // C bitfield width: 4
    pub pool: u64, // C bitfield width: 3
    pub size: u64, // C bitfield width: 16
    pub reserved_0_39: u64, // C bitfield width: 40
}

#[repr(C)]
pub struct cvmx_pko_mem_debug11_cn30xx {
    pub i: u64, // C bitfield width: 1
    pub back: u64, // C bitfield width: 4
    pub pool: u64, // C bitfield width: 3
    pub size: u64, // C bitfield width: 16
    pub ptr: u64, // C bitfield width: 40
}

#[repr(C)]
pub struct cvmx_pko_mem_debug11_cn50xx {
    pub reserved_23_63: u64, // C bitfield width: 41
    pub maj: u64, // C bitfield width: 1
    pub uid: u64, // C bitfield width: 3
    pub sop: u64, // C bitfield width: 1
    pub len: u64, // C bitfield width: 1
    pub chk: u64, // C bitfield width: 1
    pub cnt: u64, // C bitfield width: 13
    pub mod: u64, // C bitfield width: 3
}

#[repr(C)]
pub union cvmx_pko_mem_debug12 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug12_s: cvmx_pko_mem_debug12_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug12_cn30xx: cvmx_pko_mem_debug12_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug12_cn50xx: cvmx_pko_mem_debug12_cn50xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug12_cn68xx: cvmx_pko_mem_debug12_cn68xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug12_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug12_cn30xx {
    pub data: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug12_cn50xx {
    pub fau: u64, // C bitfield width: 28
    pub cmd: u64, // C bitfield width: 14
    pub segs: u64, // C bitfield width: 6
    pub size: u64, // C bitfield width: 16
}

#[repr(C)]
pub struct cvmx_pko_mem_debug12_cn68xx {
    pub state: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_mem_debug13 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug13_s: cvmx_pko_mem_debug13_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug13_cn30xx: cvmx_pko_mem_debug13_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug13_cn50xx: cvmx_pko_mem_debug13_cn50xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug13_cn68xx: cvmx_pko_mem_debug13_cn68xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug13_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug13_cn30xx {
    pub reserved_51_63: u64, // C bitfield width: 13
    pub widx: u64, // C bitfield width: 17
    pub ridx2: u64, // C bitfield width: 17
    pub widx2: u64, // C bitfield width: 17
}

#[repr(C)]
pub struct cvmx_pko_mem_debug13_cn50xx {
    pub i: u64, // C bitfield width: 1
    pub back: u64, // C bitfield width: 4
    pub pool: u64, // C bitfield width: 3
    pub size: u64, // C bitfield width: 16
    pub ptr: u64, // C bitfield width: 40
}

#[repr(C)]
pub struct cvmx_pko_mem_debug13_cn68xx {
    pub state: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_mem_debug14 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug14_s: cvmx_pko_mem_debug14_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug14_cn30xx: cvmx_pko_mem_debug14_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug14_cn52xx: cvmx_pko_mem_debug14_cn52xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug14_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug14_cn30xx {
    pub reserved_17_63: u64, // C bitfield width: 47
    pub ridx: u64, // C bitfield width: 17
}

#[repr(C)]
pub struct cvmx_pko_mem_debug14_cn52xx {
    pub data: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_mem_debug2 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug2_s: cvmx_pko_mem_debug2_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug2_s {
    pub i: u64, // C bitfield width: 1
    pub back: u64, // C bitfield width: 4
    pub pool: u64, // C bitfield width: 3
    pub size: u64, // C bitfield width: 16
    pub ptr: u64, // C bitfield width: 40
}

#[repr(C)]
pub union cvmx_pko_mem_debug3 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug3_s: cvmx_pko_mem_debug3_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug3_cn30xx: cvmx_pko_mem_debug3_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug3_cn50xx: cvmx_pko_mem_debug3_cn50xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug3_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug3_cn30xx {
    pub i: u64, // C bitfield width: 1
    pub back: u64, // C bitfield width: 4
    pub pool: u64, // C bitfield width: 3
    pub size: u64, // C bitfield width: 16
    pub ptr: u64, // C bitfield width: 40
}

#[repr(C)]
pub struct cvmx_pko_mem_debug3_cn50xx {
    pub data: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_mem_debug4 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug4_s: cvmx_pko_mem_debug4_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug4_cn30xx: cvmx_pko_mem_debug4_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug4_cn50xx: cvmx_pko_mem_debug4_cn50xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug4_cn52xx: cvmx_pko_mem_debug4_cn52xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug4_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug4_cn30xx {
    pub data: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug4_cn50xx {
    pub cmnd_segs: u64, // C bitfield width: 3
    pub cmnd_siz: u64, // C bitfield width: 16
    pub cmnd_off: u64, // C bitfield width: 6
    pub uid: u64, // C bitfield width: 3
    pub dread_sop: u64, // C bitfield width: 1
    pub init_dwrite: u64, // C bitfield width: 1
    pub chk_once: u64, // C bitfield width: 1
    pub chk_mode: u64, // C bitfield width: 1
    pub active: u64, // C bitfield width: 1
    pub static_p: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
    pub qcb_ridx: u64, // C bitfield width: 5
    pub qid_off_max: u64, // C bitfield width: 4
    pub qid_off: u64, // C bitfield width: 4
    pub qid_base: u64, // C bitfield width: 8
    pub wait: u64, // C bitfield width: 1
    pub minor: u64, // C bitfield width: 2
    pub major: u64, // C bitfield width: 3
}

#[repr(C)]
pub struct cvmx_pko_mem_debug4_cn52xx {
    pub curr_siz: u64, // C bitfield width: 8
    pub curr_off: u64, // C bitfield width: 16
    pub cmnd_segs: u64, // C bitfield width: 6
    pub cmnd_siz: u64, // C bitfield width: 16
    pub cmnd_off: u64, // C bitfield width: 6
    pub uid: u64, // C bitfield width: 2
    pub dread_sop: u64, // C bitfield width: 1
    pub init_dwrite: u64, // C bitfield width: 1
    pub chk_once: u64, // C bitfield width: 1
    pub chk_mode: u64, // C bitfield width: 1
    pub wait: u64, // C bitfield width: 1
    pub minor: u64, // C bitfield width: 2
    pub major: u64, // C bitfield width: 3
}

#[repr(C)]
pub union cvmx_pko_mem_debug5 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug5_s: cvmx_pko_mem_debug5_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug5_cn30xx: cvmx_pko_mem_debug5_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug5_cn50xx: cvmx_pko_mem_debug5_cn50xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug5_cn52xx: cvmx_pko_mem_debug5_cn52xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug5_cn61xx: cvmx_pko_mem_debug5_cn61xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug5_cn68xx: cvmx_pko_mem_debug5_cn68xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug5_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug5_cn30xx {
    pub dwri_mod: u64, // C bitfield width: 1
    pub dwri_sop: u64, // C bitfield width: 1
    pub dwri_len: u64, // C bitfield width: 1
    pub dwri_cnt: u64, // C bitfield width: 13
    pub cmnd_siz: u64, // C bitfield width: 16
    pub uid: u64, // C bitfield width: 1
    pub xfer_wor: u64, // C bitfield width: 1
    pub xfer_dwr: u64, // C bitfield width: 1
    pub cbuf_fre: u64, // C bitfield width: 1
    pub reserved_27_27: u64, // C bitfield width: 1
    pub chk_mode: u64, // C bitfield width: 1
    pub active: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
    pub qcb_ridx: u64, // C bitfield width: 5
    pub qid_off: u64, // C bitfield width: 3
    pub qid_base: u64, // C bitfield width: 7
    pub wait: u64, // C bitfield width: 1
    pub minor: u64, // C bitfield width: 2
    pub major: u64, // C bitfield width: 4
}

#[repr(C)]
pub struct cvmx_pko_mem_debug5_cn50xx {
    pub curr_ptr: u64, // C bitfield width: 29
    pub curr_siz: u64, // C bitfield width: 16
    pub curr_off: u64, // C bitfield width: 16
    pub cmnd_segs: u64, // C bitfield width: 3
}

#[repr(C)]
pub struct cvmx_pko_mem_debug5_cn52xx {
    pub reserved_54_63: u64, // C bitfield width: 10
    pub nxt_inflt: u64, // C bitfield width: 6
    pub curr_ptr: u64, // C bitfield width: 40
    pub curr_siz: u64, // C bitfield width: 8
}

#[repr(C)]
pub struct cvmx_pko_mem_debug5_cn61xx {
    pub reserved_56_63: u64, // C bitfield width: 8
    pub ptp: u64, // C bitfield width: 1
    pub major_3: u64, // C bitfield width: 1
    pub nxt_inflt: u64, // C bitfield width: 6
    pub curr_ptr: u64, // C bitfield width: 40
    pub curr_siz: u64, // C bitfield width: 8
}

#[repr(C)]
pub struct cvmx_pko_mem_debug5_cn68xx {
    pub reserved_57_63: u64, // C bitfield width: 7
    pub uid_2: u64, // C bitfield width: 1
    pub ptp: u64, // C bitfield width: 1
    pub major_3: u64, // C bitfield width: 1
    pub nxt_inflt: u64, // C bitfield width: 6
    pub curr_ptr: u64, // C bitfield width: 40
    pub curr_siz: u64, // C bitfield width: 8
}

#[repr(C)]
pub union cvmx_pko_mem_debug6 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug6_s: cvmx_pko_mem_debug6_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug6_cn30xx: cvmx_pko_mem_debug6_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug6_cn50xx: cvmx_pko_mem_debug6_cn50xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug6_cn52xx: cvmx_pko_mem_debug6_cn52xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug6_s {
    pub reserved_37_63: u64, // C bitfield width: 27
    pub qid_offres: u64, // C bitfield width: 4
    pub qid_offths: u64, // C bitfield width: 4
    pub preempter: u64, // C bitfield width: 1
    pub preemptee: u64, // C bitfield width: 1
    pub preempted: u64, // C bitfield width: 1
    pub active: u64, // C bitfield width: 1
    pub statc: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
    pub qcb_ridx: u64, // C bitfield width: 5
    pub qid_offmax: u64, // C bitfield width: 4
    pub reserved_0_11: u64, // C bitfield width: 12
}

#[repr(C)]
pub struct cvmx_pko_mem_debug6_cn30xx {
    pub reserved_11_63: u64, // C bitfield width: 53
    pub qid_offm: u64, // C bitfield width: 3
    pub static_p: u64, // C bitfield width: 1
    pub work_min: u64, // C bitfield width: 3
    pub dwri_chk: u64, // C bitfield width: 1
    pub dwri_uid: u64, // C bitfield width: 1
    pub dwri_mod: u64, // C bitfield width: 2
}

#[repr(C)]
pub struct cvmx_pko_mem_debug6_cn50xx {
    pub reserved_11_63: u64, // C bitfield width: 53
    pub curr_ptr: u64, // C bitfield width: 11
}

#[repr(C)]
pub struct cvmx_pko_mem_debug6_cn52xx {
    pub reserved_37_63: u64, // C bitfield width: 27
    pub qid_offres: u64, // C bitfield width: 4
    pub qid_offths: u64, // C bitfield width: 4
    pub preempter: u64, // C bitfield width: 1
    pub preemptee: u64, // C bitfield width: 1
    pub preempted: u64, // C bitfield width: 1
    pub active: u64, // C bitfield width: 1
    pub statc: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
    pub qcb_ridx: u64, // C bitfield width: 5
    pub qid_offmax: u64, // C bitfield width: 4
    pub qid_off: u64, // C bitfield width: 4
    pub qid_base: u64, // C bitfield width: 8
}

#[repr(C)]
pub union cvmx_pko_mem_debug7 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug7_s: cvmx_pko_mem_debug7_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug7_cn30xx: cvmx_pko_mem_debug7_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug7_cn50xx: cvmx_pko_mem_debug7_cn50xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug7_cn68xx: cvmx_pko_mem_debug7_cn68xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug7_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_mem_debug7_cn30xx {
    pub reserved_58_63: u64, // C bitfield width: 6
    pub dwb: u64, // C bitfield width: 9
    pub start: u64, // C bitfield width: 33
    pub size: u64, // C bitfield width: 16
}

#[repr(C)]
pub struct cvmx_pko_mem_debug7_cn50xx {
    pub qos: u64, // C bitfield width: 5
    pub tail: u64, // C bitfield width: 1
    pub buf_siz: u64, // C bitfield width: 13
    pub buf_ptr: u64, // C bitfield width: 33
    pub qcb_widx: u64, // C bitfield width: 6
    pub qcb_ridx: u64, // C bitfield width: 6
}

#[repr(C)]
pub struct cvmx_pko_mem_debug7_cn68xx {
    pub qos: u64, // C bitfield width: 3
    pub tail: u64, // C bitfield width: 1
    pub buf_siz: u64, // C bitfield width: 13
    pub buf_ptr: u64, // C bitfield width: 33
    pub qcb_widx: u64, // C bitfield width: 7
    pub qcb_ridx: u64, // C bitfield width: 7
}

#[repr(C)]
pub union cvmx_pko_mem_debug8 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug8_s: cvmx_pko_mem_debug8_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug8_cn30xx: cvmx_pko_mem_debug8_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug8_cn50xx: cvmx_pko_mem_debug8_cn50xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug8_cn52xx: cvmx_pko_mem_debug8_cn52xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug8_cn61xx: cvmx_pko_mem_debug8_cn61xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug8_cn68xx: cvmx_pko_mem_debug8_cn68xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug8_s {
    pub reserved_59_63: u64, // C bitfield width: 5
    pub tail: u64, // C bitfield width: 1
    pub buf_siz: u64, // C bitfield width: 13
    pub reserved_0_44: u64, // C bitfield width: 45
}

#[repr(C)]
pub struct cvmx_pko_mem_debug8_cn30xx {
    pub qos: u64, // C bitfield width: 5
    pub tail: u64, // C bitfield width: 1
    pub buf_siz: u64, // C bitfield width: 13
    pub buf_ptr: u64, // C bitfield width: 33
    pub qcb_widx: u64, // C bitfield width: 6
    pub qcb_ridx: u64, // C bitfield width: 6
}

#[repr(C)]
pub struct cvmx_pko_mem_debug8_cn50xx {
    pub reserved_28_63: u64, // C bitfield width: 36
    pub doorbell: u64, // C bitfield width: 20
    pub reserved_6_7: u64, // C bitfield width: 2
    pub static_p: u64, // C bitfield width: 1
    pub s_tail: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
}

#[repr(C)]
pub struct cvmx_pko_mem_debug8_cn52xx {
    pub reserved_29_63: u64, // C bitfield width: 35
    pub preempter: u64, // C bitfield width: 1
    pub doorbell: u64, // C bitfield width: 20
    pub reserved_7_7: u64, // C bitfield width: 1
    pub preemptee: u64, // C bitfield width: 1
    pub static_p: u64, // C bitfield width: 1
    pub s_tail: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
}

#[repr(C)]
pub struct cvmx_pko_mem_debug8_cn61xx {
    pub reserved_42_63: u64, // C bitfield width: 22
    pub qid_qqos: u64, // C bitfield width: 8
    pub reserved_33_33: u64, // C bitfield width: 1
    pub qid_idx: u64, // C bitfield width: 4
    pub preempter: u64, // C bitfield width: 1
    pub doorbell: u64, // C bitfield width: 20
    pub reserved_7_7: u64, // C bitfield width: 1
    pub preemptee: u64, // C bitfield width: 1
    pub static_p: u64, // C bitfield width: 1
    pub s_tail: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
}

#[repr(C)]
pub struct cvmx_pko_mem_debug8_cn68xx {
    pub reserved_37_63: u64, // C bitfield width: 27
    pub preempter: u64, // C bitfield width: 1
    pub doorbell: u64, // C bitfield width: 20
    pub reserved_9_15: u64, // C bitfield width: 7
    pub preemptee: u64, // C bitfield width: 1
    pub static_p: u64, // C bitfield width: 1
    pub s_tail: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 5
}

#[repr(C)]
pub union cvmx_pko_mem_debug9 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_debug9_s: cvmx_pko_mem_debug9_s,
    #[repr(C)]
    pub cvmx_pko_mem_debug9_cn30xx: cvmx_pko_mem_debug9_cn30xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug9_cn38xx: cvmx_pko_mem_debug9_cn38xx,
    #[repr(C)]
    pub cvmx_pko_mem_debug9_cn50xx: cvmx_pko_mem_debug9_cn50xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_debug9_s {
    pub reserved_49_63: u64, // C bitfield width: 15
    pub ptrs0: u64, // C bitfield width: 17
    pub reserved_0_31: u64, // C bitfield width: 32
}

#[repr(C)]
pub struct cvmx_pko_mem_debug9_cn30xx {
    pub reserved_28_63: u64, // C bitfield width: 36
    pub doorbell: u64, // C bitfield width: 20
    pub reserved_5_7: u64, // C bitfield width: 3
    pub s_tail: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
}

#[repr(C)]
pub struct cvmx_pko_mem_debug9_cn38xx {
    pub reserved_28_63: u64, // C bitfield width: 36
    pub doorbell: u64, // C bitfield width: 20
    pub reserved_6_7: u64, // C bitfield width: 2
    pub static_p: u64, // C bitfield width: 1
    pub s_tail: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
}

#[repr(C)]
pub struct cvmx_pko_mem_debug9_cn50xx {
    pub reserved_49_63: u64, // C bitfield width: 15
    pub ptrs0: u64, // C bitfield width: 17
    pub reserved_17_31: u64, // C bitfield width: 15
    pub ptrs3: u64, // C bitfield width: 17
}

#[repr(C)]
pub union cvmx_pko_mem_iport_ptrs {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_iport_ptrs_s: cvmx_pko_mem_iport_ptrs_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_iport_ptrs_s {
    pub reserved_63_63: u64, // C bitfield width: 1
    pub crc: u64, // C bitfield width: 1
    pub static_p: u64, // C bitfield width: 1
    pub qos_mask: u64, // C bitfield width: 8
    pub min_pkt: u64, // C bitfield width: 3
    pub reserved_31_49: u64, // C bitfield width: 19
    pub pipe: u64, // C bitfield width: 7
    pub reserved_21_23: u64, // C bitfield width: 3
    pub intr: u64, // C bitfield width: 5
    pub reserved_13_15: u64, // C bitfield width: 3
    pub eid: u64, // C bitfield width: 5
    pub reserved_7_7: u64, // C bitfield width: 1
    pub ipid: u64, // C bitfield width: 7
}

#[repr(C)]
pub union cvmx_pko_mem_iport_qos {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_iport_qos_s: cvmx_pko_mem_iport_qos_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_iport_qos_s {
    pub reserved_61_63: u64, // C bitfield width: 3
    pub qos_mask: u64, // C bitfield width: 8
    pub reserved_13_52: u64, // C bitfield width: 40
    pub eid: u64, // C bitfield width: 5
    pub reserved_7_7: u64, // C bitfield width: 1
    pub ipid: u64, // C bitfield width: 7
}

#[repr(C)]
pub union cvmx_pko_mem_iqueue_ptrs {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_iqueue_ptrs_s: cvmx_pko_mem_iqueue_ptrs_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_iqueue_ptrs_s {
    pub s_tail: u64, // C bitfield width: 1
    pub static_p: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos_mask: u64, // C bitfield width: 8
    pub buf_ptr: u64, // C bitfield width: 31
    pub tail: u64, // C bitfield width: 1
    pub index: u64, // C bitfield width: 5
    pub reserved_15_15: u64, // C bitfield width: 1
    pub ipid: u64, // C bitfield width: 7
    pub qid: u64, // C bitfield width: 8
}

#[repr(C)]
pub union cvmx_pko_mem_iqueue_qos {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_iqueue_qos_s: cvmx_pko_mem_iqueue_qos_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_iqueue_qos_s {
    pub reserved_61_63: u64, // C bitfield width: 3
    pub qos_mask: u64, // C bitfield width: 8
    pub reserved_15_52: u64, // C bitfield width: 38
    pub ipid: u64, // C bitfield width: 7
    pub qid: u64, // C bitfield width: 8
}

#[repr(C)]
pub union cvmx_pko_mem_port_ptrs {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_port_ptrs_s: cvmx_pko_mem_port_ptrs_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_port_ptrs_s {
    pub reserved_62_63: u64, // C bitfield width: 2
    pub static_p: u64, // C bitfield width: 1
    pub qos_mask: u64, // C bitfield width: 8
    pub reserved_16_52: u64, // C bitfield width: 37
    pub bp_port: u64, // C bitfield width: 6
    pub eid: u64, // C bitfield width: 4
    pub pid: u64, // C bitfield width: 6
}

#[repr(C)]
pub union cvmx_pko_mem_port_qos {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_port_qos_s: cvmx_pko_mem_port_qos_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_port_qos_s {
    pub reserved_61_63: u64, // C bitfield width: 3
    pub qos_mask: u64, // C bitfield width: 8
    pub reserved_10_52: u64, // C bitfield width: 43
    pub eid: u64, // C bitfield width: 4
    pub pid: u64, // C bitfield width: 6
}

#[repr(C)]
pub union cvmx_pko_mem_port_rate0 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_port_rate0_s: cvmx_pko_mem_port_rate0_s,
    #[repr(C)]
    pub cvmx_pko_mem_port_rate0_cn52xx: cvmx_pko_mem_port_rate0_cn52xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_port_rate0_s {
    pub reserved_51_63: u64, // C bitfield width: 13
    pub rate_word: u64, // C bitfield width: 19
    pub rate_pkt: u64, // C bitfield width: 24
    pub reserved_7_7: u64, // C bitfield width: 1
    pub pid: u64, // C bitfield width: 7
}

#[repr(C)]
pub struct cvmx_pko_mem_port_rate0_cn52xx {
    pub reserved_51_63: u64, // C bitfield width: 13
    pub rate_word: u64, // C bitfield width: 19
    pub rate_pkt: u64, // C bitfield width: 24
    pub reserved_6_7: u64, // C bitfield width: 2
    pub pid: u64, // C bitfield width: 6
}

#[repr(C)]
pub union cvmx_pko_mem_port_rate1 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_port_rate1_s: cvmx_pko_mem_port_rate1_s,
    #[repr(C)]
    pub cvmx_pko_mem_port_rate1_cn52xx: cvmx_pko_mem_port_rate1_cn52xx,
}

#[repr(C)]
pub struct cvmx_pko_mem_port_rate1_s {
    pub reserved_32_63: u64, // C bitfield width: 32
    pub rate_lim: u64, // C bitfield width: 24
    pub reserved_7_7: u64, // C bitfield width: 1
    pub pid: u64, // C bitfield width: 7
}

#[repr(C)]
pub struct cvmx_pko_mem_port_rate1_cn52xx {
    pub reserved_32_63: u64, // C bitfield width: 32
    pub rate_lim: u64, // C bitfield width: 24
    pub reserved_6_7: u64, // C bitfield width: 2
    pub pid: u64, // C bitfield width: 6
}

#[repr(C)]
pub union cvmx_pko_mem_queue_ptrs {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_queue_ptrs_s: cvmx_pko_mem_queue_ptrs_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_queue_ptrs_s {
    pub s_tail: u64, // C bitfield width: 1
    pub static_p: u64, // C bitfield width: 1
    pub static_q: u64, // C bitfield width: 1
    pub qos_mask: u64, // C bitfield width: 8
    pub buf_ptr: u64, // C bitfield width: 36
    pub tail: u64, // C bitfield width: 1
    pub index: u64, // C bitfield width: 3
    pub port: u64, // C bitfield width: 6
    pub queue: u64, // C bitfield width: 7
}

#[repr(C)]
pub union cvmx_pko_mem_queue_qos {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_queue_qos_s: cvmx_pko_mem_queue_qos_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_queue_qos_s {
    pub reserved_61_63: u64, // C bitfield width: 3
    pub qos_mask: u64, // C bitfield width: 8
    pub reserved_13_52: u64, // C bitfield width: 40
    pub pid: u64, // C bitfield width: 6
    pub qid: u64, // C bitfield width: 7
}

#[repr(C)]
pub union cvmx_pko_mem_throttle_int {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_throttle_int_s: cvmx_pko_mem_throttle_int_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_throttle_int_s {
    pub reserved_47_63: u64, // C bitfield width: 17
    pub word: u64, // C bitfield width: 15
    pub reserved_14_31: u64, // C bitfield width: 18
    pub packet: u64, // C bitfield width: 6
    pub reserved_5_7: u64, // C bitfield width: 3
    pub intr: u64, // C bitfield width: 5
}

#[repr(C)]
pub union cvmx_pko_mem_throttle_pipe {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_mem_throttle_pipe_s: cvmx_pko_mem_throttle_pipe_s,
}

#[repr(C)]
pub struct cvmx_pko_mem_throttle_pipe_s {
    pub reserved_47_63: u64, // C bitfield width: 17
    pub word: u64, // C bitfield width: 15
    pub reserved_14_31: u64, // C bitfield width: 18
    pub packet: u64, // C bitfield width: 6
    pub reserved_7_7: u64, // C bitfield width: 1
    pub pipe: u64, // C bitfield width: 7
}

#[repr(C)]
pub union cvmx_pko_reg_bist_result {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_bist_result_s: cvmx_pko_reg_bist_result_s,
    #[repr(C)]
    pub cvmx_pko_reg_bist_result_cn30xx: cvmx_pko_reg_bist_result_cn30xx,
    #[repr(C)]
    pub cvmx_pko_reg_bist_result_cn50xx: cvmx_pko_reg_bist_result_cn50xx,
    #[repr(C)]
    pub cvmx_pko_reg_bist_result_cn52xx: cvmx_pko_reg_bist_result_cn52xx,
    #[repr(C)]
    pub cvmx_pko_reg_bist_result_cn68xx: cvmx_pko_reg_bist_result_cn68xx,
    #[repr(C)]
    pub cvmx_pko_reg_bist_result_cn68xxp1: cvmx_pko_reg_bist_result_cn68xxp1,
}

#[repr(C)]
pub struct cvmx_pko_reg_bist_result_s {
    pub reserved_0_63: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_reg_bist_result_cn30xx {
    pub reserved_27_63: u64, // C bitfield width: 37
    pub psb2: u64, // C bitfield width: 5
    pub count: u64, // C bitfield width: 1
    pub rif: u64, // C bitfield width: 1
    pub wif: u64, // C bitfield width: 1
    pub ncb: u64, // C bitfield width: 1
    pub out: u64, // C bitfield width: 1
    pub crc: u64, // C bitfield width: 1
    pub chk: u64, // C bitfield width: 1
    pub qsb: u64, // C bitfield width: 2
    pub qcb: u64, // C bitfield width: 2
    pub pdb: u64, // C bitfield width: 4
    pub psb: u64, // C bitfield width: 7
}

#[repr(C)]
pub struct cvmx_pko_reg_bist_result_cn50xx {
    pub reserved_33_63: u64, // C bitfield width: 31
    pub csr: u64, // C bitfield width: 1
    pub iob: u64, // C bitfield width: 1
    pub out_crc: u64, // C bitfield width: 1
    pub out_ctl: u64, // C bitfield width: 3
    pub out_sta: u64, // C bitfield width: 1
    pub out_wif: u64, // C bitfield width: 1
    pub prt_chk: u64, // C bitfield width: 3
    pub prt_nxt: u64, // C bitfield width: 1
    pub prt_psb: u64, // C bitfield width: 6
    pub ncb_inb: u64, // C bitfield width: 2
    pub prt_qcb: u64, // C bitfield width: 2
    pub prt_qsb: u64, // C bitfield width: 3
    pub dat_dat: u64, // C bitfield width: 4
    pub dat_ptr: u64, // C bitfield width: 4
}

#[repr(C)]
pub struct cvmx_pko_reg_bist_result_cn52xx {
    pub reserved_35_63: u64, // C bitfield width: 29
    pub csr: u64, // C bitfield width: 1
    pub iob: u64, // C bitfield width: 1
    pub out_dat: u64, // C bitfield width: 1
    pub out_ctl: u64, // C bitfield width: 3
    pub out_sta: u64, // C bitfield width: 1
    pub out_wif: u64, // C bitfield width: 1
    pub prt_chk: u64, // C bitfield width: 3
    pub prt_nxt: u64, // C bitfield width: 1
    pub prt_psb: u64, // C bitfield width: 8
    pub ncb_inb: u64, // C bitfield width: 2
    pub prt_qcb: u64, // C bitfield width: 2
    pub prt_qsb: u64, // C bitfield width: 3
    pub prt_ctl: u64, // C bitfield width: 2
    pub dat_dat: u64, // C bitfield width: 2
    pub dat_ptr: u64, // C bitfield width: 4
}

#[repr(C)]
pub struct cvmx_pko_reg_bist_result_cn68xx {
    pub reserved_36_63: u64, // C bitfield width: 28
    pub crc: u64, // C bitfield width: 1
    pub csr: u64, // C bitfield width: 1
    pub iob: u64, // C bitfield width: 1
    pub out_dat: u64, // C bitfield width: 1
    pub reserved_31_31: u64, // C bitfield width: 1
    pub out_ctl: u64, // C bitfield width: 2
    pub out_sta: u64, // C bitfield width: 1
    pub out_wif: u64, // C bitfield width: 1
    pub prt_chk: u64, // C bitfield width: 3
    pub prt_nxt: u64, // C bitfield width: 1
    pub prt_psb7: u64, // C bitfield width: 1
    pub reserved_21_21: u64, // C bitfield width: 1
    pub prt_psb: u64, // C bitfield width: 6
    pub ncb_inb: u64, // C bitfield width: 2
    pub prt_qcb: u64, // C bitfield width: 2
    pub prt_qsb: u64, // C bitfield width: 3
    pub prt_ctl: u64, // C bitfield width: 2
    pub dat_dat: u64, // C bitfield width: 2
    pub dat_ptr: u64, // C bitfield width: 4
}

#[repr(C)]
pub struct cvmx_pko_reg_bist_result_cn68xxp1 {
    pub reserved_35_63: u64, // C bitfield width: 29
    pub csr: u64, // C bitfield width: 1
    pub iob: u64, // C bitfield width: 1
    pub out_dat: u64, // C bitfield width: 1
    pub reserved_31_31: u64, // C bitfield width: 1
    pub out_ctl: u64, // C bitfield width: 2
    pub out_sta: u64, // C bitfield width: 1
    pub out_wif: u64, // C bitfield width: 1
    pub prt_chk: u64, // C bitfield width: 3
    pub prt_nxt: u64, // C bitfield width: 1
    pub prt_psb7: u64, // C bitfield width: 1
    pub reserved_21_21: u64, // C bitfield width: 1
    pub prt_psb: u64, // C bitfield width: 6
    pub ncb_inb: u64, // C bitfield width: 2
    pub prt_qcb: u64, // C bitfield width: 2
    pub prt_qsb: u64, // C bitfield width: 3
    pub prt_ctl: u64, // C bitfield width: 2
    pub dat_dat: u64, // C bitfield width: 2
    pub dat_ptr: u64, // C bitfield width: 4
}

#[repr(C)]
pub union cvmx_pko_reg_cmd_buf {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_cmd_buf_s: cvmx_pko_reg_cmd_buf_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_cmd_buf_s {
    pub reserved_23_63: u64, // C bitfield width: 41
    pub pool: u64, // C bitfield width: 3
    pub reserved_13_19: u64, // C bitfield width: 7
    pub size: u64, // C bitfield width: 13
}

#[repr(C)]
pub union cvmx_pko_reg_crc_ctlx {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_crc_ctlx_s: cvmx_pko_reg_crc_ctlx_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_crc_ctlx_s {
    pub reserved_2_63: u64, // C bitfield width: 62
    pub invres: u64, // C bitfield width: 1
    pub refin: u64, // C bitfield width: 1
}

#[repr(C)]
pub union cvmx_pko_reg_crc_enable {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_crc_enable_s: cvmx_pko_reg_crc_enable_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_crc_enable_s {
    pub reserved_32_63: u64, // C bitfield width: 32
    pub enable: u64, // C bitfield width: 32
}

#[repr(C)]
pub union cvmx_pko_reg_crc_ivx {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_crc_ivx_s: cvmx_pko_reg_crc_ivx_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_crc_ivx_s {
    pub reserved_32_63: u64, // C bitfield width: 32
    pub iv: u64, // C bitfield width: 32
}

#[repr(C)]
pub union cvmx_pko_reg_debug0 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_debug0_s: cvmx_pko_reg_debug0_s,
    #[repr(C)]
    pub cvmx_pko_reg_debug0_cn30xx: cvmx_pko_reg_debug0_cn30xx,
}

#[repr(C)]
pub struct cvmx_pko_reg_debug0_s {
    pub asserts: u64, // C bitfield width: 64
}

#[repr(C)]
pub struct cvmx_pko_reg_debug0_cn30xx {
    pub reserved_17_63: u64, // C bitfield width: 47
    pub asserts: u64, // C bitfield width: 17
}

#[repr(C)]
pub union cvmx_pko_reg_debug1 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_debug1_s: cvmx_pko_reg_debug1_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_debug1_s {
    pub asserts: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_reg_debug2 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_debug2_s: cvmx_pko_reg_debug2_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_debug2_s {
    pub asserts: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_reg_debug3 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_debug3_s: cvmx_pko_reg_debug3_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_debug3_s {
    pub asserts: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_reg_debug4 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_debug4_s: cvmx_pko_reg_debug4_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_debug4_s {
    pub asserts: u64, // C bitfield width: 64
}

#[repr(C)]
pub union cvmx_pko_reg_engine_inflight {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_engine_inflight_s: cvmx_pko_reg_engine_inflight_s,
    #[repr(C)]
    pub cvmx_pko_reg_engine_inflight_cn52xx: cvmx_pko_reg_engine_inflight_cn52xx,
    #[repr(C)]
    pub cvmx_pko_reg_engine_inflight_cn61xx: cvmx_pko_reg_engine_inflight_cn61xx,
    #[repr(C)]
    pub cvmx_pko_reg_engine_inflight_cn63xx: cvmx_pko_reg_engine_inflight_cn63xx,
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_inflight_s {
    pub engine15: u64, // C bitfield width: 4
    pub engine14: u64, // C bitfield width: 4
    pub engine13: u64, // C bitfield width: 4
    pub engine12: u64, // C bitfield width: 4
    pub engine11: u64, // C bitfield width: 4
    pub engine10: u64, // C bitfield width: 4
    pub engine9: u64, // C bitfield width: 4
    pub engine8: u64, // C bitfield width: 4
    pub engine7: u64, // C bitfield width: 4
    pub engine6: u64, // C bitfield width: 4
    pub engine5: u64, // C bitfield width: 4
    pub engine4: u64, // C bitfield width: 4
    pub engine3: u64, // C bitfield width: 4
    pub engine2: u64, // C bitfield width: 4
    pub engine1: u64, // C bitfield width: 4
    pub engine0: u64, // C bitfield width: 4
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_inflight_cn52xx {
    pub reserved_40_63: u64, // C bitfield width: 24
    pub engine9: u64, // C bitfield width: 4
    pub engine8: u64, // C bitfield width: 4
    pub engine7: u64, // C bitfield width: 4
    pub engine6: u64, // C bitfield width: 4
    pub engine5: u64, // C bitfield width: 4
    pub engine4: u64, // C bitfield width: 4
    pub engine3: u64, // C bitfield width: 4
    pub engine2: u64, // C bitfield width: 4
    pub engine1: u64, // C bitfield width: 4
    pub engine0: u64, // C bitfield width: 4
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_inflight_cn61xx {
    pub reserved_56_63: u64, // C bitfield width: 8
    pub engine13: u64, // C bitfield width: 4
    pub engine12: u64, // C bitfield width: 4
    pub engine11: u64, // C bitfield width: 4
    pub engine10: u64, // C bitfield width: 4
    pub engine9: u64, // C bitfield width: 4
    pub engine8: u64, // C bitfield width: 4
    pub engine7: u64, // C bitfield width: 4
    pub engine6: u64, // C bitfield width: 4
    pub engine5: u64, // C bitfield width: 4
    pub engine4: u64, // C bitfield width: 4
    pub engine3: u64, // C bitfield width: 4
    pub engine2: u64, // C bitfield width: 4
    pub engine1: u64, // C bitfield width: 4
    pub engine0: u64, // C bitfield width: 4
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_inflight_cn63xx {
    pub reserved_48_63: u64, // C bitfield width: 16
    pub engine11: u64, // C bitfield width: 4
    pub engine10: u64, // C bitfield width: 4
    pub engine9: u64, // C bitfield width: 4
    pub engine8: u64, // C bitfield width: 4
    pub engine7: u64, // C bitfield width: 4
    pub engine6: u64, // C bitfield width: 4
    pub engine5: u64, // C bitfield width: 4
    pub engine4: u64, // C bitfield width: 4
    pub engine3: u64, // C bitfield width: 4
    pub engine2: u64, // C bitfield width: 4
    pub engine1: u64, // C bitfield width: 4
    pub engine0: u64, // C bitfield width: 4
}

#[repr(C)]
pub union cvmx_pko_reg_engine_inflight1 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_engine_inflight1_s: cvmx_pko_reg_engine_inflight1_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_inflight1_s {
    pub reserved_16_63: u64, // C bitfield width: 48
    pub engine19: u64, // C bitfield width: 4
    pub engine18: u64, // C bitfield width: 4
    pub engine17: u64, // C bitfield width: 4
    pub engine16: u64, // C bitfield width: 4
}

#[repr(C)]
pub union cvmx_pko_reg_engine_storagex {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_engine_storagex_s: cvmx_pko_reg_engine_storagex_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_storagex_s {
    pub engine15: u64, // C bitfield width: 4
    pub engine14: u64, // C bitfield width: 4
    pub engine13: u64, // C bitfield width: 4
    pub engine12: u64, // C bitfield width: 4
    pub engine11: u64, // C bitfield width: 4
    pub engine10: u64, // C bitfield width: 4
    pub engine9: u64, // C bitfield width: 4
    pub engine8: u64, // C bitfield width: 4
    pub engine7: u64, // C bitfield width: 4
    pub engine6: u64, // C bitfield width: 4
    pub engine5: u64, // C bitfield width: 4
    pub engine4: u64, // C bitfield width: 4
    pub engine3: u64, // C bitfield width: 4
    pub engine2: u64, // C bitfield width: 4
    pub engine1: u64, // C bitfield width: 4
    pub engine0: u64, // C bitfield width: 4
}

#[repr(C)]
pub union cvmx_pko_reg_engine_thresh {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_engine_thresh_s: cvmx_pko_reg_engine_thresh_s,
    #[repr(C)]
    pub cvmx_pko_reg_engine_thresh_cn52xx: cvmx_pko_reg_engine_thresh_cn52xx,
    #[repr(C)]
    pub cvmx_pko_reg_engine_thresh_cn61xx: cvmx_pko_reg_engine_thresh_cn61xx,
    #[repr(C)]
    pub cvmx_pko_reg_engine_thresh_cn63xx: cvmx_pko_reg_engine_thresh_cn63xx,
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_thresh_s {
    pub reserved_20_63: u64, // C bitfield width: 44
    pub mask: u64, // C bitfield width: 20
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_thresh_cn52xx {
    pub reserved_10_63: u64, // C bitfield width: 54
    pub mask: u64, // C bitfield width: 10
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_thresh_cn61xx {
    pub reserved_14_63: u64, // C bitfield width: 50
    pub mask: u64, // C bitfield width: 14
}

#[repr(C)]
pub struct cvmx_pko_reg_engine_thresh_cn63xx {
    pub reserved_12_63: u64, // C bitfield width: 52
    pub mask: u64, // C bitfield width: 12
}

#[repr(C)]
pub union cvmx_pko_reg_error {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_error_s: cvmx_pko_reg_error_s,
    #[repr(C)]
    pub cvmx_pko_reg_error_cn30xx: cvmx_pko_reg_error_cn30xx,
    #[repr(C)]
    pub cvmx_pko_reg_error_cn50xx: cvmx_pko_reg_error_cn50xx,
}

#[repr(C)]
pub struct cvmx_pko_reg_error_s {
    pub reserved_4_63: u64, // C bitfield width: 60
    pub loopback: u64, // C bitfield width: 1
    pub currzero: u64, // C bitfield width: 1
    pub doorbell: u64, // C bitfield width: 1
    pub parity: u64, // C bitfield width: 1
}

#[repr(C)]
pub struct cvmx_pko_reg_error_cn30xx {
    pub reserved_2_63: u64, // C bitfield width: 62
    pub doorbell: u64, // C bitfield width: 1
    pub parity: u64, // C bitfield width: 1
}

#[repr(C)]
pub struct cvmx_pko_reg_error_cn50xx {
    pub reserved_3_63: u64, // C bitfield width: 61
    pub currzero: u64, // C bitfield width: 1
    pub doorbell: u64, // C bitfield width: 1
    pub parity: u64, // C bitfield width: 1
}

#[repr(C)]
pub union cvmx_pko_reg_flags {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_flags_s: cvmx_pko_reg_flags_s,
    #[repr(C)]
    pub cvmx_pko_reg_flags_cn30xx: cvmx_pko_reg_flags_cn30xx,
    #[repr(C)]
    pub cvmx_pko_reg_flags_cn61xx: cvmx_pko_reg_flags_cn61xx,
    #[repr(C)]
    pub cvmx_pko_reg_flags_cn68xxp1: cvmx_pko_reg_flags_cn68xxp1,
}

#[repr(C)]
pub struct cvmx_pko_reg_flags_s {
    pub reserved_9_63: u64, // C bitfield width: 55
    pub dis_perf3: u64, // C bitfield width: 1
    pub dis_perf2: u64, // C bitfield width: 1
    pub dis_perf1: u64, // C bitfield width: 1
    pub dis_perf0: u64, // C bitfield width: 1
    pub ena_throttle: u64, // C bitfield width: 1
    pub reset: u64, // C bitfield width: 1
    pub store_be: u64, // C bitfield width: 1
    pub ena_dwb: u64, // C bitfield width: 1
    pub ena_pko: u64, // C bitfield width: 1
}

#[repr(C)]
pub struct cvmx_pko_reg_flags_cn30xx {
    pub reserved_4_63: u64, // C bitfield width: 60
    pub reset: u64, // C bitfield width: 1
    pub store_be: u64, // C bitfield width: 1
    pub ena_dwb: u64, // C bitfield width: 1
    pub ena_pko: u64, // C bitfield width: 1
}

#[repr(C)]
pub struct cvmx_pko_reg_flags_cn61xx {
    pub reserved_9_63: u64, // C bitfield width: 55
    pub dis_perf3: u64, // C bitfield width: 1
    pub dis_perf2: u64, // C bitfield width: 1
    pub reserved_4_6: u64, // C bitfield width: 3
    pub reset: u64, // C bitfield width: 1
    pub store_be: u64, // C bitfield width: 1
    pub ena_dwb: u64, // C bitfield width: 1
    pub ena_pko: u64, // C bitfield width: 1
}

#[repr(C)]
pub struct cvmx_pko_reg_flags_cn68xxp1 {
    pub reserved_7_63: u64, // C bitfield width: 57
    pub dis_perf1: u64, // C bitfield width: 1
    pub dis_perf0: u64, // C bitfield width: 1
    pub ena_throttle: u64, // C bitfield width: 1
    pub reset: u64, // C bitfield width: 1
    pub store_be: u64, // C bitfield width: 1
    pub ena_dwb: u64, // C bitfield width: 1
    pub ena_pko: u64, // C bitfield width: 1
}

#[repr(C)]
pub union cvmx_pko_reg_gmx_port_mode {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_gmx_port_mode_s: cvmx_pko_reg_gmx_port_mode_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_gmx_port_mode_s {
    pub reserved_6_63: u64, // C bitfield width: 58
    pub mode1: u64, // C bitfield width: 3
    pub mode0: u64, // C bitfield width: 3
}

#[repr(C)]
pub union cvmx_pko_reg_int_mask {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_int_mask_s: cvmx_pko_reg_int_mask_s,
    #[repr(C)]
    pub cvmx_pko_reg_int_mask_cn30xx: cvmx_pko_reg_int_mask_cn30xx,
    #[repr(C)]
    pub cvmx_pko_reg_int_mask_cn50xx: cvmx_pko_reg_int_mask_cn50xx,
}

#[repr(C)]
pub struct cvmx_pko_reg_int_mask_s {
    pub reserved_4_63: u64, // C bitfield width: 60
    pub loopback: u64, // C bitfield width: 1
    pub currzero: u64, // C bitfield width: 1
    pub doorbell: u64, // C bitfield width: 1
    pub parity: u64, // C bitfield width: 1
}

#[repr(C)]
pub struct cvmx_pko_reg_int_mask_cn30xx {
    pub reserved_2_63: u64, // C bitfield width: 62
    pub doorbell: u64, // C bitfield width: 1
    pub parity: u64, // C bitfield width: 1
}

#[repr(C)]
pub struct cvmx_pko_reg_int_mask_cn50xx {
    pub reserved_3_63: u64, // C bitfield width: 61
    pub currzero: u64, // C bitfield width: 1
    pub doorbell: u64, // C bitfield width: 1
    pub parity: u64, // C bitfield width: 1
}

#[repr(C)]
pub union cvmx_pko_reg_loopback_bpid {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_loopback_bpid_s: cvmx_pko_reg_loopback_bpid_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_loopback_bpid_s {
    pub reserved_59_63: u64, // C bitfield width: 5
    pub bpid7: u64, // C bitfield width: 6
    pub reserved_52_52: u64, // C bitfield width: 1
    pub bpid6: u64, // C bitfield width: 6
    pub reserved_45_45: u64, // C bitfield width: 1
    pub bpid5: u64, // C bitfield width: 6
    pub reserved_38_38: u64, // C bitfield width: 1
    pub bpid4: u64, // C bitfield width: 6
    pub reserved_31_31: u64, // C bitfield width: 1
    pub bpid3: u64, // C bitfield width: 6
    pub reserved_24_24: u64, // C bitfield width: 1
    pub bpid2: u64, // C bitfield width: 6
    pub reserved_17_17: u64, // C bitfield width: 1
    pub bpid1: u64, // C bitfield width: 6
    pub reserved_10_10: u64, // C bitfield width: 1
    pub bpid0: u64, // C bitfield width: 6
    pub reserved_0_3: u64, // C bitfield width: 4
}

#[repr(C)]
pub union cvmx_pko_reg_loopback_pkind {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_loopback_pkind_s: cvmx_pko_reg_loopback_pkind_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_loopback_pkind_s {
    pub reserved_59_63: u64, // C bitfield width: 5
    pub pkind7: u64, // C bitfield width: 6
    pub reserved_52_52: u64, // C bitfield width: 1
    pub pkind6: u64, // C bitfield width: 6
    pub reserved_45_45: u64, // C bitfield width: 1
    pub pkind5: u64, // C bitfield width: 6
    pub reserved_38_38: u64, // C bitfield width: 1
    pub pkind4: u64, // C bitfield width: 6
    pub reserved_31_31: u64, // C bitfield width: 1
    pub pkind3: u64, // C bitfield width: 6
    pub reserved_24_24: u64, // C bitfield width: 1
    pub pkind2: u64, // C bitfield width: 6
    pub reserved_17_17: u64, // C bitfield width: 1
    pub pkind1: u64, // C bitfield width: 6
    pub reserved_10_10: u64, // C bitfield width: 1
    pub pkind0: u64, // C bitfield width: 6
    pub num_ports: u64, // C bitfield width: 4
}

#[repr(C)]
pub union cvmx_pko_reg_min_pkt {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_min_pkt_s: cvmx_pko_reg_min_pkt_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_min_pkt_s {
    pub size7: u64, // C bitfield width: 8
    pub size6: u64, // C bitfield width: 8
    pub size5: u64, // C bitfield width: 8
    pub size4: u64, // C bitfield width: 8
    pub size3: u64, // C bitfield width: 8
    pub size2: u64, // C bitfield width: 8
    pub size1: u64, // C bitfield width: 8
    pub size0: u64, // C bitfield width: 8
}

#[repr(C)]
pub union cvmx_pko_reg_preempt {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_preempt_s: cvmx_pko_reg_preempt_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_preempt_s {
    pub reserved_16_63: u64, // C bitfield width: 48
    pub min_size: u64, // C bitfield width: 16
}

#[repr(C)]
pub union cvmx_pko_reg_queue_mode {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_queue_mode_s: cvmx_pko_reg_queue_mode_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_queue_mode_s {
    pub reserved_2_63: u64, // C bitfield width: 62
    pub mode: u64, // C bitfield width: 2
}

#[repr(C)]
pub union cvmx_pko_reg_queue_preempt {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_queue_preempt_s: cvmx_pko_reg_queue_preempt_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_queue_preempt_s {
    pub reserved_2_63: u64, // C bitfield width: 62
    pub preemptee: u64, // C bitfield width: 1
    pub preempter: u64, // C bitfield width: 1
}

#[repr(C)]
pub union cvmx_pko_reg_queue_ptrs1 {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_queue_ptrs1_s: cvmx_pko_reg_queue_ptrs1_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_queue_ptrs1_s {
    pub reserved_2_63: u64, // C bitfield width: 62
    pub idx3: u64, // C bitfield width: 1
    pub qid7: u64, // C bitfield width: 1
}

#[repr(C)]
pub union cvmx_pko_reg_read_idx {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_read_idx_s: cvmx_pko_reg_read_idx_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_read_idx_s {
    pub reserved_16_63: u64, // C bitfield width: 48
    pub inc: u64, // C bitfield width: 8
    pub index: u64, // C bitfield width: 8
}

#[repr(C)]
pub union cvmx_pko_reg_throttle {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_throttle_s: cvmx_pko_reg_throttle_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_throttle_s {
    pub reserved_32_63: u64, // C bitfield width: 32
    pub int_mask: u64, // C bitfield width: 32
}

#[repr(C)]
pub union cvmx_pko_reg_timestamp {
    pub u64: u64,
    #[repr(C)]
    pub cvmx_pko_reg_timestamp_s: cvmx_pko_reg_timestamp_s,
}

#[repr(C)]
pub struct cvmx_pko_reg_timestamp_s {
    pub reserved_4_63: u64, // C bitfield width: 60
    pub wqe_word: u64, // C bitfield width: 4
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
