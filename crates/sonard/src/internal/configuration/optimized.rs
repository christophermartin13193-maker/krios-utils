use super::{MicConfiguration, SinksConfiguration, ValidConfiguration};
use crate::internal::icons::*;

pub struct OptimizedConfiguration {
    pub optimized_sinks: OptimizedSinks,
    pub optimized_mic: OptimizedMic,
}

pub struct OptimizedSinks {
    pub plugged_flag: bool,
    pub unplugged_flag: bool,
    pub current_flag: bool,

    pub plugged_icon: String,
    pub unplugged_icon: String,
    pub current_icon: String,
}
pub struct OptimizedMic {
    pub plugged_flag: bool,
    pub unplugged_flag: bool,
    pub muted_flag: bool,
    pub unmuted_flag: bool,

    pub plugged_icon: String,
    pub unplugged_icon: String,
    pub muted_icon: String,
    pub unmuted_icon: String,
}

impl ValidConfiguration {
    pub fn get_optimized_configuration(self) -> OptimizedConfiguration {
        let optimized_sinks = get_optimized_sinks(self.sinks);
        let optimized_mic = get_optimized_mic(self.mic);

        OptimizedConfiguration {
            optimized_sinks,
            optimized_mic,
        }
    }
}

fn get_optimized_sinks(sinks: Option<SinksConfiguration>) -> OptimizedSinks {
    match sinks {
        None => OptimizedSinks {
            plugged_flag: false,
            unplugged_flag: false,
            current_flag: false,
            plugged_icon: DEFAULT_SINK_PLUGGED.to_string(),
            unplugged_icon: DEFAULT_SINK_UNPLUGGED.to_string(),
            current_icon: DEFAULT_SINK_CURRENT.to_string(),
        },
        Some(sinks) => {
            let plugged_flag = sinks.plugged.is_some();
            let plugged_icon = sinks
                .plugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_SINK_PLUGGED.to_string());

            let unplugged_flag = sinks.unplugged.is_some();
            let unplugged_icon = sinks
                .unplugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_SINK_UNPLUGGED.to_string());

            let current_flag = sinks.current.is_some();
            let current_icon = sinks
                .current
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_SINK_CURRENT.to_string());

            OptimizedSinks {
                plugged_flag,
                unplugged_flag,
                current_flag,
                plugged_icon,
                unplugged_icon,
                current_icon,
            }
        }
    }
}

fn get_optimized_mic(mic: Option<MicConfiguration>) -> OptimizedMic {
    match mic {
        None => OptimizedMic {
            plugged_flag: false,
            unplugged_flag: false,
            muted_flag: false,
            unmuted_flag: false,
            plugged_icon: DEFAULT_MIC_PLUGGED.to_string(),
            unplugged_icon: DEFAULT_MIC_UNPLUGGED.to_string(),
            muted_icon: DEFAULT_MIC_MUTED.to_string(),
            unmuted_icon: DEFAULT_MIC_UNMUTED.to_string(),
        },
        Some(mic) => {
            let plugged_flag = mic.plugged.is_some();
            let plugged_icon = mic
                .plugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_PLUGGED.to_string());

            let unplugged_flag = mic.unplugged.is_some();
            let unplugged_icon = mic
                .unplugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_UNPLUGGED.to_string());

            let muted_flag = mic.muted.is_some();
            let muted_icon = mic
                .muted
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_MUTED.to_string());

            let unmuted_flag = mic.unmuted.is_some();
            let unmuted_icon = mic
                .unmuted
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_UNMUTED.to_string());

            OptimizedMic {
                plugged_flag,
                unplugged_flag,
                muted_flag,
                unmuted_flag,
                plugged_icon,
                unplugged_icon,
                muted_icon,
                unmuted_icon,
            }
        }
    }
}
