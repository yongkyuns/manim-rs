use generational_arena::Index;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectangleId(pub Index);

impl From<Index> for RectangleId {
    fn from(index: Index) -> Self {
        Self(index)
    }
}

impl From<RectangleId> for Index {
    fn from(id: RectangleId) -> Self {
        id.0
    }
}
