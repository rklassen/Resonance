use serde::Deserialize;

pub const G2_LATENT_SWEEP_FIXTURE_JSON: &str =
    include_str!("../artifacts/gamma/g2-latent-axis-sweeps-v1.json");

#[derive(Debug, Deserialize)]
pub struct GammaLatentSweepFixture {
    pub source: String,
    pub text: String,
    pub axes: Vec<GammaLatentAxisFixture>,
}

#[derive(Debug, Deserialize)]
pub struct GammaLatentAxisFixture {
    pub axis: String,
    pub left_pole: String,
    pub right_pole: String,
    pub model_id: String,
    pub output_contract: String,
    pub mean_score: f32,
    pub spread: f32,
    pub stability: String,
    pub dominant_pole: Option<String>,
    pub variants: Vec<GammaLatentVariantFixture>,
}

#[derive(Debug, Deserialize)]
pub struct GammaLatentVariantFixture {
    pub name: String,
    pub prompt_id: String,
    pub score: f32,
}

pub fn assert_close(actual: f32, expected: f32, label: &str) {
    let delta = (actual - expected).abs();
    assert!(delta <= 1.0e-6, "{label} drifted: expected {expected:+.9}, got {actual:+.9}");
}