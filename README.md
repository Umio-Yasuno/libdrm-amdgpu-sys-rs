# libdrm-amdgpu-sys-rs
libdrm_amdgpu bindings for Rust, and some methods ported from Mesa3D (mainly [ac_gpu_info.c](https://gitlab.freedesktop.org/mesa/mesa/blob/main/src/amd/common/ac_gpu_info.c)).  

---

## Reference
 * [Mesa / drm · GitLab](https://gitlab.freedesktop.org/mesa/drm/)
 * [Mesa / mesa · GitLab](https://gitlab.freedesktop.org/mesa/mesa/)
 * Linux Kernel
    * `drivers/gpu/drm/amd/amdgpu/amdgpu_kms.c`

---

## License⚖️ 
 * [MIT License](https://github.com/Umio-Yasuno/libdrm-amdgpu-sys-rs/blob/main/LICENSE)

---

## Documentation📚 
 * [libdrm_amdgpu_sys - Rust](https://docs.rs/libdrm_amdgpu_sys/latest/libdrm_amdgpu_sys/)

 ---

## Examples🕹️ 

```rust
let (amdgpu_dev, drm_major, drm_minor) = {
    use std::fs::File;
    use std::os::fd::IntoRawFd;

    let fd = File::open("/dev/dri/renderD128").unwrap();

    AMDGPU::DeviceHandle::init(fd.into_raw_fd()).unwrap()
};
let mark_name = amdgpu_dev.get_marketing_name().unwrap();
let device_info = amdgpu_dev.device_info().unwrap();
```

---

### amdgpu_info

```bash
cargo run --example amdgpu_info
```

<details>

<summary>📑 </summary>

<br>

It [example](https://github.com/Umio-Yasuno/libdrm-amdgpu-sys-rs/blob/main/examples/amdgpu_info.rs) retrieves the path of the AMD GPU device from the environment variable AMDGPU_PATH. If the variable is not set, it uses the default device path (/dev/dri/renderD128). It opens the device file specified by the path and initializes the AMD GPU device handle using the file descriptor obtained. If the DRM (Direct Rendering Manager) version information can be retrieved from the AMD GPU device, it is printed. The marketing name of the AMD GPU device is printed.

Various information about the device is retrieved and printed, including device and revision IDs, family name, ASIC name, chip class, GPU type (APU or discrete GPU), shader engine and array details, compute unit information, clock frequencies, performance metrics, VRAM details, cache sizes, etc. Memory usage information, including VRAM usage, CPU-accessible VRAM usage, and Graphics Translation Table usage, is printed. Hardware IP information, such as the count, version, and number of queues for different IP types (GFX, Compute, DMA, UVD, VCE, etc.), is retrieved and printed.

Firmware version information for various firmware types (VCE, UVD, GMC, etc.) is retrieved and printed.
Video capabilities information, including decode and encode capabilities, is retrieved and printed.
PCI bus information, including domain, bus, device, and function details, is printed. Additionally, current and minimum/maximum PCI link speeds are displayed.

VBIOS information, including name, part number, version, and date, is printed. Sensor information, such as GPU clock, memory clock, GPU temperature, GPU load, GPU power, VDDNB voltage, VDDGFX voltage, stable P-state GPU clock, and stable P-state memory clock, is retrieved and printed. The system file system and hardware monitoring paths for the AMD GPU device are displayed. If available, power cap information (type, current value, default value, and range) is printed. If available, temperature information (edge, junction, and memory temperatures) is printed.


</details>

### vbios_dump

```bash
cargo run --example vbios_dump
```

<details>

<summary>📑 </summary>

<br>

This [example](https://github.com/Umio-Yasuno/libdrm-amdgpu-sys-rs/blob/main/examples/vbios_dump.rs) interacts with libdrm_amdgpu_sys to access AMD GPU information and dump the VBIOS image if requested. It imports dependencies, defines the dump() function to save the image data into a file, and retrieves the device path from the AMDGPU_PATH environment variable. It initializes the AMDGPU device handle and prints details about the VBIOS if available. If the user specifies the "-d" or "--dump" command-line argument, it requests the VBIOS image and calls the dump() function to save it to a file. In summary, this code retrieves AMD GPU's VBIOS information and allows the user to save the VBIOS image if desired.

</details>

### gpu_metrics

```bash
cargo run --example gpu_metrics
```
<details>

<summary>📑 </summary>

<br>

This [example](https://github.com/Umio-Yasuno/libdrm-amdgpu-sys-rs/blob/main/examples/gpu_metrics.rs) retrieves GPU metrics information for AMD GPU devices using the libdrm_amdgpu_sys library. It opens a device file specified by the "AMDGPU_PATH" environment variable or uses the default file if not specified. The AMDGPU device handle is then initialized with the opened file. The code attempts to obtain GPU metrics from the system path of the device. If successful, it prints the metrics, including the average socket power if available. In case of failure, it checks if the GPU supports metrics by retrieving the device information. The code provides a relevant message based on the GPU support.

This example is useful for obtaining GPU metrics data and can be used to monitor and analyze performance for AMD GPU devices. The README provides instructions on how to set the device file path and highlights the fallback default file. Additionally, it mentions that certain GPU models support GPU metrics and clarifies the expected results in case of failure.

</details>

---

## Build
To generate a new `bindings/drm.rs` .

```bash
cargo build --features=buildtime_bindgen
```
