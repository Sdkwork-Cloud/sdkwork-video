# SDKWork Video Generation MCP Service

Provider-neutral MCP protocol adapter for `sdkwork-video-generation-service`.

- Tools: `video.generate`, `video.retrieve`, `video.cancel`, `video.capabilities`
- Resources: `sdkwork://video/generation/capabilities`, `sdkwork://video/generation/vendors`
- Prompt: `video.generation.request`
- Transports: stdio and MCP Streamable HTTP with SSE delivery

Authentication, authorization, origin validation, rate limits, tracing, listener binding, and
graceful shutdown belong to the composition root that mounts this service.
