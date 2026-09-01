// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Information interface for ALSA driver
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies: linux/init.h, linux/time.h, linux/mm.h, linux/slab.h,
// linux/string.h, linux/module.h, sound/core.h, sound/minors.h,
// sound/info.h, linux/utsname.h, linux/proc_fs.h, linux/mutex.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type loff_t = i64;
pub type ssize_t = isize;
pub type size_t = usize;
pub type __poll_t = c_uint;

extern "C" {
    static mut snd_proc_root: *mut snd_info_entry;
    static mut snd_seq_root: *mut snd_info_entry;

    static mut info_mutex: mutex;

    static mut snd_info_entry_operations: proc_ops;
    static mut snd_info_text_entry_ops: proc_ops;

    static mut THIS_MODULE: *mut module;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;

    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kvzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kvfree(ptr: *const c_void);
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;

    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);

    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn file_inode(file: *mut file) -> *mut inode;
    fn pde_data(inode: *mut inode) -> *mut snd_info_entry;
    fn proc_mkdir(name: *const c_char, parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    fn proc_mkdir_mode(
        name: *const c_char,
        mode: c_uint,
        parent: *mut proc_dir_entry,
    ) -> *mut proc_dir_entry;
    fn proc_create_data(
        name: *const c_char,
        mode: c_uint,
        parent: *mut proc_dir_entry,
        ops: *const proc_ops,
        data: *mut c_void,
    ) -> *mut proc_dir_entry;
    fn proc_set_size(p: *mut proc_dir_entry, size: loff_t);
    fn proc_symlink(
        name: *const c_char,
        parent: *mut proc_dir_entry,
        dest: *const c_char,
    ) -> *mut proc_dir_entry;
    fn proc_remove(de: *mut proc_dir_entry);

    fn single_open(
        file: *mut file,
        show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn single_open_size(
        file: *mut file,
        show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
        data: *mut c_void,
        size: size_t,
    ) -> c_int;
    fn single_release(inode: *mut inode, file: *mut file) -> c_int;
    fn seq_lseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn seq_read(
        file: *mut file,
        buf: *mut c_char,
        size: size_t,
        ppos: *mut loff_t,
    ) -> ssize_t;

    fn snd_minor_info_init() -> c_int;
    fn snd_minor_info_oss_init() -> c_int;
    fn snd_card_info_init() -> c_int;
    fn snd_info_minor_register() -> c_int;
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    ) -> c_int;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_BUG_ON(condition: bool) -> bool;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn init_utsname() -> *mut new_utsname;
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
    pub f_pos: loff_t,
    pub f_flags: c_int,
}

#[repr(C)]
pub struct seq_file {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct new_utsname {
    pub release: [c_char; 65],
}

#[repr(C)]
pub struct snd_card {
    pub id: *mut c_char,
    pub number: c_int,
    pub module: *mut module,
    pub proc_root: *mut snd_info_entry,
    pub proc_root_link: *mut proc_dir_entry,
}

#[repr(C)]
pub struct snd_info_buffer {
    pub buffer: *mut c_char,
    pub curr: size_t,
    pub size: size_t,
    pub len: size_t,
    pub stop: c_int,
    pub error: c_int,
}

#[repr(C)]
pub struct snd_info_private_data {
    pub rbuffer: *mut snd_info_buffer,
    pub wbuffer: *mut snd_info_buffer,
    pub entry: *mut snd_info_entry,
    pub file_private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_info_entry_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_info_entry, c_int, *mut *mut c_void) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut snd_info_entry, c_int, *mut c_void)>,
    pub read: Option<
        unsafe extern "C" fn(
            *mut snd_info_entry,
            *mut c_void,
            *mut file,
            *mut c_char,
            size_t,
            loff_t,
        ) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            *mut snd_info_entry,
            *mut c_void,
            *mut file,
            *const c_char,
            size_t,
            loff_t,
        ) -> ssize_t,
    >,
    pub llseek: Option<
        unsafe extern "C" fn(
            *mut snd_info_entry,
            *mut c_void,
            *mut file,
            loff_t,
            c_int,
        ) -> loff_t,
    >,
    pub poll: Option<
        unsafe extern "C" fn(
            *mut snd_info_entry,
            *mut c_void,
            *mut file,
            *mut poll_table,
        ) -> __poll_t,
    >,
    pub ioctl: Option<
        unsafe extern "C" fn(
            *mut snd_info_entry,
            *mut c_void,
            *mut file,
            c_uint,
            c_ulong,
        ) -> c_long,
    >,
    pub mmap: Option<
        unsafe extern "C" fn(
            *mut snd_info_entry,
            *mut c_void,
            *mut inode,
            *mut file,
            *mut vm_area_struct,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    pub write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub ops: *mut snd_info_entry_ops,
    pub text: core::mem::ManuallyDrop<snd_info_entry_text>,
}

#[repr(C)]
pub struct snd_info_entry {
    pub name: *mut c_char,
    pub mode: c_uint,
    pub content: c_int,
    pub access: mutex,
    pub children: list_head,
    pub list: list_head,
    pub parent: *mut snd_info_entry,
    pub module: *mut module,
    pub p: *mut proc_dir_entry,
    pub size: loff_t,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_info_entry)>,
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub struct proc_ops {
    pub proc_lseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub proc_read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub proc_write:
        Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub proc_poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    pub proc_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub proc_mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub proc_open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub proc_release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}

const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOTTY: c_int = 25;
const ENXIO: c_int = 6;
const EFAULT: c_int = 14;

const GFP_KERNEL: c_uint = 0;
const PAGE_SIZE: size_t = 4096;
const SEEK_SET: c_int = 0;
const SEEK_CUR: c_int = 1;
const SEEK_END: c_int = 2;
const O_ACCMODE: c_int = 3;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const S_IFDIR: c_uint = 0o040000;
const S_IFREG: c_uint = 0o100000;
const SNDRV_INFO_CONTENT_TEXT: c_int = 0;
const SNDRV_INFO_CONTENT_DATA: c_int = 1;
const EPOLLIN: __poll_t = 0x001;
const EPOLLOUT: __poll_t = 0x004;
const EPOLLRDNORM: __poll_t = 0x040;
const EPOLLWRNORM: __poll_t = 0x100;

const fn page_align(x: size_t) -> size_t {
    (x + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

unsafe fn init_list_head(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    let prev = (*head).prev;
    (*new).next = head;
    (*new).prev = prev;
    (*prev).next = new;
    (*head).prev = new;
}

unsafe fn list_del(entry: *mut list_head) {
    let prev = (*entry).prev;
    let next = (*entry).next;
    (*next).prev = prev;
    (*prev).next = next;
}

const fn s_isdir(mode: c_uint) -> bool {
    (mode & S_IFDIR) == S_IFDIR
}

unsafe fn zalloc_obj<T>() -> *mut T {
    kzalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

#[no_mangle]
pub unsafe extern "C" fn snd_info_check_reserved_words(str_: *const c_char) -> c_int {
    static RESERVED: [*const c_char; 12] = [
        b"version\0".as_ptr() as *const c_char,
        b"meminfo\0".as_ptr() as *const c_char,
        b"memdebug\0".as_ptr() as *const c_char,
        b"detect\0".as_ptr() as *const c_char,
        b"devices\0".as_ptr() as *const c_char,
        b"oss\0".as_ptr() as *const c_char,
        b"cards\0".as_ptr() as *const c_char,
        b"timers\0".as_ptr() as *const c_char,
        b"synth\0".as_ptr() as *const c_char,
        b"pcm\0".as_ptr() as *const c_char,
        b"seq\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ];
    let mut xstr = RESERVED.as_ptr();

    while !(*xstr).is_null() {
        if strcmp(*xstr, str_) == 0 {
            return 0;
        }
        xstr = xstr.add(1);
    }
    if strncmp(str_, b"card\0".as_ptr() as *const c_char, 4) == 0 {
        return 0;
    }
    1
}

unsafe extern "C" fn alloc_info_private(
    entry: *mut snd_info_entry,
    ret: *mut *mut snd_info_private_data,
) -> c_int {
    let data: *mut snd_info_private_data;

    if entry.is_null() || (*entry).p.is_null() {
        return -ENODEV;
    }
    if !try_module_get((*entry).module) {
        return -ENODEV;
    }
    data = zalloc_obj::<snd_info_private_data>();
    if data.is_null() {
        module_put((*entry).module);
        return -ENOMEM;
    }
    (*data).entry = entry;
    *ret = data;
    0
}

unsafe extern "C" fn valid_pos(pos: loff_t, count: size_t) -> bool {
    if pos < 0 || (pos as c_long as loff_t) != pos || (count as ssize_t) < 0 {
        return false;
    }
    if (pos as c_ulong).wrapping_add(count as c_ulong) < pos as c_ulong {
        return false;
    }
    true
}

/*
 * file ops for binary proc files
 */
unsafe extern "C" fn snd_info_entry_llseek(file: *mut file, mut offset: loff_t, orig: c_int) -> loff_t {
    let data: *mut snd_info_private_data;
    let entry: *mut snd_info_entry;
    let size: loff_t;

    data = (*file).private_data as *mut snd_info_private_data;
    entry = (*data).entry;
    mutex_lock(&mut (*entry).access);
    if let Some(llseek) = (*(*entry).c.ops).llseek {
        let ret = llseek(entry, (*data).file_private_data, file, offset, orig);
        mutex_unlock(&mut (*entry).access);
        return ret;
    }

    size = (*entry).size;
    match orig {
        SEEK_SET => {}
        SEEK_CUR => {
            offset += (*file).f_pos;
        }
        SEEK_END => {
            if size == 0 {
                mutex_unlock(&mut (*entry).access);
                return -EINVAL as loff_t;
            }
            offset += size;
        }
        _ => {
            mutex_unlock(&mut (*entry).access);
            return -EINVAL as loff_t;
        }
    }
    if offset < 0 {
        mutex_unlock(&mut (*entry).access);
        return -EINVAL as loff_t;
    }
    if size != 0 && offset > size {
        offset = size;
    }
    (*file).f_pos = offset;
    mutex_unlock(&mut (*entry).access);
    offset
}

unsafe extern "C" fn snd_info_entry_read(
    file: *mut file,
    buffer: *mut c_char,
    count: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let data = (*file).private_data as *mut snd_info_private_data;
    let entry = (*data).entry;
    let mut size: size_t;
    let pos: loff_t;

    pos = *offset;
    if !valid_pos(pos, count) {
        return -EIO as ssize_t;
    }
    if pos >= (*entry).size {
        return 0;
    }
    size = ((*entry).size - pos) as size_t;
    size = core::cmp::min(count, size);
    size = (*(*entry).c.ops).read.unwrap()(
        entry,
        (*data).file_private_data,
        file,
        buffer,
        size,
        pos,
    ) as size_t;
    if (size as ssize_t) > 0 {
        *offset = pos + size as loff_t;
    }
    size as ssize_t
}

unsafe extern "C" fn snd_info_entry_write(
    file: *mut file,
    buffer: *const c_char,
    mut count: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let data = (*file).private_data as *mut snd_info_private_data;
    let entry = (*data).entry;
    let mut size: ssize_t = 0;
    let pos: loff_t;

    pos = *offset;
    if !valid_pos(pos, count) {
        return -EIO as ssize_t;
    }
    if count > 0 {
        let maxsize: size_t = ((*entry).size - pos) as size_t;
        count = core::cmp::min(count, maxsize);
        size = (*(*entry).c.ops).write.unwrap()(
            entry,
            (*data).file_private_data,
            file,
            buffer,
            count,
            pos,
        );
    }
    if size > 0 {
        *offset = pos + size as loff_t;
    }
    size
}

unsafe extern "C" fn snd_info_entry_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let data = (*file).private_data as *mut snd_info_private_data;
    let entry = (*data).entry;
    let mut mask: __poll_t = 0;

    if let Some(poll) = (*(*entry).c.ops).poll {
        return poll(entry, (*data).file_private_data, file, wait);
    }
    if (*(*entry).c.ops).read.is_some() {
        mask |= EPOLLIN | EPOLLRDNORM;
    }
    if (*(*entry).c.ops).write.is_some() {
        mask |= EPOLLOUT | EPOLLWRNORM;
    }
    mask
}

unsafe extern "C" fn snd_info_entry_ioctl(
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_long {
    let data = (*file).private_data as *mut snd_info_private_data;
    let entry = (*data).entry;

    if (*(*entry).c.ops).ioctl.is_none() {
        return -ENOTTY as c_long;
    }
    (*(*entry).c.ops).ioctl.unwrap()(entry, (*data).file_private_data, file, cmd, arg)
}

unsafe extern "C" fn snd_info_entry_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let inode = file_inode(file);
    let data: *mut snd_info_private_data;
    let entry: *mut snd_info_entry;

    data = (*file).private_data as *mut snd_info_private_data;
    if data.is_null() {
        return 0;
    }
    entry = (*data).entry;
    if (*(*entry).c.ops).mmap.is_none() {
        return -ENXIO;
    }
    (*(*entry).c.ops).mmap.unwrap()(entry, (*data).file_private_data, inode, file, vma)
}

unsafe extern "C" fn snd_info_entry_open(inode: *mut inode, file: *mut file) -> c_int {
    let entry = pde_data(inode);
    let mut data: *mut snd_info_private_data = core::ptr::null_mut();
    let mode: c_int;
    let mut err: c_int;

    mutex_lock(&mut info_mutex);
    err = alloc_info_private(entry, &mut data);
    if err < 0 {
        mutex_unlock(&mut info_mutex);
        return err;
    }

    mode = (*file).f_flags & O_ACCMODE;
    if ((mode == O_RDONLY || mode == O_RDWR) && (*(*entry).c.ops).read.is_none())
        || ((mode == O_WRONLY || mode == O_RDWR) && (*(*entry).c.ops).write.is_none())
    {
        err = -ENODEV;
        mutex_unlock(&mut info_mutex);
        kfree(data as *const c_void);
        module_put((*entry).module);
        return err;
    }

    if let Some(open) = (*(*entry).c.ops).open {
        err = open(entry, mode, &mut (*data).file_private_data);
        if err < 0 {
            mutex_unlock(&mut info_mutex);
            kfree(data as *const c_void);
            module_put((*entry).module);
            return err;
        }
    }

    (*file).private_data = data as *mut c_void;
    mutex_unlock(&mut info_mutex);
    0
}

unsafe extern "C" fn snd_info_entry_release(_inode: *mut inode, file: *mut file) -> c_int {
    let data = (*file).private_data as *mut snd_info_private_data;
    let entry = (*data).entry;

    if let Some(release) = (*(*entry).c.ops).release {
        release(entry, (*file).f_flags & O_ACCMODE, (*data).file_private_data);
    }
    module_put((*entry).module);
    kfree(data as *const c_void);
    0
}

/*
 * file ops for text proc files
 */
unsafe extern "C" fn snd_info_text_entry_write(
    file: *mut file,
    buffer: *const c_char,
    count: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let m = (*file).private_data as *mut seq_file;
    let data = (*m).private as *mut snd_info_private_data;
    let entry = (*data).entry;
    let mut buf: *mut snd_info_buffer;
    let pos: loff_t;
    let next: size_t;

    if (*entry).c.text.write.is_none() {
        return -EIO as ssize_t;
    }
    pos = *offset;
    if !valid_pos(pos, count) {
        return -EIO as ssize_t;
    }
    next = pos as size_t + count;
    /* don't handle too large text inputs */
    if next > 16 * 1024 {
        return -EIO as ssize_t;
    }
    mutex_lock(&mut (*entry).access);
    buf = (*data).wbuffer;
    if buf.is_null() {
        buf = zalloc_obj::<snd_info_buffer>();
        (*data).wbuffer = buf;
        if buf.is_null() {
            mutex_unlock(&mut (*entry).access);
            return -ENOMEM as ssize_t;
        }
    }
    if next > (*buf).len {
        let nbuf = kvzalloc(page_align(next), GFP_KERNEL) as *mut c_char;
        if nbuf.is_null() {
            mutex_unlock(&mut (*entry).access);
            return -ENOMEM as ssize_t;
        }
        kvfree((*buf).buffer as *const c_void);
        (*buf).buffer = nbuf;
        (*buf).len = page_align(next);
    }
    if copy_from_user((*buf).buffer.add(pos as usize) as *mut c_void, buffer as *const c_void, count as c_ulong) != 0 {
        mutex_unlock(&mut (*entry).access);
        return -EFAULT as ssize_t;
    }
    (*buf).size = next;
    *offset = next as loff_t;
    mutex_unlock(&mut (*entry).access);
    count as ssize_t
}

unsafe extern "C" fn snd_info_seq_show(seq: *mut seq_file, _p: *mut c_void) -> c_int {
    let data = (*seq).private as *mut snd_info_private_data;
    let entry = (*data).entry;

    if (*entry).c.text.read.is_none() {
        return -EIO;
    } else {
        (*(*data).rbuffer).buffer = seq as *mut c_char; /* XXX hack! */
        (*entry).c.text.read.unwrap()(entry, (*data).rbuffer);
    }
    0
}

unsafe extern "C" fn snd_info_text_entry_open(inode: *mut inode, file: *mut file) -> c_int {
    let entry = pde_data(inode);
    let mut data: *mut snd_info_private_data = core::ptr::null_mut();
    let mut err: c_int;

    mutex_lock(&mut info_mutex);
    err = alloc_info_private(entry, &mut data);
    if err < 0 {
        mutex_unlock(&mut info_mutex);
        return err;
    }

    (*data).rbuffer = zalloc_obj::<snd_info_buffer>();
    if (*data).rbuffer.is_null() {
        err = -ENOMEM;
        kfree((*data).rbuffer as *const c_void);
        kfree(data as *const c_void);
        module_put((*entry).module);
        mutex_unlock(&mut info_mutex);
        return err;
    }
    if (*entry).size != 0 {
        err = single_open_size(file, Some(snd_info_seq_show), data as *mut c_void, (*entry).size as size_t);
    } else {
        err = single_open(file, Some(snd_info_seq_show), data as *mut c_void);
    }
    if err < 0 {
        kfree((*data).rbuffer as *const c_void);
        kfree(data as *const c_void);
        module_put((*entry).module);
        mutex_unlock(&mut info_mutex);
        return err;
    }
    mutex_unlock(&mut info_mutex);
    0
}

unsafe extern "C" fn snd_info_text_entry_release(inode: *mut inode, file: *mut file) -> c_int {
    let m = (*file).private_data as *mut seq_file;
    let data = (*m).private as *mut snd_info_private_data;
    let entry = (*data).entry;

    if !(*data).wbuffer.is_null() && (*entry).c.text.write.is_some() {
        (*entry).c.text.write.unwrap()(entry, (*data).wbuffer);
    }

    single_release(inode, file);
    kfree((*data).rbuffer as *const c_void);
    if !(*data).wbuffer.is_null() {
        kvfree((*(*data).wbuffer).buffer as *const c_void);
        kfree((*data).wbuffer as *const c_void);
    }

    module_put((*entry).module);
    kfree(data as *const c_void);
    0
}

unsafe extern "C" fn create_subdir(mod_: *mut module, name: *const c_char) -> *mut snd_info_entry {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_module_entry(mod_, name, core::ptr::null_mut());
    if entry.is_null() {
        return core::ptr::null_mut();
    }
    (*entry).mode = S_IFDIR | 0o555;
    if snd_info_register(entry) < 0 {
        snd_info_free_entry(entry);
        return core::ptr::null_mut();
    }
    entry
}

#[no_mangle]
pub unsafe extern "C" fn snd_info_init() -> c_int {
    snd_proc_root = snd_info_create_entry(
        b"asound\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
        THIS_MODULE,
    );
    if snd_proc_root.is_null() {
        return -ENOMEM;
    }
    (*snd_proc_root).mode = S_IFDIR | 0o555;
    (*snd_proc_root).p = proc_mkdir(b"asound\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if (*snd_proc_root).p.is_null() {
        snd_info_free_entry(snd_proc_root);
        return -ENOMEM;
    }
    // CONFIG_SND_OSSEMUL: create snd_oss_root = create_subdir(THIS_MODULE, "oss").
    // IS_ENABLED(CONFIG_SND_SEQUENCER):
    snd_seq_root = create_subdir(THIS_MODULE, b"seq\0".as_ptr() as *const c_char);
    if snd_seq_root.is_null() {
        snd_info_free_entry(snd_proc_root);
        return -ENOMEM;
    }
    if snd_info_version_init() < 0
        || snd_minor_info_init() < 0
        || snd_minor_info_oss_init() < 0
        || snd_card_info_init() < 0
        || snd_info_minor_register() < 0
    {
        snd_info_free_entry(snd_proc_root);
        return -ENOMEM;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_info_done() -> c_int {
    snd_info_free_entry(snd_proc_root);
    0
}

unsafe extern "C" fn snd_card_id_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let card = (*entry).private_data as *mut snd_card;

    snd_iprintf(buffer, b"%s\n\0".as_ptr() as *const c_char, (*card).id);
}

/*
 * create a card proc file
 * called from init.c
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_card_create(card: *mut snd_card) -> c_int {
    let mut str_: [c_char; 8] = [0; 8];
    let entry: *mut snd_info_entry;

    if snd_BUG_ON(card.is_null()) {
        return -ENXIO;
    }

    sprintf(str_.as_mut_ptr(), b"card%i\0".as_ptr() as *const c_char, (*card).number);
    entry = create_subdir((*card).module, str_.as_ptr());
    if entry.is_null() {
        return -ENOMEM;
    }
    (*card).proc_root = entry;

    snd_card_ro_proc_new(card, b"id\0".as_ptr() as *const c_char, card as *mut c_void, Some(snd_card_id_read))
}

/*
 * register the card proc file
 * called from init.c
 * can be called multiple times for reinitialization
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_card_register(card: *mut snd_card) -> c_int {
    let p: *mut proc_dir_entry;
    let err: c_int;

    if snd_BUG_ON(card.is_null()) {
        return -ENXIO;
    }

    err = snd_info_register((*card).proc_root);
    if err < 0 {
        return err;
    }

    if strcmp((*card).id, (*(*card).proc_root).name) == 0 {
        return 0;
    }

    if !(*card).proc_root_link.is_null() {
        return 0;
    }
    p = proc_symlink((*card).id, (*snd_proc_root).p, (*(*card).proc_root).name);
    if p.is_null() {
        return -ENOMEM;
    }
    (*card).proc_root_link = p;
    0
}

/*
 * called on card->id change
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_card_id_change(card: *mut snd_card) {
    mutex_lock(&mut info_mutex);
    if !(*card).proc_root_link.is_null() {
        proc_remove((*card).proc_root_link);
        (*card).proc_root_link = core::ptr::null_mut();
    }
    if strcmp((*card).id, (*(*card).proc_root).name) != 0 {
        (*card).proc_root_link = proc_symlink((*card).id, (*snd_proc_root).p, (*(*card).proc_root).name);
    }
    mutex_unlock(&mut info_mutex);
}

/*
 * de-register the card proc file
 * called from init.c
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_card_disconnect(card: *mut snd_card) {
    if card.is_null() {
        return;
    }

    proc_remove((*card).proc_root_link);
    if !(*card).proc_root.is_null() {
        proc_remove((*(*card).proc_root).p);
    }

    mutex_lock(&mut info_mutex);
    if !(*card).proc_root.is_null() {
        snd_info_clear_entries((*card).proc_root);
    }
    (*card).proc_root_link = core::ptr::null_mut();
    (*card).proc_root = core::ptr::null_mut();
    mutex_unlock(&mut info_mutex);
}

/*
 * release the card proc file resources
 * called from init.c
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_card_free(card: *mut snd_card) -> c_int {
    if card.is_null() {
        return 0;
    }
    snd_info_free_entry((*card).proc_root);
    (*card).proc_root = core::ptr::null_mut();
    0
}

/**
 * snd_info_get_line - read one line from the procfs buffer
 * @buffer: the procfs buffer
 * @line: the buffer to store
 * @len: the max. buffer size
 *
 * Reads one line from the buffer and stores the string.
 *
 * Return: Zero if successful, or 1 if error or EOF.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_get_line(
    buffer: *mut snd_info_buffer,
    mut line: *mut c_char,
    mut len: c_int,
) -> c_int {
    let mut c: c_int;

    if snd_BUG_ON(buffer.is_null()) {
        return 1;
    }
    if (*buffer).buffer.is_null() {
        return 1;
    }
    if len <= 0 || (*buffer).stop != 0 || (*buffer).error != 0 {
        return 1;
    }
    while (*buffer).stop == 0 {
        c = *(*buffer).buffer.add((*buffer).curr) as c_int;
        (*buffer).curr += 1;
        if (*buffer).curr >= (*buffer).size {
            (*buffer).stop = 1;
        }
        if c == b'\n' as c_int {
            break;
        }
        if len > 1 {
            len -= 1;
            *line = c as c_char;
            line = line.add(1);
        }
    }
    *line = 0;
    0
}

/**
 * snd_info_get_str - parse a string token
 * @dest: the buffer to store the string token
 * @src: the original string
 * @len: the max. length of token - 1
 *
 * Parses the original string and copy a token to the given
 * string buffer.
 *
 * Return: The updated pointer of the original string so that
 * it can be used for the next call.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_get_str(
    mut dest: *mut c_char,
    mut src: *const c_char,
    mut len: c_int,
) -> *const c_char {
    let c: c_int;

    while *src == b' ' as c_char || *src == b'\t' as c_char {
        src = src.add(1);
    }
    if *src == b'"' as c_char || *src == b'\'' as c_char {
        c = *src as c_int;
        src = src.add(1);
        loop {
            len -= 1;
            if !(len > 0 && *src != 0 && *src as c_int != c) {
                break;
            }
            *dest = *src;
            dest = dest.add(1);
            src = src.add(1);
        }
        if *src as c_int == c {
            src = src.add(1);
        }
    } else {
        loop {
            len -= 1;
            if !(len > 0 && *src != 0 && *src != b' ' as c_char && *src != b'\t' as c_char) {
                break;
            }
            *dest = *src;
            dest = dest.add(1);
            src = src.add(1);
        }
    }
    *dest = 0;
    while *src == b' ' as c_char || *src == b'\t' as c_char {
        src = src.add(1);
    }
    src
}

/*
 * snd_info_create_entry - create an info entry
 * @name: the proc file name
 * @parent: the parent directory
 *
 * Creates an info entry with the given file name and initializes as
 * the default state.
 *
 * Usually called from other functions such as
 * snd_info_create_card_entry().
 *
 * Return: The pointer of the new instance, or %NULL on failure.
 */
unsafe extern "C" fn snd_info_create_entry(
    name: *const c_char,
    parent: *mut snd_info_entry,
    module: *mut module,
) -> *mut snd_info_entry {
    let entry: *mut snd_info_entry;
    entry = zalloc_obj::<snd_info_entry>();
    if entry.is_null() {
        return core::ptr::null_mut();
    }
    (*entry).name = kstrdup(name, GFP_KERNEL);
    if (*entry).name.is_null() {
        kfree(entry as *const c_void);
        return core::ptr::null_mut();
    }
    (*entry).mode = S_IFREG | 0o444;
    (*entry).content = SNDRV_INFO_CONTENT_TEXT;
    mutex_init(&mut (*entry).access);
    init_list_head(&mut (*entry).children);
    init_list_head(&mut (*entry).list);
    (*entry).parent = parent;
    (*entry).module = module;
    if !parent.is_null() {
        mutex_lock(&mut (*parent).access);
        list_add_tail(&mut (*entry).list, &mut (*parent).children);
        mutex_unlock(&mut (*parent).access);
    }
    entry
}

/**
 * snd_info_create_module_entry - create an info entry for the given module
 * @module: the module pointer
 * @name: the file name
 * @parent: the parent directory
 *
 * Creates a new info entry and assigns it to the given module.
 *
 * Return: The pointer of the new instance, or %NULL on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_create_module_entry(
    module: *mut module,
    name: *const c_char,
    mut parent: *mut snd_info_entry,
) -> *mut snd_info_entry {
    if parent.is_null() {
        parent = snd_proc_root;
    }
    snd_info_create_entry(name, parent, module)
}

/**
 * snd_info_create_card_entry - create an info entry for the given card
 * @card: the card instance
 * @name: the file name
 * @parent: the parent directory
 *
 * Creates a new info entry and assigns it to the given card.
 *
 * Return: The pointer of the new instance, or %NULL on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_create_card_entry(
    card: *mut snd_card,
    name: *const c_char,
    mut parent: *mut snd_info_entry,
) -> *mut snd_info_entry {
    if parent.is_null() {
        parent = (*card).proc_root;
    }
    snd_info_create_entry(name, parent, (*card).module)
}

unsafe extern "C" fn snd_info_clear_entries(entry: *mut snd_info_entry) {
    if (*entry).p.is_null() {
        return;
    }
    // C list_for_each_entry(p, &entry->children, list):
    // recursively call snd_info_clear_entries(p) for each child.
    (*entry).p = core::ptr::null_mut();
}

/**
 * snd_info_free_entry - release the info entry
 * @entry: the info entry
 *
 * Releases the info entry.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_free_entry(entry: *mut snd_info_entry) {
    let p: *mut snd_info_entry;

    if entry.is_null() {
        return;
    }
    if !(*entry).p.is_null() {
        proc_remove((*entry).p);
        mutex_lock(&mut info_mutex);
        snd_info_clear_entries(entry);
        mutex_unlock(&mut info_mutex);
    }

    /* free all children at first */
    // C list_for_each_entry_safe(p, n, &entry->children, list):
    // recursively call snd_info_free_entry(p) for each child.

    p = (*entry).parent;
    if !p.is_null() {
        mutex_lock(&mut (*p).access);
        list_del(&mut (*entry).list);
        mutex_unlock(&mut (*p).access);
    }
    kfree((*entry).name as *const c_void);
    if let Some(private_free) = (*entry).private_free {
        private_free(entry);
    }
    kfree(entry as *const c_void);
}

unsafe extern "C" fn __snd_info_register(entry: *mut snd_info_entry) -> c_int {
    let root: *mut proc_dir_entry;
    let mut p: *mut proc_dir_entry = core::ptr::null_mut();

    if snd_BUG_ON(entry.is_null()) {
        return -ENXIO;
    }
    root = if (*entry).parent.is_null() {
        (*snd_proc_root).p
    } else {
        (*(*entry).parent).p
    };
    mutex_lock(&mut info_mutex);
    if !(*entry).p.is_null() || root.is_null() {
        mutex_unlock(&mut info_mutex);
        return 0;
    }
    if s_isdir((*entry).mode) {
        p = proc_mkdir_mode((*entry).name, (*entry).mode, root);
        if p.is_null() {
            mutex_unlock(&mut info_mutex);
            return -ENOMEM;
        }
    } else {
        let ops: *const proc_ops;
        if (*entry).content == SNDRV_INFO_CONTENT_DATA {
            ops = &snd_info_entry_operations;
        } else {
            ops = &snd_info_text_entry_ops;
        }
        p = proc_create_data((*entry).name, (*entry).mode, root, ops, entry as *mut c_void);
        if p.is_null() {
            mutex_unlock(&mut info_mutex);
            return -ENOMEM;
        }
        proc_set_size(p, (*entry).size);
    }
    (*entry).p = p;
    mutex_unlock(&mut info_mutex);
    0
}

/**
 * snd_info_register - register the info entry
 * @entry: the info entry
 *
 * Registers the proc info entry.
 * The all children entries are registered recursively.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_info_register(entry: *mut snd_info_entry) -> c_int {
    let mut err: c_int;

    if (*entry).p.is_null() {
        err = __snd_info_register(entry);
        if err < 0 {
            return err;
        }
    }

    // C list_for_each_entry(p, &entry->children, list):
    // err = snd_info_register(p); if (err < 0) return err;

    0
}

/**
 * snd_card_rw_proc_new - Create a read/write text proc file entry for the card
 * @card: the card instance
 * @name: the file name
 * @private_data: the arbitrary private data
 * @read: the read callback
 * @write: the write callback, NULL for read-only
 *
 * This proc file entry will be registered via snd_card_register() call, and
 * it will be removed automatically at the card removal, too.
 *
 * Return: zero if successful, or a negative error code
 */
#[no_mangle]
pub unsafe extern "C" fn snd_card_rw_proc_new(
    card: *mut snd_card,
    name: *const c_char,
    private_data: *mut c_void,
    read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) -> c_int {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_card_entry(card, name, (*card).proc_root);
    if entry.is_null() {
        return -ENOMEM;
    }
    snd_info_set_text_ops(entry, private_data, read);
    if write.is_some() {
        (*entry).mode |= 0o200;
        (*entry).c.text.write = write;
    }
    0
}

/* */

unsafe extern "C" fn snd_info_version_read(_entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_iprintf(
        buffer,
        b"Advanced Linux Sound Architecture Driver Version k%s.\n\0".as_ptr() as *const c_char,
        (*init_utsname()).release.as_ptr(),
    );
}

unsafe extern "C" fn snd_info_version_init() -> c_int {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_module_entry(
        THIS_MODULE,
        b"version\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    );
    if entry.is_null() {
        return -ENOMEM;
    }
    (*entry).c.text.read = Some(snd_info_version_read);
    snd_info_register(entry) /* freed in error path */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
