# libdrm-amdgpu-sys-rs
libdrm_amdgpu bindings for Rust, and some methods ported from Mesa3D (mainly [ac_gpu_info.c](https://gitlab.freedesktop.org/mesa/mesa/blob/main/src/amd/common/ac_gpu_info.c)).  

## Reference
 * [Mesa / drm · GitLab](https://gitlab.freedesktop.org/mesa/drm/)
 * [Mesa / mesa · GitLab](https://gitlab.freedesktop.org/mesa/mesa/)
 * Linux Kernel
    * `drivers/gpu/drm/amd/amdgpu/amdgpu_kms.c`

## License
 * MIT License

## Examples
```
let (amdgpu_dev, drm_major, drm_minor) = {
    use std::fs::File;
    use std::os::fd::IntoRawFd;

    let fd = File::open("/dev/dri/renderD128").unwrap();

    AMDGPU::DeviceHandle::init(fd.into_raw_fd()).unwrap()
};
```
### amdgpu_info
```
cargo run --example amdgpu_info
```
### vbios_dump
```
cargo run --example vbios_dump
```
## Build
To generate a new `bindings/drm.rs` .

```
cargo build --features=buildtime_bindgen
```
