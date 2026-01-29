# Next Steps Plan (Updated with ACP Communication)

**Last Updated**: 2026-01-29
**Communication Method**: ACP + Stdio (Recommended)
**Estimated Time**: 7 days

---

## Current Status

### Completed Work
- ✅ Phase 1-7: Basic Architecture and Components
- ✅ P0.1: Verify CodeBuddy Code Communication Support

### Verification Results
- **CodeBuddy Code Version**: 2.41.6
- **Supports WebSocket**: Not needed (ACP is better)
- **Supports JSON Output**: ✅ Yes (text, json, stream-json)
- **Supports Special Markers**: ❌ No
- **Supports MCP**: ✅ Yes (full support)
- **Supports `--acp`**: ✅ Yes (ACP mode)

### Recommended Solution
- **Method**: ACP (Agent Client Protocol) + Stdio
- **Priority**: Highest (P0)
- **Implementation Difficulty**: Low
- **Risk**: Low
- **Performance**: Excellent

---

## P0 Tasks (Critical Blocking)

### P0.1: ✅ Verify CodeBuddy Code Support (Completed)
- **Status**: Done
- **Result**: ACP protocol is supported with `--acp --acp-transport "stdio"`

### P0.2: Implement ACP Communication Layer (2 days)
**File**: `src-tauri/src/bridge/acp_transport.rs` (new)

**Actions**:
1. [ ] Add ndjson dependency to Cargo.toml
2. [ ] Implement ndjson::parser for reading ndjson from stdout
3. [ ] Implement ndjson::writer for writing ndjson to stdin
4. [ ] Create ACP transport module with write() and read() methods
5. [ ] Implement error handling and connection management
6. [ ] Add logging for all operations

**Estimated Time**: 2 days

---

## P1 Tasks (High Priority - Core Features)

### P1.1: Update CodeBuddyAdapter to Use ACP (1 day)
**File**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**Actions**:
1. [ ] Remove WebSocket server code (establish_control_channel)
2. [ ] Add `--acp --acp-transport "stdio"` to startup command
3. [ ] Replace send_message() with ndjson writer
4. [ ] Replace receive loop with ndjson parser
5. [ ] Update all message handling to use ACP protocol
6. [ ] Test bidirectional communication

**Estimated Time**: 1 day

### P1.2: Implement Frontend Event Listening (1 day)
**Files**: `src/stores/agent.ts`, `src/stores/session.ts`, `src/composables/useAgent.ts`

**Actions**:
1. [ ] Implement `useEvent` composable for Tauri events
2. [ ] Listen to `agent-event` events in Agent store
3. [ ] Listen to `session-updated` events in Session store
4. [ ] Handle ACP protocol messages
5. [ ] Update state in response to events
6. [ ] Handle errors gracefully

**Estimated Time**: 1 day

### P1.3: Implement Streaming Data Processing (2 days)
**File**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**Actions**:
1. [ ] Process ndjson "start" message
2. [ ] Process ndjson "data" messages (stream chunks)
3. [ ] Process ndjson "progress" messages
4. [ ] Process ndjson "error" messages
5. [ ] Process ndjson "end" messages
6. [ ] Update SessionManager for each message type
7. [ ] Emit events to frontend via EventEmitter
8. [ ] Implement data batching for large payloads
9. [ ] Prevent memory overflow

**Estimated Time**: 2 days

### P1.4: Complete Frontend Integration (2 days)
**Files**: All frontend components

**Actions**:
1. [ ] OmniBox - Fully integrate with Tauri invoke
2. [ ] SkillDock - Integrate with Agent Store and events
3. [ ] SemanticCanvas - Integrate with Session Store
4. [ ] Implement real-time state updates
5. [ ] Implement autocomplete
6. [ ] Implement input validation
7. [ ] Implement global error notifications (Toast)
8. [ ] Implement keyboard shortcuts

**Estimated Time**: 2 days

---

## P2 Tasks (Medium Priority - Testing & Optimization)

### P2.1: Write Unit Tests (2 days)
**Files**: `src-tauri/src/bridge/acp_transport_test.rs` (new), etc.

**Actions**:
1. [ ] Write ndjson parser tests (target: 90% coverage)
2. [ ] Write ndjson writer tests (target: 80% coverage)
3. [ ] Write protocol tests (target: 90% coverage)
4. [ ] Write session_manager tests (target: 80% coverage)
5. [ ] Write codebuddy_adapter tests (target: 70% coverage)

**Estimated Time**: 2 days

### P2.2: Write Integration Tests (1 day)
**File**: `src-tauri/tests/acp_integration.rs` (new)

**Actions**:
1. [ ] Test Agent startup with ACP
2. [ ] Test Skill discovery via ACP
3. [ ] Test Skill execution via ACP
4. [ ] Test streaming data transmission
5. [ ] Test error scenarios

**Estimated Time**: 1 day

### P2.3: Performance Optimization (2 days)

**Actions**:
1. [ ] Implement virtual scrolling for Table and Log
2. [ ] Batch UI updates (use requestAnimationFrame)
3. [ ] Move protocol parsing to Web Worker
4. [ ] Implement data lazy loading
5. [ ] Optimize memory usage
6. [ ] Performance benchmarking

**Estimated Time**: 2 days

### P2.4: Error Scenario Testing & Fixes (1 day)
**File**: All related files

**Actions**:
1. [ ] Test Agent process crash
2. [ ] Test stdout/stderr disconnection
3. [ ] Test Skill execution failure
4. [ ] Test network errors
5. [ ] Test memory leaks
6. [ ] Verify recovery mechanisms
7. [ ] Fix discovered issues
8. [ ] Regression testing

**Estimated Time**: 1 day

---

## Time Estimation

| Priority | Tasks | Estimated Time | Cumulative |
|----------|-------|---------------|-------------|
| P0 | ACP Communication Layer | 2 days | 2 days |
| P1 | Core Features | 6 days | 8 days |
| P2 | Testing & Optimization | 6 days | 14 days |

**Total**: 14 days (2 weeks)

---

## Milestones

### M1: ACP Communication Complete
- **Target**: Day 2
- **Status**: Pending
- **Acceptance**: P0.1 and P0.2 complete

### M2: Core Features Complete
- **Target**: Day 8
- **Status**: Pending
- **Acceptance**: All P1 tasks complete

### M3: Testing Complete
- **Target**: Day 11
- **Status**: Pending
- **Acceptance**: P2.1 and P2.2 complete

### M4: Performance Optimized
- **Target**: Day 13
- **Status**: Pending
- **Acceptance**: P2.3 complete, benchmarks pass

### M5: MVP Release Ready
- **Target**: Day 14
- **Status**: Pending
- **Acceptance**: All tasks complete, ready for release

---

## Success Criteria

### Functionality
- [ ] CodeBuddy Code can start/stop with ACP mode
- [ ] Can get full Skill list and metadata
- [ ] Can execute Skill with real-time streaming output
- [ ] Supports at least 3 render modes (log, code, table)
- [ ] Supports multiple concurrent Skills
- [ ] Can manage MCP servers

### Stability
- [ ] Process crash recovery works
- [ ] Stdio disconnection recovery works
- [ ] No memory leaks after 1+ hours
- [ ] No lag with 10,000+ log entries

### User Experience
- [ ] Natural language input triggers Skill correctly
- [ ] Streaming output latency < 100ms
- [ ] Error messages are clear with suggestions
- [ ] UI is responsive, no noticeable lag
- [ ] Supports Chinese interface (at least for errors)

### Code Quality
- [ ] Rust unit test coverage >= 80%
- [ ] Integration tests cover main user flows
- [ ] No compilation warnings (warnings as errors)
- [ ] Complies with Rust Clippy
- [ ] TypeScript type-safe, no any types

---

## Implementation Notes

### ACP Protocol Details

**Message Format** (Newline-Delimited JSON - ndjson):

**Request (stdin → CodeBuddy)**:
```json
{"command":"execute_skill","skill_name":"security-review","input":"Review code security"}
```

**Response (CodeBuddy → stdout)**:
```json
{"type":"start","session_id":"uuid","skill_name":"security-review"}
{"type":"data","session_id":"uuid","data":"..."}
{"type":"progress","session_id":"uuid","current":1,"total":10}
{"type":"error","session_id":"uuid","code":"ERROR","message":"...","suggestion":"..."}
{"type":"end","session_id":"uuid","success":true,"summary":"..."}
```

**Event Types**:
- `start`: Session started
- `data`: Streaming data chunk
- `progress`: Progress update
- `error`: Error occurred
- `end`: Session ended

### Key Commands

**Start with ACP mode**:
```bash
codebuddy --acp --acp-transport "stdio"
```

**Get Skills**:
```bash
echo '{"command":"get_skills"}' | codebuddy --acp --acp-transport "stdio"
```

**Execute Skill**:
```bash
echo '{"command":"execute_skill","skill_name":"security-review","input":"Review code security"}' | codebuddy --acp --acp-transport "stdio"
```

---

## Dependencies

### Rust Dependencies
```toml
[dependencies]
# Existing dependencies
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
thiserror = "1"
futures = "0.3"
uuid = { version = "1", features = ["v4", "serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# New dependencies
ndjson = "0.8"
tokio-util = "0.7"  # needed for async utilities
```

### Frontend Dependencies
```json
{
  "dependencies": {
    "ndjson-stream": "^0.6.0"
  }
}
```

---

## Risk Assessment

### High Risks (Mitigated)
- ✅ **CodeBuddy Code communication method** - Verified: ACP + Stdio is fully supported
- ✅ **ACP protocol understanding** - Documented and clear

### Medium Risks
- ⚠️  **Stdio pipe buffering** - May block on high output volume
  - **Mitigation**: Use async reading with proper buffering

- ⚠️ **Frontend event handling** - May miss events under high load
  - **Mitigation**: Implement retry logic and buffering

### Low Risks
- ⚠️ **Test coverage** - Currently at 0%
  - **Mitigation**: Comprehensive test plan included

---

## Next Immediate Actions

### Today
1. [ ] **Update Cargo.toml** with ndjson dependency
2. [ ] **Create ACP transport module** (acp_transport.rs)
3. [ ] **Test ACP communication** with a simple command
4. [ ] **Update CodeBuddyAdapter** to use ACP mode

### This Week
1. [ ] **Phase 1**: ACP Communication Layer (2 days)
2. [ ] **Phase 1**: Frontend Event Listening (1 day)
3. [ ] **Phase 1**: Streaming Data Processing (2 days)

### Next Week
1. [ ] **Phase 1**: Complete Frontend Integration (2 days)
2. [ ] **Phase 2**: Unit Tests (2 days)
3. [ ] **Phase 2**: Integration Tests (1 day)

---

## Questions

1. **Do you want to proceed with the ACP implementation plan?**
2. **Would you like me to start implementing P0.2 (ACP Transport Layer) now?**
3. **Do you want to make any changes to the plan?**
4. **Any concerns about the ACP approach?**

---

**Please let me know if you want me to start implementing the ACP communication layer now, or if you have any questions about the plan!**
