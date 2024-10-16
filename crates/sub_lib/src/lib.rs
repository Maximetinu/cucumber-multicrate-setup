#[cfg(test)]
pub mod uat;

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
pub struct Cat {
    pub hungry: bool,
}

impl Cat {
    pub fn feed(&mut self) {
        self.hungry = false;
    }
}
