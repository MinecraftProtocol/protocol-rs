use crate::versions::ReleaseVersion::*;

pub enum ReleaseVersion {
    V1_8,
    V1_9,
    V1_9_1,
    V1_9_2,
    V1_9_3,
    V1_10,
    V1_11,
    V1_11_1,
    V1_12,
    V1_12_1,
    V1_12_2,
    V1_13,
    V1_13_1,
    V1_13_2,
    V1_14,
    V1_14_1,
    V1_14_2,
    V1_14_3,
    V1_14_4,
}

impl ReleaseVersion {
    pub fn get_from_protocol(protocol: i16) -> Option<ReleaseVersion> {
        match protocol {
            47 => Some(V1_8),
            107 => Some(V1_9),
            108 => Some(V1_9_1),
            109 => Some(V1_9_2),
            110 => Some(V1_9_3),
            210 => Some(V1_10),
            315 => Some(V1_11),
            316 => Some(V1_11_1),
            335 => Some(V1_12),
            338 => Some(V1_12_1),
            340 => Some(V1_12_2),
            393 => Some(V1_13),
            401 => Some(V1_13_1),
            404 => Some(V1_13_2),
            477 => Some(V1_14),
            480 => Some(V1_14_1),
            485 => Some(V1_14_2),
            490 => Some(V1_14_3),
            498 => Some(V1_14_4),
            _ => None
        }
    }

    pub fn get_protocol(&self) -> i16 {
        match self {
            V1_8 => 47,
            V1_9 => 107,
            V1_9_1 => 108,
            V1_9_2 => 109,
            V1_9_3 => 110,
            V1_10 => 210,
            V1_11 => 315,
            V1_11_1 => 316,
            V1_12 => 335,
            V1_12_1 => 338,
            V1_12_2 => 340,
            V1_13 => 393,
            V1_13_1 => 401,
            V1_13_2 => 404,
            V1_14 => 477,
            V1_14_1 => 480,
            V1_14_2 => 485,
            V1_14_3 => 490,
            V1_14_4 => 498,
        }
    }
}
