pub enum Duty {
    UnpluggedSink,
    UnpluggedMic,
    Hidden,
}

pub struct GlobalRemoveInformation {
    pub description: String,
    pub icon: String,
    pub duty: Duty,
    pub _hook: Option<u32>,
}
