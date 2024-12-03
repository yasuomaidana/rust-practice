#[macro_export]
macro_rules! add_mutable_struct_data_into_scope {
    ($scope:expr, ($($data:expr),+)) => {
        $scope
            $(.app_data($data.clone()))+
    };
    ($scope:expr, $data:expr) => {
        $scope.app_data($data.clone())
    };
}

// #[cfg(test)]
// mod tests {
//     use actix_web::web;
//     use std::sync::Mutex;
// 
//     #[allow(unused)]
//     struct StringData {
//         pub message: Mutex<String>,
//     }
// 
//     #[test]
//     fn test_add_into_scope_single_annotated() {
//         let scope = web::scope("");
//         let mut data = web::Data::new(StringData {
//             message: "Hello world!".to_string().into(),
//         });
// 
//         let _scope = add_mutable_struct_data_into_scope!(scope, data);
//     }
// 
//     #[test]
//     fn test_add_into_scope_multiple_annotated() {
//         let scope = web::scope("");
//         let mut data = web::Data::new(StringData {
//             message: "Hello world!".to_string().into(),
//         });
//         let mut data_2 = web::Data::new(StringData {
//             message: "Hello world!".to_string().into(),
//         });
//         let _scope = add_mutable_struct_data_into_scope!(scope, (data, data_2));
//     }
// }
