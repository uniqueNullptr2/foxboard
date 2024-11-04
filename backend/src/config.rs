use clap::Parser;

#[derive(Parser)]
#[command(name="foxboard", version="0.1", about="personal kanban board for zoomy creatures", long_about = None)]
pub struct Config {
    #[arg(long, env = "FOXB_DB_HOST")]
    pub db_host: String,
    #[arg(long, env = "FOXB_DB_USER")]
    pub db_user: String,
    #[arg(long, env = "FOXB_DB_PASSWORD")]
    pub db_password: String,
    #[arg(long, env = "FOXB_DB_DB")]
    pub db_db: String,
    #[arg(long, env = "FOXB_ADMIN_USER")]
    pub admin_user: String,
    #[arg(long, env = "FOXB_DB_ADMIN_INITIAL_PASSWORD")]
    pub admin_initial_password: String,

    #[arg(long, env = "FOXB_SMTP_USER")]
    pub smtp_user: Option<String>,
    #[arg(long, env = "FOXB_SMTP_HOST")]
    pub smtp_host: Option<String>,
    #[arg(long, env = "FOXB_SMTP_PASSWORD")]
    pub smtp_password: Option<String>,
    /// Optional name to operate on
    pub name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, env="FOXB_DEBUG", action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}
