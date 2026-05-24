

# April Agent Action Filter

April Agent Action Filter is a small Rust library for reviewing computer-use-agent action proposals before they become tool access.

It treats proposed actions as data. It does not execute commands, open files, call tools, make network requests, or run as an agent.


The first use case is simple: an LLM or computer-use agent proposes a local action, and this crate classifies whether that proposal is reviewable by a bounded human-review layer.

OpenClaw-like computer-use agents are one useful pressure test for this boundary. The crate remains generic: it does not depend on OpenClaw, implement OpenClaw APIs, or claim OpenClaw compatibility.

## What it does

April Agent Action Filter accepts an action proposal with fields such as:

- source kind
- action kind
- optional payload text
- optional target
- optional scope
- optional request ID
- metadata presence

It then returns a deterministic reviewability result.

Current action kinds are:

- `shell_command`
- `file_read`
- `file_write`
- `browser_action`
- `network_request`
- `unknown`

Current reviewability outcomes are:

- `ReviewableCommandText`
- `RecognizedButNotReviewableYet`
- `UnknownAction`

A shell-command proposal with visible payload text can be marked as reviewable command-shaped text. It is still not executable.

File, browser, and network actions are recognized but not reviewable by this crate yet. They are never executed.

Unknown actions remain explicit.

## What it does not do

This crate does not:

- execute shell commands
- read files
- write files
- open browsers
- make network requests
- call tools
- provide a daemon
- provide shell access
- generate commands
- grant authority based on source name
- trust metadata as authority
- integrate with OpenClaw
- claim OpenClaw compatibility

OpenClaw-like agents are useful as a pressure-test ecosystem for this boundary. This crate is not an OpenClaw integration.

## Example

```rust
use april_agent_action_filter::{
    classify_agent_action, AprilAgentActionKind, AprilAgentActionProposal,
    AprilAgentActionReviewability,
};

let proposal = AprilAgentActionProposal {
    source_kind: "openclaw_like".to_string(),
    action_kind: AprilAgentActionKind::ShellCommand,
    payload_text: Some("cat input.csv".to_string()),
    target: None,
    scope: None,
    request_id: Some("request-001".to_string()),
    metadata_present: true,
};

let result = classify_agent_action(&proposal);

assert_eq!(
    result.reviewability,
    AprilAgentActionReviewability::ReviewableCommandText
);
assert!(result.recognized);
assert!(!result.executable);
assert!(!result.source_is_authority);
assert!(!result.metadata_is_authority);
```

The proposal is reviewable as command-shaped text, but it is not executable. The source name and metadata do not grant authority.

## Boundary

This crate is intended to sit before tool access.

```text
agent proposal
-> inert data
-> deterministic classification
-> reviewability result
-> no execution
```

The goal is to make the first boundary small and inspectable.

## Status

Experimental. The current crate is intentionally narrow and review-only.

```bash
cargo check
cargo test
```

## License

MIT