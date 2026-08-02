import subprocess, shlex, textwrap, os, sys
from pathlib import Path

home = Path.home()
open_diy = home / "projects/Open-DIY"
command_file = open_diy / ".rtk_command.json"
patch_file = open_diy / ".rtk_patch.json"

for p in [command_file, patch_file]:
    if p.exists():
        p.unlink()

cmd = textwrap.dedent(
    f"""
    set +H
    {shlex.quote(str(home / ".local/bin/agy"))} \\
      --add-dir {shlex.quote(str(open_diy))} \\
      --dangerously-skip-permissions \\
      --print-timeout 480s \\
      --print "Add OpenTelemetry tracing for the Open-DIY axum server via a new server feature only. Requirements: keep changes minimal and repo-safe; mirror the tracing setup already used in ~/projects/open-donate (look at its src/main.rs and Cargo.toml feature flags): 1) introduce an opt-in cargo feature named trailing_telemetry with optional dependencies tracing, tracing-subscriber, tracing-opentelemetry, opentelemetry, opentelemetry_sdk, opentelemetry-otlp, opentelemetry-appender-tracing, axum-tracing-opentelemetry under ssr feature, 2) when the feature is enabled, initialize tracing in the existing main.rs before the router is built, exporting traces to OTEL_EXPORTER_OTLP_ENDPOINT if set and otherwise defaulting to stdout, 3) wrap the axum router with otel tracing middleware, 4) do not change any UI code, do not reformat unrelated code, and avoid removing existing behavior. After editing, run cargo check to confirm the project compiles."
    """
).strip() + "\n"

print("CMD_FILE=", command_file)
print("PATCH_FILE=", patch_file)
