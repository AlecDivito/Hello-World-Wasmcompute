use std::{fmt::Display, time::Instant};

use wasmcompute_lib::cache::Cache;
use wasmcompute_lib::db::Sql;
use wasmcompute_lib::http::types::StatusCode;
use wasmcompute_lib::http::{http_serve, WasmRequest, WasmResponse};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct UserLog {
    id: i64,
    duration: u128,
    addr: String,
    method: String,
    url_path: String,
    status: String,
}

impl UserLog {
    pub fn get_recent_db() -> Vec<UserLog> {
        Sql::query("SELECT * FROM user_log ORDER BY id DESC LIMIT 50")
            .fetch()
            .unwrap_or(vec![])
    }

    pub fn get_recent_cache() -> Vec<UserLog> {
        match Cache::get("logs").unwrap() {
            Some(users) => serde_json::from_str(&users).unwrap_or(vec![]),
            None => vec![],
        }
    }

    pub fn insert(&self) {
        println!("{}", self);
        match Sql::query(
            "INSERT INTO user_log (duration, addr, method, url_path, status) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(self.duration)
        .bind(self.addr.clone())
        .bind(self.method.clone())
        .bind(self.url_path.clone())
        .bind(self.status.clone())
        .execute() {
            Ok(_) => println!("successfully inserted {}", self),
            Err(_) => println!("failed to insert {}", self)
        };
        let mut recent_users = UserLog::get_recent_cache();
        recent_users.push(self.clone());
        if recent_users.len() > 5 {
            recent_users.reverse();
            recent_users.pop();
        }
        let json = serde_json::to_string(&recent_users).unwrap_or("[]".to_string());
        Cache::set("logs", &json).unwrap_or(());
    }

    pub fn get_table_html(&self) -> String {
        format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            self.duration, self.addr, self.method, self.url_path, self.status,
        )
    }
}

impl Display for UserLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}µ) [{}] {:?} {} {}",
            self.duration, self.addr, self.method, self.url_path, self.status
        )
    }
}

fn handle(req: WasmRequest) -> WasmResponse {
    let start = Instant::now();

    let db_users = UserLog::get_recent_db();
    let cache_users = UserLog::get_recent_cache();
    let html = build_page(cache_users, db_users);

    let mut res = WasmResponse::new(StatusCode::Ok);
    res.html(&html);
    log_request(start, &req, res)
}

fn log_request(start: Instant, req: &WasmRequest, res: WasmResponse) -> WasmResponse {
    let addr = req
        .peer_address
        .as_ref()
        .unwrap_or(&String::from("0.0.0.0"))
        .clone();
    let path = req.path();
    let method = &req.method;
    let duration = start.elapsed().as_millis();

    UserLog {
        id: 0,
        duration,
        addr,
        method: format!("{}", method),
        url_path: path.to_string(),
        status: format!("{}", res.status),
    }
    .insert();

    res
}

fn main() -> wasmcompute_lib::error::Result<()> {
    match http_serve(handle) {
        Ok(_) => println!("User was serviced successfully"),
        Err(e) => println!("The application errored '{}' we paniced", e),
    };
    Ok(())
}

fn build_page(cached_users: Vec<UserLog>, db_users: Vec<UserLog>) -> String {
    let converter = |data: Vec<UserLog>| {
        data.iter()
            .fold(String::new(), |o, n| format!("{}{}", o, n.get_table_html()))
    };

    let index_file = match std::fs::read_to_string("/app/templates/index.html") {
        Ok(s) => s,
        Err(e) => {
            let err = format!("Failed to read /app/index.html file with error: {}", e);
            println!("{}", err);
            return err;
        }
    };

    let cached_requests = converter(cached_users);
    let db_requests = converter(db_users);

    let index_file = index_file.replace("{{CACHE_RESPONSES}}", &cached_requests);
    let index_file = index_file.replace("{{DATABASE_RESPONSES}}", &db_requests);
    index_file
}
