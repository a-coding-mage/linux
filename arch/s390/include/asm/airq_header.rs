/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 2002, 2007
 *    Author(s): Ingo Adlung <adlung@de.ibm.com>
 *             Cornelia Huck <cornelia.huck@de.ibm.com>
 *             Arnd Bergmann <arndb@de.ibm.com>
 *             Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
 */

// Dependencies supplied by other headers: hlist_node, tpi_info, dma_addr_t,
// spinlock_t, BITS_PER_LONG, bit_spin_lock, and bit_spin_unlock.

#[repr(C)]
pub struct airq_struct {
    pub list: hlist_node, // Handler queueing.
    pub handler: Option<unsafe extern "C" fn(airq: *mut airq_struct, tpi_info: *mut tpi_info)>,
    pub lsi_ptr: *mut u8, // Local-Summary-Indicator pointer
    pub isc: u8,          // Interrupt-subclass
    pub flags: u8,
}

pub const AIRQ_PTR_ALLOCATED: u8 = 0x01;

unsafe extern "C" {
    pub fn register_adapter_interrupt(airq: *mut airq_struct) -> i32;
    pub fn unregister_adapter_interrupt(airq: *mut airq_struct);
}

/* Adapter interrupt bit vector */
#[repr(C)]
pub struct airq_iv {
    pub vector: *mut c_ulong,     // Adapter interrupt bit vector
    pub vector_dma: dma_addr_t,  // Adapter interrupt bit vector dma
    pub avail: *mut c_ulong,      // Allocation bit mask for the bit vector
    pub bitlock: *mut c_ulong,    // Lock bit mask for the bit vector
    pub ptr: *mut c_ulong,        // Pointer associated with each bit
    pub data: *mut c_uint,        // 32 bit value associated with each bit
    pub bits: c_ulong,            // Number of bits in the vector
    pub end: c_ulong,             // Number of highest allocated bit + 1
    pub flags: c_ulong,           // Allocation flags
    pub lock: spinlock_t,         // Lock to protect alloc & free
}

pub const AIRQ_IV_ALLOC: c_ulong = 1;       // Use an allocation bit mask
pub const AIRQ_IV_BITLOCK: c_ulong = 2;     // Allocate the lock bit mask
pub const AIRQ_IV_PTR: c_ulong = 4;         // Allocate the ptr array
pub const AIRQ_IV_DATA: c_ulong = 8;        // Allocate the data array
pub const AIRQ_IV_CACHELINE: c_ulong = 16; // Cacheline alignment for the vector
pub const AIRQ_IV_GUESTVEC: c_ulong = 32;   // Vector is a pinned guest page

unsafe extern "C" {
    pub fn airq_iv_create(bits: c_ulong, flags: c_ulong, vec: *mut c_ulong) -> *mut airq_iv;
    pub fn airq_iv_release(iv: *mut airq_iv);
    pub fn airq_iv_alloc(iv: *mut airq_iv, num: c_ulong) -> c_ulong;
    pub fn airq_iv_free(iv: *mut airq_iv, bit: c_ulong, num: c_ulong);
    pub fn airq_iv_scan(iv: *mut airq_iv, start: c_ulong, end: c_ulong) -> c_ulong;
    pub fn bit_spin_lock(bit: c_ulong, addr: *mut c_ulong);
    pub fn bit_spin_unlock(bit: c_ulong, addr: *mut c_ulong);
}

#[inline]
pub unsafe fn airq_iv_alloc_bit(iv: *mut airq_iv) -> c_ulong {
    airq_iv_alloc(iv, 1)
}

#[inline]
pub unsafe fn airq_iv_free_bit(iv: *mut airq_iv, bit: c_ulong) {
    airq_iv_free(iv, bit, 1);
}

#[inline]
pub unsafe fn airq_iv_end(iv: *mut airq_iv) -> c_ulong {
    (*iv).end
}

#[inline]
pub unsafe fn airq_iv_lock(iv: *mut airq_iv, bit: c_ulong) {
    let be_to_le: c_ulong = BITS_PER_LONG - 1;
    bit_spin_lock(bit ^ be_to_le, (*iv).bitlock);
}

#[inline]
pub unsafe fn airq_iv_unlock(iv: *mut airq_iv, bit: c_ulong) {
    let be_to_le: c_ulong = BITS_PER_LONG - 1;
    bit_spin_unlock(bit ^ be_to_le, (*iv).bitlock);
}

#[inline]
pub unsafe fn airq_iv_set_data(iv: *mut airq_iv, bit: c_ulong, data: c_uint) {
    *(*iv).data.add(bit as usize) = data;
}

#[inline]
pub unsafe fn airq_iv_get_data(iv: *mut airq_iv, bit: c_ulong) -> c_uint {
    *(*iv).data.add(bit as usize)
}

#[inline]
pub unsafe fn airq_iv_set_ptr(iv: *mut airq_iv, bit: c_ulong, ptr: c_ulong) {
    *(*iv).ptr.add(bit as usize) = ptr;
}

#[inline]
pub unsafe fn airq_iv_get_ptr(iv: *mut airq_iv, bit: c_ulong) -> c_ulong {
    *(*iv).ptr.add(bit as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
