// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/map_symbol.c
// Original includes: "map_symbol.h", "maps.h", "map.h", "thread.h"

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub ms: map_symbol,
    pub addr: u64,
    pub al_addr: u64,
    pub al_level: i32,
    pub phys_addr: u64,
    pub data_page_size: u64,
}

unsafe extern "C" {
    fn thread__zput(thread: *mut thread);
    fn map__zput(map: *mut map);
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn map__get(map: *mut map) -> *mut map;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_symbol__exit(ms: *mut map_symbol) {
    unsafe {
        thread__zput((*ms).thread);
        map__zput((*ms).map);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn addr_map_symbol__exit(ams: *mut addr_map_symbol) {
    unsafe {
        map_symbol__exit(&mut (*ams).ms);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_symbol__copy(dst: *mut map_symbol, src: *mut map_symbol) {
    unsafe {
        (*dst).thread = thread__get((*src).thread);
        (*dst).map = map__get((*src).map);
        (*dst).sym = (*src).sym;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn addr_map_symbol__copy(
    dst: *mut addr_map_symbol,
    src: *mut addr_map_symbol,
) {
    unsafe {
        map_symbol__copy(&mut (*dst).ms, &mut (*src).ms);

        (*dst).addr = (*src).addr;
        (*dst).al_addr = (*src).al_addr;
        (*dst).al_level = (*src).al_level;
        (*dst).phys_addr = (*src).phys_addr;
        (*dst).data_page_size = (*src).data_page_size;
    }
}
