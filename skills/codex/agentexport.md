---
name: agentexport
description: Publish this Codex session transcript using agentexport.
allowed-tools: Bash(agentexport:*)
---

# Agent Export

Publish the current Codex session transcript using agentexport.

## Instructions

1. Generate a short, descriptive title for this session (max 60 chars). Summarize what was accomplished or discussed. Examples: "Implement user authentication", "Debug API rate limiting", "Refactor database schema".

2. Use the agentexport CLI to publish the current Codex session transcript, passing the title:

```
agentexport publish --tool codex --max-age-minutes 120 --title "<your title here>"
```

The CLI automatically finds the transcript for the current working directory.

## Managing Shares

To list or delete previously shared transcripts:

```
agentexport shares
```
