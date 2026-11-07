//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationMode {
    NoRotate,
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Clone)]
pub struct PopdOptions {
    pub rotation_mode: RotationMode,
}

impl Default for PopdOptions {
    fn default() -> Self {
        Self {
            rotation_mode: RotationMode::NoRotate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popd_options() {
        let o = PopdOptions::default();
        assert_eq!(o.rotation_mode, RotationMode::NoRotate);
    }
}
