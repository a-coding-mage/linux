/* SPDX-License-Identifier: GPL-2.0 */

/* C header included <linux/stddef.h> under __KERNEL__, otherwise <stddef.h>, for NULL. */

pub static initial_sid_to_string: [*const ::core::ffi::c_char; 28] = [
    ::core::ptr::null(), /* zero placeholder, not used */
    b"kernel\0".as_ptr() as *const ::core::ffi::c_char, /* kernel / SECINITSID_KERNEL */
    b"security\0".as_ptr() as *const ::core::ffi::c_char, /* security / SECINITSID_SECURITY */
    b"unlabeled\0".as_ptr() as *const ::core::ffi::c_char, /* unlabeled / SECINITSID_UNLABELED */
    ::core::ptr::null(), /* fs */
    b"file\0".as_ptr() as *const ::core::ffi::c_char, /* file / SECINITSID_FILE */
    ::core::ptr::null(), /* file_labels */
    b"init\0".as_ptr() as *const ::core::ffi::c_char, /* init / SECINITSID_INIT */
    b"any_socket\0".as_ptr() as *const ::core::ffi::c_char, /* any_socket / SECINITSID_ANY_SOCKET */
    b"port\0".as_ptr() as *const ::core::ffi::c_char, /* port / SECINITSID_PORT */
    b"netif\0".as_ptr() as *const ::core::ffi::c_char, /* netif / SECINITSID_NETIF */
    b"netmsg\0".as_ptr() as *const ::core::ffi::c_char, /* netmsg / SECINITSID_NETMSG */
    b"node\0".as_ptr() as *const ::core::ffi::c_char, /* node / SECINITSID_NODE */
    ::core::ptr::null(), /* igmp_packet */
    ::core::ptr::null(), /* icmp_socket */
    ::core::ptr::null(), /* tcp_socket */
    ::core::ptr::null(), /* sysctl_modprobe */
    ::core::ptr::null(), /* sysctl */
    ::core::ptr::null(), /* sysctl_fs */
    ::core::ptr::null(), /* sysctl_kernel */
    ::core::ptr::null(), /* sysctl_net */
    ::core::ptr::null(), /* sysctl_net_unix */
    ::core::ptr::null(), /* sysctl_vm */
    ::core::ptr::null(), /* sysctl_dev */
    ::core::ptr::null(), /* kmod */
    ::core::ptr::null(), /* policy */
    ::core::ptr::null(), /* scmp_packet */
    b"devnull\0".as_ptr() as *const ::core::ffi::c_char, /* devnull / SECINITSID_DEVNULL */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
