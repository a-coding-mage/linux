// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>

const QUEUE: i32 = 0;
const STACK: i32 = 1;

unsafe fn test_queue_stack_map_by_type(type_: i32) {
    const MAP_SIZE: usize = 32;
    let mut vals: [__u32; MAP_SIZE] = [0; MAP_SIZE];
    let mut val: __u32 = 0;
    let mut i: i32;
    let mut err: i32;
    let mut prog_fd: i32 = 0;
    let mut map_in_fd: i32;
    let mut map_out_fd: i32;
    let mut file: [libc::c_char; 32] = [0; 32];
    let mut buf: [u8; 128] = [0; 128];
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut iph: iphdr = core::mem::zeroed();
    let mut topts = bpf_test_run_opts {
        data_in: &mut pkt_v4 as *mut _ as *mut libc::c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        data_out: buf.as_mut_ptr() as *mut libc::c_void,
        data_size_out: core::mem::size_of_val(&buf) as u32,
        repeat: 1,
        ..core::mem::zeroed()
    };

    /* Fill test values to be used */
    i = 0;
    while i < MAP_SIZE as i32 {
        vals[i as usize] = rand() as __u32;
        i += 1;
    }

    if type_ == QUEUE {
        strscpy(
            file.as_mut_ptr(),
            b"./test_queue_map.bpf.o\0".as_ptr() as *const libc::c_char,
            file.len(),
        );
    } else if type_ == STACK {
        strscpy(
            file.as_mut_ptr(),
            b"./test_stack_map.bpf.o\0".as_ptr() as *const libc::c_char,
            file.len(),
        );
    } else {
        return;
    }

    err = bpf_prog_test_load(
        file.as_ptr(),
        BPF_PROG_TYPE_SCHED_CLS,
        &mut obj,
        &mut prog_fd,
    );
    if CHECK_FAIL(err) {
        return;
    }

    map_in_fd = bpf_find_map(
        b"test_queue_stack_map_by_type\0".as_ptr() as *const libc::c_char,
        obj,
        b"map_in\0".as_ptr() as *const libc::c_char,
    );
    if map_in_fd < 0 {
        goto_out(&mut pkt_v4, obj);
        return;
    }

    map_out_fd = bpf_find_map(
        b"test_queue_stack_map_by_type\0".as_ptr() as *const libc::c_char,
        obj,
        b"map_out\0".as_ptr() as *const libc::c_char,
    );
    if map_out_fd < 0 {
        goto_out(&mut pkt_v4, obj);
        return;
    }

    /* Push 32 elements to the input map */
    i = 0;
    while i < MAP_SIZE as i32 {
        err = bpf_map_update_elem(
            map_in_fd,
            core::ptr::null(),
            &vals[i as usize] as *const __u32 as *const libc::c_void,
            0,
        );
        if CHECK_FAIL(err) {
            goto_out(&mut pkt_v4, obj);
            return;
        }
        i += 1;
    }

    /* The eBPF program pushes iph.saddr in the output map,
     * pops the input map and saves this value in iph.daddr
     */
    i = 0;
    while i < MAP_SIZE as i32 {
        if type_ == QUEUE {
            val = vals[i as usize];
            pkt_v4.iph.saddr = vals[i as usize].wrapping_mul(5);
        } else if type_ == STACK {
            val = vals[MAP_SIZE - 1 - i as usize];
            pkt_v4.iph.saddr = vals[MAP_SIZE - 1 - i as usize].wrapping_mul(5);
        }

        topts.data_size_out = core::mem::size_of_val(&buf) as u32;
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        if err != 0
            || topts.retval != 0
            || topts.data_size_out != core::mem::size_of_val(&pkt_v4) as u32
        {
            break;
        }
        core::ptr::copy_nonoverlapping(
            buf.as_ptr().add(core::mem::size_of::<ethhdr>()),
            &mut iph as *mut iphdr as *mut u8,
            core::mem::size_of_val(&iph),
        );
        if iph.daddr != val {
            break;
        }
        i += 1;
    }

    ASSERT_OK(err, b"bpf_map_pop_elem\0".as_ptr() as *const libc::c_char);
    ASSERT_OK(
        topts.retval,
        b"bpf_map_pop_elem test retval\0".as_ptr() as *const libc::c_char,
    );
    ASSERT_EQ(
        topts.data_size_out,
        core::mem::size_of_val(&pkt_v4) as u32,
        b"bpf_map_pop_elem data_size_out\0".as_ptr() as *const libc::c_char,
    );
    ASSERT_EQ(
        iph.daddr,
        val,
        b"bpf_map_pop_elem iph.daddr\0".as_ptr() as *const libc::c_char,
    );

    /* Queue is empty, program should return TC_ACT_SHOT */
    topts.data_size_out = core::mem::size_of_val(&buf) as u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(
        err,
        b"check-queue-stack-map-empty\0".as_ptr() as *const libc::c_char,
    );
    ASSERT_EQ(
        topts.retval,
        2, /* TC_ACT_SHOT */
        b"check-queue-stack-map-empty test retval\0".as_ptr() as *const libc::c_char,
    );
    ASSERT_EQ(
        topts.data_size_out,
        core::mem::size_of_val(&pkt_v4) as u32,
        b"check-queue-stack-map-empty data_size_out\0".as_ptr() as *const libc::c_char,
    );

    /* Check that the program pushed elements correctly */
    i = 0;
    while i < MAP_SIZE as i32 {
        err = bpf_map_lookup_and_delete_elem(
            map_out_fd,
            core::ptr::null(),
            &mut val as *mut __u32 as *mut libc::c_void,
        );
        ASSERT_OK(
            err,
            b"bpf_map_lookup_and_delete_elem\0".as_ptr() as *const libc::c_char,
        );
        ASSERT_EQ(
            val,
            vals[i as usize].wrapping_mul(5),
            b"bpf_map_push_elem val\0".as_ptr() as *const libc::c_char,
        );
        i += 1;
    }

    goto_out(&mut pkt_v4, obj);
}

unsafe fn goto_out(pkt_v4_ptr: *mut pkt_v4_type, obj: *mut bpf_object) {
    (*pkt_v4_ptr).iph.saddr = 0;
    bpf_object__close(obj);
}

pub unsafe fn test_queue_stack_map() {
    test_queue_stack_map_by_type(QUEUE);
    test_queue_stack_map_by_type(STACK);
}
