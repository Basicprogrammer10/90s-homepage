use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

use afire::{Method, Response, Server};

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

        dbg!(html_cache.clone());

        Self {
            data_dir: dir,
            serve_path: "/".to_owned() + "**",
            page_cache: html_cache,
        }
    }

    pub fn attach(self, server: &mut Server<App>) {
        server.route(Method::GET, &self.serve_path.to_owned(), move |req| {
            let path = safe_path(
                req.path
                    .strip_prefix(&self.serve_path)
                    .unwrap_or(&req.path)
                    .to_owned(),
            )
            .strip_prefix('/')
            .unwrap_or_default()
            .to_owned();

            let path = self
                .page_cache
                .get(&path)
                .map(|x| x.to_path_buf())
                .unwrap_or_else(|| self.data_dir.join(path));

            let stream = match File::open(&path) {
                Ok(file) => file,
                Err(_) => {
                    return Response::new()
                        .status(404)
                        .text(format!("Not Found: {path:?}"))
                }
            };
            Response::new().stream(stream)
        });
    }
}

// dont look too closely
fn safe_path(mut path: String) -> String {
    path = path.replace('\\', "/");
    while path.contains("/..") {
        path = path.replace("/..", "");
    }
    path
}
