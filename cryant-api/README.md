# Cyrant API

This is the API Service for Cyrant. It is built with Axum.

This repo cargo will consume all the packages in the workspace to build the API system.

## Development

To run the development server, run the following command:

```bash
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

## Deployment