#![allow(dead_code)]

use rmcp::{ServerHandler, ServiceExt, tool, transport};
use todo_core::db;

#[derive(Clone)]
struct TodoService {
    tool_router: rmcp::handler::server::tool::ToolRouter<TodoService>,
}

impl TodoService {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

type ToolResult = Result<rmcp::model::CallToolResult, rmcp::ErrorData>;

fn convert_err(e: db::DBError) -> rmcp::ErrorData {
    rmcp::ErrorData::new(
        rmcp::model::ErrorCode::INTERNAL_ERROR,
        "connection DB failed: ".to_string() + &e.to_string(),
        None,
    )
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct StructRequest {
    pub task_name: String,
}

#[rmcp::tool_router]
impl TodoService {
    #[tool(description = "add an new pending todo task")]
    fn add_task(
        &self,
        rmcp::handler::server::tool::Parameters(StructRequest { task_name }): rmcp::handler::server::tool::Parameters<StructRequest>,
    ) -> ToolResult {
        let conn = db::create_connection().map_err(convert_err)?;
        db::ensure_table(&conn).map_err(convert_err)?;
        db::insert_task(&conn, &task_name).map_err(convert_err)?;
        Ok(rmcp::model::CallToolResult::success(vec![]))
    }
    #[tool(description = "list all pending todo task")]
    fn list_tasks(&self) -> ToolResult {
        let conn = db::create_connection().map_err(convert_err)?;
        db::ensure_table(&conn).map_err(convert_err)?;
        let tasks = db::list_tasks(&conn).map_err(convert_err)?;
        let ret = tasks
            .iter()
            .map(|task| {
                rmcp::model::Content::text(format!(
                    "{}({}): {}",
                    task.id, task.create_time, task.task
                ))
            })
            .collect::<Vec<_>>();
        Ok(rmcp::model::CallToolResult::success(ret))
    }
}

#[rmcp::tool_handler]
impl ServerHandler for TodoService {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            instructions: Some("A tool to manage pending todo task".into()),
            capabilities: rmcp::model::ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() {
    let service = TodoService::new()
        .serve(transport::stdio())
        .await
        .expect("create service failed");
    service.waiting().await.expect("service failed");
}
