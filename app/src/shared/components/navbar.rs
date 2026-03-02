use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::Menu;

#[component]
pub fn Navbar() -> impl IntoView {
    let scrolled = RwSignal::new(false);

    // -- scroll listener via window event
    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::prelude::*;
        use web_sys::window;

        let closure = Closure::wrap(Box::new(move || {
            let y = window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
            scrolled.set(y > 20.0);
        }) as Box<dyn Fn()>);

        if let Some(win) = window() {
            let _ =
                win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }

        closure.forget();
    }

    view! {
        <nav class=move || {
            let base = "fixed top-0 left-0 right-0 z-50 transition-all duration-500 px-6 py-3";
            if scrolled.get() {
                format!("{} bg-white/20 backdrop-blur-sm shadow-md border-b border-gray-200/50", base)
            } else {
                format!("{} bg-white border-b border-gray-100", base)
            }
        }>
            <div class="max-w-6xl mx-auto flex items-center justify-between">

                // -- logo
                <A href="/" attr:class="flex items-center gap-2 group no-underline">
                    <img
                        src="/logo.png"
                        alt="StegoSign"
                        class="w-8 h-8 object-contain group-hover:scale-110 transition-transform duration-300"
                    />
                    <span class="font-display font-bold text-xl text-navy group-hover:text-primary-500 transition-colors duration-300">
                        "StegoSign"
                    </span>
                </A>

                // -- desktop nav
                <div class="hidden sm:flex items-center gap-1">
                    <NavLink href="/"          label="Home"/>
                    <NavLink href="/sign"      label="Sign"/>
                    <NavLink href="/verify"    label="Verify"/>
                    <NavLink href="/documents" label="Documents"/>
                </div>

                // -- mobile menu icon (no-op for now)
                <button class="sm:hidden p-2 text-gray-500 hover:text-primary-500 transition-colors">
                    <Menu size=22 />
                </button>
            </div>
        </nav>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    let location = leptos_router::hooks::use_location();

    let is_active = move || {
        let path = location.pathname.get();
        if href == "/" {
            path == "/"
        } else {
            path.starts_with(href)
        }
    };

    view! {
        <A
            href=href
            attr:class=move || {
                if is_active() {
                    "relative px-4 py-2 text-sm font-medium text-primary-600 transition-colors duration-200 rounded-lg group"
                } else {
                    "relative px-4 py-2 text-sm font-medium text-gray-600 hover:text-primary-500 transition-colors duration-200 rounded-lg group"
                }
            }
        >
            {label}
            <span class=move || {
                if is_active() {
                    "absolute bottom-1 left-4 right-4 h-0.5 bg-gradient-to-r from-primary-500 to-primary-400 scale-x-100 rounded-full"
                } else {
                    "absolute bottom-1 left-4 right-4 h-0.5 bg-gradient-to-r from-primary-500 to-primary-400 scale-x-0 group-hover:scale-x-100 transition-transform duration-300 origin-left rounded-full"
                }
            }>
            </span>
        </A>
    }
}
