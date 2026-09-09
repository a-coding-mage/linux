/* Translation of cvmx-pko.h. C dependencies are supplied by the surrounding SDK. */

pub const CVMX_PKO_COMMAND_BUFFER_SIZE_ADJUST: i32 = 1;
pub const CVMX_PKO_MAX_OUTPUT_QUEUES_STATIC: i32 = 256;
/* OCTEON_IS_MODEL-dependent in the original header. */
pub const CVMX_PKO_NUM_OUTPUT_PORTS: i32 = 40;
pub const CVMX_PKO_MEM_QUEUE_PTRS_ILLEGAL_PID: i32 = 63;
pub const CVMX_PKO_QUEUE_STATIC_PRIORITY: i32 = 9;
pub const CVMX_PKO_ILLEGAL_QUEUE: u32 = 0xffff;
pub const CVMX_PKO_MAX_QUEUE_DEPTH: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_pko_status {
    CVMX_PKO_SUCCESS,
    CVMX_PKO_INVALID_PORT,
    CVMX_PKO_INVALID_QUEUE,
    CVMX_PKO_INVALID_PRIORITY,
    CVMX_PKO_NO_MEMORY,
    CVMX_PKO_PORT_ALREADY_SETUP,
    CVMX_PKO_CMD_QUEUE_INIT_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_pko_lock {
    CVMX_PKO_LOCK_NONE = 0,
    CVMX_PKO_LOCK_ATOMIC_TAG = 1,
    CVMX_PKO_LOCK_CMD_QUEUE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pko_port_status {
    pub packets: u32,
    pub octets: u64,
    pub doorbell: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pko_doorbell_address_t {
    pub u64: u64,
    /* C bitfields in s are represented as their source-level values. */
    pub s: cvmx_pko_doorbell_address_s,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_pko_doorbell_address_s {
    pub mem_space: u64, pub reserved: u64, pub is_io: u64, pub did: u64,
    pub reserved2: u64, pub reserved3: u64, pub port: u64, pub queue: u64,
    pub reserved4: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pko_command_word0 {
    pub u64: u64,
    pub s: cvmx_pko_command_word0_s,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_pko_command_word0_s {
    pub size1: u64, pub size0: u64, pub subone1: u64, pub reg1: u64,
    pub subone0: u64, pub reg0: u64, pub le: u64, pub n2: u64, pub wqp: u64,
    pub rsp: u64, pub gather: u64, pub ipoffp1: u64, pub ignore_i: u64,
    pub dontfree: u64, pub segs: u64, pub total_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pko_state_elem_t { pub start_ptr: *mut u64 }

extern "C" {
    pub fn cvmx_pko_initialize_global();
    pub fn cvmx_pko_enable();
    pub fn cvmx_pko_disable();
    pub fn cvmx_pko_shutdown();
    pub fn cvmx_pko_config_port(port: u64, base_queue: u64, num_queues: u64,
                                priority: *const u64) -> cvmx_pko_status;
    pub fn cvmx_pko_rate_limit_packets(port: i32, packets_s: i32, burst: i32) -> i32;
    pub fn cvmx_pko_rate_limit_bits(port: i32, bits_s: u64, burst: i32) -> i32;
}

/* The following inline routines retain the original calls and dependencies. */
#[inline]
pub unsafe fn cvmx_pko_doorbell(port: u64, queue: u64, len: u64) {
    let mut ptr = cvmx_pko_doorbell_address_t { u64: 0, s: Default::default() };
    ptr.s.mem_space = CVMX_IO_SEG as u64;
    ptr.s.did = CVMX_OCT_DID_PKT_SEND as u64;
    ptr.s.is_io = 1;
    ptr.s.port = port;
    ptr.s.queue = queue;
    CVMX_SYNCWS!();
    cvmx_write_io(ptr.u64, len);
}

#[inline]
pub unsafe fn cvmx_pko_send_packet_prepare(port: u64, queue: u64,
                                            use_locking: cvmx_pko_lock) {
    if use_locking == cvmx_pko_lock::CVMX_PKO_LOCK_ATOMIC_TAG {
        let tag = (CVMX_TAG_SW_BITS_INTERNAL << CVMX_TAG_SW_SHIFT)
            | (CVMX_TAG_SUBGROUP_PKO << CVMX_TAG_SUBGROUP_SHIFT)
            | (CVMX_TAG_SUBGROUP_MASK & queue as u32);
        cvmx_pow_tag_sw_full(cvmx_phys_to_ptr(0x80) as *mut cvmx_wqe,
                             tag, CVMX_POW_TAG_TYPE_ATOMIC, 0);
    }
}

#[inline]
pub unsafe fn cvmx_pko_send_packet_finish(
    port: u64, queue: u64, pko_command: cvmx_pko_command_word0,
    packet: cvmx_buf_ptr, use_locking: cvmx_pko_lock) -> cvmx_pko_status {
    if use_locking == cvmx_pko_lock::CVMX_PKO_LOCK_ATOMIC_TAG { cvmx_pow_tag_sw_wait(); }
    let result = cvmx_cmd_queue_write2(cvmx_cmd_queue_pko(queue),
        use_locking == cvmx_pko_lock::CVMX_PKO_LOCK_CMD_QUEUE,
        pko_command.u64, packet.u64);
    if result == CVMX_CMD_QUEUE_SUCCESS { cvmx_pko_doorbell(port, queue, 2); cvmx_pko_status::CVMX_PKO_SUCCESS }
    else if result == CVMX_CMD_QUEUE_NO_MEMORY || result == CVMX_CMD_QUEUE_FULL { cvmx_pko_status::CVMX_PKO_NO_MEMORY }
    else { cvmx_pko_status::CVMX_PKO_INVALID_QUEUE }
}

#[inline]
pub unsafe fn cvmx_pko_send_packet_finish3(
    port: u64, queue: u64, pko_command: cvmx_pko_command_word0,
    packet: cvmx_buf_ptr, addr: u64, use_locking: cvmx_pko_lock) -> cvmx_pko_status {
    if use_locking == cvmx_pko_lock::CVMX_PKO_LOCK_ATOMIC_TAG { cvmx_pow_tag_sw_wait(); }
    let result = cvmx_cmd_queue_write3(cvmx_cmd_queue_pko(queue),
        use_locking == cvmx_pko_lock::CVMX_PKO_LOCK_CMD_QUEUE,
        pko_command.u64, packet.u64, addr);
    if result == CVMX_CMD_QUEUE_SUCCESS { cvmx_pko_doorbell(port, queue, 3); cvmx_pko_status::CVMX_PKO_SUCCESS }
    else if result == CVMX_CMD_QUEUE_NO_MEMORY || result == CVMX_CMD_QUEUE_FULL { cvmx_pko_status::CVMX_PKO_NO_MEMORY }
    else { cvmx_pko_status::CVMX_PKO_INVALID_QUEUE }
}

/* Model- and platform-dependent queue constants/functions remain external dependencies. */
#[inline]
pub unsafe fn cvmx_pko_get_base_queue_per_core(port: i32, core: i32) -> i32 {
    if port < CVMX_PKO_MAX_PORTS_INTERFACE0 { port * CVMX_PKO_QUEUES_PER_PORT_INTERFACE0 + core }
    else if port >= 16 && port < 16 + CVMX_PKO_MAX_PORTS_INTERFACE1 { CVMX_PKO_MAX_PORTS_INTERFACE0 * CVMX_PKO_QUEUES_PER_PORT_INTERFACE0 + (port - 16) * CVMX_PKO_QUEUES_PER_PORT_INTERFACE1 + core }
    else if port >= 32 && port < 36 { CVMX_PKO_MAX_PORTS_INTERFACE0 * CVMX_PKO_QUEUES_PER_PORT_INTERFACE0 + CVMX_PKO_MAX_PORTS_INTERFACE1 * CVMX_PKO_QUEUES_PER_PORT_INTERFACE1 + (port - 32) * CVMX_PKO_QUEUES_PER_PORT_PCI }
    else if port >= 36 && port < 40 { CVMX_PKO_MAX_PORTS_INTERFACE0 * CVMX_PKO_QUEUES_PER_PORT_INTERFACE0 + CVMX_PKO_MAX_PORTS_INTERFACE1 * CVMX_PKO_QUEUES_PER_PORT_INTERFACE1 + 4 * CVMX_PKO_QUEUES_PER_PORT_PCI + (port - 36) * CVMX_PKO_QUEUES_PER_PORT_LOOP }
    else { CVMX_PKO_ILLEGAL_QUEUE as i32 }
}

#[inline] pub unsafe fn cvmx_pko_get_base_queue(port: i32) -> i32 {
    if OCTEON_IS_MODEL(OCTEON_CN68XX) { port } else { cvmx_pko_get_base_queue_per_core(port, 0) }
}
#[inline] pub fn cvmx_pko_get_num_queues(port: i32) -> i32 {
    if port < 16 { CVMX_PKO_QUEUES_PER_PORT_INTERFACE0 } else if port < 32 { CVMX_PKO_QUEUES_PER_PORT_INTERFACE1 } else if port < 36 { CVMX_PKO_QUEUES_PER_PORT_PCI } else if port < 40 { CVMX_PKO_QUEUES_PER_PORT_LOOP } else { 0 }
}

#[inline]
pub unsafe fn cvmx_pko_get_port_status(port_num: u64, clear: u64,
                                        status: *mut cvmx_pko_port_status) {
    let mut idx = cvmx_pko_reg_read_idx { u64: 0 };
    let mut count0 = cvmx_pko_mem_count0 { u64: 0 };
    let mut count1 = cvmx_pko_mem_count1 { u64: 0 };
    idx.s.index = port_num;
    cvmx_write_csr(CVMX_PKO_REG_READ_IDX, idx.u64);
    count0.u64 = cvmx_read_csr(CVMX_PKO_MEM_COUNT0);
    (*status).packets = count0.s.count;
    if clear != 0 { count0.s.count = port_num; cvmx_write_csr(CVMX_PKO_MEM_COUNT0, count0.u64); }
    count1.u64 = cvmx_read_csr(CVMX_PKO_MEM_COUNT1);
    (*status).octets = count1.s.count;
    if clear != 0 { count1.s.count = port_num; cvmx_write_csr(CVMX_PKO_MEM_COUNT1, count1.u64); }
    idx.s.index = cvmx_pko_get_base_queue(port_num as i32) as u64;
    cvmx_write_csr(CVMX_PKO_REG_READ_IDX, idx.u64);
    if OCTEON_IS_MODEL(OCTEON_CN3XXX) {
        let d = cvmx_pko_mem_debug9 { u64: cvmx_read_csr(CVMX_PKO_MEM_DEBUG9) };
        (*status).doorbell = d.cn38xx.doorbell;
    } else {
        let d = cvmx_pko_mem_debug8 { u64: cvmx_read_csr(CVMX_PKO_MEM_DEBUG8) };
        (*status).doorbell = d.cn50xx.doorbell;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
