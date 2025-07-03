use chrono::Local;
use clap::Parser;
use dotenv::dotenv;
use inquire::Select;
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;
use std::env;
use std::fs;

mod payload_editor;
use payload_editor::{load_payload_templates, prompt_valid_json_for_rpc};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
}

#[derive(Serialize)]
struct NakamaRequest {
    id: String,
    payload: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct NakamaResponse {
    payload: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let args = Args::parse();

    let http_key = env::var("NAKAMA_HTTP_KEY").expect("NAKAMA_HTTP_KEY missing in .env");
    let nakama_url = env::var("NAKAMA_URL").expect("NAKAMA_URL missing in .env");

    let templates = load_payload_templates("rpc_payload_models.json");
    let client = reqwest::Client::new();

    loop {
        let rpc_functions = fs::read_to_string("rpc_function_names.txt")
            .expect("Cannot read rpc_function_names.txt");
        let rpc_list: Vec<&str> = rpc_functions.lines().collect();

        let selected_rpc = Select::new("Choose a RPC function:", rpc_list)
            .prompt()
            .unwrap();

        let payload = prompt_valid_json_for_rpc(selected_rpc, &templates);

        let url = format!("{nakama_url}/v2/rpc/{selected_rpc}?http_key={http_key}");
        println!("➡️ Calling: {url}");

        let res = client
            .post(&url)
            .json(&NakamaRequest {
                id: selected_rpc.to_string(),
                payload: serde_json::to_string(&payload).expect("Failed to stringify JSON payload"),
            })
            .send()
            .await?;

        let status = res.status();
        let content_type = res
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if status.is_success() && content_type.contains("application/json") {
            let nk_resp: NakamaResponse = res.json().await?;
            println!("✅ Success: {}", nk_resp.payload);

            let timestamp = Local::now().format("%Y-%m-%dT%H-%M-%S").to_string();
            let log_path = format!("logs/{timestamp}_{selected_rpc}.json");
            fs::write(&log_path, serde_json::to_string_pretty(&nk_resp).unwrap())
                .expect("Cannot write log file");
        } else {
            let error_text = res.text().await?;
            eprintln!("❌ Error {status}: {error_text}");
        }

        if !args.interactive {
            break;
        }
    }

    Ok(())
}
