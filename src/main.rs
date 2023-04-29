slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component CounterWindow inherits Window  {
        in-out property <int> count: 0;
        callback increment <=> incr_button.clicked;
        VerticalBox {
            incr_button := Button {
                text: "Increment";
            }
            Text { text: root.count; }
        }
    }
}

fn make_counter() -> Result<CounterWindow, slint::PlatformError> {
    let counter = CounterWindow::new()?;
    let counter_handle = counter.as_weak();
    counter.on_increment(move || {
        let counter = counter_handle.unwrap();
        counter.set_count(counter.get_count() + 1);
    });
    Ok(counter)
}

slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component MainWindow inherits Window  {
        callback make_counter <=> counter_button.clicked;
        VerticalBox {
            counter_button := Button {
                text: "Open counter";
            }
        }
    }
}

fn make_main_window() -> Result<MainWindow, slint::PlatformError> {
    let main_window = MainWindow::new()?;
    Ok(main_window)
}

fn main() -> Result<(), slint::PlatformError> {
    let main_window = make_main_window()?;
    let counter = make_counter()?;

    main_window.on_make_counter(move || {
        counter.show().unwrap();
    });

    main_window.run()
}
