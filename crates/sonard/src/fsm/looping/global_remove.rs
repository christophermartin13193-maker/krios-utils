use super::{Duty, GlobalRemove, PipeIndex, optimized_config::OptimizedConfig};
use notify_rust::Notification;

fn get_func(p: PipeIndex) -> GlobalRemove {
    Some(Box::new(move |id| {
        if let Ok(mut map) = p.lock()
            && let Some(info) = map.remove(&id)
        {
            let description = info.description;
            let icon = info.icon;

            match info.duty {
                Duty::UnpluggedMic => {
                    if let Err(err) = Notification::new()
                        .summary("Microphone")
                        .body(&format!("Device disconnected: {description}"))
                        .icon(&icon)
                        .show()
                    {
                        eprintln!("Notification error: {err}")
                    }
                }
                Duty::UnpluggedSink => {
                    if let Err(err) = Notification::new()
                        .summary("Audio Sink")
                        .body(&format!("Device disconnected: {description}"))
                        .icon(&icon)
                        .show()
                    {
                        eprintln!("Notification error: {err}")
                    }
                }
            }
        }
    }))
}

pub fn get_global_remove(config: &OptimizedConfig, p: PipeIndex) -> GlobalRemove {
    if config.sinks_unplugged_flag || config.mic_unplugged_flag {
        get_func(p)
    }
    else {
        None
    }
}
