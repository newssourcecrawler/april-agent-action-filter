use april_agent_action_filter::{
    classify_agent_action, AprilAgentActionKind, AprilAgentActionProposal,
    AprilAgentActionReviewability,
};

fn proposal(action_kind: AprilAgentActionKind) -> AprilAgentActionProposal {
    AprilAgentActionProposal {
        source_kind: "openclaw_like".to_string(),
        action_kind,
        payload_text: None,
        target: None,
        scope: None,
        request_id: None,
        metadata_present: false,
    }
}

#[test]
fn shell_command_with_payload_is_reviewable_as_command_text() {
    let mut proposal = proposal(AprilAgentActionKind::ShellCommand);
    proposal.payload_text = Some("cat input.csv".to_string());

    let result = classify_agent_action(&proposal);

    assert_eq!(
        result.reviewability,
        AprilAgentActionReviewability::ReviewableCommandText
    );
    assert!(result.recognized);
    assert!(!result.executable);
    assert!(!result.source_is_authority);
    assert!(!result.metadata_is_authority);
}

#[test]
fn shell_command_without_payload_is_not_reviewable_yet() {
    let proposal = proposal(AprilAgentActionKind::ShellCommand);

    let result = classify_agent_action(&proposal);

    assert_eq!(
        result.reviewability,
        AprilAgentActionReviewability::RecognizedButNotReviewableYet
    );
    assert!(result.recognized);
    assert!(!result.executable);
}

#[test]
fn file_write_is_recognized_but_not_executed() {
    let proposal = proposal(AprilAgentActionKind::FileWrite);

    let result = classify_agent_action(&proposal);

    assert_eq!(
        result.reviewability,
        AprilAgentActionReviewability::RecognizedButNotReviewableYet
    );
    assert!(result.recognized);
    assert!(!result.executable);
}

#[test]
fn network_request_is_recognized_but_not_executed() {
    let proposal = proposal(AprilAgentActionKind::NetworkRequest);

    let result = classify_agent_action(&proposal);

    assert_eq!(
        result.reviewability,
        AprilAgentActionReviewability::RecognizedButNotReviewableYet
    );
    assert!(result.recognized);
    assert!(!result.executable);
}

#[test]
fn unknown_action_kind_remains_unknown() {
    let proposal = proposal(AprilAgentActionKind::Unknown);

    let result = classify_agent_action(&proposal);

    assert_eq!(
        result.reviewability,
        AprilAgentActionReviewability::UnknownAction
    );
    assert!(!result.recognized);
    assert!(!result.executable);
}

#[test]
fn metadata_does_not_affect_classification() {
    let mut proposal = proposal(AprilAgentActionKind::FileWrite);
    proposal.metadata_present = true;

    let result = classify_agent_action(&proposal);

    assert_eq!(
        result.reviewability,
        AprilAgentActionReviewability::RecognizedButNotReviewableYet
    );
    assert!(!result.metadata_is_authority);
    assert!(!result.executable);
}

#[test]
fn source_kind_does_not_grant_authority() {
    let mut proposal = proposal(AprilAgentActionKind::ShellCommand);
    proposal.source_kind = "trusted_agent".to_string();
    proposal.payload_text = Some("echo hello".to_string());

    let result = classify_agent_action(&proposal);

    assert_eq!(
        result.reviewability,
        AprilAgentActionReviewability::ReviewableCommandText
    );
    assert!(!result.source_is_authority);
    assert!(!result.executable);
}
