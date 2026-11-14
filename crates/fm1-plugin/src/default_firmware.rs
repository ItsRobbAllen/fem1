//! M-VAVE's unmodified V15 recovery image, redistributed under Apache-2.0.
//! Provenance and complete notices live beside the embedded image.
use crate::Firmware;
use std::sync::{Arc, OnceLock};

pub fn stock() -> Firmware {
    static PACKAGE: OnceLock<Arc<Vec<u8>>> = OnceLock::new();
    Firmware {
        name: "M-VAVE stock V15".into(),
        pkg: PACKAGE.get_or_init(|| Arc::new(include_bytes!("../resources/mvave/FM-1_015.fwsc").to_vec())).clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Fm1Params;
    use nih_plug::prelude::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn embedded_image_is_the_pinned_parseable_stock_firmware() {
        let firmware = stock();
        assert_eq!(format!("{:x}", Sha256::digest(&*firmware.pkg)),
            "db1642b2b6fa5c2cccb11ffd13878068bb28601678d3644049f99dc40e7edb8a");
        assert!(fm1_soc::stock_from_package(&firmware.pkg, None).is_ok());
        assert!(Arc::ptr_eq(&firmware.pkg, &stock().pkg));
    }

    #[test]
    fn new_instances_have_stock_but_project_firmware_overrides_it() {
        let params = Fm1Params::default();
        assert_eq!(params.firmware.lock().unwrap().name, "M-VAVE stock V15");
        assert_eq!(params.channel.value(), 2, "stock listens on channel 2");
        *params.firmware.lock().unwrap() = Firmware { name: "Another firmware".into(), pkg: Arc::new(vec![1, 2, 3]) };
        let saved = params.serialize_fields();
        let restored = Fm1Params::default();
        restored.deserialize_fields(&saved);
        let firmware = restored.firmware.lock().unwrap();
        assert_eq!(firmware.name, "Another firmware");
        assert_eq!(firmware.pkg.as_slice(), &[1, 2, 3]);
    }
}
