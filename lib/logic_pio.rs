// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2017 HiSilicon Limited, All Rights Reserved.
 * Author: Gabriele Paoloni <gabriele.paoloni@huawei.com>
 * Author: Zhichang Yuan <yuanzhichang@hisilicon.com>
 * Author: John Garry <john.garry@huawei.com>
 */

// pr_fmt(fmt) = "LOGIC PIO: " fmt

// Kernel dependencies supplied by the surrounding repository.

static mut IO_RANGE_LIST: ListHead = LIST_HEAD_INIT;
static mut IO_RANGE_MUTEX: Mutex = DEFINE_MUTEX_INIT;

pub unsafe fn logic_pio_register_range(new_range: *mut logic_pio_hwaddr) -> i32 {
    let mut range: *mut logic_pio_hwaddr;
    let start: resource_size_t;
    let end: resource_size_t;
    let mut mmio_end: resource_size_t = 0;
    let mut iio_sz: resource_size_t = MMIO_UPPER_LIMIT;
    let mut ret: i32 = 0;

    if new_range.is_null()
        || (*new_range).fwnode.is_null()
        || (*new_range).size == 0
        || ((*new_range).flags == LOGIC_PIO_INDIRECT && (*new_range).ops.is_null())
    {
        return -EINVAL;
    }

    start = (*new_range).hw_start;
    end = (*new_range).hw_start.wrapping_add((*new_range).size);

    mutex_lock(&raw mut IO_RANGE_MUTEX);
    list_for_each_entry!(range, &raw mut IO_RANGE_LIST, list, {
        if (*range).fwnode == (*new_range).fwnode {
            // range already there
            ret = -EEXIST;
            goto end_register;
        }
        if (*range).flags == LOGIC_PIO_CPU_MMIO
            && (*new_range).flags == LOGIC_PIO_CPU_MMIO
        {
            // for MMIO ranges we need to check for overlap
            if start >= (*range).hw_start.wrapping_add((*range).size)
                || end < (*range).hw_start
            {
                mmio_end = (*range).io_start.wrapping_add((*range).size);
            } else {
                ret = -EFAULT;
                goto end_register;
            }
        } else if (*range).flags == LOGIC_PIO_INDIRECT
            && (*new_range).flags == LOGIC_PIO_INDIRECT
        {
            iio_sz = iio_sz.wrapping_add((*range).size);
        }
    });

    // range not registered yet, check for available space
    if (*new_range).flags == LOGIC_PIO_CPU_MMIO {
        if mmio_end.wrapping_add((*new_range).size).wrapping_sub(1) > MMIO_UPPER_LIMIT {
            // if it's too big check if 64K space can be reserved
            if mmio_end.wrapping_add(SZ_64K).wrapping_sub(1) > MMIO_UPPER_LIMIT {
                ret = -E2BIG;
                goto end_register;
            }
            (*new_range).size = SZ_64K;
            pr_warn!("Requested IO range too big, new size set to 64K\n");
        }
        (*new_range).io_start = mmio_end;
    } else if (*new_range).flags == LOGIC_PIO_INDIRECT {
        if iio_sz.wrapping_add((*new_range).size).wrapping_sub(1) > IO_SPACE_LIMIT {
            ret = -E2BIG;
            goto end_register;
        }
        (*new_range).io_start = iio_sz;
    } else {
        // invalid flag
        ret = -EINVAL;
        goto end_register;
    }

    list_add_tail_rcu!(&mut (*new_range).list, &raw mut IO_RANGE_LIST);

end_register:
    mutex_unlock(&raw mut IO_RANGE_MUTEX);
    ret
}

pub unsafe fn logic_pio_unregister_range(range: *mut logic_pio_hwaddr) {
    mutex_lock(&raw mut IO_RANGE_MUTEX);
    list_del_rcu!(&mut (*range).list);
    mutex_unlock(&raw mut IO_RANGE_MUTEX);
    synchronize_rcu();
}

pub unsafe fn find_io_range_by_fwnode(
    fwnode: *const fwnode_handle,
) -> *mut logic_pio_hwaddr {
    let mut range: *mut logic_pio_hwaddr;
    let mut found_range: *mut logic_pio_hwaddr = core::ptr::null_mut();

    rcu_read_lock();
    list_for_each_entry_rcu!(range, &raw mut IO_RANGE_LIST, list, {
        if (*range).fwnode == fwnode {
            found_range = range;
            break;
        }
    });
    rcu_read_unlock();
    found_range
}

// Return a registered range given an input PIO token.
unsafe fn find_io_range(pio: usize) -> *mut logic_pio_hwaddr {
    let mut range: *mut logic_pio_hwaddr;
    let mut found_range: *mut logic_pio_hwaddr = core::ptr::null_mut();

    rcu_read_lock();
    list_for_each_entry_rcu!(range, &raw mut IO_RANGE_LIST, list, {
        if in_range(pio, (*range).io_start, (*range).size) {
            found_range = range;
            break;
        }
    });
    rcu_read_unlock();

    if found_range.is_null() {
        pr_err!("PIO entry token 0x%lx invalid\n", pio);
    }
    found_range
}

pub unsafe fn logic_pio_to_hwaddr(pio: usize) -> resource_size_t {
    let range = find_io_range(pio);
    if !range.is_null() {
        return (*range).hw_start.wrapping_add(
            (pio as resource_size_t).wrapping_sub((*range).io_start),
        );
    }
    !0 as resource_size_t
}

pub unsafe fn logic_pio_trans_hwaddr(
    fwnode: *const fwnode_handle,
    addr: resource_size_t,
    size: resource_size_t,
) -> usize {
    let range = find_io_range_by_fwnode(fwnode);
    if range.is_null() || (*range).flags == LOGIC_PIO_CPU_MMIO {
        pr_err!("IO range not found or invalid\n");
        return !0usize;
    }
    if (*range).size < size {
        pr_err!("resource size %pa cannot fit in IO range size %pa\n", &size, &(*range).size);
        return !0usize;
    }
    addr.wrapping_sub((*range).hw_start)
        .wrapping_add((*range).io_start) as usize
}

pub unsafe fn logic_pio_trans_cpuaddr(addr: resource_size_t) -> usize {
    let mut range: *mut logic_pio_hwaddr;
    rcu_read_lock();
    list_for_each_entry_rcu!(range, &raw mut IO_RANGE_LIST, list, {
        if (*range).flags != LOGIC_PIO_CPU_MMIO {
            continue;
        }
        if in_range(addr, (*range).hw_start, (*range).size) {
            let cpuaddr = addr.wrapping_sub((*range).hw_start)
                .wrapping_add((*range).io_start) as usize;
            rcu_read_unlock();
            return cpuaddr;
        }
    });
    rcu_read_unlock();
    pr_err!("addr %pa not registered in io_range_list\n", &addr);
    !0usize
}

// The following helpers correspond to BUILD_LOGIC_IO and are present only
// when CONFIG_INDIRECT_PIO and PCI_IOBASE are enabled in the C build.
#[cfg(all(feature = "CONFIG_INDIRECT_PIO", feature = "PCI_IOBASE"))]
pub unsafe fn logic_inb(addr: usize) -> u8 { logic_in::<u8>(addr) }
#[cfg(all(feature = "CONFIG_INDIRECT_PIO", feature = "PCI_IOBASE"))]
pub unsafe fn logic_inw(addr: usize) -> u16 { logic_in::<u16>(addr) }
#[cfg(all(feature = "CONFIG_INDIRECT_PIO", feature = "PCI_IOBASE"))]
pub unsafe fn logic_inl(addr: usize) -> u32 { logic_in::<u32>(addr) }

#[cfg(all(feature = "CONFIG_INDIRECT_PIO", feature = "PCI_IOBASE"))]
unsafe fn logic_in<T: Copy>(addr: usize) -> T {
    let mut ret: T = core::mem::zeroed();
    if addr < MMIO_UPPER_LIMIT {
        ret = _in::<T>(addr);
    } else if addr < IO_SPACE_LIMIT {
        let entry = find_io_range(addr);
        if !entry.is_null() {
            ret = ((*(*entry).ops).in_)((*entry).hostdata, addr, core::mem::size_of::<T>())
                as T;
        } else {
            WARN_ON_ONCE!(true);
        }
    }
    ret
}

// logic_out*, logic_ins*, and logic_outs* are direct Rust expansions of the
// remaining BUILD_LOGIC_IO macro bodies and use the external kernel I/O APIs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
