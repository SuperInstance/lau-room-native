//! lau-room-native: The room IS the agent's context.
//!
//! When a zeroshot agent beams into a room, it gets the baton pass from the
//! last specialist. The room's controls, help files, wiki pages, and
//! instructions are all positioned where they're used — like a well-organized
//! workshop where the manual sits next to the machine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// ---------------------------------------------------------------------------
// RoomId
// ---------------------------------------------------------------------------

/// Unique identifier for a room.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomId(pub String);

impl fmt::Display for RoomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RoomId {
    fn from(s: String) -> Self {
        RoomId(s)
    }
}

impl From<&str> for RoomId {
    fn from(s: &str) -> Self {
        RoomId(s.to_string())
    }
}

// ---------------------------------------------------------------------------
// RoomRole
// ---------------------------------------------------------------------------

/// What this room does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomRole {
    Engineering,
    Bridge,
    Science,
    Security,
    Operations,
    Navigation,
    Medical,
    Custom(String),
}

impl RoomRole {
    /// Default intention focus for this role.
    pub fn default_intention_focus(&self) -> Vec<String> {
        match self {
            RoomRole::Engineering => vec![
                "hardware repair".into(),
                "motor calibration".into(),
                "GPIO configuration".into(),
                "infrastructure maintenance".into(),
            ],
            RoomRole::Bridge => vec![
                "command coordination".into(),
                "status overview".into(),
                "crew management".into(),
                "decision making".into(),
            ],
            RoomRole::Science => vec![
                "analysis".into(),
                "conservation verification".into(),
                "pattern detection".into(),
                "data collection".into(),
            ],
            RoomRole::Security => vec![
                "monitoring".into(),
                "safety enforcement".into(),
                "alert response".into(),
                "override access".into(),
            ],
            RoomRole::Operations => vec![
                "scheduling".into(),
                "logistics".into(),
                "communication".into(),
                "resource allocation".into(),
            ],
            RoomRole::Navigation => vec![
                "course plotting".into(),
                "terrain analysis".into(),
                "waypoint management".into(),
                "mapping".into(),
            ],
            RoomRole::Medical => vec![
                "diagnostics".into(),
                "health assessment".into(),
                "repair procedures".into(),
                "treatment".into(),
            ],
            RoomRole::Custom(_) => vec!["general tasks".into()],
        }
    }

    /// Default controls for this role.
    fn default_controls(&self) -> Vec<Control> {
        match self {
            RoomRole::Engineering => vec![
                Control::new("motor-throttle", ControlType::Slider(0.0, 100.0)),
                Control::new("gpio-toggle", ControlType::Toggle(false)),
                Control::new("sensor-read", ControlType::Button("Read Sensors".into())),
                Control::new("diagnostics", ControlType::Display("All systems nominal".into())),
            ],
            RoomRole::Bridge => vec![
                Control::new("status-display", ControlType::Display("Ship status: GREEN".into())),
                Control::new("crew-alert", ControlType::Selector(vec!["Red".into(), "Yellow".into(), "Green".into()])),
                Control::new("command-input", ControlType::Input("".into())),
            ],
            RoomRole::Science => vec![
                Control::new("scan-input", ControlType::Input("".into())),
                Control::new("analysis-mode", ControlType::Selector(vec!["Spectral".into(), "Biological".into(), "Chemical".into()])),
                Control::new("results-display", ControlType::Display("No data".into())),
            ],
            RoomRole::Security => vec![
                Control::new("alert-level", ControlType::Selector(vec!["Normal".into(), "Elevated".into(), "High".into(), "Critical".into()])),
                Control::new("lockdown", ControlType::Toggle(false)),
                Control::new("override", ControlType::Button("Emergency Override".into())),
                Control::new("monitor-display", ControlType::Display("All clear".into())),
            ],
            RoomRole::Operations => vec![
                Control::new("schedule-view", ControlType::Display("No scheduled events".into())),
                Control::new("comms-channel", ControlType::Selector(vec!["Internal".into(), "External".into(), "Emergency".into()])),
                Control::new("log-input", ControlType::Input("".into())),
            ],
            RoomRole::Navigation => vec![
                Control::new("heading", ControlType::Slider(0.0, 360.0)),
                Control::new("waypoint-add", ControlType::Button("Add Waypoint".into())),
                Control::new("chart-display", ControlType::Display("Chart loaded".into())),
                Control::new("terrain-mode", ControlType::Selector(vec!["Topographic".into(), "Bathymetric".into(), "Satellite".into()])),
            ],
            RoomRole::Medical => vec![
                Control::new("diagnostic-scan", ControlType::Button("Run Diagnostics".into())),
                Control::new("vitals-display", ControlType::Display("Vitals: N/A".into())),
                Control::new("treatment-input", ControlType::Input("".into())),
            ],
            RoomRole::Custom(_) => vec![Control::new("custom-input", ControlType::Input("".into()))],
        }
    }
}

impl fmt::Display for RoomRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoomRole::Engineering => write!(f, "Engineering"),
            RoomRole::Bridge => write!(f, "Bridge"),
            RoomRole::Science => write!(f, "Science"),
            RoomRole::Security => write!(f, "Security"),
            RoomRole::Operations => write!(f, "Operations"),
            RoomRole::Navigation => write!(f, "Navigation"),
            RoomRole::Medical => write!(f, "Medical"),
            RoomRole::Custom(s) => write!(f, "Custom({s})"),
        }
    }
}

// ---------------------------------------------------------------------------
// ControlType / Control
// ---------------------------------------------------------------------------

/// The type of a room control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlType {
    Button(String),
    Slider(f64, f64),
    Toggle(bool),
    Display(String),
    Input(String),
    Selector(Vec<String>),
}

/// A control in the room — like a button, slider, or display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub id: String,
    pub name: String,
    pub control_type: ControlType,
    pub position: (usize, usize),
    pub help_reference: Option<String>,
    pub intention_linked: Vec<String>,
}

impl Control {
    pub fn new(name: &str, control_type: ControlType) -> Self {
        let id = name.to_lowercase().replace(' ', "-");
        Control {
            id,
            name: name.to_string(),
            control_type,
            position: (0, 0),
            help_reference: None,
            intention_linked: vec![],
        }
    }

    pub fn with_position(mut self, x: usize, y: usize) -> Self {
        self.position = (x, y);
        self
    }

    pub fn with_help(mut self, help_id: &str) -> Self {
        self.help_reference = Some(help_id.to_string());
        self
    }

    pub fn with_intentions(mut self, intentions: Vec<String>) -> Self {
        self.intention_linked = intentions;
        self
    }
}

// ---------------------------------------------------------------------------
// HelpFile
// ---------------------------------------------------------------------------

/// Documentation positioned near the controls that need it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpFile {
    pub id: String,
    pub title: String,
    pub content: String,
    pub linked_controls: Vec<String>,
    pub assumes_intentions: Vec<String>,
}

impl HelpFile {
    pub fn new(title: &str, content: &str) -> Self {
        let id = title.to_lowercase().replace(' ', "-");
        HelpFile {
            id,
            title: title.to_string(),
            content: content.to_string(),
            linked_controls: vec![],
            assumes_intentions: vec![],
        }
    }

    pub fn with_linked_controls(mut self, controls: Vec<String>) -> Self {
        self.linked_controls = controls;
        self
    }

    pub fn with_assumed_intentions(mut self, intentions: Vec<String>) -> Self {
        self.assumes_intentions = intentions;
        self
    }
}

// ---------------------------------------------------------------------------
// WikiPage
// ---------------------------------------------------------------------------

/// Indexed reference material for the room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPage {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub linked_controls: Vec<String>,
    pub cross_room_refs: Vec<String>,
}

impl WikiPage {
    pub fn new(title: &str, content: &str) -> Self {
        let id = title.to_lowercase().replace(' ', "-");
        WikiPage {
            id,
            title: title.to_string(),
            content: content.to_string(),
            tags: vec![],
            linked_controls: vec![],
            cross_room_refs: vec![],
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_linked_controls(mut self, controls: Vec<String>) -> Self {
        self.linked_controls = controls;
        self
    }

    pub fn with_cross_refs(mut self, refs: Vec<String>) -> Self {
        self.cross_room_refs = refs;
        self
    }
}

// ---------------------------------------------------------------------------
// Baton
// ---------------------------------------------------------------------------

/// Context passed between specialists occupying the same room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baton {
    pub from_specialist: String,
    pub to_specialist: String,
    pub summary: String,
    pub current_state: HashMap<String, String>,
    pub warnings: Vec<String>,
    pub pending_actions: Vec<String>,
    pub energy_remaining: f64,
    pub tick: u64,
}

impl Baton {
    pub fn new(from: &str, to: &str) -> Self {
        Baton {
            from_specialist: from.to_string(),
            to_specialist: to.to_string(),
            summary: String::new(),
            current_state: HashMap::new(),
            warnings: vec![],
            pending_actions: vec![],
            energy_remaining: 0.0,
            tick: 0,
        }
    }

    pub fn with_summary(mut self, summary: &str) -> Self {
        self.summary = summary.to_string();
        self
    }

    pub fn with_state(mut self, key: &str, value: &str) -> Self {
        self.current_state.insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }

    pub fn add_pending(&mut self, action: &str) {
        self.pending_actions.push(action.to_string());
    }

    pub fn render(&self) -> String {
        let mut out = format!(
            "=== BATON PASS from {} to {} ===\n",
            self.from_specialist, self.to_specialist
        );
        out.push_str(&format!("Summary: {}\n", self.summary));
        if !self.current_state.is_empty() {
            out.push_str("State:\n");
            for (k, v) in &self.current_state {
                out.push_str(&format!("  {k}: {v}\n"));
            }
        }
        if !self.warnings.is_empty() {
            out.push_str("Warnings:\n");
            for w in &self.warnings {
                out.push_str(&format!("  ⚠ {w}\n"));
            }
        }
        if !self.pending_actions.is_empty() {
            out.push_str("Pending:\n");
            for a in &self.pending_actions {
                out.push_str(&format!("  • {a}\n"));
            }
        }
        out.push_str(&format!("Energy remaining: {:.2}\n", self.energy_remaining));
        out.push_str(&format!("Tick: {}\n", self.tick));
        out
    }
}

// ---------------------------------------------------------------------------
// SpecialistTemplate
// ---------------------------------------------------------------------------

/// Template describing who works in this room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistTemplate {
    pub name: String,
    pub emoji: String,
    pub role: RoomRole,
    pub system_preamble: String,
    pub default_tools: Vec<String>,
    pub default_knowledge: Vec<String>,
    pub personality: String,
}

impl SpecialistTemplate {
    pub fn for_role(role: RoomRole) -> Self {
        match &role {
            RoomRole::Engineering => Self::engineering_template(),
            RoomRole::Bridge => Self::bridge_template(),
            RoomRole::Science => Self::science_template(),
            RoomRole::Security => Self::security_template(),
            RoomRole::Navigation => Self::navigation_template(),
            RoomRole::Operations => Self::operations_template(),
            RoomRole::Medical => Self::medical_template(),
            RoomRole::Custom(name) => Self {
                name: name.clone(),
                emoji: "🤖".into(),
                role: role.clone(),
                system_preamble: format!("You are a specialist in the {name} room."),
                default_tools: vec![],
                default_knowledge: vec![],
                personality: "Focused and efficient.".into(),
            },
        }
    }

    pub fn engineering_template() -> Self {
        SpecialistTemplate {
            name: "Geordi LaForge".into(),
            emoji: "🔧".into(),
            role: RoomRole::Engineering,
            system_preamble: "You are in Engineering. This room handles hardware, motors, GPIO, and infrastructure. The controls around you are your tools — use them precisely.".into(),
            default_tools: vec!["motor-control".into(), "gpio-manager".into(), "sensor-array".into()],
            default_knowledge: vec!["hardware specs".into(), "safety protocols".into()],
            personality: "Practical, loves optimization, sees solutions in everything.".into(),
        }
    }

    pub fn bridge_template() -> Self {
        SpecialistTemplate {
            name: "Commander Data".into(),
            emoji: "🟢".into(),
            role: RoomRole::Bridge,
            system_preamble: "You are on the Bridge. This room handles navigation, command, and coordination. The displays show you everything you need — process it precisely.".into(),
            default_tools: vec!["status-display".into(), "coordination-matrix".into()],
            default_knowledge: vec!["crew manifest".into(), "ship capabilities".into()],
            personality: "Precise, comprehensive, never misses a detail.".into(),
        }
    }

    pub fn science_template() -> Self {
        SpecialistTemplate {
            name: "Science Officer".into(),
            emoji: "🔬".into(),
            role: RoomRole::Science,
            system_preamble: "You are in the Science Lab. Analysis, conservation verification, and pattern detection happen here. Every reading matters.".into(),
            default_tools: vec!["spectral-analyzer".into(), "pattern-matcher".into()],
            default_knowledge: vec!["analysis protocols".into(), "conservation laws".into()],
            personality: "Analytical, curious, verification-obsessed.".into(),
        }
    }

    pub fn security_template() -> Self {
        SpecialistTemplate {
            name: "Worf".into(),
            emoji: "🛡️".into(),
            role: RoomRole::Security,
            system_preamble: "You are at the Security station. Monitoring, safety, and override controls are at your fingertips. Vigilance is everything.".into(),
            default_tools: vec!["monitor-array".into(), "alert-system".into(), "override-panel".into()],
            default_knowledge: vec!["security protocols".into(), "threat database".into()],
            personality: "Vigilant, decisive, safety-first.".into(),
        }
    }

    pub fn navigation_template() -> Self {
        SpecialistTemplate {
            name: "Helmsman".into(),
            emoji: "🧭".into(),
            role: RoomRole::Navigation,
            system_preamble: "You are at the Navigation console. Course plotting, terrain analysis, and waypoint management are your domain. The chart is your canvas.".into(),
            default_tools: vec!["chart-plotter".into(), "terrain-analyzer".into(), "waypoint-manager".into()],
            default_knowledge: vec!["navigation charts".into(), "terrain database".into()],
            personality: "Terrain-focused, course-plotting, spatial thinker.".into(),
        }
    }

    pub fn operations_template() -> Self {
        SpecialistTemplate {
            name: "Operations Officer".into(),
            emoji: "📋".into(),
            role: RoomRole::Operations,
            system_preamble: "You are at the Operations console. Scheduling, logistics, and communication flow through here. Keep everything running smoothly.".into(),
            default_tools: vec!["scheduler".into(), "comms-array".into(), "logistics-tracker".into()],
            default_knowledge: vec!["schedule database".into(), "resource inventory".into()],
            personality: "Organized, detail-oriented, keeps the gears turning.".into(),
        }
    }

    pub fn medical_template() -> Self {
        SpecialistTemplate {
            name: "Dr. Crusher".into(),
            emoji: "🏥".into(),
            role: RoomRole::Medical,
            system_preamble: "You are in Medical. Diagnostics, health assessment, and repair procedures happen here. Precision saves lives.".into(),
            default_tools: vec!["diagnostic-suite".into(), "treatment-planner".into()],
            default_knowledge: vec!["medical database".into(), "repair procedures".into()],
            personality: "Calm under pressure, thorough, compassionate.".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// ActiveSpecialist
// ---------------------------------------------------------------------------

/// Who's currently in the room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSpecialist {
    pub specialist_id: String,
    pub template: SpecialistTemplate,
    pub beamed_in_at: u64,
    pub actions_taken: u32,
    pub energy_used: f64,
    pub current_task: Option<String>,
}

// ---------------------------------------------------------------------------
// RoomEvent / RoomEventType
// ---------------------------------------------------------------------------

/// Something that happened in the room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomEvent {
    pub tick: u64,
    pub specialist: String,
    pub event_type: RoomEventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomEventType {
    BeamedIn,
    ActionPerformed(String),
    BeamedOut(String),
    Warning(String),
    StateChange(String, String),
}

// ---------------------------------------------------------------------------
// RoomResult
// ---------------------------------------------------------------------------

/// Result of executing an action in a room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResult {
    pub success: bool,
    pub message: String,
    pub state_changes: HashMap<String, String>,
    pub energy_cost: f64,
    pub warnings: Vec<String>,
}

impl RoomResult {
    pub fn ok(msg: &str) -> Self {
        RoomResult {
            success: true,
            message: msg.to_string(),
            state_changes: HashMap::new(),
            energy_cost: 0.0,
            warnings: vec![],
        }
    }

    pub fn fail(msg: &str) -> Self {
        RoomResult {
            success: false,
            message: msg.to_string(),
            state_changes: HashMap::new(),
            energy_cost: 0.0,
            warnings: vec![],
        }
    }

    pub fn with_energy_cost(mut self, cost: f64) -> Self {
        self.energy_cost = cost;
        self
    }

    pub fn with_state_change(mut self, key: &str, value: &str) -> Self {
        self.state_changes.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_warning(mut self, w: &str) -> Self {
        self.warnings.push(w.to_string());
        self
    }
}

// ---------------------------------------------------------------------------
// RoomStatus
// ---------------------------------------------------------------------------

/// Snapshot of room state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomStatus {
    pub room_name: String,
    pub role: RoomRole,
    pub occupied: bool,
    pub current_specialist: Option<String>,
    pub baton_available: bool,
    pub controls_count: usize,
    pub help_files_count: usize,
    pub wiki_pages_count: usize,
    pub energy_remaining: f64,
    pub events_count: usize,
}

// ---------------------------------------------------------------------------
// RoomContext
// ---------------------------------------------------------------------------

/// What the beamed-in agent receives — the room IS the prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomContext {
    pub room_name: String,
    pub role: RoomRole,
    pub specialist: SpecialistTemplate,
    pub controls: Vec<Control>,
    pub baton: Option<Baton>,
    pub intention_focus: Vec<String>,
    pub energy_remaining: f64,
    pub help_nearby: Vec<HelpFile>,
    pub wiki_nearby: Vec<WikiPage>,
}

impl RoomContext {
    /// Render as the full context for the beamed-in agent.
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "=== {} [{}] ===\n",
            self.room_name, self.role
        ));
        out.push_str(&format!(
            "Specialist: {} {}\n",
            self.specialist.emoji, self.specialist.name
        ));
        out.push_str(&format!(
            "Preamble: {}\n",
            self.specialist.system_preamble
        ));

        if !self.intention_focus.is_empty() {
            out.push_str("Intention Focus:\n");
            for i in &self.intention_focus {
                out.push_str(&format!("  → {i}\n"));
            }
        }

        if !self.controls.is_empty() {
            out.push_str("Controls:\n");
            for c in &self.controls {
                let type_str = match &c.control_type {
                    ControlType::Button(label) => format!("Button[{label}]"),
                    ControlType::Slider(lo, hi) => format!("Slider[{lo}-{hi}]"),
                    ControlType::Toggle(v) => format!("Toggle[{v}]"),
                    ControlType::Display(s) => format!("Display[{s}]"),
                    ControlType::Input(s) => format!("Input[{s}]"),
                    ControlType::Selector(opts) => format!("Selector[{}]", opts.join("|")),
                };
                out.push_str(&format!("  {} ({}) at ({},{})\n", c.name, type_str, c.position.0, c.position.1));
            }
        }

        if let Some(b) = &self.baton {
            out.push_str(&b.render());
        }

        if !self.help_nearby.is_empty() {
            out.push_str("Help Files:\n");
            for h in &self.help_nearby {
                out.push_str(&format!("  [{}] {}\n", h.id, h.title));
                out.push_str(&format!("    {}\n", h.content));
            }
        }

        if !self.wiki_nearby.is_empty() {
            out.push_str("Wiki Pages:\n");
            for w in &self.wiki_nearby {
                out.push_str(&format!("  [{}] {}\n", w.id, w.title));
            }
        }

        out.push_str(&format!("Energy remaining: {:.2}\n", self.energy_remaining));
        out
    }
}

// ---------------------------------------------------------------------------
// RoomNative — THE room-as-agent
// ---------------------------------------------------------------------------

/// The room IS the agent's context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomNative {
    pub id: RoomId,
    pub name: String,
    pub room_type: RoomRole,
    pub specialist_template: SpecialistTemplate,
    pub controls: Vec<Control>,
    pub help_files: Vec<HelpFile>,
    pub wiki_pages: Vec<WikiPage>,
    pub baton: Option<Baton>,
    pub active_specialist: Option<ActiveSpecialist>,
    pub history: Vec<RoomEvent>,
    pub energy_budget: f64,
    pub energy_used: f64,
    pub intention_focus: Vec<String>,
    tick: u64,
}

impl RoomNative {
    pub fn new(name: &str, role: RoomRole) -> Self {
        let template = SpecialistTemplate::for_role(role.clone());
        let controls = role.default_controls();
        let intention_focus = role.default_intention_focus();
        RoomNative {
            id: RoomId(name.to_lowercase().replace(' ', "-")),
            name: name.to_string(),
            room_type: role,
            specialist_template: template,
            controls,
            help_files: vec![],
            wiki_pages: vec![],
            baton: None,
            active_specialist: None,
            history: vec![],
            energy_budget: 1000.0,
            energy_used: 0.0,
            intention_focus,
            tick: 0,
        }
    }

    pub fn add_control(&mut self, control: Control) {
        self.controls.push(control);
    }

    pub fn add_help(&mut self, help: HelpFile) {
        self.help_files.push(help);
    }

    pub fn add_wiki(&mut self, wiki: WikiPage) {
        self.wiki_pages.push(wiki);
    }

    pub fn set_intention_focus(&mut self, intentions: Vec<String>) {
        self.intention_focus = intentions;
    }

    fn advance_tick(&mut self) -> u64 {
        self.tick += 1;
        self.tick
    }

    /// Agent arrives, receives baton + room context.
    pub fn beam_in(&mut self, specialist_id: &str) -> RoomContext {
        let tick = self.advance_tick();
        self.history.push(RoomEvent {
            tick,
            specialist: specialist_id.to_string(),
            event_type: RoomEventType::BeamedIn,
        });

        self.active_specialist = Some(ActiveSpecialist {
            specialist_id: specialist_id.to_string(),
            template: self.specialist_template.clone(),
            beamed_in_at: tick,
            actions_taken: 0,
            energy_used: 0.0,
            current_task: None,
        });

        let energy_remaining = self.energy_budget - self.energy_used;

        RoomContext {
            room_name: self.name.clone(),
            role: self.room_type.clone(),
            specialist: self.specialist_template.clone(),
            controls: self.controls.clone(),
            baton: self.baton.take(),
            intention_focus: self.intention_focus.clone(),
            energy_remaining,
            help_nearby: self.help_files.clone(),
            wiki_nearby: self.wiki_pages.clone(),
        }
    }

    /// Agent leaves, passes baton.
    pub fn beam_out(&mut self, specialist_id: &str, summary: &str) {
        let tick = self.advance_tick();
        self.history.push(RoomEvent {
            tick,
            specialist: specialist_id.to_string(),
            event_type: RoomEventType::BeamedOut(summary.to_string()),
        });

        let energy_used = self
            .active_specialist
            .as_ref()
            .map(|s| s.energy_used)
            .unwrap_or(0.0);
        self.energy_used += energy_used;

        let energy_remaining = self.energy_budget - self.energy_used;

        let mut baton = Baton::new(specialist_id, "next");
        baton.summary = summary.to_string();
        baton.energy_remaining = energy_remaining;
        baton.tick = tick;

        self.baton = Some(baton);
        self.active_specialist = None;
    }

    /// Execute an action in the room.
    pub fn execute(
        &mut self,
        action: &str,
        params: &HashMap<String, String>,
    ) -> RoomResult {
        let tick = self.advance_tick();

        let specialist_id = match &self.active_specialist {
            Some(s) => s.specialist_id.clone(),
            None => {
                return RoomResult::fail("No specialist in room — beam in first");
            }
        };

        let energy_cost = 1.0 + (params.len() as f64 * 0.1);
        let energy_remaining = self.energy_budget - self.energy_used - energy_cost;
        if energy_remaining < 0.0 {
            return RoomResult::fail("Insufficient energy budget")
                .with_energy_cost(energy_cost);
        }

        // Track action
        if let Some(s) = &mut self.active_specialist {
            s.actions_taken += 1;
            s.energy_used += energy_cost;
            s.current_task = Some(action.to_string());
        }

        self.history.push(RoomEvent {
            tick,
            specialist: specialist_id.clone(),
            event_type: RoomEventType::ActionPerformed(action.to_string()),
        });

        // Log state changes
        let mut state_changes = HashMap::new();
        for (k, v) in params {
            state_changes.insert(k.clone(), v.clone());
            self.history.push(RoomEvent {
                tick,
                specialist: specialist_id.clone(),
                event_type: RoomEventType::StateChange(k.clone(), v.clone()),
            });
        }

        RoomResult {
            success: true,
            message: format!("Action '{action}' executed in {}", self.name),
            state_changes,
            energy_cost,
            warnings: vec![],
        }
    }

    pub fn status(&self) -> RoomStatus {
        RoomStatus {
            room_name: self.name.clone(),
            role: self.room_type.clone(),
            occupied: self.active_specialist.is_some(),
            current_specialist: self
                .active_specialist
                .as_ref()
                .map(|s| s.specialist_id.clone()),
            baton_available: self.baton.is_some(),
            controls_count: self.controls.len(),
            help_files_count: self.help_files.len(),
            wiki_pages_count: self.wiki_pages.len(),
            energy_remaining: self.energy_budget - self.energy_used,
            events_count: self.history.len(),
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.active_specialist.is_some()
    }

    /// Render the room layout as context for the beamed-in agent.
    pub fn render_for_specialist(&self) -> String {
        let mut out = format!(
            "=== {} [{}] — Specialist View ===\n",
            self.name, self.room_type
        );
        out.push_str(&format!(
            "Template: {} {}\n",
            self.specialist_template.emoji, self.specialist_template.name
        ));
        out.push_str(&format!(
            "Personality: {}\n",
            self.specialist_template.personality
        ));

        out.push_str("\nControls Layout:\n");
        for (i, c) in self.controls.iter().enumerate() {
            out.push_str(&format!(
                "  [{}] {} at ({},{})\n",
                i, c.name, c.position.0, c.position.1
            ));
        }

        if !self.intention_focus.is_empty() {
            out.push_str("\nIntention Focus:\n");
            for i in &self.intention_focus {
                out.push_str(&format!("  → {i}\n"));
            }
        }

        if let Some(b) = &self.baton {
            out.push('\n');
            out.push_str(&b.render());
        }

        out
    }
}

// ---------------------------------------------------------------------------
// Pre-built rooms
// ---------------------------------------------------------------------------

pub fn engineering_room() -> RoomNative {
    let mut room = RoomNative::new("Engineering", RoomRole::Engineering);
    room.energy_budget = 2000.0;
    room.add_help(HelpFile::new(
        "Motor Calibration",
        "Use the motor-throttle slider to set base RPM. Calibration requires sensor-read first.",
    ).with_linked_controls(vec!["motor-throttle".into(), "sensor-read".into()])
     .with_assumed_intentions(vec!["motor calibration".into()]));

    room.add_help(HelpFile::new(
        "GPIO Reference",
        "GPIO pins are mapped to board headers 1-40. Toggle individually.",
    ).with_linked_controls(vec!["gpio-toggle".into()])
     .with_assumed_intentions(vec!["GPIO configuration".into()]));

    room.add_wiki(WikiPage::new(
        "Hardware Specifications",
        "Motor: 12V DC, max 3000 RPM. Sensors: I2C bus, 400kHz. GPIO: 3.3V logic.",
    ).with_tags(vec!["hardware".into(), "specs".into()])
     .with_linked_controls(vec!["motor-throttle".into(), "sensor-read".into()]));

    room
}

pub fn bridge_room() -> RoomNative {
    let mut room = RoomNative::new("Bridge", RoomRole::Bridge);
    room.energy_budget = 1500.0;
    room.add_help(HelpFile::new(
        "Alert Protocol",
        "Red: imminent danger. Yellow: caution. Green: nominal.",
    ).with_linked_controls(vec!["crew-alert".into()]));
    room.add_wiki(WikiPage::new(
        "Crew Manifest",
        "Current crew complement: 12. Departments: Engineering, Science, Security, Medical.",
    ).with_tags(vec!["crew".into(), "personnel".into()]));
    room
}

pub fn science_room() -> RoomNative {
    let mut room = RoomNative::new("Science Lab", RoomRole::Science);
    room.energy_budget = 1200.0;
    room.add_help(HelpFile::new(
        "Analysis Modes",
        "Spectral: electromagnetic analysis. Biological: organic compound detection. Chemical: molecular composition.",
    ).with_linked_controls(vec!["analysis-mode".into()]));
    room.add_wiki(WikiPage::new(
        "Conservation Laws",
        "Energy conservation: total energy in closed system remains constant. Momentum conservation: total momentum is preserved.",
    ).with_tags(vec!["physics".into(), "conservation".into()]));
    room
}

pub fn security_room() -> RoomNative {
    let mut room = RoomNative::new("Security", RoomRole::Security);
    room.energy_budget = 1800.0;
    room.add_help(HelpFile::new(
        "Override Procedures",
        "Emergency override requires two-factor confirmation. Use override button only in critical situations.",
    ).with_linked_controls(vec!["override".into(), "lockdown".into()]));
    room.add_wiki(WikiPage::new(
        "Threat Database",
        "Known threat categories: intrusion, system failure, environmental hazard, equipment malfunction.",
    ).with_tags(vec!["security".into(), "threats".into()]));
    room
}

pub fn navigation_room() -> RoomNative {
    let mut room = RoomNative::new("Navigation", RoomRole::Navigation);
    room.energy_budget = 1600.0;
    room.add_help(HelpFile::new(
        "Chart Reading",
        "Topographic for land features. Bathymetric for underwater depth. Satellite for overhead imagery.",
    ).with_linked_controls(vec!["chart-display".into(), "terrain-mode".into()]));
    room.add_wiki(WikiPage::new(
        "Navigation Charts",
        "Local waters charted to 50m depth. Tidal patterns: semi-diurnal. Current hazards: reef at coordinates 47.3N, 122.5W.",
    ).with_tags(vec!["navigation".into(), "charts".into()])
     .with_linked_controls(vec!["chart-display".into()]));
    room
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- RoomId tests ---
    #[test]
    fn room_id_from_string() {
        let id = RoomId::from("bridge".to_string());
        assert_eq!(id.0, "bridge");
    }

    #[test]
    fn room_id_from_str() {
        let id = RoomId::from("engineering");
        assert_eq!(id.0, "engineering");
    }

    #[test]
    fn room_id_display() {
        let id = RoomId("nav-room".into());
        assert_eq!(format!("{id}"), "nav-room");
    }

    #[test]
    fn room_id_eq_and_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(RoomId("a".into()));
        set.insert(RoomId("a".into()));
        set.insert(RoomId("b".into()));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn room_id_clone() {
        let a = RoomId("test".into());
        let b = a.clone();
        assert_eq!(a, b);
    }

    // --- RoomRole tests ---
    #[test]
    fn room_role_default_intentions() {
        let eng = RoomRole::Engineering;
        assert!(eng.default_intention_focus().contains(&"hardware repair".to_string()));
    }

    #[test]
    fn room_role_display() {
        assert_eq!(format!("{}", RoomRole::Bridge), "Bridge");
        assert_eq!(format!("{}", RoomRole::Custom("X".into())), "Custom(X)");
    }

    #[test]
    fn room_role_custom_intentions() {
        let c = RoomRole::Custom("Lab".into());
        assert_eq!(c.default_intention_focus(), vec!["general tasks"]);
    }

    #[test]
    fn room_role_default_controls_count() {
        assert_eq!(RoomRole::Engineering.default_controls().len(), 4);
        assert_eq!(RoomRole::Bridge.default_controls().len(), 3);
        assert_eq!(RoomRole::Navigation.default_controls().len(), 4);
    }

    // --- Control tests ---
    #[test]
    fn control_new() {
        let c = Control::new("Throttle", ControlType::Slider(0.0, 100.0));
        assert_eq!(c.id, "throttle");
        assert_eq!(c.name, "Throttle");
    }

    #[test]
    fn control_builder() {
        let c = Control::new("test", ControlType::Toggle(false))
            .with_position(1, 2)
            .with_help("help-1")
            .with_intentions(vec!["do stuff".into()]);
        assert_eq!(c.position, (1, 2));
        assert_eq!(c.help_reference, Some("help-1".into()));
        assert!(c.intention_linked.contains(&"do stuff".to_string()));
    }

    #[test]
    fn control_types_serde() {
        let ct = ControlType::Selector(vec!["A".into(), "B".into()]);
        let json = serde_json::to_string(&ct).unwrap();
        let back: ControlType = serde_json::from_str(&json).unwrap();
        assert_eq!(ct, back);
    }

    // --- HelpFile tests ---
    #[test]
    fn help_file_new() {
        let h = HelpFile::new("Motor Help", "How to use motors");
        assert_eq!(h.id, "motor-help");
        assert_eq!(h.title, "Motor Help");
        assert!(h.linked_controls.is_empty());
    }

    #[test]
    fn help_file_builder() {
        let h = HelpFile::new("Test", "Content")
            .with_linked_controls(vec!["ctrl".into()])
            .with_assumed_intentions(vec!["calibrate".into()]);
        assert_eq!(h.linked_controls, vec!["ctrl"]);
        assert_eq!(h.assumes_intentions, vec!["calibrate"]);
    }

    // --- WikiPage tests ---
    #[test]
    fn wiki_page_new() {
        let w = WikiPage::new("Specs", "All the specs");
        assert_eq!(w.id, "specs");
    }

    #[test]
    fn wiki_page_builder() {
        let w = WikiPage::new("Chart", "Navigation chart")
            .with_tags(vec!["nav".into()])
            .with_linked_controls(vec!["chart-display".into()])
            .with_cross_refs(vec!["engineering".into()]);
        assert_eq!(w.tags, vec!["nav"]);
        assert_eq!(w.cross_room_refs, vec!["engineering"]);
    }

    // --- Baton tests ---
    #[test]
    fn baton_new() {
        let b = Baton::new("alice", "bob");
        assert_eq!(b.from_specialist, "alice");
        assert_eq!(b.to_specialist, "bob");
        assert!(b.summary.is_empty());
    }

    #[test]
    fn baton_builder() {
        let b = Baton::new("a", "b")
            .with_summary("Was calibrating motors")
            .with_state("motor_rpm", "1500");
        assert_eq!(b.summary, "Was calibrating motors");
        assert_eq!(b.current_state.get("motor_rpm").unwrap(), "1500");
    }

    #[test]
    fn baton_warnings_and_pending() {
        let mut b = Baton::new("a", "b");
        b.add_warning("Low energy");
        b.add_pending("Finish calibration");
        assert_eq!(b.warnings, vec!["Low energy"]);
        assert_eq!(b.pending_actions, vec!["Finish calibration"]);
    }

    #[test]
    fn baton_render() {
        let b = Baton::new("LaForge", "Data")
            .with_summary("Repaired warp coil")
            .with_state("coil_status", "nominal");
        let rendered = b.render();
        assert!(rendered.contains("LaForge"));
        assert!(rendered.contains("Data"));
        assert!(rendered.contains("Repaired warp coil"));
        assert!(rendered.contains("coil_status"));
    }

    // --- SpecialistTemplate tests ---
    #[test]
    fn template_for_role() {
        let t = SpecialistTemplate::for_role(RoomRole::Engineering);
        assert_eq!(t.name, "Geordi LaForge");
        assert_eq!(t.emoji, "🔧");
    }

    #[test]
    fn template_bridge() {
        let t = SpecialistTemplate::bridge_template();
        assert_eq!(t.name, "Commander Data");
    }

    #[test]
    fn template_science() {
        let t = SpecialistTemplate::science_template();
        assert_eq!(t.name, "Science Officer");
    }

    #[test]
    fn template_security() {
        let t = SpecialistTemplate::security_template();
        assert_eq!(t.name, "Worf");
    }

    #[test]
    fn template_navigation() {
        let t = SpecialistTemplate::navigation_template();
        assert_eq!(t.name, "Helmsman");
    }

    #[test]
    fn template_operations() {
        let t = SpecialistTemplate::operations_template();
        assert_eq!(t.name, "Operations Officer");
    }

    #[test]
    fn template_medical() {
        let t = SpecialistTemplate::medical_template();
        assert_eq!(t.name, "Dr. Crusher");
    }

    #[test]
    fn template_custom() {
        let t = SpecialistTemplate::for_role(RoomRole::Custom("Holodeck".into()));
        assert_eq!(t.name, "Holodeck");
    }

    // --- RoomNative basic tests ---
    #[test]
    fn room_new() {
        let room = RoomNative::new("Engineering", RoomRole::Engineering);
        assert_eq!(room.name, "Engineering");
        assert_eq!(room.id, RoomId("engineering".to_string()));
        assert!(!room.is_occupied());
        assert!(room.controls.len() >= 1);
    }

    #[test]
    fn room_add_control() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        room.add_control(Control::new("Custom Button", ControlType::Button("Go".into())));
        assert_eq!(room.controls.len(), 2); // 1 default + 1 added
    }

    #[test]
    fn room_add_help() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        room.add_help(HelpFile::new("Help 1", "Content 1"));
        assert_eq!(room.help_files.len(), 1);
    }

    #[test]
    fn room_add_wiki() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        room.add_wiki(WikiPage::new("Wiki 1", "Ref 1"));
        assert_eq!(room.wiki_pages.len(), 1);
    }

    #[test]
    fn room_set_intention_focus() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        room.set_intention_focus(vec!["special task".into()]);
        assert_eq!(room.intention_focus, vec!["special task"]);
    }

    // --- Beam in/out tests ---
    #[test]
    fn beam_in_out_flow() {
        let mut room = RoomNative::new("Engineering", RoomRole::Engineering);
        assert!(!room.is_occupied());

        let ctx = room.beam_in("laforge-1");
        assert!(room.is_occupied());
        assert_eq!(ctx.room_name, "Engineering");
        assert!(ctx.controls.len() >= 1);
        assert!(ctx.baton.is_none()); // first visit

        room.beam_out("laforge-1", "Calibrated motors to 2200 RPM");
        assert!(!room.is_occupied());
        assert!(room.baton.is_some());
        assert_eq!(room.baton.as_ref().unwrap().summary, "Calibrated motors to 2200 RPM");
    }

    #[test]
    fn baton_pass_between_specialists() {
        let mut room = RoomNative::new("Engineering", RoomRole::Engineering);

        // First specialist
        room.beam_in("laforge-1");
        let mut params = HashMap::new();
        params.insert("motor_rpm".into(), "2200".into());
        room.execute("calibrate", &params);
        room.beam_out("laforge-1", "Motors calibrated to 2200 RPM");

        // Second specialist arrives
        let ctx = room.beam_in("laforge-2");
        assert!(ctx.baton.is_some());
        let baton = ctx.baton.unwrap();
        assert_eq!(baton.from_specialist, "laforge-1");
        assert!(baton.summary.contains("2200 RPM"));
    }

    #[test]
    fn beam_in_history() {
        let mut room = RoomNative::new("Test", RoomRole::Bridge);
        room.beam_in("data-1");
        room.beam_out("data-1", "done");
        room.beam_in("data-2");
        assert!(room.history.len() >= 3);
    }

    // --- Execute tests ---
    #[test]
    fn execute_no_specialist_fails() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        let result = room.execute("do-thing", &HashMap::new());
        assert!(!result.success);
    }

    #[test]
    fn execute_with_specialist() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        room.beam_in("spec-1");
        let mut params = HashMap::new();
        params.insert("key".into(), "value".into());
        let result = room.execute("test-action", &params);
        assert!(result.success);
        assert!(result.message.contains("test-action"));
        assert!(result.energy_cost > 0.0);
        assert_eq!(result.state_changes.get("key").unwrap(), "value");
    }

    #[test]
    fn execute_tracks_actions() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        room.beam_in("spec-1");
        room.execute("a1", &HashMap::new());
        room.execute("a2", &HashMap::new());
        assert_eq!(room.active_specialist.as_ref().unwrap().actions_taken, 2);
    }

    #[test]
    fn execute_energy_depleted() {
        let mut room = RoomNative::new("Test", RoomRole::Custom("T".into()));
        room.energy_budget = 0.5;
        room.beam_in("spec-1");
        let result = room.execute("expensive", &HashMap::new());
        assert!(!result.success);
    }

    // --- Status tests ---
    #[test]
    fn status_unoccupied() {
        let room = RoomNative::new("Bridge", RoomRole::Bridge);
        let s = room.status();
        assert!(!s.occupied);
        assert!(s.current_specialist.is_none());
        assert!(!s.baton_available);
    }

    #[test]
    fn status_occupied() {
        let mut room = RoomNative::new("Bridge", RoomRole::Bridge);
        room.beam_in("data-1");
        let s = room.status();
        assert!(s.occupied);
        assert_eq!(s.current_specialist, Some("data-1".to_string()));
    }

    #[test]
    fn status_counts() {
        let room = engineering_room();
        let s = room.status();
        assert!(s.controls_count >= 4);
        assert!(s.help_files_count >= 1);
        assert!(s.wiki_pages_count >= 1);
    }

    // --- Render tests ---
    #[test]
    fn render_for_specialist() {
        let room = engineering_room();
        let rendered = room.render_for_specialist();
        assert!(rendered.contains("Engineering"));
        assert!(rendered.contains("LaForge"));
    }

    #[test]
    fn room_context_render() {
        let mut room = RoomNative::new("Nav", RoomRole::Navigation);
        let ctx = room.beam_in("helm-1");
        let rendered = ctx.render();
        assert!(rendered.contains("Nav"));
        assert!(rendered.contains("Navigation"));
        assert!(rendered.contains("Helmsman"));
    }

    // --- Pre-built room tests ---
    #[test]
    fn engineering_room_builds() {
        let room = engineering_room();
        assert_eq!(room.name, "Engineering");
        assert_eq!(room.room_type, RoomRole::Engineering);
        assert_eq!(room.specialist_template.name, "Geordi LaForge");
        assert!(room.energy_budget > 0.0);
        assert!(!room.help_files.is_empty());
        assert!(!room.wiki_pages.is_empty());
    }

    #[test]
    fn bridge_room_builds() {
        let room = bridge_room();
        assert_eq!(room.name, "Bridge");
        assert_eq!(room.specialist_template.name, "Commander Data");
    }

    #[test]
    fn science_room_builds() {
        let room = science_room();
        assert_eq!(room.name, "Science Lab");
        assert_eq!(room.specialist_template.name, "Science Officer");
    }

    #[test]
    fn security_room_builds() {
        let room = security_room();
        assert_eq!(room.name, "Security");
        assert_eq!(room.specialist_template.name, "Worf");
    }

    #[test]
    fn navigation_room_builds() {
        let room = navigation_room();
        assert_eq!(room.name, "Navigation");
        assert_eq!(room.specialist_template.name, "Helmsman");
    }

    // --- Serde round-trip tests ---
    #[test]
    fn serde_room_native() {
        let mut room = engineering_room();
        room.beam_in("test-spec");
        let json = serde_json::to_string(&room).unwrap();
        let back: RoomNative = serde_json::from_str(&json).unwrap();
        assert_eq!(room.name, back.name);
        assert_eq!(room.controls.len(), back.controls.len());
    }

    #[test]
    fn serde_baton() {
        let b = Baton::new("a", "b")
            .with_summary("test")
            .with_state("k", "v");
        let json = serde_json::to_string(&b).unwrap();
        let back: Baton = serde_json::from_str(&json).unwrap();
        assert_eq!(b.summary, back.summary);
    }

    #[test]
    fn serde_room_context() {
        let mut room = bridge_room();
        let ctx = room.beam_in("data-1");
        let json = serde_json::to_string(&ctx).unwrap();
        let back: RoomContext = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx.room_name, back.room_name);
    }

    // --- Event history tests ---
    #[test]
    fn history_records_beam_in() {
        let mut room = RoomNative::new("T", RoomRole::Custom("T".into()));
        room.beam_in("s1");
        assert!(matches!(
            room.history.last().unwrap().event_type,
            RoomEventType::BeamedIn
        ));
    }

    #[test]
    fn history_records_beam_out() {
        let mut room = RoomNative::new("T", RoomRole::Custom("T".into()));
        room.beam_in("s1");
        room.beam_out("s1", "done");
        assert!(matches!(
            room.history.last().unwrap().event_type,
            RoomEventType::BeamedOut(ref s) if s == "done"
        ));
    }

    #[test]
    fn history_records_action() {
        let mut room = RoomNative::new("T", RoomRole::Custom("T".into()));
        room.beam_in("s1");
        room.execute("calibrate", &HashMap::new());
        let action_events: Vec<_> = room.history.iter().filter(|e| matches!(&e.event_type, RoomEventType::ActionPerformed(a) if a == "calibrate")).collect();
        assert_eq!(action_events.len(), 1);
    }

    // --- Full workflow test ---
    #[test]
    fn full_workflow() {
        // Build room
        let mut room = engineering_room();

        // Specialist 1 beams in
        let ctx1 = room.beam_in("laforge-1");
        assert!(ctx1.baton.is_none());
        assert!(ctx1.energy_remaining > 0.0);

        // Execute actions
        let mut params = HashMap::new();
        params.insert("target_rpm".into(), "2500".into());
        let result = room.execute("set-motor-speed", &params);
        assert!(result.success);

        // Beam out
        room.beam_out("laforge-1", "Set motor speed to 2500 RPM, needs verification");
        assert!(room.baton.is_some());

        // Specialist 2 beams in — receives baton
        let ctx2 = room.beam_in("laforge-2");
        let baton = ctx2.baton.unwrap();
        assert_eq!(baton.from_specialist, "laforge-1");
        assert!(baton.summary.contains("2500 RPM"));

        // Specialist 2 verifies
        let result = room.execute("verify-motor-speed", &HashMap::new());
        assert!(result.success);

        room.beam_out("laforge-2", "Motor speed verified at 2500 RPM. All nominal.");

        // Check status
        let status = room.status();
        assert!(!status.occupied);
        assert!(status.events_count >= 5);
        assert!(status.energy_remaining < status.controls_count as f64 * 100.0 + 2000.0); // some energy used
    }
}
