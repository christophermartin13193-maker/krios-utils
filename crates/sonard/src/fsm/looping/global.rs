use super::{Duty, Global, GlobalRemoveInfo, PipeIndex, optimized_config::OptimizedConfig, Arc};
use notify_rust::Notification;

pub fn get_global(config: Arc<OptimizedConfig>, p: PipeIndex) -> Global {
    Some(Box::new(
        move |id, _permissions, _type_name, _version, props| {
            let media_class = match props.get("media.class") {
                None => return,
                Some(media_class) => media_class,
            };

            match media_class {
                "Audio/Sink" => {
                    // Description
                    let description = props
                        .get("node.description")
                        .unwrap_or("Unknown")
                        .to_string();
                    // Notification
                    if config.sinks_plugged_flag && let Err(error) = Notification::new()
                            .summary("Audio Sink")
                            .body(&format!("Device connected: {description}"))
                            .icon(&config.sinks_plugged_icon)
                            .show()
                        {
                            eprintln!("Notification error: {error}")
                        }
                    // PipeIndex if needed
                    if config.sinks_unplugged_flag && let Ok(mut map) = p.lock() {
                            map.insert(
                                id,
                                GlobalRemoveInfo {
                                    description,
                                    icon: config.sinks_unplugged_icon.clone(),
                                    duty: Duty::UnpluggedSink,
                                },
                            );
                        }
                }

                "Audio/Source" => {
                    // Description
                    let description = props
                        .get("node.description")
                        .unwrap_or("Unknown")
                        .to_string();
                    // Notification
                    if config.mic_plugged_flag && let Err(error) = Notification::new()
                            .summary("Microphone")
                            .body(&format!("Device connected: {description}"))
                            .icon(&config.mic_plugged_icon)
                            .show()
                        {
                            eprintln!("Notification error: {error}")
                        }
                    // PipeIndex if needed
                    if config.mic_unplugged_flag && let Ok(mut map) = p.lock() {
                            map.insert(
                                id,
                                GlobalRemoveInfo {
                                    description,
                                    icon: config.mic_unplugged_icon.clone(),
                                    duty: Duty::UnpluggedMic,
                                },
                            );
                        }
                }
                _ => (),
            }
        },
    ))
}
