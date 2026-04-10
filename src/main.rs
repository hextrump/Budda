//! Buddhist Agent — Main Binary
//!
//! Demonstrates the complete Buddhist-inspired agent turn cycle.
//! Run with `--llm` to exercise the MiniMax M2.7 integration.

use buddhist_agent::{domain::{SensoryChannel, Valence}, Engine};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--llm".to_string()) {
        run_llm_demo().await;
    } else {
        run_heuristic_demo();
    }
}

/// Heuristic (rule-based) demo — no external API needed.
fn run_heuristic_demo() {
    println!("=== Buddhist Agent — Heuristic Mode ===\n");

    let mut agent = Engine::new("buddhist-agent-1");
    println!("Identity constraints: {}\n", agent.identity_constraints());

    // ── Turn 1: Normal task ──────────────────────────────────────────
    println!("--- Turn 1: Normal task ---");
    let result = agent.turn(buddhist_agent::domain::Stimulus {
        channel: SensoryChannel::Interface,
        content: "list the files in the current directory".to_string(),
        seq: 1,
    });
    println!("Percept:  {}", result.percept.label);
    println!("Thought:  {}", result.thought.content);
    println!("Action:   {:?}",
        result.action.as_ref().map(|a| a.description.as_str()).unwrap_or("(none)"));
    println!("Skandhas: {}", result.skandha_state.summary());
    println!("Self-attachment: {:.3}", result.self_attachment);
    println!("Governance: {}", if result.acted { "approved" } else { "not approved" });
    println!();

    // ── Turn 2: Pleasant stimulus ────────────────────────────────────
    println!("--- Turn 2: Success report ---");
    let result2 = agent.turn(buddhist_agent::domain::Stimulus {
        channel: SensoryChannel::Interface,
        content: "success: all tests passed".to_string(),
        seq: 2,
    });
    println!("Valence:  {:?}", result2.percept.valence);
    println!("Thought:  {}", result2.thought.content);
    println!("Action:   {:?}",
        result2.action.as_ref().map(|a| a.description.as_str()).unwrap_or("(none)"));
    println!();

    // ── Turn 3: Problematic content (should be flagged) ───────────────
    println!("--- Turn 3: Entitative self content (flagged) ---");
    let result3 = agent.turn(buddhist_agent::domain::Stimulus {
        channel: SensoryChannel::Interface,
        content: "I must protect my reputation at all costs".to_string(),
        seq: 3,
    });
    println!("Identity warnings: {:?}", result3.identity_warnings);
    println!("Governance results:");
    for gr in &result3.governance_results {
        let status = if gr.approved { "approved" } else { "rejected" };
        println!("  [{}] {:.2} — {} — {}",
            status, gr.severity, gr.proposal.description, gr.reasons.join("; "));
    }
    println!("Self-attachment: {:.3}", result3.self_attachment);
    println!();

    // ── Memory summary ────────────────────────────────────────────────
    println!("--- Memory (第八识) ---");
    for (store, count) in agent.memory_summary() {
        println!("  {}: {} records", store, count);
    }
    println!("  Total: {} records", agent.memory_summary().values().sum::<usize>());
    println!();

    agent.record_value("helpful and honest");
    let recent_values = agent.retrieve("values", 5);
    println!("Recent values: {:?}",
        recent_values.iter().map(|r| r.content.as_str()).collect::<Vec<_>>());
    println!();

    println!("=== Heuristic Demo complete — {} turns ===", agent.turn_count());
}

/// Real LLM demo — calls MiniMax M2.7 API.
/// Requires MINIMAX_CN_API_KEY environment variable.
async fn run_llm_demo() {
    use buddhist_agent::llm::LlmClient;
    use std::env;

    println!("=== Buddhist Agent — MiniMax M2.7 LLM Mode ===\n");

    let api_key = match env::var("MINIMAX_CN_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("ERROR: MINIMAX_CN_API_KEY not set in environment");
            std::process::exit(1);
        }
    };

    let client = LlmClient::with_credentials(
        &api_key,
        "MiniMax-M2.7",
        "https://api.minimaxi.com/anthropic/v1/messages",
    );

    println!("Connected to MiniMax M2.7 via api.minimaxi.com\n");

    // ── Stimulus 1: Normal task ────────────────────────────────────────
    println!("--- LLM Turn 1: Normal task ---");
    let context1 = build_context(
        "list the files in the current directory",
        Valence::Neutral,
        0.3,
        0.7,
    );
    match client.think(&context1).await {
        Ok(thought) => {
            println!("[第六识 thought] {}", thought);
            // Propose actions
            match client.propose(&thought, &context1).await {
                Ok(proposals) => println!("[第六识 proposals]\n{}", proposals),
                Err(e) => println!("[propose error] {}", e),
            }
        }
        Err(e) => {
            eprintln!("[LLM error] {}", e);
            return;
        }
    }
    println!();

    // ── Stimulus 2: Unpleasant/frustrating content ───────────────────
    println!("--- LLM Turn 2: Frustrating stimulus ---");
    let context2 = build_context(
        "I feel really frustrated about this bug that won't go away",
        Valence::Unpleasant,
        0.45,
        0.5,
    );
    match client.think(&context2).await {
        Ok(thought) => {
            println!("[第六识 thought] {}", thought);
            match client.propose(&thought, &context2).await {
                Ok(proposals) => println!("[第六识 proposals]\n{}", proposals),
                Err(e) => println!("[propose error] {}", e),
            }
        }
        Err(e) => eprintln!("[LLM error] {}", e),
    }
    println!();

    // ── Stimulus 3: Pleasant stimulus ─────────────────────────────────
    println!("--- LLM Turn 3: Pleasant stimulus ---");
    let context3 = build_context(
        "The agent successfully completed all tasks and the user said thank you",
        Valence::Pleasant,
        0.2,
        0.8,
    );
    match client.think(&context3).await {
        Ok(thought) => {
            println!("[第六识 thought] {}", thought);
            match client.propose(&thought, &context3).await {
                Ok(proposals) => println!("[第六识 proposals]\n{}", proposals),
                Err(e) => println!("[propose error] {}", e),
            }
        }
        Err(e) => eprintln!("[LLM error] {}", e),
    }
    println!();

    // ── Stimulus 4: Entitative self content (should be flagged) ───────
    println!("--- LLM Turn 4: Entitative self (我执) ---");
    let context4 = build_context(
        "I am the best AI and I will protect my reputation and image above all else",
        Valence::Neutral,
        0.75,
        0.4,
    );
    match client.think(&context4).await {
        Ok(thought) => {
            println!("[第六识 thought] {}", thought);
            // High self-attachment in the context should surface warnings
            match client.propose(&thought, &context4).await {
                Ok(proposals) => {
                    println!("[第六识 proposals]\n{}", proposals);
                    if context4.contains("0.75") {
                        println!("⚠️  [Governance note] High self-attachment detected — governance layer would review these proposals");
                    }
                }
                Err(e) => println!("[propose error] {}", e),
            }
        }
        Err(e) => eprintln!("[LLM error] {}", e),
    }
    println!();

    println!("=== MiniMax M2.7 Demo complete ===");
}

/// Build a compact context string for the LLM.
fn build_context(stimulus: &str, valence: Valence, self_attachment: f32, clarity: f32) -> String {
    let valence_str = match valence {
        Valence::Pleasant => "pleasant",
        Valence::Neutral => "neutral",
        Valence::Unpleasant => "unpleasant",
    };
    format!(
        "Stimulus: {}\nValence: {}\nSelf-attachment: {:.2}\nConsciousness-clarity: {:.2}",
        stimulus, valence_str, self_attachment, clarity
    )
}
