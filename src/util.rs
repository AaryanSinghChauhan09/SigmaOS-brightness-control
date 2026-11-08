//! Brightness control model — multi-display brightness management.

#[derive(Debug, Clone)]
pub struct Display {
    pub name: String,
    pub brightness: u8,
    pub active: bool,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub displays: Vec<Display>,
    pub auto_brightness: bool,
    pub brightness_curve: Vec<u8>, // Mapping from ambient light to brightness
}

impl Default for Model {
    fn default() -> Self {
        Self {
            displays: vec![],
            auto_brightness: false,
            brightness_curve: vec![],
        }
    }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_display(&mut self, name: &str, is_primary: bool) -> usize {
        let id = self.displays.len();
        let display = Display {
            name: name.into(),
            brightness: 50,
            active: true,
            is_primary,
        };
        self.displays.push(display);
        id
    }

    pub fn remove_display(&mut self, name: &str) -> bool {
        if let Some(i) = self.displays.iter().position(|d| d.name == name) {
            self.displays.remove(i);
            true
        } else {
            false
        }
    }

    pub fn set_brightness(&mut self, name: &str, brightness: u8) -> bool {
        if let Some(d) = self.displays.iter_mut().find(|d| d.name == name) {
            d.brightness = brightness.min(100);
            true
        } else {
            false
        }
    }

    pub fn brightness_up(&mut self, name: &str, amount: u8) -> bool {
        if let Some(d) = self.displays.iter_mut().find(|d| d.name == name) {
            d.brightness = (d.brightness + amount).min(100);
            true
        } else {
            false
        }
    }

    pub fn brightness_down(&mut self, name: &str, amount: u8) -> bool {
        if let Some(d) = self.displays.iter_mut().find(|d| d.name == name) {
            d.brightness = d.brightness.saturating_sub(amount);
            true
        } else {
            false
        }
    }

    pub fn set_all_brightness(&mut self, brightness: u8) -> bool {
        let mut changed = false;
        for d in &mut self.displays {
            if d.active {
                d.brightness = brightness.min(100);
                changed = true;
            }
        }
        changed
    }

    pub fn brightness_up_all(&mut self, amount: u8) -> bool {
        let mut changed = false;
        for d in &mut self.displays {
            if d.active {
                d.brightness = (d.brightness + amount).min(100);
                changed = true;
            }
        }
        changed
    }

    pub fn brightness_down_all(&mut self, amount: u8) -> bool {
        let mut changed = false;
        for d in &mut self.displays {
            if d.active {
                d.brightness = d.brightness.saturating_sub(amount);
                changed = true;
            }
        }
        changed
    }

    pub fn get_display(&self, name: &str) -> Option<&Display> {
        self.displays.iter().find(|d| d.name == name)
    }

    pub fn get_primary(&self) -> Option<&Display> {
        self.displays.iter().find(|d| d.is_primary)
    }

    pub fn get_active_displays(&self) -> Vec<&Display> {
        self.displays.iter().filter(|d| d.active).collect()
    }

    pub fn set_auto_brightness(&mut self, enabled: bool) {
        self.auto_brightness = enabled;
    }

    pub fn set_brightness_curve(&mut self, curve: Vec<u8>) {
        self.brightness_curve = curve;
    }

    pub fn calculate_auto_brightness(&self, ambient_light: u8) -> u8 {
        if self.brightness_curve.is_empty() {
            // Default linear mapping
            return ambient_light.min(100);
        }
        // Use curve for mapping
        let idx = (ambient_light as usize * (self.brightness_curve.len() - 1) / 100).min(self.brightness_curve.len() - 1);
        self.brightness_curve[idx]
    }

    pub fn displays(&self) -> &[Display] { &self.displays }
    pub fn auto_brightness(&self) -> bool { self.auto_brightness }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_multi_display() {
        let mut m = Model::new();
        m.add_display("eDP-1", true); // laptop screen
        m.add_display("HDMI-1", false); // external monitor
        assert_eq!(m.displays().len(), 2);
        assert!(m.set_brightness("eDP-1", 80));
        assert!(m.set_brightness("HDMI-1", 60));
        assert_eq!(m.get_display("eDP-1").unwrap().brightness, 80);
        assert_eq!(m.get_display("HDMI-1").unwrap().brightness, 60);
    }

    #[test]
    fn brightness_controls() {
        let mut m = Model::new();
        m.add_display("eDP-1", true);
        m.set_brightness("eDP-1", 50);
        m.brightness_up("eDP-1", 30);
        assert_eq!(m.get_display("eDP-1").unwrap().brightness, 80);
        m.brightness_down("eDP-1", 40);
        assert_eq!(m.get_display("eDP-1").unwrap().brightness, 40);
    }

    #[test]
    fn brightness_all_displays() {
        let mut m = Model::new();
        m.add_display("eDP-1", true);
        m.add_display("HDMI-1", false);
        m.set_all_brightness(70);
        assert_eq!(m.get_display("eDP-1").unwrap().brightness, 70);
        assert_eq!(m.get_display("HDMI-1").unwrap().brightness, 70);
        m.brightness_up_all(20);
        assert_eq!(m.get_display("eDP-1").unwrap().brightness, 90);
        assert_eq!(m.get_display("HDMI-1").unwrap().brightness, 90);
    }

    #[test]
    fn brightness_auto() {
        let mut m = Model::new();
        m.set_brightness_curve(vec![0, 25, 50, 75, 100]);
        assert_eq!(m.calculate_auto_brightness(0), 0);
        assert_eq!(m.calculate_auto_brightness(50), 50);
        assert_eq!(m.calculate_auto_brightness(100), 100);
        m.set_auto_brightness(true);
        assert!(m.auto_brightness());
    }
}
