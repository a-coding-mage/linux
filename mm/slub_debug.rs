// SPDX-License-Identifier: GPL-2.0
// Native C varargs are used only as a printf ABI leaf. Decisions, repairs,
// traversal, formatting choice and KUnit accounting remain in Rust.
#[cfg(CONFIG_SLUB_DEBUG)]
macro_rules! slab_bug {
    ($s:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s: *mut kmem_cache = $s;
        rust_slub_printk(c"\x013=============================================================================\n".as_ptr().cast::<CChar>());
        rust_slub_printk(concat!("\x013BUG %s (%s): ", $fmt, "\n\0").as_ptr().cast(),
            if s.is_null() { c"<unknown>".as_ptr().cast::<CChar>() } else { (*s).name }, print_tainted() $(, $arg)*);
        rust_slub_printk(c"\x013-----------------------------------------------------------------------------\n\n".as_ptr().cast::<CChar>());
    }};
}
#[cfg(CONFIG_SLUB_DEBUG)]
macro_rules! slab_fix {
    ($s:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        if !slab_add_kunit_errors() {
            rust_slub_printk(concat!("\x013FIX %s: ", $fmt, "\n\0").as_ptr().cast(), (*$s).name $(, $arg)*);
        }
    }};
}
#[cfg(CONFIG_SLUB_DEBUG)]
macro_rules! slab_err {
    ($s:expr, $slab:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        if !slab_add_kunit_errors() { slab_bug!($s, $fmt $(, $arg)*); __slab_err($slab); }
    }};
}
#[cfg(CONFIG_SLUB_DEBUG)]
static mut object_map: [ULong;
    (RSL_MAX_OBJS_PER_PAGE as usize + ULong::BITS as usize - 1) / ULong::BITS as usize] =
    [0; (RSL_MAX_OBJS_PER_PAGE as usize + ULong::BITS as usize - 1) / ULong::BITS as usize];
#[cfg(CONFIG_SLUB_DEBUG_ON)]
static mut slub_debug: slab_flags_t = RSL_DEBUG_DEFAULT_FLAGS;
#[cfg(all(CONFIG_SLUB_DEBUG, not(CONFIG_SLUB_DEBUG_ON)))]
static mut slub_debug: slab_flags_t = 0;
#[cfg(not(CONFIG_SLUB_DEBUG))]
const slub_debug: slab_flags_t = 0;
#[cfg(CONFIG_SLUB_DEBUG)]
#[link_section = ".data..ro_after_init"]
static mut slub_debug_string: *const CChar = null();
#[cfg(CONFIG_SLUB_DEBUG)]
static mut disable_higher_order_debug: i32 = 0;
#[cfg(not(CONFIG_SLUB_DEBUG))]
const disable_higher_order_debug: i32 = 0;
#[cfg(CONFIG_SLUB_DEBUG)]
#[inline]
unsafe fn validate_slab_ptr(slab: *mut slab) -> bool {
    rust_slub_page_is_slab(rust_slub_slab_page(slab))
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn __fill_map(obj_map: *mut ULong, s: *mut kmem_cache, slab: *mut slab) {
    let addr = rust_slub_slab_address(slab);
    rust_slub_bitmap_zero(obj_map, rust_slub_slab_objects(slab));
    let mut p = rust_slub_slab_freelist(slab);
    while !p.is_null() {
        rust_slub_set_bit(rust_slub_obj_to_index(s, addr, p) as ULong, obj_map);
        p = get_freepointer(s, p);
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn slab_add_kunit_errors() -> bool {
    #[cfg(any(CONFIG_KUNIT, CONFIG_KUNIT_MODULE))]
    {
        let test = rust_slub_current_kunit_test();
        if test.is_null() {
            return false;
        }
        let resource =
            rust_slub_kunit_find_named_resource(test, c"slab_errors".as_ptr().cast::<CChar>());
        if resource.is_null() {
            return false;
        }
        *(*resource).data.cast::<i32>() += 1;
        rust_slub_kunit_put_resource(resource);
        return true;
    }
    #[cfg(not(any(CONFIG_KUNIT, CONFIG_KUNIT_MODULE)))]
    {
        false
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[cfg_attr(any(CONFIG_KUNIT, CONFIG_KUNIT_MODULE), no_mangle)]
pub unsafe extern "C" fn slab_in_kunit_test() -> bool {
    #[cfg(any(CONFIG_KUNIT, CONFIG_KUNIT_MODULE))]
    {
        let test = rust_slub_current_kunit_test();
        if test.is_null() {
            return false;
        }
        let resource =
            rust_slub_kunit_find_named_resource(test, c"slab_errors".as_ptr().cast::<CChar>());
        if resource.is_null() {
            return false;
        }
        rust_slub_kunit_put_resource(resource);
        return true;
    }
    #[cfg(not(any(CONFIG_KUNIT, CONFIG_KUNIT_MODULE)))]
    {
        false
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[inline]
unsafe fn size_from_object(s: *mut kmem_cache) -> u32 {
    (*s).size
        - if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
            (*s).red_left_pad
        } else {
            0
        }
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[inline]
unsafe fn restore_red_left(s: *mut kmem_cache, p: *mut Void) -> *mut Void {
    if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
        p.byte_sub((*s).red_left_pad as usize)
    } else {
        p
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[inline]
unsafe fn check_valid_pointer(s: *mut kmem_cache, slab: *mut slab, object: *mut Void) -> i32 {
    if object.is_null() {
        return 1;
    }
    let base = rust_slub_slab_address(slab) as usize;
    let object = restore_red_left(s, rust_slub_kasan_reset_tag(object)) as usize;
    if object < base
        || object >= base + (rust_slub_slab_objects(slab) * (*s).size) as usize
        || (object - base) % (*s).size as usize != 0
    {
        0
    } else {
        1
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn print_section(level: *const CChar, text: *const CChar, addr: *mut u8, length: u32) {
    rust_slub_metadata_access_enable();
    rust_slub_print_hex_dump(
        level,
        text,
        DUMP_PREFIX_ADDRESS as i32,
        16,
        1,
        rust_slub_kasan_reset_tag(addr.cast()),
        length as usize,
        true,
    );
    rust_slub_metadata_access_disable();
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn get_track(s: *mut kmem_cache, object: *mut Void, alloc: track_item) -> *mut track {
    let p = object.byte_add(get_info_end(s) as usize).cast::<track>();
    rust_slub_kasan_reset_tag(p.add(alloc as usize).cast()).cast()
}
#[cfg_attr(all(CONFIG_SLUB_DEBUG, CONFIG_STACKDEPOT), inline(never))]
unsafe fn set_track_prepare(gfp_flags: gfp_t) -> depot_stack_handle_t {
    #[cfg(all(CONFIG_SLUB_DEBUG, CONFIG_STACKDEPOT))]
    {
        let mut entries = [0; RSL_TRACK_ADDRS_COUNT as usize];
        let nr = stack_trace_save(entries.as_mut_ptr(), entries.len() as u32, 3);
        return stack_depot_save(entries.as_mut_ptr(), nr, gfp_flags);
    }
    #[cfg(not(all(CONFIG_SLUB_DEBUG, CONFIG_STACKDEPOT)))]
    {
        0
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn set_track_update(
    s: *mut kmem_cache,
    object: *mut Void,
    alloc: track_item,
    addr: ULong,
    handle: depot_stack_handle_t,
) {
    let p = get_track(s, object, alloc);
    #[cfg(CONFIG_STACKDEPOT)]
    {
        (*p).handle = handle;
    }
    (*p).addr = addr;
    (*p).cpu = rust_slub_raw_smp_processor_id();
    (*p).pid = rust_slub_current_pid();
    (*p).when = rust_slub_jiffies();
}
#[inline(always)]
unsafe fn set_track(
    s: *mut kmem_cache,
    object: *mut Void,
    alloc: track_item,
    addr: ULong,
    gfp_flags: gfp_t,
) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let handle = set_track_prepare(gfp_flags);
        set_track_update(s, object, alloc, addr, handle);
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn init_tracking(s: *mut kmem_cache, object: *mut Void) {
    if (*s).flags & RSL_SLAB_STORE_USER == 0 {
        return;
    }
    core::ptr::write_bytes(
        get_track(s, object, TRACK_ALLOC).cast::<u8>(),
        0,
        2 * size_of::<track>(),
    );
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn print_track(s: *const CChar, t: *mut track, pr_time: ULong) {
    if (*t).addr == 0 {
        return;
    }
    rust_slub_printk(
        c"\x013%s in %pS age=%lu cpu=%u pid=%d\n"
            .as_ptr()
            .cast::<CChar>(),
        s,
        (*t).addr as *mut Void,
        pr_time.wrapping_sub((*t).when),
        (*t).cpu as u32,
        (*t).pid,
    );
    #[cfg(CONFIG_STACKDEPOT)]
    {
        let handle = rust_slub_read_uint(addr_of!((*t).handle));
        if handle != 0 {
            stack_depot_print(handle);
        } else {
            rust_slub_printk(
                c"\x013object allocation/free stack trace missing\n"
                    .as_ptr()
                    .cast::<CChar>(),
            );
        }
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn print_tracking(s: *mut kmem_cache, object: *mut Void) {
    let now = rust_slub_jiffies();
    if (*s).flags & RSL_SLAB_STORE_USER == 0 {
        return;
    }
    print_track(
        c"Allocated".as_ptr().cast::<CChar>(),
        get_track(s, object, TRACK_ALLOC),
        now,
    );
    print_track(
        c"Freed".as_ptr().cast::<CChar>(),
        get_track(s, object, TRACK_FREE),
        now,
    );
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn print_slab_info(slab: *const slab) {
    rust_slub_printk(
        c"\x013Slab 0x%p objects=%u used=%u fp=0x%p flags=%pGp\n"
            .as_ptr()
            .cast::<CChar>(),
        slab,
        rust_slub_slab_objects(slab),
        rust_slub_slab_inuse(slab),
        rust_slub_slab_freelist(slab),
        rust_slub_slab_flags_ptr(slab),
    );
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn skip_orig_size_check(s: *mut kmem_cache, object: *const Void) {
    set_orig_size(s, object.cast_mut(), (*s).object_size as ULong);
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn slab_bug(s: *mut kmem_cache, reason: *const CChar) {
    slab_bug!(s, "%s", reason);
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn print_trailer(s: *mut kmem_cache, slab: *mut slab, p: *mut u8) {
    let addr = rust_slub_slab_address(slab).cast::<u8>();
    print_tracking(s, p.cast());
    print_slab_info(slab);
    rust_slub_printk(
        c"\x013Object 0x%p @offset=%tu fp=0x%p\n\n"
            .as_ptr()
            .cast::<CChar>(),
        p,
        (p as isize).wrapping_sub(addr as isize),
        get_freepointer(s, p.cast()),
    );
    if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
        print_section(
            c"\x013".as_ptr().cast::<CChar>(),
            c"Redzone  ".as_ptr().cast::<CChar>(),
            p.sub((*s).red_left_pad as usize),
            (*s).red_left_pad,
        );
    } else if p > addr.add(16) {
        print_section(
            c"\x013".as_ptr().cast::<CChar>(),
            c"Bytes b4 ".as_ptr().cast::<CChar>(),
            p.sub(16),
            16,
        );
    }
    print_section(
        c"\x013".as_ptr().cast::<CChar>(),
        c"Object   ".as_ptr().cast::<CChar>(),
        p,
        min((*s).object_size, RSL_PAGE_SIZE),
    );
    if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
        print_section(
            c"\x013".as_ptr().cast::<CChar>(),
            c"Redzone  ".as_ptr().cast::<CChar>(),
            p.add((*s).object_size as usize),
            (*s).inuse - (*s).object_size,
        );
    }
    let mut off = get_info_end(s);
    if (*s).flags & RSL_SLAB_STORE_USER != 0 {
        off += (2 * size_of::<track>()) as u32;
    }
    if rust_slub_slub_debug_orig_size(s) {
        off += size_of::<ULong>() as u32;
    }
    off += rust_slub_kasan_metadata_size(s, false) as u32;
    if rust_slub_obj_exts_in_object(slab) {
        off += rust_slub_slab_obj_ext_size(slab);
    }
    if off != size_from_object(s) {
        print_section(
            c"\x013".as_ptr().cast::<CChar>(),
            c"Padding  ".as_ptr().cast::<CChar>(),
            p.add(off as usize),
            size_from_object(s) - off,
        );
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn object_err(s: *mut kmem_cache, slab: *mut slab, object: *mut Void, reason: *const CChar) {
    if slab_add_kunit_errors() {
        return;
    }
    slab_bug(s, reason);
    if object.is_null() || check_valid_pointer(s, slab, object) == 0 {
        print_slab_info(slab);
        rust_slub_printk(
            c"\x013Invalid pointer 0x%p\n".as_ptr().cast::<CChar>(),
            object,
        );
    } else {
        print_trailer(s, slab, object.cast());
    }
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    rust_slub_warn_object_error();
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn __slab_err(slab: *mut slab) {
    if slab_in_kunit_test() {
        return;
    }
    print_slab_info(slab);
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    rust_slub_warn_slab_error();
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn slab_err_invalid_page(s: *mut kmem_cache, slab: *mut slab) {
    slab_err!(s, slab, "Not a valid slab page");
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn init_object(s: *mut kmem_cache, object: *mut Void, val: u8) {
    let p = rust_slub_kasan_reset_tag(object).cast::<u8>();
    let mut poison_size = (*s).object_size;
    if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
        rust_slub_memset_no_sanitize_memory(
            p.sub((*s).red_left_pad as usize).cast(),
            val as i32,
            (*s).red_left_pad as usize,
        );
        if rust_slub_slub_debug_orig_size(s) && val == SLUB_RED_ACTIVE as u8 {
            poison_size = get_orig_size(s, object) as u32;
        }
    }
    if (*s).flags & RSL___OBJECT_POISON != 0 {
        rust_slub_memset_no_sanitize_memory(
            p.cast(),
            POISON_FREE as i32,
            (poison_size - 1) as usize,
        );
        rust_slub_memset_no_sanitize_memory(
            p.add(poison_size as usize - 1).cast(),
            POISON_END as i32,
            1,
        );
    }
    if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
        rust_slub_memset_no_sanitize_memory(
            p.add(poison_size as usize).cast(),
            val as i32,
            ((*s).inuse - poison_size) as usize,
        );
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn restore_bytes(
    s: *mut kmem_cache,
    message: *const CChar,
    data: u8,
    from: *mut u8,
    to: *mut u8,
) {
    slab_fix!(
        s,
        "Restoring %s 0x%p-0x%p=0x%x",
        message,
        from,
        to.sub(1),
        data as u32
    );
    core::ptr::write_bytes(from, data, (to as usize).wrapping_sub(from as usize));
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[cfg_attr(CONFIG_KMSAN, inline(never))]
#[cfg_attr(CONFIG_KMSAN, no_sanitize(memory))]
unsafe fn check_bytes_and_report(
    s: *mut kmem_cache,
    slab: *mut slab,
    object: *mut Void,
    what: *const CChar,
    start: *mut u8,
    value: u32,
    bytes: u32,
    slab_obj_print: bool,
) -> i32 {
    let addr = rust_slub_slab_address(slab).cast::<u8>();
    rust_slub_metadata_access_enable();
    let fault = memchr_inv(
        rust_slub_kasan_reset_tag(start.cast()),
        value as i32,
        bytes as usize,
    )
    .cast::<u8>();
    rust_slub_metadata_access_disable();
    if fault.is_null() {
        return 1;
    }
    let mut end = start.add(bytes as usize);
    while end > fault && *end.sub(1) == value as u8 {
        end = end.sub(1);
    }
    if !slab_add_kunit_errors() {
        rust_slub_printk(
            c"\x013[%s overwritten] 0x%p-0x%p @offset=%tu. First byte 0x%x instead of 0x%x\n"
                .as_ptr(),
            what,
            fault,
            end.sub(1),
            (fault as isize).wrapping_sub(addr as isize),
            *fault as u32,
            value,
        );
        if slab_obj_print {
            object_err(s, slab, object, c"Object corrupt".as_ptr().cast::<CChar>());
        }
    }
    restore_bytes(s, what, value as u8, fault, end);
    0
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn check_pad_bytes(s: *mut kmem_cache, slab: *mut slab, p: *mut u8) -> i32 {
    let mut off = get_info_end(s) as ULong;
    if (*s).flags & RSL_SLAB_STORE_USER != 0 {
        off += (2 * size_of::<track>()) as ULong;
        if (*s).flags & RSL_SLAB_KMALLOC != 0 {
            off += size_of::<ULong>() as ULong;
        }
    }
    off += rust_slub_kasan_metadata_size(s, false) as ULong;
    if rust_slub_obj_exts_in_object(slab) {
        off += rust_slub_slab_obj_ext_size(slab) as ULong;
    }
    if size_from_object(s) as ULong == off {
        return 1;
    }
    check_bytes_and_report(
        s,
        slab,
        p.cast(),
        c"Object padding".as_ptr().cast::<CChar>(),
        p.add(off as usize),
        POISON_INUSE,
        (size_from_object(s) as ULong - off) as u32,
        true,
    )
}
#[cfg_attr(all(CONFIG_SLUB_DEBUG, CONFIG_KMSAN), inline(never))]
#[cfg_attr(all(CONFIG_SLUB_DEBUG, CONFIG_KMSAN), no_sanitize(memory))]
unsafe fn slab_pad_check(s: *mut kmem_cache, slab: *mut slab) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if (*s).flags & RSL_SLAB_POISON == 0 {
            return;
        }
        let start = rust_slub_slab_address(slab).cast::<u8>();
        let length = rust_slub_slab_size(slab) as i32;
        let mut end = start.add(length as usize);
        let remainder = if obj_exts_in_slab(s, slab) && !rust_slub_obj_exts_in_object(slab) {
            length
                .wrapping_sub(obj_exts_offset_in_slab(s, slab) as i32)
                .wrapping_sub(obj_exts_size_in_slab(slab) as i32)
        } else {
            (length as u32 % (*s).size) as i32
        };
        if remainder == 0 {
            return;
        }
        let pad = end.sub(remainder as usize);
        rust_slub_metadata_access_enable();
        let fault = memchr_inv(
            rust_slub_kasan_reset_tag(pad.cast()),
            POISON_INUSE as i32,
            remainder as usize,
        )
        .cast::<u8>();
        rust_slub_metadata_access_disable();
        if fault.is_null() {
            return;
        }
        while end > fault && *end.sub(1) == POISON_INUSE as u8 {
            end = end.sub(1);
        }
        slab_bug!(
            s,
            "Padding overwritten. 0x%p-0x%p @offset=%tu",
            fault,
            end.sub(1),
            (fault as isize).wrapping_sub(start as isize)
        );
        print_section(
            c"\x013".as_ptr().cast::<CChar>(),
            c"Padding ".as_ptr().cast::<CChar>(),
            pad,
            remainder as u32,
        );
        __slab_err(slab);
        restore_bytes(
            s,
            c"slab padding".as_ptr().cast::<CChar>(),
            POISON_INUSE as u8,
            fault,
            end,
        );
    }
}
unsafe fn check_object(s: *mut kmem_cache, slab: *mut slab, object: *mut Void, val: u8) -> i32 {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let p = object.cast::<u8>();
        let endobject = p.add((*s).object_size as usize);
        let mut ret = 1;
        if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
            if check_bytes_and_report(
                s,
                slab,
                object,
                c"Left Redzone".as_ptr().cast::<CChar>(),
                p.sub((*s).red_left_pad as usize),
                val as u32,
                (*s).red_left_pad,
                ret != 0,
            ) == 0
            {
                ret = 0;
            }
            if check_bytes_and_report(
                s,
                slab,
                object,
                c"Right Redzone".as_ptr().cast::<CChar>(),
                endobject,
                val as u32,
                (*s).inuse - (*s).object_size,
                ret != 0,
            ) == 0
            {
                ret = 0;
            }
            if rust_slub_slub_debug_orig_size(s) && val == SLUB_RED_ACTIVE as u8 {
                let orig_size = get_orig_size(s, object) as u32;
                if (*s).object_size > orig_size
                    && check_bytes_and_report(
                        s,
                        slab,
                        object,
                        c"kmalloc Redzone".as_ptr().cast::<CChar>(),
                        p.add(orig_size as usize),
                        val as u32,
                        (*s).object_size - orig_size,
                        ret != 0,
                    ) == 0
                {
                    ret = 0;
                }
            }
        } else if (*s).flags & RSL_SLAB_POISON != 0 && (*s).object_size < (*s).inuse {
            if check_bytes_and_report(
                s,
                slab,
                object,
                c"Alignment padding".as_ptr().cast::<CChar>(),
                endobject,
                POISON_INUSE,
                (*s).inuse - (*s).object_size,
                ret != 0,
            ) == 0
            {
                ret = 0;
            }
        }
        if (*s).flags & RSL_SLAB_POISON != 0 {
            if val != SLUB_RED_ACTIVE as u8 && (*s).flags & RSL___OBJECT_POISON != 0 {
                let kasan_meta_size = rust_slub_kasan_metadata_size(s, true) as u32;
                if kasan_meta_size < (*s).object_size - 1
                    && check_bytes_and_report(
                        s,
                        slab,
                        object,
                        c"Poison".as_ptr().cast::<CChar>(),
                        p.add(kasan_meta_size as usize),
                        POISON_FREE,
                        (*s).object_size - kasan_meta_size - 1,
                        ret != 0,
                    ) == 0
                {
                    ret = 0;
                }
                if kasan_meta_size < (*s).object_size
                    && check_bytes_and_report(
                        s,
                        slab,
                        object,
                        c"End Poison".as_ptr().cast::<CChar>(),
                        p.add((*s).object_size as usize - 1),
                        POISON_END,
                        1,
                        ret != 0,
                    ) == 0
                {
                    ret = 0;
                }
            }
            if check_pad_bytes(s, slab, p) == 0 {
                ret = 0;
            }
        }
        if (freeptr_outside_object(s) || val != SLUB_RED_ACTIVE as u8)
            && check_valid_pointer(s, slab, get_freepointer(s, object)) == 0
        {
            object_err(
                s,
                slab,
                object,
                c"Freepointer corrupt".as_ptr().cast::<CChar>(),
            );
            set_freepointer(s, object, null_mut());
            ret = 0;
        }
        return ret;
    }
    #[cfg(not(CONFIG_SLUB_DEBUG))]
    {
        1
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn check_slab(s: *mut kmem_cache, slab: *mut slab) -> i32 {
    let maxobj = order_objects(rust_slub_slab_order(slab) as u32, (*s).size);
    let objects = rust_slub_slab_objects(slab);
    let inuse = rust_slub_slab_inuse(slab);
    if objects > maxobj {
        slab_err!(s, slab, "objects %u > max %u", objects, maxobj);
        return 0;
    }
    if inuse > objects {
        slab_err!(s, slab, "inuse %u > max %u", inuse, objects);
        return 0;
    }
    if rust_slub_slab_frozen(slab) {
        slab_err!(
            s,
            slab,
            "Slab disabled since SLUB metadata consistency check failed"
        );
        return 0;
    }
    slab_pad_check(s, slab);
    1
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn on_freelist(s: *mut kmem_cache, slab: *mut slab, search: *mut Void) -> bool {
    let mut nr = 0;
    let mut fp = rust_slub_slab_freelist(slab);
    let mut object: *mut Void = null_mut();
    while !fp.is_null() && nr <= rust_slub_slab_objects(slab) {
        if fp == search {
            return true;
        }
        if check_valid_pointer(s, slab, fp) == 0 {
            if !object.is_null() {
                object_err(
                    s,
                    slab,
                    object,
                    c"Freechain corrupt".as_ptr().cast::<CChar>(),
                );
                set_freepointer(s, object, null_mut());
                break;
            }
            slab_err!(s, slab, "Freepointer corrupt");
            rust_slub_slab_set_freelist(slab, null_mut());
            rust_slub_slab_set_inuse(slab, rust_slub_slab_objects(slab));
            slab_fix!(s, "Freelist cleared");
            return false;
        }
        object = fp;
        fp = get_freepointer(s, object);
        nr += 1;
    }
    if nr > rust_slub_slab_objects(slab) {
        slab_err!(s, slab, "Freelist cycle detected");
        rust_slub_slab_set_freelist(slab, null_mut());
        rust_slub_slab_set_inuse(slab, rust_slub_slab_objects(slab));
        slab_fix!(s, "Freelist cleared");
        return false;
    }
    let max_objects = min(
        order_objects(rust_slub_slab_order(slab) as u32, (*s).size),
        RSL_MAX_OBJS_PER_PAGE,
    );
    if rust_slub_slab_objects(slab) != max_objects {
        slab_err!(
            s,
            slab,
            "Wrong number of objects. Found %d but should be %d",
            rust_slub_slab_objects(slab),
            max_objects
        );
        rust_slub_slab_set_objects(slab, max_objects);
        slab_fix!(s, "Number of objects adjusted");
    }
    if rust_slub_slab_inuse(slab) != rust_slub_slab_objects(slab) - nr {
        slab_err!(
            s,
            slab,
            "Wrong object count. Counter is %d but counted were %d",
            rust_slub_slab_inuse(slab),
            rust_slub_slab_objects(slab) - nr
        );
        rust_slub_slab_set_inuse(slab, rust_slub_slab_objects(slab) - nr);
        slab_fix!(s, "Object count adjusted");
    }
    search.is_null()
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn trace(s: *mut kmem_cache, slab: *mut slab, object: *mut Void, alloc: i32) {
    if (*s).flags & RSL_SLAB_TRACE == 0 {
        return;
    }
    rust_slub_printk(
        c"\x016TRACE %s %s 0x%p inuse=%d fp=0x%p\n"
            .as_ptr()
            .cast::<CChar>(),
        (*s).name,
        if alloc != 0 {
            c"alloc".as_ptr().cast::<CChar>()
        } else {
            c"free".as_ptr().cast::<CChar>()
        },
        object,
        rust_slub_slab_inuse(slab),
        rust_slub_slab_freelist(slab),
    );
    if alloc == 0 {
        print_section(
            c"\x016".as_ptr().cast::<CChar>(),
            c"Object ".as_ptr().cast::<CChar>(),
            object.cast(),
            (*s).object_size,
        );
    }
    dump_stack();
}
unsafe fn add_full(s: *mut kmem_cache, n: *mut kmem_cache_node, slab: *mut slab) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if (*s).flags & RSL_SLAB_STORE_USER == 0 {
            return;
        }
        rust_slub_assert_spin_held(addr_of_mut!((*n).list_lock));
        slab_attach_kprobe_locked();
        rust_slub_list_add(rust_slub_slab_list(slab), addr_of_mut!((*n).full));
    }
}
unsafe fn remove_full(s: *mut kmem_cache, n: *mut kmem_cache_node, slab: *mut slab) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if (*s).flags & RSL_SLAB_STORE_USER == 0 {
            return;
        }
        rust_slub_assert_spin_held(addr_of_mut!((*n).list_lock));
        slab_attach_kprobe_locked();
        rust_slub_list_del(rust_slub_slab_list(slab));
    }
}
#[inline]
unsafe fn node_nr_slabs(n: *mut kmem_cache_node) -> ULong {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        rust_slub_atomic_long_read(addr_of!((*n).nr_slabs)) as ULong
    }
    #[cfg(not(CONFIG_SLUB_DEBUG))]
    {
        0
    }
}
#[inline]
unsafe fn inc_slabs_node(s: *mut kmem_cache, node: i32, objects: i32) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let n = get_node(s, node);
        rust_slub_atomic_long_inc(addr_of_mut!((*n).nr_slabs));
        rust_slub_atomic_long_add(objects as Long, addr_of_mut!((*n).total_objects));
    }
}
#[inline]
unsafe fn dec_slabs_node(s: *mut kmem_cache, node: i32, objects: i32) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let n = get_node(s, node);
        rust_slub_atomic_long_dec(addr_of_mut!((*n).nr_slabs));
        rust_slub_atomic_long_sub(objects as Long, addr_of_mut!((*n).total_objects));
    }
}
unsafe fn setup_object_debug(s: *mut kmem_cache, object: *mut Void) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if !rust_slub_kmem_cache_debug_flags(
            s,
            RSL_SLAB_STORE_USER | RSL_SLAB_RED_ZONE | RSL___OBJECT_POISON,
        ) {
            return;
        }
        init_object(s, object, SLUB_RED_INACTIVE as u8);
        init_tracking(s, object);
    }
}
unsafe fn setup_slab_debug(s: *mut kmem_cache, slab: *mut slab, addr: *mut Void) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if !rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_POISON) {
            return;
        }
        rust_slub_metadata_access_enable();
        core::ptr::write_bytes(
            rust_slub_kasan_reset_tag(addr).cast::<u8>(),
            POISON_INUSE as u8,
            rust_slub_slab_size(slab) as usize,
        );
        rust_slub_metadata_access_disable();
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[inline]
unsafe fn alloc_consistency_checks(s: *mut kmem_cache, slab: *mut slab, object: *mut Void) -> i32 {
    if check_slab(s, slab) == 0 {
        return 0;
    }
    if check_valid_pointer(s, slab, object) == 0 {
        object_err(
            s,
            slab,
            object,
            c"Freelist Pointer check fails".as_ptr().cast::<CChar>(),
        );
        return 0;
    }
    if check_object(s, slab, object, SLUB_RED_INACTIVE as u8) == 0 {
        return 0;
    }
    1
}
#[cfg_attr(CONFIG_SLUB_DEBUG, inline(never))]
unsafe fn alloc_debug_processing(
    s: *mut kmem_cache,
    slab: *mut slab,
    object: *mut Void,
    orig_size: i32,
) -> bool {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if (*s).flags & RSL_SLAB_CONSISTENCY_CHECKS != 0
            && alloc_consistency_checks(s, slab, object) == 0
        {
            slab_fix!(s, "Marking all objects used");
            rust_slub_slab_set_inuse(slab, rust_slub_slab_objects(slab));
            rust_slub_slab_set_freelist(slab, null_mut());
            rust_slub_slab_set_frozen(slab, true);
            return false;
        }
        trace(s, slab, object, 1);
        set_orig_size(s, object, orig_size as ULong);
        init_object(s, object, SLUB_RED_ACTIVE as u8);
    }
    true
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[inline]
unsafe fn free_consistency_checks(
    s: *mut kmem_cache,
    slab: *mut slab,
    object: *mut Void,
    addr: ULong,
) -> i32 {
    if check_valid_pointer(s, slab, object) == 0 {
        slab_err!(s, slab, "Invalid object pointer 0x%p", object);
        return 0;
    }
    if on_freelist(s, slab, object) {
        object_err(
            s,
            slab,
            object,
            c"Object already free".as_ptr().cast::<CChar>(),
        );
        return 0;
    }
    if check_object(s, slab, object, SLUB_RED_ACTIVE as u8) == 0 {
        return 0;
    }
    if s != (*slab).slab_cache {
        if (*slab).slab_cache.is_null() {
            slab_err!(null_mut(), slab, "No slab cache for object 0x%p", object);
        } else {
            object_err(
                s,
                slab,
                object,
                c"page slab pointer corrupt.".as_ptr().cast::<CChar>(),
            );
        }
        return 0;
    }
    1
}
