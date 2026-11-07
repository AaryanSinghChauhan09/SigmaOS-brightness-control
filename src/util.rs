//! Brightness control model.

#[derive(Debug, Clone)]
pub struct Model {
    pub brightness: u8,
}

impl Default for Model {
    fn default() -> Self { Self { brightness: 50 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, brightness: u8) {
        self.brightness = brightness.min(100);
    }

    pub fn up(&mut self, amount: u8) {
        self.brightness = (self.brightness + amount).min(100);
    }

    pub fn down(&mut self, amount: u8) {
        self.brightness = self.brightness.saturating_sub(amount);
    }

    pub fn brightness(&self) -> u8 { self.brightness }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brightness_up_down() {
        let mut m = Model::new();
        m.set(80);
        assert_eq!(m.brightness(), 80);
        m.down(30);
        assert_eq!(m.brightness(), 50);
        m.up(60);
        assert_eq!(m.brightness(), 100);
    }
}
