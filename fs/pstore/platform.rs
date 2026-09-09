// SPDX-License-Identifier: GPL-2.0-only
/* Persistent Storage - platform driver interface parts. */

// Linux kernel dependencies and configuration symbols are supplied by other
// translated units.

static mut PSTORE_UPDATE_MS: i32 = -1;
static PSTORE_TYPE_NAMES: [&[u8]; 9] = [
    b"dmesg\0", b"mce\0", b"console\0", b"ftrace\0", b"rtas\0",
    b"powerpc-ofw\0", b"powerpc-common\0", b"pmsg\0", b"powerpc-opal\0",
];

static mut PSTORE_NEW_ENTRY: i32 = 0;
static mut PSINFO: *mut PstoreInfo = core::ptr::null_mut();
static mut BACKEND: *mut core::ffi::c_char = core::ptr::null_mut();
static mut COMPRESS: *mut core::ffi::c_char = b"deflate\0".as_ptr() as *mut _;
static mut KMSG_BYTES: u32 = CONFIG_PSTORE_DEFAULT_KMSG_BYTES;
static mut COMPRESS_WORKSPACE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut BIG_OOPS_BUF: *mut core::ffi::c_char = core::ptr::null_mut();
static mut MAX_COMPRESSED_SIZE: usize = 0;
static mut OOPSCOUNT: i32 = 0;

extern "C" {
    static mut PSINFO_LOCK: Mutex;
    static mut PSTORE_TIMER: TimerList;
    static mut PSTORE_WORK: WorkStruct;
    static mut PSTORE_DUMPER: KmsgDumper;
    fn pstore_dowork(work: *mut WorkStruct);
    fn pstore_timefunc(timer: *mut TimerList);
    fn pstore_get_records(n: i32);
    fn pstore_mkfile(root: *mut Dentry, record: *mut PstoreRecord) -> i32;
    fn pstore_put_backend_records(psi: *mut PstoreInfo);
    fn pstore_init_fs() -> i32;
    fn pstore_exit_fs();
    fn pstore_register_ftrace();
    fn pstore_unregister_ftrace();
    fn pstore_register_pmsg();
    fn pstore_unregister_pmsg();
}

pub unsafe fn pstore_set_kmsg_bytes(bytes: u32) { KMSG_BYTES = bytes; }

pub unsafe extern "C" fn pstore_type_to_name(type_: PstoreTypeId) -> *const core::ffi::c_char {
    if type_ >= PSTORE_TYPE_MAX { return b"unknown\0".as_ptr() as *const _; }
    PSTORE_TYPE_NAMES[type_ as usize].as_ptr() as *const _
}

pub unsafe extern "C" fn pstore_name_to_type(name: *const core::ffi::c_char) -> PstoreTypeId {
    for i in 0..PSTORE_TYPE_MAX {
        if strcmp(PSTORE_TYPE_NAMES[i as usize].as_ptr() as *const _, name) == 0 { return i; }
    }
    PSTORE_TYPE_MAX
}

unsafe fn pstore_timer_kick() {
    if PSTORE_UPDATE_MS < 0 { return; }
    mod_timer(&mut PSTORE_TIMER, jiffies() + msecs_to_jiffies(PSTORE_UPDATE_MS as u64));
}

unsafe fn pstore_cannot_block_path(reason: KmsgDumpReason) -> bool {
    if in_nmi() { return true; }
    match reason { KMSG_DUMP_PANIC | KMSG_DUMP_EMERG => true, _ => false }
}

unsafe fn pstore_compress(in_: *const core::ffi::c_void, out: *mut core::ffi::c_void, inlen: u32, outlen: u32) -> i32 {
    if !IS_ENABLED_CONFIG_PSTORE_COMPRESS { return -EINVAL; }
    let mut zstream = ZStream { next_in: in_, avail_in: inlen, next_out: out, avail_out: outlen, workspace: COMPRESS_WORKSPACE, ..core::mem::zeroed() };
    if zlib_deflate_init2(&mut zstream, Z_DEFAULT_COMPRESSION, Z_DEFLATED, -MAX_WBITS, DEF_MEM_LEVEL, Z_DEFAULT_STRATEGY) != Z_OK { return -EINVAL; }
    if zlib_deflate(&mut zstream, Z_FINISH) != Z_STREAM_END { return -EINVAL; }
    let ret = zlib_deflate_end(&mut zstream);
    if ret != Z_OK { pr_warn_once!("zlib_deflateEnd() failed: %d\n", ret); }
    zstream.total_out as i32
}

unsafe fn allocate_buf_for_compression() {
    if !IS_ENABLED_CONFIG_PSTORE_COMPRESS || COMPRESS.is_null() || strcmp(COMPRESS, b"none\0".as_ptr() as *const _) == 0 { COMPRESS = core::ptr::null_mut(); return; }
    if strcmp(COMPRESS, b"deflate\0".as_ptr() as *const _) != 0 { pr_err!("Unsupported compression, falling back to deflate\n"); COMPRESS = b"deflate\0".as_ptr() as *mut _; }
    let size = ((*PSINFO).bufsize * 100) / DMESG_COMP_PERCENT;
    let buf = kvzalloc(size, GFP_KERNEL) as *mut _;
    if buf.is_null() { pr_err!("Failed compression buffer allocation\n"); return; }
    COMPRESS_WORKSPACE = vmalloc(zlib_deflate_workspacesize(MAX_WBITS, DEF_MEM_LEVEL));
    if COMPRESS_WORKSPACE.is_null() { kvfree(buf as *mut _); return; }
    BIG_OOPS_BUF = buf; MAX_COMPRESSED_SIZE = size;
}

unsafe fn free_buf_for_compression() {
    if IS_ENABLED_CONFIG_PSTORE_COMPRESS && !COMPRESS_WORKSPACE.is_null() { vfree(COMPRESS_WORKSPACE); COMPRESS_WORKSPACE = core::ptr::null_mut(); }
    kvfree(BIG_OOPS_BUF as *mut _); BIG_OOPS_BUF = core::ptr::null_mut(); MAX_COMPRESSED_SIZE = 0;
}

pub unsafe fn pstore_record_init(record: *mut PstoreRecord, psinfo: *mut PstoreInfo) {
    core::ptr::write_bytes(record, 0, 1); (*record).psi = psinfo; (*record).time = ns_to_timespec64(ktime_get_real_fast_ns());
}

unsafe fn pstore_register_kmsg() { kmsg_dump_register(&mut PSTORE_DUMPER); }
unsafe fn pstore_unregister_kmsg() { kmsg_dump_unregister(&mut PSTORE_DUMPER); }

pub unsafe extern "C" fn pstore_register(psi: *mut PstoreInfo) -> i32 {
    if !BACKEND.is_null() && strcmp(BACKEND, (*psi).name) != 0 { return -EBUSY; }
    if (*psi).flags == 0 || (*psi).read.is_none() || (*psi).write.is_none() { return -EINVAL; }
    let new_backend = kstrdup((*psi).name, GFP_KERNEL); if new_backend.is_null() { return -ENOMEM; }
    mutex_lock(&mut PSINFO_LOCK);
    if !PSINFO.is_null() { mutex_unlock(&mut PSINFO_LOCK); kfree(new_backend); return -EBUSY; }
    if (*psi).write_user.is_none() { (*psi).write_user = Some(pstore_write_user_compat); }
    PSINFO = psi; mutex_init(&mut (*psi).read_mutex); raw_spin_lock_init(&mut (*psi).buf_lock);
    if (*psi).flags & PSTORE_FLAGS_DMESG != 0 { allocate_buf_for_compression(); }
    pstore_get_records(0);
    if (*psi).flags & PSTORE_FLAGS_DMESG != 0 { PSTORE_DUMPER.max_reason = (*psi).max_reason; pstore_register_kmsg(); }
    if (*psi).flags & PSTORE_FLAGS_FTRACE != 0 { pstore_register_ftrace(); }
    if (*psi).flags & PSTORE_FLAGS_PMSG != 0 { pstore_register_pmsg(); }
    pstore_timer_kick(); BACKEND = new_backend; mutex_unlock(&mut PSINFO_LOCK); 0
}

pub unsafe extern "C" fn pstore_unregister(psi: *mut PstoreInfo) {
    if psi.is_null() { return; } mutex_lock(&mut PSINFO_LOCK);
    if psi != PSINFO { mutex_unlock(&mut PSINFO_LOCK); return; }
    if (*psi).flags & PSTORE_FLAGS_PMSG != 0 { pstore_unregister_pmsg(); }
    if (*psi).flags & PSTORE_FLAGS_FTRACE != 0 { pstore_unregister_ftrace(); }
    if (*psi).flags & PSTORE_FLAGS_DMESG != 0 { pstore_unregister_kmsg(); }
    timer_delete_sync(&mut PSTORE_TIMER); flush_work(&mut PSTORE_WORK); pstore_put_backend_records(psi);
    free_buf_for_compression(); PSINFO = core::ptr::null_mut(); kfree(BACKEND); BACKEND = core::ptr::null_mut(); mutex_unlock(&mut PSINFO_LOCK);
}

unsafe fn pstore_write_user_compat(record: *mut PstoreRecord, buf: *const core::ffi::c_void) -> i32 {
    if !(*record).buf.is_null() { return -EINVAL; }
    (*record).buf = vmemdup_user(buf, (*record).size); if is_err((*record).buf) { let r = ptr_err((*record).buf); (*record).buf = core::ptr::null_mut(); return r; }
    let ret = ((*(*record).psi).write.unwrap())(record); kvfree((*record).buf); (*record).buf = core::ptr::null_mut(); if ret < 0 { ret } else { (*record).size as i32 }
}

pub unsafe fn pstore_get_backend_records(psi: *mut PstoreInfo, root: *mut Dentry, quiet: i32) {
    if psi.is_null() || root.is_null() { return; }
    let mut zstream: ZStream = core::mem::zeroed(); let mut stop_loop = 65536u32; let mut failed = 0;
    mutex_lock(&mut (*psi).read_mutex); if let Some(open) = (*psi).open { if open(psi) != 0 { mutex_unlock(&mut (*psi).read_mutex); return; } }
    while stop_loop != 0 { stop_loop -= 1; let record = kzalloc_obj::<PstoreRecord>(); if record.is_null() { break; } pstore_record_init(record, psi); (*record).size = ((*psi).read.unwrap())(record); if (*record).size <= 0 { kfree(record); break; } let rc = pstore_mkfile(root, record); if rc != 0 { kvfree((*record).buf); kfree((*record).priv_); kfree(record); if rc != -EEXIST || quiet == 0 { failed += 1; } } }
    if let Some(close) = (*psi).close { close(psi); } mutex_unlock(&mut (*psi).read_mutex); kvfree(zstream.workspace);
}

const DMESG_COMP_PERCENT: usize = 60;

unsafe fn pstore_dump(_dumper: *mut KmsgDumper, detail: *mut KmsgDumpDetail) {
    let mut iter: KmsgDumpIter = core::mem::zeroed(); let remaining = KMSG_BYTES as u64; let mut total = 0u64; let mut part = 1u32; let mut saved_ret = 0i32; let mut flags = 0ul;
    let why = kmsg_dump_reason_str((*detail).reason); let psi = &mut *PSINFO;
    if pstore_cannot_block_path((*detail).reason) { if !raw_spin_trylock_irqsave(&mut psi.buf_lock, &mut flags) { return; } } else { raw_spin_lock_irqsave(&mut psi.buf_lock, &mut flags); }
    kmsg_dump_rewind(&mut iter); OOPSCOUNT += 1;
    while total < remaining { let mut record: PstoreRecord = core::mem::zeroed(); pstore_record_init(&mut record, PSINFO); record.type_ = PSTORE_TYPE_DMESG; record.count = OOPSCOUNT; record.reason = (*detail).reason; record.part = part; record.buf = psi.buf;
        let dst = if !BIG_OOPS_BUF.is_null() { BIG_OOPS_BUF } else { psi.buf }; let dst_size = if MAX_COMPRESSED_SIZE != 0 { MAX_COMPRESSED_SIZE } else { psi.bufsize }; let header = snprintf(dst, dst_size, why, OOPSCOUNT, part); let mut dump_size = 0usize;
        if !kmsg_dump_get_buffer(&mut iter, true, dst.add(header), dst_size - header, &mut dump_size) { break; }
        if !BIG_OOPS_BUF.is_null() { let zipped = pstore_compress(dst as *const _, psi.buf as *mut _, (header + dump_size) as u32, psi.bufsize as u32); if zipped > 0 { record.compressed = true; record.size = zipped as usize; } else { record.size = psi.bufsize; core::ptr::copy_nonoverlapping(dst, psi.buf, psi.bufsize); } } else { record.size = header + dump_size; }
        let ret = (psi.write.unwrap())(&mut record); if ret == 0 && (*detail).reason == KMSG_DUMP_OOPS { PSTORE_NEW_ENTRY = 1; pstore_timer_kick(); } else if saved_ret == 0 { saved_ret = ret; } total += record.size; part += 1;
    }
    raw_spin_unlock_irqrestore(&mut psi.buf_lock, flags);
}

#[cfg(CONFIG_PSTORE_CONSOLE)]
unsafe fn pstore_console_write(_con: *mut Console, s: *const core::ffi::c_char, c: u32) { if c == 0 { return; } let mut r: PstoreRecord = core::mem::zeroed(); pstore_record_init(&mut r, PSINFO); r.type_ = PSTORE_TYPE_CONSOLE; r.buf = s as *mut _; r.size = c as usize; ((*PSINFO).write.unwrap())(&mut r); }

unsafe fn decompress_record(record: *mut PstoreRecord, zstream: *mut ZStream) {
    if !IS_ENABLED_CONFIG_PSTORE_COMPRESS || !(*record).compressed || (*record).type_ != PSTORE_TYPE_DMESG || (*zstream).workspace.is_null() { return; }
    if zlib_inflate_reset(zstream) != Z_OK { return; }
    let max = 3 * (*PSINFO).bufsize; let workspace = kvzalloc(max + (*record).ecc_notice_size, GFP_KERNEL) as *mut u8; if workspace.is_null() { return; }
    (*zstream).next_in = (*record).buf; (*zstream).avail_in = (*record).size as u32; (*zstream).next_out = workspace as *mut _; (*zstream).avail_out = max as u32;
    if zlib_inflate(zstream, Z_FINISH) != Z_STREAM_END { kvfree(workspace as *mut _); return; }
    let len = (*zstream).total_out as usize; core::ptr::copy_nonoverlapping((*record).buf.add((*record).size), workspace.add(len), (*record).ecc_notice_size); let unzipped = kvmemdup(workspace, len + (*record).ecc_notice_size, GFP_KERNEL); kvfree(workspace as *mut _); if unzipped.is_null() { return; }
    kvfree((*record).buf); (*record).buf = unzipped; (*record).size = len; (*record).compressed = false;
}

unsafe extern "C" fn pstore_init() -> i32 { let ret = pstore_init_fs(); if ret != 0 { free_buf_for_compression(); } ret }
unsafe extern "C" fn pstore_exit() { pstore_exit_fs(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
