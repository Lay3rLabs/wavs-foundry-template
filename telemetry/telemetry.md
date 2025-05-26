# Telemetry usage

Our Docker Compose configuration now includes:
- **Jaeger all-in-one**: For exporting traces, accessible at [http://localhost:16686](http://localhost:16686).
- **Prometheus**: For collecting and querying metrics, accessible at [http://localhost:9090](http://localhost:9090).

When the log level is set to `debug`, you will notice traces from both `wavs` and `wavs-aggregator` being exported to Jaeger.

For Prometheus:
- Use `{__name__=~".+"}` to query all available metrics.
- The interface supports auto-completion, so typing the name of the specific metric you're looking for will also work.
