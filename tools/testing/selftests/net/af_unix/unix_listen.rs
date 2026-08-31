// SPDX-License-Identifier: GPL-2.0
/*
 * Tests for the state checks in AF_UNIX listen().
 *
 * The central case is a regression test: listen() on a bound socket that
 * is already connected (i.e. not in TCP_CLOSE or TCP_LISTEN state) must
 * fail with EINVAL.  A prior change accidentally let it return success
 * without doing anything, because a helper called in between reset the
 * error code to 0.  The neighbouring checks (unbound, already listening)
 * are tested too so they cannot silently regress the same way.
 *
 * Every case runs for both listenable socket types (SOCK_STREAM and
 * SOCK_SEQPACKET) and both pathname and abstract addresses.
 */

// C dependencies: errno.h, stddef.h, stdio.h, string.h, unistd.h,
// sys/socket.h, sys/un.h, and "kselftest_harness.h".

const SK_NAME: &[u8] = b"unix_listen_sk\0";
const SRV_NAME: &[u8] = b"unix_listen_srv\0";

#[repr(C)]
struct unix_listen {
    sk: libc::c_int,     /* socket under test */
    server: libc::c_int, /* a listening peer, when a test needs one */
    addr: libc::sockaddr_un,
    srv_addr: libc::sockaddr_un,
    addrlen: libc::socklen_t,
    srv_addrlen: libc::socklen_t,
}

#[repr(C)]
struct unix_listen_variant {
    type_: libc::c_int,
    abstract_: libc::c_int,
}

static stream_pathname: unix_listen_variant = unix_listen_variant {
    type_: libc::SOCK_STREAM,
    abstract_: 0,
};

static stream_abstract: unix_listen_variant = unix_listen_variant {
    type_: libc::SOCK_STREAM,
    abstract_: 1,
};

static seqpacket_pathname: unix_listen_variant = unix_listen_variant {
    type_: libc::SOCK_SEQPACKET,
    abstract_: 0,
};

static seqpacket_abstract: unix_listen_variant = unix_listen_variant {
    type_: libc::SOCK_SEQPACKET,
    abstract_: 1,
};

/* Fill @addr with a pathname or abstract address named @name. */
unsafe fn unix_set_addr(
    addr: *mut libc::sockaddr_un,
    name: *const libc::c_char,
    abstract_: libc::c_int,
) -> libc::socklen_t {
    let len: libc::size_t = libc::strlen(name);

    libc::memset(
        addr as *mut libc::c_void,
        0,
        core::mem::size_of_val(&*addr),
    );
    (*addr).sun_family = libc::AF_UNIX as libc::sa_family_t;
    /* An abstract address leads with a NUL and has no filesystem entry. */
    libc::memcpy(
        (*addr)
            .sun_path
            .as_mut_ptr()
            .add(if abstract_ != 0 { 1 } else { 0 }) as *mut libc::c_void,
        name as *const libc::c_void,
        len,
    );

    core::mem::offset_of!(libc::sockaddr_un, sun_path) as libc::socklen_t
        + len as libc::socklen_t
        + 1
}

unsafe fn unix_listen_setup(self_: *mut unix_listen, variant: *const unix_listen_variant) {
    (*self_).sk = -1;
    (*self_).server = -1;
    (*self_).addrlen = unix_set_addr(
        &mut (*self_).addr,
        SK_NAME.as_ptr() as *const libc::c_char,
        (*variant).abstract_,
    );
    (*self_).srv_addrlen = unix_set_addr(
        &mut (*self_).srv_addr,
        SRV_NAME.as_ptr() as *const libc::c_char,
        (*variant).abstract_,
    );
}

unsafe fn unix_listen_teardown(self_: *mut unix_listen, variant: *const unix_listen_variant) {
    if (*self_).sk >= 0 {
        libc::close((*self_).sk);
    }
    if (*self_).server >= 0 {
        libc::close((*self_).server);
    }

    /* Pathname sockets leave a filesystem entry behind; abstract ones do not. */
    if (*variant).abstract_ == 0 {
        libc::remove(SK_NAME.as_ptr() as *const libc::c_char);
        libc::remove(SRV_NAME.as_ptr() as *const libc::c_char);
    }
}

/* A bound socket in TCP_CLOSE is the normal, allowed case. */
unsafe fn bound_is_ok(self_: *mut unix_listen, variant: *const unix_listen_variant) {
    let mut err: libc::c_int;

    (*self_).sk = libc::socket(libc::AF_UNIX, (*variant).type_, 0);
    ASSERT_LE!(0, (*self_).sk);

    err = libc::bind(
        (*self_).sk,
        &mut (*self_).addr as *mut libc::sockaddr_un as *mut libc::sockaddr,
        (*self_).addrlen,
    );
    ASSERT_EQ!(0, err);

    err = libc::listen((*self_).sk, 8);
    EXPECT_EQ!(0, err);
}

/* Listening again on an already-listening socket (TCP_LISTEN) is allowed. */
unsafe fn relisten_is_ok(self_: *mut unix_listen, variant: *const unix_listen_variant) {
    let mut err: libc::c_int;

    (*self_).sk = libc::socket(libc::AF_UNIX, (*variant).type_, 0);
    ASSERT_LE!(0, (*self_).sk);

    err = libc::bind(
        (*self_).sk,
        &mut (*self_).addr as *mut libc::sockaddr_un as *mut libc::sockaddr,
        (*self_).addrlen,
    );
    ASSERT_EQ!(0, err);

    err = libc::listen((*self_).sk, 8);
    ASSERT_EQ!(0, err);

    err = libc::listen((*self_).sk, 16);
    EXPECT_EQ!(0, err);
}

/* listen() on an unbound socket fails: there is nothing to listen on. */
unsafe fn unbound_is_einval(self_: *mut unix_listen, variant: *const unix_listen_variant) {
    let mut err: libc::c_int;

    (*self_).sk = libc::socket(libc::AF_UNIX, (*variant).type_, 0);
    ASSERT_LE!(0, (*self_).sk);

    err = libc::listen((*self_).sk, 8);
    EXPECT_EQ!(-1, err);
    EXPECT_EQ!(libc::EINVAL, *libc::__errno_location());
}

/*
 * The regression: a bound socket that has already been connected is not in
 * TCP_CLOSE or TCP_LISTEN, so listen() must reject it with EINVAL rather
 * than quietly succeeding.
 */
unsafe fn connected_is_einval(self_: *mut unix_listen, variant: *const unix_listen_variant) {
    let mut err: libc::c_int;

    (*self_).server = libc::socket(libc::AF_UNIX, (*variant).type_, 0);
    ASSERT_LE!(0, (*self_).server);

    err = libc::bind(
        (*self_).server,
        &mut (*self_).srv_addr as *mut libc::sockaddr_un as *mut libc::sockaddr,
        (*self_).srv_addrlen,
    );
    ASSERT_EQ!(0, err);

    err = libc::listen((*self_).server, 8);
    ASSERT_EQ!(0, err);

    (*self_).sk = libc::socket(libc::AF_UNIX, (*variant).type_, 0);
    ASSERT_LE!(0, (*self_).sk);

    /* Bind first so the unbound check does not mask the state check. */
    err = libc::bind(
        (*self_).sk,
        &mut (*self_).addr as *mut libc::sockaddr_un as *mut libc::sockaddr,
        (*self_).addrlen,
    );
    ASSERT_EQ!(0, err);

    err = libc::connect(
        (*self_).sk,
        &mut (*self_).srv_addr as *mut libc::sockaddr_un as *mut libc::sockaddr,
        (*self_).srv_addrlen,
    );
    ASSERT_EQ!(0, err);

    err = libc::listen((*self_).sk, 8);
    EXPECT_EQ!(-1, err);
    EXPECT_EQ!(libc::EINVAL, *libc::__errno_location());
}

// TEST_HARNESS_MAIN
