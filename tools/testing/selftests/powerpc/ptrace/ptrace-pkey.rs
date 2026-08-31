// SPDX-License-Identifier: GPL-2.0+
/*
 * Ptrace test for Memory Protection Key registers
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 * Copyright (C) 2018 IBM Corporation.
 */
// C dependencies: "ptrace.h", "child.h", "pkeys.h"

type pid_t = libc::pid_t;

static user_read: &[u8] = b"[User Read (Running)]\0";
static user_write: &[u8] = b"[User Write (Running)]\0";
static ptrace_read_running: &[u8] = b"[Ptrace Read (Running)]\0";
static ptrace_write_running: &[u8] = b"[Ptrace Write (Running)]\0";

/* Information shared between the parent and the child. */
#[repr(C)]
struct shared_info {
    child_sync: child_sync,

    /* AMR value the parent expects to read from the child. */
    amr1: libc::c_ulong,

    /* AMR value the parent is expected to write to the child. */
    amr2: libc::c_ulong,

    /* AMR value that ptrace should refuse to write to the child. */
    invalid_amr: libc::c_ulong,

    /* IAMR value the parent expects to read from the child. */
    expected_iamr: libc::c_ulong,

    /* UAMOR value the parent expects to read from the child. */
    expected_uamor: libc::c_ulong,

    /*
     * IAMR and UAMOR values that ptrace should refuse to write to the child
     * (even though they're valid ones) because userspace doesn't have
     * access to those registers.
     */
    invalid_iamr: libc::c_ulong,
    invalid_uamor: libc::c_ulong,
}

unsafe fn child(info: *mut shared_info) -> libc::c_int {
    let mut reg: libc::c_ulong;
    let mut disable_execute: bool = true;
    let mut pkey1: libc::c_int;
    let pkey2: libc::c_int;
    let pkey3: libc::c_int;
    let mut ret: libc::c_int;

    /* Wait until parent fills out the initial register values. */
    ret = wait_parent(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    /* Get some pkeys so that we can change their bits in the AMR. */
    pkey1 = sys_pkey_alloc(0, PKEY_DISABLE_EXECUTE);
    if pkey1 < 0 {
        pkey1 = sys_pkey_alloc(0, PKEY_UNRESTRICTED);
        CHILD_FAIL_IF!(pkey1 < 0, &mut (*info).child_sync);

        disable_execute = false;
    }

    pkey2 = sys_pkey_alloc(0, PKEY_UNRESTRICTED);
    CHILD_FAIL_IF!(pkey2 < 0, &mut (*info).child_sync);

    pkey3 = sys_pkey_alloc(0, PKEY_UNRESTRICTED);
    CHILD_FAIL_IF!(pkey3 < 0, &mut (*info).child_sync);

    (*info).amr1 |= 3u64 as libc::c_ulong << pkeyshift(pkey1);
    (*info).amr2 |= 3u64 as libc::c_ulong << pkeyshift(pkey2);
    /*
     * invalid amr value where we try to force write
     * things which are deined by a uamor setting.
     */
    (*info).invalid_amr = (*info).amr2 | (!(0x0u64 as libc::c_ulong) & !(*info).expected_uamor);

    /*
     * if PKEY_DISABLE_EXECUTE succeeded we should update the expected_iamr
     */
    if disable_execute {
        (*info).expected_iamr |= 1u64 as libc::c_ulong << pkeyshift(pkey1);
    } else {
        (*info).expected_iamr &= !(1u64 as libc::c_ulong << pkeyshift(pkey1));
    }

    /*
     * We allocated pkey2 and pkey 3 above. Clear the IAMR bits.
     */
    (*info).expected_iamr &= !(1u64 as libc::c_ulong << pkeyshift(pkey2));
    (*info).expected_iamr &= !(1u64 as libc::c_ulong << pkeyshift(pkey3));

    /*
     * Create an IAMR value different from expected value.
     * Kernel will reject an IAMR and UAMOR change.
     */
    (*info).invalid_iamr =
        (*info).expected_iamr | (1u64 as libc::c_ulong << pkeyshift(pkey1)
            | 1u64 as libc::c_ulong << pkeyshift(pkey2));
    (*info).invalid_uamor = (*info).expected_uamor & !(0x3u64 as libc::c_ulong << pkeyshift(pkey1));

    printf(
        b"%-30s AMR: %016lx pkey1: %d pkey2: %d pkey3: %d\n\0".as_ptr() as *const libc::c_char,
        user_write.as_ptr(),
        (*info).amr1,
        pkey1,
        pkey2,
        pkey3,
    );

    set_amr((*info).amr1);

    /* Wait for parent to read our AMR value and write a new one. */
    ret = prod_parent(&mut (*info).child_sync);
    CHILD_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    ret = wait_parent(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    reg = mfspr(SPRN_AMR);

    printf(
        b"%-30s AMR: %016lx\n\0".as_ptr() as *const libc::c_char,
        user_read.as_ptr(),
        reg,
    );

    CHILD_FAIL_IF!(reg != (*info).amr2, &mut (*info).child_sync);

    /*
     * Wait for parent to try to write an invalid AMR value.
     */
    ret = prod_parent(&mut (*info).child_sync);
    CHILD_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    ret = wait_parent(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    reg = mfspr(SPRN_AMR);

    printf(
        b"%-30s AMR: %016lx\n\0".as_ptr() as *const libc::c_char,
        user_read.as_ptr(),
        reg,
    );

    CHILD_FAIL_IF!(reg != (*info).amr2, &mut (*info).child_sync);

    /*
     * Wait for parent to try to write an IAMR and a UAMOR value. We can't
     * verify them, but we can verify that the AMR didn't change.
     */
    ret = prod_parent(&mut (*info).child_sync);
    CHILD_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    ret = wait_parent(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    reg = mfspr(SPRN_AMR);

    printf(
        b"%-30s AMR: %016lx\n\0".as_ptr() as *const libc::c_char,
        user_read.as_ptr(),
        reg,
    );

    CHILD_FAIL_IF!(reg != (*info).amr2, &mut (*info).child_sync);

    /* Now let parent now that we are finished. */

    ret = prod_parent(&mut (*info).child_sync);
    CHILD_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    TEST_PASS
}

unsafe fn parent(info: *mut shared_info, pid: pid_t) -> libc::c_int {
    let mut regs: [libc::c_ulong; 3] = [0; 3];
    let mut ret: libc::c_int;
    let mut status: libc::c_int = 0;

    /*
     * Get the initial values for AMR, IAMR and UAMOR and communicate them
     * to the child.
     */
    ret = ptrace_read_regs(pid, NT_PPC_PKEY, regs.as_mut_ptr(), 3);
    PARENT_SKIP_IF_UNSUPPORTED!(ret, &mut (*info).child_sync, "PKEYs not supported");
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    (*info).amr1 = regs[0];
    (*info).amr2 = (*info).amr1;
    (*info).expected_iamr = regs[1];
    (*info).expected_uamor = regs[2];

    /* Wake up child so that it can set itself up. */
    ret = prod_child(&mut (*info).child_sync);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    ret = wait_child(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    /* Verify that we can read the pkey registers from the child. */
    ret = ptrace_read_regs(pid, NT_PPC_PKEY, regs.as_mut_ptr(), 3);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    printf(
        b"%-30s AMR: %016lx IAMR: %016lx UAMOR: %016lx\n\0".as_ptr() as *const libc::c_char,
        ptrace_read_running.as_ptr(),
        regs[0],
        regs[1],
        regs[2],
    );

    PARENT_FAIL_IF!(regs[0] != (*info).amr1, &mut (*info).child_sync);
    PARENT_FAIL_IF!(regs[1] != (*info).expected_iamr, &mut (*info).child_sync);
    PARENT_FAIL_IF!(regs[2] != (*info).expected_uamor, &mut (*info).child_sync);

    /* Write valid AMR value in child. */
    ret = ptrace_write_regs(pid, NT_PPC_PKEY, &mut (*info).amr2, 1);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    printf(
        b"%-30s AMR: %016lx\n\0".as_ptr() as *const libc::c_char,
        ptrace_write_running.as_ptr(),
        (*info).amr2,
    );

    /* Wake up child so that it can verify it changed. */
    ret = prod_child(&mut (*info).child_sync);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    ret = wait_child(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    /* Write invalid AMR value in child. */
    ret = ptrace_write_regs(pid, NT_PPC_PKEY, &mut (*info).invalid_amr, 1);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    printf(
        b"%-30s AMR: %016lx\n\0".as_ptr() as *const libc::c_char,
        ptrace_write_running.as_ptr(),
        (*info).invalid_amr,
    );

    /* Wake up child so that it can verify it didn't change. */
    ret = prod_child(&mut (*info).child_sync);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    ret = wait_child(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    /* Try to write to IAMR. */
    regs[0] = (*info).amr1;
    regs[1] = (*info).invalid_iamr;
    ret = ptrace_write_regs(pid, NT_PPC_PKEY, regs.as_mut_ptr(), 2);
    PARENT_FAIL_IF!(ret == 0, &mut (*info).child_sync);

    printf(
        b"%-30s AMR: %016lx IAMR: %016lx\n\0".as_ptr() as *const libc::c_char,
        ptrace_write_running.as_ptr(),
        regs[0],
        regs[1],
    );

    /* Try to write to IAMR and UAMOR. */
    regs[2] = (*info).invalid_uamor;
    ret = ptrace_write_regs(pid, NT_PPC_PKEY, regs.as_mut_ptr(), 3);
    PARENT_FAIL_IF!(ret == 0, &mut (*info).child_sync);

    printf(
        b"%-30s AMR: %016lx IAMR: %016lx UAMOR: %016lx\n\0".as_ptr() as *const libc::c_char,
        ptrace_write_running.as_ptr(),
        regs[0],
        regs[1],
        regs[2],
    );

    /* Verify that all registers still have their expected values. */
    ret = ptrace_read_regs(pid, NT_PPC_PKEY, regs.as_mut_ptr(), 3);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    printf(
        b"%-30s AMR: %016lx IAMR: %016lx UAMOR: %016lx\n\0".as_ptr() as *const libc::c_char,
        ptrace_read_running.as_ptr(),
        regs[0],
        regs[1],
        regs[2],
    );

    PARENT_FAIL_IF!(regs[0] != (*info).amr2, &mut (*info).child_sync);
    PARENT_FAIL_IF!(regs[1] != (*info).expected_iamr, &mut (*info).child_sync);
    PARENT_FAIL_IF!(regs[2] != (*info).expected_uamor, &mut (*info).child_sync);

    /* Wake up child so that it can verify AMR didn't change and wrap up. */
    ret = prod_child(&mut (*info).child_sync);
    PARENT_FAIL_IF!(ret != 0, &mut (*info).child_sync);

    ret = wait(&mut status);
    if ret != pid {
        printf(b"Child's exit status not captured\n\0".as_ptr() as *const libc::c_char);
        ret = TEST_PASS;
    } else if !WIFEXITED(status) {
        printf(b"Child exited abnormally\n\0".as_ptr() as *const libc::c_char);
        ret = TEST_FAIL;
    } else {
        ret = if WEXITSTATUS(status) != 0 {
            TEST_FAIL
        } else {
            TEST_PASS
        };
    }

    ret
}

unsafe fn ptrace_pkey() -> libc::c_int {
    let mut info: *mut shared_info;
    let shm_id: libc::c_int;
    let mut ret: libc::c_int;
    let pid: pid_t;

    shm_id = shmget(
        IPC_PRIVATE,
        core::mem::size_of::<shared_info>(),
        0o777 | IPC_CREAT,
    );
    info = shmat(shm_id, core::ptr::null_mut(), 0) as *mut shared_info;

    ret = init_child_sync(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    pid = fork();
    if pid < 0 {
        perror(b"fork() failed\0".as_ptr() as *const libc::c_char);
        ret = TEST_FAIL;
    } else if pid == 0 {
        ret = child(info);
    } else {
        ret = parent(info, pid);
    }

    shmdt(info as *const libc::c_void);

    if pid != 0 {
        destroy_child_sync(&mut (*info).child_sync);
        shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
    }

    ret
}

fn main() -> libc::c_int {
    unsafe { test_harness(ptrace_pkey, b"ptrace_pkey\0".as_ptr() as *const libc::c_char) }
}
