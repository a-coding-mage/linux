// SPDX-License-Identifier: GPL-2.0
// Translated from C source. Original includes:
// getopt.h, limits.h, string.h, poll.h, sys/eventfd.h, stdlib.h, assert.h,
// unistd.h, sys/ioctl.h, sys/stat.h, sys/types.h, fcntl.h, stdbool.h,
// linux/vhost.h, linux/if.h, linux/if_tun.h, linux/in.h,
// linux/if_packet.h, linux/virtio_net.h, netinet/ether.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const TEST_BUF_LEN: usize = 256;
const TEST_PTYPE: c_int = ETH_P_LOOPBACK;
const DESC_NUM: c_int = 256;
const HDR_LEN: usize = size_of::<virtio_net_hdr_mrg_rxbuf>();

static mut __kmalloc_fake: *mut c_void = ptr::null_mut();
static mut __kfree_ignore_start: *mut c_void = ptr::null_mut();
static mut __kfree_ignore_end: *mut c_void = ptr::null_mut();

#[repr(C)]
struct vq_info {
    kick: c_int,
    call: c_int,
    idx: c_int,
    started: c_long,
    completed: c_long,
    fds: pollfd,
    ring: *mut c_void,
    /* copy used for control */
    vring: vring,
    vq: *mut virtqueue,
}

#[repr(C)]
struct vdev_info {
    vdev: virtio_device,
    control: c_int,
    vqs: [vq_info; 2],
    nvqs: c_int,
    buf: *mut c_void,
    buf_size: usize,
    test_buf: *mut c_char,
    res_buf: *mut c_char,
    mem: *mut vhost_memory,
    sock: c_int,
    ifindex: c_int,
    mac: [u8; ETHER_ADDR_LEN],
}

unsafe fn tun_alloc(dev: *mut vdev_info, tun_name: *mut c_char) -> c_int {
    let mut ifr: ifreq = core::mem::zeroed();
    let mut len: c_int = HDR_LEN as c_int;
    let fd: c_int;
    let mut e: c_int;

    fd = open(c"/dev/net/tun".as_ptr(), O_RDWR);
    if fd < 0 {
        perror(c"Cannot open /dev/net/tun".as_ptr());
        return fd;
    }

    memset(
        &mut ifr as *mut ifreq as *mut c_void,
        0,
        size_of::<ifreq>(),
    );

    ifr.ifr_ifru.ifru_flags = (IFF_TAP | IFF_NO_PI | IFF_VNET_HDR) as i16;
    strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), tun_name, IFNAMSIZ);

    e = ioctl(fd, TUNSETIFF, &mut ifr as *mut ifreq);
    if e < 0 {
        perror(c"ioctl[TUNSETIFF]".as_ptr());
        close(fd);
        return e;
    }

    e = ioctl(fd, TUNSETVNETHDRSZ, &mut len as *mut c_int);
    if e < 0 {
        perror(c"ioctl[TUNSETVNETHDRSZ]".as_ptr());
        close(fd);
        return e;
    }

    e = ioctl(fd, SIOCGIFHWADDR, &mut ifr as *mut ifreq);
    if e < 0 {
        perror(c"ioctl[SIOCGIFHWADDR]".as_ptr());
        close(fd);
        return e;
    }

    memcpy(
        (*dev).mac.as_mut_ptr() as *mut c_void,
        ifr.ifr_ifru.ifru_hwaddr.sa_data.as_ptr() as *const c_void,
        ETHER_ADDR_LEN,
    );
    fd
}

unsafe fn vdev_create_socket(dev: *mut vdev_info, tun_name: *mut c_char) {
    let mut ifr: ifreq = core::mem::zeroed();

    (*dev).sock = socket(AF_PACKET, SOCK_RAW, htons(TEST_PTYPE as u16) as c_int);
    assert!((*dev).sock != -1);

    strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), tun_name, IFNAMSIZ);
    assert!(ioctl((*dev).sock, SIOCGIFINDEX, &mut ifr as *mut ifreq) >= 0);

    (*dev).ifindex = ifr.ifr_ifru.ifru_ifindex;

    /* Set the flags that bring the device up */
    assert!(ioctl((*dev).sock, SIOCGIFFLAGS, &mut ifr as *mut ifreq) >= 0);
    ifr.ifr_ifru.ifru_flags |= (IFF_UP | IFF_RUNNING) as i16;
    assert!(ioctl((*dev).sock, SIOCSIFFLAGS, &mut ifr as *mut ifreq) >= 0);
}

unsafe fn vdev_send_packet(dev: *mut vdev_info) {
    let sendbuf = (*dev).test_buf.add(HDR_LEN);
    let mut saddrll: sockaddr_ll = core::mem::zeroed();
    let sockfd = (*dev).sock;
    let ret: c_int;

    saddrll.sll_family = PF_PACKET as u16;
    saddrll.sll_ifindex = (*dev).ifindex;
    saddrll.sll_halen = ETH_ALEN as u8;
    saddrll.sll_protocol = htons(TEST_PTYPE as u16);

    ret = sendto(
        sockfd,
        sendbuf as *const c_void,
        TEST_BUF_LEN,
        0,
        &saddrll as *const sockaddr_ll as *const sockaddr,
        size_of::<sockaddr_ll>() as socklen_t,
    ) as c_int;
    assert!(ret >= 0);
}

unsafe extern "C" fn vq_notify(vq: *mut virtqueue) -> bool {
    let info = (*vq).r#priv as *mut vq_info;
    let v: c_ulonglong = 1;
    let r: isize;

    r = write(
        (*info).kick,
        &v as *const c_ulonglong as *const c_void,
        size_of::<c_ulonglong>(),
    );
    assert!(r == size_of::<c_ulonglong>() as isize);

    true
}

unsafe fn vhost_vq_setup(dev: *mut vdev_info, info: *mut vq_info) {
    let mut addr = vhost_vring_addr {
        index: (*info).idx as c_uint,
        flags: 0,
        desc_user_addr: (*info).vring.desc as c_ulonglong,
        used_user_addr: (*info).vring.used as c_ulonglong,
        avail_user_addr: (*info).vring.avail as c_ulonglong,
        log_guest_addr: 0,
    };
    let mut state = vhost_vring_state {
        index: (*info).idx as c_uint,
        num: 0,
    };
    let mut file = vhost_vring_file {
        index: (*info).idx as c_uint,
        fd: 0,
    };
    let mut r: c_int;

    state.num = (*info).vring.num;
    r = ioctl((*dev).control, VHOST_SET_VRING_NUM, &mut state as *mut vhost_vring_state);
    assert!(r >= 0);

    state.num = 0;
    r = ioctl((*dev).control, VHOST_SET_VRING_BASE, &mut state as *mut vhost_vring_state);
    assert!(r >= 0);

    r = ioctl((*dev).control, VHOST_SET_VRING_ADDR, &mut addr as *mut vhost_vring_addr);
    assert!(r >= 0);

    file.fd = (*info).kick;
    r = ioctl((*dev).control, VHOST_SET_VRING_KICK, &mut file as *mut vhost_vring_file);
    assert!(r >= 0);
}

unsafe fn vq_reset(info: *mut vq_info, num: c_int, vdev: *mut virtio_device) {
    if !(*info).vq.is_null() {
        vring_del_virtqueue((*info).vq);
    }

    memset((*info).ring, 0, vring_size(num as c_uint, 4096));
    vring_init(&mut (*info).vring, num as c_uint, (*info).ring, 4096);
    (*info).vq = vring_new_virtqueue(
        (*info).idx as c_uint,
        num as c_uint,
        4096,
        vdev,
        true,
        false,
        (*info).ring,
        Some(vq_notify),
        ptr::null_mut(),
        c"test".as_ptr(),
    );
    assert!(!(*info).vq.is_null());
    (*(*info).vq).r#priv = info as *mut c_void;
}

unsafe fn vq_info_add(dev: *mut vdev_info, idx: c_int, num: c_int, fd: c_int) {
    let mut backend = vhost_vring_file {
        index: idx as c_uint,
        fd,
    };
    let info = &mut (*dev).vqs[idx as usize] as *mut vq_info;
    let mut r: c_int;

    (*info).idx = idx;
    (*info).kick = eventfd(0, EFD_NONBLOCK);
    r = posix_memalign(
        &mut (*info).ring as *mut *mut c_void,
        4096,
        vring_size(num as c_uint, 4096),
    );
    assert!(r >= 0);
    vq_reset(info, num, &mut (*dev).vdev as *mut virtio_device);
    vhost_vq_setup(dev, info);

    r = ioctl((*dev).control, VHOST_NET_SET_BACKEND, &mut backend as *mut vhost_vring_file);
    assert!(r == 0);
}

unsafe fn vdev_info_init(dev: *mut vdev_info, features: c_ulonglong) {
    let mut eh: *mut ether_header;
    let mut i: c_int;
    let mut r: c_int;

    (*dev).vdev.features = features;
    INIT_LIST_HEAD(&mut (*dev).vdev.vqs as *mut list_head);
    spin_lock_init(&mut (*dev).vdev.vqs_list_lock as *mut spinlock_t);

    (*dev).buf_size = (HDR_LEN + TEST_BUF_LEN) * 2;
    (*dev).buf = malloc((*dev).buf_size);
    assert!(!(*dev).buf.is_null());
    (*dev).test_buf = (*dev).buf as *mut c_char;
    (*dev).res_buf = (*dev).test_buf.add(HDR_LEN + TEST_BUF_LEN);

    memset((*dev).test_buf as *mut c_void, 0, HDR_LEN + TEST_BUF_LEN);
    eh = (*dev).test_buf.add(HDR_LEN) as *mut ether_header;
    (*eh).ether_type = htons(TEST_PTYPE as u16);
    memcpy(
        (*eh).ether_dhost.as_mut_ptr() as *mut c_void,
        (*dev).mac.as_ptr() as *const c_void,
        ETHER_ADDR_LEN,
    );
    memcpy(
        (*eh).ether_shost.as_mut_ptr() as *mut c_void,
        (*dev).mac.as_ptr() as *const c_void,
        ETHER_ADDR_LEN,
    );

    i = size_of::<ether_header>() as c_int;
    while i < TEST_BUF_LEN as c_int {
        *(*dev).test_buf.add(i as usize + HDR_LEN) = i as c_char;
        i += 1;
    }

    (*dev).control = open(c"/dev/vhost-net".as_ptr(), O_RDWR);
    assert!((*dev).control >= 0);

    r = ioctl((*dev).control, VHOST_SET_OWNER, ptr::null_mut::<c_void>());
    assert!(r >= 0);

    (*dev).mem = malloc(
        offset_of!(vhost_memory, regions) + size_of::<vhost_memory_region>(),
    ) as *mut vhost_memory;
    assert!(!(*dev).mem.is_null());
    memset(
        (*dev).mem as *mut c_void,
        0,
        offset_of!(vhost_memory, regions) + size_of::<vhost_memory_region>(),
    );
    (*(*dev).mem).nregions = 1;
    (*(*dev).mem).regions[0].guest_phys_addr = (*dev).buf as c_ulonglong;
    (*(*dev).mem).regions[0].userspace_addr = (*dev).buf as c_ulonglong;
    (*(*dev).mem).regions[0].memory_size = (*dev).buf_size as c_ulonglong;

    r = ioctl((*dev).control, VHOST_SET_MEM_TABLE, (*dev).mem);
    assert!(r >= 0);

    r = ioctl(
        (*dev).control,
        VHOST_SET_FEATURES,
        &features as *const c_ulonglong,
    );
    assert!(r >= 0);

    (*dev).nvqs = 2;
}

unsafe fn wait_for_interrupt(vq: *mut vq_info) {
    let mut val: c_ulonglong = 0;

    poll(&mut (*vq).fds as *mut pollfd, 1, 100);

    if ((*vq).fds.revents & POLLIN) != 0 {
        read(
            (*vq).fds.fd,
            &mut val as *mut c_ulonglong as *mut c_void,
            size_of::<c_ulonglong>(),
        );
    }
}

unsafe fn verify_res_buf(res_buf: *mut c_char) {
    let mut i: c_int;

    i = ETHER_HDR_LEN as c_int;
    while i < TEST_BUF_LEN as c_int {
        assert!(*res_buf.add(i as usize) == i as c_char);
        i += 1;
    }
}

unsafe fn run_tx_test(dev: *mut vdev_info, vq: *mut vq_info, delayed: bool, bufs: c_int) {
    let mut spurious: i64 = 0;
    let mut sl: scatterlist = core::mem::zeroed();
    let mut len: c_uint = 0;
    let mut r: c_int;

    loop {
        let started_before = (*vq).started;
        let completed_before = (*vq).completed;

        virtqueue_disable_cb((*vq).vq);
        loop {
            while (*vq).started < bufs as c_long && ((*vq).started - (*vq).completed) < 1 {
                sg_init_one(&mut sl as *mut scatterlist, (*dev).test_buf as *const c_void, HDR_LEN + TEST_BUF_LEN);
                r = virtqueue_add_outbuf(
                    (*vq).vq,
                    &mut sl as *mut scatterlist,
                    1,
                    (*dev).test_buf.add((*vq).started as usize) as *mut c_void,
                    GFP_ATOMIC,
                );
                if unlikely(r != 0) {
                    break;
                }

                (*vq).started += 1;

                if unlikely(!virtqueue_kick((*vq).vq)) {
                    r = -1;
                    break;
                }
            }

            if (*vq).started >= bufs as c_long {
                r = -1;
            }

            /* Flush out completed bufs if any */
            while !virtqueue_get_buf((*vq).vq, &mut len as *mut c_uint).is_null() {
                let n: c_int;

                n = recvfrom(
                    (*dev).sock,
                    (*dev).res_buf as *mut c_void,
                    TEST_BUF_LEN,
                    0,
                    ptr::null_mut(),
                    ptr::null_mut(),
                ) as c_int;
                assert!(n == TEST_BUF_LEN as c_int);
                verify_res_buf((*dev).res_buf);

                (*vq).completed += 1;
                r = 0;
            }
            if r != 0 {
                break;
            }
        }

        if (*vq).completed == completed_before && (*vq).started == started_before {
            spurious += 1;
        }

        assert!((*vq).completed <= bufs as c_long);
        assert!((*vq).started <= bufs as c_long);
        if (*vq).completed == bufs as c_long {
            break;
        }

        if delayed {
            if virtqueue_enable_cb_delayed((*vq).vq) {
                wait_for_interrupt(vq);
            }
        } else if virtqueue_enable_cb((*vq).vq) {
            wait_for_interrupt(vq);
        }
    }
    printf(
        c"TX spurious wakeups: 0x%llx started=0x%lx completed=0x%lx\n".as_ptr(),
        spurious,
        (*vq).started,
        (*vq).completed,
    );
}

unsafe fn run_rx_test(dev: *mut vdev_info, vq: *mut vq_info, delayed: bool, bufs: c_int) {
    let mut spurious: i64 = 0;
    let mut sl: scatterlist = core::mem::zeroed();
    let mut len: c_uint = 0;
    let mut r: c_int;

    loop {
        let started_before = (*vq).started;
        let completed_before = (*vq).completed;

        loop {
            while (*vq).started < bufs as c_long && ((*vq).started - (*vq).completed) < 1 {
                sg_init_one(&mut sl as *mut scatterlist, (*dev).res_buf as *const c_void, HDR_LEN + TEST_BUF_LEN);

                r = virtqueue_add_inbuf(
                    (*vq).vq,
                    &mut sl as *mut scatterlist,
                    1,
                    (*dev).res_buf.add((*vq).started as usize) as *mut c_void,
                    GFP_ATOMIC,
                );
                if unlikely(r != 0) {
                    break;
                }

                (*vq).started += 1;

                vdev_send_packet(dev);

                if unlikely(!virtqueue_kick((*vq).vq)) {
                    r = -1;
                    break;
                }
            }

            if (*vq).started >= bufs as c_long {
                r = -1;
            }

            /* Flush out completed bufs if any */
            while !virtqueue_get_buf((*vq).vq, &mut len as *mut c_uint).is_null() {
                let eh: *mut ether_header;

                eh = (*dev).res_buf.add(HDR_LEN) as *mut ether_header;

                /* tun netdev is up and running, only handle the
                 * TEST_PTYPE packet.
                 */
                if (*eh).ether_type == htons(TEST_PTYPE as u16) {
                    assert!(len == (TEST_BUF_LEN + HDR_LEN) as c_uint);
                    verify_res_buf((*dev).res_buf.add(HDR_LEN));
                }

                (*vq).completed += 1;
                r = 0;
            }
            if r != 0 {
                break;
            }
        }

        if (*vq).completed == completed_before && (*vq).started == started_before {
            spurious += 1;
        }

        assert!((*vq).completed <= bufs as c_long);
        assert!((*vq).started <= bufs as c_long);
        if (*vq).completed == bufs as c_long {
            break;
        }
    }

    printf(
        c"RX spurious wakeups: 0x%llx started=0x%lx completed=0x%lx\n".as_ptr(),
        spurious,
        (*vq).started,
        (*vq).completed,
    );
}

static optstring: &[u8] = b"h\0";
static longopts: [option; 12] = [
    option {
        name: c"help".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'h' as c_int,
    },
    option {
        name: c"event-idx".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'E' as c_int,
    },
    option {
        name: c"no-event-idx".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'e' as c_int,
    },
    option {
        name: c"indirect".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'I' as c_int,
    },
    option {
        name: c"no-indirect".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'i' as c_int,
    },
    option {
        name: c"virtio-1".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'1' as c_int,
    },
    option {
        name: c"no-virtio-1".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'0' as c_int,
    },
    option {
        name: c"delayed-interrupt".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'D' as c_int,
    },
    option {
        name: c"no-delayed-interrupt".as_ptr(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'd' as c_int,
    },
    option {
        name: c"buf-num".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: b'n' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe fn help(status: c_int) -> ! {
    fprintf(
        stderr,
        c"Usage: vhost_net_test [--help] [--no-indirect] [--no-event-idx] [--no-virtio-1] [--delayed-interrupt] [--buf-num]\n".as_ptr(),
    );

    exit(status);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut features: c_ulonglong = (1u64 << VIRTIO_RING_F_INDIRECT_DESC)
        | (1u64 << VIRTIO_RING_F_EVENT_IDX)
        | (1u64 << VIRTIO_F_VERSION_1);
    let mut tun_name: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    let mut nbufs: c_long = 0x100000;
    let mut dev: vdev_info = core::mem::zeroed();
    let mut delayed = false;
    let mut o: c_int;
    let fd: c_int;

    loop {
        o = getopt_long(
            argc,
            argv,
            optstring.as_ptr() as *const c_char,
            longopts.as_ptr(),
            ptr::null_mut(),
        );
        match o {
            -1 => break,
            x if x == b'?' as c_int => help(2),
            x if x == b'e' as c_int => {
                features &= !(1u64 << VIRTIO_RING_F_EVENT_IDX);
            }
            x if x == b'h' as c_int => help(0),
            x if x == b'i' as c_int => {
                features &= !(1u64 << VIRTIO_RING_F_INDIRECT_DESC);
            }
            x if x == b'0' as c_int => {
                features &= !(1u64 << VIRTIO_F_VERSION_1);
            }
            x if x == b'D' as c_int => {
                delayed = true;
            }
            x if x == b'n' as c_int => {
                nbufs = strtol(optarg, ptr::null_mut(), 10);
                assert!(nbufs > 0);
            }
            _ => {
                assert!(false);
            }
        }
    }

    memset(
        &mut dev as *mut vdev_info as *mut c_void,
        0,
        size_of::<vdev_info>(),
    );
    snprintf(
        tun_name.as_mut_ptr(),
        IFNAMSIZ,
        c"tun_%d".as_ptr(),
        getpid(),
    );

    fd = tun_alloc(&mut dev as *mut vdev_info, tun_name.as_mut_ptr());
    assert!(fd >= 0);

    vdev_info_init(&mut dev as *mut vdev_info, features);
    vq_info_add(&mut dev as *mut vdev_info, 0, DESC_NUM, fd);
    vq_info_add(&mut dev as *mut vdev_info, 1, DESC_NUM, fd);
    vdev_create_socket(&mut dev as *mut vdev_info, tun_name.as_mut_ptr());

    run_rx_test(
        &mut dev as *mut vdev_info,
        &mut dev.vqs[0] as *mut vq_info,
        delayed,
        nbufs as c_int,
    );
    run_tx_test(
        &mut dev as *mut vdev_info,
        &mut dev.vqs[1] as *mut vq_info,
        delayed,
        nbufs as c_int,
    );

    0
}

type socklen_t = c_uint;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe impl Sync for option {}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_ll {
    sll_family: u16,
    sll_protocol: u16,
    sll_ifindex: c_int,
    sll_hatype: u16,
    sll_pkttype: u8,
    sll_halen: u8,
    sll_addr: [u8; 8],
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __data: [u8; 126],
}

#[repr(C)]
union ifr_ifrn {
    ifrn_name: [c_char; IFNAMSIZ],
}

#[repr(C)]
union ifr_ifru {
    ifru_addr: sockaddr,
    ifru_dstaddr: sockaddr,
    ifru_broadaddr: sockaddr,
    ifru_netmask: sockaddr,
    ifru_hwaddr: sockaddr,
    ifru_flags: i16,
    ifru_ifindex: c_int,
}

#[repr(C)]
struct ifreq {
    ifr_ifrn: ifr_ifrn,
    ifr_ifru: ifr_ifru,
}

#[repr(C)]
struct ether_header {
    ether_dhost: [u8; ETHER_ADDR_LEN],
    ether_shost: [u8; ETHER_ADDR_LEN],
    ether_type: u16,
}

#[repr(C)]
struct virtio_net_hdr_mrg_rxbuf {
    hdr: virtio_net_hdr,
    num_buffers: u16,
}

#[repr(C)]
struct virtio_net_hdr {
    flags: u8,
    gso_type: u8,
    hdr_len: u16,
    gso_size: u16,
    csum_start: u16,
    csum_offset: u16,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct virtio_device {
    features: c_ulonglong,
    vqs: list_head,
    vqs_list_lock: spinlock_t,
}

#[repr(C)]
struct virtqueue {
    r#priv: *mut c_void,
}

#[repr(C)]
struct vring {
    num: c_uint,
    desc: *mut c_void,
    avail: *mut c_void,
    used: *mut c_void,
}

#[repr(C)]
struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
struct vhost_vring_addr {
    index: c_uint,
    flags: c_uint,
    desc_user_addr: c_ulonglong,
    used_user_addr: c_ulonglong,
    avail_user_addr: c_ulonglong,
    log_guest_addr: c_ulonglong,
}

#[repr(C)]
struct vhost_vring_state {
    index: c_uint,
    num: c_uint,
}

#[repr(C)]
struct vhost_vring_file {
    index: c_uint,
    fd: c_int,
}

#[repr(C)]
struct vhost_memory_region {
    guest_phys_addr: c_ulonglong,
    memory_size: c_ulonglong,
    userspace_addr: c_ulonglong,
    flags_padding: c_ulonglong,
}

#[repr(C)]
struct vhost_memory {
    nregions: c_uint,
    padding: c_uint,
    regions: [vhost_memory_region; 1],
}

extern "C" {
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn vring_del_virtqueue(vq: *mut virtqueue);
    fn vring_size(num: c_uint, align: c_ulong) -> usize;
    fn vring_init(vr: *mut vring, num: c_uint, p: *mut c_void, align: c_ulong);
    fn vring_new_virtqueue(
        index: c_uint,
        num: c_uint,
        vring_align: c_uint,
        vdev: *mut virtio_device,
        weak_barriers: bool,
        context: bool,
        pages: *mut c_void,
        notify: Option<unsafe extern "C" fn(*mut virtqueue) -> bool>,
        callback: *mut c_void,
        name: *const c_char,
    ) -> *mut virtqueue;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: c_uint);
    fn virtqueue_disable_cb(vq: *mut virtqueue);
    fn virtqueue_add_outbuf(
        vq: *mut virtqueue,
        sg: *mut scatterlist,
        num: c_uint,
        data: *mut c_void,
        gfp: c_int,
    ) -> c_int;
    fn virtqueue_add_inbuf(
        vq: *mut virtqueue,
        sg: *mut scatterlist,
        num: c_uint,
        data: *mut c_void,
        gfp: c_int,
    ) -> c_int;
    fn virtqueue_kick(vq: *mut virtqueue) -> bool;
    fn virtqueue_get_buf(vq: *mut virtqueue, len: *mut c_uint) -> *mut c_void;
    fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: usize,
        flags: c_int,
        src_addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> isize;
    fn virtqueue_enable_cb_delayed(vq: *mut virtqueue) -> bool;
    fn virtqueue_enable_cb(vq: *mut virtqueue) -> bool;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
}

fn unlikely(x: bool) -> bool {
    x
}

const O_RDWR: c_int = 0o2;
const EFD_NONBLOCK: c_int = 0o4000;
const AF_PACKET: c_int = 17;
const PF_PACKET: c_int = AF_PACKET;
const SOCK_RAW: c_int = 3;
const POLLIN: i16 = 0x0001;
const IFNAMSIZ: usize = 16;
const IFF_UP: c_int = 0x1;
const IFF_RUNNING: c_int = 0x40;
const IFF_TAP: c_int = 0x0002;
const IFF_NO_PI: c_int = 0x1000;
const IFF_VNET_HDR: c_int = 0x4000;
const ETH_ALEN: usize = 6;
const ETHER_ADDR_LEN: usize = 6;
const ETHER_HDR_LEN: usize = 14;
const ETH_P_LOOPBACK: c_int = 0x0060;
const GFP_ATOMIC: c_int = 0;
const required_argument: c_int = 1;

const VIRTIO_RING_F_INDIRECT_DESC: u64 = 28;
const VIRTIO_RING_F_EVENT_IDX: u64 = 29;
const VIRTIO_F_VERSION_1: u64 = 32;

const TUNSETIFF: c_ulong = 0x400454ca;
const TUNSETVNETHDRSZ: c_ulong = 0x400454d8;
const SIOCGIFHWADDR: c_ulong = 0x8927;
const SIOCGIFINDEX: c_ulong = 0x8933;
const SIOCGIFFLAGS: c_ulong = 0x8913;
const SIOCSIFFLAGS: c_ulong = 0x8914;
const VHOST_SET_OWNER: c_ulong = 0xaf01;
const VHOST_SET_MEM_TABLE: c_ulong = 0x4008af03;
const VHOST_SET_FEATURES: c_ulong = 0x4008af00;
const VHOST_SET_VRING_NUM: c_ulong = 0x4008af10;
const VHOST_SET_VRING_BASE: c_ulong = 0x4008af12;
const VHOST_SET_VRING_ADDR: c_ulong = 0x4028af11;
const VHOST_SET_VRING_KICK: c_ulong = 0x4008af20;
const VHOST_NET_SET_BACKEND: c_ulong = 0x4008af30;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
