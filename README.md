Showing that actix middlewares won't run if server functions aren't called from the client side. 

Middlewares won't run in:
- Async Rendering
- In-Order Streaming
- Out-Of-Order Streaming

Middlewares will run in:
- Synchronous Rendering
