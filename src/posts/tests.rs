// TODO: Look into using a testing database, in memory postgreSQL?
// TODO: Make sure to fill the database with test data.

#[cfg(test)]
mod tests {
    use crate::helpers::test_app_factory;
    use crate::posts::models::JsonInputPost;
    use crate::posts::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_all_posts() {
        let app = test::init_service(
            App::new()
                .configure(test_app_factory)
                .service(get_all_posts),
        )
        .await;
        let req = test::TestRequest::get().uri("/get/posts").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_single_post() {
        let app = test::init_service(
            App::new()
                .configure(test_app_factory)
                .service(get_single_post),
        )
        .await;
        let req = test::TestRequest::get().uri("/get/post/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_all_posts_by_user() {
        let app = test::init_service(
            App::new()
                .configure(test_app_factory)
                .service(get_all_posts_by_user),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/get/posts/user/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_post_new_post() {
        let app = test::init_service(
            App::new()
                .configure(test_app_factory)
                .service(post_new_post),
        )
        .await;
        let payload = JsonInputPost {
            title: "Test".to_string(),
            body: "Test".to_string(),
            user_id: 4,
        };
        let req = test::TestRequest::post()
            .set_json(&payload)
            .uri("/post/post")
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_delete_single_post() {
        let app = test::init_service(
            App::new()
                .configure(test_app_factory)
                .service(delete_single_post),
        )
        .await;
        let req = test::TestRequest::post().uri("/delete/post/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
