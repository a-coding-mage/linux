// C dependencies: linux/stddef.h, linux/ipv6.h, linux/bpf.h, linux/in.h,
// sys/socket.h, bpf/bpf_helpers.h, bpf/bpf_endian.h

extern "C" {
    fn bpf_bind(ctx: *mut bpf_sock_addr, addr: *mut sockaddr, addr_len: u32) -> i64;
}

#[no_mangle]
#[link_section = "freplace/do_bind"]
pub unsafe extern "C" fn new_do_bind(ctx: *mut bpf_sock_addr) -> i32 {
    let mut sa: sockaddr_in = core::mem::zeroed();

    bpf_bind(
        ctx,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        core::mem::size_of::<sockaddr_in>() as u32,
    );
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
