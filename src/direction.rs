use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction3D {
    Top,
    Bottom,
    Left,
    Right,
    Up,
    Down,
    None,
}

impl Direction3D {
    pub fn opposite(&self) -> Direction3D {
        match self {
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::None => Self::None,
        }
    }
}

impl fmt::Display for Direction3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gives_opposite_direction() {
        assert_eq!(Direction3D::Top, Direction3D::Bottom.opposite());
        assert_eq!(Direction3D::Bottom, Direction3D::Top.opposite());
        assert_eq!(Direction3D::Left, Direction3D::Right.opposite());
        assert_eq!(Direction3D::Right, Direction3D::Left.opposite());
        assert_eq!(Direction3D::Up, Direction3D::Down.opposite());
        assert_eq!(Direction3D::Down, Direction3D::Up.opposite());
    }
}
