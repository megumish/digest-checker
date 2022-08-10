use std::str::FromStr;

pub struct BearerAuth {
    realm: reqwest::Url,
    service: String,
    scope: Option<String>,
}

impl BearerAuth {
    pub fn to_url(&self) -> String {
        if let Some(ref scope) = self.scope {
            format!("{}?service={}&scope={}", self.realm, self.service, scope)
        } else {
            format!("{}?service={}", self.realm, self.service)
        }
    }
}

impl FromStr for BearerAuth {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("Bearer") {
            return Err(Self::Err::NoBearer);
        }
        let s = &s["Bearer".len()..];

        Ok(parse(s)?)
    }
}

#[derive(Debug)]
struct BearerAuthBuilder {
    realm: Option<String>,
    service: Option<String>,
    scope: Option<String>,
}

fn parse(s: &str) -> Result<BearerAuth, Error> {
    let mut builder = BearerAuthBuilder {
        realm: None,
        service: None,
        scope: None,
    };

    let values = s.split(",");
    for v in values {
        let v = v.trim_start();
        if v.starts_with("realm") {
            let v = &v["realm".len()..];
            let v = v.trim_start();
            if v.starts_with("=") {
                let v = &v[1..];
                let v = v.trim_start();
                if v.starts_with("\"") {
                    let v = v.trim_end();
                    if v.ends_with("\"") {
                        builder.realm = Some(v[1..v.len() - 1].to_owned());
                    }
                }
            }
        }
        if v.starts_with("service") {
            let v = &v["service".len()..];
            let v = v.trim_start();
            if v.starts_with("=") {
                let v = &v[1..];
                if v.starts_with("\"") {
                    let v = v.trim_end();
                    if v.ends_with("\"") {
                        builder.service = Some(v[1..v.len() - 1].to_owned());
                    }
                }
            }
        }
        if v.starts_with("scope") {
            let v = &v["scope".len()..];
            let v = v.trim_start();
            if v.starts_with("=") {
                let v = &v[1..];
                if v.starts_with("\"") {
                    let v = v.trim_end();
                    if v.ends_with("\"") {
                        builder.scope = Some(v[1..v.len() - 1].to_owned());
                    }
                }
            }
        }
    }
    builder.build()
}

impl BearerAuthBuilder {
    fn build(self) -> Result<BearerAuth, Error> {
        let realm = self.realm.ok_or(Error::NoRealm)?;
        let realm = realm.parse()?;
        let service = self.service.ok_or(Error::NoService)?;
        let scope = self.scope;
        Ok(BearerAuth {
            realm,
            service,
            scope,
        })
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("no bearer")]
    NoBearer,
    #[error("realm must exist")]
    NoRealm,
    #[error("realm is not url")]
    RealmIsNotUrl(#[from] url::ParseError),
    #[error("service must exist")]
    NoService,
}
