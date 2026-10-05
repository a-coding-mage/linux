// SPDX-License-Identifier: GPL-2.0
// CONFIG_PROC_FS owner. C retains only proc_ops tables and initcall metadata.

/// # Safety
/// Called from the native proc write callback with a live file and user buffer.
unsafe fn psi_write(file: *mut b::file, user_buf: *const c_char,
                    nbytes: usize, res: b::psi_res) -> b::ssize_t {
    unsafe {
        if b::rust_psi_disabled() { return -(b::RUST_PSI_EOPNOTSUPP as b::ssize_t); }
        if nbytes == 0 { return -(b::RUST_PSI_EINVAL as b::ssize_t); }
        let mut buf = [0 as c_char; 32];
        let size = min(nbytes, buf.len());
        if b::rust_psi_copy_from_user(buf.as_mut_ptr().cast(), user_buf.cast(), size as c_ulong) != 0 {
            return -(b::RUST_PSI_EFAULT as b::ssize_t);
        }
        buf[size - 1] = 0;
        let seq = (*file).private_data.cast::<b::seq_file>();
        b::rust_psi_mutex_lock(addr_of_mut!((*seq).lock));
        let slot = b::rust_psi_seq_private_slot(seq);
        if !(*slot).is_null() {
            b::rust_psi_mutex_unlock(addr_of_mut!((*seq).lock));
            return -(b::RUST_PSI_EBUSY as b::ssize_t);
        }
        let mut need_worker = false;
        let new = psi_trigger_create(addr_of_mut!(b::psi_system), buf.as_mut_ptr(),
                                     res, file, null_mut(), &mut need_worker);
        if b::rust_psi_is_err(new.cast()) {
            b::rust_psi_mutex_unlock(addr_of_mut!((*seq).lock));
            return b::rust_psi_ptr_err(new.cast()) as b::ssize_t;
        }
        if need_worker {
            let ret = psi_trigger_create_rtpoll_worker(addr_of_mut!(b::psi_system));
            if ret != 0 {
                psi_trigger_destroy(new);
                b::rust_psi_mutex_unlock(addr_of_mut!((*seq).lock));
                return ret as b::ssize_t;
            }
        }
        b::rust_psi_store_trigger(slot, new);
        b::rust_psi_mutex_unlock(addr_of_mut!((*seq).lock));
        nbytes as b::ssize_t
    }
}

macro_rules! proc_callbacks {
    ($show:ident, $open:ident, $write:ident, $resource:ident) => {
        /// # Safety
        /// Native seq_file callback arguments remain valid throughout the call.
        #[no_mangle]
        pub unsafe extern "C" fn $show(m: *mut b::seq_file, _data: *mut c_void) -> c_int {
            unsafe { psi_show(m, addr_of_mut!(b::psi_system), b::$resource as b::psi_res) }
        }
        /// # Safety
        /// Native proc open callback owns the live inode and file references.
        #[no_mangle]
        pub unsafe extern "C" fn $open(_inode: *mut b::inode, file: *mut b::file) -> c_int {
            unsafe { b::rust_psi_single_open(file, Some($show)) }
        }
        /// # Safety
        /// Native proc write callback supplies a valid file and userspace range.
        #[no_mangle]
        pub unsafe extern "C" fn $write(file: *mut b::file, user_buf: *const c_char,
            nbytes: usize, _pos: *mut b::loff_t) -> b::ssize_t {
            unsafe { psi_write(file, user_buf, nbytes, b::$resource as b::psi_res) }
        }
    };
}
proc_callbacks!(rust_psi_io_show, rust_psi_io_open, rust_psi_io_write, RUST_PSI_IO);
proc_callbacks!(rust_psi_memory_show, rust_psi_memory_open, rust_psi_memory_write, RUST_PSI_MEM);
proc_callbacks!(rust_psi_cpu_show, rust_psi_cpu_open, rust_psi_cpu_write, RUST_PSI_CPU);
#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
proc_callbacks!(rust_psi_irq_show, rust_psi_irq_open, rust_psi_irq_write, RUST_PSI_IRQ);

/// # Safety
/// File's seq_file/trigger lifetime is stabilized by the native poll caller.
#[no_mangle]
pub unsafe extern "C" fn rust_psi_fop_poll(file: *mut b::file, wait: *mut b::poll_table) -> b::__poll_t {
    unsafe {
        let seq = (*file).private_data.cast::<b::seq_file>();
        psi_trigger_poll(b::rust_psi_seq_private_slot(seq), file, wait)
    }
}

/// # Safety
/// Final native file release owns the trigger and seq_file being destroyed.
#[no_mangle]
pub unsafe extern "C" fn rust_psi_fop_release(inode: *mut b::inode, file: *mut b::file) -> c_int {
    unsafe {
        let seq = (*file).private_data.cast::<b::seq_file>();
        psi_trigger_destroy((*b::rust_psi_seq_private_slot(seq)).cast());
        b::rust_psi_single_release(inode, file)
    }
}

/// # Safety
/// Called once by native initcall registration after boot command-line parsing.
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_psi_proc_init() -> c_int {
    unsafe {
        if b::rust_psi_enable {
            b::rust_psi_proc_mkdir();
            b::rust_psi_proc_create_io();
            b::rust_psi_proc_create_memory();
            b::rust_psi_proc_create_cpu();
            #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
            b::rust_psi_proc_create_irq();
        }
        0
    }
}
