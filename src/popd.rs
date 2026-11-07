//! Popd model.

#[derive(Debug, Clone)]
pub struct DirectoryStack {
    pub directories: Vec<String>,
}

impl Default for DirectoryStack {
    fn default() -> Self {
        Self {
            directories: vec!["/home/user".to_string()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Popd {
    pub rotate: bool,
}

impl Default for Popd {
    fn default() -> Self {
        Self { rotate: false }
    }
}

impl Popd {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pop(&self, stack: &mut DirectoryStack) -> Option<String> {
        if stack.directories.len() > 1 {
            stack.directories.pop()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popd() {
        let p = Popd::new();
        let mut s = DirectoryStack::default();
        s.directories.push("/tmp".to_string());
        let r = p.pop(&mut s);
        assert!(r.is_some());
    }
}
