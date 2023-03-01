use libdrm_amdgpu_sys::*;

/* ref: https://gitlab.freedesktop.org/tomstdenis/umr/ */
/* ref: https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/gallium/drivers/radeonsi/si_gpu_load.c */

/* ref: https://developer.amd.com/wordpress/media/2013/10/R6xx_R7xx_3D.pdf */
/* ref: http://developer.amd.com/wordpress/media/2013/10/CIK_3D_registers_v2.pdf */

struct GRBM {
    ta: u32, // Texture Addresser?
    gds: u32, // Global Data Share
    vgt: u32, // Vertex Grouper and Tessellator
    ia: u32, // Input Assembly?
    sx: u32, // Shader Export
    spi: u32, // Shader Pipe Interpolator
    bci: u32, // Barycentric interpolation control
    sc: u32, // Scan Convertor
    pa: u32, // Primitive Assembly
    db: u32, // Depth Block? Depth Buffer?
    cp: u32, // Command Processor?
    cb: u32, // Color Buffer
    gui_active: u32,
}

impl GRBM {
    fn new() -> Self {
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
            db: 0, // Depth Block
            cp: 0,
            cb: 0,
            gui_active: 0,
        }
    }

    fn clear(&mut self) {
        self.ta = 0;
        self.gds = 0;
        self.vgt = 0;
        self.ia = 0;
        self.sx = 0;
        self.spi = 0;
        self.bci = 0;
        self.sc = 0;
        self.pa = 0;
        self.db = 0;
        self.cp = 0;
        self.cb = 0;
        self.gui_active = 0;
    }

    fn acc(&mut self, reg: u32) {
        self.ta += (reg >> 14) & 0b1;
        self.gds += (reg >> 15) & 0b1;
        self.vgt += (reg >> 17) & 0b1;
        self.ia += (reg >> 19) & 0b1;
        self.sx += (reg >> 20) & 0b1;
        self.spi += (reg >> 22) & 0b1;
        self.bci += (reg >> 23) & 0b1;
        self.sc += (reg >> 24) & 0b1;
        self.pa += (reg >> 25) & 0b1;
        self.db += (reg >> 26) & 0b1;
        self.cp += (reg >> 29) & 0b1;
        self.cb += (reg >> 30) & 0b1;
        self.gui_active += (reg >> 31) & 0b1;
    }
}

fn main() {
    let (amdgpu_dev, _major, _minor) = {
        use std::fs::File;
        use std::os::fd::IntoRawFd;

        let fd = File::open("/dev/dri/renderD128").unwrap();

        AMDGPU::DeviceHandle::init(fd.into_raw_fd()).unwrap()
    };

    let mut grbm = GRBM::new();
    let delay = std::time::Duration::from_millis(10);
    for _ in 0..10 {
        for _ in 0..100 {
            if let Ok(out) = amdgpu_dev.read_grbm() {
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
