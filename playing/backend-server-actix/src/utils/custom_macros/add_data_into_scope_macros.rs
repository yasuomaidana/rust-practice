#[macro_export]
macro_rules! add_struct_data_into_scope {
    ($scope:expr, ($($data:expr),+)) => {
        $scope
            $(.app_data(web::Data::new($data)))+
    };
    ($scope:expr, $data:expr) => {
        $scope.app_data(web::Data::new($data))
    };
}

#[cfg(test)]
mod tests {
    use actix_web::web;

    #[allow(unused)]
    struct StringData {
        pub message: String,
    }

    #[test]
    fn test_add_into_scope_single_annotated() {
        let scope = web::scope("");
        let data = StringData {
            message: "Hello world!".to_string(),
        };
        let _scope = add_struct_data_into_scope!(scope, data);
    }

    #[test]
    fn test_add_into_scope_multiple_annotated() {
        let scope = web::scope("");
        let data = StringData {
            message: "Hello world!".to_string(),
        };
        let data_2 = StringData {
            message: "Hello world!".to_string(),
        };
        let _scope = add_struct_data_into_scope!(scope, (data, data_2));
    }
}
