// SPDX-License-Identifier: GPL-2.0
/*
 * linux/kernel/capability.c
 *
 * Copyright (C) 1997  Andrew Main <zefram@fysh.org>
 *
 * Integrated into 2.1.97+,  Andrew G. Morgan <morgan@kernel.org>
 * 30 May 2002: Cleanup, Robert M. Love <rml@tech9.net>
 */

// #include dependencies are supplied by the kernel environment.

pub static mut file_caps_enabled: i32 = 1;

unsafe fn file_caps_disable(_str: *mut i8) -> i32 {
    file_caps_enabled = 0;
    1
}

// __setup("no_file_caps", file_caps_disable);

#[cfg(CONFIG_MULTIUSER)]
unsafe fn warn_legacy_capability_use() {
    pr_info_once!("warning: `{}` uses 32-bit capabilities (legacy support in use)\n", (*current).comm);
}

#[cfg(CONFIG_MULTIUSER)]
unsafe fn warn_deprecated_v2() {
    pr_info_once!("warning: `{}` uses deprecated v2 capabilities in a way that may be insecure\n", (*current).comm);
}

#[cfg(CONFIG_MULTIUSER)]
unsafe fn cap_validate_magic(header: cap_user_header_t, tocopy: *mut u32) -> i32 {
    let mut version: __u32 = 0;

    if get_user(&mut version, &(*header).version) != 0 {
        return -EFAULT;
    }

    match version {
        _LINUX_CAPABILITY_VERSION_1 => {
            warn_legacy_capability_use();
            *tocopy = _LINUX_CAPABILITY_U32S_1;
        }
        _LINUX_CAPABILITY_VERSION_2 => {
            warn_deprecated_v2();
            *tocopy = _LINUX_CAPABILITY_U32S_3;
        }
        _LINUX_CAPABILITY_VERSION_3 => {
            *tocopy = _LINUX_CAPABILITY_U32S_3;
        }
        _ => {
            if put_user(_KERNEL_CAPABILITY_VERSION as u32, &mut (*header).version) != 0 {
                return -EFAULT;
            }
            return -EINVAL;
        }
    }
    0
}

#[cfg(CONFIG_MULTIUSER)]
unsafe fn cap_get_target_pid(pid: pid_t, p_ep: *mut kernel_cap_t,
                             p_ip: *mut kernel_cap_t, p_pp: *mut kernel_cap_t) -> i32 {
    let ret: i32;
    if pid != 0 && pid != task_pid_vnr(current) {
        rcu_read_lock();
        let target = find_task_by_vpid(pid);
        ret = if target.is_null() { -ESRCH } else { security_capget(target, p_ep, p_ip, p_pp) };
        rcu_read_unlock();
    } else {
        ret = security_capget(current, p_ep, p_ip, p_pp);
    }
    ret
}

#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn capget(header: cap_user_header_t, dataptr: cap_user_data_t) -> i32 {
    let mut tocopy: u32 = 0;
    let ret = cap_validate_magic(header, &mut tocopy);
    if dataptr.is_null() || ret != 0 {
        return if dataptr.is_null() && ret == -EINVAL { 0 } else { ret };
    }
    let mut pid: pid_t = 0;
    if get_user(&mut pid, &(*header).pid) != 0 { return -EFAULT; }
    if pid < 0 { return -EINVAL; }

    let (mut p_e, mut p_i, mut p_p): (kernel_cap_t, kernel_cap_t, kernel_cap_t) = (core::mem::zeroed(), core::mem::zeroed(), core::mem::zeroed());
    let ret = cap_get_target_pid(pid, &mut p_e, &mut p_i, &mut p_p);
    if ret != 0 { return ret; }

    let mut kdata: [__user_cap_data_struct; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    kdata[0].effective = p_e.val as u32; kdata[1].effective = (p_e.val >> 32) as u32;
    kdata[0].permitted = p_p.val as u32; kdata[1].permitted = (p_p.val >> 32) as u32;
    kdata[0].inheritable = p_i.val as u32; kdata[1].inheritable = (p_i.val >> 32) as u32;

    if copy_to_user(dataptr, kdata.as_ptr() as *const _, tocopy as usize * core::mem::size_of::<__user_cap_data_struct>()) != 0 { return -EFAULT; }
    0
}

#[cfg(CONFIG_MULTIUSER)]
unsafe fn mk_kernel_cap(low: u32, high: u32) -> kernel_cap_t {
    kernel_cap_t { val: ((low as u64 | ((high as u64) << 32)) & CAP_VALID_MASK) }
}

#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn capset(header: cap_user_header_t, data: cap_user_data_t) -> i32 {
    let mut kdata: [__user_cap_data_struct; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    let mut tocopy = 0u32;
    let ret = cap_validate_magic(header, &mut tocopy);
    if ret != 0 { return ret; }
    let mut pid = 0;
    if get_user(&mut pid, &(*header).pid) != 0 { return -EFAULT; }
    if pid != 0 && pid != task_pid_vnr(current) { return -EPERM; }
    let copybytes = tocopy as usize * core::mem::size_of::<__user_cap_data_struct>();
    if copybytes > core::mem::size_of_val(&kdata) { return -EFAULT; }
    if copy_from_user(kdata.as_mut_ptr() as *mut _, data, copybytes) != 0 { return -EFAULT; }

    let effective = mk_kernel_cap(kdata[0].effective, kdata[1].effective);
    let permitted = mk_kernel_cap(kdata[0].permitted, kdata[1].permitted);
    let inheritable = mk_kernel_cap(kdata[0].inheritable, kdata[1].inheritable);
    let new = prepare_creds();
    if new.is_null() { return -ENOMEM; }
    let ret = security_capset(new, current_cred(), &effective, &inheritable, &permitted);
    if ret < 0 { abort_creds(new); return ret; }
    audit_log_capset(new, current_cred());
    commit_creds(new)
}

#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn has_ns_capability(t: *mut task_struct, ns: *mut user_namespace, cap: i32) -> bool {
    rcu_read_lock();
    let ret = security_capable(__task_cred(t), ns, cap, CAP_OPT_NONE);
    rcu_read_unlock();
    ret == 0
}

#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn has_ns_capability_noaudit(t: *mut task_struct, ns: *mut user_namespace, cap: i32) -> bool {
    rcu_read_lock();
    let ret = security_capable(__task_cred(t), ns, cap, CAP_OPT_NOAUDIT);
    rcu_read_unlock();
    ret == 0
}

#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn has_capability_noaudit(t: *mut task_struct, cap: i32) -> bool {
    has_ns_capability_noaudit(t, &mut init_user_ns, cap)
}

#[cfg(CONFIG_MULTIUSER)]
unsafe fn ns_capable_common(ns: *mut user_namespace, cap: i32, opts: u32) -> bool {
    if !cap_valid(cap) { pr_crit!("capable() called with invalid cap={}\n", cap); BUG!(); }
    if security_capable(current_cred(), ns, cap, opts) == 0 {
        (*current).flags |= PF_SUPERPRIV;
        true
    } else { false }
}

#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn ns_capable(ns: *mut user_namespace, cap: i32) -> bool { ns_capable_common(ns, cap, CAP_OPT_NONE) }
#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn ns_capable_noaudit(ns: *mut user_namespace, cap: i32) -> bool { ns_capable_common(ns, cap, CAP_OPT_NOAUDIT) }
#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn ns_capable_setid(ns: *mut user_namespace, cap: i32) -> bool { ns_capable_common(ns, cap, CAP_OPT_INSETID) }
#[cfg(CONFIG_MULTIUSER)]
pub unsafe fn capable(cap: i32) -> bool { ns_capable(&mut init_user_ns, cap) }

pub unsafe fn file_ns_capable(file: *const file, ns: *mut user_namespace, cap: i32) -> bool {
    if !cap_valid(cap) { WARN_ON_ONCE!(); return false; }
    security_capable((*file).f_cred, ns, cap, CAP_OPT_NONE) == 0
}

pub unsafe fn privileged_wrt_inode_uidgid(ns: *mut user_namespace, idmap: *mut mnt_idmap, inode: *const inode) -> bool {
    vfsuid_has_mapping(ns, i_uid_into_vfsuid(idmap, inode)) && vfsgid_has_mapping(ns, i_gid_into_vfsgid(idmap, inode))
}

pub unsafe fn capable_wrt_inode_uidgid(idmap: *mut mnt_idmap, inode: *const inode, cap: i32) -> bool {
    let ns = current_user_ns();
    ns_capable(ns, cap) && privileged_wrt_inode_uidgid(ns, idmap, inode)
}

pub unsafe fn ptracer_capable(tsk: *mut task_struct, ns: *mut user_namespace) -> bool {
    let mut ret = 0;
    rcu_read_lock();
    let cred = rcu_dereference((*tsk).ptracer_cred);
    if !cred.is_null() { ret = security_capable(cred, ns, CAP_SYS_PTRACE, CAP_OPT_NOAUDIT); }
    rcu_read_unlock();
    ret == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
