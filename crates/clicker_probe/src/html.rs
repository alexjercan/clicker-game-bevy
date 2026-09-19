pub fn render_run_report(
    metadata: &serde_json::Value,
    snapshot: &str,
    timeline: &str,
    timing: &str,
    stdout: &str,
    stderr: &str,
    trace_available: bool,
) -> String {
    let success = metadata["success"].as_bool().unwrap_or(false);
    let verdict = if success { "PASS" } else { "FAIL" };
    let class = if success { "pass" } else { "fail" };
    let example = escape(metadata["example"].as_str().unwrap_or("unknown"));
    let timing_json = serde_json::from_str::<serde_json::Value>(timing).unwrap_or_default();
    let timing_complete = timing_json["complete"].as_bool().unwrap_or(false);
    let no_render = metadata["norender"].as_bool().unwrap_or(false);
    let rate_label = if no_render { "UPS" } else { "FPS" };
    let intervals = timing_complete.then_some(&timing_json["frame_intervals"]);
    let update_work = timing_complete.then_some(&timing_json["update_work"]);

    let mut html = report_head("Clicker probe report");
    html.push_str("<header><div><p class=\"eyebrow\">CLICKER PROBE</p><h1>");
    html.push_str(&example);
    html.push_str("</h1></div><div class=\"verdict ");
    html.push_str(class);
    html.push_str("\">");
    html.push_str(verdict);
    html.push_str("</div></header>");

    html.push_str("<section><h2>Run</h2><table><tbody>");
    table_row(
        &mut html,
        "Mode",
        if no_render { "No-render" } else { "Rendered" },
    );
    table_row(&mut html, "Seed", &display_value(&metadata["seed"]));
    table_row(
        &mut html,
        "Deadline",
        &format!("{} s", display_value(&metadata["deadline_secs"])),
    );
    table_row(
        &mut html,
        "Exit code",
        &display_value(&metadata["exit_code"]),
    );
    table_row(
        &mut html,
        "Artifacts complete",
        &display_value(&metadata["artifacts_complete"]),
    );
    html.push_str("</tbody></table></section>");

    html.push_str("<section><div class=\"section-heading\"><div><p class=\"eyebrow\">PERFORMANCE</p><h2>Update timing</h2></div><div class=\"headline\">");
    html.push_str(&rate_value(intervals, "mean_ms"));
    html.push(' ');
    html.push_str(rate_label);
    html.push_str("</div></div>");
    if !timing_complete {
        html.push_str("<p class=\"notice\">N/A: this example did not produce enough post-startup Playing samples.</p>");
    }
    html.push_str("<div class=\"columns\"><div><h3>Capture</h3><table><tbody>");
    table_row(
        &mut html,
        "Status",
        if timing_complete { "Complete" } else { "N/A" },
    );
    table_row(
        &mut html,
        "Captured updates",
        &display_value(&timing_json["captured_updates"]),
    );
    table_row(
        &mut html,
        "Target updates",
        &display_value(&timing_json["capture_updates"]),
    );
    table_row(
        &mut html,
        "Source runs",
        &display_value(&timing_json["source_runs"]),
    );
    html.push_str("</tbody></table></div><div><h3>Throughput</h3><table><thead><tr><th>Statistic</th><th>Frame time</th><th>");
    html.push_str(rate_label);
    html.push_str("</th></tr></thead><tbody>");
    timing_row(&mut html, "Mean", intervals, "mean_ms");
    timing_row(&mut html, "Median", intervals, "p50_ms");
    timing_row(&mut html, "p95", intervals, "p95_ms");
    timing_row(&mut html, "p99", intervals, "p99_ms");
    timing_row(&mut html, "Maximum", intervals, "max_ms");
    html.push_str("</tbody></table></div></div>");

    html.push_str("<h3>Main-world update work</h3><table><thead><tr><th>Statistic</th><th>Duration</th></tr></thead><tbody>");
    duration_row(&mut html, "Mean", update_work, "mean_ms");
    duration_row(&mut html, "Median", update_work, "p50_ms");
    duration_row(&mut html, "p95", update_work, "p95_ms");
    duration_row(&mut html, "p99", update_work, "p99_ms");
    duration_row(&mut html, "Maximum", update_work, "max_ms");
    html.push_str("</tbody></table><p class=\"explanation\">Frame time is measured only while the game is in Playing. Main-world update work covers First through Last.</p><div class=\"links\"><a href=\"timing.json\">timing.json</a>");
    if trace_available {
        html.push_str("<a href=\"trace.json\">trace.json</a>");
    }
    html.push_str("</div>");
    details(&mut html, "Raw timing", timing, false);
    html.push_str("</section>");

    html.push_str("<section><h2>Correctness evidence</h2>");
    details(&mut html, "Snapshot", snapshot, true);
    details(&mut html, "Timeline", timeline, false);
    html.push_str("</section><section><h2>Process logs</h2>");
    details(&mut html, "Standard output", stdout, false);
    details(
        &mut html,
        "Standard error",
        stderr,
        !stderr.trim().is_empty(),
    );
    html.push_str("</section></main></body></html>");
    html
}

pub fn render_all_report(rows: &[serde_json::Value]) -> String {
    let success = rows
        .iter()
        .all(|row| row["success"].as_bool().unwrap_or(false));
    let verdict = if success { "PASS" } else { "FAIL" };
    let mut html = report_head("Clicker probe report: all examples");
    html.push_str("<header><div><p class=\"eyebrow\">CLICKER PROBE</p><h1>All examples</h1></div><div class=\"verdict ");
    html.push_str(if success { "pass" } else { "fail" });
    html.push_str("\">");
    html.push_str(verdict);
    html.push_str("</div></header><section><h2>Runs</h2><table><thead><tr><th>Example</th><th>Result</th></tr></thead><tbody>");
    for row in rows {
        let example = escape(row["example"].as_str().unwrap_or("unknown"));
        let passed = row["success"].as_bool().unwrap_or(false);
        html.push_str("<tr><td><a href=\"");
        html.push_str(&example);
        html.push_str("/report.html\">");
        html.push_str(&example);
        html.push_str("</a></td><td class=\"status ");
        html.push_str(if passed { "pass" } else { "fail" });
        html.push_str("\">");
        html.push_str(if passed { "PASS" } else { "FAIL" });
        html.push_str("</td></tr>");
    }
    html.push_str("</tbody></table></section></main></body></html>");
    html
}

fn report_head(title: &str) -> String {
    let mut html = String::from("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>");
    html.push_str(&escape(title));
    html.push_str("</title><style>:root{color-scheme:dark;--bg:#101217;--panel:#191d25;--line:#394150;--muted:#9ba6b8;--text:#f0f3f8;--pass:#63d68a;--fail:#ff7272;--accent:#79b8ff}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:15px/1.5 system-ui,sans-serif}main{width:min(1180px,calc(100% - 32px));margin:0 auto 64px}header{display:flex;align-items:end;justify-content:space-between;padding:48px 0 24px;border-bottom:2px solid var(--line)}h1{margin:0;font-size:32px}h2{margin:0 0 16px;font-size:20px}h3{margin:24px 0 10px;color:var(--muted);font-size:13px;letter-spacing:.08em;text-transform:uppercase}.eyebrow{margin:0 0 4px;color:var(--accent);font-size:12px;font-weight:800;letter-spacing:.16em}.verdict{padding:8px 14px;border:2px solid currentColor;font-weight:800}.pass{color:var(--pass)}.fail{color:var(--fail)}section{margin-top:24px;padding:24px;background:var(--panel);border:1px solid var(--line)}.section-heading{display:flex;align-items:end;justify-content:space-between;margin-bottom:18px}.section-heading h2{margin:0}.headline{font-size:28px;font-weight:800;font-variant-numeric:tabular-nums}.columns{display:grid;grid-template-columns:minmax(240px,.7fr) minmax(420px,1.3fr);gap:24px}.columns h3{margin-top:0}table{width:100%;border-collapse:collapse;border:1px solid var(--line)}th,td{padding:10px 12px;border:1px solid var(--line);text-align:left;font-variant-numeric:tabular-nums}th{background:#151921;color:var(--muted);font-weight:700}td:last-child:not(:first-child),th:last-child:not(:first-child){text-align:right}.status{font-weight:800}.notice{padding:12px;border-left:4px solid var(--muted);background:#151921;color:var(--muted)}.explanation{color:var(--muted)}a{color:var(--accent)}.links{display:flex;gap:10px;margin:16px 0}.links a{padding:7px 11px;border:1px solid var(--line);text-decoration:none}details{margin-top:12px;border:1px solid var(--line)}summary{cursor:pointer;padding:11px 14px;font-weight:700}pre{max-height:560px;margin:0;padding:14px;overflow:auto;border-top:1px solid var(--line);background:#11141a;white-space:pre-wrap;font:12px/1.55 ui-monospace,monospace;color:#d9e0ea}@media(max-width:760px){header,.section-heading{align-items:start;gap:20px}.columns{grid-template-columns:1fr}.headline{font-size:22px}section{padding:16px}}</style></head><body><main>");
    html
}

fn timing_row(html: &mut String, label: &str, stats: Option<&serde_json::Value>, field: &str) {
    let milliseconds = stats.and_then(|stats| stats[field].as_f64());
    html.push_str("<tr><th>");
    html.push_str(&escape(label));
    html.push_str("</th><td>");
    html.push_str(&format_milliseconds(milliseconds));
    html.push_str("</td><td>");
    html.push_str(&milliseconds.map_or_else(
        || "N/A".to_string(),
        |milliseconds| format!("{:.1}", 1000.0 / milliseconds),
    ));
    html.push_str("</td></tr>");
}

fn duration_row(html: &mut String, label: &str, stats: Option<&serde_json::Value>, field: &str) {
    html.push_str("<tr><th>");
    html.push_str(&escape(label));
    html.push_str("</th><td>");
    html.push_str(&format_milliseconds(
        stats.and_then(|stats| stats[field].as_f64()),
    ));
    html.push_str("</td></tr>");
}

fn rate_value(stats: Option<&serde_json::Value>, field: &str) -> String {
    stats.and_then(|stats| stats[field].as_f64()).map_or_else(
        || "N/A".to_string(),
        |milliseconds| format!("{:.1}", 1000.0 / milliseconds),
    )
}

fn format_milliseconds(value: Option<f64>) -> String {
    value.map_or_else(|| "N/A".to_string(), |value| format!("{value:.3} ms"))
}

fn display_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "N/A".to_string(),
        serde_json::Value::String(value) => value.clone(),
        value => value.to_string(),
    }
}

fn table_row(html: &mut String, label: &str, value: &str) {
    html.push_str("<tr><th>");
    html.push_str(&escape(label));
    html.push_str("</th><td>");
    html.push_str(&escape(value));
    html.push_str("</td></tr>");
}

fn details(html: &mut String, title: &str, contents: &str, open: bool) {
    html.push_str(if open { "<details open>" } else { "<details>" });
    html.push_str("<summary>");
    html.push_str(&escape(title));
    html.push_str("</summary><pre>");
    html.push_str(&escape(contents));
    html.push_str("</pre></details>");
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
