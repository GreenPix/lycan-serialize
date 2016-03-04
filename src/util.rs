
#[cfg(capnp)]
mod capnp {
    use super::Direction;
    use common_capnp::Direction as CapnpDirection;
    use super::ErrorCode;
    use notifications_capnp::ErrorCode as CapnpErrorCode;
    impl From<Option<Direction>> for CapnpDirection {
        fn from(direction: Option<Direction>) -> CapnpDirection {
            match direction {
                None => CapnpDirection::None,
                Some(Direction::West) => CapnpDirection::West,
                Some(Direction::South) => CapnpDirection::South,
                Some(Direction::East) => CapnpDirection::East,
                Some(Direction::North) => CapnpDirection::North,
            }
        }
    }

    impl From<CapnpDirection> for Option<Direction> {
        fn from(direction: CapnpDirection) -> Option<Direction> {
            match direction {
                CapnpDirection::None => None,
                CapnpDirection::West => Some(Direction::West),
                CapnpDirection::South => Some(Direction::South),
                CapnpDirection::East => Some(Direction::East),
                CapnpDirection::North => Some(Direction::North),
            }
        }
    }

    impl From<CapnpErrorCode> for ErrorCode {
        fn from(code: CapnpErrorCode) -> ErrorCode {
            match code {
                CapnpErrorCode::Success => ErrorCode::Success,
                CapnpErrorCode::Error => ErrorCode::Error,
            }
        }
    }

    impl From<ErrorCode> for CapnpErrorCode {
        fn from(code: ErrorCode) -> CapnpErrorCode {
            match code {
                ErrorCode::Success => CapnpErrorCode::Success,
                ErrorCode::Error => CapnpErrorCode::Error,
            }
        }
    }
}
