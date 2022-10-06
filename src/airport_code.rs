use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub(super) enum AirportCode {
    ATL,
    BWI,
    DFW,
    EWR,
    IND,
    GSO,
    JFK,
    ORD,
    POR,
    SFO,
    SLC,
    YOW,
    YUL,
    YLW,
    YVR,
    YYZ,
}
