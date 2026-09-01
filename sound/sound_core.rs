// SPDX-License-Identifier: GPL-2.0-or-later
//
// Sound core.  This file is composed of two parts.  sound_class
// which is common to both OSS and ALSA and OSS sound core which
// is used OSS or emulation of it.
//
// Linux kernel sound module - requires linux-sys or kernel bindings.

// First, the common part.

#[cfg(feature = "CONFIG_SOUND_OSS_CORE")]
extern "C" {
    fn init_oss_soundcore() -> i32;
    fn cleanup_oss_soundcore();
}

#[cfg(not(feature = "CONFIG_SOUND_OSS_CORE"))]
unsafe extern "C" fn init_oss_soundcore() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SOUND_OSS_CORE"))]
unsafe extern "C" fn cleanup_oss_soundcore() {}

// MODULE_DESCRIPTION("Core sound module");
// MODULE_AUTHOR("Alan Cox");
// MODULE_LICENSE("GPL");

extern "C" {
    type device;
    type class;

    fn MAJOR(devt: u32) -> u32;
    fn kasprintf(flags: i32, fmt: *const u8, ...) -> *mut u8;
    fn dev_name(dev: *const device) -> *const u8;
    fn class_register(class: *const class) -> i32;
    fn class_unregister(class: *const class);
    fn IS_ENABLED(x: i32) -> i32;

    static mut sound_class: class;
}

const SOUND_MAJOR: u32 = 1;

unsafe extern "C" fn sound_devnode(dev: *const device, mode: *mut u32) -> *mut u8 {
    if MAJOR((*dev).devt) == SOUND_MAJOR {
        return std::ptr::null_mut();
    }
    kasprintf(0, "snd/%s\0".as_ptr(), dev_name(dev))
}

#[repr(C)]
pub struct FileOperations {
    owner: *mut std::ffi::c_void,
    open: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32>,
    llseek: Option<unsafe extern "C" fn() -> i32>,
}

#[repr(C)]
pub struct SoundClass {
    name: *const u8,
    devnode: Option<unsafe extern "C" fn(*const device, *mut u32) -> *mut u8>,
}

extern "C" {
    static THIS_MODULE: *mut std::ffi::c_void;
    fn noop_llseek() -> i32;
}

const sound_class_value: SoundClass = SoundClass {
    name: "sound\0".as_ptr(),
    devnode: Some(sound_devnode),
};

// EXPORT_SYMBOL(sound_class);

unsafe extern "C" fn init_soundcore() -> i32 {
    let mut rc: i32;

    rc = init_oss_soundcore();
    if rc != 0 {
        return rc;
    }

    rc = class_register(&sound_class);
    if rc != 0 {
        cleanup_oss_soundcore();
        return rc;
    }

    0
}

unsafe extern "C" fn cleanup_soundcore() {
    cleanup_oss_soundcore();
    class_unregister(&sound_class);
}

// subsys_initcall(init_soundcore);
// module_exit(cleanup_soundcore);

#[cfg(feature = "CONFIG_SOUND_OSS_CORE")]
mod oss_core {
    use std::ffi::CStr;
    use std::ptr;

    // OSS sound core handling. Breaks out sound functions to submodules
    //
    // Author:		Alan Cox <alan@lxorguk.ukuu.org.uk>
    //
    // Fixes:
    //
    //                         --------------------
    //
    // Top level handler for the sound subsystem. Various devices can
    // plug into this. The fact they don't all go via OSS doesn't mean
    // they don't have to implement the OSS API. There is a lot of logic
    // to keeping much of the OSS weight out of the code in a compatibility
    // module, but it's up to the driver to rember to load it...
    //
    // The code provides a set of functions for registration of devices
    // by type. This is done rather than providing a single call so that
    // we can hide any future changes in the internals (eg when we go to
    // 32bit dev_t) from the modules and their interface.
    //
    // Secondly we need to allocate the dsp, dsp16 and audio devices as
    // one. Thus we misuse the chains a bit to simplify this.
    //
    // Thirdly to make it more fun and for 2.3.x and above we do all
    // of this using fine grained locking.
    //
    // FIXME: we have to resolve modules and fine grained load/unload
    // locking at some point in 2.3.x.

    extern "C" {
        type file_operations;
        type inode;
        type file;
        type device;
        type spinlock_t;

        fn kmalloc_obj(obj: *mut SoundUnit) -> *mut SoundUnit;
        fn kfree(ptr: *mut std::ffi::c_void);
        fn spin_lock(lock: *mut spinlock_t);
        fn spin_unlock(lock: *mut spinlock_t);
        fn __register_chrdev(major: u32, minor: u32, count: u32, name: *const u8, fops: *const FileOperations) -> i32;
        fn __unregister_chrdev(major: u32, minor: u32, count: u32, name: *const u8);
        fn device_create(class: *mut std::ffi::c_void, parent: *mut device, devt: u32, drvdata: *mut std::ffi::c_void, fmt: *const u8, ...);
        fn device_destroy(class: *mut std::ffi::c_void, devt: u32);
        fn sprintf(s: *mut u8, format: *const u8, ...) -> i32;
        fn strcat(dest: *mut u8, src: *const u8) -> *mut u8;
        fn printk(fmt: *const u8, ...);
        fn request_module(fmt: *const u8, ...);
        fn fops_get(fops: *const file_operations) -> *const file_operations;
        fn replace_fops(file: *mut file, fops: *const file_operations);
        fn iminor(inode: *const inode) -> u32;
        fn unregister_chrdev(major: u32, name: *const u8);
        fn register_chrdev(major: u32, name: *const u8, fops: *const FileOperations) -> i32;
        fn MKDEV(major: u32, minor: u32) -> u32;

        static mut sound_class: std::ffi::c_void;
    }

    const SOUND_STEP: usize = 16;
    const SOUND_MAJOR: u32 = 1;

    #[repr(C)]
    pub struct SoundUnit {
        unit_minor: i32,
        unit_fops: *const file_operations,
        next: *mut SoundUnit,
        name: [u8; 32],
    }

    #[repr(C)]
    pub struct FileOperations {
        owner: *mut std::ffi::c_void,
        open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
        llseek: Option<unsafe extern "C" fn() -> i32>,
    }

    // By default, OSS sound_core claims full legacy minor range (0-255)
    // of SOUND_MAJOR to trap open attempts to any sound minor and
    // requests modules using custom sound-slot/service-* module aliases.
    // The only benefit of doing this is allowing use of custom module
    // aliases instead of the standard char-major-* ones.  This behavior
    // prevents alternative OSS implementation and is scheduled to be
    // removed.
    //
    // CONFIG_SOUND_OSS_CORE_PRECLAIM and soundcore.preclaim_oss kernel
    // parameter are added to allow distros and developers to try and
    // switch to alternative implementations without needing to rebuild
    // the kernel in the meantime.  If preclaim_oss is non-zero, the
    // kernel will behave the same as before.  All SOUND_MAJOR minors are
    // preclaimed and the custom module aliases along with standard chrdev
    // ones are emitted if a missing device is opened.  If preclaim_oss is
    // zero, sound_core only grabs what's actually in use and for missing
    // devices only the standard chrdev aliases are requested.
    //
    // All these clutters are scheduled to be removed along with
    // sound-slot/service-* module aliases.

    static mut preclaim_oss: i32 = 0;

    // module_param(preclaim_oss, int, 0444);

    extern "C" {
        static THIS_MODULE: *mut std::ffi::c_void;
        fn noop_llseek() -> i32;
    }

    unsafe extern "C" fn soundcore_open(inode: *mut inode, file: *mut file) -> i32;

    static soundcore_fops: FileOperations = FileOperations {
        owner: std::ptr::null_mut(),
        open: Some(soundcore_open),
        llseek: Some(noop_llseek),
    };

    // Low level list operator. Scan the ordered list, find a hole and
    // join into it. Called with the lock asserted

    unsafe fn __sound_insert_unit(s: *mut SoundUnit, list: *mut *mut SoundUnit, fops: *const file_operations, index: i32, low: i32, top: i32) -> i32 {
        let mut n = low;
        let mut list_ptr = list;

        if index < 0 {
            // first free
            while !(*list_ptr).is_null() && (*(*list_ptr)).unit_minor < n {
                list_ptr = &mut (*(*list_ptr)).next;
            }

            while n < top {
                // Found a hole ?
                if (*list_ptr).is_null() || (*(*list_ptr)).unit_minor > n {
                    break;
                }
                list_ptr = &mut (*(*list_ptr)).next;
                n += SOUND_STEP as i32;
            }

            if n >= top {
                return -2; // -ENOENT
            }
        } else {
            n = low + (index * 16);
            while !(*list_ptr).is_null() {
                if (*(*list_ptr)).unit_minor == n {
                    return -16; // -EBUSY
                }
                if (*(*list_ptr)).unit_minor > n {
                    break;
                }
                list_ptr = &mut (*(*list_ptr)).next;
            }
        }

        // Fill it in
        (*s).unit_minor = n;
        (*s).unit_fops = fops;

        // Link it
        (*s).next = *list_ptr;
        *list_ptr = s;

        n
    }

    // Remove a node from the chain. Called with the lock asserted

    unsafe fn __sound_remove_unit(list: *mut *mut SoundUnit, unit: i32) -> *mut SoundUnit {
        let mut list_ptr = list;
        while !(*list_ptr).is_null() {
            let p = *list_ptr;
            if (*p).unit_minor == unit {
                *list_ptr = (*p).next;
                return p;
            }
            list_ptr = &mut (*p).next;
        }
        printk("Sound device %d went missing!\n\0".as_ptr(), unit);
        ptr::null_mut()
    }

    // This lock guards the sound loader list.

    static mut sound_loader_lock: spinlock_t = unsafe { std::mem::zeroed() };

    // Allocate the controlling structure and add it to the sound driver
    // list. Acquires locks as needed

    unsafe fn sound_insert_unit(list: *mut *mut SoundUnit, fops: *const file_operations, index: i32, low: i32, top: i32, name: *const u8, _mode: u32, dev: *mut device) -> i32 {
        let s: *mut SoundUnit = kmalloc_obj(ptr::null_mut());
        let mut r: i32;

        if s.is_null() {
            return -12; // -ENOMEM
        }

        spin_lock(&mut sound_loader_lock);
        loop {
            r = __sound_insert_unit(s, list, fops, index, low, top);
            spin_unlock(&mut sound_loader_lock);

            if r < 0 {
                break;
            } else if r < SOUND_STEP as i32 {
                sprintf((*s).name.as_mut_ptr(), "sound/%s\0".as_ptr(), name);
            } else {
                sprintf((*s).name.as_mut_ptr(), "sound/%s%d\0".as_ptr(), name, r / SOUND_STEP as i32);
            }

            if preclaim_oss == 0 {
                // Something else might have grabbed the minor.  If
                // first free slot is requested, rescan with @low set
                // to the next unit; otherwise, -EBUSY.
                r = __register_chrdev(SOUND_MAJOR, (*s).unit_minor as u32, 1, (*s).name.as_ptr(), &soundcore_fops);
                if r < 0 {
                    spin_lock(&mut sound_loader_lock);
                    __sound_remove_unit(list, (*s).unit_minor);
                    if index < 0 {
                        low = (*s).unit_minor + SOUND_STEP as i32;
                    } else {
                        spin_unlock(&mut sound_loader_lock);
                        r = -16; // -EBUSY
                        break;
                    }
                    // Continue retry loop
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if r >= 0 {
            device_create(&mut sound_class, dev, MKDEV(SOUND_MAJOR, (*s).unit_minor as u32), ptr::null_mut(), "%s\0".as_ptr(), (*s).name.as_ptr().add(6));
            return (*s).unit_minor;
        }

        // fail:
        kfree(s as *mut std::ffi::c_void);
        r
    }

    // Remove a unit. Acquires locks as needed. The drivers MUST have
    // completed the removal before their file operations become
    // invalid.

    unsafe fn sound_remove_unit(list: *mut *mut SoundUnit, unit: i32) {
        spin_lock(&mut sound_loader_lock);
        let p = __sound_remove_unit(list, unit);
        spin_unlock(&mut sound_loader_lock);
        if !p.is_null() {
            if preclaim_oss == 0 {
                __unregister_chrdev(SOUND_MAJOR, (*p).unit_minor as u32, 1, (*p).name.as_ptr());
            }
            device_destroy(&mut sound_class, MKDEV(SOUND_MAJOR, (*p).unit_minor as u32));
            kfree(p as *mut std::ffi::c_void);
        }
    }

    // Allocations
    //
    // 0	*16		Mixers
    // 1	*8		Sequencers
    // 2	*16		Midi
    // 3	*16		DSP
    // 4	*16		SunDSP
    // 5	*16		DSP16
    // 6	--		sndstat (obsolete)
    // 7	*16		unused
    // 8	--		alternate sequencer (see above)
    // 9	*16		raw synthesizer access
    // 10	*16		unused
    // 11	*16		unused
    // 12	*16		unused
    // 13	*16		unused
    // 14	*16		unused
    // 15	*16		unused

    static mut chains: [*mut SoundUnit; SOUND_STEP] = [ptr::null_mut(); SOUND_STEP];

    /// register_sound_special_device - register a special sound node
    /// @fops: File operations for the driver
    /// @unit: Unit number to allocate
    /// @dev: device pointer
    ///
    /// Allocate a special sound device by minor number from the sound
    /// subsystem.
    ///
    /// Return: The allocated number is returned on success. On failure,
    /// a negative error code is returned.

    #[no_mangle]
    pub unsafe extern "C" fn register_sound_special_device(fops: *const file_operations, unit: i32, dev: *mut device) -> i32 {
        let chain = (unit % SOUND_STEP as i32) as usize;
        let mut max_unit = 256i32;
        let mut name: *const u8;
        let mut _name: [u8; 16] = [0; 16];

        name = match chain {
            0 => "mixer\0".as_ptr(),
            1 => {
                if unit >= SOUND_STEP as i32 {
                    let fmt = "unknown%d\0".as_ptr();
                    let cstr_fmt = fmt as *const u8;
                    sprintf(_name.as_mut_ptr(), cstr_fmt, chain as i32);
                    if unit >= SOUND_STEP as i32 {
                        strcat(_name.as_mut_ptr(), "-\0".as_ptr());
                    }
                    _name.as_ptr()
                } else {
                    max_unit = unit + 1;
                    "sequencer\0".as_ptr()
                }
            }
            2 => "midi\0".as_ptr(),
            3 => "dsp\0".as_ptr(),
            4 => "audio\0".as_ptr(),
            5 => "dspW\0".as_ptr(),
            8 => {
                if unit >= SOUND_STEP as i32 {
                    let fmt = "unknown%d\0".as_ptr();
                    let cstr_fmt = fmt as *const u8;
                    sprintf(_name.as_mut_ptr(), cstr_fmt, chain as i32);
                    if unit >= SOUND_STEP as i32 {
                        strcat(_name.as_mut_ptr(), "-\0".as_ptr());
                    }
                    _name.as_ptr()
                } else {
                    max_unit = unit + 1;
                    "sequencer2\0".as_ptr()
                }
            }
            9 => "dmmidi\0".as_ptr(),
            10 => "dmfm\0".as_ptr(),
            12 => "adsp\0".as_ptr(),
            13 => "amidi\0".as_ptr(),
            14 => "admmidi\0".as_ptr(),
            _ => {
                sprintf(_name.as_mut_ptr(), "unknown%d\0".as_ptr(), chain as i32);
                if unit >= SOUND_STEP as i32 {
                    strcat(_name.as_mut_ptr(), "-\0".as_ptr());
                }
                _name.as_ptr()
            }
        };

        sound_insert_unit(&mut chains[chain], fops, -1, unit, max_unit, name, 0o600, dev)
    }

    // EXPORT_SYMBOL(register_sound_special_device);

    #[no_mangle]
    pub unsafe extern "C" fn register_sound_special(fops: *const file_operations, unit: i32) -> i32 {
        register_sound_special_device(fops, unit, ptr::null_mut())
    }

    // EXPORT_SYMBOL(register_sound_special);

    /// register_sound_mixer - register a mixer device
    /// @fops: File operations for the driver
    /// @dev: Unit number to allocate
    ///
    /// Allocate a mixer device. Unit is the number of the mixer requested.
    /// Pass -1 to request the next free mixer unit.
    ///
    /// Return: On success, the allocated number is returned. On failure,
    /// a negative error code is returned.

    #[no_mangle]
    pub unsafe extern "C" fn register_sound_mixer(fops: *const file_operations, dev: i32) -> i32 {
        sound_insert_unit(&mut chains[0], fops, dev, 0, 128, "mixer\0".as_ptr(), 0o600, ptr::null_mut())
    }

    // EXPORT_SYMBOL(register_sound_mixer);

    // DSP's are registered as a triple. Register only one and cheat
    // in open - see below.

    /// register_sound_dsp - register a DSP device
    /// @fops: File operations for the driver
    /// @dev: Unit number to allocate
    ///
    /// Allocate a DSP device. Unit is the number of the DSP requested.
    /// Pass -1 to request the next free DSP unit.
    ///
    /// This function allocates both the audio and dsp device entries together
    /// and will always allocate them as a matching pair - eg dsp3/audio3
    ///
    /// Return: On success, the allocated number is returned. On failure,
    /// a negative error code is returned.

    #[no_mangle]
    pub unsafe extern "C" fn register_sound_dsp(fops: *const file_operations, dev: i32) -> i32 {
        sound_insert_unit(&mut chains[3], fops, dev, 3, 131, "dsp\0".as_ptr(), 0o600, ptr::null_mut())
    }

    // EXPORT_SYMBOL(register_sound_dsp);

    /// unregister_sound_special - unregister a special sound device
    /// @unit: unit number to allocate
    ///
    /// Release a sound device that was allocated with
    /// register_sound_special(). The unit passed is the return value from
    /// the register function.

    #[no_mangle]
    pub unsafe extern "C" fn unregister_sound_special(unit: i32) {
        sound_remove_unit(&mut chains[(unit % SOUND_STEP as i32) as usize], unit);
    }

    // EXPORT_SYMBOL(unregister_sound_special);

    /// unregister_sound_mixer - unregister a mixer
    /// @unit: unit number to allocate
    ///
    /// Release a sound device that was allocated with register_sound_mixer().
    /// The unit passed is the return value from the register function.

    #[no_mangle]
    pub unsafe extern "C" fn unregister_sound_mixer(unit: i32) {
        sound_remove_unit(&mut chains[0], unit);
    }

    // EXPORT_SYMBOL(unregister_sound_mixer);

    /// unregister_sound_dsp - unregister a DSP device
    /// @unit: unit number to allocate
    ///
    /// Release a sound device that was allocated with register_sound_dsp().
    /// The unit passed is the return value from the register function.
    ///
    /// Both of the allocated units are released together automatically.

    #[no_mangle]
    pub unsafe extern "C" fn unregister_sound_dsp(unit: i32) {
        sound_remove_unit(&mut chains[3], unit);
    }

    // EXPORT_SYMBOL(unregister_sound_dsp);

    unsafe fn __look_for_unit(chain: usize, unit: i32) -> *mut SoundUnit {
        let mut s = chains[chain];

        while !s.is_null() && (*s).unit_minor <= unit {
            if (*s).unit_minor == unit {
                return s;
            }
            s = (*s).next;
        }
        ptr::null_mut()
    }

    unsafe extern "C" fn soundcore_open(inode: *mut inode, file: *mut file) -> i32 {
        let mut chain: usize;
        let mut unit = iminor(inode) as i32;
        let mut s: *mut SoundUnit;
        let mut new_fops: *const file_operations = ptr::null();

        chain = (unit & 0x0Fi32) as usize;
        if chain == 4 || chain == 5 {
            // dsp/audio/dsp16
            unit &= 0xF0i32;
            unit |= 3i32;
            chain = 3;
        }

        spin_lock(&mut sound_loader_lock);
        s = __look_for_unit(chain, unit);
        if !s.is_null() {
            new_fops = fops_get((*s).unit_fops);
        }
        if preclaim_oss != 0 && new_fops.is_null() {
            spin_unlock(&mut sound_loader_lock);

            // Please, don't change this order or code.
            // For ALSA slot means soundcard and OSS emulation code
            // comes as add-on modules which aren't depend on
            // ALSA toplevel modules for soundcards, thus we need
            // load them at first.	  [Jaroslav Kysela <perex@jcu.cz>]
            request_module("sound-slot-%i\0".as_ptr(), unit >> 4);
            request_module("sound-service-%i-%i\0".as_ptr(), unit >> 4, chain as i32);

            // sound-slot/service-* module aliases are scheduled
            // for removal in favor of the standard char-major-*
            // module aliases.  For the time being, generate both
            // the legacy and standard module aliases to ease
            // transition.
            if request_module("char-major-%d-%d\0".as_ptr(), SOUND_MAJOR, unit as u32) > 0 {
                request_module("char-major-%d\0".as_ptr(), SOUND_MAJOR);
            }

            spin_lock(&mut sound_loader_lock);
            s = __look_for_unit(chain, unit);
            if !s.is_null() {
                new_fops = fops_get((*s).unit_fops);
            }
        }
        spin_unlock(&mut sound_loader_lock);

        if new_fops.is_null() {
            return -19; // -ENODEV
        }

        // We rely upon the fact that we can't be unloaded while the
        // subdriver is there.
        replace_fops(file, new_fops);

        if (*file).f_op.is_null() || (*(*file).f_op).open.is_none() {
            return -19; // -ENODEV
        }

        (*(*file).f_op).open.unwrap()(inode, file)
    }

    // MODULE_ALIAS_CHARDEV_MAJOR(SOUND_MAJOR);

    unsafe fn cleanup_oss_soundcore() {
        // We have nothing to really do here - we know the lists must be empty
        unregister_chrdev(SOUND_MAJOR, "sound\0".as_ptr());
    }

    unsafe fn init_oss_soundcore() -> i32 {
        if preclaim_oss != 0 && register_chrdev(SOUND_MAJOR, "sound\0".as_ptr(), &soundcore_fops) < 0 {
            printk("soundcore: sound device already in use.\n\0".as_ptr());
            return -16; // -EBUSY
        }

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
