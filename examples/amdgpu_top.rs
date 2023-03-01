use libdrm_amdgpu_sys::*;

/* ref: https://gitlab.freedesktop.org/tomstdenis/umr/ */
/* ref: https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/gallium/drivers/radeonsi/si_gpu_load.c */

/* ref: https://developer.amd.com/wordpress/media/2013/10/R6xx_R7xx_3D.pdf */
/* ref: http://developer.amd.com/wordpress/media/2013/10/CIK_3D_registers_v2.pdf */

macro_rules! get_bit {
    ($reg: expr, $shift: expr) => {
        (($reg >> $shift) & 0b1) as u8
    };
}

struct GRBM {
    ta: u8, // Texture Addresser?
    gds: u8, // Global Data Share
    vgt: u8, // Vertex Grouper and Tessellator
    ia: u8, // Input Assembly?
    sx: u8, // Shader Export
    spi: u8, // Shader Pipe Interpolator
    bci: u8, // Barycentric interpolation control
    sc: u8, // Scan Convertor
    pa: u8, // Primitive Assembly
    db: u8, // Depth Block? Depth Buffer?
    cp: u8, // Command Processor?
    cb: u8, // Color Buffer
    gui_active: u8,
}

impl GRBM {
    const fn new() -> Self {
        Self {
            ta: 0,
            gds: 0,
            vgt: 0,
            ia: 0,
            sx: 0,
            spi: 0,
            bci: 0,
            sc: 0,
            pa: 0,
            db: 0,
            cp: 0,
            cb: 0,
            gui_active: 0,
        }
    }

    fn clear(&mut self) {
        *self = Self::new()
    }

    fn acc(&mut self, reg: u32) {
        self.ta += get_bit!(reg, 14);
        self.gds += get_bit!(reg, 15);
        self.vgt += get_bit!(reg, 17);
        self.ia += get_bit!(reg, 19);
        self.sx += get_bit!(reg, 20);
        self.spi += get_bit!(reg, 22);
        self.bci += get_bit!(reg, 23);
        self.sc += get_bit!(reg, 24);
        self.pa += get_bit!(reg, 25);
        self.db += get_bit!(reg, 26);
        self.cp += get_bit!(reg, 29);
        self.cb += get_bit!(reg, 30);
        self.gui_active += get_bit!(reg, 31);
    }
}


/*
// SRBM
struct UVD_BUSY(u8);
impl UVD_BUSY {
    const fn new() -> Self {
        Self(0u8)
    }

    fn clear(&mut self) {
        *self = Self::new()
    }

    fn acc(&mut self, reg: u32) {
        self.0 += get_bit!(reg, 19);
    }
}
*/

fn main() {
    let (amdgpu_dev, _major, _minor) = {
        use std::fs::File;
        use std::os::fd::IntoRawFd;

        let fd = File::open("/dev/dri/renderD128").unwrap();

        AMDGPU::DeviceHandle::init(fd.into_raw_fd()).unwrap()
    };

    let offset = {
        use AMDGPU::GPU_INFO;
        let ext_info = amdgpu_dev.device_info().unwrap();

        ext_info.get_family_name().get_grbm_offset()
    };

    let mut grbm = GRBM::new();
    let delay = std::time::Duration::from_millis(10);

    for _ in 0..10 {
        for _ in 0..100 {
            if let Ok(out) = amdgpu_dev.read_mm_registers(offset) {
                grbm.acc(out);
            }
            std::thread::sleep(delay);
        }
        println!(
            "TA:{ta:3}%, VGT:{vgt:3}%, SX:{sx:3}%, SPI:{spi:3}%, DB:{db:3}%, CB:{cb:3}%, CP:{cp:3}%, GUI:{gui:3}%",
            ta = grbm.ta,
            vgt = grbm.vgt,
            sx = grbm.sx,
            spi = grbm.spi,
            db = grbm.db,
            cb = grbm.cb,
            cp = grbm.cp,
            gui = grbm.gui_active,
        );
        grbm.clear();
    }
}
