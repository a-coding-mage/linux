// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ioctl handler
 * Linux ethernet bridge
 *
 * Authors:
 * Lennert Buytenhek <buytenh@gnu.org>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn get_bridge_ifindices(net: *mut net, indices: *mut i32, num: i32) -> i32 {
    let mut dev: *mut net_device;
    let mut i: i32 = 0;

    rcu_read_lock();
    for_each_netdev_rcu!(net, dev) {
        if i >= num { break; }
        if netif_is_bridge_master(dev) {
            *indices.add(i as usize) = (*dev).ifindex;
            i += 1;
        }
    }
    rcu_read_unlock();
    i
}

/* called with RTNL */
unsafe fn get_port_ifindices(br: *mut net_bridge, ifindices: *mut i32, num: i32) {
    let mut p: *mut net_bridge_port;
    list_for_each_entry!(p, &(*br).port_list, list) {
        if (*p).port_no < num {
            *ifindices.add((*p).port_no as usize) = (*(*p).dev).ifindex;
        }
    }
}

/*
 * Format up to a page worth of forwarding table entries
 * userbuf -- where to copy result
 * maxnum  -- maximum number of entries desired
 *            (limited to a page for sanity)
 * offset  -- number of records to skip
 */
unsafe fn get_fdb_entries(br: *mut net_bridge, userbuf: *mut core::ffi::c_void,
                          mut maxnum: usize, offset: usize) -> i32 {
    let num: i32;
    let buf: *mut core::ffi::c_void;
    let size: usize;

    if maxnum > PAGE_SIZE / core::mem::size_of::<__fdb_entry>() {
        maxnum = PAGE_SIZE / core::mem::size_of::<__fdb_entry>();
    }
    size = maxnum * core::mem::size_of::<__fdb_entry>();
    buf = kmalloc(size, GFP_USER);
    if buf.is_null() { return -ENOMEM; }

    num = br_fdb_fillbuf(br, buf, maxnum, offset);
    let result = if num > 0 {
        if copy_to_user(userbuf, buf, (num as usize) * core::mem::size_of::<__fdb_entry>()) != 0 { -EFAULT } else { num }
    } else { num };
    kfree(buf);
    result
}

/* called with RTNL */
unsafe fn add_del_if(br: *mut net_bridge, ifindex: i32, isadd: i32) -> i32 {
    let net = dev_net((*br).dev);
    let dev: *mut net_device;
    if !ns_capable((*net).user_ns, CAP_NET_ADMIN) { return -EPERM; }
    dev = __dev_get_by_index(net, ifindex);
    if dev.is_null() { return -EINVAL; }
    if isadd != 0 { br_add_if(br, dev, core::ptr::null_mut()) } else { br_del_if(br, dev) }
}

const BR_UARGS_MAX: usize = 4;
unsafe fn br_dev_read_uargs(args: *mut usize, nr_args: usize,
                            argp: *mut *mut core::ffi::c_void,
                            data: *mut core::ffi::c_void) -> i32 {
    if nr_args < 2 || nr_args > BR_UARGS_MAX { return -EINVAL; }
    if in_compat_syscall() {
        let mut cargs = [0u32; BR_UARGS_MAX];
        if copy_from_user(cargs.as_mut_ptr() as *mut _, data, nr_args * core::mem::size_of::<u32>()) != 0 { return -EFAULT; }
        for i in 0..nr_args { *args.add(i) = cargs[i] as usize; }
        *argp = compat_ptr(*args.add(1) as u32);
    } else {
        if copy_from_user(args, data, nr_args * core::mem::size_of::<usize>()) != 0 { return -EFAULT; }
        *argp = *args.add(1) as *mut core::ffi::c_void;
    }
    0
}

/* Legacy ioctl's through SIOCDEVPRIVATE. */
pub unsafe fn br_dev_siocdevprivate(dev: *mut net_device, rq: *mut ifreq,
                                    data: *mut core::ffi::c_void, cmd: i32) -> i32 {
    let br = netdev_priv(dev) as *mut net_bridge;
    let mut p: *mut net_bridge_port = core::ptr::null_mut();
    let mut args = [0usize; 4];
    let mut argp: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut ret: i32;
    ret = br_dev_read_uargs(args.as_mut_ptr(), args.len(), &mut argp, data);
    if ret != 0 { return ret; }

    match args[0] {
        BRCTL_ADD_IF | BRCTL_DEL_IF => add_del_if(br, args[1] as i32, (args[0] == BRCTL_ADD_IF) as i32),
        BRCTL_GET_BRIDGE_INFO => {
            let mut b: __bridge_info = core::mem::zeroed();
            rcu_read_lock();
            core::ptr::copy_nonoverlapping(&(*br).designated_root as *const _, &mut b.designated_root as *mut _, 8);
            core::ptr::copy_nonoverlapping(&(*br).bridge_id as *const _, &mut b.bridge_id as *mut _, 8);
            b.root_path_cost = (*br).root_path_cost;
            b.max_age = jiffies_to_clock_t((*br).max_age);
            b.hello_time = jiffies_to_clock_t((*br).hello_time);
            b.forward_delay = (*br).forward_delay;
            b.bridge_max_age = (*br).bridge_max_age;
            b.bridge_hello_time = (*br).bridge_hello_time;
            b.bridge_forward_delay = jiffies_to_clock_t((*br).bridge_forward_delay);
            b.topology_change = (*br).topology_change;
            b.topology_change_detected = (*br).topology_change_detected;
            b.root_port = (*br).root_port;
            b.stp_enabled = ((*br).stp_enabled != BR_NO_STP) as _;
            b.ageing_time = jiffies_to_clock_t((*br).ageing_time);
            b.hello_timer_value = br_timer_value(&(*br).hello_timer);
            b.tcn_timer_value = br_timer_value(&(*br).tcn_timer);
            b.topology_change_timer_value = br_timer_value(&(*br).topology_change_timer);
            b.gc_timer_value = br_timer_value(&(*br).gc_work.timer);
            rcu_read_unlock();
            if copy_to_user(args[1] as *mut _, &b, core::mem::size_of_val(&b)) != 0 { -EFAULT } else { 0 }
        }
        BRCTL_GET_PORT_LIST => {
            let mut num = args[2] as i32;
            if num < 0 { return -EINVAL; }
            if num == 0 { num = 256; }
            if num > BR_MAX_PORTS { num = BR_MAX_PORTS; }
            let indices = kzalloc((num as usize) * core::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
            if indices.is_null() { return -ENOMEM; }
            get_port_ifindices(br, indices, num);
            ret = if copy_to_user(argp, indices, (num as usize) * core::mem::size_of::<i32>()) != 0 { -EFAULT } else { num };
            kfree(indices as *mut _); ret
        }
        BRCTL_SET_BRIDGE_FORWARD_DELAY => { if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; } br_set_forward_delay(br, args[1] as _ ) }
        BRCTL_SET_BRIDGE_HELLO_TIME => { if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; } br_set_hello_time(br, args[1] as _ ) }
        BRCTL_SET_BRIDGE_MAX_AGE => { if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; } br_set_max_age(br, args[1] as _ ) }
        BRCTL_SET_AGEING_TIME => { if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; } br_set_ageing_time(br, args[1] as _ ) }
        BRCTL_GET_PORT_INFO => {
            let mut pinfo: __port_info = core::mem::zeroed();
            rcu_read_lock();
            let pt = br_get_port(br, args[2] as _);
            if pt.is_null() { rcu_read_unlock(); return -EINVAL; }
            core::ptr::copy_nonoverlapping(&(*pt).designated_root as *const _, &mut pinfo.designated_root as *mut _, 8);
            core::ptr::copy_nonoverlapping(&(*pt).designated_bridge as *const _, &mut pinfo.designated_bridge as *mut _, 8);
            pinfo.port_id = READ_ONCE((*pt).port_id);
            pinfo.designated_port = READ_ONCE((*pt).designated_port);
            pinfo.path_cost = READ_ONCE((*pt).path_cost);
            pinfo.designated_cost = READ_ONCE((*pt).designated_cost);
            pinfo.state = (*pt).state;
            pinfo.top_change_ack = (*pt).topology_change_ack;
            pinfo.config_pending = READ_ONCE((*pt).config_pending);
            pinfo.message_age_timer_value = br_timer_value(&(*pt).message_age_timer);
            pinfo.forward_delay_timer_value = br_timer_value(&(*pt).forward_delay_timer);
            pinfo.hold_timer_value = br_timer_value(&(*pt).hold_timer);
            rcu_read_unlock();
            if copy_to_user(argp, &pinfo, core::mem::size_of_val(&pinfo)) != 0 { -EFAULT } else { 0 }
        }
        BRCTL_SET_BRIDGE_STP_STATE => { if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; } br_stp_set_enabled(br, args[1] as _, core::ptr::null_mut()) }
        BRCTL_SET_BRIDGE_PRIORITY => { if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; } br_stp_set_bridge_priority(br, args[1] as _); 0 }
        BRCTL_SET_PORT_PRIORITY => {
            if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; }
            spin_lock_bh(&mut (*br).lock);
            p = br_get_port(br, args[1] as _);
            ret = if p.is_null() { -EINVAL } else { br_stp_set_port_priority(p, args[2] as _) };
            spin_unlock_bh(&mut (*br).lock); ret
        }
        BRCTL_SET_PATH_COST => {
            if !ns_capable(dev_net(dev).user_ns, CAP_NET_ADMIN) { return -EPERM; }
            spin_lock_bh(&mut (*br).lock);
            p = br_get_port(br, args[1] as _);
            ret = if p.is_null() { -EINVAL } else { br_stp_set_path_cost(p, args[2] as _) };
            spin_unlock_bh(&mut (*br).lock); ret
        }
        BRCTL_GET_FDB_ENTRIES => get_fdb_entries(br, argp, args[2], args[3]),
        _ => -EOPNOTSUPP
    }
}

unsafe fn old_deviceless(net: *mut net, data: *mut core::ffi::c_void) -> i32 {
    let mut args = [0usize; 3]; let mut argp = core::ptr::null_mut();
    let ret = br_dev_read_uargs(args.as_mut_ptr(), args.len(), &mut argp, data); if ret != 0 { return ret; }
    match args[0] {
        BRCTL_GET_VERSION => BRCTL_VERSION,
        BRCTL_GET_BRIDGES => {
            if args[2] >= 2048 { return -ENOMEM; }
            let indices = kzalloc(args[2] * core::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
            if indices.is_null() { return -ENOMEM; }
            let n = get_bridge_ifindices(net, indices, args[2] as i32) as usize;
            let r = if copy_to_user(argp, indices, n * core::mem::size_of::<i32>()) != 0 { -EFAULT } else { n as i32 };
            kfree(indices as *mut _); r
        }
        BRCTL_ADD_BRIDGE | BRCTL_DEL_BRIDGE => {
            if !ns_capable((*net).user_ns, CAP_NET_ADMIN) { return -EPERM; }
            let mut buf = [0i8; IFNAMSIZ];
            if copy_from_user(buf.as_mut_ptr() as *mut _, argp, IFNAMSIZ) != 0 { return -EFAULT; }
            buf[IFNAMSIZ - 1] = 0;
            if args[0] == BRCTL_ADD_BRIDGE { br_add_bridge(net, buf.as_mut_ptr()) } else { br_del_bridge(net, buf.as_mut_ptr()) }
        }
        _ => -EOPNOTSUPP,
    }
}

pub unsafe fn br_ioctl_stub(net: *mut net, cmd: u32, uarg: *mut core::ffi::c_void) -> i32 {
    let mut ret = -EOPNOTSUPP;
    let mut ifr: ifreq = core::mem::zeroed();
    if cmd == SIOCBRADDIF || cmd == SIOCBRDELIF {
        let mut data = core::ptr::null_mut();
        if !ns_capable((*net).user_ns, CAP_NET_ADMIN) { return -EPERM; }
        if get_user_ifreq(&mut ifr, &mut data, uarg) != 0 { return -EFAULT; }
        ifr.ifr_name[IFNAMSIZ - 1] = 0;
        let mut colon = ifr.ifr_name.as_mut_ptr();
        while !colon.is_null() && *colon != 0 { if *colon == b':' as i8 { *colon = 0; break; } colon = colon.add(1); }
    }
    rtnl_lock();
    match cmd {
        SIOCGIFBR | SIOCSIFBR => ret = old_deviceless(net, uarg),
        SIOCBRADDBR | SIOCBRDELBR => {
            if !ns_capable((*net).user_ns, CAP_NET_ADMIN) { ret = -EPERM; }
            else { let mut buf = [0i8; IFNAMSIZ]; ret = if copy_from_user(buf.as_mut_ptr() as *mut _, uarg, IFNAMSIZ) != 0 { -EFAULT } else if cmd == SIOCBRADDBR { br_add_bridge(net, buf.as_mut_ptr()) } else { br_del_bridge(net, buf.as_mut_ptr()) }; }
        }
        SIOCBRADDIF | SIOCBRDELIF => {
            let dev = __dev_get_by_name(net, ifr.ifr_name.as_ptr());
            if dev.is_null() || !netif_device_present(dev) { ret = -ENODEV; }
            else if !netif_is_bridge_master(dev) { ret = -EOPNOTSUPP; }
            else { ret = add_del_if(netdev_priv(dev) as *mut net_bridge, ifr.ifr_ifindex, (cmd == SIOCBRADDIF) as i32); }
        }
        _ => {}
    }
    rtnl_unlock(); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
