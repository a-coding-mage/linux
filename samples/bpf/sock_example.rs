/* eBPF example program:
 * - creates arraymap in kernel with key 4 bytes and value 8 bytes
 *
 * - loads eBPF program:
 *   r0 = skb->data[ETH_HLEN + offsetof(struct iphdr, protocol)];
 *   *(u32*)(fp - 4) = r0;
 *   // assuming packet is IPv4, lookup ip->proto in a map
 *   value = bpf_map_lookup_elem(map_fd, fp - 4);
 *   if (value)
 *        (*(u64*)value) += 1;
 *
 * - attaches this program to loopback interface "lo" raw socket
 *
 * - every second user space reads map[tcp], map[udp], map[icmp] to see
 *   how many packets of given protocol were seen on "lo"
 */

// C headers and project headers provide the constants, types, macros, and
// external functions referenced below.

static mut BPF_LOG_BUF: [u8; BPF_LOG_BUF_SIZE] = [0; BPF_LOG_BUF_SIZE];

unsafe fn test_sock() -> i32 {
    let mut sock: i32 = -1;
    let mut map_fd: i32;
    let mut prog_fd: i32;
    let mut i: i32;
    let mut key: i32;
    let mut value: i64 = 0;
    let mut tcp_cnt: i64 = 0;
    let mut udp_cnt: i64 = 0;
    let mut icmp_cnt: i64 = 0;

    map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null(),
                            core::mem::size_of::<i32>(),
                            core::mem::size_of::<i64>(), 256,
                            core::ptr::null());
    if map_fd < 0 {
        printf(b"failed to create map '%s'\0".as_ptr(), strerror(errno));
        return 0;
    }

    let prog = [
        BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),
        BPF_LD_ABS(BPF_B, ETH_HLEN + core::mem::offset_of!(iphdr, protocol)),
        BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4),
        BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -4),
        BPF_LD_MAP_FD(BPF_REG_1, map_fd),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
        BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 2),
        BPF_MOV64_IMM(BPF_REG_1, 1),
        BPF_ATOMIC_OP(BPF_DW, BPF_ADD, BPF_REG_0, BPF_REG_1, 0),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let insns_cnt = prog.len();
    let mut opts = bpf_prog_load_opts {
        log_buf: BPF_LOG_BUF.as_mut_ptr(),
        log_size: BPF_LOG_BUF_SIZE,
        ..core::mem::zeroed()
    };

    prog_fd = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, core::ptr::null(),
                            b"GPL\0".as_ptr() as *const _, prog.as_ptr(),
                            insns_cnt, &mut opts);
    if prog_fd < 0 {
        printf(b"failed to load prog '%s'\0".as_ptr(), strerror(errno));
        return 0;
    }

    sock = open_raw_sock(b"lo\0".as_ptr());
    if setsockopt(sock, SOL_SOCKET, SO_ATTACH_BPF, &prog_fd as *const _ as *const _,
                  core::mem::size_of_val(&prog_fd)) < 0 {
        printf(b"setsockopt %s\n\0".as_ptr(), strerror(errno));
        return 0;
    }

    i = 0;
    while i < 10 {
        key = IPPROTO_TCP;
        assert(bpf_map_lookup_elem(map_fd, &key, &mut tcp_cnt) == 0);
        key = IPPROTO_UDP;
        assert(bpf_map_lookup_elem(map_fd, &key, &mut udp_cnt) == 0);
        key = IPPROTO_ICMP;
        assert(bpf_map_lookup_elem(map_fd, &key, &mut icmp_cnt) == 0);
        printf(b"TCP %lld UDP %lld ICMP %lld packets\n\0".as_ptr(),
               tcp_cnt, udp_cnt, icmp_cnt);
        sleep(1);
        i += 1;
    }

    // maps, programs, raw sockets will auto cleanup on process exit
    let _ = (sock, value);
    0
}

fn main() {
    unsafe {
        let f = popen(b"ping -4 -c5 localhost\0".as_ptr(), b"r\0".as_ptr());
        let _ = f;
        let _ = test_sock();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
