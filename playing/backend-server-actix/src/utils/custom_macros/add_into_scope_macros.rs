#[macro_export]
/// Add endpoints into a scope
///
/// # Examples
///
/// ```
/// pub async fn path_1() -> impl Responder {
///     let message = "Hello world!";
///     HttpResponse::Ok().body(message)
/// }
///
/// pub async fn path_2() -> impl Responder {
///     let message = "Hello world!";
///     HttpResponse::Ok().body(message)
/// }
///
/// let scope = web::scope("");
/// let _scope = add_into_scope!(scope, web::get, "/path_1", path_1);
/// let _expanded = add_into_scope!(scope, web::get, ("/path_1", "/path_2"), [path_1, path_2]);
/// ```
macro_rules! add_into_scope {

    ($scope:expr, $func:ident) =>{
        $scope
            .service($func)
    };
    ($scope:expr, ($($func:ident),+) ) => {
        $scope
        $(.service($func))+
    };

    ($scope:expr,$method:expr,$path:expr, $func:ident) => {
        $scope.route($path, $method().to($func))
    };

    ($scope:expr,$method:expr, ($( $path:expr ),+), ($( $func:ident ),+)) => {
    {
        $scope$(.route($path, web::get().to($func)))+
    }
};

}

#[cfg(test)]
mod tests {
    use actix_web::{get, web};
    use actix_web::{HttpResponse, Responder};

    pub async fn path_1() -> impl Responder {
        let message = "Hello world!";
        HttpResponse::Ok().body(message)
    }

    pub async fn path_2() -> impl Responder {
        let message = "Hello world!";
        HttpResponse::Ok().body(message)
    }

    #[get("/")]
    pub async fn path_3() -> impl Responder {
        let message = "Hello world!";
        HttpResponse::Ok().body(message)
    }

    #[get("/2")]
    pub async fn path_4() -> impl Responder {
        let message = "Hello world!";
        HttpResponse::Ok().body(message)
    }

    #[test]
    fn test_add_into_scope_single() {
        let scope = web::scope("");
        let _scope = add_into_scope!(scope, web::get, "/path_1", path_1);
    }

    #[test]
    fn test_add_into_scope_multiple() {
        let scope = web::scope("");
        // this works for ($scope:expr,$method:expr, ($( $path:expr ),+), [$( $func:ident ),+])
        // let _expanded = add_into_scope!(scope, web::get, ("/path_1", "/path_2"), [path_1, path_2]);
        let _expanded = add_into_scope!(scope, web::get, ("/path_1", "/path_2"), (path_1, path_2));
    }

    #[test]
    fn test_add_into_scope_single_annotated() {
        let scope = web::scope("");
        let _scope = add_into_scope!(scope, path_3);
    }

    #[test]
    fn test_add_into_scope_multiple_annotated() {
        let scope = web::scope("");
        let _scope = add_into_scope!(scope, (path_3, path_4));
    }
}
