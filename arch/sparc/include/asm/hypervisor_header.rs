/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of sparc/include/asm/hypervisor.h.
// Hypervisor trap-interface documentation from the C header is preserved here
// as comments; C preprocessor guards and includes are intentionally omitted.

pub const HV_FAST_TRAP: u64 = 0x80;
pub const HV_MMU_MAP_ADDR_TRAP: u64 = 0x83;
pub const HV_MMU_UNMAP_ADDR_TRAP: u64 = 0x84;
pub const HV_TTRACE_ADDENTRY_TRAP: u64 = 0x85;
pub const HV_CORE_TRAP: u64 = 0xff;

pub const HV_EOK: u64 = 0;
pub const HV_ENOCPU: u64 = 1;
pub const HV_ENORADDR: u64 = 2;
pub const HV_ENOINTR: u64 = 3;
pub const HV_EBADPGSZ: u64 = 4;
pub const HV_EBADTSB: u64 = 5;
pub const HV_EINVAL: u64 = 6;
pub const HV_EBADTRAP: u64 = 7;
pub const HV_EBADALIGN: u64 = 8;
pub const HV_EWOULDBLOCK: u64 = 9;
pub const HV_ENOACCESS: u64 = 10;
pub const HV_EIO: u64 = 11;
pub const HV_ECPUERROR: u64 = 12;
pub const HV_ENOTSUPPORTED: u64 = 13;
pub const HV_ENOMAP: u64 = 14;
pub const HV_ETOOMANY: u64 = 15;
pub const HV_ECHANNEL: u64 = 16;
pub const HV_EBUSY: u64 = 17;
pub const HV_EUNAVAILABLE: u64 = 23;

pub const HV_FAST_MACH_EXIT: u64 = 0x00;
pub const HV_FAST_MACH_DESC: u64 = 0x01;
pub const HV_FAST_MACH_SIR: u64 = 0x02;
pub const HV_FAST_MACH_SET_WATCHDOG: u64 = 0x05;
pub const HV_FAST_CPU_START: u64 = 0x10;
pub const HV_FAST_CPU_STOP: u64 = 0x11;
pub const HV_FAST_CPU_YIELD: u64 = 0x12;
pub const HV_FAST_CPU_POKE: u64 = 0x13;
pub const HV_FAST_CPU_QCONF: u64 = 0x14;
pub const HV_FAST_CPU_QINFO: u64 = 0x15;
pub const HV_FAST_CPU_MYID: u64 = 0x16;
pub const HV_FAST_CPU_STATE: u64 = 0x17;
pub const HV_FAST_CPU_SET_RTBA: u64 = 0x18;
pub const HV_FAST_CPU_GET_RTBA: u64 = 0x19;
pub const HV_CPU_QUEUE_CPU_MONDO: u64 = 0x3c;
pub const HV_CPU_QUEUE_DEVICE_MONDO: u64 = 0x3d;
pub const HV_CPU_QUEUE_RES_ERROR: u64 = 0x3e;
pub const HV_CPU_QUEUE_NONRES_ERROR: u64 = 0x3f;
pub const HV_CPU_STATE_STOPPED: u64 = 1;
pub const HV_CPU_STATE_RUNNING: u64 = 2;
pub const HV_CPU_STATE_ERROR: u64 = 3;

#[repr(C)]
pub struct hv_tsb_descr {
    pub pgsz_idx: u16, pub assoc: u16, pub num_ttes: u32, pub ctx_idx: u32,
    pub pgsz_mask: u32, pub tsb_base: usize, pub resv: usize,
}

pub const HV_PGSZ_MASK_8K: u64 = 1 << 0;
pub const HV_PGSZ_MASK_64K: u64 = 1 << 1;
pub const HV_PGSZ_MASK_512K: u64 = 1 << 2;
pub const HV_PGSZ_MASK_4MB: u64 = 1 << 3;
pub const HV_PGSZ_MASK_32MB: u64 = 1 << 4;
pub const HV_PGSZ_MASK_256MB: u64 = 1 << 5;
pub const HV_PGSZ_MASK_2GB: u64 = 1 << 6;
pub const HV_PGSZ_MASK_16GB: u64 = 1 << 7;
pub const HV_PGSZ_IDX_8K: u64 = 0; pub const HV_PGSZ_IDX_64K: u64 = 1;
pub const HV_PGSZ_IDX_512K: u64 = 2; pub const HV_PGSZ_IDX_4MB: u64 = 3;
pub const HV_PGSZ_IDX_32MB: u64 = 4; pub const HV_PGSZ_IDX_256MB: u64 = 5;
pub const HV_PGSZ_IDX_2GB: u64 = 6; pub const HV_PGSZ_IDX_16GB: u64 = 7;

#[repr(C)]
pub struct hv_fault_status {
    pub i_fault_type: usize, pub i_fault_addr: usize, pub i_fault_ctx: usize,
    pub i_reserved: [usize; 5], pub d_fault_type: usize, pub d_fault_addr: usize,
    pub d_fault_ctx: usize, pub d_reserved: [usize; 5],
}

pub const HV_MMU_DMMU: u64 = 0x01;
pub const HV_MMU_IMMU: u64 = 0x02;
pub const HV_MMU_ALL: u64 = HV_MMU_DMMU | HV_MMU_IMMU;

pub const HV_FAST_MMU_TSB_CTX0: u64 = 0x20;
pub const HV_FAST_MMU_TSB_CTXNON0: u64 = 0x21;
pub const HV_FAST_MMU_DEMAP_PAGE: u64 = 0x22;
pub const HV_FAST_MMU_DEMAP_CTX: u64 = 0x23;
pub const HV_FAST_MMU_DEMAP_ALL: u64 = 0x24;
pub const HV_FAST_MMU_MAP_PERM_ADDR: u64 = 0x25;
pub const HV_FAST_MMU_FAULT_AREA_CONF: u64 = 0x26;
pub const HV_FAST_MMU_ENABLE: u64 = 0x27;
pub const HV_FAST_MMU_UNMAP_PERM_ADDR: u64 = 0x28;
pub const HV_FAST_MMU_TSB_CTX0_INFO: u64 = 0x29;
pub const HV_FAST_MMU_TSB_CTXNON0_INFO: u64 = 0x2a;
pub const HV_FAST_MMU_FAULT_AREA_INFO: u64 = 0x2b;

pub const HV_FAST_MEM_SCRUB: u64 = 0x31;
pub const HV_FAST_MEM_SYNC: u64 = 0x32;
pub const HV_CCB_SUBMIT: u64 = 0x34;
pub const HV_CCB_INFO: u64 = 0x35;
pub const HV_CCB_KILL: u64 = 0x36;
pub const HV_FAST_TOD_GET: u64 = 0x50;
pub const HV_FAST_TOD_SET: u64 = 0x51;
pub const HV_FAST_CONS_GETCHAR: u64 = 0x60;
pub const HV_FAST_CONS_PUTCHAR: u64 = 0x61;
pub const HV_FAST_CONS_READ: u64 = 0x62;
pub const HV_FAST_CONS_WRITE: u64 = 0x63;
pub const HV_FAST_MACH_SET_SOFT_STATE: u64 = 0x70;
pub const HV_FAST_MACH_GET_SOFT_STATE: u64 = 0x71;
pub const HV_FAST_SVC_SEND: u64 = 0x80;
pub const HV_FAST_SVC_RECV: u64 = 0x81;
pub const HV_FAST_SVC_GETSTATUS: u64 = 0x82;
pub const HV_FAST_SVC_SETSTATUS: u64 = 0x83;
pub const HV_FAST_SVC_CLRSTATUS: u64 = 0x84;

#[repr(C)]
pub struct hv_trap_trace_control { pub head_offset: usize, pub tail_offset: usize, pub __reserved: [usize; 6] }
#[repr(C)]
pub struct hv_trap_trace_entry {
    pub type_: u8, pub hpstate: u8, pub tl: u8, pub gl: u8, pub tt: u16, pub tag: u16,
    pub tstate: usize, pub tick: usize, pub tpc: usize, pub f1: usize, pub f2: usize,
    pub f3: usize, pub f4: usize,
}

pub const HV_INTR_STATE_IDLE: u64 = 0;
pub const HV_INTR_STATE_RECEIVED: u64 = 1;
pub const HV_INTR_STATE_DELIVERED: u64 = 2;
pub const HV_INTR_DISABLED: u64 = 0;
pub const HV_INTR_ENABLED: u64 = 1;

pub const HV_PCI_MAP_ATTR_READ: u64 = 0x01;
pub const HV_PCI_MAP_ATTR_WRITE: u64 = 0x02;
pub const HV_PCI_MAP_ATTR_RELAXED_ORDER: u64 = 0x04;
#[inline] pub const fn HV_PCI_DEVICE_BUILD(b: u64, d: u64, f: u64) -> u64 { ((b & 0xff) << 16) | ((d & 0x1f) << 11) | ((f & 0x07) << 8) }
#[inline] pub const fn HV_PCI_TSBID(tsb_num: u64, tsb_index: u64) -> u64 { (tsb_num << 32) | tsb_index }
#[inline] pub const fn HV_PCI_IOTSB_INDEX_COUNT(iottes: u64, iotsb_index: u64) -> u64 { (iottes << 48) | iotsb_index }

extern "C" {
    pub fn sun4v_mach_exit(exit_code: usize) -> !;
    pub fn sun4v_mach_desc(buffer_pa: usize, buf_len: usize, real_buf_len: *mut usize) -> usize;
    pub fn sun4v_mach_sir() -> !;
    pub fn sun4v_mach_set_watchdog(timeout: usize, orig_timeout: *mut usize) -> usize;
    pub fn sun4v_cpu_start(cpuid: usize, pc: usize, rtba: usize, arg0: usize) -> usize;
    pub fn sun4v_cpu_stop(cpuid: usize) -> usize;
    pub fn sun4v_cpu_yield() -> usize;
    pub fn sun4v_cpu_poke(cpuid: usize) -> usize;
    pub fn sun4v_cpu_qconf(kind: usize, queue_paddr: usize, num_queue_entries: usize) -> usize;
    pub fn sun4v_cpu_mondo_send(cpu_count: usize, cpu_list_pa: usize, mondo_block_pa: usize) -> usize;
    pub fn sun4v_cpu_state(cpuid: usize) -> isize;
    pub fn sun4v_mmu_demap_all() -> !;
    pub fn sun4v_mmu_tsb_ctx0(num_descriptions: usize, tsb_desc_ra: usize) -> usize;
    pub fn sun4v_mmu_map_perm_addr(vaddr: usize, set_to_zero: usize, tte: usize, flags: usize) -> usize;
    pub fn sun4v_ccb_submit(ccb_buf: usize, len: usize, flags: usize, reserved: usize, submitted_len: *mut core::ffi::c_void, status_data: *mut core::ffi::c_void) -> usize;
    pub fn sun4v_ccb_info(ca: usize, info_arr: *mut core::ffi::c_void) -> usize;
    pub fn sun4v_ccb_kill(ca: usize, kill_status: *mut core::ffi::c_void) -> usize;
    pub fn sun4v_tod_get(time: *mut usize) -> usize;
    pub fn sun4v_tod_set(time: usize) -> usize;
    pub fn sun4v_con_getchar(status: *mut isize) -> isize;
    pub fn sun4v_con_putchar(c: isize) -> isize;
    pub fn sun4v_con_read(buffer: usize, size: usize, bytes_read: *mut usize) -> isize;
    pub fn sun4v_con_write(buffer: usize, size: usize, bytes_written: *mut usize) -> usize;
    pub fn sun4v_mach_set_soft_state(soft_state: usize, msg_string_ra: usize) -> usize;
    pub fn sun4v_svc_send(svc_id: usize, buffer: usize, buffer_size: usize, sent_bytes: *mut usize) -> usize;
    pub fn sun4v_svc_recv(svc_id: usize, buffer: usize, buffer_size: usize, recv_bytes: *mut usize) -> usize;
    pub fn sun4v_svc_getstatus(svc_id: usize, status_bits: *mut usize) -> usize;
    pub fn sun4v_svc_setstatus(svc_id: usize, status_bits: usize) -> usize;
    pub fn sun4v_svc_clrstatus(svc_id: usize, status_bits: usize) -> usize;
    pub fn sun4v_devino_to_sysino(devhandle: usize, devino: usize) -> usize;
    pub fn sun4v_intr_getenabled(sysino: usize) -> usize;
    pub fn sun4v_intr_setenabled(sysino: usize, intr_enabled: usize) -> usize;
    pub fn sun4v_intr_getstate(sysino: usize) -> usize;
    pub fn sun4v_intr_setstate(sysino: usize, intr_state: usize) -> usize;
    pub fn sun4v_intr_gettarget(sysino: usize) -> usize;
    pub fn sun4v_intr_settarget(sysino: usize, cpuid: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
