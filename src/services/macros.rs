#[macro_export]
macro_rules! print_err {
    ($err: ident) => {
        log::error!(
            "error {:?} in thread {:?}, {}:{}:{}",
            $err,
            std::thread::current().name().unwrap_or_default(),
            file!(),
            line!(),
            column!()
        );
    };
}

#[macro_export]
macro_rules! catch_err {
    ($e: expr) => {
        match $e {
            Ok(s) => s,
            Err(e) => {
                $crate::print_err!(e);
                return Err(e);
            }
        }
    };
}

#[macro_export]
macro_rules! yields {
    ($($b: tt)*) => {
        async {$($b)*;Ok(())}.await?;
    };
}

#[macro_export]
macro_rules! tryopt {
    () => {{}};
    ($e:expr) => {
        match $e {
            Some(s) => s,
            None => {
                return Err(anyhow::anyhow!(format!(
                    "returned None on file {}:{}, {}",
                    file!(),
                    line!(),
                    module_path!(),
                )))
            }
        }
    };
}
