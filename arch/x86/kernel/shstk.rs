// SPDX-License-Identifier: GPL-2.0
/*
 * shstk.c - Intel shadow stack support
 *
 * Copyright (c) 2021, Intel Corporation.
 * Yu-cheng Yu <yu-cheng.yu@intel.com>
 */

// Kernel dependencies are supplied by the surrounding translation unit.

const SS_FRAME_SIZE: usize = 8;

unsafe fn features_enabled(features: c_ulong) -> bool {
    ((*current).thread.features & features) != 0
}

unsafe fn features_set(features: c_ulong) {
    (*current).thread.features |= features;
}

unsafe fn features_clr(features: c_ulong) {
    (*current).thread.features &= !features;
}

/*
 * Create a restore token on the shadow stack.  A token is always 8-byte
 * and aligned to 8.
 */
unsafe fn create_rstor_token(mut ssp: c_ulong, token_addr: *mut c_ulong) -> c_int {
    let addr: c_ulong;

    /* Token must be aligned */
    if !is_aligned(ssp, 8) {
        return -EINVAL;
    }

    addr = ssp - SS_FRAME_SIZE as c_ulong;

    /*
     * SSP is aligned, so reserved bits and mode bit are a zero, just mark
     * the token 64-bit.
     */
    ssp |= BIT(0);

    if write_user_shstk_64(addr as *mut u64, ssp as u64) != 0 {
        return -EFAULT;
    }

    if !token_addr.is_null() {
        *token_addr = addr;
    }

    0
}

/*
 * VM_SHADOW_STACK will have a guard page. This helps userspace protect
 * itself from attacks. The reasoning is as follows:
 *
 * The shadow stack pointer(SSP) is moved by CALL, RET, and INCSSPQ. The
 * INCSSP instruction can increment the shadow stack pointer. It is the
 * shadow stack analog of an instruction like:
 *
 *   addq $0x80, %rsp
 *
 * However, there is one important difference between an ADD on %rsp
 * and INCSSP. In addition to modifying SSP, INCSSP also reads from the
 * memory of the first and last elements that were "popped". It can be
 * thought of as acting like this:
 *
 * READ_ONCE(ssp);       // read+discard top element on stack
 * ssp += nr_to_pop * 8; // move the shadow stack
 * READ_ONCE(ssp-8);     // read+discard last popped stack element
 *
 * The maximum distance INCSSP can move the SSP is 2040 bytes, before
 * it would read the memory. Therefore a single page gap will be enough
 * to prevent any operation from shifting the SSP to an adjacent stack,
 * since it would have to land in the gap at least once, causing a
 * fault.
 */
unsafe fn alloc_shstk(addr: c_ulong, size: c_ulong, token_offset: c_ulong, set_res_tok: bool) -> c_ulong {
    let mapped_addr = vm_mmap_shadow_stack(addr, size, MAP_ABOVE4G);

    if !set_res_tok || is_err_value(mapped_addr) {
        return mapped_addr;
    }

    if create_rstor_token(mapped_addr + token_offset, core::ptr::null_mut()) != 0 {
        vm_munmap(mapped_addr, size);
        return -EINVAL as c_ulong;
    }

    mapped_addr
}

unsafe fn adjust_shstk_size(size: c_ulong) -> c_ulong {
    if size != 0 {
        return page_align(size);
    }

    page_align(core::cmp::min(rlimit(RLIMIT_STACK), SZ_4G as c_ulong))
}

unsafe fn unmap_shadow_stack(base: u64, size: u64) {
    let r = vm_munmap(base as c_ulong, size as c_ulong);

    /* mmap_write_lock_killable() failed with -EINTR. */
    if r == -EINTR {
        return;
    }

    /* For all other types of vm_munmap() failure, either the system is out of memory or there is bug. */
    warn_on_once(r != 0);
}

unsafe fn shstk_setup() -> c_int {
    let shstk = &mut (*current).thread.shstk;
    let size: c_ulong;
    let addr: c_ulong;

    if features_enabled(ARCH_SHSTK_SHSTK) {
        return 0;
    }
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) || in_ia32_syscall() {
        return -EOPNOTSUPP;
    }

    size = adjust_shstk_size(0);
    addr = alloc_shstk(0, size, 0, false);
    if is_err_value(addr) {
        return ptr_err(addr as *mut core::ffi::c_void);
    }

    fpregs_lock_and_load();
    wrmsrq(MSR_IA32_PL3_SSP, addr + size);
    wrmsrq(MSR_IA32_U_CET, CET_SHSTK_EN);
    fpregs_unlock();

    shstk.base = addr;
    shstk.size = size;
    features_set(ARCH_SHSTK_SHSTK);
    0
}

pub unsafe fn reset_thread_features() {
    core::ptr::write_bytes(&mut (*current).thread.shstk as *mut _, 0, 1);
    (*current).thread.features = 0;
    (*current).thread.features_locked = 0;
}

pub unsafe fn shstk_alloc_thread_stack(tsk: *mut task_struct, clone_flags: u64, stack_size: c_ulong) -> c_ulong {
    let shstk = &mut (*tsk).thread.shstk;
    if !features_enabled(ARCH_SHSTK_SHSTK) { return 0; }
    if clone_flags & CLONE_VFORK != 0 {
        shstk.base = 0; shstk.size = 0; return 0;
    }
    if clone_flags & CLONE_VM == 0 { return 0; }
    let size = adjust_shstk_size(stack_size);
    let addr = alloc_shstk(0, size, 0, false);
    if is_err_value(addr) { return addr; }
    shstk.base = addr; shstk.size = size;
    addr + size
}

unsafe fn get_user_shstk_addr() -> c_ulong {
    let mut ssp = 0u64;
    fpregs_lock_and_load();
    rdmsrq(MSR_IA32_PL3_SSP, &mut ssp);
    fpregs_unlock();
    ssp as c_ulong
}

pub unsafe fn shstk_pop(val: *mut u64) -> c_int {
    if !features_enabled(ARCH_SHSTK_SHSTK) { return -ENOTSUPP; }
    fpregs_lock_and_load();
    let mut ssp = 0u64;
    rdmsrq(MSR_IA32_PL3_SSP, &mut ssp);
    let ret = if !val.is_null() && get_user(val, ssp as *const u64) != 0 { -EFAULT } else {
        wrmsrq(MSR_IA32_PL3_SSP, ssp + SS_FRAME_SIZE as u64); 0
    };
    fpregs_unlock(); ret
}

pub unsafe fn shstk_push(val: u64) -> c_int {
    if !features_enabled(ARCH_SHSTK_SHSTK) { return -ENOTSUPP; }
    fpregs_lock_and_load();
    let mut ssp = 0u64; rdmsrq(MSR_IA32_PL3_SSP, &mut ssp);
    ssp -= SS_FRAME_SIZE as u64;
    let ret = write_user_shstk_64(ssp as *mut core::ffi::c_void, val);
    if ret == 0 { wrmsrq(MSR_IA32_PL3_SSP, ssp); }
    fpregs_unlock(); ret
}

const SHSTK_DATA_BIT: u64 = BIT(63);

unsafe fn put_shstk_data(addr: *mut u64, data: u64) -> c_int {
    if warn_on_once(data & SHSTK_DATA_BIT != 0) { return -EINVAL; }
    if write_user_shstk_64(addr, data | SHSTK_DATA_BIT) != 0 { return -EFAULT; }
    0
}

unsafe fn get_shstk_data(data: *mut c_ulong, addr: *const c_ulong) -> c_int {
    let mut ldata = 0;
    if get_user(&mut ldata, addr) != 0 { return -EFAULT; }
    if ldata & SHSTK_DATA_BIT as c_ulong == 0 { return -EINVAL; }
    *data = ldata & !(SHSTK_DATA_BIT as c_ulong); 0
}

unsafe fn shstk_push_sigframe(ssp: *mut c_ulong) -> c_int {
    let target_ssp = *ssp;
    if !is_aligned(target_ssp, 8) { return -EINVAL; }
    *ssp -= SS_FRAME_SIZE as c_ulong;
    if put_shstk_data(*ssp as *mut u64, target_ssp as u64) != 0 { return -EFAULT; }
    0
}

unsafe fn shstk_pop_sigframe(ssp: *mut c_ulong) -> c_int {
    if !is_aligned(*ssp, 8) { return -EINVAL; }
    let mut token_addr = 0;
    let mut seq = 0;
    loop {
        if mmap_read_lock_killable((*current).mm) != 0 { return -EINTR; }
        let vma = find_vma((*current).mm, *ssp);
        let valid_vma = !vma.is_null() && ((*vma).vm_flags & VM_SHADOW_STACK) != 0;
        mmap_lock_speculate_try_begin((*current).mm, &mut seq);
        mmap_read_unlock((*current).mm);
        if !valid_vma { return -EINVAL; }
        let err = get_shstk_data(&mut token_addr, *ssp as *const c_ulong);
        if err != 0 { return err; }
        if mmap_lock_speculate_retry((*current).mm, seq) == 0 { break; }
    }
    if !is_aligned(token_addr, 8) || token_addr >= TASK_SIZE_MAX { return -EINVAL; }
    *ssp = token_addr; 0
}

pub unsafe fn setup_signal_shadow_stack(ksig: *mut ksignal) -> c_int {
    let restorer = (*ksig).ka.sa.sa_restorer;
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) || !features_enabled(ARCH_SHSTK_SHSTK) { return 0; }
    if restorer.is_null() { return -EINVAL; }
    let mut ssp = get_user_shstk_addr();
    if ssp == 0 { return -EINVAL; }
    let err = shstk_push_sigframe(&mut ssp);
    if err != 0 { return err; }
    ssp -= SS_FRAME_SIZE as c_ulong;
    if write_user_shstk_64(ssp as *mut u64, restorer as u64) != 0 { return -EFAULT; }
    fpregs_lock_and_load(); wrmsrq(MSR_IA32_PL3_SSP, ssp); fpregs_unlock(); 0
}

pub unsafe fn restore_signal_shadow_stack() -> c_int {
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) || !features_enabled(ARCH_SHSTK_SHSTK) { return 0; }
    let mut ssp = get_user_shstk_addr();
    if ssp == 0 { return -EINVAL; }
    let err = shstk_pop_sigframe(&mut ssp);
    if err != 0 { return err; }
    fpregs_lock_and_load(); wrmsrq(MSR_IA32_PL3_SSP, ssp); fpregs_unlock(); 0
}

pub unsafe fn shstk_free(tsk: *mut task_struct) {
    let shstk = &mut (*tsk).thread.shstk;
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) || !features_enabled(ARCH_SHSTK_SHSTK) { return; }
    if (*tsk).mm.is_null() || (*tsk).mm != (*current).mm || shstk.base == 0 { return; }
    if warn_on(shstk.size == 0) { return; }
    unmap_shadow_stack(shstk.base as u64, shstk.size as u64); shstk.size = 0;
}

unsafe fn wrss_control(enable: bool) -> c_int {
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) { return -EOPNOTSUPP; }
    if !features_enabled(ARCH_SHSTK_SHSTK) { return -EPERM; }
    if features_enabled(ARCH_SHSTK_WRSS) == enable { return 0; }
    fpregs_lock_and_load(); let mut msrval = 0; rdmsrq(MSR_IA32_U_CET, &mut msrval);
    if enable { features_set(ARCH_SHSTK_WRSS); msrval |= CET_WRSS_EN; }
    else { features_clr(ARCH_SHSTK_WRSS); if msrval & CET_WRSS_EN == 0 { fpregs_unlock(); return 0; } msrval &= !CET_WRSS_EN; }
    wrmsrq(MSR_IA32_U_CET, msrval); fpregs_unlock(); 0
}

unsafe fn shstk_disable() -> c_int {
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) { return -EOPNOTSUPP; }
    if !features_enabled(ARCH_SHSTK_SHSTK) { return 0; }
    fpregs_lock_and_load(); wrmsrq(MSR_IA32_U_CET, 0); wrmsrq(MSR_IA32_PL3_SSP, 0); fpregs_unlock();
    shstk_free(current); features_clr(ARCH_SHSTK_SHSTK | ARCH_SHSTK_WRSS); 0
}

pub unsafe fn map_shadow_stack(addr: c_ulong, size: c_ulong, flags: c_uint) -> c_long {
    let set_tok = flags & SHADOW_STACK_SET_TOKEN != 0;
    if !cpu_feature_enabled(X86_FEATURE_USER_SHSTK) { return -EOPNOTSUPP as c_long; }
    if flags & !SHADOW_STACK_SET_TOKEN != 0 { return -EINVAL as c_long; }
    if set_tok && size < 8 { return -ENOSPC as c_long; }
    if addr != 0 && addr < SZ_4G as c_ulong { return -ERANGE as c_long; }
    let aligned_size = page_align(size);
    if aligned_size < size { return -EOVERFLOW as c_long; }
    alloc_shstk(addr, aligned_size, size, set_tok) as c_long
}

pub unsafe fn shstk_prctl(task: *mut task_struct, option: c_int, arg2: c_ulong) -> c_long {
    let features = arg2;
    if option == ARCH_SHSTK_STATUS { return put_user((*task).thread.features, arg2 as *mut c_ulong) as c_long; }
    if option == ARCH_SHSTK_LOCK { (*task).thread.features_locked |= features; return 0; }
    if task != current { return -EINVAL as c_long; }
    if features & (*task).thread.features_locked != 0 { return -EPERM as c_long; }
    if hweight_long(features) > 1 { return -EINVAL as c_long; }
    if option == ARCH_SHSTK_DISABLE {
        if features & ARCH_SHSTK_WRSS != 0 { return wrss_control(false) as c_long; }
        if features & ARCH_SHSTK_SHSTK != 0 { return shstk_disable() as c_long; }
        return -EINVAL as c_long;
    }
    if features & ARCH_SHSTK_SHSTK != 0 { return shstk_setup() as c_long; }
    if features & ARCH_SHSTK_WRSS != 0 { return wrss_control(true) as c_long; }
    -EINVAL as c_long
}

pub unsafe fn shstk_update_last_frame(val: c_ulong) -> c_int {
    if !features_enabled(ARCH_SHSTK_SHSTK) { return 0; }
    write_user_shstk_64(get_user_shstk_addr() as *mut u64, val as u64)
}

pub unsafe fn shstk_is_enabled() -> bool {
    features_enabled(ARCH_SHSTK_SHSTK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
