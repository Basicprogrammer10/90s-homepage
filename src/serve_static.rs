use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

use afire::{extension::serve_static::TYPES, Content, Method, Response, Server};

use crate::app::App;

pub struct ServeStatic {
    data_dir: PathBuf,
    serve_path: String,
    // maps path names to file
    // EX: /about -> /about.html
    page_cache: HashMap<String, PathBuf>,
}

impl ServeStatic {
    pub fn new(dir: impl AsRef<Path>) -> Self {
        let dir = dir.as_ref().to_path_buf();
        let mut html_cache = HashMap::new();
        html_cache.insert("".to_owned(), dir.join("index.html"));

        for i in fs::read_dir(&dir).unwrap().map(Result::unwrap) {
            let path = i.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if let Some(i) = name.strip_suffix(".html") {
                html_cache.insert(i.to_owned(), path);
            }
        }

        Self {
            data_dir: dir,
            serve_path: "**".to_owned(),
            page_cache: html_cache,
        }
    }

    pub fn attach(self, server: &mut Server<App>) {
        let serve_path = self.serve_path.to_owned();
        server.route(Method::GET, serve_path, move |req| {
            let path = safe_path(req.path.strip_prefix(&self.serve_path).unwrap_or(&req.path))
                .strip_prefix('/')
                .unwrap_or_default()
                .to_owned();

            let path = self
                .page_cache
                .get(&path)
                .map(|x| x.to_path_buf())
                .unwrap_or_else(|| self.data_dir.join(path));

            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            let content_type = get_type(&file_name, &TYPES);

            let file = match File::open(&path) {
                Ok(file) => file,
                Err(_) => {
                    return Response::new()
                        .status(404)
                        .text(format!("Not Found: {path:?}"))
                }
            };

            let mut res = Response::new();
            if let Ok(i) = file.metadata() {
                res.headers.add("Content-Length", i.len().to_string());
            }

            res.stream(file).content(Content::Custom(
                content_type.unwrap_or("application/octet-stream"),
            ))
        });
    }
}

pub fn safe_path(path: &str) -> Cow<'_, str> {
    if !path.contains("..") {
        return Cow::Borrowed(path);
    }

    let mut out = Vec::new();
    for i in path.split(['/', '\\']) {
        match i {
            ".." => {
                out.pop();
            }
            _ => out.push(i),
        }
    }

    Cow::Owned(out.join("/"))
}

fn get_type<'a>(path: &str, types: &'a [(&str, &str)]) -> Option<&'a str> {
    let ext = path.rsplit('.').next()?;
    Some(types.iter().find(|x| x.0 == ext)?.1)
}
