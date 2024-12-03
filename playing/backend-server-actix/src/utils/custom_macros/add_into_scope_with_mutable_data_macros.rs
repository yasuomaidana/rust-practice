#[macro_export]
macro_rules! add_into_scope_with_mutable_struct_data {
    
    ($scope:expr,$data:expr, $func:ident) =>{
        add_into_scope!(
            add_mutable_struct_data_into_scope!($scope, $data), 
            $func)
    };
    
    ($scope:expr, $data:expr, ($($func:ident),+)) => {
        add_into_scope!(add_mutable_struct_data_into_scope!($scope, $data), ($($func),+))
    };
    
}

// #[cfg(test)]
// mod tests {
//     
//     use actix_web::{get, web};
//     use actix_web::{HttpResponse, Responder};
//     use crate::{add_into_scope_with_mutable_struct_data, add_into_scope, add_mutable_struct_data_into_scope};
// 
//     #[allow(unused)]
//     struct StringData {
//         message: String,
//     }
// 
//     // pub async fn path_1() -> impl Responder {
//     //     let message = "Hello world!";
//     //     HttpResponse::Ok().body(message)
//     // }
//     // 
//     // pub async fn path_2() -> impl Responder {
//     //     let message = "Hello world!";
//     //     HttpResponse::Ok().body(message)
//     // }
// 
//     #[get("/")]
//     pub async fn path_3() -> impl Responder {
//         let message = "Hello world!";
//         HttpResponse::Ok().body(message)
//     }
// 
//     #[get("/2")]
//     pub async fn path_4() -> impl Responder {
//         let message = "Hello world!";
//         HttpResponse::Ok().body(message)
//     }
// 
//     #[test]
//     fn test_add_into_scope_with_inmutable_data() {
//         let scope = web::scope("");
//         let data = StringData {
//             message: "Hello world!".to_string(),
//         };
//         let _scope = add_into_scope_with_mutable_struct_data!(scope, data, path_3);
//     }
// 
//     #[test]
//     fn test_add_into_scope_with_multiple_inmutable_data() {
//         let scope = web::scope("");
//         let data = StringData {
//             message: "Hello world!".to_string(),
//         };
//         let data_2 = StringData {
//             message: "Hello world!".to_string(),
//         };
//         let scope = add_into_scope_with_mutable_struct_data!(scope, (data,data_2), path_3);
//     }
// 
//     #[test]
//     fn test_add_into_scope_with_multiple_inmutable_data_and_annotated_paths() {
//         let scope = web::scope("");
//         let data = StringData {
//             message: "Hello world!".to_string(),
//         };
//         let data_2 = StringData {
//             message: "Hello world!".to_string(),
//         };
//         let scope = add_into_scope_with_mutable_struct_data!(scope, (data,data_2), 
//             (path_3, path_4));
//     }
// 
//     #[test]
//     fn test_add_into_scope_with_multiple_annotated_paths() {
//         let scope = web::scope("");
//         let data = StringData {
//             message: "Hello world!".to_string(),
//         };
//         
//         let scope = add_into_scope_with_mutable_struct_data!(scope, data, 
//             (path_3, path_4));
//     }
//     
// }
