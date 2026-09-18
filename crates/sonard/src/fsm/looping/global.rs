use crate::exit::SonardError;

use pipewire_native_spa::{param::props::Prop, pod::parser::ObjectParser};

use super::{
    Arc, Duty, Global, GlobalRemoveInfo, PipeIndex, SharedRegistry,
    optimized_config::OptimizedConfig,
};
use notify_rust::Notification;
use pipewire_native::proxy::node::{Node, NodeEvents};
use pipewire_native_spa::{
    param::ParamType,
    pod::{RawPodOwned, parser::Parser},
};

type NodeEventsParam = Option<Box<dyn FnMut(u32, ParamType, u32, u32, &RawPodOwned) + Send>>;

pub fn get_global(config: Arc<OptimizedConfig>, p: PipeIndex, s: SharedRegistry) -> Global {
    Some(Box::new(
        move |id, _permissions, type_name, version, props| {
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
                    if config.sinks_plugged_flag
                        && let Err(error) = Notification::new()
                            .summary("Audio Sink")
                            .body(&format!("Device connected: {description}"))
                            .icon(&config.sinks_plugged_icon)
                            .show()
                    {
                        eprintln!("Notification error: {error}")
                    }
                    // PipeIndex if needed
                    if config.sinks_unplugged_flag
                        && let Ok(mut map) = p.lock()
                    {
                        map.insert(
                            id,
                            GlobalRemoveInfo {
                                description,
                                icon: config.sinks_unplugged_icon.clone(),
                                duty: Duty::UnpluggedSink,
                                node_id: None,
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
                    if config.mic_plugged_flag
                        && let Err(error) = Notification::new()
                            .summary("Microphone")
                            .body(&format!("Device connected: {description}"))
                            .icon(&config.mic_plugged_icon)
                            .show()
                    {
                        eprintln!("Notification error: {error}")
                    }
                    // Mute/Opened
                    let node: Result<Node, SonardError> =
                        if config.mic_muted_flag || config.mic_opened_flag {
                            let node_opt = {
                                if let Ok(s) = s.lock() {
                                    s.bind(id, type_name, version)
                                        .ok()
                                        .and_then(|proxy| proxy.downcast::<Node>())
                                } else {
                                    None
                                }
                            };
                            node_opt.ok_or(SonardError::RetrievingWrapper)
                        } else {
                            Err(SonardError::PlaceHolder)
                        };

                    let description_for_mute = description.clone();
                    let mute_icon = config.mic_muted_icon.clone();
                    let open_icon = config.mic_opened_icon.clone();

                    let mute_flag = config.mic_muted_flag;
                    let open_flag = config.mic_opened_flag;

                    let node_id = if let Ok(node) = node {
                        let param: NodeEventsParam = Some(Box::new(
                            move |_id, _param_type, _index, _next, raw_pod_owned| {
                                let mut parser = Parser::new(raw_pod_owned.data());

                                let res: Result<
                                    (Option<bool>, usize),
                                    pipewire_native_spa::pod::Error,
                                > = parser.pop_object(
                                    |object_parser: &mut ObjectParser<Prop>, _id: u32| {
                                        for (key, _flags, raw_pod) in object_parser {
                                            if key == Prop::Mute {
                                                let mut prop_parser = Parser::new(raw_pod.data());
                                                if let Ok(is_muted) = prop_parser.pop_bool() {
                                                    return Ok(Some(is_muted));
                                                }
                                            }
                                        }
                                        Ok(None)
                                    },
                                );
                                match res {
                                    Ok((Some(is_muted), _)) => {
                                        if is_muted
                                            && mute_flag
                                            && let Err(error) = Notification::new()
                                                .summary("Microphone")
                                                .body(&format!(
                                                    "Device muted: {description_for_mute}"
                                                ))
                                                .icon(&mute_icon)
                                                .show()
                                        {
                                            eprintln!("Notification error: {error}")
                                        } else if !is_muted
                                            && open_flag
                                            && let Err(error) = Notification::new()
                                                .summary("Microphone")
                                                .body(&format!(
                                                    "Device open: {description_for_mute}"
                                                ))
                                                .icon(&open_icon)
                                                .show()
                                        {
                                            eprintln!("Notification error: {error}")
                                        }
                                    }
                                    _ => (),
                                };
                            },
                        ));

                        let hook_id = node.add_listener(NodeEvents { info: None, param });
                        let _ = node.subscribe_params(&[ParamType::Props]);

                        Some(hook_id)
                    } else {
                        None
                    };

                    // PipeIndex if needed
                    if (config.mic_unplugged_flag || node_id.is_some())
                        && let Ok(mut map) = p.lock()
                    {
                        let duty = if config.mic_unplugged_flag {
                            Duty::UnpluggedMic
                        } else {
                            Duty::Hidden
                        };
                        map.insert(
                            id,
                            GlobalRemoveInfo {
                                description,
                                icon: config.mic_unplugged_icon.clone(),
                                duty,
                                node_id,
                            },
                        );
                    }
                }
                _ => (),
            }
        },
    ))
}
