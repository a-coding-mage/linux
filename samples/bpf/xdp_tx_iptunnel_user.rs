// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook */

// External declarations corresponding to the C headers used by this source.
// The supplied headers and bindings are provided by the surrounding build.

const STATS_INTERVAL_S: u32 = 2;

static mut ifindex: i32 = -1;
static mut xdp_flags: u32 = XDP_FLAGS_UPDATE_IF_NOEXIST;
static mut rxcnt_map_fd: i32 = 0;
static mut prog_id: u32 = 0;

unsafe fn int_exit(_sig: i32) {
    let mut curr_prog_id: u32 = 0;

    if ifindex > -1 {
        if bpf_xdp_query_id(ifindex, xdp_flags, &mut curr_prog_id) != 0 {
            printf(b"bpf_xdp_query_id failed\n\0".as_ptr() as *const i8);
            exit(1);
        }
        if prog_id == curr_prog_id {
            bpf_xdp_detach(ifindex, xdp_flags, core::ptr::null_mut());
        } else if curr_prog_id == 0 {
            printf(b"couldn't find a prog id on a given iface\n\0".as_ptr() as *const i8);
        } else {
            printf(b"program on interface changed, not removing\n\0".as_ptr() as *const i8);
        }
    }
    exit(0);
}

/* simple per-protocol drop counter */
unsafe fn poll_stats(kill_after_s: u32) {
    let nr_protos: usize = 256;
    let nr_cpus: usize = bpf_num_possible_cpus() as usize;
    let started_at = time(core::ptr::null_mut());
    let mut values = vec![0u64; nr_cpus];
    let mut prev = vec![vec![0u64; nr_cpus]; nr_protos];
    let mut proto: u32;

    while kill_after_s == 0 || time(core::ptr::null_mut()).wrapping_sub(started_at) <= kill_after_s as i64 {
        sleep(STATS_INTERVAL_S);

        for p in 0..nr_protos {
            proto = p as u32;
            let mut sum: u64 = 0;

            assert!(bpf_map_lookup_elem(rxcnt_map_fd, &proto as *const u32 as *const _, values.as_mut_ptr() as *mut _) == 0);
            for i in 0..nr_cpus {
                sum = sum.wrapping_add(values[i].wrapping_sub(prev[p][i]));
            }

            if sum != 0 {
                printf(
                    b"proto %u: sum:%10llu pkts, rate:%10llu pkts/s\n\0".as_ptr() as *const i8,
                    proto, sum, sum / STATS_INTERVAL_S as u64,
                );
            }
            prev[p].copy_from_slice(&values);
        }
    }
}

unsafe fn usage(cmd: *const i8) {
    printf(b"Start a XDP prog which encapsulates incoming packets\nin an IPv4/v6 header and XDP_TX it out.  The dst <VIP:PORT>\nis used to select packets to encapsulate\n\n\0".as_ptr() as *const i8);
    printf(b"Usage: %s [... ]\n\0".as_ptr() as *const i8, cmd);
    printf(b"    -i <ifname|ifindex> Interface\n    -a <vip-service-address> IPv4 or IPv6\n    -p <vip-service-port> A port range (e.g. 433-444) is also allowed\n    -s <source-ip> Used in the IPTunnel header\n    -d <dest-ip> Used in the IPTunnel header\n    -m <dest-MAC> Used in sending the IP Tunneled pkt\n    -T <stop-after-X-seconds> Default: 0 (forever)\n    -P <IP-Protocol> Default is TCP\n    -S use skb-mode\n    -N enforce native mode\n    -F Force loading the XDP prog\n    -h Display this help\n\0".as_ptr() as *const i8);
}

unsafe fn parse_ipstr(ipstr: *const i8, addr: *mut u32) -> i32 {
    if inet_pton(AF_INET6, ipstr, addr as *mut _) == 1 {
        AF_INET6
    } else if inet_pton(AF_INET, ipstr, addr as *mut _) == 1 {
        *addr.add(1) = 0;
        *addr.add(2) = 0;
        *addr.add(3) = 0;
        AF_INET
    } else {
        fprintf(stderr, b"%s is an invalid IP\n\0".as_ptr() as *const i8, ipstr);
        AF_UNSPEC
    }
}

unsafe fn parse_ports(port_str: *const i8, min_port: *mut i32, max_port: *mut i32) -> i32 {
    let mut end: *mut i8 = core::ptr::null_mut();
    let tmp_min_port = strtol(optarg, &mut end, 10);
    if tmp_min_port < 1 || tmp_min_port > 65535 {
        fprintf(stderr, b"Invalid port(s):%s\n\0".as_ptr() as *const i8, optarg);
        return 1;
    }
    let tmp_max_port = if *end == b'-' as i8 {
        strtol(end.add(1), core::ptr::null_mut(), 10)
    } else { tmp_min_port };
    if tmp_max_port < 1 || tmp_max_port > 65535 || tmp_min_port > tmp_max_port {
        fprintf(stderr, b"Invalid port(s):%s\n\0".as_ptr() as *const i8, optarg);
        return 1;
    }
    if tmp_max_port - tmp_min_port + 1 > MAX_IPTNL_ENTRIES as i64 {
        fprintf(stderr, b"Port range (%s) is larger than %u\n\0".as_ptr() as *const i8, port_str, MAX_IPTNL_ENTRIES);
        return 1;
    }
    *min_port = tmp_min_port as i32;
    *max_port = tmp_max_port as i32;
    0
}

pub unsafe fn main(argc: i32, argv: *mut *mut i8) -> i32 {
    let mut min_port = 0i32;
    let mut max_port = 0i32;
    let mut vip2tnl_map_fd: i32;
    let optstr = b"i:a:p:s:d:m:T:P:FSNh\0";
    let mut opt_flags = [0u8; 256];
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_len: u32 = core::mem::size_of::<bpf_prog_info>() as u32;
    let mut kill_after_s = 0u32;
    let mut tnl: iptnl_info = core::mem::zeroed();
    let mut vip: vip = core::mem::zeroed();
    let mut filename = [0i8; 256];
    let mut opt: i32;
    let mut prog_fd: i32;
    let mut err: i32;

    tnl.family = AF_UNSPEC as _;
    vip.protocol = IPPROTO_TCP as _;
    let optstr_len = strlen(optstr.as_ptr() as *const i8);
    for i in 0..optstr_len {
        let c = optstr[i as usize];
        if c != b'h' && c >= b'a' && c <= b'z' { opt_flags[c as usize] = 1; }
    }
    while { opt = getopt(argc, argv, optstr.as_ptr() as *const i8); opt != -1 } {
        match opt {
            b'i' as i32 => { ifindex = if_nametoindex(optarg); if ifindex == 0 { ifindex = atoi(optarg); } }
            b'a' as i32 => { vip.family = parse_ipstr(optarg, vip.daddr.v6.as_mut_ptr()); if vip.family == AF_UNSPEC { return 1; } }
            b'p' as i32 => { if parse_ports(optarg, &mut min_port, &mut max_port) != 0 { return 1; } }
            b'P' as i32 => { vip.protocol = atoi(optarg) as _; }
            b's' as i32 | b'd' as i32 => {
                let v6 = if opt == b's' as i32 { tnl.saddr.v6.as_mut_ptr() } else { tnl.daddr.v6.as_mut_ptr() };
                let family = parse_ipstr(optarg, v6);
                if family == AF_UNSPEC { return 1; }
                if tnl.family == AF_UNSPEC as _ { tnl.family = family as _; }
                else if tnl.family != family as _ { fprintf(stderr, b"The IP version of the src and dst addresses used in the IP encapsulation does not match\n\0".as_ptr() as *const i8); return 1; }
            }
            b'm' as i32 => { if ether_aton_r(optarg, &mut *(tnl.dmac.as_mut_ptr() as *mut ether_addr)).is_null() { fprintf(stderr, b"Invalid mac address:%s\n\0".as_ptr() as *const i8, optarg); return 1; } }
            b'T' as i32 => { kill_after_s = atoi(optarg) as u32; }
            b'S' as i32 => { xdp_flags |= XDP_FLAGS_SKB_MODE; }
            b'N' as i32 => {}
            b'F' as i32 => { xdp_flags &= !XDP_FLAGS_UPDATE_IF_NOEXIST; }
            _ => { usage(*argv); return 1; }
        }
        opt_flags[opt as usize] = 0;
    }
    if xdp_flags & XDP_FLAGS_SKB_MODE == 0 { xdp_flags |= XDP_FLAGS_DRV_MODE; }
    for i in 0..optstr_len { let c = optstr[i as usize]; if opt_flags[c as usize] != 0 { fprintf(stderr, b"Missing argument -%c\n\0".as_ptr() as *const i8, c); usage(*argv); return 1; } }
    if ifindex == 0 { fprintf(stderr, b"Invalid ifname\n\0".as_ptr() as *const i8); return 1; }
    snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as *const i8, *argv);
    let obj = bpf_object__open_file(filename.as_ptr(), core::ptr::null());
    if libbpf_get_error(obj) != 0 { return 1; }
    let prog = bpf_object__next_program(obj, core::ptr::null_mut());
    bpf_program__set_type(prog, BPF_PROG_TYPE_XDP);
    err = bpf_object__load(obj); if err != 0 { printf(b"bpf_object__load(): %s\n\0".as_ptr() as *const i8, strerror(errno)); return 1; }
    prog_fd = bpf_program__fd(prog);
    rxcnt_map_fd = bpf_object__find_map_fd_by_name(obj, b"rxcnt\0".as_ptr() as *const i8);
    vip2tnl_map_fd = bpf_object__find_map_fd_by_name(obj, b"vip2tnl\0".as_ptr() as *const i8);
    if vip2tnl_map_fd < 0 || rxcnt_map_fd < 0 { printf(b"bpf_object__find_map_fd_by_name failed\n\0".as_ptr() as *const i8); return 1; }
    signal(SIGINT, int_exit); signal(SIGTERM, int_exit);
    while min_port <= max_port { vip.dport = htons(min_port as _); min_port += 1; if bpf_map_update_elem(vip2tnl_map_fd, &vip as *const _ as *const _, &tnl as *const _ as *const _, BPF_NOEXIST) != 0 { perror(b"bpf_map_update_elem(&vip2tnl)\0".as_ptr() as *const i8); return 1; } }
    if bpf_xdp_attach(ifindex, prog_fd, xdp_flags, core::ptr::null_mut()) < 0 { printf(b"link set xdp fd failed\n\0".as_ptr() as *const i8); return 1; }
    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len); if err != 0 { printf(b"can't get prog info - %s\n\0".as_ptr() as *const i8, strerror(errno)); return err; }
    prog_id = info.id;
    poll_stats(kill_after_s);
    bpf_xdp_detach(ifindex, xdp_flags, core::ptr::null_mut());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
