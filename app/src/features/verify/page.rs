use leptos::prelude::*;

#[component]
pub fn VerifyPage() -> impl IntoView {
    view! {
        <div class="py-10">
            <h1 class="text-3xl font-bold text-primary mb-2">"Verify Document"</h1>
            <p class="text-slate-400">"Upload a signed file to verify its integrity."</p>
        </div>
    }
}
