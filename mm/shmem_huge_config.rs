// SPDX-License-Identifier: GPL-2.0-only
#[cfg(CONFIG_SYSFS)]
#[export_name = "rust_shmem_owner_shmem_enabled_show"]
unsafe extern "C" fn shmem_enabled_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let values = [
        RUST_SHMEM_SHMEM_HUGE_ALWAYS,
        RUST_SHMEM_SHMEM_HUGE_WITHIN_SIZE,
        RUST_SHMEM_SHMEM_HUGE_ADVISE,
        RUST_SHMEM_SHMEM_HUGE_NEVER,
        RUST_SHMEM_SHMEM_HUGE_DENY,
        RUST_SHMEM_SHMEM_HUGE_FORCE,
    ];
    let mut len = 0;
    for (i, value) in values.into_iter().enumerate() {
        len += sysfs_emit_at(
            buf,
            len,
            if rust_shmem_data_shmem_huge == value {
                c"%s[%s]".as_char_ptr()
            } else {
                c"%s%s".as_char_ptr()
            },
            if i == 0 {
                c"".as_char_ptr()
            } else {
                c" ".as_char_ptr()
            },
            shmem_format_huge(value),
        );
    }
    len += sysfs_emit_at(buf, len, c"\n".as_char_ptr());
    len as ssize_t
}
#[cfg(CONFIG_SYSFS)]
#[export_name = "rust_shmem_owner_shmem_enabled_store"]
unsafe extern "C" fn shmem_enabled_store(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const c_char,
    count: usize,
) -> ssize_t {
    let mut tmp = [0 as c_char; 16];
    if count.wrapping_add(1) > tmp.len() {
        return -(RUST_SHMEM_EINVAL as ssize_t);
    }
    core::ptr::copy_nonoverlapping(buf, tmp.as_mut_ptr(), count);
    tmp[count] = 0;
    if count != 0 && tmp[count - 1] == b'\n' as c_char {
        tmp[count - 1] = 0;
    }
    let huge = shmem_parse_huge(tmp.as_ptr());
    if huge == -(RUST_SHMEM_EINVAL as c_int) {
        return huge as ssize_t;
    }
    rust_shmem_data_shmem_huge = huge;
    if rust_shmem_data_shmem_huge > RUST_SHMEM_SHMEM_HUGE_DENY {
        (*SHMEM_SB((*rust_shmem_data_shm_mnt).mnt_sb)).huge = rust_shmem_data_shmem_huge as u8;
    }
    let err = start_stop_khugepaged();
    if err != 0 {
        err as ssize_t
    } else {
        count as ssize_t
    }
}
#[cfg(CONFIG_SYSFS)]
#[export_name = "rust_shmem_owner_thpsize_shmem_enabled_show"]
unsafe extern "C" fn thpsize_shmem_enabled_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let order = (*rust_shmem_to_thpsize(kobj)).order;
    let mut active = HUGE_SHMEM_ENABLED_NEVER as usize;
    let mut len = 0;
    for i in 0..rust_shmem_data_huge_mode_orders.len() {
        if rust_shmem_test_bit(order as _, rust_shmem_data_huge_mode_orders[i]) {
            active = i;
            break;
        }
    }
    for i in 0..rust_shmem_data_huge_mode_strings.len() {
        len += sysfs_emit_at(
            buf,
            len,
            if i == active {
                c"[%s] ".as_char_ptr()
            } else {
                c"%s ".as_char_ptr()
            },
            rust_shmem_data_huge_mode_strings[i],
        );
    }
    *buf.add(len as usize - 1) = b'\n' as c_char;
    len as ssize_t
}
#[cfg(CONFIG_SYSFS)]
unsafe fn set_shmem_enabled_mode(order: c_int, mode: huge_mode) -> bool {
    let mut changed = false;
    spin_lock(addr_of_mut!(rust_shmem_data_huge_shmem_orders_lock));
    for i in 0..rust_shmem_data_huge_mode_orders.len() {
        if i == mode as usize {
            changed |=
                !rust_shmem___test_and_set_bit(order as _, rust_shmem_data_huge_mode_orders[i]);
        } else {
            changed |=
                rust_shmem___test_and_clear_bit(order as _, rust_shmem_data_huge_mode_orders[i]);
        }
    }
    spin_unlock(addr_of_mut!(rust_shmem_data_huge_shmem_orders_lock));
    changed
}
#[cfg(CONFIG_SYSFS)]
#[export_name = "rust_shmem_owner_thpsize_shmem_enabled_store"]
unsafe extern "C" fn thpsize_shmem_enabled_store(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const c_char,
    count: usize,
) -> ssize_t {
    let order = (*rust_shmem_to_thpsize(kobj)).order;
    let mode = __sysfs_match_string(
        rust_shmem_data_huge_mode_strings.as_ptr(),
        rust_shmem_data_huge_mode_strings.len(),
        buf,
    );
    if mode < 0 {
        return mode as ssize_t;
    }
    if mode == HUGE_SHMEM_ENABLED_INHERIT as c_int
        && rust_shmem_data_shmem_huge == RUST_SHMEM_SHMEM_HUGE_FORCE
        && !is_pmd_order(order)
    {
        return -(RUST_SHMEM_EINVAL as ssize_t);
    }
    if set_shmem_enabled_mode(order, mode as huge_mode) {
        let err = start_stop_khugepaged();
        if err != 0 {
            return err as ssize_t;
        }
    } else {
        set_recommended_min_free_kbytes();
    }
    count as ssize_t
}
#[export_name = "rust_shmem_owner_setup_transparent_hugepage_shmem"]
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
unsafe extern "C" fn setup_transparent_hugepage_shmem(s: *mut c_char) -> c_int {
    let huge = shmem_parse_huge(s);
    if huge == -(RUST_SHMEM_EINVAL as c_int) {
        rust_shmem_pr_warn(c"transparent_hugepage_shmem= cannot parse, ignored\n".as_char_ptr());
        return huge;
    }
    rust_shmem_data_shmem_huge = huge;
    1
}
#[export_name = "rust_shmem_owner_setup_transparent_hugepage_tmpfs"]
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
unsafe extern "C" fn setup_transparent_hugepage_tmpfs(s: *mut c_char) -> c_int {
    let huge = shmem_parse_huge(s);
    if huge < 0 {
        rust_shmem_pr_warn(c"transparent_hugepage_tmpfs= cannot parse, ignored\n".as_char_ptr());
        return huge;
    }
    rust_shmem_data_tmpfs_huge = huge;
    1
}
#[export_name = "rust_shmem_owner_setup_thp_shmem"]
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
unsafe extern "C" fn setup_thp_shmem(s: *mut c_char) -> c_int {
    'parse: {
        if s.is_null() || rust_shmem_strlen(s).wrapping_add(1) > RUST_SHMEM_PAGE_SIZE as usize {
            break 'parse;
        }
        rust_shmem_copy_boot_string(s);
        let mut always = rust_shmem_data_huge_shmem_orders_always;
        let mut inherit = rust_shmem_data_huge_shmem_orders_inherit;
        let mut advise = rust_shmem_data_huge_shmem_orders_madvise;
        let mut within = rust_shmem_data_huge_shmem_orders_within_size;
        let mut p = rust_shmem_data_str_dup.as_mut_ptr();
        loop {
            let mut token = strsep(&mut p, c";".as_char_ptr());
            if token.is_null() {
                break;
            }
            let mut range = strsep(&mut token, c":".as_char_ptr());
            let policy = token;
            if policy.is_null() {
                break 'parse;
            }
            loop {
                let mut sub = strsep(&mut range, c",".as_char_ptr());
                if sub.is_null() {
                    break;
                }
                let (start_size, end_size);
                if !strchr(sub, b'-' as c_int).is_null() {
                    start_size = strsep(&mut sub, c"-".as_char_ptr());
                    end_size = sub;
                } else {
                    start_size = sub;
                    end_size = sub;
                }
                let start = get_order_from_str(
                    start_size,
                    RUST_SHMEM_THP_ORDERS_ALL_FILE_DEFAULT as c_ulong,
                );
                let end =
                    get_order_from_str(end_size, RUST_SHMEM_THP_ORDERS_ALL_FILE_DEFAULT as c_ulong);
                if start < 0 {
                    rust_shmem_invalid_thp_size(start_size);
                    break 'parse;
                }
                if end < 0 {
                    rust_shmem_invalid_thp_size(end_size);
                    break 'parse;
                }
                if start > end {
                    break 'parse;
                }
                let nr = end - start + 1;
                let selected = if strcmp(policy, c"always".as_char_ptr()) == 0 {
                    0
                } else if strcmp(policy, c"advise".as_char_ptr()) == 0 {
                    1
                } else if strcmp(policy, c"inherit".as_char_ptr()) == 0 {
                    2
                } else if strcmp(policy, c"within_size".as_char_ptr()) == 0 {
                    3
                } else if strcmp(policy, c"never".as_char_ptr()) == 0 {
                    4
                } else {
                    rust_shmem_invalid_thp_policy(policy);
                    break 'parse;
                };
                for (i, mask) in [&mut always, &mut advise, &mut inherit, &mut within]
                    .into_iter()
                    .enumerate()
                {
                    if i == selected {
                        bitmap_set(mask, start as _, nr as _);
                    } else {
                        bitmap_clear(mask, start as _, nr as _);
                    }
                }
            }
        }
        rust_shmem_data_huge_shmem_orders_always = always;
        rust_shmem_data_huge_shmem_orders_madvise = advise;
        rust_shmem_data_huge_shmem_orders_inherit = inherit;
        rust_shmem_data_huge_shmem_orders_within_size = within;
        rust_shmem_data_shmem_orders_configured = true;
        return 1;
    }
    rust_shmem_invalid_thp_string(s);
    0
}
