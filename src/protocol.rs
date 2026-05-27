#[derive(Debug, Clone, PartialEq, Default)]
pub enum DomeResponse {
    #[default]
    Unknown,
    Open,
    Closed,
}

pub fn parse_response(bytes: &[u8]) -> Option<DomeResponse> {
    let mut result = None;
    for &b in bytes {
        match b {
            b'Y' => result = Some(DomeResponse::Open),
            b'X' => result = Some(DomeResponse::Closed),
            _ => {}
        }
    }
    result
}
