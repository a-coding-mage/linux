// SPDX-License-Identifier: GPL-2.0-only
// Original unit lines 3689-4886: user API, negotiated features and entry points.
unsafe fn validate_unaligned_range(mm: *mut mm_struct, start: u64, len: u64) -> c_int {
    let size = mm_task_size(mm) as u64;
    if len & !(PAGE_MASK as u64) != 0
        || len == 0
        || start >= size
        || len > size - start
        || start.wrapping_add(len) <= start
    {
        error(EINVAL)
    } else {
        0
    }
}
unsafe fn validate_range(mm: *mut mm_struct, start: u64, len: u64) -> c_int {
    if start & !(PAGE_MASK as u64) != 0 {
        error(EINVAL)
    } else {
        validate_unaligned_range(mm, start, len)
    }
}
unsafe fn read_user<T>(out: *mut T, arg: c_ulong, n: usize) -> bool {
    copy_from_user(out as _, arg as _, n as _) != 0
}
unsafe fn userfaultfd_register(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mm = (*c).mm;
    let mut reg: uffdio_register = zeroed();
    let user = arg as *mut uffdio_register;
    if read_user(
        &mut reg,
        arg,
        size_of::<uffdio_register>() - size_of::<u64>(),
    ) {
        return error(EFAULT);
    }
    if reg.mode == 0 || reg.mode & !RUST_UFFD_UFFD_API_REGISTER_MODES != 0 {
        return error(EINVAL);
    }
    let mut flags = 0;
    if reg.mode & RUST_UFFD_UFFDIO_REGISTER_MODE_MISSING != 0 {
        flags |= RUST_UFFD_VM_UFFD_MISSING;
    }
    if reg.mode & RUST_UFFD_UFFDIO_REGISTER_MODE_WP != 0 {
        if !pgtable_supports_uffd() {
            return error(EINVAL);
        }
        flags |= RUST_UFFD_VM_UFFD_WP;
    }
    if reg.mode & RUST_UFFD_UFFDIO_REGISTER_MODE_RWP != 0 {
        if !pgtable_supports_uffd()
            || RUST_UFFD_VM_UFFD_RWP == RUST_UFFD_VM_NONE
            || userfaultfd_features(c) & RUST_UFFD_UFFD_FEATURE_RWP == 0
        {
            return error(EINVAL);
        }
        flags |= RUST_UFFD_VM_UFFD_RWP;
    }
    if flags & RUST_UFFD_VM_UFFD_WP != 0 && flags & RUST_UFFD_VM_UFFD_RWP != 0 {
        return error(EINVAL);
    }
    if reg.mode & RUST_UFFD_UFFDIO_REGISTER_MODE_MINOR != 0 {
        #[cfg(not(CONFIG_HAVE_ARCH_USERFAULTFD_MINOR))]
        return error(EINVAL);
        #[cfg(CONFIG_HAVE_ARCH_USERFAULTFD_MINOR)]
        {
            flags |= RUST_UFFD_VM_UFFD_MINOR;
        }
    }
    let ret = validate_range(mm, reg.range.start, reg.range.len);
    if ret != 0 {
        return ret;
    }
    let start = reg.range.start as c_ulong;
    let end = start.wrapping_add(reg.range.len as c_ulong);
    if !mmget_not_zero(mm) {
        return error(ENOMEM);
    }
    mmap_write_lock(mm);
    let wp_async = userfaultfd_wp_async_ctx(c);
    let mut basic = false;
    let ret = (|| {
        let mut i = iterator(mm, start);
        let first = vma_find(&mut i, end);
        if first.is_null() {
            return error(EINVAL);
        }
        if is_vm_hugetlb_page(first) && start & (vma_kernel_pagesize(first) - 1) != 0 {
            return error(EINVAL);
        }
        let mut v = first;
        let mut found = false;
        loop {
            cond_resched();
            vm_warn!(
                warn_044,
                (!vctx(v).is_null()) ^ (vflags(v) & RUST_UFFD___VM_UFFD_FLAGS != 0)
            );
            if !vma_can_userfault(v, flags, wp_async)
                || flags & RUST_UFFD_VM_UFFD_RWP != 0 && !vma_is_accessible(v)
            {
                return error(EINVAL);
            }
            if vflags(v) & RUST_UFFD_VM_MAYWRITE == 0 {
                return error(EPERM);
            }
            if is_vm_hugetlb_page(v)
                && end <= vend(v)
                && end > vstart(v)
                && end & (vma_kernel_pagesize(v) - 1) != 0
            {
                return error(EINVAL);
            }
            if flags & RUST_UFFD_VM_UFFD_WP != 0 && vflags(v) & RUST_UFFD_VM_MAYWRITE == 0 {
                return error(EPERM);
            }
            if !vctx(v).is_null() && vctx(v) != c
                || vctx(v) == c
                    && vflags(v) & (RUST_UFFD_VM_UFFD_WP | RUST_UFFD_VM_UFFD_RWP) & !flags != 0
            {
                return error(EBUSY);
            }
            if is_vm_hugetlb_page(v) {
                basic = true;
            }
            found = true;
            v = vma_find(&mut i, end);
            if v.is_null() {
                break;
            }
        }
        vm_warn!(warn_045, !found);
        userfaultfd_register_range(c, first, flags, start, end, wp_async)
    })();
    mmap_write_unlock(mm);
    mmput(mm);
    if ret != 0 {
        return ret;
    }
    let mut ioctls = if basic {
        RUST_UFFD_UFFD_API_RANGE_IOCTLS_BASIC
    } else {
        RUST_UFFD_UFFD_API_RANGE_IOCTLS
    };
    if reg.mode & RUST_UFFD_UFFDIO_REGISTER_MODE_WP == 0 {
        ioctls &= !(1u64 << _UFFDIO_WRITEPROTECT);
    }
    if reg.mode & RUST_UFFD_UFFDIO_REGISTER_MODE_MINOR == 0 {
        ioctls &= !(1u64 << _UFFDIO_CONTINUE);
    }
    if reg.mode & RUST_UFFD_UFFDIO_REGISTER_MODE_RWP == 0 {
        ioctls &= !(1u64 << _UFFDIO_RWPROTECT);
    }
    if put_user_u64(ioctls, addr_of_mut!((*user).ioctls)) != 0 {
        error(EFAULT)
    } else {
        0
    }
}
unsafe fn userfaultfd_unregister(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mm = (*c).mm;
    let mut range: uffdio_range = zeroed();
    if read_user(&mut range, arg, size_of::<uffdio_range>()) {
        return error(EFAULT);
    }
    let ret = validate_range(mm, range.start, range.len);
    if ret != 0 {
        return ret;
    }
    let mut start = range.start as c_ulong;
    let end = start.wrapping_add(range.len as c_ulong);
    let wp_async = userfaultfd_wp_async_ctx(c);
    if !mmget_not_zero(mm) {
        return error(ENOMEM);
    }
    mmap_write_lock(mm);
    let ret = (|| {
        let mut i = iterator(mm, start);
        let first = vma_find(&mut i, end);
        if first.is_null() {
            return error(EINVAL);
        }
        if is_vm_hugetlb_page(first) && start & (vma_kernel_pagesize(first) - 1) != 0 {
            return error(EINVAL);
        }
        let mut v = first;
        let mut found = false;
        loop {
            cond_resched();
            vm_warn!(
                warn_046,
                (!vctx(v).is_null()) ^ (vflags(v) & RUST_UFFD___VM_UFFD_FLAGS != 0)
            );
            if !vctx(v).is_null() && vctx(v) != c || !vma_can_userfault(v, vflags(v), wp_async) {
                return error(EINVAL);
            }
            found = true;
            v = vma_find(&mut i, end);
            if v.is_null() {
                break;
            }
        }
        vm_warn!(warn_047, !found);
        vma_iter_set(&mut i, start);
        let mut prev = vma_prev(&mut i);
        if vstart(first) < start {
            prev = first;
        }
        loop {
            v = vma_find(&mut i, end);
            if v.is_null() {
                break;
            }
            cond_resched();
            if !vctx(v).is_null() {
                vm_warn!(warn_048, vctx(v) != c);
                vm_warn!(warn_049, !vma_can_userfault(v, vflags(v), wp_async));
                vm_warn!(warn_050, vflags(v) & RUST_UFFD_VM_MAYWRITE == 0);
                start = core::cmp::max(start, vstart(v));
                let ve = core::cmp::min(end, vend(v));
                if userfaultfd_missing(v) {
                    let mut r = userfaultfd_wake_range {
                        start,
                        len: ve - start,
                    };
                    wake_userfault(vctx(v), &mut r);
                }
                v = userfaultfd_clear_vma(&mut i, prev, v, start, ve);
                if is_err(v) {
                    return ptr_err(v);
                }
            }
            prev = v;
            start = vend(v);
        }
        0
    })();
    mmap_write_unlock(mm);
    mmput(mm);
    ret
}
unsafe fn userfaultfd_wake(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut r: uffdio_range = zeroed();
    if read_user(&mut r, arg, size_of::<uffdio_range>()) {
        return error(EFAULT);
    }
    let ret = validate_range((*c).mm, r.start, r.len);
    if ret != 0 {
        return ret;
    }
    let mut r = userfaultfd_wake_range {
        start: r.start as _,
        len: r.len as _,
    };
    vm_warn!(warn_051, r.len == 0);
    wake_userfault(c, &mut r);
    0
}
enum FillResultWarning {
    Copy,
    Zero,
    Continue,
    Poison,
    Move,
}
unsafe fn fill_result(
    c: *mut userfaultfd_ctx,
    ret: i64,
    out: *mut i64,
    start: u64,
    len: u64,
    dontwake: bool,
    warning: FillResultWarning,
) -> c_int {
    if put_user_i64(ret, out) != 0 {
        return error(EFAULT);
    }
    if ret < 0 {
        return ret as c_int;
    }
    #[cfg(CONFIG_DEBUG_VM)]
    match warning {
        FillResultWarning::Copy => warn_copy_zero(ret == 0),
        FillResultWarning::Zero => warn_zeropage_zero(ret == 0),
        FillResultWarning::Continue => warn_continue_zero(ret == 0),
        FillResultWarning::Poison => warn_poison_zero(ret == 0),
        FillResultWarning::Move => warn_move_zero(ret == 0),
    }
    let mut r = userfaultfd_wake_range {
        start: start as _,
        len: ret as _,
    };
    if !dontwake {
        wake_userfault(c, &mut r);
    }
    if r.len as u64 == len {
        0
    } else {
        error(EAGAIN)
    }
}
unsafe fn userfaultfd_copy(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let user = arg as *mut uffdio_copy;
    let out = addr_of_mut!((*user).copy);
    if atomic_read(addr_of!((*c).mmap_changing)) != 0 {
        return if put_user_i64(error(EAGAIN) as _, out) != 0 {
            error(EFAULT)
        } else {
            error(EAGAIN)
        };
    }
    let mut p: uffdio_copy = zeroed();
    if read_user(&mut p, arg, size_of::<uffdio_copy>() - size_of::<i64>()) {
        return error(EFAULT);
    }
    let ret = validate_unaligned_range((*c).mm, p.src, p.len);
    if ret != 0 {
        return ret;
    }
    let ret = validate_range((*c).mm, p.dst, p.len);
    if ret != 0 {
        return ret;
    }
    if p.mode & !(RUST_UFFD_UFFDIO_COPY_MODE_DONTWAKE | RUST_UFFD_UFFDIO_COPY_MODE_WP) != 0 {
        return error(EINVAL);
    }
    let flags = if p.mode & RUST_UFFD_UFFDIO_COPY_MODE_WP != 0 {
        RUST_UFFD_MFILL_ATOMIC_WP
    } else {
        0
    };
    if !mmget_not_zero((*c).mm) {
        return error(ESRCH);
    }
    let ret = mfill_atomic_copy(c, p.dst as _, p.src as _, p.len as _, flags);
    mmput((*c).mm);
    fill_result(
        c,
        ret as _,
        out,
        p.dst,
        p.len,
        p.mode & RUST_UFFD_UFFDIO_COPY_MODE_DONTWAKE != 0,
        FillResultWarning::Copy,
    )
}
macro_rules! range_fill_ioctl {
    ($name:ident,$ty:ty,$out:ident,$allowed:ident,$wp:expr,$dontwake:ident,$body:expr,$warning:ident) => {
        unsafe fn $name(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
            let user = arg as *mut $ty;
            let out = addr_of_mut!((*user).$out);
            if atomic_read(addr_of!((*c).mmap_changing)) != 0 {
                return if put_user_i64(error(EAGAIN) as _, out) != 0 {
                    error(EFAULT)
                } else {
                    error(EAGAIN)
                };
            }
            let mut p: $ty = zeroed();
            if read_user(&mut p, arg, size_of::<$ty>() - size_of::<i64>()) {
                return error(EFAULT);
            }
            let ret = validate_range((*c).mm, p.range.start, p.range.len);
            if ret != 0 {
                return ret;
            }
            if p.mode & !$allowed != 0 {
                return error(EINVAL);
            }
            let flags = if p.mode & $wp != 0 {
                RUST_UFFD_MFILL_ATOMIC_WP
            } else {
                0
            };
            if !mmget_not_zero((*c).mm) {
                return error(ESRCH);
            }
            let ret = ($body)(c, p.range.start as c_ulong, p.range.len as c_ulong, flags);
            mmput((*c).mm);
            fill_result(
                c,
                ret as _,
                out,
                p.range.start,
                p.range.len,
                p.mode & $dontwake != 0,
                FillResultWarning::$warning,
            )
        }
    };
}
const CONTINUE_MODES: u64 =
    RUST_UFFD_UFFDIO_CONTINUE_MODE_DONTWAKE | RUST_UFFD_UFFDIO_CONTINUE_MODE_WP;
range_fill_ioctl!(
    userfaultfd_zeropage,
    uffdio_zeropage,
    zeropage,
    RUST_UFFD_UFFDIO_ZEROPAGE_MODE_DONTWAKE,
    0u64,
    RUST_UFFD_UFFDIO_ZEROPAGE_MODE_DONTWAKE,
    |c, s, l, _f| mfill_atomic_zeropage(c, s, l),
    Zero
);
range_fill_ioctl!(
    userfaultfd_continue,
    uffdio_continue,
    mapped,
    CONTINUE_MODES,
    RUST_UFFD_UFFDIO_CONTINUE_MODE_WP,
    RUST_UFFD_UFFDIO_CONTINUE_MODE_DONTWAKE,
    mfill_atomic_continue,
    Continue
);
range_fill_ioctl!(
    userfaultfd_poison,
    uffdio_poison,
    updated,
    RUST_UFFD_UFFDIO_POISON_MODE_DONTWAKE,
    0u64,
    RUST_UFFD_UFFDIO_POISON_MODE_DONTWAKE,
    mfill_atomic_poison,
    Poison
);
macro_rules! protect_ioctl {
    ($name:ident,$ty:ty,$protect:ident,$dontwake:ident,$body:ident) => {
        unsafe fn $name(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
            if atomic_read(addr_of!((*c).mmap_changing)) != 0 {
                return error(EAGAIN);
            }
            let mut p: $ty = zeroed();
            if read_user(&mut p, arg, size_of::<$ty>()) {
                return error(EFAULT);
            }
            let ret = validate_range((*c).mm, p.range.start, p.range.len);
            if ret != 0 {
                return ret;
            }
            if p.mode & !($protect | $dontwake) != 0 {
                return error(EINVAL);
            }
            let enable = p.mode & $protect != 0;
            let no_wake = p.mode & $dontwake != 0;
            if enable && no_wake {
                return error(EINVAL);
            }
            if !mmget_not_zero((*c).mm) {
                return error(ESRCH);
            }
            let ret = $body(c, p.range.start as _, p.range.len as _, enable);
            mmput((*c).mm);
            if ret != 0 {
                return ret;
            }
            if !enable && !no_wake {
                let mut r = userfaultfd_wake_range {
                    start: p.range.start as _,
                    len: p.range.len as _,
                };
                wake_userfault(c, &mut r);
            }
            ret
        }
    };
}
protect_ioctl!(
    userfaultfd_writeprotect,
    uffdio_writeprotect,
    RUST_UFFD_UFFDIO_WRITEPROTECT_MODE_WP,
    RUST_UFFD_UFFDIO_WRITEPROTECT_MODE_DONTWAKE,
    mwriteprotect_range
);
protect_ioctl!(
    userfaultfd_rwprotect,
    uffdio_rwprotect,
    RUST_UFFD_UFFDIO_RWPROTECT_MODE_RWP,
    RUST_UFFD_UFFDIO_RWPROTECT_MODE_DONTWAKE,
    mrwprotect_range
);
unsafe fn uffd_api_available_features() -> u64 {
    let mut f = RUST_UFFD_UFFD_API_FEATURES;
    #[cfg(not(CONFIG_HAVE_ARCH_USERFAULTFD_MINOR))]
    {
        f &=
            !((RUST_UFFD_UFFD_FEATURE_MINOR_HUGETLBFS | RUST_UFFD_UFFD_FEATURE_MINOR_SHMEM) as u64);
    }
    if !pgtable_supports_uffd() {
        f &= !(RUST_UFFD_UFFD_FEATURE_PAGEFAULT_FLAG_WP as u64);
    }
    if !uffd_supports_wp_marker() {
        f &= !((RUST_UFFD_UFFD_FEATURE_WP_HUGETLBFS_SHMEM
            | RUST_UFFD_UFFD_FEATURE_WP_UNPOPULATED
            | RUST_UFFD_UFFD_FEATURE_WP_ASYNC) as u64);
    }
    if RUST_UFFD_VM_UFFD_RWP == RUST_UFFD_VM_NONE || !pgtable_supports_uffd() {
        f &= !((RUST_UFFD_UFFD_FEATURE_RWP | RUST_UFFD_UFFD_FEATURE_RWP_ASYNC) as u64);
    }
    f
}
unsafe fn userfaultfd_set_mode(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut mode: uffdio_set_mode = zeroed();
    if read_user(&mut mode, arg, size_of::<uffdio_set_mode>()) {
        return error(EFAULT);
    }
    if mode.enable & mode.disable != 0
        || (mode.enable | mode.disable)
            & !(uffd_api_available_features() & RUST_UFFD_UFFD_FEATURE_RWP_ASYNC as u64)
            != 0
    {
        return error(EINVAL);
    }
    if mode.enable & RUST_UFFD_UFFD_FEATURE_RWP_ASYNC as u64 != 0
        && userfaultfd_features(c) & RUST_UFFD_UFFD_FEATURE_RWP == 0
    {
        return error(EINVAL);
    }
    let mm = (*c).mm;
    if !mmget_not_zero(mm) {
        return error(ESRCH);
    }
    mmap_write_lock(mm);
    let mut i = iterator(mm, 0);
    loop {
        let v = vma_next(&mut i);
        if v.is_null() {
            break;
        }
        if vctx(v) == c {
            vma_start_write(v);
        }
    }
    write_features(
        c,
        (((*c).features as u64 | mode.enable) & !mode.disable) as u32,
    );
    mmap_write_unlock(mm);
    if mode.enable & RUST_UFFD_UFFD_FEATURE_RWP_ASYNC as u64 != 0 {
        let mut r: userfaultfd_wake_range = zeroed();
        spin_lock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
        __wake_up_locked_key(
            addr_of_mut!((*c).fault_pending_wqh),
            RUST_UFFD_TASK_NORMAL,
            &mut r as *mut _ as _,
        );
        __wake_up(
            addr_of_mut!((*c).fault_wqh),
            RUST_UFFD_TASK_NORMAL,
            1,
            &mut r as *mut _ as _,
        );
        spin_unlock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    }
    mmput(mm);
    0
}
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_wp_async(v: *mut vm_area_struct) -> bool {
    userfaultfd_wp_async_ctx(vctx(v))
}
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_rwp_async(v: *mut vm_area_struct) -> bool {
    userfaultfd_rwp_async_ctx(vctx(v))
}
fn uffd_ctx_features(f: u64) -> u32 {
    f as u32 | UFFD_FEATURE_INITIALIZED
}
unsafe fn userfaultfd_move(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let user = arg as *mut uffdio_move;
    let out = addr_of_mut!((*user).move_);
    if atomic_read(addr_of!((*c).mmap_changing)) != 0 {
        return if put_user_i64(error(EAGAIN) as _, out) != 0 {
            error(EFAULT)
        } else {
            error(EAGAIN)
        };
    }
    let mut p: uffdio_move = zeroed();
    if read_user(&mut p, arg, size_of::<uffdio_move>() - size_of::<i64>()) {
        return error(EFAULT);
    }
    let mm = (*c).mm;
    if mm != current_mm() {
        return error(EINVAL);
    }
    let ret = validate_range(mm, p.dst, p.len);
    if ret != 0 {
        return ret;
    }
    let ret = validate_range(mm, p.src, p.len);
    if ret != 0 {
        return ret;
    }
    if p.mode & !(RUST_UFFD_UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES | RUST_UFFD_UFFDIO_MOVE_MODE_DONTWAKE)
        != 0
    {
        return error(EINVAL);
    }
    if !mmget_not_zero(mm) {
        return error(ESRCH);
    }
    let ret = move_pages(c, p.dst as _, p.src as _, p.len as _, p.mode);
    mmput(mm);
    fill_result(
        c,
        ret as _,
        out,
        p.dst,
        p.len,
        p.mode & RUST_UFFD_UFFDIO_MOVE_MODE_DONTWAKE != 0,
        FillResultWarning::Move,
    )
}
unsafe fn userfaultfd_api(c: *mut userfaultfd_ctx, arg: c_ulong) -> c_int {
    let mut api: uffdio_api = zeroed();
    if read_user(&mut api, arg, size_of::<uffdio_api>()) {
        return error(EFAULT);
    }
    let mut f = api.features;
    let ret = (|| {
        if api.api != RUST_UFFD_UFFD_API {
            return error(EINVAL);
        }
        if f & RUST_UFFD_UFFD_FEATURE_EVENT_FORK as u64 != 0 && !capable(CAP_SYS_PTRACE as c_int) {
            return error(EPERM);
        }
        if f & RUST_UFFD_UFFD_FEATURE_WP_ASYNC as u64 != 0 {
            f |= RUST_UFFD_UFFD_FEATURE_WP_UNPOPULATED as u64;
        }
        if f & RUST_UFFD_UFFD_FEATURE_RWP_ASYNC as u64 != 0
            && f & RUST_UFFD_UFFD_FEATURE_RWP as u64 == 0
        {
            return error(EINVAL);
        }
        api.features = uffd_api_available_features();
        if f & !api.features != 0 {
            return error(EINVAL);
        }
        api.ioctls = RUST_UFFD_UFFD_API_IOCTLS;
        if copy_to_user(
            arg as _,
            &api as *const _ as _,
            size_of::<uffdio_api>() as _,
        ) != 0
        {
            return error(EFAULT);
        }
        if cmpxchg_features(c, 0, uffd_ctx_features(f)) != 0 {
            return error(EINVAL);
        }
        0
    })();
    // The original copy-to-user fault exits directly; semantic errors zero the
    // entire API result, including padding, before returning to userspace.
    if ret != 0 && ret != error(EFAULT) {
        core::ptr::write_bytes(&mut api, 0, 1);
        if copy_to_user(
            arg as _,
            &api as *const _ as _,
            size_of::<uffdio_api>() as _,
        ) != 0
        {
            return error(EFAULT);
        }
    }
    ret
}
unsafe extern "C" fn userfaultfd_ioctl(file: *mut file, cmd: u32, arg: c_ulong) -> c_long {
    let c = (*file).private_data as *mut userfaultfd_ctx;
    if cmd != RUST_UFFD_UFFDIO_API && !userfaultfd_is_initialized(c) {
        return error(EINVAL) as c_long;
    }
    (match cmd {
        RUST_UFFD_UFFDIO_API => userfaultfd_api(c, arg),
        RUST_UFFD_UFFDIO_REGISTER => userfaultfd_register(c, arg),
        RUST_UFFD_UFFDIO_UNREGISTER => userfaultfd_unregister(c, arg),
        RUST_UFFD_UFFDIO_WAKE => userfaultfd_wake(c, arg),
        RUST_UFFD_UFFDIO_COPY => userfaultfd_copy(c, arg),
        RUST_UFFD_UFFDIO_ZEROPAGE => userfaultfd_zeropage(c, arg),
        RUST_UFFD_UFFDIO_MOVE => userfaultfd_move(c, arg),
        RUST_UFFD_UFFDIO_WRITEPROTECT => userfaultfd_writeprotect(c, arg),
        RUST_UFFD_UFFDIO_CONTINUE => userfaultfd_continue(c, arg),
        RUST_UFFD_UFFDIO_POISON => userfaultfd_poison(c, arg),
        RUST_UFFD_UFFDIO_RWPROTECT => userfaultfd_rwprotect(c, arg),
        RUST_UFFD_UFFDIO_SET_MODE => userfaultfd_set_mode(c, arg),
        _ => error(EINVAL),
    }) as c_long
}
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn userfaultfd_show_fdinfo(m: *mut seq_file, f: *mut file) {
    let c = (*f).private_data as *mut userfaultfd_ctx;
    let mut pending = 0 as c_ulong;
    let mut total = 0 as c_ulong;
    spin_lock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    let h = addr_of_mut!((*c).fault_pending_wqh.head);
    let mut n = (*h).next;
    while n != h {
        pending += 1;
        total += 1;
        n = (*n).next;
    }
    let h = addr_of_mut!((*c).fault_wqh.head);
    let mut n = (*h).next;
    while n != h {
        total += 1;
        n = (*n).next;
    }
    spin_unlock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    seq_printf(
        m,
        c"pending:\t%lu\ntotal:\t%lu\nAPI:\t%Lx:%x:%Lx\n".as_ptr(),
        pending,
        total,
        RUST_UFFD_UFFD_API,
        userfaultfd_features(c),
        RUST_UFFD_UFFD_API_IOCTLS | RUST_UFFD_UFFD_API_RANGE_IOCTLS,
    );
}
// SAFETY: These generated native callback records are immutable static tables.
// Their raw pointers name static kernel code/data; using a callback's context
// still requires the original operation's locks and reference ownership.
unsafe impl Sync for file_operations {}
static userfaultfd_fops: file_operations = file_operations {
    #[cfg(CONFIG_PROC_FS)]
    show_fdinfo: Some(userfaultfd_show_fdinfo),
    release: Some(userfaultfd_release),
    poll: Some(userfaultfd_poll),
    read_iter: Some(userfaultfd_read_iter),
    unlocked_ioctl: Some(userfaultfd_ioctl),
    #[cfg(CONFIG_COMPAT)]
    compat_ioctl: Some(compat_ptr_ioctl),
    #[cfg(not(CONFIG_COMPAT))]
    compat_ioctl: None,
    llseek: Some(noop_llseek),
    ..unsafe { zeroed() }
};
unsafe extern "C" fn init_once_userfaultfd_ctx(mem: *mut c_void) {
    let c = mem as *mut userfaultfd_ctx;
    init_waitqueue_head_pending(addr_of_mut!((*c).fault_pending_wqh));
    init_waitqueue_head_fault(addr_of_mut!((*c).fault_wqh));
    init_waitqueue_head_event(addr_of_mut!((*c).event_wqh));
    init_waitqueue_head_fd(addr_of_mut!((*c).fd_wqh));
    seqcount_spinlock_init(
        addr_of_mut!((*c).refile_seq),
        addr_of_mut!((*c).fault_pending_wqh.lock),
    );
}
unsafe fn new_userfaultfd(flags: c_int) -> c_int {
    vm_warn!(warn_053, current_mm().is_null());
    const {
        assert!(RUST_UFFD_UFFD_USER_MODE_ONLY & RUST_UFFD_UFFD_SHARED_FCNTL_FLAGS == 0);
    }
    if (flags as u32) & !(RUST_UFFD_UFFD_SHARED_FCNTL_FLAGS | RUST_UFFD_UFFD_USER_MODE_ONLY) != 0 {
        return error(EINVAL);
    }
    let c =
        kmem_cache_alloc(userfaultfd_ctx_cachep, RUST_UFFD_GFP_KERNEL as _) as *mut userfaultfd_ctx;
    if c.is_null() {
        return error(ENOMEM);
    }
    refcount_set(addr_of_mut!((*c).refcount), 1);
    (*c).flags = flags as u32;
    (*c).features = 0;
    (*c).released = false;
    init_rwsem_new(addr_of_mut!((*c).map_changing_lock));
    atomic_set(addr_of_mut!((*c).mmap_changing), 0);
    (*c).mm = current_mm();
    // Rust expansion of FD_PREPARE and its cleanup. Descriptor allocation must
    // precede the file-creation security hooks; neither failure pins the mm.
    let fd = get_unused_fd_flags((flags as u32) & RUST_UFFD_UFFD_SHARED_FCNTL_FLAGS);
    if fd < 0 {
        kfree(c as _);
        return fd;
    }
    let f = anon_inode_create_getfile(
        c"[userfaultfd]".as_ptr(),
        addr_of!(userfaultfd_fops),
        c as _,
        (RUST_UFFD_O_RDONLY | ((flags as u32) & RUST_UFFD_UFFD_SHARED_FCNTL_FLAGS)) as c_int,
        null_mut(),
    );
    if f.is_null() || is_err(f) {
        let ret = if f.is_null() {
            error(ENOMEM)
        } else {
            ptr_err(f)
        };
        put_unused_fd(fd as _);
        kfree(c as _);
        return ret;
    }
    mmgrab((*c).mm);
    (*f).f_mode |= RUST_UFFD_FMODE_NOWAIT;
    fd_install(fd as _, f);
    fd
}
unsafe fn userfaultfd_syscall_allowed(flags: c_int) -> bool {
    flags & (RUST_UFFD_UFFD_USER_MODE_ONLY as c_int) != 0
        || capable(CAP_SYS_PTRACE as c_int)
        || rust_uffd_sysctl_unprivileged_userfaultfd != 0
}
#[no_mangle]
pub unsafe extern "C" fn rust_uffd_sys_userfaultfd(flags: c_int) -> c_long {
    if !userfaultfd_syscall_allowed(flags) {
        error(EPERM) as c_long
    } else {
        new_userfaultfd(flags) as c_long
    }
}
unsafe extern "C" fn userfaultfd_dev_ioctl(file: *mut file, cmd: u32, flags: c_ulong) -> c_long {
    if cmd != RUST_UFFD_USERFAULTFD_IOC_NEW {
        error(EINVAL) as c_long
    } else {
        new_userfaultfd(flags as c_int) as c_long
    }
}
static userfaultfd_dev_fops: file_operations = file_operations {
    unlocked_ioctl: Some(userfaultfd_dev_ioctl),
    compat_ioctl: Some(userfaultfd_dev_ioctl),
    owner: null_mut(),
    llseek: Some(noop_llseek),
    ..unsafe { zeroed() }
};
static mut userfaultfd_misc: miscdevice = miscdevice {
    minor: RUST_UFFD_MISC_DYNAMIC_MINOR as c_int,
    name: c"userfaultfd".as_ptr(),
    fops: addr_of!(userfaultfd_dev_fops),
    ..unsafe { zeroed() }
};
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_uffd_init() -> c_int {
    let ret = misc_register(addr_of_mut!(userfaultfd_misc));
    if ret != 0 {
        return ret;
    }
    userfaultfd_ctx_cachep = kmem_cache_create(
        c"userfaultfd_ctx_cache".as_ptr(),
        size_of::<userfaultfd_ctx>() as _,
        0,
        (RUST_UFFD_SLAB_HWCACHE_ALIGN | RUST_UFFD_SLAB_PANIC) as _,
        Some(init_once_userfaultfd_ctx),
    );
    #[cfg(CONFIG_SYSCTL)]
    register_sysctl_table();
    0
}
