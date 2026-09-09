// SPDX-License-Identifier: GPL-2.0

// C tracepoint include: <linux/tracepoint.h>
// The TRACE_SYSTEM is ext2.

#[repr(C)]
pub struct Ext2DioClassEntry {
    pub dev: dev_t,
    pub ino: u64,
    pub isize: loff_t,
    pub pos: loff_t,
    pub count: usize,
    pub ki_flags: i32,
    pub aio: bool,
    pub ret: ssize_t,
}

#[repr(C)]
pub struct Ext2DioWriteEndioEntry {
    pub dev: dev_t,
    pub ino: u64,
    pub isize: loff_t,
    pub pos: loff_t,
    pub size: ssize_t,
    pub ki_flags: i32,
    pub aio: bool,
    pub ret: i32,
}

// DECLARE_EVENT_CLASS(ext2_dio_class,
//     TP_PROTO(struct kiocb *iocb, struct iov_iter *iter, ssize_t ret),
//     TP_ARGS(iocb, iter, ret),
//     TP_fast_assign:
//         dev = file_inode(iocb->ki_filp)->i_sb->s_dev;
//         ino = file_inode(iocb->ki_filp)->i_ino;
//         isize = file_inode(iocb->ki_filp)->i_size;
//         pos = iocb->ki_pos;
//         count = iov_iter_count(iter);
//         ki_flags = iocb->ki_flags;
//         aio = !is_sync_kiocb(iocb);
//         ret = ret;
//     TP_printk("dev %d:%d ino 0x%llx isize 0x%llx pos 0x%llx len %zu flags %s aio %d ret %zd",
//         MAJOR(dev), MINOR(dev), ino, isize, pos, count,
//         __print_flags(ki_flags, "|", TRACE_IOCB_STRINGS), aio, ret)
// );

// DEFINE_DIO_RW_EVENT(name) expands to DEFINE_EVENT(ext2_dio_class, name,
// TP_PROTO(struct kiocb *iocb, struct iov_iter *iter, ssize_t ret),
// TP_ARGS(iocb, iter, ret)).
pub const EXT2_DIO_EVENTS: &[&str] = &[
    "ext2_dio_write_begin",
    "ext2_dio_write_end",
    "ext2_dio_write_buff_end",
    "ext2_dio_read_begin",
    "ext2_dio_read_end",
];

// TRACE_EVENT(ext2_dio_write_endio,
//     TP_PROTO(struct kiocb *iocb, ssize_t size, int ret),
//     TP_ARGS(iocb, size, ret),
//     TP_fast_assign:
//         dev = file_inode(iocb->ki_filp)->i_sb->s_dev;
//         ino = file_inode(iocb->ki_filp)->i_ino;
//         isize = file_inode(iocb->ki_filp)->i_size;
//         pos = iocb->ki_pos;
//         size = size;
//         ki_flags = iocb->ki_flags;
//         aio = !is_sync_kiocb(iocb);
//         ret = ret;
//     TP_printk("dev %d:%d ino 0x%llx isize 0x%llx pos 0x%llx len %zd flags %s aio %d ret %d",
//         MAJOR(dev), MINOR(dev), ino, isize, pos, size,
//         __print_flags(ki_flags, "|", TRACE_IOCB_STRINGS), aio, ret)
// );

// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE trace
// <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
