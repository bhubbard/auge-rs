use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Plain,
    Md,
    Json,
    Ndjson,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Plain
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisMode {
    Ocr,
    Classify,
    Barcode,
    Faces,
    FaceLandmarks,
    FaceQuality,
    Humans,
    TextRectangles,
    Rectangles,
    Horizon,
    Animals,
    AnimalPose,
    BodyPose,
    HandPose,
    SaliencyAttention,
    SaliencyObjectness,
    Contours,
    FeaturePrint,
    Compare,
    Aesthetics,
    Smudge,
    Document,
    Subject,
    PersonsMask,
    All,
    Model,
    Motion,
    Align,
    Track,
    Trajectories,
    Video,
}

impl AnalysisMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalysisMode::Ocr => "ocr",
            AnalysisMode::Classify => "classify",
            AnalysisMode::Barcode => "barcode",
            AnalysisMode::Faces => "faces",
            AnalysisMode::FaceLandmarks => "face-landmarks",
            AnalysisMode::FaceQuality => "face-quality",
            AnalysisMode::Humans => "humans",
            AnalysisMode::TextRectangles => "text-rectangles",
            AnalysisMode::Rectangles => "rectangles",
            AnalysisMode::Horizon => "horizon",
            AnalysisMode::Animals => "animals",
            AnalysisMode::AnimalPose => "animal-pose",
            AnalysisMode::BodyPose => "body-pose",
            AnalysisMode::HandPose => "hand-pose",
            AnalysisMode::SaliencyAttention => "saliency-attention",
            AnalysisMode::SaliencyObjectness => "saliency-objectness",
            AnalysisMode::Contours => "contours",
            AnalysisMode::FeaturePrint => "feature-print",
            AnalysisMode::Compare => "compare",
            AnalysisMode::Aesthetics => "aesthetics",
            AnalysisMode::Smudge => "smudge",
            AnalysisMode::Document => "document",
            AnalysisMode::Subject => "subject",
            AnalysisMode::PersonsMask => "persons-mask",
            AnalysisMode::All => "all",
            AnalysisMode::Model => "model",
            AnalysisMode::Motion => "motion",
            AnalysisMode::Align => "align",
            AnalysisMode::Track => "track",
            AnalysisMode::Trajectories => "trajectories",
            AnalysisMode::Video => "video",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointResult {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRLine {
    pub text: String,
    pub confidence: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRPayload {
    pub text: String,
    pub lines: Vec<OCRLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub label: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeResult {
    pub payload: String,
    pub symbology: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceResult {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanResult {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub confidence: f64,
    pub upper_body_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectangleResult {
    pub top_left: PointResult,
    pub top_right: PointResult,
    pub bottom_left: PointResult,
    pub bottom_right: PointResult,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizonResult {
    pub angle_radians: f64,
    pub angle_degrees: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimalResult {
    pub label: String,
    pub confidence: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugeResponse<T> {
    pub mode: String,
    pub file: String,
    pub results: T,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub on_device: bool,
    pub version: String,
    pub schema: String,
}
