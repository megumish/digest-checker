use std::path::PathBuf;

use bearer_auth::BearerAuth;
use clap::{Parser, Subcommand};
use registry::Registry;
use reqwest::{header, StatusCode};
use serde_json::Value;

mod bearer_auth;
mod registry;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    registry: String,
    #[clap(short, long, value_parser)]
    image: String,
    #[clap(short, long, value_parser)]
    tag: Option<String>,
    #[clap(value_parser)]
    digest: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ecr {
        #[clap(short, long, value_parser, value_name = "FILE")]
        password: PathBuf,
        #[clap(short, long, value_parser)]
        repository: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let registry = cli.registry.into();
    let image = cli.image;
    let tag = cli.tag.unwrap_or("latest".to_owned());
    let digest = cli.digest;

    if let Some(digest) = is_cached(registry, image, tag, digest)? {
        println!("{digest}");
    } else {
        println!("true");
    }

    Ok(())
}

fn is_cached(
    registry: Registry,
    image: String,
    tag: String,
    target_digest: String,
) -> Result<Option<String>, anyhow::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("https://{registry}/v2/{image}/manifests/{tag}"))
        .header(
            header::ACCEPT,
            "application/vnd.docker.distribution.manifest.list.v2+json",
        )
        .send()?;
    if res.status() == StatusCode::OK {
        let headers = res.headers();
        if let Some(digest) = headers
            .get("docker-content-digest")
            .or(headers.get("Docker-Content-Digest"))
        {
            let digest = digest.to_str()?;
            if target_digest == digest {
                return Ok(None);
            } else {
                return Ok(Some(digest.to_owned()));
            }
        }
    }
    let headers = res.headers();
    if let Some(www_autheticate) = headers.get(header::WWW_AUTHENTICATE) {
        let www_authenticate = www_autheticate.to_str()?;
        let token = {
            let auth: BearerAuth = www_authenticate.parse()?;
            let auth_url = auth.to_url();
            let res = client.get(auth_url).send()?;
            let body = res.text()?.to_owned();
            let json_body: Value = serde_json::from_str(&body)?;
            let token = json_body["token"].to_string();
            token[1..token.len() - 1].to_owned()
        };
        let res = client
            .get(format!("https://{registry}/v2/{image}/manifests/{tag}"))
            .header(
                header::ACCEPT,
                "application/vnd.docker.distribution.manifest.list.v2+json",
            )
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .send()?;
        if res.status() == StatusCode::OK {
            let headers = res.headers();
            if let Some(digest) = headers
                .get("docker-content-digest")
                .or(headers.get("Docker-Content-Digest"))
            {
                let digest = digest.to_str()?;
                if target_digest == digest {
                    return Ok(None);
                } else {
                    return Ok(Some(digest.to_owned()));
                }
            }
        }
    }
    anyhow::bail!("digest is not found");
}
