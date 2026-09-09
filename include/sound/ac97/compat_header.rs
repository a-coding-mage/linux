/* SPDX-License-Identifier: GPL-2.0
 *
 *  Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
 *
 * This file is for backward compatibility with snd_ac97 structure and its
 * multiple usages, such as the snd_ac97_bus and snd_ac97_build_ops.
 */

// Dependency corresponding to <sound/ac97_codec.h>.

extern "C" {
    pub fn snd_ac97_compat_alloc(
        adev: *mut ac97_codec_device,
    ) -> *mut snd_ac97;
    pub fn snd_ac97_compat_release(ac97: *mut snd_ac97);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
