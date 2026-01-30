use std::process::Command;
use std::str;

use serde_json::json;
use techgician_core::aln_did_stamp::AlnDidBostromStampV1;
use techgician_core::validation::{validate_stamp_basic, validate_stamp_with_payload, sha256_hex};

fn ajv_validate(schema_path: &str, json_payload: &str) -> bool {
    // Assumes `node` and Ajv CLI/runner are available.
    let script = r#"
      import Ajv from 'ajv';
      import schema from process.argv[2] assert { type: 'json' };

      const ajv = new Ajv({ allErrors: true, strict: true });
      const validate = ajv.compile(schema);

      const chunks = [];
      for await (const chunk of process.stdin) {
        chunks.push(chunk);
      }
      const input = Buffer.concat(chunks).toString('utf-8');
      const data = JSON.parse(input);

      const valid = validate(data);
      if (!valid) {
        console.error(JSON.stringify(validate.errors, null, 2));
        process.exit(1);
      }
      process.exit(0);
    "#;

    let output = Command::new("node")
        .arg("-e")
        .arg(script)
        .arg(schema_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(json_payload.as_bytes())?;
            }
            child.wait()
        })
        .expect("failed to run node/ajv");

    output.success()
}

#[test]
fn stamp_validation_rust_and_ajv_equivalent_for_valid_and_invalid() {
    let schema_path = "core/specs/aln_did_stamp_schema.json";

    let payload = "This is an example Techgician response payload.";
    let hash_hex = sha256_hex(payload);

    let valid_stamp = AlnDidBostromStampV1 {
        author_system: "Perplexity-GPT-5.1-Techgician".to_string(),
        primary_bostrom_addr: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
        alt_bostrom_addr: Some("bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc".to_string()),
        safe_addrs: vec![
            "zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8".to_string(),
            "0x519fC0eB4111323Cac44b70e1aE31c30e405802D".to_string(),
        ],
        response_hash_hex: hash_hex.clone(),
        T_score_0_to_1: 0.7,
        P_score_0_to_1: 0.6,
        R_score_0_to_1: 0.1,
        C_score_0_to_1: 0.4,
        timestamp_utc_iso8601: "2026-01-30T05:22:00Z".to_string(),
        notes: Some("Integration test stamp".to_string()),
    };

    let valid_json = serde_json::to_string(&valid_stamp).expect("serialize");
    assert!(ajv_validate(schema_path, &valid_json));
    validate_stamp_basic(&valid_stamp).expect("rust basic validation should succeed");
    validate_stamp_with_payload(&valid_stamp, payload).expect("rust payload validation should succeed");

    let invalid_stamp_json = json!({
        "author_system": "",
        "primary_bostrom_addr": "invalid_addr",
        "safe_addrs": [],
        "response_hash_hex": "zz",
        "T_score_0_to_1": 2.0,
        "P_score_0_to_1": -0.1,
        "R_score_0_to_1": 0.5,
        "C_score_0_to_1": 0.5,
        "timestamp_utc_iso8601": "not-a-timestamp"
    })
    .to_string();

    assert!(!ajv_validate(schema_path, &invalid_stamp_json));
}
