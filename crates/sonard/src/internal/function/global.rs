use std::sync::Arc;

use notify_rust::Notification;
use pipewire_native::{
    permission::PermissionBits,
    properties::Properties,
    proxy::node::{Node, NodeEvents},
};
use pipewire_native_spa::param::ParamType;

use crate::internal::SonardError;

use super::{
    Duty, GlobalRemoveInformation, Index, SharedMinimized, SharedOptimized, SharedRegistry,
    get_func,
};

type Global = Option<Box<dyn FnMut(u32, PermissionBits, &str, u32, &Properties) + Send + 'static>>;

pub fn get_global(
    optimized: SharedOptimized,
    minimized: SharedMinimized,
    index: Index,
    registry: SharedRegistry,
) -> Global {
    Some(Box::new(
        move |id, _permissions, type_name, version, props| {
            let media_class = match props.get("media.class") {
                None => return,
                Some(media_class) => media_class,
            };

            let description = props
                .get("node.description")
                .unwrap_or("Unknown")
                .to_string();

            match media_class {
                "Audio/Sink" => {
                    if optimized.optimized_sinks.plugged_flag
                        && let Err(error) = Notification::new()
                            .summary("Audio Sink")
                            .body(&format!("Device connected: {description}"))
                            .icon(&optimized.optimized_sinks.plugged_icon)
                            .show()
                    {
                        eprintln!("Notification error: {error}")
                    }
                    if optimized.optimized_sinks.unplugged_flag
                        && let Ok(mut map) = index.lock()
                    {
                        map.insert(
                            id,
                            GlobalRemoveInformation {
                                description: description.clone(),
                                icon: optimized.optimized_sinks.unplugged_icon.clone(),
                                duty: Duty::UnpluggedSink,
                                _hook: None,
                            },
                        );
                    }
                }

                "Audio/Source" => {
                    if optimized.optimized_mic.plugged_flag
                        && let Err(error) = Notification::new()
                            .summary("Microphone")
                            .body(&format!("Device connected: {description}"))
                            .icon(&optimized.optimized_mic.plugged_icon)
                            .show()
                    {
                        eprintln!("Notification error: {error}")
                    }

                    let node: Option<Node> = if optimized.optimized_mic.muted_flag
                        || optimized.optimized_mic.unmuted_flag
                    {
                        match registry.bind(id, type_name, version) {
                            Err(_) => {
                                eprintln!("{}", SonardError::Bind);
                                None
                            }
                            Ok(node) => node.downcast::<Node>(),
                        }
                    } else {
                        None
                    };

                    let hook = if let Some(node) = node {
                        let param = get_func(Arc::clone(&minimized), description.clone());
                        let hook = node.add_listener(NodeEvents { info: None, param });

                        let _ = node.subscribe_params(&[ParamType::Props]);
                        Some(hook)
                    } else {
                        None
                    };

                    if (optimized.optimized_mic.unplugged_flag || hook.is_some())
                        && let Ok(mut map) = index.lock()
                    {
                        let duty = if optimized.optimized_mic.unplugged_flag {
                            Duty::UnpluggedMic
                        } else {
                            Duty::Hidden
                        };

                        map.insert(
                            id,
                            GlobalRemoveInformation {
                                description,
                                icon: optimized.optimized_mic.unplugged_icon.clone(),
                                duty,
                                _hook: hook,
                            },
                        );
                    }
                }

                _ => (),
            }
        },
    ))
}
