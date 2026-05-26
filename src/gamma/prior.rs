use crate::beta::{load_public_fixtures, BetaFixturePrior};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GammaPriorSource {
    ReceptorMaps,
    FunctionalGradients,
    StructuralConnectivity,
    VisualBenchmarkPriors,
    ImageryPriors,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GammaPriorAlignment {
    CoordinateAligned,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaPriorRecord {
    pub source: GammaPriorSource,
    pub prior_id: String,
    pub source_record: Option<String>,
    pub alignment: GammaPriorAlignment,
    pub atlas_id: Option<String>,
    pub transform_id: Option<String>,
    pub detail: String,
    pub required_follow_up: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaPriorEnsembleSuite {
    pub priors: Vec<GammaPriorRecord>,
}

pub fn run_gamma_prior_ensemble_suite() -> crate::SemanticResult<GammaPriorEnsembleSuite> {
    let fixtures = load_public_fixtures()?;
    let mut priors = fixtures
        .priors
        .iter()
        .map(|prior| aligned_receptor_prior(fixtures.atlas.id.as_str(), prior))
        .collect::<Vec<_>>();

    priors.extend([
        blocked_prior(
            GammaPriorSource::FunctionalGradients,
            "functional-gradients.pending",
            "functional gradient priors are not yet declared with a coordinate-aligned source record",
            "add a declared functional gradient source plus transform provenance before enabling gamma prior use",
        ),
        blocked_prior(
            GammaPriorSource::StructuralConnectivity,
            "structural-connectivity.pending",
            "structural connectivity priors are not yet declared with a coordinate-aligned source record",
            "add a declared structural connectivity source plus transform provenance before enabling gamma prior use",
        ),
        blocked_prior(
            GammaPriorSource::VisualBenchmarkPriors,
            "visual-benchmark-priors.pending",
            "visual benchmark priors are not yet declared with a coordinate-aligned source record",
            "add declared visual benchmark priors plus transform provenance before enabling gamma prior use",
        ),
        blocked_prior(
            GammaPriorSource::ImageryPriors,
            "imagery-priors.pending",
            "imagery priors are not yet declared with a coordinate-aligned source record",
            "add declared imagery priors plus transform provenance before enabling gamma prior use",
        ),
    ]);

    Ok(GammaPriorEnsembleSuite {
        priors,
    })
}

fn aligned_receptor_prior(atlas_id: &str, prior: &BetaFixturePrior) -> GammaPriorRecord {
    GammaPriorRecord {
        source: GammaPriorSource::ReceptorMaps,
        prior_id: prior.id.clone(),
        source_record: Some(prior.source.clone()),
        alignment: GammaPriorAlignment::CoordinateAligned,
        atlas_id: Some(atlas_id.into()),
        transform_id: Some(prior.transform.clone()),
        detail: format!(
            "receptor prior {}:{} is coordinate-aligned through {}",
            prior.source, prior.desc, prior.transform
        ),
        required_follow_up: None,
    }
}

fn blocked_prior(
    source: GammaPriorSource,
    prior_id: &str,
    detail: &str,
    required_follow_up: &str,
) -> GammaPriorRecord {
    GammaPriorRecord {
        source,
        prior_id: prior_id.into(),
        source_record: None,
        alignment: GammaPriorAlignment::Blocked,
        atlas_id: None,
        transform_id: None,
        detail: detail.into(),
        required_follow_up: Some(required_follow_up.into()),
    }
}
