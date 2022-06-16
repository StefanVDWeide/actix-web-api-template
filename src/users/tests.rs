#[cfg(test)]
mod tests {
    use crate::helpers::test_app_factory;
    use crate::users::{delete_user, get_all_users, get_user, post_user};
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_all_users() {
        let app = test::init_service(
            App::new()
                .configure(test_app_factory)
                .service(get_all_users),
        )
        .await;
        let req = test::TestRequest::get().uri("/get/users").to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_user() {
        let app =
            test::init_service(App::new().configure(test_app_factory).service(get_user)).await;
        let req = test::TestRequest::get().uri("/get/user/1").to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_post_user() {
        let app =
            test::init_service(App::new().configure(test_app_factory).service(post_user)).await;
        let req = test::TestRequest::post().uri("/post/users").to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_delete_user() {
        let app =
            test::init_service(App::new().configure(test_app_factory).service(delete_user)).await;
        let req = test::TestRequest::post().uri("/delete/user/1").to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_client_error());
    }
}
