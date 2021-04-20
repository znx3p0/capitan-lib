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
macro_rules! abort_if_err {
    (
        services: $s:ident;
        run: $run: expr;
        current: $current: expr;
        channel: $channel: ident;
        id: $id: ident;
    ) => {
        if let Err(e) = $run {
            catch_err!($current.abort(e).await);
            let alive = {
                let mut $s = $s.write().await;
                $s.remove(&$id);
                $s.len() == 0
            };
            $channel.send(alive).ok();
        }
    };
}

#[macro_export]
macro_rules! ignore_print_result {
    ($e:expr, $s: ident, $p: ident) => {
        log::info!("error {:?}", &$s);
        match $e {
            Ok(_) => log::info!("successfully catched sid {}", $p),
            Err(e) => {
                print_err!(e);
                break e;
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
                return Err(anyhow::anyhow!(
                    "returned None on file {}:{}, {}",
                    file!(),
                    line!(),
                    module_path!(),
                ))
            }
        }
    };
}
