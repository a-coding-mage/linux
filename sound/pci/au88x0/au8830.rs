// SPDX-License-Identifier: GPL-2.0

// C source included these dependencies:
// #include "au8830.h"
// #include "au88x0.h"

static snd_vortex_ids: [pci_device_id; 2] = [
    pci_device_id {
        // PCI_VDEVICE(AUREAL, PCI_DEVICE_ID_AUREAL_VORTEX_2)
        driver_data: 0,
        ..PCI_VDEVICE(AUREAL, PCI_DEVICE_ID_AUREAL_VORTEX_2)
    },
    unsafe { core::mem::zeroed() },
];

// C source textually included these implementation units:
// #include "au88x0_synth.c"
// #include "au88x0_core.c"
// #include "au88x0_pcm.c"
// #include "au88x0_mixer.c"
// #include "au88x0_mpu401.c"
// #include "au88x0_game.c"
// #include "au88x0_eq.c"
// #include "au88x0_a3d.c"
// #include "au88x0_xtalk.c"
// #include "au88x0.c"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
