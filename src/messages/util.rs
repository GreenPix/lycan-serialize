use super::Direction;
use common_capnp::Direction as CapnpDirection;

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

