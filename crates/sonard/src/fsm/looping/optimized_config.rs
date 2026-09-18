use crate::config::{Config, MicConfig, SinksConfig};

fn get_optimized_sinks(sinks: Option<SinksConfig>) -> (String, String, bool, bool) {
    const DEFAULT_SINK_PLUGGED: &str = "audio-speakers";
    const DEFAULT_SINK_UNPLUGGED: &str = "dialog-warning";

    match sinks {
        None => (
            DEFAULT_SINK_PLUGGED.to_string(),
            DEFAULT_SINK_UNPLUGGED.to_string(),
            false,
            false,
        ),
        Some(sinks) => {
            let (sinks_plugged_icon, sinks_plugged_flag) = match sinks.plugged {
                None => (DEFAULT_SINK_PLUGGED.to_string(), false),
                Some(plugged) => {
                    let icon = if plugged.is_empty() {
                        DEFAULT_SINK_PLUGGED.to_string()
                    } else {
                        plugged
                    };
                    (icon, true)
                }
            };

            let (sinks_unplugged_icon, sinks_unplugged_flag) = match sinks.unplugged {
                None => (DEFAULT_SINK_UNPLUGGED.to_string(), false),
                Some(unplugged) => {
                    let icon = if unplugged.is_empty() {
                        DEFAULT_SINK_UNPLUGGED.to_string()
                    } else {
                        unplugged
                    };
                    (icon, true)
                }
            };

            (
                sinks_plugged_icon,
                sinks_unplugged_icon,
                sinks_plugged_flag,
                sinks_unplugged_flag,
            )
        }
    }
}

fn get_optimized_mic(
    mic: Option<MicConfig>,
) -> (String, String, String, String, bool, bool, bool, bool) {
    const DEFAULT_MIC_PLUGGED: &str = "audio-input-microphone";
    const DEFAULT_MIC_UNPLUGGED: &str = "dialog-warning";
    const DEFAULT_MIC_MUTED: &str = "audio-input-microphone";
    const DEFAULT_MIC_OPENED: &str = "audio-input-microphone";

    match mic {
        None => (
            DEFAULT_MIC_PLUGGED.to_string(),
            DEFAULT_MIC_UNPLUGGED.to_string(),
            DEFAULT_MIC_MUTED.to_string(),
            DEFAULT_MIC_OPENED.to_string(),
            false,
            false,
            false,
            false,
        ),
        Some(mic) => {
            let (mic_plugged_icon, mic_plugged_flag) = match mic.plugged {
                None => (DEFAULT_MIC_PLUGGED.to_string(), false),
                Some(plugged) => {
                    let icon = if plugged.is_empty() {
                        DEFAULT_MIC_PLUGGED.to_string()
                    } else {
                        plugged
                    };
                    (icon, true)
                }
            };

            let (mic_unplugged_icon, mic_unplugged_flag) = match mic.unplugged {
                None => (DEFAULT_MIC_UNPLUGGED.to_string(), false),
                Some(unplugged) => {
                    let icon = if unplugged.is_empty() {
                        DEFAULT_MIC_UNPLUGGED.to_string()
                    } else {
                        unplugged
                    };
                    (icon, true)
                }
            };

            let (mic_muted_icon, mic_muted_flag) = match mic.muted {
                None => (DEFAULT_MIC_MUTED.to_string(), false),
                Some(muted) => {
                    let icon = if muted.is_empty() {
                        DEFAULT_MIC_MUTED.to_string()
                    } else {
                        muted
                    };
                    (icon, true)
                }
            };

            let (mic_opened_icon, mic_opened_flag) = match mic.open {
                None => (DEFAULT_MIC_OPENED.to_string(), false),
                Some(opened) => {
                    let icon = if opened.is_empty() {
                        DEFAULT_MIC_OPENED.to_string()
                    } else {
                        opened
                    };
                    (icon, true)
                }
            };

            (
                mic_plugged_icon,
                mic_unplugged_icon,
                mic_muted_icon,
                mic_opened_icon,
                mic_plugged_flag,
                mic_unplugged_flag,
                mic_muted_flag,
                mic_opened_flag,
            )
        }
    }
}

pub struct OptimizedConfig {
    // flags
    pub sinks_plugged_flag: bool,
    pub sinks_unplugged_flag: bool,
    pub mic_plugged_flag: bool,
    pub mic_unplugged_flag: bool,
    pub mic_opened_flag: bool,
    pub mic_muted_flag: bool,

    // icons
    pub sinks_plugged_icon: String,
    pub sinks_unplugged_icon: String,
    pub mic_plugged_icon: String,
    pub mic_unplugged_icon: String,
    pub mic_opened_icon: String,
    pub mic_muted_icon: String,
}

impl OptimizedConfig {
    pub fn new(config: Config) -> Self {
        let (sinks_plugged_icon, sinks_unplugged_icon, sinks_plugged_flag, sinks_unplugged_flag) =
            get_optimized_sinks(config.sinks);
        let (
            mic_plugged_icon,
            mic_unplugged_icon,
            mic_muted_icon,
            mic_opened_icon,
            mic_plugged_flag,
            mic_unplugged_flag,
            mic_muted_flag,
            mic_opened_flag,
        ) = get_optimized_mic(config.mic);

        OptimizedConfig {
            sinks_plugged_flag,
            sinks_unplugged_flag,
            mic_muted_icon,
            mic_opened_icon,
            mic_plugged_flag,
            mic_unplugged_flag,
            sinks_plugged_icon,
            sinks_unplugged_icon,
            mic_plugged_icon,
            mic_unplugged_icon,
            mic_muted_flag,
            mic_opened_flag,
        }
    }
}
