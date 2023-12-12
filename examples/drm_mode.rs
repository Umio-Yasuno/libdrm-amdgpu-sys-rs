use libdrm_amdgpu_sys::{drmModeRes, drmModePropertyBlob, drmModePropType};
use std::fs::File;

fn main() {
    let fd = {
        use std::os::fd::IntoRawFd;

        let f = File::open("/dev/dri/card0").unwrap();

        f.into_raw_fd()
    };

    let drm_mode_res = drmModeRes::get(fd).unwrap();
    let current_connectors = drm_mode_res.get_all_connector_current(fd);

    for connector in current_connectors.iter() {
        println!(
            "Connector {} ({}-{}), {}",
            connector.connector_id(),
            connector.connector_type(),
            connector.connector_type_id(),
            connector.connection(),
        );
        if let Some(conn_prop) = connector.get_connector_props(fd) {
            let mode_props = conn_prop.get_mode_property(fd);

            for (prop, value) in mode_props {
                let type_ = prop.property_type();

                println!(
                    "    {:?}, id: {}, value: {}, type: {}",
                    prop.name(),
                    prop.prop_id(),
                    value,
                    type_,
                );

                match type_ {
                    drmModePropType::RANGE =>
                        println!("        values: {:?}", prop.values()),
                    drmModePropType::ENUM => {
                        print!("        enums: [");
                        for enum_ in prop.enums().iter() {
                            print!("{:?}={}, ", enum_.name(), enum_.value);
                        }
                        println!("] ");
                    },
                    drmModePropType::BLOB => {
                        if let Some(b) = drmModePropertyBlob::get(fd, value as u32) {
                            print!("        blob:");

                            for (i, byte) in b.data().iter().enumerate() {
                                if (i % 16) == 0 { print!("\n            "); }
                                print!("{byte:02x}");
                            }

                            println!();
                        }
                    },
                    _ => {},
                }
            }
        }
        println!();
    }
}
