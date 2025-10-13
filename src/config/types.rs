//! Configuration type definitions.

use super::enums::{ColorSpec, StatusPosition};
use serde::{Deserialize, Serialize};

/// Drawing-related settings.
///
/// Controls the default appearance of drawing tools when the overlay first opens.
/// Users can change these values at runtime using keybindings.
#[derive(Debug, Serialize, Deserialize)]
pub struct DrawingConfig {
    /// Default pen color - either a named color (red, green, blue, yellow, orange, pink, white, black)
    /// or an RGB array like `[255, 0, 0]` for red
    #[serde(default = "default_color")]
    pub default_color: ColorSpec,

    /// Default pen thickness in pixels (valid range: 1.0 - 20.0)
    #[serde(default = "default_thickness")]
    pub default_thickness: f64,

    /// Default font size for text mode in points (valid range: 8.0 - 72.0)
    #[serde(default = "default_font_size")]
    pub default_font_size: f64,

    /// Font family name for text rendering (e.g., "Sans", "Monospace", "JetBrains Mono")
    /// Falls back to "Sans" if the specified font is not available
    /// Note: Install fonts system-wide and reference by family name
    #[serde(default = "default_font_family")]
    pub font_family: String,

    /// Font weight (e.g., "normal", "bold", "light", 400, 700)
    /// Can be a named weight or a numeric value (100-900)
    #[serde(default = "default_font_weight")]
    pub font_weight: String,

    /// Font style (e.g., "normal", "italic", "oblique")
    #[serde(default = "default_font_style")]
    pub font_style: String,

    /// Enable semi-transparent background box behind text for better contrast
    #[serde(default = "default_text_background")]
    pub text_background_enabled: bool,
}

impl Default for DrawingConfig {
    fn default() -> Self {
        Self {
            default_color: default_color(),
            default_thickness: default_thickness(),
            default_font_size: default_font_size(),
            font_family: default_font_family(),
            font_weight: default_font_weight(),
            font_style: default_font_style(),
            text_background_enabled: default_text_background(),
        }
    }
}

/// Arrow drawing settings.
///
/// Controls the appearance of arrowheads when using the arrow tool (Ctrl+Shift+Drag).
#[derive(Debug, Serialize, Deserialize)]
pub struct ArrowConfig {
    /// Arrowhead length in pixels (valid range: 5.0 - 50.0)
    #[serde(default = "default_arrow_length")]
    pub length: f64,

    /// Arrowhead angle in degrees (valid range: 15.0 - 60.0)
    /// Smaller angles create narrower arrowheads, larger angles create wider ones
    #[serde(default = "default_arrow_angle")]
    pub angle_degrees: f64,
}

impl Default for ArrowConfig {
    fn default() -> Self {
        Self {
            length: default_arrow_length(),
            angle_degrees: default_arrow_angle(),
        }
    }
}

/// Performance tuning options.
///
/// These settings control rendering performance and smoothness. Most users
/// won't need to change these from their defaults.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Number of buffers for buffering (valid range: 2 - 4)
    /// - 2 = double buffering (lower memory, potential tearing)
    /// - 3 = triple buffering (balanced, recommended)
    /// - 4 = quad buffering (highest memory, smoothest)
    #[serde(default = "default_buffer_count")]
    pub buffer_count: u32,

    /// Enable vsync frame synchronization to prevent tearing
    /// Set to false for lower latency at the cost of potential screen tearing
    #[serde(default = "default_enable_vsync")]
    pub enable_vsync: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            buffer_count: default_buffer_count(),
            enable_vsync: default_enable_vsync(),
        }
    }
}

/// UI display preferences.
///
/// Controls the visibility and positioning of on-screen UI elements.
#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    /// Show the status bar displaying current color, thickness, and tool
    #[serde(default = "default_show_status")]
    pub show_status_bar: bool,

    /// Status bar screen position (top-left, top-right, bottom-left, bottom-right)
    #[serde(default = "default_status_position")]
    pub status_bar_position: StatusPosition,

    /// Status bar styling options
    #[serde(default)]
    pub status_bar_style: StatusBarStyle,

    /// Help overlay styling options
    #[serde(default)]
    pub help_overlay_style: HelpOverlayStyle,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_status_bar: default_show_status(),
            status_bar_position: default_status_position(),
            status_bar_style: StatusBarStyle::default(),
            help_overlay_style: HelpOverlayStyle::default(),
        }
    }
}

/// Status bar styling configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusBarStyle {
    /// Font size for status bar text
    #[serde(default = "default_status_font_size")]
    pub font_size: f64,

    /// Padding around status bar text
    #[serde(default = "default_status_padding")]
    pub padding: f64,

    /// Background color [R, G, B, A] (0.0-1.0 range)
    #[serde(default = "default_status_bg_color")]
    pub bg_color: [f64; 4],

    /// Text color [R, G, B, A] (0.0-1.0 range)
    #[serde(default = "default_status_text_color")]
    pub text_color: [f64; 4],

    /// Color indicator dot radius
    #[serde(default = "default_status_dot_radius")]
    pub dot_radius: f64,
}

impl Default for StatusBarStyle {
    fn default() -> Self {
        Self {
            font_size: default_status_font_size(),
            padding: default_status_padding(),
            bg_color: default_status_bg_color(),
            text_color: default_status_text_color(),
            dot_radius: default_status_dot_radius(),
        }
    }
}

/// Help overlay styling configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct HelpOverlayStyle {
    /// Font size for help overlay text
    #[serde(default = "default_help_font_size")]
    pub font_size: f64,

    /// Line height for help text
    #[serde(default = "default_help_line_height")]
    pub line_height: f64,

    /// Padding around help box
    #[serde(default = "default_help_padding")]
    pub padding: f64,

    /// Background color [R, G, B, A] (0.0-1.0 range)
    #[serde(default = "default_help_bg_color")]
    pub bg_color: [f64; 4],

    /// Border color [R, G, B, A] (0.0-1.0 range)
    #[serde(default = "default_help_border_color")]
    pub border_color: [f64; 4],

    /// Border line width
    #[serde(default = "default_help_border_width")]
    pub border_width: f64,

    /// Text color [R, G, B, A] (0.0-1.0 range)
    #[serde(default = "default_help_text_color")]
    pub text_color: [f64; 4],
}

impl Default for HelpOverlayStyle {
    fn default() -> Self {
        Self {
            font_size: default_help_font_size(),
            line_height: default_help_line_height(),
            padding: default_help_padding(),
            bg_color: default_help_bg_color(),
            border_color: default_help_border_color(),
            border_width: default_help_border_width(),
            text_color: default_help_text_color(),
        }
    }
}

// =============================================================================
// Default value functions
// =============================================================================

fn default_color() -> ColorSpec {
    ColorSpec::Name("red".to_string())
}

fn default_thickness() -> f64 {
    3.0
}

fn default_font_size() -> f64 {
    32.0
}

fn default_font_family() -> String {
    "Sans".to_string()
}

fn default_font_weight() -> String {
    "bold".to_string()
}

fn default_font_style() -> String {
    "normal".to_string()
}

fn default_text_background() -> bool {
    false
}

fn default_arrow_length() -> f64 {
    20.0
}

fn default_arrow_angle() -> f64 {
    30.0
}

fn default_buffer_count() -> u32 {
    3
}

fn default_enable_vsync() -> bool {
    true
}

fn default_show_status() -> bool {
    true
}

fn default_status_position() -> StatusPosition {
    StatusPosition::BottomLeft
}

// Status bar style defaults
fn default_status_font_size() -> f64 {
    21.0 // 50% larger than previous 14.0
}

fn default_status_padding() -> f64 {
    15.0 // 50% larger than previous 10.0
}

fn default_status_bg_color() -> [f64; 4] {
    [0.0, 0.0, 0.0, 0.85] // More opaque (was 0.7) for better visibility
}

fn default_status_text_color() -> [f64; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn default_status_dot_radius() -> f64 {
    6.0 // 50% larger than previous 4.0
}

// Help overlay style defaults
fn default_help_font_size() -> f64 {
    16.0
}

fn default_help_line_height() -> f64 {
    22.0
}

fn default_help_padding() -> f64 {
    20.0
}

fn default_help_bg_color() -> [f64; 4] {
    [0.0, 0.0, 0.0, 0.85]
}

fn default_help_border_color() -> [f64; 4] {
    [0.3, 0.6, 1.0, 0.9]
}

fn default_help_border_width() -> f64 {
    2.0
}

fn default_help_text_color() -> [f64; 4] {
    [1.0, 1.0, 1.0, 1.0]
}
