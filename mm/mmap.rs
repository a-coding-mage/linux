// SPDX-License-Identifier: GPL-2.0-only
// Rust owner of mm/mmap.c. Native headers are the authority for every layout,
// constant, configured operation, and callback ABI. No mmap.c body is in C.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/mmap_native_generated.rs"
    ));
}
use b::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// These are typed aliases of compiler-evaluated native macros, not literals.
const PAGE_SIZE: c_ulong = RUST_MMAP_PAGE_SIZE as c_ulong;
// Native PAGE_MASK definition, enforced by a header static_assert.
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const PAGE_SHIFT: u32 = RUST_MMAP_PAGE_SHIFT as u32;
const GFP_KERNEL: gfp_t = RUST_MMAP_GFP_KERNEL as gfp_t;
const ULONG_MAX: c_ulong = c_ulong::MAX;
#[inline]
fn err(e: u32) -> c_ulong {
    (-(e as c_long)) as c_ulong
}
#[inline]
fn page_align(n: c_ulong) -> c_ulong {
    n.wrapping_add(PAGE_SIZE - 1) & PAGE_MASK
}
#[inline]
fn align(n: c_ulong, a: c_ulong) -> c_ulong {
    n.wrapping_add(a - 1) & !(a - 1)
}
#[inline]
unsafe fn is_err(n: c_ulong) -> bool {
    rust_mmap_is_err_value(n)
}
#[inline]
unsafe fn start(v: *mut vm_area_struct) -> c_ulong {
    *rust_mmap_vma_vm_start(v)
}
#[inline]
unsafe fn end(v: *mut vm_area_struct) -> c_ulong {
    *rust_mmap_vma_vm_end(v)
}
#[inline]
unsafe fn flags(v: *mut vm_area_struct) -> vm_flags_t {
    *rust_mmap_vma_vm_flags(v)
}
#[inline]
unsafe fn test(v: *mut vm_area_struct, bit: vma_flag_t) -> bool {
    rust_mmap_vma_test(v, bit)
}
#[inline]
unsafe fn flags_test(f: *const vma_flags_t, bit: vma_flag_t) -> bool {
    rust_mmap_flags_test(f, bit)
}
#[inline]
unsafe fn set(f: *mut vma_flags_t, bit: vma_flag_t) {
    rust_mmap_flags_set(f, bit);
}
#[inline]
unsafe fn clear(f: *mut vma_flags_t, bit: vma_flag_t) {
    rust_mmap_flags_clear(f, bit);
}
#[inline]
unsafe fn empty_flags() -> vma_flags_t {
    rust_mmap_empty_flags()
}
unsafe fn init_list(l: *mut list_head) {
    (*l).next = l;
    (*l).prev = l;
}
unsafe fn iterator(mm: *mut mm_struct, addr: c_ulong) -> vma_iterator {
    // Exact VMA_ITERATOR aggregate initializer. It intentionally differs from
    // mas_init/vma_iter_init in last and max until the initial tree walk.
    let mut i: vma_iterator = zeroed();
    i.mas.tree = rust_mmap_mm_mm_mt(mm);
    i.mas.index = addr;
    i.mas.status = ma_start;
    i
}

#[cfg(CONFIG_HAVE_ARCH_MMAP_RND_BITS)]
#[no_mangle]
pub static mmap_rnd_bits_min: c_int = RUST_MMAP_CONFIG_ARCH_MMAP_RND_BITS_MIN;
#[cfg(CONFIG_HAVE_ARCH_MMAP_RND_BITS)]
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut mmap_rnd_bits_max: c_int = RUST_MMAP_CONFIG_ARCH_MMAP_RND_BITS_MAX;
#[cfg(CONFIG_HAVE_ARCH_MMAP_RND_BITS)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut mmap_rnd_bits: c_int = RUST_MMAP_CONFIG_ARCH_MMAP_RND_BITS;
#[cfg(CONFIG_HAVE_ARCH_MMAP_RND_COMPAT_BITS)]
#[no_mangle]
pub static mmap_rnd_compat_bits_min: c_int = RUST_MMAP_CONFIG_ARCH_MMAP_RND_COMPAT_BITS_MIN;
#[cfg(CONFIG_HAVE_ARCH_MMAP_RND_COMPAT_BITS)]
#[no_mangle]
pub static mmap_rnd_compat_bits_max: c_int = RUST_MMAP_CONFIG_ARCH_MMAP_RND_COMPAT_BITS_MAX;
#[cfg(CONFIG_HAVE_ARCH_MMAP_RND_COMPAT_BITS)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut mmap_rnd_compat_bits: c_int = RUST_MMAP_CONFIG_ARCH_MMAP_RND_COMPAT_BITS;
#[no_mangle]
pub static mut ignore_rlimit_data: bool = false;
#[no_mangle]
pub static mut stack_guard_gap: c_ulong = 256 << PAGE_SHIFT;

#[no_mangle]
pub unsafe extern "C" fn vma_set_page_prot(vma: *mut vm_area_struct) {
    let mut f = *rust_mmap_vma_flags(vma);
    let mut p = rust_mmap_pgprot_modify((*vma).vm_page_prot, f);
    if vma_wants_writenotify(vma, p) {
        clear(&mut f, VMA_SHARED_BIT as _);
        p = rust_mmap_pgprot_modify(p, f);
    }
    rust_mmap_page_prot_write_once(vma, p);
}
unsafe fn check_brk_limits(addr: c_ulong, len: c_ulong) -> c_int {
    let mm = rust_mmap_current_mm();
    let locked = flags_test(rust_mmap_mm_def_vma_flags(mm), VMA_LOCKED_BIT as _);
    let mapped = __get_unmapped_area(null_mut(), addr, len, 0, MAP_FIXED as _, empty_flags());
    if is_err(mapped) {
        return mapped as c_int;
    }
    if mlock_future_ok(mm, locked, len) {
        0
    } else {
        -(EAGAIN as c_int)
    }
}
#[no_mangle]
pub unsafe extern "C" fn rust_mmap_sys_brk(brk: c_ulong) -> c_long {
    let mm = rust_mmap_current_mm();
    let mut uf: list_head = zeroed();
    init_list(&mut uf);
    if rust_mmap_mmap_write_lock_killable(mm) != 0 {
        return -(EINTR as c_long);
    }
    let origbrk = *rust_mmap_mm_brk(mm);
    let mut min_brk = *rust_mmap_mm_start_brk(mm);
    #[cfg(CONFIG_COMPAT_BRK)]
    if !rust_mmap_brk_randomized() {
        min_brk = *rust_mmap_mm_end_data(mm);
    }
    // Structured early-exit block models out/success/success_unlocked exactly.
    let mut populate = false;
    let mut unlocked = false;
    let newbrk = page_align(brk);
    let oldbrk = page_align(origbrk);
    let success = 'operation: {
        if brk < min_brk {
            break 'operation false;
        }
        if rust_mmap_check_data_rlimit(
            rust_mmap_rlimit(RLIMIT_DATA),
            brk,
            *rust_mmap_mm_start_brk(mm),
            *rust_mmap_mm_end_data(mm),
            *rust_mmap_mm_start_data(mm),
        ) != 0
        {
            break 'operation false;
        }
        if oldbrk == newbrk {
            *rust_mmap_mm_brk(mm) = brk;
            break 'operation true;
        }
        let mut vmi: vma_iterator = zeroed();
        if brk <= origbrk {
            rust_mmap_vma_iter_init(&mut vmi, mm, newbrk);
            let v = rust_mmap_vma_find(&mut vmi, oldbrk);
            if v.is_null() || start(v) >= oldbrk {
                break 'operation false;
            }
            *rust_mmap_mm_brk(mm) = brk;
            if do_vmi_align_munmap(&mut vmi, v, mm, newbrk, oldbrk, &mut uf, true) != 0 {
                break 'operation false;
            }
            unlocked = true;
            break 'operation true;
        }
        if check_brk_limits(oldbrk, newbrk.wrapping_sub(oldbrk)) != 0 {
            break 'operation false;
        }
        rust_mmap_vma_iter_init(&mut vmi, mm, oldbrk);
        let next = rust_mmap_vma_find(
            &mut vmi,
            newbrk.wrapping_add(PAGE_SIZE).wrapping_add(stack_guard_gap),
        );
        if !next.is_null() && newbrk.wrapping_add(PAGE_SIZE) > rust_mmap_vm_start_gap(next) {
            break 'operation false;
        }
        let v = rust_mmap_vma_prev_limit(&mut vmi, *rust_mmap_mm_start_brk(mm));
        if do_brk_flags(
            &mut vmi,
            v,
            oldbrk,
            newbrk.wrapping_sub(oldbrk),
            empty_flags(),
        ) < 0
        {
            break 'operation false;
        }
        *rust_mmap_mm_brk(mm) = brk;
        populate = flags_test(rust_mmap_mm_def_vma_flags(mm), VMA_LOCKED_BIT as _);
        true
    };
    if !success {
        *rust_mmap_mm_brk(mm) = origbrk;
        rust_mmap_mmap_write_unlock(mm);
        return origbrk as c_long;
    }
    if !unlocked {
        rust_mmap_mmap_write_unlock(mm);
    }
    rust_mmap_userfaultfd_unmap_complete(mm, &mut uf);
    if populate {
        rust_mmap_mm_populate(oldbrk, newbrk.wrapping_sub(oldbrk));
    }
    brk as c_long
}
unsafe fn round_hint_to_min(hint: c_ulong) -> c_ulong {
    let hint = hint & PAGE_MASK;
    if hint != 0 && hint < mmap_min_addr {
        page_align(mmap_min_addr)
    } else {
        hint
    }
}
#[no_mangle]
pub unsafe extern "C" fn mlock_future_ok(
    mm: *const mm_struct,
    locked: bool,
    bytes: c_ulong,
) -> bool {
    if !locked || capable(CAP_IPC_LOCK as c_int) {
        return true;
    }
    let locked_pages = (bytes >> PAGE_SHIFT).wrapping_add(*rust_mmap_mm_locked_vm(mm.cast_mut()));
    locked_pages <= (rust_mmap_rlimit(RLIMIT_MEMLOCK) >> PAGE_SHIFT)
}
unsafe fn file_mmap_size_max(file: *mut file, inode: *mut inode) -> u64 {
    if rust_mmap_is_regular((*inode).i_mode)
        || rust_mmap_is_block((*inode).i_mode)
        || rust_mmap_is_socket((*inode).i_mode)
    {
        return RUST_MMAP_MAX_LFS_FILESIZE as u64;
    }
    if (*(*file).f_op).fop_flags & RUST_MMAP_FOP_UNSIGNED_OFFSET as u32 != 0 {
        return 0;
    }
    ULONG_MAX as u64
}
unsafe fn file_mmap_ok(file: *mut file, inode: *mut inode, pgoff: c_ulong, len: c_ulong) -> bool {
    let max = file_mmap_size_max(file, inode);
    if max != 0 && len as u64 > max {
        return false;
    }
    pgoff as u64 <= (max.wrapping_sub(len as u64) >> PAGE_SHIFT)
}
#[no_mangle]
pub unsafe extern "C" fn do_mmap(
    file: *mut file,
    mut addr: c_ulong,
    mut len: c_ulong,
    mut prot: c_ulong,
    mut flags: c_ulong,
    mut vma_flags: vma_flags_t,
    mut pgoff: c_ulong,
    populate: *mut c_ulong,
    uf: *mut list_head,
) -> c_ulong {
    let mm = rust_mmap_current_mm();
    let mut pkey = 0;
    *populate = 0;
    rust_mmap_mmap_assert_write_locked(mm);
    if len == 0 {
        return err(EINVAL);
    }
    if prot & PROT_READ as c_ulong != 0
        && rust_mmap_personality() & READ_IMPLIES_EXEC != 0
        && (file.is_null() || !rust_mmap_path_noexec(rust_mmap_file_f_path(file)))
    {
        prot |= PROT_EXEC as c_ulong;
    }
    if flags & MAP_FIXED_NOREPLACE as c_ulong != 0 {
        flags |= MAP_FIXED as c_ulong;
    }
    if flags & MAP_FIXED as c_ulong == 0 {
        addr = round_hint_to_min(addr);
    }
    len = page_align(len);
    if len == 0 {
        return err(ENOMEM);
    }
    if pgoff.wrapping_add(len >> PAGE_SHIFT) < pgoff {
        return err(EOVERFLOW);
    }
    if *rust_mmap_mm_map_count(mm) > rust_mmap_max_map_count() {
        return err(ENOMEM);
    }
    if prot == PROT_EXEC as c_ulong {
        pkey = rust_mmap_execute_only_pkey(mm);
        if pkey < 0 {
            pkey = 0;
        }
    }
    rust_mmap_flags_set_mask(
        &mut vma_flags,
        rust_mmap_legacy_to_flags(rust_mmap_calc_prot(prot, pkey as _)),
    );
    rust_mmap_flags_set_mask(
        &mut vma_flags,
        rust_mmap_legacy_to_flags(rust_mmap_calc_flags(file, flags)),
    );
    rust_mmap_flags_set_mask(&mut vma_flags, *rust_mmap_mm_def_vma_flags(mm));
    set(&mut vma_flags, VMA_MAYREAD_BIT as _);
    set(&mut vma_flags, VMA_MAYWRITE_BIT as _);
    set(&mut vma_flags, VMA_MAYEXEC_BIT as _);
    addr = __get_unmapped_area(file, addr, len, pgoff, flags, vma_flags);
    if is_err(addr) {
        return addr;
    }
    if flags & MAP_FIXED_NOREPLACE as c_ulong != 0
        && !find_vma_intersection(mm, addr, addr.wrapping_add(len)).is_null()
    {
        return err(EEXIST);
    }
    if flags & MAP_LOCKED as c_ulong != 0 && !rust_mmap_can_do_mlock() {
        return err(EPERM);
    }
    if !mlock_future_ok(mm, flags_test(&vma_flags, VMA_LOCKED_BIT as _), len) {
        return err(EAGAIN);
    }
    if !file.is_null() {
        let inode = rust_mmap_file_inode(file);
        if !file_mmap_ok(file, inode, pgoff, len) {
            return err(EOVERFLOW);
        }
        let mut flags_mask = RUST_MMAP_LEGACY_MAP_MASK as c_ulong;
        if (*(*file).f_op).fop_flags & RUST_MMAP_FOP_MMAP_SYNC as u32 != 0 {
            flags_mask |= MAP_SYNC as c_ulong;
        }
        let kind = flags & MAP_TYPE as c_ulong;
        if kind == MAP_SHARED as c_ulong || kind == MAP_SHARED_VALIDATE as c_ulong {
            if kind == MAP_SHARED as c_ulong {
                flags &= RUST_MMAP_LEGACY_MAP_MASK as c_ulong;
            }
            if flags & !flags_mask != 0 {
                return err(EOPNOTSUPP);
            }
            if prot & PROT_WRITE as c_ulong != 0 {
                if (*file).f_mode & RUST_MMAP_FMODE_WRITE as u32 == 0 {
                    return err(EACCES);
                }
                if rust_mmap_is_swapfile((*(*file).f_mapping).host) {
                    return err(ETXTBSY);
                }
            }
            if rust_mmap_is_append(inode) && (*file).f_mode & RUST_MMAP_FMODE_WRITE as u32 != 0 {
                return err(EACCES);
            }
            set(&mut vma_flags, VMA_SHARED_BIT as _);
            set(&mut vma_flags, VMA_MAYSHARE_BIT as _);
            if (*file).f_mode & RUST_MMAP_FMODE_WRITE as u32 == 0 {
                clear(&mut vma_flags, VMA_MAYWRITE_BIT as _);
                clear(&mut vma_flags, VMA_SHARED_BIT as _);
            }
        } else if kind != MAP_PRIVATE as c_ulong {
            return err(EINVAL);
        }
        if (*file).f_mode & RUST_MMAP_FMODE_READ as u32 == 0 {
            return err(EACCES);
        }
        if rust_mmap_path_noexec(rust_mmap_file_f_path(file)) {
            if flags_test(&vma_flags, VMA_EXEC_BIT as _) {
                return err(EPERM);
            }
            clear(&mut vma_flags, VMA_MAYEXEC_BIT as _);
        }
        if !rust_mmap_can_mmap_file(file) {
            return err(ENODEV);
        }
        if rust_mmap_flags_can_grow(&vma_flags) {
            return err(EINVAL);
        }
        let ret = rust_mmap_memfd_check_seals_mmap(file, &mut vma_flags);
        if ret != 0 {
            return ret as c_ulong;
        }
    } else {
        let kind = flags & MAP_TYPE as c_ulong;
        if kind == MAP_SHARED as c_ulong {
            if rust_mmap_flags_can_grow(&vma_flags) {
                return err(EINVAL);
            }
            pgoff = 0;
            set(&mut vma_flags, VMA_SHARED_BIT as _);
            set(&mut vma_flags, VMA_MAYSHARE_BIT as _);
        } else {
            if kind == MAP_DROPPABLE as c_ulong {
                let droppable = rust_mmap_droppable_flags();
                if rust_mmap_flags_empty(&droppable) {
                    return err(EOPNOTSUPP);
                }
                rust_mmap_flags_set_mask(&mut vma_flags, droppable);
                if flags & (MAP_LOCKED | MAP_HUGETLB) as c_ulong != 0 {
                    return err(EINVAL);
                }
                if rust_mmap_flags_can_grow(&vma_flags) {
                    return err(EINVAL);
                }
                set(&mut vma_flags, VMA_NORESERVE_BIT as _);
                set(&mut vma_flags, VMA_WIPEONFORK_BIT as _);
                set(&mut vma_flags, VMA_DONTDUMP_BIT as _);
            } else if kind != MAP_PRIVATE as c_ulong {
                return err(EINVAL);
            }
            pgoff = addr >> PAGE_SHIFT;
        }
    }
    if flags & MAP_NORESERVE as c_ulong != 0 {
        if sysctl_overcommit_memory != OVERCOMMIT_NEVER as c_int {
            set(&mut vma_flags, VMA_NORESERVE_BIT as _);
        }
        if !file.is_null() && rust_mmap_is_file_hugepages(file) {
            set(&mut vma_flags, VMA_NORESERVE_BIT as _);
        }
    }
    addr = mmap_region(file, addr, len, vma_flags, pgoff, uf);
    if !is_err(addr)
        && (flags_test(&vma_flags, VMA_LOCKED_BIT as _)
            || flags & (MAP_POPULATE | MAP_NONBLOCK) as c_ulong == MAP_POPULATE as c_ulong)
    {
        *populate = len;
    }
    addr
}
#[no_mangle]
pub unsafe extern "C" fn ksys_mmap_pgoff(
    addr: c_ulong,
    mut len: c_ulong,
    prot: c_ulong,
    flags: c_ulong,
    fd: c_ulong,
    pgoff: c_ulong,
) -> c_ulong {
    let mut file: *mut file = null_mut();
    if flags & MAP_ANONYMOUS as c_ulong == 0 {
        rust_mmap_audit_mmap_fd(fd, flags);
        file = fget(fd as _);
        if file.is_null() {
            return err(EBADF);
        }
        if rust_mmap_is_file_hugepages(file) {
            len = align(len, rust_mmap_huge_page_size(rust_mmap_hstate_file(file)));
        } else if flags & MAP_HUGETLB as c_ulong != 0 {
            fput(file);
            return err(EINVAL);
        }
    } else if flags & MAP_HUGETLB as c_ulong != 0 {
        let log = (flags >> MAP_HUGE_SHIFT) & MAP_HUGE_MASK as c_ulong;
        let hs = rust_mmap_hstate_sizelog(log as _);
        if hs.is_null() {
            return err(EINVAL);
        }
        len = align(len, rust_mmap_huge_page_size(hs));
        let mut vf = empty_flags();
        set(&mut vf, VMA_NORESERVE_BIT as _);
        file = rust_mmap_hugetlb_file_setup(len as _, vf, log as _);
        if is_err(file as c_ulong) {
            return file as c_ulong;
        }
    }
    let retval = vm_mmap_pgoff(file, addr, len, prot, flags, pgoff);
    if !file.is_null() {
        fput(file);
    }
    retval
}
#[no_mangle]
pub unsafe extern "C" fn rust_mmap_sys_mmap_pgoff(
    addr: c_ulong,
    len: c_ulong,
    prot: c_ulong,
    flags: c_ulong,
    fd: c_ulong,
    pgoff: c_ulong,
) -> c_long {
    ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff) as c_long
}
#[cfg(RUST_MMAP_ARCH_WANT_SYS_OLD_MMAP)]
#[no_mangle]
pub unsafe extern "C" fn rust_mmap_sys_old_mmap(arg: *mut mmap_arg_struct) -> c_long {
    let mut a: mmap_arg_struct = zeroed();
    if rust_mmap_copy_from_user(
        addr_of_mut!(a).cast(),
        arg.cast(),
        size_of::<mmap_arg_struct>() as _,
    ) != 0
    {
        return -(EFAULT as c_long);
    }
    if a.offset & !PAGE_MASK != 0 {
        return -(EINVAL as c_long);
    }
    ksys_mmap_pgoff(a.addr, a.len, a.prot, a.flags, a.fd, a.offset >> PAGE_SHIFT) as c_long
}
unsafe fn stack_guard_placement(f: vma_flags_t) -> c_ulong {
    if rust_mmap_flags_test_single(&f, rust_mmap_shadow_stack_flags()) {
        PAGE_SIZE
    } else {
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn vm_unmapped_area(info: *mut vm_unmapped_area_info) -> c_ulong {
    let addr = if (*info).flags & VM_UNMAPPED_AREA_TOPDOWN as c_ulong != 0 {
        unmapped_area_topdown(info)
    } else {
        unmapped_area(info)
    };
    rust_mmap_trace_unmapped_area(addr, info);
    addr
}
#[no_mangle]
pub unsafe extern "C" fn generic_get_unmapped_area(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    _pgoff: c_ulong,
    flags: c_ulong,
    vma_flags: vma_flags_t,
) -> c_ulong {
    let mm = rust_mmap_current_mm();
    let mmap_end = rust_mmap_arch_mmap_end(addr, len, flags);
    if len > mmap_end.wrapping_sub(mmap_min_addr) {
        return err(ENOMEM);
    }
    if flags & MAP_FIXED as c_ulong != 0 {
        return addr;
    }
    if addr != 0 {
        addr = page_align(addr);
        let mut prev = null_mut();
        let vma = find_vma_prev(mm, addr, &mut prev);
        if mmap_end.wrapping_sub(len) >= addr
            && addr >= mmap_min_addr
            && (vma.is_null() || addr.wrapping_add(len) <= rust_mmap_vm_start_gap(vma))
            && (prev.is_null() || addr >= rust_mmap_vm_end_gap(prev))
        {
            return addr;
        }
    }
    let mut info: vm_unmapped_area_info = zeroed();
    info.length = len;
    info.low_limit = *rust_mmap_mm_mmap_base(mm);
    info.high_limit = mmap_end;
    info.start_gap = stack_guard_placement(vma_flags);
    if !filp.is_null() && rust_mmap_is_file_hugepages(filp) {
        info.align_mask = rust_mmap_huge_page_mask_align(filp);
    }
    vm_unmapped_area(&mut info)
}
#[cfg(not(RUST_MMAP_HAVE_ARCH_UNMAPPED_AREA))]
#[no_mangle]
pub unsafe extern "C" fn arch_get_unmapped_area(
    f: *mut file,
    a: c_ulong,
    l: c_ulong,
    p: c_ulong,
    flags: c_ulong,
    vf: vm_flags_t,
) -> c_ulong {
    generic_get_unmapped_area(f, a, l, p, flags, rust_mmap_legacy_to_flags(vf))
}
#[no_mangle]
pub unsafe extern "C" fn generic_get_unmapped_area_topdown(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    _pgoff: c_ulong,
    flags: c_ulong,
    vma_flags: vma_flags_t,
) -> c_ulong {
    let mm = rust_mmap_current_mm();
    let mmap_end = rust_mmap_arch_mmap_end(addr, len, flags);
    if len > mmap_end.wrapping_sub(mmap_min_addr) {
        return err(ENOMEM);
    }
    if flags & MAP_FIXED as c_ulong != 0 {
        return addr;
    }
    if addr != 0 {
        addr = page_align(addr);
        let mut prev = null_mut();
        let vma = find_vma_prev(mm, addr, &mut prev);
        if mmap_end.wrapping_sub(len) >= addr
            && addr >= mmap_min_addr
            && (vma.is_null() || addr.wrapping_add(len) <= rust_mmap_vm_start_gap(vma))
            && (prev.is_null() || addr >= rust_mmap_vm_end_gap(prev))
        {
            return addr;
        }
    }
    let mut info: vm_unmapped_area_info = zeroed();
    info.flags = VM_UNMAPPED_AREA_TOPDOWN as _;
    info.length = len;
    info.low_limit = PAGE_SIZE;
    info.high_limit = rust_mmap_arch_mmap_base(addr, *rust_mmap_mm_mmap_base(mm));
    info.start_gap = stack_guard_placement(vma_flags);
    if !filp.is_null() && rust_mmap_is_file_hugepages(filp) {
        info.align_mask = rust_mmap_huge_page_mask_align(filp);
    }
    addr = vm_unmapped_area(&mut info);
    if addr & !PAGE_MASK != 0 {
        rust_mmap_bug_bad_topdown(addr != err(ENOMEM));
        info.flags = 0;
        info.low_limit = rust_mmap_task_unmapped_base();
        info.high_limit = mmap_end;
        addr = vm_unmapped_area(&mut info);
    }
    addr
}
#[cfg(not(RUST_MMAP_HAVE_ARCH_UNMAPPED_AREA_TOPDOWN))]
#[no_mangle]
pub unsafe extern "C" fn arch_get_unmapped_area_topdown(
    f: *mut file,
    a: c_ulong,
    l: c_ulong,
    p: c_ulong,
    flags: c_ulong,
    vf: vm_flags_t,
) -> c_ulong {
    generic_get_unmapped_area_topdown(f, a, l, p, flags, rust_mmap_legacy_to_flags(vf))
}
#[no_mangle]
pub unsafe extern "C" fn mm_get_unmapped_area_vmaflags(
    f: *mut file,
    a: c_ulong,
    l: c_ulong,
    p: c_ulong,
    flags: c_ulong,
    vf: vma_flags_t,
) -> c_ulong {
    if rust_mmap_mm_flags_test(MMF_TOPDOWN as _, rust_mmap_current_mm()) {
        arch_get_unmapped_area_topdown(f, a, l, p, flags, rust_mmap_flags_to_legacy(vf))
    } else {
        arch_get_unmapped_area(f, a, l, p, flags, rust_mmap_flags_to_legacy(vf))
    }
}
#[no_mangle]
pub unsafe extern "C" fn __get_unmapped_area(
    file: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    mut pgoff: c_ulong,
    flags: c_ulong,
    vf: vma_flags_t,
) -> c_ulong {
    let error = rust_mmap_arch_mmap_check(addr, len, flags);
    if error != 0 {
        return error;
    }
    if len > rust_mmap_task_size() {
        return err(ENOMEM);
    }
    let mut get_area: Option<
        unsafe extern "C" fn(*mut file, c_ulong, c_ulong, c_ulong, c_ulong) -> c_ulong,
    > = None;
    if !file.is_null() {
        get_area = (*(*file).f_op).get_unmapped_area;
    } else if flags & MAP_SHARED as c_ulong != 0 {
        get_area = Some(shmem_get_unmapped_area);
    }
    if file.is_null() {
        pgoff = 0;
    }
    if let Some(get_area) = get_area {
        addr = get_area(file, addr, len, pgoff, flags);
    } else {
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        if file.is_null() && addr == 0 && len & ((RUST_MMAP_PMD_SIZE as c_ulong) - 1) == 0 {
            addr = rust_mmap_thp_area(file, addr, len, pgoff, flags, vf);
        } else {
            addr = mm_get_unmapped_area_vmaflags(file, addr, len, pgoff, flags, vf);
        }
        #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            addr = mm_get_unmapped_area_vmaflags(file, addr, len, pgoff, flags, vf);
        }
    }
    if is_err(addr) {
        return addr;
    }
    if addr > rust_mmap_task_size().wrapping_sub(len) {
        return err(ENOMEM);
    }
    if addr & !PAGE_MASK != 0 {
        return err(EINVAL);
    }
    let error = rust_mmap_security_mmap_addr(addr);
    if error != 0 {
        error as c_ulong
    } else {
        addr
    }
}
#[no_mangle]
pub unsafe extern "C" fn mm_get_unmapped_area(
    file: *mut file,
    addr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
) -> c_ulong {
    mm_get_unmapped_area_vmaflags(file, addr, len, pgoff, flags, empty_flags())
}
#[no_mangle]
pub unsafe extern "C" fn find_vma_intersection(
    mm: *mut mm_struct,
    mut a: c_ulong,
    e: c_ulong,
) -> *mut vm_area_struct {
    rust_mmap_mmap_assert_locked(mm);
    mt_find(rust_mmap_mm_mm_mt(mm), &mut a, e.wrapping_sub(1)).cast()
}
#[no_mangle]
pub unsafe extern "C" fn find_vma(mm: *mut mm_struct, mut addr: c_ulong) -> *mut vm_area_struct {
    rust_mmap_mmap_assert_locked(mm);
    mt_find(rust_mmap_mm_mm_mt(mm), &mut addr, ULONG_MAX).cast()
}
#[no_mangle]
pub unsafe extern "C" fn find_vma_prev(
    mm: *mut mm_struct,
    addr: c_ulong,
    prev: *mut *mut vm_area_struct,
) -> *mut vm_area_struct {
    let mut vmi = iterator(mm, addr);
    let mut vma = rust_mmap_vma_iter_load(&mut vmi);
    *prev = rust_mmap_vma_prev(&mut vmi);
    if vma.is_null() {
        vma = rust_mmap_vma_next(&mut vmi);
    }
    vma
}
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn cmdline_parse_stack_guard_gap(p: *mut c_char) -> c_int {
    let mut endptr: *mut c_char = null_mut();
    let val = simple_strtoul(p, &mut endptr, 10);
    if *endptr == 0 {
        stack_guard_gap = val.wrapping_shl(PAGE_SHIFT);
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn expand_stack_locked(vma: *mut vm_area_struct, address: c_ulong) -> c_int {
    #[cfg(CONFIG_STACK_GROWSUP)]
    {
        expand_upwards(vma, address)
    }
    #[cfg(not(CONFIG_STACK_GROWSUP))]
    {
        expand_downwards(vma, address)
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_extend_vma_locked(
    mm: *mut mm_struct,
    addr: c_ulong,
) -> *mut vm_area_struct {
    let addr = addr & PAGE_MASK;
    #[cfg(CONFIG_STACK_GROWSUP)]
    {
        let mut prev = null_mut();
        let vma = find_vma_prev(mm, addr, &mut prev);
        if !vma.is_null() && start(vma) <= addr {
            return vma;
        }
        if prev.is_null() || expand_stack_locked(prev, addr) != 0 {
            return null_mut();
        }
        if test(prev, VMA_LOCKED_BIT as _) {
            populate_vma_page_range(prev, addr, end(prev), null_mut());
        }
        prev
    }
    #[cfg(not(CONFIG_STACK_GROWSUP))]
    {
        let vma = find_vma(mm, addr);
        if vma.is_null() {
            return null_mut();
        }
        if start(vma) <= addr {
            return vma;
        }
        let old_start = start(vma);
        if expand_stack_locked(vma, addr) != 0 {
            return null_mut();
        }
        if test(vma, VMA_LOCKED_BIT as _) {
            populate_vma_page_range(vma, addr, old_start, null_mut());
        }
        vma
    }
}
#[no_mangle]
pub unsafe extern "C" fn expand_stack(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct {
    rust_mmap_mmap_read_unlock(mm);
    if rust_mmap_mmap_write_lock_killable(mm) != 0 {
        return null_mut();
    }
    let mut prev = null_mut();
    let mut vma = find_vma_prev(mm, addr, &mut prev);
    let mut success = !vma.is_null() && start(vma) <= addr;
    #[cfg(CONFIG_STACK_GROWSUP)]
    if !success && !prev.is_null() && expand_upwards(prev, addr) == 0 {
        vma = prev;
        success = true;
    }
    #[cfg(not(CONFIG_STACK_GROWSUP))]
    if !success && !vma.is_null() && expand_downwards(vma, addr) == 0 {
        success = true;
    }
    if success {
        rust_mmap_mmap_write_downgrade(mm);
        vma
    } else {
        rust_mmap_mmap_write_unlock(mm);
        null_mut()
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_munmap(
    mm: *mut mm_struct,
    start: c_ulong,
    len: usize,
    uf: *mut list_head,
) -> c_int {
    let mut vmi = iterator(mm, start);
    do_vmi_munmap(&mut vmi, mm, start, len, uf, false)
}
#[no_mangle]
pub unsafe extern "C" fn vm_munmap(start: c_ulong, len: usize) -> c_int {
    __vm_munmap(start, len, false)
}
#[no_mangle]
pub unsafe extern "C" fn rust_mmap_sys_munmap(addr: c_ulong, len: usize) -> c_long {
    __vm_munmap(rust_mmap_untagged_addr(addr), len, true) as c_long
}
#[no_mangle]
pub unsafe extern "C" fn rust_mmap_sys_remap_file_pages(
    mut start_addr: c_ulong,
    mut size: c_ulong,
    mut prot: c_ulong,
    pgoff: c_ulong,
    mut map_flags: c_ulong,
) -> c_long {
    let mm = rust_mmap_current_mm();
    let mut populate = 0;
    rust_mmap_warn_remap();
    if prot != 0 {
        return -(EINVAL as c_long);
    }
    start_addr &= PAGE_MASK;
    size &= PAGE_MASK;
    let finish = start_addr.wrapping_add(size);
    if finish <= start_addr || pgoff.wrapping_add(size >> PAGE_SHIFT) < pgoff {
        return -(EINVAL as c_long);
    }
    if rust_mmap_mmap_read_lock_killable(mm) != 0 {
        return -(EINTR as c_long);
    }
    let mut vma = rust_mmap_vma_lookup(mm, start_addr);
    if vma.is_null() || !test(vma, VMA_SHARED_BIT as _) {
        rust_mmap_mmap_read_unlock(mm);
        return -(EINVAL as c_long);
    }
    if test(vma, VMA_READ_BIT as _) {
        prot |= PROT_READ as c_ulong;
    }
    if test(vma, VMA_WRITE_BIT as _) {
        prot |= PROT_WRITE as c_ulong;
    }
    if test(vma, VMA_EXEC_BIT as _) {
        prot |= PROT_EXEC as c_ulong;
    }
    map_flags &= MAP_NONBLOCK as c_ulong;
    map_flags |= (MAP_SHARED | MAP_FIXED | MAP_POPULATE) as c_ulong;
    if test(vma, VMA_LOCKED_BIT as _) {
        map_flags |= MAP_LOCKED as c_ulong;
    }
    let vm_flags = flags(vma);
    let file = rust_mmap_get_file((*vma).vm_file);
    rust_mmap_mmap_read_unlock(mm);
    let check = rust_mmap_security_mmap_file(file, prot, map_flags);
    if check != 0 {
        fput(file);
        return check as c_long;
    }
    if rust_mmap_mmap_write_lock_killable(mm) != 0 {
        fput(file);
        return -(EINTR as c_long);
    }
    let mut ret = err(EINVAL);
    'mapping: {
        vma = rust_mmap_vma_lookup(mm, start_addr);
        if vma.is_null() || flags(vma) != vm_flags || (*vma).vm_file != file {
            break 'mapping;
        }
        if finish > end(vma) {
            let mut vmi = iterator(mm, end(vma));
            let mut prev = vma;
            loop {
                let next = rust_mmap_vma_find(&mut vmi, finish);
                if next.is_null() {
                    break 'mapping;
                }
                if start(next) != end(prev)
                    || (*next).vm_file != (*vma).vm_file
                    || flags(next) != flags(vma)
                {
                    break 'mapping;
                }
                if finish <= end(next) {
                    break;
                }
                prev = next;
            }
        }
        ret = do_mmap(
            (*vma).vm_file,
            start_addr,
            size,
            prot,
            map_flags,
            empty_flags(),
            pgoff,
            &mut populate,
            null_mut(),
        );
    }
    rust_mmap_mmap_write_unlock(mm);
    fput(file);
    if populate != 0 {
        rust_mmap_mm_populate(ret, populate);
    }
    if !is_err(ret) {
        ret = 0;
    }
    ret as c_long
}
#[no_mangle]
pub unsafe extern "C" fn vm_brk_flags(addr: c_ulong, request: c_ulong, is_exec: bool) -> c_int {
    let mut vf = empty_flags();
    if is_exec {
        set(&mut vf, VMA_EXEC_BIT as _);
    }
    let mm = rust_mmap_current_mm();
    let mut uf: list_head = zeroed();
    init_list(&mut uf);
    let mut vmi = iterator(mm, addr);
    let len = page_align(request);
    if len < request {
        return -(ENOMEM as c_int);
    }
    if len == 0 {
        return 0;
    }
    if rust_mmap_mmap_write_lock_killable(mm) != 0 {
        return -(EINTR as c_int);
    }
    let mut ret = check_brk_limits(addr, len);
    if ret == 0 {
        ret = do_vmi_munmap(&mut vmi, mm, addr, len as usize, &mut uf, false);
    }
    if ret != 0 {
        rust_mmap_mmap_write_unlock(mm);
        return ret;
    }
    let vma = rust_mmap_vma_prev(&mut vmi);
    ret = do_brk_flags(&mut vmi, vma, addr, len, vf);
    let populate = flags_test(rust_mmap_mm_def_vma_flags(mm), VMA_LOCKED_BIT as _);
    rust_mmap_mmap_write_unlock(mm);
    rust_mmap_userfaultfd_unmap_complete(mm, &mut uf);
    if populate && ret == 0 {
        rust_mmap_mm_populate(addr, len);
    }
    ret
}
unsafe fn tear_down_vmas(
    mm: *mut mm_struct,
    vmi: *mut vma_iterator,
    mut vma: *mut vm_area_struct,
    limit: c_ulong,
) -> c_ulong {
    let mut nr_accounted: c_ulong = 0;
    let mut count: c_int = 0;
    rust_mmap_mmap_assert_write_locked(mm);
    rust_mmap_vma_iter_set(vmi, end(vma));
    loop {
        if test(vma, VMA_ACCOUNT_BIT as _) {
            nr_accounted = nr_accounted.wrapping_add(rust_mmap_vma_pages(vma));
        }
        rust_mmap_vma_mark_detached(vma);
        remove_vma(vma);
        count += 1;
        rust_mmap_cond_resched();
        vma = rust_mmap_vma_next(vmi);
        if vma.is_null() || end(vma) > limit {
            break;
        }
    }
    rust_mmap_warn_vma_count(count != *rust_mmap_mm_map_count(mm));
    nr_accounted
}
// Native header UNMAP_STATE/unmap_all_init/unmap_pgtable_init initializers are
// expressed here so their range and tree-walk policy remains Rust-owned too.
unsafe fn unmap_all(vmi: *mut vma_iterator, vma: *mut vm_area_struct) -> unmap_desc {
    unmap_desc {
        mas: addr_of_mut!((*vmi).mas),
        first: vma,
        pg_start: RUST_MMAP_FIRST_USER_ADDRESS as _,
        pg_end: RUST_MMAP_USER_PGTABLES_CEILING as _,
        vma_start: 0,
        vma_end: ULONG_MAX,
        tree_end: ULONG_MAX,
        tree_reset: end(vma),
        mm_wr_locked: false,
    }
}
unsafe fn unmap_pgtable(unmap: *mut unmap_desc, vmi: *mut vma_iterator) {
    rust_mmap_vma_iter_set(vmi, (*unmap).tree_reset);
    (*unmap).vma_start = RUST_MMAP_FIRST_USER_ADDRESS as _;
    (*unmap).vma_end = RUST_MMAP_USER_PGTABLES_CEILING as _;
    (*unmap).tree_end = RUST_MMAP_USER_PGTABLES_CEILING as _;
}
#[no_mangle]
pub unsafe extern "C" fn exit_mmap(mm: *mut mm_struct) {
    let mut vmi = iterator(mm, 0);
    let mut nr_accounted = 0;
    rust_mmap_mmu_notifier_release(mm);
    rust_mmap_mmap_read_lock(mm);
    rust_mmap_arch_exit_mmap(mm);
    let vma = rust_mmap_vma_next(&mut vmi);
    if vma.is_null() {
        rust_mmap_mmap_read_unlock(mm);
        rust_mmap_mmap_write_lock(mm);
    } else {
        let mut unmap = unmap_all(&mut vmi, vma);
        let mut tlb: mmu_gather = zeroed();
        rust_mmap_flush_cache_mm(mm);
        rust_mmap_tlb_gather_fullmm(&mut tlb, mm);
        unmap_vmas(&mut tlb, &mut unmap);
        rust_mmap_mmap_read_unlock(mm);
        rust_mmap_mm_flags_set(MMF_OOM_SKIP as _, mm);
        rust_mmap_mmap_write_lock(mm);
        unmap.mm_wr_locked = true;
        rust_mmap_mt_clear_in_rcu(rust_mmap_mm_mm_mt(mm));
        unmap_pgtable(&mut unmap, &mut vmi);
        free_pgtables(&mut tlb, &mut unmap);
        rust_mmap_tlb_finish(&mut tlb);
        nr_accounted = tear_down_vmas(mm, &mut vmi, vma, ULONG_MAX);
    }
    __mt_destroy(rust_mmap_mm_mm_mt(mm));
    rust_mmap_trace_exit(mm);
    rust_mmap_mmap_write_unlock(mm);
    rust_mmap_vm_unacct_memory(nr_accounted as _);
}
#[no_mangle]
pub unsafe extern "C" fn may_expand_vm(
    mm: *mut mm_struct,
    vf: *const vma_flags_t,
    npages: c_ulong,
) -> bool {
    if (*rust_mmap_mm_total_vm(mm)).wrapping_add(npages) > rust_mmap_rlimit(RLIMIT_AS) >> PAGE_SHIFT
    {
        return false;
    }
    let data = (*rust_mmap_mm_data_vm(mm)).wrapping_add(npages);
    if rust_mmap_is_data_flags(vf) && data > rust_mmap_rlimit(RLIMIT_DATA) >> PAGE_SHIFT {
        if rust_mmap_rlimit(RLIMIT_DATA) == 0
            && data <= rust_mmap_rlimit_max(RLIMIT_DATA) >> PAGE_SHIFT
        {
            return true;
        }
        let suffix = if ignore_rlimit_data {
            c"".as_ptr().cast()
        } else {
            c" or use boot option ignore_rlimit_data".as_ptr().cast()
        };
        rust_mmap_warn_data(
            data.wrapping_shl(PAGE_SHIFT),
            rust_mmap_rlimit(RLIMIT_DATA),
            suffix,
        );
        if !ignore_rlimit_data {
            return false;
        }
    }
    true
}
#[no_mangle]
pub unsafe extern "C" fn vm_stat_account(mm: *mut mm_struct, flags: vm_flags_t, npages: c_long) {
    rust_mmap_total_vm_write_once(
        mm,
        rust_mmap_total_vm_read_once(mm).wrapping_add(npages as c_ulong),
    );
    let field = if rust_mmap_is_exec_mapping(flags) {
        rust_mmap_mm_exec_vm(mm)
    } else if rust_mmap_is_stack_mapping(flags) {
        rust_mmap_mm_stack_vm(mm)
    } else if rust_mmap_is_data_mapping(flags) {
        rust_mmap_mm_data_vm(mm)
    } else {
        return;
    };
    *field = (*field).wrapping_add(npages as c_ulong);
}
unsafe extern "C" fn special_mapping_close(vma: *mut vm_area_struct) {
    let sm = (*vma).vm_private_data.cast::<vm_special_mapping>();
    if let Some(close) = (*sm).close {
        close(sm, vma);
    }
}
unsafe extern "C" fn special_mapping_name(vma: *mut vm_area_struct) -> *const c_char {
    (*(*vma).vm_private_data.cast::<vm_special_mapping>()).name
}
unsafe extern "C" fn special_mapping_mremap(vma: *mut vm_area_struct) -> c_int {
    let sm = (*vma).vm_private_data.cast::<vm_special_mapping>();
    if rust_mmap_warn_special_mm(rust_mmap_current_mm() != (*vma).vm_mm) {
        return -(EFAULT as c_int);
    }
    if let Some(mremap) = (*sm).mremap {
        return mremap(sm, vma);
    }
    0
}
unsafe extern "C" fn special_mapping_split(_vma: *mut vm_area_struct, _addr: c_ulong) -> c_int {
    -(EINVAL as c_int)
}
#[repr(transparent)]
struct MmapVmOps(vm_operations_struct);
unsafe impl Sync for MmapVmOps {}
static special_mapping_vmops: MmapVmOps = MmapVmOps({
    let mut ops: vm_operations_struct = unsafe { zeroed() };
    ops.close = Some(special_mapping_close);
    ops.fault = Some(special_mapping_fault);
    ops.mremap = Some(special_mapping_mremap);
    ops.name = Some(special_mapping_name);
    ops.access = None;
    ops.may_split = Some(special_mapping_split);
    ops
});
unsafe extern "C" fn special_mapping_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let vma = *rust_mmap_vmf_vma(vmf);
    let sm = (*vma).vm_private_data.cast::<vm_special_mapping>();
    if let Some(fault) = (*sm).fault {
        return fault(sm, vma, vmf);
    }
    let mut pages = (*sm).pages;
    let mut pgoff = *rust_mmap_vmf_pgoff(vmf);
    while pgoff != 0 && !(*pages).is_null() {
        pages = pages.add(1);
        pgoff -= 1;
    }
    if !(*pages).is_null() {
        let page = *pages;
        rust_mmap_get_page(page);
        (*vmf).page = page;
        return 0;
    }
    RUST_MMAP_VM_FAULT_SIGBUS as _
}
#[no_mangle]
pub unsafe extern "C" fn vma_is_special_mapping(
    vma: *const vm_area_struct,
    sm: *const vm_special_mapping,
) -> bool {
    (*vma).vm_private_data.cast_const() == sm.cast()
        && (*vma).vm_ops == addr_of!(special_mapping_vmops.0)
}
#[no_mangle]
pub unsafe extern "C" fn _install_special_mapping(
    mm: *mut mm_struct,
    addr: c_ulong,
    len: c_ulong,
    vf: vm_flags_t,
    spec: *const vm_special_mapping,
) -> *mut vm_area_struct {
    __install_special_mapping(
        mm,
        addr,
        len,
        vf,
        spec.cast_mut().cast(),
        addr_of!(special_mapping_vmops.0),
    )
}
#[cfg(all(CONFIG_SYSCTL, RUST_MMAP_LEGACY_VA_LAYOUT))]
#[no_mangle]
pub static mut sysctl_legacy_va_layout: c_int = 0;
#[cfg(CONFIG_SYSCTL)]
#[repr(transparent)]
struct MmapTable(
    [ctl_table;
        1 + cfg!(RUST_MMAP_LEGACY_VA_LAYOUT) as usize
            + cfg!(CONFIG_HAVE_ARCH_MMAP_RND_BITS) as usize
            + cfg!(CONFIG_HAVE_ARCH_MMAP_RND_COMPAT_BITS) as usize],
);
#[cfg(CONFIG_SYSCTL)]
unsafe impl Sync for MmapTable {}
#[cfg(CONFIG_SYSCTL)]
const fn sysctl_entry(
    name: *const c_char,
    data: *mut c_void,
    mode: u16,
    min: *mut c_void,
    max: *mut c_void,
) -> ctl_table {
    ctl_table {
        procname: name,
        data,
        maxlen: size_of::<c_int>() as c_int,
        mode,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: min,
        extra2: max,
        poll: null_mut(),
    }
}
#[cfg(CONFIG_SYSCTL)]
static mmap_table: MmapTable = MmapTable([
    sysctl_entry(
        c"max_map_count".as_ptr().cast(),
        addr_of_mut!(sysctl_max_map_count).cast(),
        0o644,
        addr_of!(sysctl_vals).cast_mut().cast(),
        null_mut(),
    ),
    #[cfg(RUST_MMAP_LEGACY_VA_LAYOUT)]
    sysctl_entry(
        c"legacy_va_layout".as_ptr().cast(),
        addr_of_mut!(sysctl_legacy_va_layout).cast(),
        0o644,
        addr_of!(sysctl_vals).cast_mut().cast(),
        null_mut(),
    ),
    #[cfg(CONFIG_HAVE_ARCH_MMAP_RND_BITS)]
    sysctl_entry(
        c"mmap_rnd_bits".as_ptr().cast(),
        addr_of_mut!(mmap_rnd_bits).cast(),
        0o600,
        addr_of!(mmap_rnd_bits_min).cast_mut().cast(),
        addr_of_mut!(mmap_rnd_bits_max).cast(),
    ),
    #[cfg(CONFIG_HAVE_ARCH_MMAP_RND_COMPAT_BITS)]
    sysctl_entry(
        c"mmap_rnd_compat_bits".as_ptr().cast(),
        addr_of_mut!(mmap_rnd_compat_bits).cast(),
        0o600,
        addr_of!(mmap_rnd_compat_bits_min).cast_mut().cast(),
        addr_of!(mmap_rnd_compat_bits_max).cast_mut().cast(),
    ),
]);
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn mmap_init() {
    let ret = rust_mmap_percpu_init(addr_of_mut!(vm_committed_as));
    rust_mmap_bug_counter_init(ret != 0);
    #[cfg(CONFIG_SYSCTL)]
    __register_sysctl_init(
        c"vm".as_ptr().cast(),
        mmap_table.0.as_ptr(),
        c"mmap_table".as_ptr().cast(),
        mmap_table.0.len(),
    );
    vma_state_init();
}
#[no_mangle]
pub unsafe extern "C" fn init_user_reserve() -> c_int {
    let free_kbytes = rust_mmap_free_pages().wrapping_shl(PAGE_SHIFT - 10);
    sysctl_user_reserve_kbytes = core::cmp::min(free_kbytes / 32, RUST_MMAP_SZ_128K as _);
    0
}
#[no_mangle]
pub unsafe extern "C" fn init_admin_reserve() -> c_int {
    let free_kbytes = rust_mmap_free_pages().wrapping_shl(PAGE_SHIFT - 10);
    sysctl_admin_reserve_kbytes = core::cmp::min(free_kbytes / 32, RUST_MMAP_SZ_8K as _);
    0
}
unsafe extern "C" fn reserve_mem_notifier(
    _nb: *mut notifier_block,
    action: c_ulong,
    _data: *mut c_void,
) -> c_int {
    if action == MEM_ONLINE as c_ulong {
        let tmp = sysctl_user_reserve_kbytes;
        if tmp > 0 && tmp < RUST_MMAP_SZ_128K as _ {
            init_user_reserve();
        }
        let tmp = sysctl_admin_reserve_kbytes;
        if tmp > 0 && tmp < RUST_MMAP_SZ_8K as _ {
            init_admin_reserve();
        }
    } else if action == MEM_OFFLINE as c_ulong {
        let free_kbytes = rust_mmap_free_pages().wrapping_shl(PAGE_SHIFT - 10);
        if sysctl_user_reserve_kbytes > free_kbytes {
            init_user_reserve();
            rust_mmap_info_user_reserve(sysctl_user_reserve_kbytes);
        }
        if sysctl_admin_reserve_kbytes > free_kbytes {
            init_admin_reserve();
            rust_mmap_info_admin_reserve(sysctl_admin_reserve_kbytes);
        }
    }
    NOTIFY_OK as c_int
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
pub unsafe extern "C" fn init_reserve_notifier() -> c_int {
    #[cfg(CONFIG_MEMORY_HOTPLUG)]
    if rust_mmap_register_memory_notifier(addr_of_mut!(reserve_mem_notifier_mem_nb)) != 0 {
        rust_mmap_err_notifier();
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn mmap_read_lock_maybe_expand(
    mm: *mut mm_struct,
    new_vma: *mut vm_area_struct,
    addr: c_ulong,
    write: bool,
) -> bool {
    if !write || addr >= start(new_vma) {
        rust_mmap_mmap_read_lock(mm);
        return true;
    }
    if !test(new_vma, VMA_GROWSDOWN_BIT as _) {
        return false;
    }
    rust_mmap_mmap_write_lock(mm);
    if expand_downwards(new_vma, addr) != 0 {
        rust_mmap_mmap_write_unlock(mm);
        return false;
    }
    rust_mmap_mmap_write_downgrade(mm);
    true
}
#[no_mangle]
pub unsafe extern "C" fn dup_mmap(mm: *mut mm_struct, oldmm: *mut mm_struct) -> c_int {
    let mut charge: c_ulong;
    let mut uf: list_head = zeroed();
    init_list(&mut uf);
    let mut vmi = iterator(mm, 0);
    if rust_mmap_mmap_write_lock_killable(oldmm) != 0 {
        return -(EINTR as c_int);
    }
    rust_mmap_flush_cache_dup_mm(oldmm);
    rust_mmap_uprobe_dup_mmap(oldmm, mm);
    rust_mmap_mmap_write_lock_nested(mm, SINGLE_DEPTH_NESTING as _);
    dup_mm_exe_file(mm, oldmm);
    *rust_mmap_mm_total_vm(mm) = *rust_mmap_mm_total_vm(oldmm);
    *rust_mmap_mm_data_vm(mm) = *rust_mmap_mm_data_vm(oldmm);
    *rust_mmap_mm_exec_vm(mm) = *rust_mmap_mm_exec_vm(oldmm);
    *rust_mmap_mm_stack_vm(mm) = *rust_mmap_mm_stack_vm(oldmm);
    let mut retval = __mt_dup(
        rust_mmap_mm_mm_mt(oldmm),
        rust_mmap_mm_mm_mt(mm),
        GFP_KERNEL,
    );
    if retval == 0 {
        rust_mmap_mt_clear_in_rcu(vmi.mas.tree);
        let mut mpnt: *mut vm_area_struct;
        loop {
            mpnt = rust_mmap_vma_next(&mut vmi);
            if mpnt.is_null() {
                retval = rust_mmap_arch_dup_mmap(oldmm, mm);
                break;
            }
            retval = rust_mmap_vma_start_write_killable(mpnt);
            if retval < 0 {
                break;
            }
            if test(mpnt, VMA_DONTCOPY_BIT as _) {
                retval = rust_mmap_vma_iter_clear_gfp(&mut vmi, start(mpnt), end(mpnt), GFP_KERNEL);
                if retval != 0 {
                    break;
                }
                vm_stat_account(
                    mm,
                    flags(mpnt),
                    (rust_mmap_vma_pages(mpnt) as c_long).wrapping_neg(),
                );
                continue;
            }
            charge = 0;
            if test(mpnt, VMA_ACCOUNT_BIT as _) {
                let len = rust_mmap_vma_pages(mpnt);
                if rust_mmap_security_vm_enough_memory_mm(oldmm, len as _) != 0 {
                    retval = -(ENOMEM as c_int);
                    rust_mmap_vm_unacct_memory(charge as _);
                    break;
                }
                charge = len;
            }
            let tmp = vm_area_dup(mpnt);
            if tmp.is_null() {
                retval = -(ENOMEM as c_int);
                rust_mmap_vm_unacct_memory(charge as _);
                break;
            }
            retval = rust_mmap_vma_dup_policy(mpnt, tmp);
            if retval != 0 {
                vm_area_free(tmp);
                retval = -(ENOMEM as c_int);
                rust_mmap_vm_unacct_memory(charge as _);
                break;
            }
            (*tmp).vm_mm = mm;
            retval = rust_mmap_dup_userfaultfd(tmp, &mut uf);
            let mut failed = retval != 0;
            if !failed {
                if test(tmp, VMA_WIPEONFORK_BIT as _) {
                    (*tmp).anon_vma = null_mut();
                } else {
                    failed = anon_vma_fork(tmp, mpnt) != 0;
                }
            }
            if failed {
                rust_mmap_mpol_put(rust_mmap_vma_policy(tmp));
                vm_area_free(tmp);
                retval = -(ENOMEM as c_int);
                rust_mmap_vm_unacct_memory(charge as _);
                break;
            }
            rust_mmap_vma_start_write(tmp);
            rust_mmap_vma_clear_locked(tmp);
            if rust_mmap_is_vm_hugetlb_page(tmp) {
                rust_mmap_hugetlb_dup_vma_private(tmp);
            }
            rust_mmap_vma_iter_bulk_store(&mut vmi, tmp);
            *rust_mmap_mm_map_count(mm) += 1;
            if !(*tmp).vm_ops.is_null() {
                if let Some(open) = (*(*tmp).vm_ops).open {
                    open(tmp);
                }
            }
            let file = (*tmp).vm_file;
            if !file.is_null() {
                let mapping = (*file).f_mapping;
                rust_mmap_get_file(file);
                rust_mmap_i_mmap_lock_write(mapping);
                if rust_mmap_vma_shared_maywrite(tmp) {
                    rust_mmap_mapping_allow_writable(mapping);
                }
                rust_mmap_flush_dcache_mmap_lock(mapping);
                mapping_rmap_tree_insert_after(tmp, mpnt, mapping);
                rust_mmap_flush_dcache_mmap_unlock(mapping);
                rust_mmap_i_mmap_unlock_write(mapping);
            }
            if !test(tmp, VMA_WIPEONFORK_BIT as _) {
                retval = copy_page_range(tmp, mpnt);
            }
            if retval != 0 {
                mpnt = rust_mmap_vma_next(&mut vmi);
                break;
            }
        }
        rust_mmap_vma_iter_free(&mut vmi);
        if retval == 0 {
            rust_mmap_mt_set_in_rcu(vmi.mas.tree);
            rust_mmap_ksm_fork(mm, oldmm);
            rust_mmap_khugepaged_fork(mm, oldmm);
        } else {
            let limit = if *rust_mmap_mm_map_count(mm) == 0 {
                0
            } else if !mpnt.is_null() {
                start(mpnt)
            } else {
                ULONG_MAX
            };
            rust_mmap_mm_flags_set(MMF_OOM_SKIP as _, mm);
            if limit != 0 {
                rust_mmap_vma_iter_set(&mut vmi, 0);
                let tmp = rust_mmap_vma_next(&mut vmi);
                let mut unmap = unmap_all(&mut vmi, tmp);
                unmap.vma_end = limit;
                unmap.tree_end = limit;
                unmap.mm_wr_locked = true;
                rust_mmap_flush_cache_mm(mm);
                unmap_region(&mut unmap);
                charge = tear_down_vmas(mm, &mut vmi, tmp, limit);
                rust_mmap_vm_unacct_memory(charge as _);
            }
            __mt_destroy(rust_mmap_mm_mm_mt(mm));
            rust_mmap_mm_flags_set(MMF_UNSTABLE as _, mm);
        }
    }
    rust_mmap_mmap_write_unlock(mm);
    rust_mmap_flush_tlb_mm(oldmm);
    rust_mmap_mmap_write_unlock(oldmm);
    if retval == 0 {
        rust_mmap_dup_userfaultfd_complete(&mut uf);
    } else {
        rust_mmap_dup_userfaultfd_fail(&mut uf);
    }
    retval
}
// Storage that the original hotplug_memory_notifier macro supplied. Callback
// and native priority are bound once, before the notifier can be published.
#[cfg(CONFIG_MEMORY_HOTPLUG)]
static mut reserve_mem_notifier_mem_nb: notifier_block = notifier_block {
    notifier_call: Some(reserve_mem_notifier),
    next: null_mut(),
    priority: DEFAULT_CALLBACK_PRI as _,
};
