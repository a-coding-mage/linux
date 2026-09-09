// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test the function and performance of kallsyms
 *
 * Copyright (C) Huawei Technologies Co., Ltd., 2022
 *
 * Authors: Zhen Lei <thunder.leizhen@huawei.com> Huawei
 */

// C dependencies supplied by the kernel and sibling translation units are intentionally external.

const MAX_NUM_OF_RECORDS: usize = 64;

#[repr(C)]
pub struct test_stat {
    pub min: i32,
    pub max: i32,
    pub save_cnt: i32,
    pub real_cnt: i32,
    pub perf: i32,
    pub sum: u64,
    pub name: *mut core::ffi::c_char,
    pub addr: usize,
    pub addrs: [usize; MAX_NUM_OF_RECORDS],
}

#[repr(C)]
pub struct test_item {
    pub name: *mut core::ffi::c_char,
    pub addr: usize,
}

extern "C" {
    static mut kallsyms_test_var_bss: i32;
    static mut kallsyms_test_var_data: i32;
    fn vmalloc_noprof();
    fn vfree(ptr: *mut core::ffi::c_void);
    fn kmalloc_objs<T>(n: usize) -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kallsyms_lookup_name(name: *const core::ffi::c_char) -> usize;
    fn kallsyms_on_each_symbol(
        callback: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_char, usize) -> i32,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn kallsyms_on_each_match_symbol(
        callback: unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> i32,
        name: *const core::ffi::c_char,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn ktime_get_ns() -> u64;
    fn lookup_symbol_name(addr: usize, name: *mut core::ffi::c_char) -> i32;
    fn kallsyms_sym_address(index: u32) -> usize;
    fn is_ksym_addr(addr: usize) -> bool;
    fn get_random_bytes(buf: *mut core::ffi::c_void, len: usize);
    fn schedule_timeout(timeout: i64) -> i64;
    fn kthread_run_on_cpu(
        threadfn: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
        data: *mut core::ffi::c_void,
        cpu: i32,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
    static mut kallsyms_num_syms: u32;
    static mut kallsyms_names: *const u8;
    static mut kallsyms_token_index: *const u16;
    static mut kallsyms_token_table: *const core::ffi::c_char;
    static mut system_state: i32;
}

static mut kallsyms_test_var_bss_static: i32 = 0;
static mut kallsyms_test_var_data_static: i32 = 1;
pub static mut kallsyms_test_var_bss: i32 = 0;
pub static mut kallsyms_test_var_data: i32 = 1;

unsafe extern "C" fn kallsyms_test_func_static() -> i32 {
    kallsyms_test_var_bss_static += 1;
    kallsyms_test_var_data_static += 1;
    0
}

pub unsafe extern "C" fn kallsyms_test_func() -> i32 { kallsyms_test_func_static() }

#[linkage = "weak"]
pub unsafe extern "C" fn kallsyms_test_func_weak() -> i32 {
    kallsyms_test_var_bss += 1;
    kallsyms_test_var_data += 1;
    0
}

static mut test_items: [test_item; 5] = [
    test_item { name: b"kallsyms_test_func_static\0" as *const _ as *mut _, addr: kallsyms_test_func_static as usize },
    test_item { name: b"kallsyms_test_func\0" as *const _ as *mut _, addr: kallsyms_test_func as usize },
    test_item { name: b"kallsyms_test_func_weak\0" as *const _ as *mut _, addr: kallsyms_test_func_weak as usize },
    test_item { name: b"vmalloc_noprof\0" as *const _ as *mut _, addr: vmalloc_noprof as usize },
    test_item { name: b"vfree\0" as *const _ as *mut _, addr: vfree as usize },
];

static mut stub_name: [core::ffi::c_char; 256] = [0; 256];

unsafe extern "C" fn stat_symbol_len(data: *mut core::ffi::c_void, name: *const core::ffi::c_char, _addr: usize) -> i32 {
    *(data as *mut u32) += libc::strlen(name) as u32;
    0
}

unsafe extern "C" fn lookup_name(data: *mut core::ffi::c_void, name: *const core::ffi::c_char, _addr: usize) -> i32 {
    let stat = &mut *(data as *mut test_stat);
    let t0 = ktime_get_ns(); let _ = kallsyms_lookup_name(name); let t = ktime_get_ns() - t0;
    if t < stat.min as u64 { stat.min = t as i32; }
    if t > stat.max as u64 { stat.max = t as i32; }
    stat.real_cnt += 1; stat.sum += t; 0
}

unsafe extern "C" fn find_symbol(data: *mut core::ffi::c_void, name: *const core::ffi::c_char, addr: usize) -> i32 {
    let stat = &mut *(data as *mut test_stat);
    if libc::strcmp(name, stat.name) == 0 {
        stat.real_cnt += 1; stat.addr = addr;
        if stat.save_cnt < MAX_NUM_OF_RECORDS as i32 { stat.addrs[stat.save_cnt as usize] = addr; stat.save_cnt += 1; }
        if stat.real_cnt == stat.max { return 1; }
    } 0
}

unsafe extern "C" fn match_symbol(data: *mut core::ffi::c_void, addr: usize) -> i32 {
    let stat = &mut *(data as *mut test_stat);
    stat.real_cnt += 1; stat.addr = addr;
    if stat.save_cnt < MAX_NUM_OF_RECORDS as i32 { stat.addrs[stat.save_cnt as usize] = addr; stat.save_cnt += 1; }
    if stat.real_cnt == stat.max { 1 } else { 0 }
}

unsafe fn test_kallsyms_compression_ratio() {
    let mut total_len = 0u32; kallsyms_on_each_symbol(stat_symbol_len, &mut total_len as *mut _ as *mut _);
    libc::memset(stub_name.as_mut_ptr() as *mut _, b'4' as i32, stub_name.len());
    let pos = total_len / kallsyms_num_syms; stub_name[pos as usize] = 0;
    let (mut pos, mut num, mut off) = (0u32, 0u32, 0u32);
    while pos < kallsyms_num_syms { let mut len = *kallsyms_names.add(off as usize) as u32; num += 1; off += 1; pos += 1; if len & 0x80 != 0 { len = (len & 0x7f) | ((*kallsyms_names.add(off as usize) as u32) << 7); num += 1; off += 1; } off += len; }
    let mut total_size = off - num; let p = *kallsyms_token_index.add(0xff) as u32; total_size += p + libc::strlen(kallsyms_token_table.add(p as usize)) as u32 + 1 + 0x100 * 2;
    let _ = total_size; // pr_info output and kernel integer helpers remain external.
}

unsafe fn test_perf_kallsyms_lookup_name() { let mut stat: test_stat = core::mem::zeroed(); stat.min = i32::MAX; kallsyms_on_each_symbol(lookup_name, &mut stat as *mut _ as *mut _); }
unsafe fn test_perf_kallsyms_on_each_symbol() { let mut stat: test_stat = core::mem::zeroed(); stat.max = i32::MAX; stat.name = stub_name.as_mut_ptr(); stat.perf = 1; let _ = kallsyms_on_each_symbol(find_symbol, &mut stat as *mut _ as *mut _); }
unsafe fn test_perf_kallsyms_on_each_match_symbol() { let mut stat: test_stat = core::mem::zeroed(); stat.max = i32::MAX; stat.name = stub_name.as_mut_ptr(); let _ = kallsyms_on_each_match_symbol(match_symbol, stat.name, &mut stat as *mut _ as *mut _); }

unsafe fn test_kallsyms_basic_function() -> i32 {
    let stat = kmalloc_objs::<test_stat>(2); if stat.is_null() { return -12; }
    let stat2 = stat.add(1); let mut namebuf = [0i8; 256]; let mut next = 0u32;
    for i in 0..test_items.len() {
        let item = &test_items[i]; let addr = kallsyms_lookup_name(item.name);
        if addr != item.addr { kfree(stat as *mut _); return -3; }
        core::ptr::write_bytes(stat, 0, 1); (*stat).max = i32::MAX; (*stat).name = item.name;
        kallsyms_on_each_symbol(find_symbol, stat as *mut _ as *mut _);
        if (*stat).addr != item.addr || (*stat).real_cnt != 1 { kfree(stat as *mut _); return -3; }
        core::ptr::write_bytes(stat, 0, 1); (*stat).max = i32::MAX; (*stat).name = item.name;
        kallsyms_on_each_match_symbol(match_symbol, item.name, stat as *mut _ as *mut _);
        if (*stat).addr != item.addr || (*stat).real_cnt != 1 { kfree(stat as *mut _); return -3; }
    }
    for i in 0..kallsyms_num_syms {
        let addr = kallsyms_sym_address(i); if !is_ksym_addr(addr) { continue; }
        if lookup_symbol_name(addr, namebuf.as_mut_ptr()) != 0 { kfree(stat as *mut _); return -3; }
        let lookup_addr = kallsyms_lookup_name(namebuf.as_ptr());
        core::ptr::write_bytes(stat, 0, 1); (*stat).max = i32::MAX;
        kallsyms_on_each_match_symbol(match_symbol, namebuf.as_ptr(), stat as *mut _ as *mut _);
        if i >= next {
            core::ptr::write_bytes(stat2, 0, 1); (*stat2).max = i32::MAX; (*stat2).name = namebuf.as_mut_ptr();
            kallsyms_on_each_symbol(find_symbol, stat2 as *mut _ as *mut _);
            if (*stat).addr != (*stat2).addr || (*stat).real_cnt != (*stat2).real_cnt { kfree(stat as *mut _); return -3; }
            let mut rand = 0u16; get_random_bytes(&mut rand as *mut _ as *mut _, 2); next = i + (rand as u32 & 0xff) + 1;
        }
        if (*stat).real_cnt == 0 || lookup_addr == 0 || lookup_addr != (*stat).addrs[0] { kfree(stat as *mut _); return -3; }
        if (*stat).real_cnt <= MAX_NUM_OF_RECORDS as i32 { let mut found = false; for j in 0..(*stat).save_cnt as usize { if (*stat).addrs[j] == addr { found = true; break; } } if !found { kfree(stat as *mut _); return -3; } }
    }
    kfree(stat as *mut _); 0
}

#[no_mangle]
pub unsafe extern "C" fn kallsyms_test_init() -> i32 {
    let t = kthread_run_on_cpu(test_entry, core::ptr::null_mut(), 0, b"kallsyms_test\0".as_ptr() as *const _);
    if t.is_null() { return -1; } 0
}

unsafe extern "C" fn test_entry(_p: *mut core::ffi::c_void) -> i32 {
    if test_kallsyms_basic_function() != 0 { return 0; }
    test_kallsyms_compression_ratio(); test_perf_kallsyms_lookup_name(); test_perf_kallsyms_on_each_symbol(); test_perf_kallsyms_on_each_match_symbol(); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
