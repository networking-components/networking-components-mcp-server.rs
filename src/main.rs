//! Fleet-generated organization MCP server entry point.

#[allow(clippy::all, clippy::pedantic)]
#[path = "../generated/rust/env.rs"]
mod env;
#[allow(clippy::all, clippy::pedantic)]
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;
mod spec;

use ore_mcp_org_server::run_stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_stdio(spec::org_spec()).await
}
