use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must be unique between all the plugins, 2 characters long and match `DeviceNamespace` field in `manifest.json`
pub const DEVICE_NAMESPACE: &str = "D3";

pub const ROW_COUNT: usize = 3;
pub const COL_COUNT: usize = 4;
pub const KEY_COUNT: usize = 12;
pub const ENCODER_COUNT: usize = 0;

#[derive(Debug, Clone)]
pub enum Kind {
    Cyberaxon,
}

pub const CYBERAXON_VID: u16 = 0x0600;

pub const C_3002_PID: u16 = 0x3002;

// Map all queries to usage page 65440 and usage id 1 for now
pub const CYBERAXON_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, CYBERAXON_VID, C_3002_PID);

pub const QUERIES: [DeviceQuery; 1] = [
    CYBERAXON_QUERY,
];

impl Kind {
    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match vid {
            CYBERAXON_VID => match pid {
                C_3002_PID => Some(Kind::Cyberaxon),
                _ => None,
            },

            _ => None,
        }
    }

    /// There is no point relying on manufacturer/device names reported by the USB stack,
    /// so we return custom names for all the kinds of devices
    pub fn human_name(&self) -> String {
        match &self {
            Self::Cyberaxon => "RayCue CyberAxon D3",
        }
        .to_string()
    }

    /// Returns protocol version for device
    pub fn protocol_version(&self) -> usize {
        match self {
            Self::Cyberaxon => 3,
            _ => 2,
        }
    }

    pub fn image_format(&self) -> ImageFormat {

    if matches!(self, Self::Cyberaxon) {
        return ImageFormat {
            mode: ImageMode::JPEG,
            size: (64, 64),
            rotation: ImageRotation::Rot270,
            mirror: ImageMirroring::None,
        };
    }

        if self.protocol_version() == 3 {
            return ImageFormat {
                mode: ImageMode::JPEG,
                size: (64, 64),
                rotation: ImageRotation::Rot90,
                mirror: ImageMirroring::None,
            };
        }

        return ImageFormat {
            mode: ImageMode::JPEG,
            size: (60, 60),
            rotation: ImageRotation::Rot0,
            mirror: ImageMirroring::None,
        };
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
