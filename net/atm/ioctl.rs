// SPDX-License-Identifier: GPL-2.0
/* ATM ioctl handling */

/* Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA */
/* 2003 John Levon  <levon@movementarian.org> */

// pr_fmt(fmt) KBUILD_MODNAME ":%s: " fmt, __func__
// Kernel includes and local headers are supplied by the surrounding build.

static mut ioctl_mutex: /* DEFINE_MUTEX */ () = ();
static mut ioctl_list: /* LIST_HEAD */ () = ();

pub unsafe fn register_atm_ioctl(ioctl: *mut atm_ioctl) {
    mutex_lock(&raw mut ioctl_mutex);
    list_add_tail(&mut (*ioctl).list, &raw mut ioctl_list);
    mutex_unlock(&raw mut ioctl_mutex);
}

pub unsafe fn deregister_atm_ioctl(ioctl: *mut atm_ioctl) {
    mutex_lock(&raw mut ioctl_mutex);
    list_del(&mut (*ioctl).list);
    mutex_unlock(&raw mut ioctl_mutex);
}

unsafe fn do_vcc_ioctl(
    sock: *mut socket,
    cmd: c_uint,
    arg: c_ulong,
    compat: c_int,
) -> c_int {
    let sk = (*sock).sk;
    let vcc: *mut atm_vcc;
    let mut error: c_int;
    let mut pos: *mut list_head;
    let argp = arg as *mut c_void;
    let mut buf: *mut c_void;
    let mut len: *mut c_int;

    vcc = ATM_SD(sock);
    match cmd {
        SIOCOUTQ => {
            if (*sock).state != SS_CONNECTED || !test_bit(ATM_VF_READY, &(*vcc).flags) {
                error = -EINVAL;
                return error;
            }
            error = put_user((*sk).sk_sndbuf - sk_wmem_alloc_get(sk), argp as *mut c_int);
            return error;
        }
        SIOCINQ => {
            let mut skb: *mut sk_buff;
            let amount: c_int;
            if (*sock).state != SS_CONNECTED {
                error = -EINVAL;
                return error;
            }
            spin_lock_irq(&mut (*sk).sk_receive_queue.lock);
            skb = skb_peek(&mut (*sk).sk_receive_queue);
            amount = if !skb.is_null() { (*skb).len } else { 0 };
            spin_unlock_irq(&mut (*sk).sk_receive_queue.lock);
            error = put_user(amount, argp as *mut c_int);
            return error;
        }
        ATM_SETSC => {
            net_warn_ratelimited!("ATM_SETSC is obsolete; used by %s:%d\n", current!().comm, task_pid_nr(current!()));
            error = 0;
            return error;
        }
        ATM_SETBACKEND | ATM_NEWBACKENDIF => {
            let mut backend: atm_backend_t = core::mem::zeroed();
            error = get_user(&mut backend, argp as *mut atm_backend_t);
            if error != 0 { return error; }
            match backend {
                ATM_BACKEND_PPP => { request_module!("pppoatm"); }
                ATM_BACKEND_BR2684 => { request_module!("br2684"); }
                _ => {}
            }
        }
        _ => {}
    }

    error = -ENOIOCTLCMD;
    mutex_lock(&raw mut ioctl_mutex);
    list_for_each(pos, &raw mut ioctl_list) {
        let ic = list_entry!(pos, atm_ioctl, list);
        if try_module_get((*ic).owner) {
            error = ((*ic).ioctl)(sock, cmd, arg);
            module_put((*ic).owner);
            if error != -ENOIOCTLCMD { break; }
        }
    }
    mutex_unlock(&raw mut ioctl_mutex);

    if error != -ENOIOCTLCMD { return error; }

    if cmd == ATM_GETNAMES {
        if IS_ENABLED!(CONFIG_COMPAT) && compat != 0 {
            // #ifdef CONFIG_COMPAT
            let ciobuf = argp as *mut compat_atm_iobuf;
            let mut cbuf: compat_uptr_t = 0;
            len = &mut (*ciobuf).length;
            if get_user(&mut cbuf, &mut (*ciobuf).buffer) != 0 { return -EFAULT; }
            buf = compat_ptr(cbuf);
        } else {
            let iobuf = argp as *mut atm_iobuf;
            len = &mut (*iobuf).length;
            if get_user(&mut buf, &mut (*iobuf).buffer) != 0 { return -EFAULT; }
        }
        error = atm_getnames(buf, len);
    } else {
        let mut number: c_int = 0;
        if IS_ENABLED!(CONFIG_COMPAT) && compat != 0 {
            // #ifdef CONFIG_COMPAT
            let csioc = argp as *mut compat_atmif_sioc;
            let mut carg: compat_uptr_t = 0;
            len = &mut (*csioc).length;
            if get_user(&mut carg, &mut (*csioc).arg) != 0 { return -EFAULT; }
            buf = compat_ptr(carg);
            if get_user(&mut number, &mut (*csioc).number) != 0 { return -EFAULT; }
        } else {
            let sioc = argp as *mut atmif_sioc;
            len = &mut (*sioc).length;
            if get_user(&mut buf, &mut (*sioc).arg) != 0 { return -EFAULT; }
            if get_user(&mut number, &mut (*sioc).number) != 0 { return -EFAULT; }
        }
        error = atm_dev_ioctl(cmd, buf, len, number, compat);
    }
    error
}

pub unsafe fn vcc_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
    do_vcc_ioctl(sock, cmd, arg, 0)
}

// The following compatibility ioctl conversion is present only under CONFIG_COMPAT.
// FIXME: The compat_ioctl handling is duplicated and incomplete; the two paths
// should be merged as described in the original source.

#[cfg(CONFIG_COMPAT)]
mod compat_ioctl {
    use super::*;

    const ATM_GETLINKRATE32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 1, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_GETNAMES32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 3, core::mem::size_of::<compat_atm_iobuf>());
    const ATM_GETTYPE32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 4, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_GETESI32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 5, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_GETCIRANGE32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 10, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_SETCIRANGE32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 11, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_SETESI32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 12, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_SETESIF32: c_uint = _IOW(b'a' as c_uint, ATMIOC_ITF + 13, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_GETSTAT32: c_uint = _IOW(b'a' as c_uint, ATMIOC_SARCOM, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_GETSTATZ32: c_uint = _IOW(b'a' as c_uint, ATMIOC_SARCOM + 1, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_GETLOOP32: c_uint = _IOW(b'a' as c_uint, ATMIOC_SARCOM + 2, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_SETLOOP32: c_uint = _IOW(b'a' as c_uint, ATMIOC_SARCOM + 3, core::mem::size_of::<compat_atmif_sioc>());
    const ATM_QUERYLOOP32: c_uint = _IOW(b'a' as c_uint, ATMIOC_SARCOM + 4, core::mem::size_of::<compat_atmif_sioc>());

    struct AtmIoctlMap { cmd32: c_uint, cmd: c_uint }
    static ATM_IOCTL_MAP: &[AtmIoctlMap] = &[
        AtmIoctlMap { cmd32: ATM_GETLINKRATE32, cmd: ATM_GETLINKRATE }, AtmIoctlMap { cmd32: ATM_GETNAMES32, cmd: ATM_GETNAMES },
        AtmIoctlMap { cmd32: ATM_GETTYPE32, cmd: ATM_GETTYPE }, AtmIoctlMap { cmd32: ATM_GETESI32, cmd: ATM_GETESI },
        AtmIoctlMap { cmd32: ATM_GETCIRANGE32, cmd: ATM_GETCIRANGE }, AtmIoctlMap { cmd32: ATM_SETCIRANGE32, cmd: ATM_SETCIRANGE },
        AtmIoctlMap { cmd32: ATM_SETESI32, cmd: ATM_SETESI }, AtmIoctlMap { cmd32: ATM_SETESIF32, cmd: ATM_SETESIF },
        AtmIoctlMap { cmd32: ATM_GETSTAT32, cmd: ATM_GETSTAT }, AtmIoctlMap { cmd32: ATM_GETSTATZ32, cmd: ATM_GETSTATZ },
        AtmIoctlMap { cmd32: ATM_GETLOOP32, cmd: ATM_GETLOOP }, AtmIoctlMap { cmd32: ATM_SETLOOP32, cmd: ATM_SETLOOP },
        AtmIoctlMap { cmd32: ATM_QUERYLOOP32, cmd: ATM_QUERYLOOP },
    ];

    unsafe fn do_atm_iobuf(_sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
        let iobuf32 = compat_ptr(arg) as *mut compat_atm_iobuf;
        let mut data: u32 = 0;
        if get_user(&mut data, &mut (*iobuf32).buffer) != 0 { return -EFAULT; }
        atm_getnames(compat_ptr(data), &mut (*iobuf32).length)
    }

    unsafe fn do_atmif_sioc(_sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
        let sioc32 = compat_ptr(arg) as *mut compat_atmif_sioc;
        let mut number = 0;
        let mut data: u32 = 0;
        if get_user(&mut data, &mut (*sioc32).arg) != 0 || get_user(&mut number, &mut (*sioc32).number) != 0 { return -EFAULT; }
        atm_dev_ioctl(cmd, compat_ptr(data), &mut (*sioc32).length, number, 0)
    }

    unsafe fn do_atm_ioctl(sock: *mut socket, cmd32: c_uint, arg: c_ulong) -> c_int {
        let mut cmd = 0;
        let mut i = 0;
        while i < ATM_IOCTL_MAP.len() {
            if cmd32 == ATM_IOCTL_MAP[i].cmd32 { cmd = ATM_IOCTL_MAP[i].cmd; break; }
            i += 1;
        }
        if i == ATM_IOCTL_MAP.len() { return -EINVAL; }
        match cmd {
            ATM_GETNAMES => do_atm_iobuf(sock, cmd, arg),
            ATM_GETLINKRATE | ATM_GETTYPE | ATM_GETESI | ATM_GETCIRANGE | ATM_SETCIRANGE |
            ATM_SETESI | ATM_SETESIF | ATM_GETSTAT | ATM_GETSTATZ | ATM_GETLOOP |
            ATM_SETLOOP | ATM_QUERYLOOP => do_atmif_sioc(sock, cmd, arg),
            _ => -EINVAL,
        }
    }

    pub unsafe fn vcc_compat_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
        let ret = do_vcc_ioctl(sock, cmd, arg, 1);
        if ret != -ENOIOCTLCMD { ret } else { do_atm_ioctl(sock, cmd, arg) }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
