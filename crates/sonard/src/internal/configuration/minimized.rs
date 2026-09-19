use super::OptimizedConfiguration;

pub struct MinimizedConfiguration {
    pub muted_flag: bool,
    pub unmuted_flag: bool,

    pub muted_icon: String,
    pub unmuted_icon: String,
}

impl OptimizedConfiguration {
    pub fn get_minimized_configuration(&self) -> MinimizedConfiguration {
        MinimizedConfiguration {
            muted_flag: self.optimized_mic.muted_flag,
            unmuted_flag: self.optimized_mic.unmuted_flag,
            muted_icon: self.optimized_mic.muted_icon.clone(),
            unmuted_icon: self.optimized_mic.unmuted_icon.clone(),
        }
    }
}
