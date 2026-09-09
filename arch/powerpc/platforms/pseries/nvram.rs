// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  c 2001 PPC 64 Team, IBM Corp
 *
 * /dev/nvram driver for PPC64
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

const NVRW_CNT: usize = 0x20;

static mut nvram_size: c_uint = 0;
static mut nvram_fetch: c_int = 0;
static mut nvram_store: c_int = 0;
static mut nvram_buf: [c_char; NVRW_CNT] = [0; NVRW_CNT];
static mut nvram_lock: spinlock_t = DEFINE_SPINLOCK!();

const NVRAM_RTAS_READ_TIMEOUT: time64_t = 5;
static mut last_unread_rtas_event: time64_t = 0;

#[cfg(CONFIG_PSTORE)]
pub static mut last_rtas_event: time64_t = 0;

unsafe fn pSeries_nvram_read(buf: *mut c_char, count: size_t, index: *mut loff_t) -> ssize_t {
    let mut i: c_uint;
    let mut len: c_ulong;
    let mut done: c_int = 0;
    let mut flags: c_ulong = 0;
    let mut p = buf;

    if nvram_size == 0 || nvram_fetch == RTAS_UNKNOWN_SERVICE {
        return -ENODEV as ssize_t;
    }
    if *index >= nvram_size as loff_t {
        return 0;
    }
    i = *index as c_uint;
    let mut remaining = if i as size_t + count > nvram_size as size_t {
        nvram_size as size_t - i as size_t
    } else { count };

    spin_lock_irqsave(&raw mut nvram_lock, &mut flags);
    while remaining != 0 {
        len = remaining as c_ulong;
        if len > NVRW_CNT as c_ulong { len = NVRW_CNT as c_ulong; }
        if rtas_call(nvram_fetch, 3, 2, &mut done, i, __pa(nvram_buf.as_mut_ptr()), len) != 0 || len != done as c_ulong {
            spin_unlock_irqrestore(&raw mut nvram_lock, flags);
            return -EIO as ssize_t;
        }
        memcpy(p as *mut c_void, nvram_buf.as_ptr() as *const c_void, len as size_t);
        p = p.add(len as usize);
        i = i.wrapping_add(len as c_uint);
        remaining -= len as size_t;
    }
    spin_unlock_irqrestore(&raw mut nvram_lock, flags);
    *index = i as loff_t;
    p.offset_from(buf) as ssize_t
}

unsafe fn pSeries_nvram_write(buf: *mut c_char, count: size_t, index: *mut loff_t) -> ssize_t {
    let mut i: c_uint;
    let mut len: c_ulong;
    let mut done: c_int = 0;
    let mut flags: c_ulong = 0;
    let mut p = buf as *const c_char;
    if nvram_size == 0 || nvram_store == RTAS_UNKNOWN_SERVICE { return -ENODEV as ssize_t; }
    if *index >= nvram_size as loff_t { return 0; }
    i = *index as c_uint;
    let mut remaining = if i as size_t + count > nvram_size as size_t { nvram_size as size_t - i as size_t } else { count };
    spin_lock_irqsave(&raw mut nvram_lock, &mut flags);
    while remaining != 0 {
        len = remaining as c_ulong;
        if len > NVRW_CNT as c_ulong { len = NVRW_CNT as c_ulong; }
        memcpy(nvram_buf.as_mut_ptr() as *mut c_void, p as *const c_void, len as size_t);
        if rtas_call(nvram_store, 3, 2, &mut done, i, __pa(nvram_buf.as_mut_ptr()), len) != 0 || len != done as c_ulong {
            spin_unlock_irqrestore(&raw mut nvram_lock, flags);
            return -EIO as ssize_t;
        }
        p = p.add(len as usize); i = i.wrapping_add(len as c_uint); remaining -= len as size_t;
    }
    spin_unlock_irqrestore(&raw mut nvram_lock, flags);
    *index = i as loff_t;
    p.offset_from(buf as *const c_char) as ssize_t
}

unsafe fn pSeries_nvram_get_size() -> ssize_t { if nvram_size != 0 { nvram_size as ssize_t } else { -ENODEV as ssize_t } }

pub unsafe fn nvram_write_error_log(buff: *mut c_char, length: c_int, err_type: c_uint, error_log_cnt: c_uint) -> c_int {
    let rc = nvram_write_os_partition(&raw mut rtas_log_partition, buff, length, err_type, error_log_cnt);
    if rc == 0 {
        last_unread_rtas_event = ktime_get_real_seconds();
        #[cfg(CONFIG_PSTORE)] { last_rtas_event = ktime_get_real_seconds(); }
    }
    rc
}

pub unsafe fn nvram_read_error_log(buff: *mut c_char, length: c_int, err_type: *mut c_uint, error_log_cnt: *mut c_uint) -> c_int {
    nvram_read_partition(&raw mut rtas_log_partition, buff, length, err_type, error_log_cnt)
}

pub unsafe fn nvram_clear_error_log() -> c_int {
    let mut tmp_index: loff_t;
    let mut clear_word: c_int = ERR_FLAG_ALREADY_LOGGED;
    let rc: c_int;
    if rtas_log_partition.index == -1 { return -1; }
    tmp_index = rtas_log_partition.index;
    rc = ((*ppc_md).nvram_write.unwrap())(&mut clear_word as *mut c_int as *mut c_char, core::mem::size_of::<c_int>(), &mut tmp_index);
    if rc <= 0 { printk!(KERN_ERR, "nvram_clear_error_log: Failed nvram_write (%d)\n", rc); return rc; }
    last_unread_rtas_event = 0;
    0
}

pub unsafe fn clobbering_unread_rtas_event() -> c_int {
    if oops_log_partition.index == rtas_log_partition.index && last_unread_rtas_event != 0 && ktime_get_real_seconds() - last_unread_rtas_event <= NVRAM_RTAS_READ_TIMEOUT { 1 } else { 0 }
}

unsafe fn pseries_nvram_init_log_partitions() -> c_int {
    nvram_scan_partitions();
    let rc = nvram_init_os_partition(&raw mut rtas_log_partition);
    nvram_init_oops_partition(rc == 0);
    0
}

machine_arch_initcall!(pseries, pseries_nvram_init_log_partitions);

pub unsafe fn pSeries_nvram_init() -> c_int {
    let nvram: *mut device_node;
    let nbytes_p: *const __be32;
    let mut proplen: c_uint = 0;
    nvram = of_find_node_by_type(core::ptr::null_mut(), c"nvram".as_ptr());
    if nvram.is_null() { return -ENODEV as c_int; }
    nbytes_p = of_get_property(nvram, c"#bytes".as_ptr(), &mut proplen);
    if nbytes_p.is_null() || proplen as usize != core::mem::size_of::<c_uint>() { of_node_put(nvram); return -EIO as c_int; }
    nvram_size = be32_to_cpup(nbytes_p);
    nvram_fetch = rtas_function_token(RTAS_FN_NVRAM_FETCH);
    nvram_store = rtas_function_token(RTAS_FN_NVRAM_STORE);
    printk!(KERN_INFO, "PPC64 nvram contains %d bytes\n", nvram_size);
    of_node_put(nvram);
    (*ppc_md).nvram_read = Some(pSeries_nvram_read);
    (*ppc_md).nvram_write = Some(pSeries_nvram_write);
    (*ppc_md).nvram_size = Some(pSeries_nvram_get_size);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
