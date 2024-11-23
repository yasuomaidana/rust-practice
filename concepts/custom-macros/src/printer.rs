pub struct Printer {
    pub name: String,
    pub model: String,
    pub connection_type: String,
}

impl Printer {
    pub  fn new(name: &str, model: &str, connection_type: &str) -> Printer {
        Printer {
            name: name.to_string(),
            model: model.to_string(),
            connection_type: connection_type.to_string(),
        }
    }
}

#[macro_export]
macro_rules! create_printer {
    ($name:expr) => {
        $crate::printer::Printer::new($name, "HC-2000", "USB")
    };
    ($name:expr, $model:expr) => {
        $crate::printer::Printer::new($name, $model,"Ethernet")
    };
    ($name:expr, $model:expr, $connection_type:expr) => {
        $crate::printer::Printer::new($name, $model, $connection_type)
    };
}
