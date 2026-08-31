// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Samsung Electronics
 * Bongsu Jeon <bongsu.jeon@samsung.com>
 *
 * Test code for nci
 */

// C dependencies translated from includes:
// stdlib.h, errno.h, string.h, sys/ioctl.h, fcntl.h, pthread.h,
// linux/genetlink.h, sys/socket.h, linux/nfc.h, kselftest_harness.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;

const MAX_MSG_SIZE: usize = 1024;

const IOCTL_GET_NCIDEV_IDX: c_int = 0;
const VIRTUAL_NFC_PROTOCOLS: c_int = (NFC_PROTO_JEWEL_MASK
    | NFC_PROTO_MIFARE_MASK
    | NFC_PROTO_FELICA_MASK
    | NFC_PROTO_ISO14443_MASK
    | NFC_PROTO_ISO14443_B_MASK
    | NFC_PROTO_ISO15693_MASK) as c_int;

const nci_reset_cmd: [__u8; 4] = [0x20, 0x00, 0x01, 0x01];
const nci_init_cmd: [__u8; 3] = [0x20, 0x01, 0x00];
const nci_rf_discovery_cmd: [__u8; 12] = [
    0x21, 0x03, 0x09, 0x04, 0x00, 0x01, 0x01, 0x01, 0x02, 0x01, 0x06, 0x01,
];
const nci_init_cmd_v2: [__u8; 5] = [0x20, 0x01, 0x02, 0x00, 0x00];
const nci_rf_disc_map_cmd: [__u8; 10] = [
    0x21, 0x00, 0x07, 0x02, 0x04, 0x03, 0x02, 0x05, 0x03, 0x03,
];
const nci_rf_deact_cmd: [__u8; 4] = [0x21, 0x06, 0x01, 0x00];
const nci_reset_rsp: [__u8; 6] = [0x40, 0x00, 0x03, 0x00, 0x10, 0x01];
const nci_reset_rsp_v2: [__u8; 4] = [0x40, 0x00, 0x01, 0x00];
const nci_reset_ntf: [__u8; 12] = [
    0x60, 0x00, 0x09, 0x02, 0x01, 0x20, 0x0e, 0x04, 0x61, 0x00, 0x04, 0x02,
];
const nci_init_rsp: [__u8; 23] = [
    0x40, 0x01, 0x14, 0x00, 0x02, 0x0e, 0x02, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x02, 0xc8, 0x00, 0xff, 0x10, 0x00, 0x0e, 0x12, 0x00, 0x00, 0x04,
];
const nci_init_rsp_v2: [__u8; 31] = [
    0x40, 0x01, 0x1c, 0x00, 0x1a, 0x7e, 0x06, 0x00, 0x02, 0x92, 0x04, 0xff,
    0xff, 0x01, 0x00, 0x40, 0x06, 0x00, 0x00, 0x01, 0x01, 0x00, 0x02, 0x00,
    0x03, 0x01, 0x01, 0x06, 0x00, 0x80, 0x00,
];
const nci_rf_disc_map_rsp: [__u8; 4] = [0x41, 0x00, 0x01, 0x00];
const nci_rf_disc_rsp: [__u8; 4] = [0x41, 0x03, 0x01, 0x00];
const nci_rf_deact_rsp: [__u8; 4] = [0x41, 0x06, 0x01, 0x00];
const nci_rf_deact_ntf: [__u8; 5] = [0x61, 0x06, 0x02, 0x00, 0x00];
const nci_rf_activate_ntf: [__u8; 32] = [
    0x61, 0x05, 0x1D, 0x01, 0x02, 0x04, 0x00, 0xFF, 0xFF, 0x0C, 0x44, 0x03,
    0x07, 0x04, 0x62, 0x26, 0x11, 0x80, 0x1D, 0x80, 0x01, 0x20, 0x00, 0x00,
    0x00, 0x06, 0x05, 0x75, 0x77, 0x81, 0x02, 0x80,
];
const nci_t4t_select_cmd: [__u8; 15] = [
    0x00, 0x00, 0x0C, 0x00, 0xA4, 0x04, 0x00, 0x07, 0xD2, 0x76, 0x00, 0x00,
    0x85, 0x01, 0x01,
];
const nci_t4t_select_cmd2: [__u8; 10] = [
    0x00, 0x00, 0x07, 0x00, 0xA4, 0x00, 0x0C, 0x02, 0xE1, 0x03,
];
const nci_t4t_select_cmd3: [__u8; 10] = [
    0x00, 0x00, 0x07, 0x00, 0xA4, 0x00, 0x0C, 0x02, 0xE1, 0x04,
];
const nci_t4t_read_cmd: [__u8; 8] = [0x00, 0x00, 0x05, 0x00, 0xB0, 0x00, 0x00, 0x0F];
const nci_t4t_read_rsp: [__u8; 20] = [
    0x00, 0x00, 0x11, 0x00, 0x0F, 0x20, 0x00, 0x3B, 0x00, 0x34, 0x04, 0x06,
    0xE1, 0x04, 0x08, 0x00, 0x00, 0x00, 0x90, 0x00,
];
const nci_t4t_read_cmd2: [__u8; 8] = [0x00, 0x00, 0x05, 0x00, 0xB0, 0x00, 0x00, 0x02];
const nci_t4t_read_rsp2: [__u8; 7] = [0x00, 0x00, 0x04, 0x00, 0x0F, 0x90, 0x00];
const nci_t4t_read_cmd3: [__u8; 8] = [0x00, 0x00, 0x05, 0x00, 0xB0, 0x00, 0x02, 0x0F];
const nci_t4t_read_rsp3: [__u8; 20] = [
    0x00, 0x00, 0x11, 0xD1, 0x01, 0x0B, 0x54, 0x02, 0x65, 0x6E, 0x4E, 0x46,
    0x43, 0x20, 0x54, 0x45, 0x53, 0x54, 0x90, 0x00,
];
const nci_t4t_rsp_ok: [__u8; 5] = [0x00, 0x00, 0x02, 0x90, 0x00];

#[repr(C)]
struct msgtemplate {
    n: nlmsghdr,
    g: genlmsghdr,
    buf: [c_char; MAX_MSG_SIZE],
}

unsafe fn create_nl_socket() -> c_int {
    let fd: c_int;
    let mut local: sockaddr_nl = core::mem::zeroed();

    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if fd < 0 {
        return -1;
    }

    memset(
        &mut local as *mut sockaddr_nl as *mut c_void,
        0,
        size_of::<sockaddr_nl>(),
    );
    local.nl_family = AF_NETLINK as _;

    if bind(
        fd,
        &mut local as *mut sockaddr_nl as *mut sockaddr,
        size_of::<sockaddr_nl>() as _,
    ) < 0
    {
        close(fd);
        return -1;
    }

    fd
}

unsafe fn send_cmd_mt_nla(
    sd: c_int,
    nlmsg_type: __u16,
    nlmsg_pid: __u32,
    genl_cmd: __u8,
    nla_num: c_int,
    nla_type: *mut __u16,
    nla_data: *mut *mut c_void,
    nla_len: *mut c_int,
    flags: __u16,
) -> c_int {
    let mut nladdr: sockaddr_nl = core::mem::zeroed();
    let mut msg: msgtemplate = core::mem::zeroed();
    let mut na: *mut nlattr;
    let mut cnt: c_int;
    let mut prv_len: c_int;
    let mut r: c_int;
    let mut buflen: c_int;
    let mut buf: *mut c_char;

    msg.n.nlmsg_len = NLMSG_LENGTH(GENL_HDRLEN as _) as _;
    msg.n.nlmsg_type = nlmsg_type;
    msg.n.nlmsg_flags = flags;
    msg.n.nlmsg_seq = 0;
    msg.n.nlmsg_pid = nlmsg_pid;
    msg.g.cmd = genl_cmd;
    msg.g.version = 0x1;

    prv_len = 0;
    cnt = 0;
    while cnt < nla_num {
        na = (GENLMSG_DATA(&mut msg.n) as *mut c_char).add(prv_len as usize) as *mut nlattr;
        (*na).nla_type = *nla_type.add(cnt as usize);
        (*na).nla_len = (*nla_len.add(cnt as usize) + NLA_HDRLEN as c_int) as _;

        if *nla_len.add(cnt as usize) > 0 {
            memcpy(
                NLA_DATA(na),
                *nla_data.add(cnt as usize),
                *nla_len.add(cnt as usize) as _,
            );
        }

        prv_len = NLA_ALIGN(*nla_len.add(cnt as usize) as _) as c_int + NLA_HDRLEN as c_int;
        msg.n.nlmsg_len += prv_len as __u32;
        cnt += 1;
    }

    buf = &mut msg as *mut msgtemplate as *mut c_char;
    buflen = msg.n.nlmsg_len as c_int;
    memset(
        &mut nladdr as *mut sockaddr_nl as *mut c_void,
        0,
        size_of::<sockaddr_nl>(),
    );
    nladdr.nl_family = AF_NETLINK as _;

    loop {
        r = sendto(
            sd,
            buf as *const c_void,
            buflen as _,
            0,
            &mut nladdr as *mut sockaddr_nl as *mut sockaddr,
            size_of::<sockaddr_nl>() as _,
        ) as c_int;
        if !(r < buflen) {
            break;
        }
        if r > 0 {
            buf = buf.add(r as usize);
            buflen -= r;
        } else if *__errno_location() != EAGAIN {
            return -1;
        }
    }
    0
}

unsafe fn send_get_nfc_family(sd: c_int, pid: __u32) -> c_int {
    let mut nla_get_family_type: __u16 = CTRL_ATTR_FAMILY_NAME as _;
    let mut nla_get_family_data: *mut c_void;
    let mut nla_get_family_len: c_int;
    let mut family_name: [c_char; 100] = [0; 100];

    nla_get_family_len = strlen(NFC_GENL_NAME.as_ptr()) as c_int + 1;
    strcpy(family_name.as_mut_ptr(), NFC_GENL_NAME.as_ptr());
    nla_get_family_data = family_name.as_mut_ptr() as *mut c_void;

    send_cmd_mt_nla(
        sd,
        GENL_ID_CTRL as _,
        pid,
        CTRL_CMD_GETFAMILY as _,
        1,
        &mut nla_get_family_type,
        &mut nla_get_family_data,
        &mut nla_get_family_len,
        NLM_F_REQUEST as _,
    )
}

unsafe fn get_family_id(sd: c_int, pid: __u32, event_group: *mut __u32) -> c_int {
    #[repr(C)]
    struct Ans {
        n: nlmsghdr,
        g: genlmsghdr,
        buf: [c_char; 512],
    }

    let mut ans: Ans = core::mem::zeroed();
    let mut na: *mut nlattr;
    let mut resp_len: c_int;
    let mut id: __u16 = 0;
    let mut len: c_int;
    let mut rc: c_int;

    rc = send_get_nfc_family(sd, pid);
    if rc < 0 {
        return 0;
    }

    resp_len = recv(sd, &mut ans as *mut Ans as *mut c_void, size_of::<Ans>(), 0) as c_int;

    if ans.n.nlmsg_type == NLMSG_ERROR as __u16 || resp_len < 0 || !NLMSG_OK(&mut ans.n, resp_len) {
        return 0;
    }

    len = 0;
    resp_len = GENLMSG_PAYLOAD(&mut ans.n) as c_int;
    na = GENLMSG_DATA(&mut ans.n) as *mut nlattr;

    while len < resp_len {
        len += NLA_ALIGN((*na).nla_len as _) as c_int;
        if (*na).nla_type == CTRL_ATTR_FAMILY_ID as __u16 {
            id = *(NLA_DATA(na) as *mut __u16);
        } else if (*na).nla_type == CTRL_ATTR_MCAST_GROUPS as __u16 {
            let mut nested_na: *mut nlattr;
            let mut group_na: *mut nlattr;
            let mut group_attr_len: c_int;
            let mut group_attr: c_int;

            nested_na = (na as *mut c_char).add(NLA_HDRLEN as usize) as *mut nlattr;
            group_na = (nested_na as *mut c_char).add(NLA_HDRLEN as usize) as *mut nlattr;
            group_attr_len = 0;

            group_attr = CTRL_ATTR_MCAST_GRP_UNSPEC as c_int;
            while group_attr < CTRL_ATTR_MCAST_GRP_MAX as c_int {
                if (*group_na).nla_type == CTRL_ATTR_MCAST_GRP_ID as __u16 {
                    *event_group = *((group_na as *mut c_char).add(NLA_HDRLEN as usize) as *mut __u32);
                    break;
                }

                group_attr_len += NLA_ALIGN((*group_na).nla_len as _) as c_int + NLA_HDRLEN as c_int;
                if group_attr_len >= (*nested_na).nla_len as c_int {
                    break;
                }

                group_na = (group_na as *mut c_char)
                    .add(NLA_ALIGN((*group_na).nla_len as _) as usize)
                    as *mut nlattr;
                group_attr += 1;
            }
        }
        na = (GENLMSG_DATA(&mut ans.n) as *mut c_char).add(len as usize) as *mut nlattr;
    }
    id as c_int
}

unsafe fn send_cmd_with_idx(
    sd: c_int,
    nlmsg_type: __u16,
    nlmsg_pid: __u32,
    genl_cmd: __u8,
    dev_id: c_int,
) -> c_int {
    let mut nla_type: __u16 = NFC_ATTR_DEVICE_INDEX as _;
    let mut dev_id_mut = dev_id;
    let mut nla_data: *mut c_void = &mut dev_id_mut as *mut c_int as *mut c_void;
    let mut nla_len: c_int = 4;

    send_cmd_mt_nla(
        sd,
        nlmsg_type,
        nlmsg_pid,
        genl_cmd,
        1,
        &mut nla_type,
        &mut nla_data,
        &mut nla_len,
        NLM_F_REQUEST as _,
    )
}

unsafe fn get_nci_devid(
    sd: c_int,
    fid: __u16,
    pid: __u32,
    dev_id: c_int,
    msg: *mut msgtemplate,
) -> c_int {
    let mut rc: c_int;
    let resp_len: c_int;

    rc = send_cmd_with_idx(sd, fid, pid, NFC_CMD_GET_DEVICE as _, dev_id);
    if rc < 0 {
        rc = -1;
        return rc;
    }

    resp_len = recv(sd, msg as *mut c_void, size_of::<msgtemplate>(), 0) as c_int;
    if resp_len < 0 {
        rc = -2;
        return rc;
    }

    if (*msg).n.nlmsg_type == NLMSG_ERROR as __u16 || !NLMSG_OK(&mut (*msg).n, resp_len) {
        rc = -3;
        return rc;
    }

    0
}

unsafe fn get_dev_enable_state(msg: *mut msgtemplate) -> __u8 {
    let mut na: *mut nlattr;
    let resp_len: c_int;
    let mut len: c_int;

    resp_len = GENLMSG_PAYLOAD(&mut (*msg).n) as c_int;
    na = GENLMSG_DATA(&mut (*msg).n) as *mut nlattr;
    len = 0;

    while len < resp_len {
        len += NLA_ALIGN((*na).nla_len as _) as c_int;
        if (*na).nla_type == NFC_ATTR_DEVICE_POWERED as __u16 {
            return *(NLA_DATA(na) as *mut c_char) as __u8;
        }
        na = (GENLMSG_DATA(&mut (*msg).n) as *mut c_char).add(len as usize) as *mut nlattr;
    }

    resp_len as __u8
}

#[repr(C)]
struct NCI {
    virtual_nci_fd: c_int,
    open_state: bool,
    dev_idex: c_int,
    isNCI2: bool,
    proto: c_int,
    pid: __u32,
    fid: __u16,
    sd: c_int,
}

#[repr(C)]
struct NCI_variant {
    isNCI2: bool,
}

const NCI1_0: NCI_variant = NCI_variant { isNCI2: false };
const NCI2_0: NCI_variant = NCI_variant { isNCI2: true };

unsafe extern "C" fn virtual_dev_open(data: *mut c_void) -> *mut c_void {
    let mut buf: [c_char; 258] = [0; 258];
    let dev_fd: c_int;
    let mut len: c_int;

    dev_fd = *(data as *mut c_int);

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_reset_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_reset_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_reset_rsp.as_ptr() as *const c_void, size_of_val(&nci_reset_rsp));

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_init_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_init_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_init_rsp.as_ptr() as *const c_void, size_of_val(&nci_init_rsp));

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_rf_disc_map_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_rf_disc_map_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_rf_disc_map_rsp.as_ptr() as *const c_void, size_of_val(&nci_rf_disc_map_rsp));

    ptr::null_mut()
}

unsafe extern "C" fn virtual_dev_open_v2(data: *mut c_void) -> *mut c_void {
    let mut buf: [c_char; 258] = [0; 258];
    let dev_fd: c_int;
    let mut len: c_int;

    dev_fd = *(data as *mut c_int);

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_reset_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_reset_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_reset_rsp_v2.as_ptr() as *const c_void, size_of_val(&nci_reset_rsp_v2));
    write(dev_fd, nci_reset_ntf.as_ptr() as *const c_void, size_of_val(&nci_reset_ntf));

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_init_cmd_v2) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_init_cmd_v2.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_init_rsp_v2.as_ptr() as *const c_void, size_of_val(&nci_init_rsp_v2));

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_rf_disc_map_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_rf_disc_map_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_rf_disc_map_rsp.as_ptr() as *const c_void, size_of_val(&nci_rf_disc_map_rsp));

    ptr::null_mut()
}

fixture_setup!(NCI, |self_: *mut NCI, variant: *const NCI_variant| unsafe {
    let mut msg: msgtemplate = core::mem::zeroed();
    let mut thread_t: pthread_t = core::mem::zeroed();
    let mut event_group: __u32 = 0;
    let mut status: c_int = 0;
    let mut rc: c_int;

    (*self_).open_state = false;
    (*self_).proto = VIRTUAL_NFC_PROTOCOLS;
    (*self_).isNCI2 = (*variant).isNCI2;

    (*self_).sd = create_nl_socket();
    ASSERT_NE!((*self_).sd, -1);

    (*self_).pid = getpid() as __u32;
    (*self_).fid = get_family_id((*self_).sd, (*self_).pid, &mut event_group) as __u16;
    ASSERT_NE!((*self_).fid, -1i32 as __u16);

    (*self_).virtual_nci_fd = open(c"/dev/virtual_nci".as_ptr(), O_RDWR);
    ASSERT_GT!((*self_).virtual_nci_fd, -1);

    rc = setsockopt(
        (*self_).sd,
        SOL_NETLINK,
        NETLINK_ADD_MEMBERSHIP,
        &mut event_group as *mut __u32 as *const c_void,
        size_of::<__u32>() as _,
    );
    ASSERT_NE!(rc, -1);

    rc = ioctl(
        (*self_).virtual_nci_fd,
        IOCTL_GET_NCIDEV_IDX as _,
        &mut (*self_).dev_idex,
    );
    ASSERT_EQ!(rc, 0);

    rc = get_nci_devid((*self_).sd, (*self_).fid, (*self_).pid, (*self_).dev_idex, &mut msg);
    ASSERT_EQ!(rc, 0);
    EXPECT_EQ!(get_dev_enable_state(&mut msg), 0);

    if (*self_).isNCI2 {
        rc = pthread_create(
            &mut thread_t,
            ptr::null(),
            Some(virtual_dev_open_v2),
            &mut (*self_).virtual_nci_fd as *mut c_int as *mut c_void,
        );
    } else {
        rc = pthread_create(
            &mut thread_t,
            ptr::null(),
            Some(virtual_dev_open),
            &mut (*self_).virtual_nci_fd as *mut c_int as *mut c_void,
        );
    }
    ASSERT_GT!(rc, -1);

    rc = send_cmd_with_idx(
        (*self_).sd,
        (*self_).fid,
        (*self_).pid,
        NFC_CMD_DEV_UP as _,
        (*self_).dev_idex,
    );
    EXPECT_EQ!(rc, 0);

    pthread_join(thread_t, &mut status as *mut c_int as *mut *mut c_void);
    ASSERT_EQ!(status, 0);
    (*self_).open_state = true;
});

unsafe extern "C" fn virtual_deinit(data: *mut c_void) -> *mut c_void {
    let mut buf: [c_char; 258] = [0; 258];
    let dev_fd: c_int;
    let mut len: c_int;

    dev_fd = *(data as *mut c_int);

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_reset_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_reset_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_reset_rsp.as_ptr() as *const c_void, size_of_val(&nci_reset_rsp));

    ptr::null_mut()
}

unsafe extern "C" fn virtual_deinit_v2(data: *mut c_void) -> *mut c_void {
    let mut buf: [c_char; 258] = [0; 258];
    let dev_fd: c_int;
    let mut len: c_int;

    dev_fd = *(data as *mut c_int);

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_reset_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_reset_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_reset_rsp_v2.as_ptr() as *const c_void, size_of_val(&nci_reset_rsp_v2));
    write(dev_fd, nci_reset_ntf.as_ptr() as *const c_void, size_of_val(&nci_reset_ntf));

    ptr::null_mut()
}

fixture_teardown!(NCI, |self_: *mut NCI| unsafe {
    let mut thread_t: pthread_t = core::mem::zeroed();
    let mut status: c_int = 0;
    let mut rc: c_int;

    if (*self_).open_state {
        if (*self_).isNCI2 {
            rc = pthread_create(
                &mut thread_t,
                ptr::null(),
                Some(virtual_deinit_v2),
                &mut (*self_).virtual_nci_fd as *mut c_int as *mut c_void,
            );
        } else {
            rc = pthread_create(
                &mut thread_t,
                ptr::null(),
                Some(virtual_deinit),
                &mut (*self_).virtual_nci_fd as *mut c_int as *mut c_void,
            );
        }

        ASSERT_GT!(rc, -1);
        rc = send_cmd_with_idx(
            (*self_).sd,
            (*self_).fid,
            (*self_).pid,
            NFC_CMD_DEV_DOWN as _,
            (*self_).dev_idex,
        );
        EXPECT_EQ!(rc, 0);

        pthread_join(thread_t, &mut status as *mut c_int as *mut *mut c_void);
        ASSERT_EQ!(status, 0);
    }

    close((*self_).sd);
    close((*self_).virtual_nci_fd);
    (*self_).open_state = false;
});

test_f!(NCI, init, |self_: *mut NCI| unsafe {
    let mut msg: msgtemplate = core::mem::zeroed();
    let rc: c_int;

    rc = get_nci_devid((*self_).sd, (*self_).fid, (*self_).pid, (*self_).dev_idex, &mut msg);
    ASSERT_EQ!(rc, 0);
    EXPECT_EQ!(get_dev_enable_state(&mut msg), 1);
});

unsafe extern "C" fn virtual_poll_start(data: *mut c_void) -> *mut c_void {
    let mut buf: [c_char; 258] = [0; 258];
    let dev_fd: c_int;
    let mut len: c_int;

    dev_fd = *(data as *mut c_int);

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_rf_discovery_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_rf_discovery_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_rf_disc_rsp.as_ptr() as *const c_void, size_of_val(&nci_rf_disc_rsp));

    ptr::null_mut()
}

unsafe extern "C" fn virtual_poll_stop(data: *mut c_void) -> *mut c_void {
    let mut buf: [c_char; 258] = [0; 258];
    let dev_fd: c_int;
    let mut len: c_int;

    dev_fd = *(data as *mut c_int);

    len = read(dev_fd, buf.as_mut_ptr() as *mut c_void, 258) as c_int;
    if len <= 0 {
        return (-1isize) as *mut c_void;
    }
    if len as usize != size_of_val(&nci_rf_deact_cmd) {
        return (-1isize) as *mut c_void;
    }
    if memcmp(nci_rf_deact_cmd.as_ptr() as *const c_void, buf.as_ptr() as *const c_void, len as _) != 0 {
        return (-1isize) as *mut c_void;
    }
    write(dev_fd, nci_rf_deact_rsp.as_ptr() as *const c_void, size_of_val(&nci_rf_deact_rsp));

    ptr::null_mut()
}

unsafe fn start_polling(
    dev_idx: c_int,
    proto: c_int,
    virtual_fd: c_int,
    sd: c_int,
    fid: c_int,
    pid: c_int,
) -> c_int {
    let mut nla_start_poll_type: [__u16; 2] = [NFC_ATTR_DEVICE_INDEX as _, NFC_ATTR_PROTOCOLS as _];
    let mut dev_idx_mut = dev_idx;
    let mut proto_mut = proto;
    let mut nla_start_poll_data: [*mut c_void; 2] = [
        &mut dev_idx_mut as *mut c_int as *mut c_void,
        &mut proto_mut as *mut c_int as *mut c_void,
    ];
    let mut nla_start_poll_len: [c_int; 2] = [4, 4];
    let mut thread_t: pthread_t = core::mem::zeroed();
    let mut status: c_int = 0;
    let mut rc: c_int;
    let mut virtual_fd_mut = virtual_fd;

    rc = pthread_create(
        &mut thread_t,
        ptr::null(),
        Some(virtual_poll_start),
        &mut virtual_fd_mut as *mut c_int as *mut c_void,
    );
    if rc < 0 {
        return rc;
    }

    rc = send_cmd_mt_nla(
        sd,
        fid as __u16,
        pid as __u32,
        NFC_CMD_START_POLL as _,
        2,
        nla_start_poll_type.as_mut_ptr(),
        nla_start_poll_data.as_mut_ptr(),
        nla_start_poll_len.as_mut_ptr(),
        NLM_F_REQUEST as _,
    );
    if rc != 0 {
        return rc;
    }

    pthread_join(thread_t, &mut status as *mut c_int as *mut *mut c_void);
    status
}

unsafe fn stop_polling(dev_idx: c_int, virtual_fd: c_int, sd: c_int, fid: c_int, pid: c_int) -> c_int {
    let mut thread_t: pthread_t = core::mem::zeroed();
    let mut status: c_int = 0;
    let mut rc: c_int;
    let mut virtual_fd_mut = virtual_fd;

    rc = pthread_create(
        &mut thread_t,
        ptr::null(),
        Some(virtual_poll_stop),
        &mut virtual_fd_mut as *mut c_int as *mut c_void,
    );
    if rc < 0 {
        return rc;
    }

    rc = send_cmd_with_idx(sd, fid as __u16, pid as __u32, NFC_CMD_STOP_POLL as _, dev_idx);
    if rc != 0 {
        return rc;
    }

    pthread_join(thread_t, &mut status as *mut c_int as *mut *mut c_void);
    status
}

test_f!(NCI, start_poll, |self_: *mut NCI| unsafe {
    let mut status: c_int;

    status = start_polling(
        (*self_).dev_idex,
        (*self_).proto,
        (*self_).virtual_nci_fd,
        (*self_).sd,
        (*self_).fid as c_int,
        (*self_).pid as c_int,
    );
    EXPECT_EQ!(status, 0);

    status = stop_polling(
        (*self_).dev_idex,
        (*self_).virtual_nci_fd,
        (*self_).sd,
        (*self_).fid as c_int,
        (*self_).pid as c_int,
    );
    EXPECT_EQ!(status, 0);
});

unsafe fn get_taginfo(dev_idx: c_int, sd: c_int, fid: c_int, pid: c_int) -> c_int {
    #[repr(C)]
    struct Ans {
        n: nlmsghdr,
        g: genlmsghdr,
        buf: [c_char; 512],
    }

    let mut ans: Ans = core::mem::zeroed();
    let mut na: *mut nlattr;
    let mut protocol: __u32;
    let mut targetidx: c_int;
    let mut sel_res: __u8;
    let mut resp_len: c_int;
    let mut len: c_int;

    let mut tagid_type: __u16;
    let mut tagid_type_data: *mut c_void;
    let mut tagid_len: c_int;
    let mut dev_idx_mut = dev_idx;

    tagid_type = NFC_ATTR_DEVICE_INDEX as _;
    tagid_type_data = &mut dev_idx_mut as *mut c_int as *mut c_void;
    tagid_len = 4;

    send_cmd_mt_nla(
        sd,
        fid as __u16,
        pid as __u32,
        NFC_CMD_GET_TARGET as _,
        1,
        &mut tagid_type,
        &mut tagid_type_data,
        &mut tagid_len,
        (NLM_F_REQUEST | NLM_F_DUMP) as _,
    );
    resp_len = recv(sd, &mut ans as *mut Ans as *mut c_void, size_of::<Ans>(), 0) as c_int;
    if ans.n.nlmsg_type == NLMSG_ERROR as __u16 || resp_len < 0 || !NLMSG_OK(&mut ans.n, resp_len) {
        return -1;
    }

    resp_len = GENLMSG_PAYLOAD(&mut ans.n) as c_int;
    na = GENLMSG_DATA(&mut ans.n) as *mut nlattr;

    len = 0;
    targetidx = -1;
    protocol = -1i32 as __u32;
    sel_res = -1i32 as __u8;

    while len < resp_len {
        len += NLA_ALIGN((*na).nla_len as _) as c_int;

        if (*na).nla_type == NFC_ATTR_TARGET_INDEX as __u16 {
            targetidx = *((na as *mut c_char).add(NLA_HDRLEN as usize) as *mut c_int);
        } else if (*na).nla_type == NFC_ATTR_TARGET_SEL_RES as __u16 {
            sel_res = *((na as *mut c_char).add(NLA_HDRLEN as usize) as *mut __u8);
        } else if (*na).nla_type == NFC_ATTR_PROTOCOLS as __u16 {
            protocol = *((na as *mut c_char).add(NLA_HDRLEN as usize) as *mut __u32);
        }

        na = (GENLMSG_DATA(&mut ans.n) as *mut c_char).add(len as usize) as *mut nlattr;
    }

    if targetidx == -1 || sel_res != 0x20 || protocol != NFC_PROTO_ISO14443_MASK {
        return -1;
    }

    targetidx
}

unsafe fn connect_socket(dev_idx: c_int, target_idx: c_int) -> c_int {
    let mut addr: sockaddr_nfc = core::mem::zeroed();
    let sock: c_int;
    let mut err: c_int = 0;

    sock = socket(AF_NFC, SOCK_SEQPACKET, NFC_SOCKPROTO_RAW);
    if sock == -1 {
        return -1;
    }

    addr.sa_family = AF_NFC as _;
    addr.dev_idx = dev_idx as _;
    addr.target_idx = target_idx as _;
    addr.nfc_protocol = NFC_PROTO_ISO14443 as _;

    err = connect(
        sock,
        &mut addr as *mut sockaddr_nfc as *mut sockaddr,
        size_of::<sockaddr_nfc>() as _,
    );
    if err != 0 {
        close(sock);
        return -1;
    }

    sock
}

unsafe fn connect_tag(dev_idx: c_int, virtual_fd: c_int, sd: c_int, fid: c_int, pid: c_int) -> c_int {
    let mut genlhdr: *mut genlmsghdr;
    let mut na: *mut nlattr;
    let mut evt_data: [c_char; 255] = [0; 255];
    let target_idx: c_int;
    let resp_len: c_int;
    let evt_dev: c_int;

    write(
        virtual_fd,
        nci_rf_activate_ntf.as_ptr() as *const c_void,
        size_of_val(&nci_rf_activate_ntf),
    );
    resp_len = recv(sd, evt_data.as_mut_ptr() as *mut c_void, size_of_val(&evt_data), 0) as c_int;
    if resp_len < 0 {
        return -1;
    }

    genlhdr = (evt_data.as_mut_ptr() as *mut nlmsghdr).add(1) as *mut genlmsghdr;
    na = genlhdr.add(1) as *mut nlattr;
    evt_dev = *((na as *mut c_char).add(NLA_HDRLEN as usize) as *mut c_int);
    if dev_idx != evt_dev {
        return -1;
    }

    target_idx = get_taginfo(dev_idx, sd, fid, pid);
    if target_idx == -1 {
        return -1;
    }
    connect_socket(dev_idx, target_idx)
}

unsafe fn read_write_nci_cmd(
    nfc_sock: c_int,
    virtual_fd: c_int,
    cmd: *const __u8,
    cmd_len: __u32,
    rsp: *const __u8,
    rsp_len: __u32,
) -> c_int {
    let mut buf: [c_char; 256] = [0; 256];
    let mut len: c_int;

    send(nfc_sock, cmd.add(3) as *const c_void, (cmd_len - 3) as _, 0);
    len = read(virtual_fd, buf.as_mut_ptr() as *mut c_void, cmd_len as _) as c_int;
    if len < 0 || memcmp(buf.as_ptr() as *const c_void, cmd as *const c_void, cmd_len as _) != 0 {
        return -1;
    }

    write(virtual_fd, rsp as *const c_void, rsp_len as _);
    len = recv(nfc_sock, buf.as_mut_ptr() as *mut c_void, (rsp_len - 2) as _, 0) as c_int;
    if len < 0
        || memcmp(
            buf.as_ptr().add(1) as *const c_void,
            rsp.add(3) as *const c_void,
            (rsp_len - 3) as _,
        ) != 0
    {
        return -1;
    }

    0
}

unsafe fn read_tag(nfc_sock: c_int, virtual_fd: c_int) -> c_int {
    if read_write_nci_cmd(
        nfc_sock,
        virtual_fd,
        nci_t4t_select_cmd.as_ptr(),
        size_of_val(&nci_t4t_select_cmd) as __u32,
        nci_t4t_rsp_ok.as_ptr(),
        size_of_val(&nci_t4t_rsp_ok) as __u32,
    ) != 0
    {
        return -1;
    }

    if read_write_nci_cmd(
        nfc_sock,
        virtual_fd,
        nci_t4t_select_cmd2.as_ptr(),
        size_of_val(&nci_t4t_select_cmd2) as __u32,
        nci_t4t_rsp_ok.as_ptr(),
        size_of_val(&nci_t4t_rsp_ok) as __u32,
    ) != 0
    {
        return -1;
    }

    if read_write_nci_cmd(
        nfc_sock,
        virtual_fd,
        nci_t4t_read_cmd.as_ptr(),
        size_of_val(&nci_t4t_read_cmd) as __u32,
        nci_t4t_read_rsp.as_ptr(),
        size_of_val(&nci_t4t_read_rsp) as __u32,
    ) != 0
    {
        return -1;
    }

    if read_write_nci_cmd(
        nfc_sock,
        virtual_fd,
        nci_t4t_select_cmd3.as_ptr(),
        size_of_val(&nci_t4t_select_cmd3) as __u32,
        nci_t4t_rsp_ok.as_ptr(),
        size_of_val(&nci_t4t_rsp_ok) as __u32,
    ) != 0
    {
        return -1;
    }

    if read_write_nci_cmd(
        nfc_sock,
        virtual_fd,
        nci_t4t_read_cmd2.as_ptr(),
        size_of_val(&nci_t4t_read_cmd2) as __u32,
        nci_t4t_read_rsp2.as_ptr(),
        size_of_val(&nci_t4t_read_rsp2) as __u32,
    ) != 0
    {
        return -1;
    }

    read_write_nci_cmd(
        nfc_sock,
        virtual_fd,
        nci_t4t_read_cmd3.as_ptr(),
        size_of_val(&nci_t4t_read_cmd3) as __u32,
        nci_t4t_read_rsp3.as_ptr(),
        size_of_val(&nci_t4t_read_rsp3) as __u32,
    )
}

unsafe extern "C" fn virtual_deactivate_proc(data: *mut c_void) -> *mut c_void {
    let virtual_fd: c_int;
    let mut buf: [c_char; 256] = [0; 256];
    let deactcmd_len: c_int;
    let len: c_int;

    virtual_fd = *(data as *mut c_int);
    deactcmd_len = size_of_val(&nci_rf_deact_cmd) as c_int;
    len = read(virtual_fd, buf.as_mut_ptr() as *mut c_void, deactcmd_len as _) as c_int;
    if len != deactcmd_len
        || memcmp(
            buf.as_ptr() as *const c_void,
            nci_rf_deact_cmd.as_ptr() as *const c_void,
            deactcmd_len as _,
        ) != 0
    {
        return (-1isize) as *mut c_void;
    }

    write(virtual_fd, nci_rf_deact_rsp.as_ptr() as *const c_void, size_of_val(&nci_rf_deact_rsp));
    write(virtual_fd, nci_rf_deact_ntf.as_ptr() as *const c_void, size_of_val(&nci_rf_deact_ntf));

    ptr::null_mut()
}

unsafe fn disconnect_tag(nfc_sock: c_int, virtual_fd: c_int) -> c_int {
    let mut thread_t: pthread_t = core::mem::zeroed();
    let mut buf: [c_char; 256] = [0; 256];
    let mut status: c_int;
    let mut len: c_int;
    let mut virtual_fd_mut = virtual_fd;

    send(
        nfc_sock,
        nci_t4t_select_cmd3.as_ptr().add(3) as *const c_void,
        size_of_val(&nci_t4t_select_cmd3) - 3,
        0,
    );
    len = read(
        virtual_fd,
        buf.as_mut_ptr() as *mut c_void,
        size_of_val(&nci_t4t_select_cmd3),
    ) as c_int;
    if len < 0
        || memcmp(
            buf.as_ptr() as *const c_void,
            nci_t4t_select_cmd3.as_ptr() as *const c_void,
            size_of_val(&nci_t4t_select_cmd3),
        ) != 0
    {
        return -1;
    }

    len = recv(nfc_sock, buf.as_mut_ptr() as *mut c_void, size_of_val(&nci_t4t_rsp_ok), 0) as c_int;
    if len != -1 {
        return -1;
    }

    status = pthread_create(
        &mut thread_t,
        ptr::null(),
        Some(virtual_deactivate_proc),
        &mut virtual_fd_mut as *mut c_int as *mut c_void,
    );

    close(nfc_sock);
    pthread_join(thread_t, &mut status as *mut c_int as *mut *mut c_void);
    status
}

test_f!(NCI, t4t_tag_read, |self_: *mut NCI| unsafe {
    let nfc_sock: c_int;
    let mut status: c_int;

    status = start_polling(
        (*self_).dev_idex,
        (*self_).proto,
        (*self_).virtual_nci_fd,
        (*self_).sd,
        (*self_).fid as c_int,
        (*self_).pid as c_int,
    );
    EXPECT_EQ!(status, 0);

    nfc_sock = connect_tag(
        (*self_).dev_idex,
        (*self_).virtual_nci_fd,
        (*self_).sd,
        (*self_).fid as c_int,
        (*self_).pid as c_int,
    );
    ASSERT_GT!(nfc_sock, -1);

    status = read_tag(nfc_sock, (*self_).virtual_nci_fd);
    ASSERT_EQ!(status, 0);

    status = disconnect_tag(nfc_sock, (*self_).virtual_nci_fd);
    EXPECT_EQ!(status, 0);
});

test_f!(NCI, deinit, |self_: *mut NCI| unsafe {
    let mut msg: msgtemplate = core::mem::zeroed();
    let mut thread_t: pthread_t = core::mem::zeroed();
    let mut status: c_int = 0;
    let mut rc: c_int;

    rc = get_nci_devid((*self_).sd, (*self_).fid, (*self_).pid, (*self_).dev_idex, &mut msg);
    ASSERT_EQ!(rc, 0);
    EXPECT_EQ!(get_dev_enable_state(&mut msg), 1);

    if (*self_).isNCI2 {
        rc = pthread_create(
            &mut thread_t,
            ptr::null(),
            Some(virtual_deinit_v2),
            &mut (*self_).virtual_nci_fd as *mut c_int as *mut c_void,
        );
    } else {
        rc = pthread_create(
            &mut thread_t,
            ptr::null(),
            Some(virtual_deinit),
            &mut (*self_).virtual_nci_fd as *mut c_int as *mut c_void,
        );
    }
    ASSERT_GT!(rc, -1);

    rc = send_cmd_with_idx(
        (*self_).sd,
        (*self_).fid,
        (*self_).pid,
        NFC_CMD_DEV_DOWN as _,
        (*self_).dev_idex,
    );
    EXPECT_EQ!(rc, 0);

    pthread_join(thread_t, &mut status as *mut c_int as *mut *mut c_void);
    (*self_).open_state = false;
    ASSERT_EQ!(status, 0);

    rc = get_nci_devid((*self_).sd, (*self_).fid, (*self_).pid, (*self_).dev_idex, &mut msg);
    ASSERT_EQ!(rc, 0);
    EXPECT_EQ!(get_dev_enable_state(&mut msg), 0);

    /* Test that operations that normally send packets to the driver
     * don't cause issues when the device is already closed.
     * Note: the send of NFC_CMD_DEV_UP itself still succeeds it's just
     * that the device won't actually be up.
     */
    close((*self_).virtual_nci_fd);
    (*self_).virtual_nci_fd = -1;
    rc = send_cmd_with_idx(
        (*self_).sd,
        (*self_).fid,
        (*self_).pid,
        NFC_CMD_DEV_UP as _,
        (*self_).dev_idex,
    );
    EXPECT_EQ!(rc, 0);
});

test_harness_main!();
