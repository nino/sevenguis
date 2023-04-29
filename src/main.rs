mod memory_game;

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
    import { HorizontalBox, LineEdit } from "std-widgets.slint";

    export component TempConv inherits Window {
        in-out property <float> celsius: 0.0;
        in-out property <float> fahrenheit: 0.0;
        callback update_fahrenheit(string);
        callback update_celsius(string);
        HorizontalBox {
            celsius_field := LineEdit {
                text: root.celsius;
                edited => {
                    update_celsius(self.text);
                    fahrenheit_field.text = root.fahrenheit;
                }
            }
            Text { text: "Celsius = "; }
            fahrenheit_field := LineEdit {
                text: root.fahrenheit;
                edited => {
                    update_fahrenheit(self.text);
                    celsius_field.text = root.celsius;
                }
            }
            Text { text: "Fahrenheit"; }
        }
    }
}

fn make_temp_conv() -> Result<TempConv, slint::PlatformError> {
    let temp_conv = TempConv::new()?;

    {
        let temp_conv_handle = temp_conv.as_weak();
        temp_conv.on_update_celsius(move |new_text| {
            let temp_conv = temp_conv_handle.unwrap();
            if let Ok(celsius) = new_text.parse() {
                temp_conv.set_celsius(celsius);
                temp_conv.set_fahrenheit(celsius * 9.0 / 5.0 + 32.0);
            }
        });
    }

    {
        let temp_conv_handle = temp_conv.as_weak();
        temp_conv.on_update_fahrenheit(move |new_text| {
            let temp_conv = temp_conv_handle.unwrap();
            if let Ok(fahrenheit) = new_text.parse() {
                temp_conv.set_fahrenheit(fahrenheit);
                temp_conv.set_celsius((fahrenheit - 32.0) * 5.0 / 9.0);
            }
        });
    }

    Ok(temp_conv)
}

slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component MainWindow inherits Window  {
        callback make_counter <=> counter_button.clicked;
        callback make_temp_conv <=> temp_conv_button.clicked;
        callback make_memory_game <=> memory_game_button.clicked;
        VerticalBox {
            counter_button := Button {
                text: "Open counter";
            }
            temp_conv_button := Button {
                text: "Open temperature converter";
            }
            memory_game_button := Button {
                text: "Open memory game";
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
    let temp_conv = make_temp_conv()?;
    let game = memory_game::make_memory_game()?;

    main_window.on_make_counter(move || {
        counter.show().unwrap();
    });

    main_window.on_make_temp_conv(move || {
        temp_conv.show().unwrap();
    });

    main_window.on_make_memory_game(move || {
        game.show().unwrap();
    });

    main_window.run()
}
