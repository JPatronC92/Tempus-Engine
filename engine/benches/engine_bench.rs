use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use tempus_engine::metadata::RuleDefinition;
use tempus_engine::{execute, execute_batch, execute_chain, execute_explain, RuleStore};

// ─────────────────────────────────────────────────────────────────────────────
// Fixtures
// ─────────────────────────────────────────────────────────────────────────────

fn score_rule() -> RuleDefinition {
    RuleDefinition::new(
        "score-check",
        json!({"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]}),
    )
    .with_version("1.0.0")
    .with_tags(vec!["credit".into(), "prod".into()])
}

fn chain_rules() -> Vec<RuleDefinition> {
    vec![
        RuleDefinition::new(
            "classify",
            json!({"if": [{">": [{"var": "score"}, 700]}, "high", "low"]}),
        ),
        RuleDefinition::new(
            "limit",
            json!({"if": [{"==": [{"var": "decision"}, "high"]}, 10000, 1000]}),
        ),
    ]
}

fn sample_store_json() -> String {
    let rules: Vec<serde_json::Value> = (0..100)
        .map(|i| {
            json!({
                "name": format!("rule-{}", i),
                "logic": {">": [{"var": "x"}, i]}
            })
        })
        .collect();
    serde_json::to_string(&rules).unwrap()
}

// ─────────────────────────────────────────────────────────────────────────────
// Benchmarks
// ─────────────────────────────────────────────────────────────────────────────

fn bench_execute(c: &mut Criterion) {
    let rule = score_rule();
    let ctx = r#"{"score": 800}"#;
    c.bench_function("execute_single", |b| {
        b.iter(|| execute(black_box(&rule), black_box(ctx)).unwrap())
    });
}

fn bench_execute_batch(c: &mut Criterion) {
    let rule = score_rule();
    let contexts: Vec<String> = (0..1000)
        .map(|i| format!(r#"{{"score": {}}}"#, 500 + (i % 500)))
        .collect();

    c.bench_function("execute_batch_1000", |b| {
        b.iter(|| execute_batch(black_box(&rule), black_box(&contexts)).unwrap())
    });
}

fn bench_execute_chain(c: &mut Criterion) {
    let rules = chain_rules();
    let ctx = r#"{"score": 750}"#;
    c.bench_function("execute_chain_2_rules", |b| {
        b.iter(|| execute_chain(black_box(&rules), black_box(ctx)).unwrap())
    });
}

fn bench_execute_explain(c: &mut Criterion) {
    let rule = score_rule();
    let ctx = r#"{"score": 800}"#;
    c.bench_function("execute_explain", |b| {
        b.iter(|| execute_explain(black_box(&rule), black_box(ctx)).unwrap())
    });
}

fn bench_rule_store_get(c: &mut Criterion) {
    let json = sample_store_json();
    let store = RuleStore::from_json_str(&json).unwrap();

    c.bench_function("rule_store_get_100_rules", |b| {
        b.iter(|| store.get(black_box("rule-50")).unwrap())
    });
}

fn bench_rule_store_load(c: &mut Criterion) {
    let json = sample_store_json();
    c.bench_function("rule_store_load_100_rules", |b| {
        b.iter(|| RuleStore::from_json_str(black_box(&json)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_execute,
    bench_execute_batch,
    bench_execute_chain,
    bench_execute_explain,
    bench_rule_store_get,
    bench_rule_store_load,
);
criterion_main!(benches);
