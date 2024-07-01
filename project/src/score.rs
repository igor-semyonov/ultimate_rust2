use std::cmp::Ordering;

/// Score struct for keeping track of both current and high score.
pub struct Score {
    /// The score value
    pub value: u32,
    /// The prefix. Default is "". For high score it is "High ".
    pub prefix: String,
}
impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Score {}
impl PartialOrd for Score {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value
            .cmp(&other.value)
    }
}
impl std::ops::AddAssign<u32> for Score {
    fn add_assign(&mut self, rhs: u32) {
        self.value += rhs;
    }
}
impl Default for Score {
    fn default() -> Self {
        Self::new(0, "")
    }
}

impl Score {
    pub fn new<S: Into<String>>(value: u32, prefix: S) -> Self {
        Self {
            value,
            prefix: prefix.into(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}Score: {}",
            self.prefix, self.value
        )
    }
}
