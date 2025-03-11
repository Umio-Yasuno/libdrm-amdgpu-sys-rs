// ref: https://github.com/GPUOpen-Tools/device_info/blob/1e9040681766423e6b361c6fd541834c3864a00b/DeviceInfo.cpp
pub const AMDGPU_IDS_2: &[(u32, u32, &str)] = &[
    (0x1586, 0xC1, "Radeon 8060S Graphics"),
    (0x1586, 0xC2, "Radeon 8050S Graphics"),
    (0x1586, 0xC4, "Radeon 8050S Graphics"),
    (0x1586, 0xD1, "Radeon 8060S Graphics"),
    (0x1586, 0xD2, "Radeon 8050S Graphics"),
    (0x1586, 0xD4, "Radeon 8050S Graphics"),
    (0x1586, 0xD5, "Radeon 8040S Graphics"),
    (0x1114, 0xC2, "AMD Radeon(TM) 860M Graphics"),
    (0x1114, 0xC3, "AMD Radeon(TM) 840M Graphics"),
    (0x1114, 0xD2, "AMD Radeon(TM) 860M Graphics"),
    (0x1114, 0xD3, "AMD Radeon(TM) 840M Graphics"),
    (0x7550, 0xC0, "AMD Radeon RX 9070 XT"),
    (0x7550, 0xC3, "AMD Radeon RX 9070"),
];
