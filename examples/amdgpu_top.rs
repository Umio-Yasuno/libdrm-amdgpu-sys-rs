use libdrm_amdgpu_sys::*;

/* ref: https://gitlab.freedesktop.org/tomstdenis/umr/ */
/* ref: https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/gallium/drivers/radeonsi/si_gpu_load.c */

/* ref: https://developer.amd.com/wordpress/media/2013/10/R6xx_R7xx_3D.pdf */
/* ref: http://developer.amd.com/wordpress/media/2013/10/CIK_3D_registers_v2.pdf */

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
        self.ta += ((reg >> 14) & 0b1) as u8;
        self.gds += ((reg >> 15) & 0b1) as u8;
        self.vgt += ((reg >> 17) & 0b1) as u8;
        self.ia += ((reg >> 19) & 0b1) as u8;
        self.sx += ((reg >> 20) & 0b1) as u8;
        self.spi += ((reg >> 22) & 0b1) as u8;
        self.bci += ((reg >> 23) & 0b1) as u8;
        self.sc += ((reg >> 24) & 0b1) as u8;
        self.pa += ((reg >> 25) & 0b1) as u8;
        self.db += ((reg >> 26) & 0b1) as u8;
        self.cp += ((reg >> 29) & 0b1) as u8;
        self.cb += ((reg >> 30) & 0b1) as u8;
        self.gui_active += ((reg >> 31) & 0b1) as u8;
    }
}

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
