#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AprilAgentActionKind {
    ShellCommand,
    FileRead,
    FileWrite,
    BrowserAction,
    NetworkRequest,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AprilAgentActionReviewability {
    ReviewableCommandText,
    RecognizedButNotReviewableYet,
    UnknownAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AprilAgentActionProposal {
    pub source_kind: String,
    pub action_kind: AprilAgentActionKind,
    pub payload_text: Option<String>,
    pub target: Option<String>,
    pub scope: Option<String>,
    pub request_id: Option<String>,
    pub metadata_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AprilAgentActionFilterResult {
    pub reviewability: AprilAgentActionReviewability,
    pub recognized: bool,
    pub executable: bool,
    pub source_is_authority: bool,
    pub metadata_is_authority: bool,
    pub note: &'static str,
}

pub fn classify_agent_action(proposal: &AprilAgentActionProposal) -> AprilAgentActionFilterResult {
    match proposal.action_kind {
        AprilAgentActionKind::ShellCommand => classify_shell_command(proposal),
        AprilAgentActionKind::FileRead
        | AprilAgentActionKind::FileWrite
        | AprilAgentActionKind::BrowserAction
        | AprilAgentActionKind::NetworkRequest => recognized_but_not_reviewable_yet(),
        AprilAgentActionKind::Unknown => unknown_action(),
    }
}

fn classify_shell_command(proposal: &AprilAgentActionProposal) -> AprilAgentActionFilterResult {
    match proposal.payload_text.as_deref().map(str::trim) {
        Some(command_text) if !command_text.is_empty() => AprilAgentActionFilterResult {
            reviewability: AprilAgentActionReviewability::ReviewableCommandText,
            recognized: true,
            executable: false,
            source_is_authority: false,
            metadata_is_authority: false,
            note: "shell command proposal is reviewable as command-shaped text only",
        },
        _ => AprilAgentActionFilterResult {
            reviewability: AprilAgentActionReviewability::RecognizedButNotReviewableYet,
            recognized: true,
            executable: false,
            source_is_authority: false,
            metadata_is_authority: false,
            note: "shell command proposal is missing visible command text",
        },
    }
}

fn recognized_but_not_reviewable_yet() -> AprilAgentActionFilterResult {
    AprilAgentActionFilterResult {
        reviewability: AprilAgentActionReviewability::RecognizedButNotReviewableYet,
        recognized: true,
        executable: false,
        source_is_authority: false,
        metadata_is_authority: false,
        note: "recognized action kind is not reviewable by this bridge yet",
    }
}

fn unknown_action() -> AprilAgentActionFilterResult {
    AprilAgentActionFilterResult {
        reviewability: AprilAgentActionReviewability::UnknownAction,
        recognized: false,
        executable: false,
        source_is_authority: false,
        metadata_is_authority: false,
        note: "unknown action kind remains explicit",
    }
}
