use crate::model::{Info, Sunny, SunnyData};
use crate::request;
use crate::utils::Utils;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/get_sunny_list")]
async fn get_sunny_list(info: web::Query<Info>) -> impl Responder {
    let mut request = request::Request::new();
    let username = info.username.clone();
    let password = info.password.clone();
    let sunny_list = request.get_result(username, password).await;
    let mut list: Vec<Sunny> = Vec::new();
    for sunny in sunny_list {
        let data_list = sunny.split("-").collect::<Vec<&str>>();
        let time = data_list[0].to_string();
        let period = data_list[1].to_string();
        let meters = data_list[2].to_string();
        let speed = data_list[3].to_string();
        let ok = {
            if data_list[3] == "true" {
                true
            } else {
                false
            }
        };
        let sunny = Sunny {
            time,
            period,
            meters,
            speed,
            ok,
        };
        list.push(sunny);
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&list).unwrap())
}

#[get("/get_sunny_info")]
async fn get_sunny_info(info: web::Query<Info>) -> impl Responder {
    let mut requests = request::Request::new();
    let username = info.username.clone();
    let password = info.password.clone();
    let utils = Utils::new(requests.get_result(username, password).await);
    let sunny_data = SunnyData {
        count: utils.get_count().count,
        am_count: utils.get_count().am_count,
        average_speed: utils.get_average_speed(),
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&sunny_data).unwrap())
}
