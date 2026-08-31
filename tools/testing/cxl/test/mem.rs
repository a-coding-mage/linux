// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2021 Intel Corporation. All rights reserved.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = ::core::ffi::c_uchar;
type u16 = ::core::ffi::c_ushort;
type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;
type ssize_t = isize;
type size_t = usize;
type __le16 = u16;
type __le32 = u32;
type __le64 = u64;
type bool_t = bool;

const SZ_4K: usize = 4 * 1024;
const SZ_128K: usize = 128 * 1024;
const SZ_256M: u64 = 256 * 1024 * 1024;
const SZ_64M: usize = 64 * 1024 * 1024;
const SZ_2G: u64 = 2 * 1024 * 1024 * 1024;

const LSA_SIZE: usize = SZ_128K;
const FW_SIZE: usize = SZ_64M;
const FW_SLOTS: c_int = 3;
const DEV_SIZE: u64 = SZ_2G;
const fn EFFECT(x: u32) -> u32 {
    1u32 << x
}

const MOCK_INJECT_DEV_MAX: u32 = 8;
const MOCK_INJECT_TEST_MAX: usize = 128;

static mut poison_inject_dev_max: u32 = MOCK_INJECT_DEV_MAX;

#[repr(u32)]
enum cxl_command_effects {
    CONF_CHANGE_COLD_RESET = 0,
    CONF_CHANGE_IMMEDIATE,
    DATA_CHANGE_IMMEDIATE,
    POLICY_CHANGE_IMMEDIATE,
    LOG_CHANGE_IMMEDIATE,
    SECURITY_CHANGE_IMMEDIATE,
    BACKGROUND_OP,
    SECONDARY_MBOX_SUPPORTED,
}

const fn cpu_to_le16(v: u16) -> __le16 {
    v.to_le()
}
const fn cpu_to_le32(v: u32) -> __le32 {
    v.to_le()
}
const fn cpu_to_le64(v: u64) -> __le64 {
    v.to_le()
}
const fn le16_to_cpu(v: __le16) -> u16 {
    u16::from_le(v)
}
const fn le32_to_cpu(v: __le32) -> u32 {
    u32::from_le(v)
}
const fn le64_to_cpu(v: __le64) -> u64 {
    u64::from_le(v)
}

const CXL_CMD_EFFECT_NONE: __le16 = cpu_to_le16(0);

// External kernel/CXL constants, types, globals, helpers, and attribute macros are
// provided by the surrounding repository translation.
extern "C" {
    static CXL_VENDOR_FEATURE_TEST: uuid_t;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vzalloc(size: size_t) -> *mut c_void;
    fn vfree(addr: *mut c_void);
    fn kfree(addr: *mut c_void);
    fn ktime_get_real_ns() -> u64;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn msecs_to_jiffies(m: c_ulong) -> c_ulong;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn sysfs_notify_dirent(sd: *mut kernfs_node);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn kstrtoint(s: *const c_char, base: c_uint, res: *mut c_int) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn sha256(data: *const c_void, len: size_t, out: *mut u8);
    fn platform_get_device_id(pdev: *mut platform_device) -> *const platform_device_id;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn cxl_mailbox_init(cxl_mbox: *mut cxl_mailbox, dev: *mut device) -> c_int;
    fn cxl_memdev_state_create(dev: *mut device, serial: u64, id: c_int) -> *mut cxl_memdev_state;
    fn cxl_enumerate_cmds(mds: *mut cxl_memdev_state) -> c_int;
    fn cxl_poison_state_init(mds: *mut cxl_memdev_state) -> c_int;
    fn cxl_set_timestamp(mds: *mut cxl_memdev_state) -> c_int;
    fn cxl_dev_state_identify(mds: *mut cxl_memdev_state) -> c_int;
    fn cxl_mem_dpa_fetch(mds: *mut cxl_memdev_state, info: *mut cxl_dpa_info) -> c_int;
    fn cxl_dpa_setup(cxlds: *mut cxl_dev_state, info: *mut cxl_dpa_info) -> c_int;
    fn devm_cxl_setup_features(cxlds: *mut cxl_dev_state) -> c_int;
    fn devm_cxl_add_classdev(cxlds: *mut cxl_dev_state) -> *mut cxl_memdev;
    fn devm_cxl_setup_fw_upload(dev: *mut device, mds: *mut cxl_memdev_state) -> c_int;
    fn devm_cxl_sanitize_setup_notifier(dev: *mut device, cxlmd: *mut cxl_memdev) -> c_int;
    fn devm_cxl_setup_fwctl(dev: *mut device, cxlmd: *mut cxl_memdev) -> c_int;
    fn cxl_mem_get_event_records(mds: *mut cxl_memdev_state, status: u32);
    fn uuid_equal(a: *const uuid_t, b: *const uuid_t) -> bool;
    fn atomic_inc_return(v: *mut atomic_t) -> c_int;
}

#[repr(C)] struct device { _priv: [u8; 0] }
#[repr(C)] struct device_driver { _priv: [u8; 0] }
#[repr(C)] struct device_attribute { _priv: [u8; 0] }
#[repr(C)] struct attribute { _priv: [u8; 0] }
#[repr(C)] struct kernfs_node { _priv: [u8; 0] }
#[repr(C)] struct mutex { _priv: [u8; 0] }
#[repr(C)] struct work_struct { _priv: [u8; 0] }
#[repr(C)] struct delayed_work { work: work_struct }
#[repr(C)] struct cxl_memdev { _priv: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
struct uuid_t {
    b: [u8; 16],
}

#[repr(C)]
struct atomic_t {
    counter: c_int,
}

const fn ATOMIC_INIT(v: c_int) -> atomic_t {
    atomic_t { counter: v }
}

#[repr(C)]
struct cxl_cel_entry {
    opcode: __le16,
    effect: __le16,
}

#[repr(C)]
struct cxl_mbox_get_supported_logs {
    entries: __le16,
}

#[repr(C)]
struct cxl_gsl_entry {
    uuid: uuid_t,
    size: __le32,
}

#[repr(C)]
struct mock_gsl_payload_t {
    gsl: cxl_mbox_get_supported_logs,
    entry: cxl_gsl_entry,
}

#[repr(C, packed)]
struct cxl_mbox_health_info {
    health_status: u8,
    media_status: u8,
    ext_status: u8,
    life_used: u8,
    temperature: __le16,
    dirty_shutdowns: __le32,
    volatile_errors: __le32,
    pmem_errors: __le32,
}

const PASS_TRY_LIMIT: c_int = 3;
const CXL_TEST_EVENT_CNT_MAX: usize = 15;
const CXL_TEST_EVENT_RET_MAX: c_int = 4;

#[repr(C)]
struct mock_event_log {
    clear_idx: u16,
    cur_idx: u16,
    nr_events: u16,
    nr_overflow: u16,
    overflow_reset: u16,
    events: [*mut cxl_event_record_raw; CXL_TEST_EVENT_CNT_MAX],
}

#[repr(C)]
struct mock_event_store {
    mock_logs: [mock_event_log; CXL_EVENT_TYPE_MAX as usize],
    ev_status: u32,
}

#[repr(C, packed)]
struct vendor_test_feat {
    data: __le32,
}

#[repr(C)]
struct cxl_mockmem_data {
    lsa: *mut c_void,
    fw: *mut c_void,
    fw_slot: c_int,
    fw_staged: c_int,
    fw_size: size_t,
    security_state: u32,
    user_pass: [u8; NVDIMM_PASSPHRASE_LEN],
    master_pass: [u8; NVDIMM_PASSPHRASE_LEN],
    user_limit: c_int,
    master_limit: c_int,
    mes: mock_event_store,
    mds: *mut cxl_memdev_state,
    event_buf: [u8; SZ_4K],
    timestamp: u64,
    sanitize_timeout: c_ulong,
    test_feat: vendor_test_feat,
    shutdown_state: u8,
}

unsafe fn struct_size<T>(base: usize, elem: usize, count: usize) -> usize {
    base + elem * count
}

unsafe fn put_unaligned_le16(val: u16, p: *mut __le16) {
    ptr::write_unaligned(p, cpu_to_le16(val));
}

unsafe fn put_unaligned_le24(val: u32, p: *mut u8) {
    ptr::write_unaligned(p, (val & 0xff) as u8);
    ptr::write_unaligned(p.add(1), ((val >> 8) & 0xff) as u8);
    ptr::write_unaligned(p.add(2), ((val >> 16) & 0xff) as u8);
}

unsafe fn FIELD_GET(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> mask.trailing_zeros()
}

static mut mock_cel: [cxl_cel_entry; 17] = [
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_SUPPORTED_LOGS as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_SUPPORTED_FEATURES as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_FEATURE as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_SET_FEATURE as u16), effect: cpu_to_le16(EFFECT(cxl_command_effects::CONF_CHANGE_IMMEDIATE as u32) as u16) },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_IDENTIFY as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_LSA as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_PARTITION_INFO as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_SET_LSA as u16), effect: cpu_to_le16((EFFECT(cxl_command_effects::CONF_CHANGE_IMMEDIATE as u32) | EFFECT(cxl_command_effects::DATA_CHANGE_IMMEDIATE as u32)) as u16) },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_HEALTH_INFO as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_SET_SHUTDOWN_STATE as u16), effect: cxl_command_effects::POLICY_CHANGE_IMMEDIATE as u16 },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_POISON as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_INJECT_POISON as u16), effect: cpu_to_le16(EFFECT(cxl_command_effects::DATA_CHANGE_IMMEDIATE as u32) as u16) },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_CLEAR_POISON as u16), effect: cpu_to_le16(EFFECT(cxl_command_effects::DATA_CHANGE_IMMEDIATE as u32) as u16) },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_GET_FW_INFO as u16), effect: CXL_CMD_EFFECT_NONE },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_TRANSFER_FW as u16), effect: cpu_to_le16((EFFECT(cxl_command_effects::CONF_CHANGE_COLD_RESET as u32) | EFFECT(cxl_command_effects::BACKGROUND_OP as u32)) as u16) },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_ACTIVATE_FW as u16), effect: cpu_to_le16((EFFECT(cxl_command_effects::CONF_CHANGE_COLD_RESET as u32) | EFFECT(cxl_command_effects::CONF_CHANGE_IMMEDIATE as u32)) as u16) },
    cxl_cel_entry { opcode: cpu_to_le16(CXL_MBOX_OP_SANITIZE as u16), effect: cpu_to_le16((EFFECT(cxl_command_effects::DATA_CHANGE_IMMEDIATE as u32) | EFFECT(cxl_command_effects::SECURITY_CHANGE_IMMEDIATE as u32) | EFFECT(cxl_command_effects::BACKGROUND_OP as u32)) as u16) },
];

static mut mock_gsl_payload: mock_gsl_payload_t = mock_gsl_payload_t {
    gsl: cxl_mbox_get_supported_logs { entries: cpu_to_le16(1) },
    entry: cxl_gsl_entry { uuid: DEFINE_CXL_CEL_UUID, size: cpu_to_le32(size_of::<[cxl_cel_entry; 17]>() as u32) },
};

static mut event_counter: atomic_t = ATOMIC_INIT(0);

unsafe fn event_find_log(dev: *mut device, log_type: c_int) -> *mut mock_event_log {
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    if log_type >= CXL_EVENT_TYPE_MAX {
        return ptr::null_mut();
    }
    &mut (*mdata).mes.mock_logs[log_type as usize]
}

unsafe fn event_get_current(log: *mut mock_event_log) -> *mut cxl_event_record_raw {
    (*log).events[(*log).cur_idx as usize]
}

unsafe fn event_reset_log(log: *mut mock_event_log) {
    (*log).cur_idx = 0;
    (*log).clear_idx = 0;
    (*log).nr_overflow = (*log).overflow_reset;
}

// Handle can never be 0 use 1 based indexing for handle
unsafe fn event_get_clear_handle(log: *mut mock_event_log) -> u16 {
    (*log).clear_idx + 1
}

// Handle can never be 0 use 1 based indexing for handle
unsafe fn event_get_cur_event_handle(log: *mut mock_event_log) -> __le16 {
    let cur_handle = (*log).cur_idx + 1;
    cpu_to_le16(cur_handle)
}

unsafe fn event_log_empty(log: *mut mock_event_log) -> bool {
    (*log).cur_idx == (*log).nr_events
}

unsafe fn mes_add_event(mes: *mut mock_event_store, log_type: c_int, event: *mut cxl_event_record_raw) {
    if log_type >= CXL_EVENT_TYPE_MAX {
        return;
    }
    let log = &mut (*mes).mock_logs[log_type as usize] as *mut mock_event_log;
    if ((*log).nr_events + 1) as usize > CXL_TEST_EVENT_CNT_MAX {
        (*log).nr_overflow += 1;
        (*log).overflow_reset = (*log).nr_overflow;
        return;
    }
    (*log).events[(*log).nr_events as usize] = event;
    (*log).nr_events += 1;
}

unsafe extern "C" fn mock_get_event(dev: *mut device, cmd: *mut cxl_mbox_cmd) -> c_int {
    let mut ret_limit: c_int;
    let log_type: u8;
    let mut i: c_int;
    if (*cmd).size_in != size_of::<u8>() {
        return -EINVAL;
    }
    ret_limit = (atomic_inc_return(&mut event_counter) % CXL_TEST_EVENT_RET_MAX) + 1;
    if (*cmd).size_out < struct_size(size_of::<cxl_get_event_payload>(), size_of::<cxl_event_record_raw>(), ret_limit as usize) {
        return -EINVAL;
    }
    log_type = *((*cmd).payload_in as *mut u8);
    if log_type as c_int >= CXL_EVENT_TYPE_MAX {
        return -EINVAL;
    }
    memset((*cmd).payload_out, 0, struct_size(size_of::<cxl_get_event_payload>(), size_of::<cxl_event_record_raw>(), 0));
    let log = event_find_log(dev, log_type as c_int);
    if log.is_null() || event_log_empty(log) {
        return 0;
    }
    let pl = (*cmd).payload_out as *mut cxl_get_event_payload;
    i = 0;
    while i < ret_limit && !event_log_empty(log) {
        memcpy((*pl).records.as_mut_ptr().add(i as usize) as *mut c_void, event_get_current(log) as *const c_void, size_of::<cxl_event_record_raw>());
        (*(*pl).records.as_mut_ptr().add(i as usize)).event.generic.hdr.handle = event_get_cur_event_handle(log);
        (*log).cur_idx += 1;
        i += 1;
    }
    (*cmd).size_out = struct_size(size_of::<cxl_get_event_payload>(), size_of::<cxl_event_record_raw>(), i as usize);
    (*pl).record_count = cpu_to_le16(i as u16);
    if !event_log_empty(log) {
        (*pl).flags |= CXL_GET_EVENT_FLAG_MORE_RECORDS;
    }
    if (*log).nr_overflow != 0 {
        (*pl).flags |= CXL_GET_EVENT_FLAG_OVERFLOW;
        (*pl).overflow_err_count = cpu_to_le16((*log).nr_overflow);
        let mut ns = ktime_get_real_ns();
        ns -= 5000000000;
        (*pl).first_overflow_timestamp = cpu_to_le64(ns);
        ns = ktime_get_real_ns();
        ns -= 1000000000;
        (*pl).last_overflow_timestamp = cpu_to_le64(ns);
    }
    0
}

unsafe extern "C" fn mock_clear_event(dev: *mut device, cmd: *mut cxl_mbox_cmd) -> c_int {
    let pl: *mut cxl_mbox_clear_event_payload;
    let log: *mut mock_event_log;
    let log_type: u8;
    let mut handle: u16;
    let mut nr: c_int;
    if (*cmd).size_in < size_of::<cxl_mbox_clear_event_payload>() {
        return -EINVAL;
    }
    pl = (*cmd).payload_in as *mut cxl_mbox_clear_event_payload;
    log_type = (*pl).event_log;
    if log_type as c_int >= CXL_EVENT_TYPE_MAX {
        return -EINVAL;
    }
    log = event_find_log(dev, log_type as c_int);
    if log.is_null() {
        return 0;
    }
    if (*log).clear_idx + (*pl).nr_recs > (*log).cur_idx {
        dev_err(dev, c"Attempting to clear more events than returned!\n".as_ptr());
        return -EINVAL;
    }
    nr = 0;
    handle = event_get_clear_handle(log);
    while nr < (*pl).nr_recs as c_int {
        if handle != le16_to_cpu((*pl).handles[nr as usize]) {
            dev_err(dev, c"Clearing events out of order\n".as_ptr());
            return -EINVAL;
        }
        nr += 1;
        handle += 1;
    }
    if (*log).nr_overflow != 0 {
        (*log).nr_overflow = 0;
    }
    (*log).clear_idx += (*pl).nr_recs;
    0
}

unsafe fn cxl_mock_event_trigger(dev: *mut device) {
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    let mes = &mut (*mdata).mes as *mut mock_event_store;
    let mut i = CXL_EVENT_TYPE_INFO;
    while i < CXL_EVENT_TYPE_MAX {
        let log = event_find_log(dev, i);
        if !log.is_null() {
            event_reset_log(log);
        }
        i += 1;
    }
    cxl_mem_get_event_records((*mdata).mds, (*mes).ev_status);
}

static mut maint_needed: cxl_event_record_raw = cxl_event_record_raw {
    id: UUID_INIT(0xBA5EBA11, 0xABCD, 0xEFEB, 0xa5, 0x5a, 0xa5, 0x5a, 0xa5, 0xa5, 0x5a, 0xa5),
    event: cxl_event_record_raw_event { generic: cxl_event_generic { hdr: cxl_event_record_hdr { length: size_of::<cxl_event_record_raw>() as u8, flags: [CXL_EVENT_RECORD_FLAG_MAINT_NEEDED, 0, 0], handle: 0, related_handle: cpu_to_le16(0xa5b6) }, data: [0xDE, 0xAD, 0xBE, 0xEF] } },
};

static mut hardware_replace: cxl_event_record_raw = cxl_event_record_raw {
    id: UUID_INIT(0xABCDEFEB, 0xBA11, 0xBA5E, 0xa5, 0x5a, 0xa5, 0x5a, 0xa5, 0xa5, 0x5a, 0xa5),
    event: cxl_event_record_raw_event { generic: cxl_event_generic { hdr: cxl_event_record_hdr { length: size_of::<cxl_event_record_raw>() as u8, flags: [CXL_EVENT_RECORD_FLAG_HW_REPLACE, 0, 0], handle: 0, related_handle: cpu_to_le16(0xb6a5) }, data: [0xDE, 0xAD, 0xBE, 0xEF] } },
};

#[repr(C, packed)]
struct cxl_test_gen_media { id: uuid_t, rec: cxl_event_gen_media }
#[repr(C, packed)]
struct cxl_test_dram { id: uuid_t, rec: cxl_event_dram }
#[repr(C, packed)]
struct cxl_test_mem_module { id: uuid_t, rec: cxl_event_mem_module }

static mut gen_media: cxl_test_gen_media = cxl_test_gen_media { id: CXL_EVENT_GEN_MEDIA_UUID, rec: CXL_EVENT_GEN_MEDIA_INIT };
static mut dram: cxl_test_dram = cxl_test_dram { id: CXL_EVENT_DRAM_UUID, rec: CXL_EVENT_DRAM_INIT };
static mut mem_module: cxl_test_mem_module = cxl_test_mem_module { id: CXL_EVENT_MEM_MODULE_UUID, rec: CXL_EVENT_MEM_MODULE_INIT };

unsafe extern "C" fn mock_set_timestamp(cxlds: *mut cxl_dev_state, cmd: *mut cxl_mbox_cmd) -> c_int {
    let mdata = dev_get_drvdata((*cxlds).dev) as *mut cxl_mockmem_data;
    let ts = (*cmd).payload_in as *mut cxl_mbox_set_timestamp_in;
    if (*cmd).size_in != size_of::<cxl_mbox_set_timestamp_in>() { return -EINVAL; }
    if (*cmd).size_out != 0 { return -EINVAL; }
    (*mdata).timestamp = le64_to_cpu((*ts).timestamp);
    0
}

unsafe fn cxl_mock_add_event_logs(mes: *mut mock_event_store) {
    put_unaligned_le16((CXL_GMER_VALID_CHANNEL | CXL_GMER_VALID_RANK | CXL_GMER_VALID_COMPONENT | CXL_GMER_VALID_COMPONENT_ID_FORMAT) as u16, &mut gen_media.rec.media_hdr.validity_flags);
    put_unaligned_le16((CXL_DER_VALID_CHANNEL | CXL_DER_VALID_BANK_GROUP | CXL_DER_VALID_BANK | CXL_DER_VALID_COLUMN | CXL_DER_VALID_SUB_CHANNEL | CXL_DER_VALID_COMPONENT | CXL_DER_VALID_COMPONENT_ID_FORMAT) as u16, &mut dram.rec.media_hdr.validity_flags);
    put_unaligned_le16((CXL_MMER_VALID_COMPONENT | CXL_MMER_VALID_COMPONENT_ID_FORMAT) as u16, &mut mem_module.rec.validity_flags);
    mes_add_event(mes, CXL_EVENT_TYPE_INFO, &mut maint_needed);
    mes_add_event(mes, CXL_EVENT_TYPE_INFO, &mut gen_media as *mut _ as *mut cxl_event_record_raw);
    mes_add_event(mes, CXL_EVENT_TYPE_INFO, &mut mem_module as *mut _ as *mut cxl_event_record_raw);
    (*mes).ev_status |= CXLDEV_EVENT_STATUS_INFO;
    mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut maint_needed);
    mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut hardware_replace);
    mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut dram as *mut _ as *mut cxl_event_record_raw);
    mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut gen_media as *mut _ as *mut cxl_event_record_raw);
    mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut mem_module as *mut _ as *mut cxl_event_record_raw);
    mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut hardware_replace);
    mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut dram as *mut _ as *mut cxl_event_record_raw);
    // Overflow this log
    for _ in 0..10 { mes_add_event(mes, CXL_EVENT_TYPE_FAIL, &mut hardware_replace); }
    (*mes).ev_status |= CXLDEV_EVENT_STATUS_FAIL;
    mes_add_event(mes, CXL_EVENT_TYPE_FATAL, &mut hardware_replace);
    mes_add_event(mes, CXL_EVENT_TYPE_FATAL, &mut dram as *mut _ as *mut cxl_event_record_raw);
    (*mes).ev_status |= CXLDEV_EVENT_STATUS_FATAL;
}

unsafe extern "C" fn mock_gsl(cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_out < size_of::<mock_gsl_payload_t>() { return -EINVAL; }
    memcpy((*cmd).payload_out, &raw const mock_gsl_payload as *const c_void, size_of::<mock_gsl_payload_t>());
    (*cmd).size_out = size_of::<mock_gsl_payload_t>();
    0
}

unsafe extern "C" fn mock_get_log(mds: *mut cxl_memdev_state, cmd: *mut cxl_mbox_cmd) -> c_int {
    let cxl_mbox = &mut (*mds).cxlds.cxl_mbox as *mut cxl_mailbox;
    let uuid = DEFINE_CXL_CEL_UUID;
    if (*cmd).size_in < size_of::<cxl_mbox_get_log>() { return -EINVAL; }
    let gl = (*cmd).payload_in as *mut cxl_mbox_get_log;
    let offset = le32_to_cpu((*gl).offset);
    let length = le32_to_cpu((*gl).length);
    if length as usize > (*cxl_mbox).payload_size { return -EINVAL; }
    if offset as usize + length as usize > size_of::<[cxl_cel_entry; 17]>() { return -EINVAL; }
    if !uuid_equal(&(*gl).uuid, &uuid) { return -EINVAL; }
    if length as usize > (*cmd).size_out { return -EINVAL; }
    memcpy((*cmd).payload_out, (&raw const mock_cel as *const u8).add(offset as usize) as *const c_void, length as usize);
    0
}

unsafe extern "C" fn mock_rcd_id(cmd: *mut cxl_mbox_cmd) -> c_int {
    let mut id: cxl_mbox_identify = zeroed();
    memcpy(id.fw_revision.as_mut_ptr() as *mut c_void, c"mock fw v1 ".as_ptr() as *const c_void, 12);
    id.total_capacity = cpu_to_le64(DEV_SIZE / CXL_CAPACITY_MULTIPLIER);
    id.volatile_capacity = cpu_to_le64(DEV_SIZE / CXL_CAPACITY_MULTIPLIER);
    if (*cmd).size_out < size_of::<cxl_mbox_identify>() { return -EINVAL; }
    memcpy((*cmd).payload_out, &id as *const _ as *const c_void, size_of::<cxl_mbox_identify>());
    0
}

unsafe extern "C" fn mock_id(cmd: *mut cxl_mbox_cmd) -> c_int {
    let mut id: cxl_mbox_identify = zeroed();
    memcpy(id.fw_revision.as_mut_ptr() as *mut c_void, c"mock fw v1 ".as_ptr() as *const c_void, 12);
    id.lsa_size = cpu_to_le32(LSA_SIZE as u32);
    id.partition_align = cpu_to_le64(SZ_256M / CXL_CAPACITY_MULTIPLIER);
    id.total_capacity = cpu_to_le64(DEV_SIZE / CXL_CAPACITY_MULTIPLIER);
    id.inject_poison_limit = cpu_to_le16(MOCK_INJECT_TEST_MAX as u16);
    put_unaligned_le24(CXL_POISON_LIST_MAX as u32, id.poison_list_max_mer.as_mut_ptr());
    if (*cmd).size_out < size_of::<cxl_mbox_identify>() { return -EINVAL; }
    memcpy((*cmd).payload_out, &id as *const _ as *const c_void, size_of::<cxl_mbox_identify>());
    0
}

unsafe extern "C" fn mock_partition_info(cmd: *mut cxl_mbox_cmd) -> c_int {
    let pi = cxl_mbox_get_partition_info {
        active_volatile_cap: cpu_to_le64(DEV_SIZE / 2 / CXL_CAPACITY_MULTIPLIER),
        active_persistent_cap: cpu_to_le64(DEV_SIZE / 2 / CXL_CAPACITY_MULTIPLIER),
    };
    if (*cmd).size_out < size_of::<cxl_mbox_get_partition_info>() { return -EINVAL; }
    memcpy((*cmd).payload_out, &pi as *const _ as *const c_void, size_of::<cxl_mbox_get_partition_info>());
    0
}

#[no_mangle]
pub unsafe extern "C" fn cxl_mockmem_sanitize_work(work: *mut work_struct) {
    let mds = container_of_security_poll_dwork_work(work);
    let cxl_mbox = &mut (*mds).cxlds.cxl_mbox as *mut cxl_mailbox;
    mutex_lock(&mut (*cxl_mbox).mbox_mutex);
    if !(*mds).security.sanitize_node.is_null() {
        sysfs_notify_dirent((*mds).security.sanitize_node);
    }
    (*mds).security.sanitize_active = false;
    mutex_unlock(&mut (*cxl_mbox).mbox_mutex);
    dev_dbg((*mds).cxlds.dev, c"sanitize complete\n".as_ptr());
}

unsafe extern "C" fn mock_sanitize(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let mds = (*mdata).mds;
    let cxl_mbox = &mut (*mds).cxlds.cxl_mbox as *mut cxl_mailbox;
    let mut rc = 0;
    if (*cmd).size_in != 0 || (*cmd).size_out != 0 { return -EINVAL; }
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PASS_SET != 0 || (*mdata).security_state & CXL_PMEM_SEC_STATE_LOCKED != 0 {
        (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY;
        return -ENXIO;
    }
    mutex_lock(&mut (*cxl_mbox).mbox_mutex);
    if schedule_delayed_work(&mut (*mds).security.poll_dwork, msecs_to_jiffies((*mdata).sanitize_timeout)) {
        (*mds).security.sanitize_active = true;
        dev_dbg((*mds).cxlds.dev, c"sanitize issued\n".as_ptr());
    } else { rc = -EBUSY; }
    mutex_unlock(&mut (*cxl_mbox).mbox_mutex);
    rc
}

unsafe extern "C" fn mock_secure_erase(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in != 0 || (*cmd).size_out != 0 { return -EINVAL; }
    if (*mdata).security_state & (CXL_PMEM_SEC_STATE_USER_PASS_SET | CXL_PMEM_SEC_STATE_LOCKED) != 0 {
        (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY;
        return -ENXIO;
    }
    0
}

unsafe extern "C" fn mock_get_security_state(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in != 0 { return -EINVAL; }
    if (*cmd).size_out != size_of::<u32>() { return -EINVAL; }
    memcpy((*cmd).payload_out, &(*mdata).security_state as *const _ as *const c_void, size_of::<u32>());
    0
}

unsafe fn master_plimit_check(mdata: *mut cxl_mockmem_data) {
    if (*mdata).master_limit == PASS_TRY_LIMIT { return; }
    (*mdata).master_limit += 1;
    if (*mdata).master_limit == PASS_TRY_LIMIT { (*mdata).security_state |= CXL_PMEM_SEC_STATE_MASTER_PLIMIT; }
}

unsafe fn user_plimit_check(mdata: *mut cxl_mockmem_data) {
    if (*mdata).user_limit == PASS_TRY_LIMIT { return; }
    (*mdata).user_limit += 1;
    if (*mdata).user_limit == PASS_TRY_LIMIT { (*mdata).security_state |= CXL_PMEM_SEC_STATE_USER_PLIMIT; }
}

unsafe extern "C" fn mock_set_passphrase(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in != size_of::<cxl_set_pass>() || (*cmd).size_out != 0 { return -EINVAL; }
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_FROZEN != 0 { (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO; }
    let set_pass = (*cmd).payload_in as *mut cxl_set_pass;
    match (*set_pass).type_ as u32 {
        CXL_PMEM_SEC_PASS_MASTER => {
            if (*mdata).security_state & CXL_PMEM_SEC_STATE_MASTER_PLIMIT != 0 || (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PASS_SET != 0 {
                (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO;
            }
            if memcmp((*mdata).master_pass.as_ptr() as *const c_void, (*set_pass).old_pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN) != 0 {
                master_plimit_check(mdata); (*cmd).return_code = CXL_MBOX_CMD_RC_PASSPHRASE; return -ENXIO;
            }
            memcpy((*mdata).master_pass.as_mut_ptr() as *mut c_void, (*set_pass).new_pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN);
            (*mdata).security_state |= CXL_PMEM_SEC_STATE_MASTER_PASS_SET;
            0
        }
        CXL_PMEM_SEC_PASS_USER => {
            if (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PLIMIT != 0 { (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO; }
            if memcmp((*mdata).user_pass.as_ptr() as *const c_void, (*set_pass).old_pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN) != 0 {
                user_plimit_check(mdata); (*cmd).return_code = CXL_MBOX_CMD_RC_PASSPHRASE; return -ENXIO;
            }
            memcpy((*mdata).user_pass.as_mut_ptr() as *mut c_void, (*set_pass).new_pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN);
            (*mdata).security_state |= CXL_PMEM_SEC_STATE_USER_PASS_SET;
            0
        }
        _ => { (*cmd).return_code = CXL_MBOX_CMD_RC_INPUT; -EINVAL }
    }
}

unsafe extern "C" fn mock_disable_passphrase(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in != size_of::<cxl_disable_pass>() || (*cmd).size_out != 0 { return -EINVAL; }
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_FROZEN != 0 { (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO; }
    let dis_pass = (*cmd).payload_in as *mut cxl_disable_pass;
    match (*dis_pass).type_ as u32 {
        CXL_PMEM_SEC_PASS_MASTER => {
            if (*mdata).security_state & CXL_PMEM_SEC_STATE_MASTER_PLIMIT != 0 || (*mdata).security_state & CXL_PMEM_SEC_STATE_MASTER_PASS_SET == 0 {
                (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO;
            }
            if memcmp((*dis_pass).pass.as_ptr() as *const c_void, (*mdata).master_pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN) != 0 {
                master_plimit_check(mdata); (*cmd).return_code = CXL_MBOX_CMD_RC_PASSPHRASE; return -ENXIO;
            }
            (*mdata).master_limit = 0;
            memset((*mdata).master_pass.as_mut_ptr() as *mut c_void, 0, NVDIMM_PASSPHRASE_LEN);
            (*mdata).security_state &= !CXL_PMEM_SEC_STATE_MASTER_PASS_SET;
            0
        }
        CXL_PMEM_SEC_PASS_USER => {
            if (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PLIMIT != 0 || (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PASS_SET == 0 {
                (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO;
            }
            if memcmp((*dis_pass).pass.as_ptr() as *const c_void, (*mdata).user_pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN) != 0 {
                user_plimit_check(mdata); (*cmd).return_code = CXL_MBOX_CMD_RC_PASSPHRASE; return -ENXIO;
            }
            (*mdata).user_limit = 0;
            memset((*mdata).user_pass.as_mut_ptr() as *mut c_void, 0, NVDIMM_PASSPHRASE_LEN);
            (*mdata).security_state &= !(CXL_PMEM_SEC_STATE_USER_PASS_SET | CXL_PMEM_SEC_STATE_LOCKED);
            0
        }
        _ => { (*cmd).return_code = CXL_MBOX_CMD_RC_INPUT; -EINVAL }
    }
}

unsafe extern "C" fn mock_freeze_security(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in != 0 || (*cmd).size_out != 0 { return -EINVAL; }
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_FROZEN == 0 { (*mdata).security_state |= CXL_PMEM_SEC_STATE_FROZEN; }
    0
}

unsafe extern "C" fn mock_unlock_security(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in != NVDIMM_PASSPHRASE_LEN || (*cmd).size_out != 0 { return -EINVAL; }
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_FROZEN != 0 ||
       (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PASS_SET == 0 ||
       (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PLIMIT != 0 ||
       (*mdata).security_state & CXL_PMEM_SEC_STATE_LOCKED == 0 {
        (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO;
    }
    if memcmp((*cmd).payload_in, (*mdata).user_pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN) != 0 {
        (*mdata).user_limit += 1;
        if (*mdata).user_limit == PASS_TRY_LIMIT { (*mdata).security_state |= CXL_PMEM_SEC_STATE_USER_PLIMIT; }
        (*cmd).return_code = CXL_MBOX_CMD_RC_PASSPHRASE;
        return -ENXIO;
    }
    (*mdata).user_limit = 0;
    (*mdata).security_state &= !CXL_PMEM_SEC_STATE_LOCKED;
    0
}

unsafe extern "C" fn mock_passphrase_secure_erase(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in != size_of::<cxl_pass_erase>() || (*cmd).size_out != 0 { return -EINVAL; }
    let erase = (*cmd).payload_in as *mut cxl_pass_erase;
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_FROZEN != 0 { (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO; }
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PLIMIT != 0 && (*erase).type_ as u32 == CXL_PMEM_SEC_PASS_USER { (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO; }
    if (*mdata).security_state & CXL_PMEM_SEC_STATE_MASTER_PLIMIT != 0 && (*erase).type_ as u32 == CXL_PMEM_SEC_PASS_MASTER { (*cmd).return_code = CXL_MBOX_CMD_RC_SECURITY; return -ENXIO; }
    match (*erase).type_ as u32 {
        CXL_PMEM_SEC_PASS_MASTER => {
            if (*mdata).security_state & CXL_PMEM_SEC_STATE_MASTER_PASS_SET != 0 {
                if memcmp((*mdata).master_pass.as_ptr() as *const c_void, (*erase).pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN) != 0 {
                    master_plimit_check(mdata); (*cmd).return_code = CXL_MBOX_CMD_RC_PASSPHRASE; return -ENXIO;
                }
                (*mdata).master_limit = 0;
                (*mdata).user_limit = 0;
                (*mdata).security_state &= !CXL_PMEM_SEC_STATE_USER_PASS_SET;
                memset((*mdata).user_pass.as_mut_ptr() as *mut c_void, 0, NVDIMM_PASSPHRASE_LEN);
                (*mdata).security_state &= !CXL_PMEM_SEC_STATE_LOCKED;
            } else { return -EINVAL; }
        }
        CXL_PMEM_SEC_PASS_USER => {
            if (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PASS_SET != 0 {
                if memcmp((*mdata).user_pass.as_ptr() as *const c_void, (*erase).pass.as_ptr() as *const c_void, NVDIMM_PASSPHRASE_LEN) != 0 {
                    user_plimit_check(mdata); (*cmd).return_code = CXL_MBOX_CMD_RC_PASSPHRASE; return -ENXIO;
                }
                (*mdata).user_limit = 0;
                (*mdata).security_state &= !CXL_PMEM_SEC_STATE_USER_PASS_SET;
                memset((*mdata).user_pass.as_mut_ptr() as *mut c_void, 0, NVDIMM_PASSPHRASE_LEN);
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn mock_get_lsa(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let get_lsa = (*cmd).payload_in as *mut cxl_mbox_get_lsa;
    if size_of::<cxl_mbox_get_lsa>() > (*cmd).size_in { return -EINVAL; }
    let offset = le32_to_cpu((*get_lsa).offset) as usize;
    let length = le32_to_cpu((*get_lsa).length) as usize;
    if offset > LSA_SIZE || length > LSA_SIZE - offset || length > (*cmd).size_out { return -EINVAL; }
    memcpy((*cmd).payload_out, ((*mdata).lsa as *mut u8).add(offset) as *const c_void, length);
    0
}

unsafe extern "C" fn mock_set_lsa(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let set_lsa = (*cmd).payload_in as *mut cxl_mbox_set_lsa;
    if size_of::<cxl_mbox_set_lsa>() > (*cmd).size_in { return -EINVAL; }
    let offset = le32_to_cpu((*set_lsa).offset) as usize;
    let length = (*cmd).size_in - size_of::<cxl_mbox_set_lsa>();
    if offset > LSA_SIZE || length > LSA_SIZE - offset { return -EINVAL; }
    memcpy(((*mdata).lsa as *mut u8).add(offset) as *mut c_void, (*set_lsa).data.as_ptr() as *const c_void, length);
    0
}

unsafe extern "C" fn mock_health_info(cmd: *mut cxl_mbox_cmd) -> c_int {
    let health_info = cxl_mbox_health_info { health_status: 0x7, media_status: 0x3, ext_status: 0x18, life_used: 15, temperature: cpu_to_le16(25), dirty_shutdowns: cpu_to_le32(10), volatile_errors: cpu_to_le32(20), pmem_errors: cpu_to_le32(30) };
    if (*cmd).size_out < size_of::<cxl_mbox_health_info>() { return -EINVAL; }
    memcpy((*cmd).payload_out, &health_info as *const _ as *const c_void, size_of::<cxl_mbox_health_info>());
    0
}

unsafe extern "C" fn mock_set_shutdown_state(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let ss = (*cmd).payload_in as *mut cxl_mbox_set_shutdown_state_in;
    if (*cmd).size_in != size_of::<cxl_mbox_set_shutdown_state_in>() || (*cmd).size_out != 0 { return -EINVAL; }
    (*mdata).shutdown_state = (*ss).state;
    0
}

#[repr(C)]
struct mock_poison {
    cxlds: *mut cxl_dev_state,
    dpa: u64,
}

static mut mock_poison_list: [mock_poison; MOCK_INJECT_TEST_MAX] = [const { mock_poison { cxlds: ptr::null_mut(), dpa: 0 } }; MOCK_INJECT_TEST_MAX];

unsafe fn cxl_get_injected_po(cxlds: *mut cxl_dev_state, offset: u64, length: u64) -> *mut cxl_mbox_poison_out {
    let po = kzalloc(struct_size(size_of::<cxl_mbox_poison_out>(), size_of::<cxl_mbox_poison_record>(), poison_inject_dev_max as usize), GFP_KERNEL) as *mut cxl_mbox_poison_out;
    if po.is_null() { return ptr::null_mut(); }
    let mut nr_records = 0usize;
    for i in 0..MOCK_INJECT_TEST_MAX {
        if mock_poison_list[i].cxlds != cxlds { continue; }
        if mock_poison_list[i].dpa < offset || mock_poison_list[i].dpa > offset + length - 1 { continue; }
        let dpa = mock_poison_list[i].dpa + CXL_POISON_SOURCE_INJECTED as u64;
        (*po).record.as_mut_ptr().add(nr_records).write(cxl_mbox_poison_record { address: cpu_to_le64(dpa), length: cpu_to_le32(1) });
        nr_records += 1;
        if nr_records == poison_inject_dev_max as usize { break; }
    }
    (*po).count = cpu_to_le16(nr_records as u16);
    po
}

unsafe extern "C" fn mock_get_poison(cxlds: *mut cxl_dev_state, cmd: *mut cxl_mbox_cmd) -> c_int {
    let pi = (*cmd).payload_in as *mut cxl_mbox_poison_in;
    let po = cxl_get_injected_po(cxlds, le64_to_cpu((*pi).offset), le64_to_cpu((*pi).length));
    if po.is_null() { return -ENOMEM; }
    let nr_records = le16_to_cpu((*po).count) as usize;
    let size = struct_size(size_of::<cxl_mbox_poison_out>(), size_of::<cxl_mbox_poison_record>(), nr_records);
    memcpy((*cmd).payload_out, po as *const c_void, size);
    (*cmd).size_out = size;
    kfree(po as *mut c_void);
    0
}

unsafe fn mock_poison_dev_max_injected(cxlds: *mut cxl_dev_state) -> bool {
    let mut count = 0u32;
    for i in 0..MOCK_INJECT_TEST_MAX {
        if mock_poison_list[i].cxlds == cxlds { count += 1; }
    }
    count >= poison_inject_dev_max
}

unsafe fn mock_poison_add(cxlds: *mut cxl_dev_state, dpa: u64) -> c_int {
    if mock_poison_dev_max_injected(cxlds) {
        dev_dbg((*cxlds).dev, c"Device poison injection limit has been reached: %d\n".as_ptr(), poison_inject_dev_max);
        return -EBUSY;
    }
    for i in 0..MOCK_INJECT_TEST_MAX {
        if mock_poison_list[i].cxlds.is_null() {
            mock_poison_list[i].cxlds = cxlds;
            mock_poison_list[i].dpa = dpa;
            return 0;
        }
    }
    dev_dbg((*cxlds).dev, c"Mock test poison injection limit has been reached: %d\n".as_ptr(), MOCK_INJECT_TEST_MAX as c_int);
    -ENXIO
}

unsafe fn mock_poison_found(cxlds: *mut cxl_dev_state, dpa: u64) -> bool {
    for i in 0..MOCK_INJECT_TEST_MAX {
        if mock_poison_list[i].cxlds == cxlds && mock_poison_list[i].dpa == dpa { return true; }
    }
    false
}

unsafe extern "C" fn mock_inject_poison(cxlds: *mut cxl_dev_state, cmd: *mut cxl_mbox_cmd) -> c_int {
    let pi = (*cmd).payload_in as *mut cxl_mbox_inject_poison;
    let dpa = le64_to_cpu((*pi).address);
    if mock_poison_found(cxlds, dpa) {
        dev_dbg((*cxlds).dev, c"DPA: 0x%llx already poisoned\n".as_ptr(), dpa);
        return 0;
    }
    mock_poison_add(cxlds, dpa)
}

unsafe fn mock_poison_del(cxlds: *mut cxl_dev_state, dpa: u64) -> bool {
    for i in 0..MOCK_INJECT_TEST_MAX {
        if mock_poison_list[i].cxlds == cxlds && mock_poison_list[i].dpa == dpa {
            mock_poison_list[i].cxlds = ptr::null_mut();
            return true;
        }
    }
    false
}

unsafe extern "C" fn mock_clear_poison(cxlds: *mut cxl_dev_state, cmd: *mut cxl_mbox_cmd) -> c_int {
    let pi = (*cmd).payload_in as *mut cxl_mbox_clear_poison;
    let dpa = le64_to_cpu((*pi).address);
    if !mock_poison_del(cxlds, dpa) {
        dev_dbg((*cxlds).dev, c"DPA: 0x%llx not in poison list\n".as_ptr(), dpa);
    }
    0
}

unsafe fn mock_poison_list_empty() -> bool {
    for i in 0..MOCK_INJECT_TEST_MAX {
        if !mock_poison_list[i].cxlds.is_null() { return false; }
    }
    true
}

unsafe extern "C" fn poison_inject_max_show(_drv: *mut device_driver, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, c"%u\n".as_ptr(), poison_inject_dev_max)
}

unsafe extern "C" fn poison_inject_max_store(_drv: *mut device_driver, buf: *const c_char, len: size_t) -> ssize_t {
    let mut val: c_int = 0;
    if kstrtoint(buf, 0, &mut val) < 0 { return -EINVAL as ssize_t; }
    if !mock_poison_list_empty() { return -EBUSY as ssize_t; }
    if val <= MOCK_INJECT_TEST_MAX as c_int { poison_inject_dev_max = val as u32; } else { return -EINVAL as ssize_t; }
    len as ssize_t
}

// DRIVER_ATTR_RW(poison_inject_max);
// ATTRIBUTE_GROUPS(cxl_mock_mem_core);

unsafe extern "C" fn mock_fw_info(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let mut fw_info: cxl_mbox_get_fw_info = zeroed();
    fw_info.num_slots = FW_SLOTS as u8;
    fw_info.slot_info = (((*mdata).fw_slot & 0x7) | (((*mdata).fw_staged & 0x7) << 3)) as u8;
    fw_info.activation_cap = 0;
    strcpy(fw_info.slot_1_revision.as_mut_ptr(), c"cxl_test_fw_001".as_ptr());
    strcpy(fw_info.slot_2_revision.as_mut_ptr(), c"cxl_test_fw_002".as_ptr());
    strcpy(fw_info.slot_3_revision.as_mut_ptr(), c"cxl_test_fw_003".as_ptr());
    strcpy(fw_info.slot_4_revision.as_mut_ptr(), c"".as_ptr());
    if (*cmd).size_out < size_of::<cxl_mbox_get_fw_info>() { return -EINVAL; }
    memcpy((*cmd).payload_out, &fw_info as *const _ as *const c_void, size_of::<cxl_mbox_get_fw_info>());
    0
}

unsafe extern "C" fn mock_transfer_fw(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    if (*cmd).size_in < size_of::<cxl_mbox_transfer_fw>() { return -EINVAL; }
    let transfer = (*cmd).payload_in as *mut cxl_mbox_transfer_fw;
    let offset = le32_to_cpu((*transfer).offset) as usize * CXL_FW_TRANSFER_ALIGNMENT;
    let length = (*cmd).size_in - size_of::<cxl_mbox_transfer_fw>();
    if offset + length > FW_SIZE { return -EINVAL; }
    match (*transfer).action as u32 {
        CXL_FW_TRANSFER_ACTION_FULL => {
            if offset != 0 { return -EINVAL; }
            if (*transfer).slot == 0 || (*transfer).slot as c_int > FW_SLOTS { return -EINVAL; }
            (*mdata).fw_size = offset + length;
        }
        CXL_FW_TRANSFER_ACTION_END => {
            if (*transfer).slot == 0 || (*transfer).slot as c_int > FW_SLOTS { return -EINVAL; }
            (*mdata).fw_size = offset + length;
        }
        CXL_FW_TRANSFER_ACTION_INITIATE | CXL_FW_TRANSFER_ACTION_CONTINUE => {}
        CXL_FW_TRANSFER_ACTION_ABORT => return 0,
        _ => return -EINVAL,
    }
    memcpy(((*mdata).fw as *mut u8).add(offset) as *mut c_void, (*transfer).data.as_ptr() as *const c_void, length);
    usleep_range(1500, 2000);
    0
}

unsafe extern "C" fn mock_activate_fw(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let activate = (*cmd).payload_in as *mut cxl_mbox_activate_fw;
    if (*activate).slot == 0 || (*activate).slot as c_int > FW_SLOTS { return -EINVAL; }
    match (*activate).action as u32 {
        CXL_FW_ACTIVATE_ONLINE => { (*mdata).fw_slot = (*activate).slot as c_int; (*mdata).fw_staged = 0; 0 }
        CXL_FW_ACTIVATE_OFFLINE => { (*mdata).fw_staged = (*activate).slot as c_int; 0 }
        _ => -EINVAL,
    }
}

const MAX_CXL_TEST_FEATS: u16 = 1;

unsafe fn fill_feature_vendor_test(feat: *mut cxl_feat_entry) {
    (*feat).uuid = CXL_VENDOR_FEATURE_TEST;
    (*feat).id = 0;
    (*feat).get_feat_size = cpu_to_le16(0x4);
    (*feat).set_feat_size = cpu_to_le16(0x4);
    (*feat).flags = cpu_to_le32(CXL_FEATURE_F_CHANGEABLE | CXL_FEATURE_F_DEFAULT_SEL | CXL_FEATURE_F_SAVED_SEL);
    (*feat).get_feat_ver = 1;
    (*feat).set_feat_ver = 1;
    (*feat).effects = cpu_to_le16((CXL_CMD_CONFIG_CHANGE_COLD_RESET | CXL_CMD_EFFECTS_VALID) as u16);
}

unsafe extern "C" fn mock_get_test_feature(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let output = (*cmd).payload_out as *mut vendor_test_feat;
    if (*cmd).size_in < size_of::<cxl_mbox_get_feat_in>() { return -EINVAL; }
    let input = (*cmd).payload_in as *mut cxl_mbox_get_feat_in;
    let offset = le16_to_cpu((*input).offset) as usize;
    let count = le16_to_cpu((*input).count) as usize;
    if offset > size_of::<vendor_test_feat>() || offset + count > size_of::<vendor_test_feat>() {
        (*cmd).return_code = CXL_MBOX_CMD_RC_INPUT;
        return -EINVAL;
    }
    let ptr = (&mut (*mdata).test_feat as *mut vendor_test_feat as *mut u8).add(offset);
    memcpy((output as *mut u8).add(offset) as *mut c_void, ptr as *const c_void, count);
    0
}

unsafe extern "C" fn mock_get_feature(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let input = (*cmd).payload_in as *mut cxl_mbox_get_feat_in;
    if uuid_equal(&(*input).uuid, &CXL_VENDOR_FEATURE_TEST) { return mock_get_test_feature(mdata, cmd); }
    (*cmd).return_code = CXL_MBOX_CMD_RC_UNSUPPORTED;
    -EOPNOTSUPP
}

unsafe extern "C" fn mock_set_test_feature(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let input = (*cmd).payload_in as *mut cxl_mbox_set_feat_in;
    let test = (*input).feat_data.as_mut_ptr() as *mut vendor_test_feat;
    let action = FIELD_GET(CXL_SET_FEAT_FLAG_DATA_TRANSFER_MASK, le32_to_cpu((*input).hdr.flags));
    if action != CXL_SET_FEAT_FLAG_FULL_DATA_TRANSFER || (*input).hdr.offset != 0 {
        (*cmd).return_code = CXL_MBOX_CMD_RC_INPUT;
        return -EINVAL;
    }
    memcpy(&mut (*mdata).test_feat.data as *mut _ as *mut c_void, &(*test).data as *const _ as *const c_void, size_of::<u32>());
    0
}

unsafe extern "C" fn mock_set_feature(mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let input = (*cmd).payload_in as *mut cxl_mbox_set_feat_in;
    if uuid_equal(&(*input).hdr.uuid, &CXL_VENDOR_FEATURE_TEST) { return mock_set_test_feature(mdata, cmd); }
    (*cmd).return_code = CXL_MBOX_CMD_RC_UNSUPPORTED;
    -EOPNOTSUPP
}

unsafe extern "C" fn mock_get_supported_features(_mdata: *mut cxl_mockmem_data, cmd: *mut cxl_mbox_cmd) -> c_int {
    let input = (*cmd).payload_in as *mut cxl_mbox_get_sup_feats_in;
    let out = (*cmd).payload_out as *mut cxl_mbox_get_sup_feats_out;
    if (*cmd).size_out < size_of::<cxl_mbox_get_sup_feats_out>() {
        (*cmd).return_code = CXL_MBOX_CMD_RC_PAYLOADLEN;
        return -EINVAL;
    }
    let start_idx = le16_to_cpu((*input).start_idx);
    if start_idx != 0 {
        (*cmd).return_code = CXL_MBOX_CMD_RC_INPUT;
        return -EINVAL;
    }
    let count = le16_to_cpu((*input).count) as usize;
    if count < struct_size(size_of::<cxl_mbox_get_sup_feats_out>(), size_of::<cxl_feat_entry>(), 0) {
        (*cmd).return_code = CXL_MBOX_CMD_RC_PAYLOADLEN;
        return -EINVAL;
    }
    (*out).supported_feats = cpu_to_le16(MAX_CXL_TEST_FEATS);
    (*cmd).return_code = 0;
    if count < struct_size(size_of::<cxl_mbox_get_sup_feats_out>(), size_of::<cxl_feat_entry>(), MAX_CXL_TEST_FEATS as usize) {
        (*out).num_entries = 0;
        return 0;
    }
    (*out).num_entries = cpu_to_le16(MAX_CXL_TEST_FEATS);
    fill_feature_vendor_test((*out).ents.as_mut_ptr());
    0
}

unsafe extern "C" fn cxl_mock_mbox_send(cxl_mbox: *mut cxl_mailbox, cmd: *mut cxl_mbox_cmd) -> c_int {
    let dev = (*cxl_mbox).host;
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    let mds = (*mdata).mds;
    let cxlds = &mut (*mds).cxlds as *mut cxl_dev_state;
    let mut rc = -EIO;
    match (*cmd).opcode as u32 {
        CXL_MBOX_OP_SET_TIMESTAMP => rc = mock_set_timestamp(cxlds, cmd),
        CXL_MBOX_OP_GET_SUPPORTED_LOGS => rc = mock_gsl(cmd),
        CXL_MBOX_OP_GET_LOG => rc = mock_get_log(mds, cmd),
        CXL_MBOX_OP_IDENTIFY => if (*cxlds).rcd { rc = mock_rcd_id(cmd); } else { rc = mock_id(cmd); },
        CXL_MBOX_OP_GET_LSA => rc = mock_get_lsa(mdata, cmd),
        CXL_MBOX_OP_GET_PARTITION_INFO => rc = mock_partition_info(cmd),
        CXL_MBOX_OP_GET_EVENT_RECORD => rc = mock_get_event(dev, cmd),
        CXL_MBOX_OP_CLEAR_EVENT_RECORD => rc = mock_clear_event(dev, cmd),
        CXL_MBOX_OP_SET_LSA => rc = mock_set_lsa(mdata, cmd),
        CXL_MBOX_OP_GET_HEALTH_INFO => rc = mock_health_info(cmd),
        CXL_MBOX_OP_SANITIZE => rc = mock_sanitize(mdata, cmd),
        CXL_MBOX_OP_SECURE_ERASE => rc = mock_secure_erase(mdata, cmd),
        CXL_MBOX_OP_GET_SECURITY_STATE => rc = mock_get_security_state(mdata, cmd),
        CXL_MBOX_OP_SET_PASSPHRASE => rc = mock_set_passphrase(mdata, cmd),
        CXL_MBOX_OP_DISABLE_PASSPHRASE => rc = mock_disable_passphrase(mdata, cmd),
        CXL_MBOX_OP_FREEZE_SECURITY => rc = mock_freeze_security(mdata, cmd),
        CXL_MBOX_OP_UNLOCK => rc = mock_unlock_security(mdata, cmd),
        CXL_MBOX_OP_PASSPHRASE_SECURE_ERASE => rc = mock_passphrase_secure_erase(mdata, cmd),
        CXL_MBOX_OP_SET_SHUTDOWN_STATE => rc = mock_set_shutdown_state(mdata, cmd),
        CXL_MBOX_OP_GET_POISON => rc = mock_get_poison(cxlds, cmd),
        CXL_MBOX_OP_INJECT_POISON => rc = mock_inject_poison(cxlds, cmd),
        CXL_MBOX_OP_CLEAR_POISON => rc = mock_clear_poison(cxlds, cmd),
        CXL_MBOX_OP_GET_FW_INFO => rc = mock_fw_info(mdata, cmd),
        CXL_MBOX_OP_TRANSFER_FW => rc = mock_transfer_fw(mdata, cmd),
        CXL_MBOX_OP_ACTIVATE_FW => rc = mock_activate_fw(mdata, cmd),
        CXL_MBOX_OP_GET_SUPPORTED_FEATURES => rc = mock_get_supported_features(mdata, cmd),
        CXL_MBOX_OP_GET_FEATURE => rc = mock_get_feature(mdata, cmd),
        CXL_MBOX_OP_SET_FEATURE => rc = mock_set_feature(mdata, cmd),
        _ => {}
    }
    dev_dbg(dev, c"opcode: %#x sz_in: %zd sz_out: %zd rc: %d\n".as_ptr(), (*cmd).opcode as c_int, (*cmd).size_in, (*cmd).size_out, rc);
    rc
}

unsafe extern "C" fn label_area_release(lsa: *mut c_void) { vfree(lsa); }
unsafe extern "C" fn fw_buf_release(buf: *mut c_void) { vfree(buf); }

unsafe fn is_rcd(pdev: *mut platform_device) -> bool {
    let id = platform_get_device_id(pdev);
    !id.is_null() && (*id).driver_data != 0
}

unsafe extern "C" fn event_trigger_store(dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char, count: size_t) -> ssize_t {
    cxl_mock_event_trigger(dev);
    count as ssize_t
}
// DEVICE_ATTR_WO(event_trigger);

unsafe fn cxl_mock_mailbox_create(cxlds: *mut cxl_dev_state) -> c_int {
    let rc = cxl_mailbox_init(&mut (*cxlds).cxl_mbox, (*cxlds).dev);
    if rc != 0 { return rc; }
    0
}

unsafe fn cxl_mock_test_feat_init(mdata: *mut cxl_mockmem_data) {
    (*mdata).test_feat.data = cpu_to_le32(0xdeadbeef);
}

unsafe extern "C" fn cxl_mock_mem_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut range_info: cxl_dpa_info = zeroed();
    let mut serial: u64;
    usleep_range(500 * 1000, 1000 * 1000);
    let mdata = devm_kzalloc(dev, size_of::<cxl_mockmem_data>(), GFP_KERNEL) as *mut cxl_mockmem_data;
    if mdata.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, mdata as *mut c_void);
    (*mdata).lsa = vzalloc(LSA_SIZE);
    if (*mdata).lsa.is_null() { return -ENOMEM; }
    (*mdata).fw = vmalloc(FW_SIZE);
    if (*mdata).fw.is_null() { return -ENOMEM; }
    (*mdata).fw_slot = 2;
    let mut rc = devm_add_action_or_reset(dev, label_area_release, (*mdata).lsa);
    if rc != 0 { return rc; }
    rc = devm_add_action_or_reset(dev, fw_buf_release, (*mdata).fw);
    if rc != 0 { return rc; }
    if (*pdev).id == 7 { serial = 0x8a34567890abcdef; } else { serial = (*pdev).id as u64 + 1; }
    let mds = cxl_memdev_state_create(dev, serial, 0);
    if IS_ERR(mds as *const c_void) { return PTR_ERR(mds as *const c_void); }
    let cxlds = &mut (*mds).cxlds as *mut cxl_dev_state;
    rc = cxl_mock_mailbox_create(cxlds);
    if rc != 0 { return rc; }
    let cxl_mbox = &mut (*mds).cxlds.cxl_mbox as *mut cxl_mailbox;
    (*mdata).mds = mds;
    (*cxl_mbox).mbox_send = Some(cxl_mock_mbox_send);
    (*cxl_mbox).payload_size = SZ_4K;
    (*mds).event.buf = (*mdata).event_buf.as_mut_ptr() as *mut cxl_get_event_payload;
    INIT_DELAYED_WORK(&mut (*mds).security.poll_dwork, cxl_mockmem_sanitize_work);
    if is_rcd(pdev) { (*cxlds).rcd = true; }
    rc = cxl_enumerate_cmds(mds);
    if rc != 0 { return rc; }
    rc = cxl_poison_state_init(mds);
    if rc != 0 { return rc; }
    rc = cxl_set_timestamp(mds);
    if rc != 0 { return rc; }
    (*cxlds).media_ready = true;
    rc = cxl_dev_state_identify(mds);
    if rc != 0 { return rc; }
    rc = cxl_mem_dpa_fetch(mds, &mut range_info);
    if rc != 0 { return rc; }
    rc = cxl_dpa_setup(cxlds, &mut range_info);
    if rc != 0 { return rc; }
    rc = devm_cxl_setup_features(cxlds);
    if rc != 0 { dev_dbg(dev, c"No CXL Features discovered\n".as_ptr()); }
    cxl_mock_add_event_logs(&mut (*mdata).mes);
    let cxlmd = devm_cxl_add_classdev(cxlds);
    if IS_ERR(cxlmd as *const c_void) { return PTR_ERR(cxlmd as *const c_void); }
    rc = devm_cxl_setup_fw_upload(&mut (*pdev).dev, mds);
    if rc != 0 { return rc; }
    rc = devm_cxl_sanitize_setup_notifier(&mut (*pdev).dev, cxlmd);
    if rc != 0 { return rc; }
    rc = devm_cxl_setup_fwctl(&mut (*pdev).dev, cxlmd);
    if rc != 0 { dev_dbg(dev, c"No CXL FWCTL setup\n".as_ptr()); }
    cxl_mem_get_event_records(mds, CXLDEV_EVENT_STATUS_ALL);
    cxl_mock_test_feat_init(mdata);
    0
}

unsafe extern "C" fn security_lock_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    sysfs_emit(buf, c"%u\n".as_ptr(), ((*mdata).security_state & CXL_PMEM_SEC_STATE_LOCKED != 0) as c_uint)
}

unsafe extern "C" fn security_lock_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    let mask = CXL_PMEM_SEC_STATE_FROZEN | CXL_PMEM_SEC_STATE_USER_PLIMIT | CXL_PMEM_SEC_STATE_MASTER_PLIMIT;
    let mut val: c_int = 0;
    if kstrtoint(buf, 0, &mut val) < 0 { return -EINVAL as ssize_t; }
    if val == 1 {
        if (*mdata).security_state & CXL_PMEM_SEC_STATE_USER_PASS_SET == 0 { return -ENXIO as ssize_t; }
        (*mdata).security_state |= CXL_PMEM_SEC_STATE_LOCKED;
        (*mdata).security_state &= !mask;
    } else { return -EINVAL as ssize_t; }
    count as ssize_t
}
// DEVICE_ATTR_RW(security_lock);

unsafe extern "C" fn fw_buf_checksum_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    let mut hash = [0u8; SHA256_DIGEST_SIZE];
    sha256((*mdata).fw, (*mdata).fw_size, hash.as_mut_ptr());
    sysfs_emit(buf, c"%*phN\n".as_ptr(), SHA256_DIGEST_SIZE as c_int, hash.as_ptr())
}
// DEVICE_ATTR_RO(fw_buf_checksum);

unsafe extern "C" fn sanitize_timeout_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    sysfs_emit(buf, c"%lu\n".as_ptr(), (*mdata).sanitize_timeout)
}

unsafe extern "C" fn sanitize_timeout_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let mdata = dev_get_drvdata(dev) as *mut cxl_mockmem_data;
    let mut val: c_ulong = 0;
    let rc = kstrtoul(buf, 0, &mut val);
    if rc != 0 { return rc as ssize_t; }
    (*mdata).sanitize_timeout = val;
    count as ssize_t
}
// DEVICE_ATTR_RW(sanitize_timeout);
// ATTRIBUTE_GROUPS(cxl_mock_mem);

static mut cxl_mock_mem_ids: [platform_device_id; 3] = [
    platform_device_id { name: *b"cxl_mem\0\0\0\0\0\0\0\0\0", driver_data: 0 },
    platform_device_id { name: *b"cxl_rcd\0\0\0\0\0\0\0\0\0", driver_data: 1 },
    platform_device_id { name: [0; 16], driver_data: 0 },
];
// MODULE_DEVICE_TABLE(platform, cxl_mock_mem_ids);

static mut cxl_mock_mem_driver: platform_driver = platform_driver {
    probe: Some(cxl_mock_mem_probe),
    id_table: unsafe { cxl_mock_mem_ids.as_ptr() },
    driver: driver {
        name: KBUILD_MODNAME,
        dev_groups: cxl_mock_mem_groups,
        groups: cxl_mock_mem_core_groups,
        probe_type: PROBE_PREFER_ASYNCHRONOUS,
    },
};

// module_platform_driver(cxl_mock_mem_driver);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("cxl_test: mem device mock module");
// MODULE_IMPORT_NS("CXL");

