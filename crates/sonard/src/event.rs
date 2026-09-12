use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use notify_rust::Notification;
use pipewire_native::{permission::PermissionBits, properties::Properties};

type Sinks = Arc<Mutex<HashMap<u32, String>>>;
type Global = Option<Box<dyn FnMut(u32, PermissionBits, &str, u32, &Properties) + Send + 'static>>;
type GlobalRemove = Option<Box<dyn FnMut(u32) + Send + 'static>>;

pub fn get_global(sinks: Sinks) -> Global {
    let sinks = Arc::clone(&sinks);

    Some(Box::new(
        move |id, _permissions, _type_name, _version, props| {
            // Only the media_class "Audio/Sinks" is interesting for us
            if let Some(media_class) = props.get("media.class")
                && media_class == "Audio/Sink"
            {
                let description = props
                    .get("node.description")
                    .unwrap_or("Unknown")
                    .to_string();

                // Notification
                if let Err(error) = Notification::new()
                    .summary("Audio Sink")
                    .body(&format!("Device Connected : {description}"))
                    .icon("audio-speakers")
                    .show()
                {
                    eprintln!("Notification error : {error}")
                }

                // Sinks
                if let Ok(mut map) = sinks.lock() {
                    map.insert(id, description);
                }
            }
        },
    ))
}

pub fn get_global_remove(sinks: Arc<Mutex<HashMap<u32, String>>>) -> GlobalRemove {
    let sinks = Arc::clone(&sinks);

    Some(Box::new(move |id| {
        // Only the known id's are interesting for us
        if let Ok(mut map) = sinks.lock()
            && let Some(description) = map.remove(&id)
        {
            // Notification
            if let Err(error) = Notification::new()
                .summary("Audio Sink")
                .body(&format!("Device Disconnected : {description}"))
                .icon("dialog-warning")
                .show()
            {
                eprintln!("Notification error : {error}")
            }
        }
    }))
}
