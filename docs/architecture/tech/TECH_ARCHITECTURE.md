# SDKWork Video Technical Architecture

Status: draft
Owner: SDKWork maintainers
Updated: 2026-06-24
Specs: ARCHITECTURE_DECISION_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- Add `TECH-<topic>.md` shards in this directory when the architecture grows beyond one reviewable screen.

## 1. Architecture Overview

## 2. Technology Choices

## 3. System Boundaries And Modules

Video generation uses an L2 service, L3 provider SPI, L4 generated-SDK adapter, and L5 caller-owned
composition. SDK route/resource/method names and vendor DTO conversion are private to the adapter.
See `../decisions/ADR-20260719-video-generation-provider-spi.md`.

`sdkwork-video-generation-mcp-service` is the independent agent-facing protocol adapter. It depends
only on `VideoGenerationServicePort` and its task-context store port, and owns the `video.*` MCP
tools, resources, prompt, error mapping, stdio serving, and Streamable HTTP/SSE service builder.

## 4. Directory And Package Layout

```text
crates/sdkwork-video-generation-provider-spi/
crates/sdkwork-video-generation-service/
crates/sdkwork-video-generation-provider-adapter/
crates/sdkwork-video-generation-mcp-service/
```

## 5. API, SDK, And Data Ownership

## 6. Security, Privacy, And Observability

## 7. Deployment And Runtime Topology

MCP hosts choose stdio or Streamable HTTP. SSE is the Streamable HTTP response channel, not a
separate compatibility endpoint. The host owns authentication, authorization, host/origin policy,
limits, observability, graceful shutdown, and production task-context persistence.

## 8. Architecture Decision Index

## 9. Verification

- `cargo test -p sdkwork-video-generation-mcp-service`
- `cargo clippy -p sdkwork-video-generation-mcp-service --all-targets -- -D warnings`
