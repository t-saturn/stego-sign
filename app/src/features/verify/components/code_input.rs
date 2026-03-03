use leptos::prelude::*;

/// value: señal de 6 chars sin guion (ej "ABCDEF")
/// al submit se formatea como "ABC-DEF"
#[component]
pub fn CodeInput(value: RwSignal<String>) -> impl IntoView {
    // -- chars individuales, siempre 6 posiciones
    let chars = move || {
        let s = value.get();
        let mut v = ['_'; 6];
        for (i, c) in s.chars().enumerate().take(6) {
            v[i] = c;
        }
        v
    };

    let on_input = move |ev: web_sys::Event| {
        let raw = event_target_value(&ev);
        // -- filtra solo alfanumericos, uppercase, max 6 chars
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_uppercase())
            .take(6)
            .collect();
        value.set(cleaned);
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        // -- permite: backspace, tab, flechas, letras, numeros
        let key = ev.key();
        if key == "Backspace" || key == "Tab" || key == "ArrowLeft" || key == "ArrowRight" {
            return;
        }
        let c = key.chars().next().unwrap_or(' ');
        if !c.is_alphanumeric() {
            ev.prevent_default();
        }
    };

    view! {
        <div class="flex flex-col gap-3">
            // -- visual slots: 3 cajas + guion + 3 cajas
            <div class="flex items-center justify-center gap-2 pointer-events-none select-none">
                // -- primeros 3
                <div class="flex gap-2">
                    {move || {
                        let c = chars();
                        (0..3usize).map(|i| {
                            let ch = c[i];
                            let filled = ch != '_';
                            view! {
                                <div class=move || format!(
                                    "w-11 h-12 flex items-center justify-center rounded-xl border-2 font-mono font-bold text-lg transition-all duration-200 {}",
                                    if filled {
                                        "border-primary-500 bg-primary-50 text-primary-600"
                                    } else {
                                        "border-gray-200 bg-gray-50 text-transparent"
                                    }
                                )>
                                    {if filled { ch.to_string() } else { "·".to_string() }}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>

                // -- guion separador
                <div class="w-4 h-0.5 bg-gray-300 rounded-full"></div>

                // -- últimos 3
                <div class="flex gap-2">
                    {move || {
                        let c = chars();
                        (3..6usize).map(|i| {
                            let ch = c[i];
                            let filled = ch != '_';
                            view! {
                                <div class=move || format!(
                                    "w-11 h-12 flex items-center justify-center rounded-xl border-2 font-mono font-bold text-lg transition-all duration-200 {}",
                                    if filled {
                                        "border-primary-500 bg-primary-50 text-primary-600"
                                    } else {
                                        "border-gray-200 bg-gray-50 text-transparent"
                                    }
                                )>
                                    {if filled { ch.to_string() } else { "·".to_string() }}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>

            // -- input real debajo de los slots
            <input
                type="text"
                maxlength="6"
                placeholder="ABCDEF"
                class="w-full px-4 py-2.5 text-sm font-mono uppercase text-center border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary-300 focus:border-primary-500 transition-all duration-200 bg-white tracking-widest text-gray-500 placeholder:text-gray-300"
                on:input=on_input
                on:keydown=on_keydown
                prop:value=move || value.get()
            />
        </div>
    }
}

/// Formatea 6 chars sin guion → "ABC-DEF"
pub fn format_code(raw: &str) -> String {
    let clean: String = raw
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .take(6)
        .collect();

    if clean.len() == 6 {
        format!("{}-{}", &clean[..3], &clean[3..])
    } else {
        clean
    }
}
