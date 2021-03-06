// use serde::Serialize;
// use serde_json;
// use futures::future::{ready, Ready};
// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

// #[derive(Serialize)]
// pub struct Directory {
//     root: String,
//     access: u8,
//     directories: Vec<String>,
//     date_created: DateTime<Utc>,
//     date_updated: DateTime<Utc>,
//     files: Vec<File>
// }

// impl Responder for Directory {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }

// fn new_directory() -> Directory {
//     let mut dirs = Vec::new();
//     let mut fils = Vec::new();

//     dirs.push("aPlace".to_owned());
//     fils.push(file_factory());

//     Directory {
//         root: "/".to_owned(),
//         access: 4,
//         directories: dirs,
//         date_created: Utc::now(),
//         date_updated: Utc::now(),
//         files: fils
//     }
// }

// #[derive(Serialize)]
// pub struct File {
//     name: String,
//     access: u8,
//     date_created: DateTime<Utc>,
//     date_updated: DateTime<Utc>,
//     metadata: String
// }

// impl Responder for File {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }

// fn file_factory() -> File {
//     File {
//         name: "ExampleFile.txt".to_owned(),
//         access: 4,
//         date_created: Utc::now(),
//         date_updated: Utc::now(),
//         metadata: "No Metadata".to_owned()
//     }
// }

// pub fn get_fs_structure() -> Directory {
//     new_directory()
// }
