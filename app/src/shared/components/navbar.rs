use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let scrolled = RwSignal::new(false);

    view! {
        <nav class=move || format!(
            "fixed top-0 left-0 right-0 z-50 transition-all duration-300 px-6 py-4 {}",
            if scrolled.get() {
                "bg-white/95 backdrop-blur-md shadow-lg border-b border-primary-50"
            } else {
                "bg-white border-b border-gray-100"
            }
        )>
            <div class="max-w-6xl mx-auto flex items-center justify-between">

                // -- logo + brand
                <A href="/" attr:class="flex items-center gap-2 group no-underline">
                    <div class="w-8 h-8 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center shadow-md group-hover:shadow-primary-500/30 transition-all duration-300">
                        <span class="text-white text-sm font-bold">"S"</span>
                    </div>
                    <span class="font-display font-bold text-xl text-navy group-hover:text-primary-500 transition-colors duration-300">
                        "StegoSign"
                    </span>
                </A>

                // -- nav links
                <div class="flex items-center gap-1">
                    <NavLink href="/sign"          label="Sign"/>
                    <NavLink href="/verify"    label="Verify"/>
                    <NavLink href="/documents" label="Documents"/>
                </div>
            </div>
        </nav>
    }
}

// -- reusable nav link with active indicator
#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="relative px-4 py-2 text-sm font-medium text-gray-600 hover:text-primary-500 transition-colors duration-200 rounded-lg hover:bg-primary-50 group"
        >
            {label}
            // -- animated underline
            <span class="absolute bottom-1 left-4 right-4 h-0.5 bg-gradient-to-r from-primary-500 to-primary-400 scale-x-0 group-hover:scale-x-100 transition-transform duration-300 rounded-full"></span>
        </A>
    }
}
