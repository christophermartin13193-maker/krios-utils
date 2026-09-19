pub enum Duty {
    UnpluggedSink,
    UnpluggedMic,
    Hidden,
}

pub struct GlobalRemoveInformation {
    pub description: String,
    pub icon: String,
    pub duty: Duty,
    pub hook: Option<u32>,
}
