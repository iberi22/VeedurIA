# PROMPT: Protocol Synchronization & Reusability Agent

**Context**:
The Git-Core Protocol is currently distributed as a "monolithic dump" of all files from its source repository into the consumer's repository. This includes internal development workflows, telemetry for the protocol itself, and one-time setup scripts that clutter the user's project.

**Goal**:
Refactor the Protocol Distribution System to distinguish between "Protocol Consumer" and "Protocol Developer".

**Tasks for the Agent**:
1. **Modular Profiles**: Implement a profile system (e.g., `protocol.config.json`) that allows the consumer to select modules: `core-sync`, `ai-automation`, `security-quarantine`, `full-infra`.
2. **Post-Install Cleanup**: The installer should automatically flag/remove scripts like `init_project` and `setup-labels` after initial success.
3. **Rust CLI Primacy**: Enhance the protocol's check to detect if the `gc` Rust binary is present. If so, it should strictly hide or remove the legacy `.ps1/.sh` scripts to reduce clutter.
4. **Namespace Isolation**: Move protocol-internal telemetry and propagation logic to a hidden `.ai-core/internal` or similar to avoid polluting the user's root `scripts/`.
5. **Reusability Logic**: Standardize the `update-protocol.yml` to allow the user to point to their own "Forked Protocol" source repo instead of being locked to the hardcoded source.

**Output required**:
An implementation plan for a "Lean Integration" model that keeps the consumer repo focused on project code, with the protocol acting as an invisible but powerful engine.
