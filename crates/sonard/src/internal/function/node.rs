use super::SharedMinimized;
use notify_rust::Notification;
use pipewire_native_spa::{
    param::{ParamType, props::Prop},
    pod::{
        self, RawPodOwned,
        parser::{ObjectParser, Parser},
    },
};

type NodeEventsParam = Option<Box<dyn FnMut(u32, ParamType, u32, u32, &RawPodOwned) + Send>>;

fn parse_object(raw_pod_owned: &RawPodOwned) -> Result<(Option<bool>, usize), pod::Error> {
    let mut parser = Parser::new(raw_pod_owned.data());

    parser.pop_object(|object_parser: &mut ObjectParser<Prop>, _id: u32| {
        for (key, _, raw_pod) in object_parser {
            if key == Prop::Mute {
                let mut prop_parser = Parser::new(raw_pod.data());
                if let Ok(is_muted) = prop_parser.pop_bool() {
                    return Ok(Some(is_muted));
                }
            }
        }
        Ok(None)
    })
}

pub fn get_func(minimized: SharedMinimized, description: String) -> NodeEventsParam {
    Some(Box::new(
        move |_id, _param_type, _index, _next, raw_pod_owned| {
            match parse_object(raw_pod_owned) {
                Ok((Some(is_muted), _)) => {
                    if is_muted
                        && minimized.muted_flag
                        && let Err(error) = Notification::new()
                            .summary("Microphone")
                            .body(&format!("Device muted: {description}"))
                            .icon(&minimized.muted_icon)
                            .show()
                    {
                        eprintln!("Notification error: {error}")
                    } else if !is_muted
                        && minimized.unmuted_flag
                        && let Err(error) = Notification::new()
                            .summary("Microphone")
                            .body(&format!("Device unmuted: {description}"))
                            .icon(&minimized.unmuted_icon)
                            .show()
                    {
                        eprintln!("Notification error: {error}")
                    }
                }
                _ => (),
            };
        },
    ))
}
