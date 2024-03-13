use num_traits::{Float, Num, Zero};

/// A set of offsets in each of the four cardinal directions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeInsets<T: Num + Copy> {
    /// The top edge inset value.
    pub top: T,
    /// The left edge inset value.
    pub left: T,
    /// The bottom edge inset value.
    pub bottom: T,
    /// The right edge inset value.
    pub right: T,
}

// MARK: Creation

impl<T: Num + Copy> EdgeInsets<T> {
    /// Creates new edge insets.
    pub fn new(top: T, left: T, bottom: T, right: T) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }

    /// Creates new edge insets with all sides set to the same value.
    pub fn all(value: T) -> Self {
        Self {
            top: value,
            left: value,
            bottom: value,
            right: value,
        }
    }
}

// MARK: Zero

impl<T: Num + Copy + Zero> EdgeInsets<T> {
    /// Creates a new edge insets with zero values.
    pub fn zero() -> EdgeInsets<T> {
        EdgeInsets::all(T::zero())
    }
}

// MARK: Actions

impl<T: Float> EdgeInsets<T> {
    /// Rounds all of the insets to integer values.
    pub fn round(&mut self) {
        self.top.round();
        self.bottom.round();
        self.left.round();
        self.right.round();
    }
}
