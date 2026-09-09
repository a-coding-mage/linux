// SPDX-License-Identifier: GPL-2.0-or-later
/* Debugging module statistics. */

// Kernel dependencies supplied by the surrounding translation unit.

static mut DUP_FAILED_MODULES: ListHead = LIST_HEAD_INIT;

pub static mut total_mod_size: atomic_long_t = atomic_long_t::zero();
pub static mut total_text_size: atomic_long_t = atomic_long_t::zero();
pub static mut invalid_kread_bytes: atomic_long_t = atomic_long_t::zero();
pub static mut invalid_decompress_bytes: atomic_long_t = atomic_long_t::zero();
static mut invalid_becoming_bytes: atomic_long_t = atomic_long_t::zero();
static mut invalid_mod_bytes: atomic_long_t = atomic_long_t::zero();
pub static mut modcount: atomic_t = atomic_t::zero();
pub static mut failed_kreads: atomic_t = atomic_t::zero();
pub static mut failed_decompress: atomic_t = atomic_t::zero();
static mut failed_becoming: atomic_t = atomic_t::zero();
static mut failed_load_modules: atomic_t = atomic_t::zero();

unsafe fn mod_fail_to_str(mod_fail: *mut mod_fail_load) -> *const core::ffi::c_char {
    if test_bit(FAIL_DUP_MOD_BECOMING, &(*mod_fail).dup_fail_mask)
        && test_bit(FAIL_DUP_MOD_LOAD, &(*mod_fail).dup_fail_mask)
    {
        return c"Becoming & Load".as_ptr();
    }
    if test_bit(FAIL_DUP_MOD_BECOMING, &(*mod_fail).dup_fail_mask) {
        return c"Becoming".as_ptr();
    }
    if test_bit(FAIL_DUP_MOD_LOAD, &(*mod_fail).dup_fail_mask) {
        return c"Load".as_ptr();
    }
    c"Bug-on-stats".as_ptr()
}

pub unsafe fn mod_stat_bump_invalid(info: *mut load_info, flags: i32) {
    atomic_long_add((*info).len.wrapping_mul(2), &mut invalid_mod_bytes);
    atomic_inc(&mut failed_load_modules);
    // CONFIG_MODULE_DECOMPRESS is a build-time condition from the C source.
    #[cfg(CONFIG_MODULE_DECOMPRESS)]
    if flags & MODULE_INIT_COMPRESSED_FILE != 0 {
        atomic_long_add((*info).compressed_len, &mut invalid_mod_bytes);
    }
}

pub unsafe fn mod_stat_bump_becoming(info: *mut load_info, flags: i32) {
    atomic_inc(&mut failed_becoming);
    atomic_long_add((*info).len, &mut invalid_becoming_bytes);
    #[cfg(CONFIG_MODULE_DECOMPRESS)]
    if flags & MODULE_INIT_COMPRESSED_FILE != 0 {
        atomic_long_add((*info).compressed_len, &mut invalid_becoming_bytes);
    }
}

pub unsafe fn try_add_failed_module(
    name: *const core::ffi::c_char,
    reason: fail_dup_mod_reason,
) -> i32 {
    let mut mod_fail: *mut mod_fail_load;
    list_for_each_entry_rcu!(mod_fail, &DUP_FAILED_MODULES, list, lockdep_is_held(&module_mutex));
    {
        if strcmp((*mod_fail).name.as_ptr(), name) == 0 {
            atomic_long_inc(&mut (*mod_fail).count);
            __set_bit(reason as usize, &mut (*mod_fail).dup_fail_mask);
            return 0;
        }
    }

    mod_fail = kzalloc_obj::<mod_fail_load>();
    if mod_fail.is_null() {
        return -ENOMEM;
    }
    strscpy((*mod_fail).name.as_mut_ptr(), name);
    __set_bit(reason as usize, &mut (*mod_fail).dup_fail_mask);
    atomic_long_inc(&mut (*mod_fail).count);
    list_add_rcu(&mut (*mod_fail).list, &mut DUP_FAILED_MODULES);
    0
}

const MAX_PREAMBLE: usize = 1024;
const MAX_FAILED_MOD_PRINT: u32 = 112;
const MAX_BYTES_PER_MOD: usize = 64;

unsafe fn read_file_mod_stats(
    _file: *mut file,
    user_buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let mut mod_fail: *mut mod_fail_load;
    let mut len: u32;
    let mut count_failed: u32 = 0;
    let live_mod_count = atomic_read(&modcount) as u32;
    let fkreads = atomic_read(&failed_kreads) as u32;
    let fdecompress = atomic_read(&failed_decompress) as u32;
    let fbecoming = atomic_read(&failed_becoming) as u32;
    let floads = atomic_read(&failed_load_modules) as u32;
    let total_size = atomic_long_read(&total_mod_size);
    let text_size = atomic_long_read(&total_text_size);
    let ikread_bytes = atomic_long_read(&invalid_kread_bytes);
    let idecompress_bytes = atomic_long_read(&invalid_decompress_bytes);
    let ibecoming_bytes = atomic_long_read(&invalid_becoming_bytes);
    let imod_bytes = atomic_long_read(&invalid_mod_bytes);
    let total_virtual_lost = ikread_bytes + idecompress_bytes + ibecoming_bytes + imod_bytes;
    let size = MAX_PREAMBLE
        + core::cmp::min(floads + fbecoming, MAX_FAILED_MOD_PRINT) as usize * MAX_BYTES_PER_MOD;
    let buf = kzalloc(size, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM as ssize_t; }

    len = scnprintf(buf, size, c"%25s\t%u\n".as_ptr(), c"Mods ever loaded".as_ptr(), live_mod_count);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%u\n".as_ptr(), c"Mods failed on kread".as_ptr(), fkreads);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%u\n".as_ptr(), c"Mods failed on decompress".as_ptr(), fdecompress);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%u\n".as_ptr(), c"Mods failed on becoming".as_ptr(), fbecoming);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%u\n".as_ptr(), c"Mods failed on load".as_ptr(), floads);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Total module size".as_ptr(), total_size);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Total mod text size".as_ptr(), text_size);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Failed kread bytes".as_ptr(), ikread_bytes);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Failed decompress bytes".as_ptr(), idecompress_bytes);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Failed becoming bytes".as_ptr(), ibecoming_bytes);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Failed kmod bytes".as_ptr(), imod_bytes);
    len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Virtual mem wasted bytes".as_ptr(), total_virtual_lost);
    if live_mod_count != 0 && total_size != 0 { len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Average mod size".as_ptr(), DIV_ROUND_UP(total_size, live_mod_count as _)); }
    if live_mod_count != 0 && text_size != 0 { len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Average mod text size".as_ptr(), DIV_ROUND_UP(text_size, live_mod_count as _)); }
    WARN_ON_ONCE(ikread_bytes != 0 && fkreads == 0);
    if fkreads != 0 && ikread_bytes != 0 { len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Avg fail kread bytes".as_ptr(), DIV_ROUND_UP(ikread_bytes, fkreads as _)); }
    WARN_ON_ONCE(ibecoming_bytes != 0 && fbecoming == 0);
    if fbecoming != 0 && ibecoming_bytes != 0 { len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Avg fail becoming bytes".as_ptr(), DIV_ROUND_UP(ibecoming_bytes, fbecoming as _)); }
    WARN_ON_ONCE(idecompress_bytes != 0 && fdecompress == 0);
    if fdecompress != 0 && idecompress_bytes != 0 { len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Avg fail decomp bytes".as_ptr(), DIV_ROUND_UP(idecompress_bytes, fdecompress as _)); }
    WARN_ON_ONCE(imod_bytes != 0 && floads == 0);
    if floads != 0 && imod_bytes != 0 { len += scnprintf(buf.add(len as usize), size - len as usize, c"%25s\t%lu\n".as_ptr(), c"Average fail load bytes".as_ptr(), DIV_ROUND_UP(imod_bytes, floads as _)); }
    WARN_ON_ONCE(len >= MAX_PREAMBLE as u32);
    if !list_empty(&DUP_FAILED_MODULES) {
        len += scnprintf(buf.add(len as usize), size - len as usize, c"Duplicate failed modules:\n".as_ptr());
        mutex_lock(&mut module_mutex);
        list_for_each_entry_rcu!(mod_fail, &DUP_FAILED_MODULES, list);
        { if WARN_ON_ONCE({ count_failed += 1; count_failed >= MAX_FAILED_MOD_PRINT }) { mutex_unlock(&mut module_mutex); kfree(buf); return simple_read_from_buffer(user_buf, count, ppos, buf, len); } }
        mutex_unlock(&mut module_mutex);
    }
    let ret = simple_read_from_buffer(user_buf, count, ppos, buf, len);
    kfree(buf);
    ret
}

static fops_mod_stats: file_operations = file_operations { read: Some(read_file_mod_stats), open: Some(simple_open), owner: THIS_MODULE, llseek: Some(default_llseek) };

unsafe fn module_stats_init() -> i32 {
    debugfs_create_file(c"stats".as_ptr(), 0o400, mod_debugfs_root, mod_debugfs_root, &fops_mod_stats);
    0
}

module_init!(module_stats_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
