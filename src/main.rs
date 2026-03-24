use std::rc::Rc;
slint::include_modules!();
use slint::Model;

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    // Inicjalizacja danych
    let mut numbers = vec![30, 80, 10, 50, 40, 90, 20, 70, 60, 5];
    ui.set_data(Rc::new(slint::VecModel::from(numbers.clone())).into());

    ui.on_start_sort(move || {
        let ui = ui_handle.unwrap();
        let model = Rc::new(slint::VecModel::from(numbers.clone()));
        ui.set_data(model.clone().into());

        // Używamy Timera, aby wizualizacja "szła" w czasie, nie blokując UI
        let mut i = 0;
        let mut j = 0;
        let timer = slint::Timer::default();

        timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_millis(200),
            move || {
                let n = model.row_count();

                if i < n {
                    if j < n - i - 1 {
                        let val_a = model.row_data(j).unwrap();
                        let val_b = model.row_data(j + 1).unwrap();

                        // Pokazujemy, które elementy porównujemy
                        ui.set_active_idx_a(j as i32);
                        ui.set_active_idx_b((j + 1) as i32);

                        if val_a > val_b {
                            model.set_row_data(j, val_b);
                            model.set_row_data(j + 1, val_a);
                        }
                        j += 1;
                    } else {
                        j = 0;
                        i += 1;
                    }
                } else {
                    // Koniec sortowania
                    ui.set_active_idx_a(-1);
                    ui.set_active_idx_b(-1);
                }
            },
        );

        // Uwaga: W prawdziwej aplikacji timer musiałby żyć dłużej niż to domknięcie
        // Tutaj dla uproszczenia pozwalamy mu działać w tle (leak).
        Box::leak(Box::new(timer));
    });

    ui.run()
}
