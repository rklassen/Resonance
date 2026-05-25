use std::collections::BTreeMap;

use crate::beta::{BetaGain, BetaGainTerm};

use super::{
    GammaConfig, GammaError, GammaPriorAlignment, GammaPriorEnsembleSuite, GammaPriorRecord,
    GammaPriorSource,
};

#[derive(Clone, Debug, PartialEq)]
pub struct GammaReceptorFamilyComparison {
    pub family: Option<String>,
    pub gain_mean: Option<f32>,
    pub gain_variance: Option<f32>,
    pub source_disagreement: Option<f32>,
    pub unsupported_family: bool,
    pub prior_ids: Vec<String>,
    pub atlas_ids: Vec<String>,
    pub transform_ids: Vec<String>,
    pub detail: String,
    pub required_follow_up: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaReceptorBridgeSuite {
    pub families: Vec<GammaReceptorFamilyComparison>,
    pub extensions_disabled: bool,
}

pub fn run_gamma_receptor_bridge_suite(
    prior_ensemble: &GammaPriorEnsembleSuite,
    gain: &BetaGain,
    config: &GammaConfig,
) -> Result<GammaReceptorBridgeSuite, GammaError> {
    if config.extensions_disabled {
        return Ok(GammaReceptorBridgeSuite {
            families: Vec::new(),
            extensions_disabled: true,
        });
    }

    let aligned_priors = prior_ensemble
        .priors
        .iter()
        .filter(|prior| {
            prior.source == GammaPriorSource::ReceptorMaps
                && prior.alignment == GammaPriorAlignment::CoordinateAligned
        })
        .cloned()
        .collect::<Vec<_>>();

    let families = build_receptor_family_comparisons(&aligned_priors, &gain.terms)?;

    Ok(GammaReceptorBridgeSuite {
        families,
        extensions_disabled: false,
    })
}

fn build_receptor_family_comparisons(
    aligned_priors: &[GammaPriorRecord],
    gain_terms: &[BetaGainTerm],
) -> Result<Vec<GammaReceptorFamilyComparison>, GammaError> {
    let aligned_by_id = aligned_priors
        .iter()
        .map(|prior| (prior.prior_id.clone(), prior))
        .collect::<BTreeMap<_, _>>();
    let mut grouped = BTreeMap::<String, Vec<(&BetaGainTerm, &GammaPriorRecord)>>::new();

    for term in gain_terms {
        let Some(prior) = aligned_by_id.get(&term.prior_id) else {
            return Err(GammaError::new(format!(
                "gamma receptor bridge missing aligned provenance for gain prior {}",
                term.prior_id
            )));
        };
        if prior.atlas_id.is_none() || prior.transform_id.is_none() {
            return Err(GammaError::new(format!(
                "gamma receptor bridge cannot emit gain fields without provenance for prior {}",
                term.prior_id
            )));
        }
        grouped.entry(term.family.clone()).or_default().push((term, *prior));
    }

    let mut families = grouped
        .into_iter()
        .map(|(family, entries)| supported_family_comparison(&family, &entries))
        .collect::<Vec<_>>();

    let unsupported = aligned_priors
        .iter()
        .filter(|prior| gain_terms.iter().all(|term| term.prior_id != prior.prior_id))
        .map(unsupported_family_comparison)
        .collect::<Vec<_>>();
    families.extend(unsupported);

    Ok(families)
}

fn supported_family_comparison(
    family: &str,
    entries: &[(&BetaGainTerm, &GammaPriorRecord)],
) -> GammaReceptorFamilyComparison {
    let coefficients = entries.iter().map(|(term, _)| term.coefficient).collect::<Vec<_>>();
    let gain_mean = coefficients.iter().sum::<f32>() / coefficients.len() as f32;
    let gain_variance = coefficients
        .iter()
        .map(|coefficient| {
            let delta = *coefficient - gain_mean;
            delta * delta
        })
        .sum::<f32>()
        / coefficients.len() as f32;
    let min = coefficients.iter().copied().fold(f32::INFINITY, f32::min);
    let max = coefficients.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let prior_ids = entries.iter().map(|(term, _)| term.prior_id.clone()).collect::<Vec<_>>();
    let atlas_ids =
        entries.iter().filter_map(|(_, prior)| prior.atlas_id.clone()).collect::<Vec<_>>();
    let transform_ids =
        entries.iter().filter_map(|(_, prior)| prior.transform_id.clone()).collect::<Vec<_>>();

    GammaReceptorFamilyComparison {
        family: Some(family.into()),
        gain_mean: Some(gain_mean),
        gain_variance: Some(gain_variance),
        source_disagreement: Some(max - min),
        unsupported_family: false,
        prior_ids,
        atlas_ids,
        transform_ids,
        detail: format!(
            "receptor family {} compares {} aligned prior source(s) with mean {:+.6}, variance {:.6}, disagreement {:.6}",
            family,
            entries.len(),
            gain_mean,
            gain_variance,
            max - min,
        ),
        required_follow_up: None,
    }
}

fn unsupported_family_comparison(prior: &GammaPriorRecord) -> GammaReceptorFamilyComparison {
    GammaReceptorFamilyComparison {
        family: None,
        gain_mean: None,
        gain_variance: None,
        source_disagreement: None,
        unsupported_family: true,
        prior_ids: vec![prior.prior_id.clone()],
        atlas_ids: prior.atlas_id.clone().into_iter().collect(),
        transform_ids: prior.transform_id.clone().into_iter().collect(),
        detail: format!(
            "aligned receptor prior {} is not supported by the current receptor family bridge semantics",
            prior.prior_id
        ),
        required_follow_up: Some(
            "extend receptor family semantics before emitting gain summaries for this prior".into(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_receptor_family_comparisons_marks_unsupported_aligned_prior() {
        let aligned_priors = vec![GammaPriorRecord {
            source: GammaPriorSource::ReceptorMaps,
            prior_id: "prior-unsupported".into(),
            alignment: GammaPriorAlignment::CoordinateAligned,
            atlas_id: Some("atlas-demo".into()),
            transform_id: Some("transform-demo".into()),
            detail: "aligned for test".into(),
            required_follow_up: None,
        }];
        let comparisons = build_receptor_family_comparisons(&aligned_priors, &[])
            .expect("comparison build should succeed");

        assert_eq!(comparisons.len(), 1);
        assert!(comparisons[0].unsupported_family);
        assert_eq!(comparisons[0].prior_ids, vec!["prior-unsupported".to_string()]);
        assert!(comparisons[0].gain_mean.is_none());
        assert!(comparisons[0].required_follow_up.is_some());
    }
}
