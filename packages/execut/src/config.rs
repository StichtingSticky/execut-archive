use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub jwt_secret: String,
}
