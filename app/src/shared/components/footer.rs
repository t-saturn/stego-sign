use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::{Heart, Shield};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-[#eff1f5] text-gray-400 border-t border-gray-800">
            <div class="max-w-6xl mx-auto px-6 py-12">

                // -- top section
                <div class="grid grid-cols-1 md:grid-cols-3 gap-10 mb-10">

                    // -- brand
                    <div class="flex flex-col gap-4">
                        <div class="flex items-center gap-2">
                            <img src="/logo.png" alt="StegoSign" class="w-8 h-8 object-contain"/>
                            <span class="font-display font-bold text-white text-lg">"StegoSign"</span>
                        </div>
                        <p class="text-sm leading-relaxed">
                            "Cryptographic document integrity using steganography. "
                            "Built with Rust, Axum, Leptos and PostgreSQL."
                        </p>
                        <div class="flex items-center gap-1 text-xs text-gray-500">
                            <Shield size=12 />
                            "Ed25519 + SHA-256 signatures"
                        </div>
                    </div>

                    // -- links
                    <div class="flex flex-col gap-3">
                        <h4 class="text-white font-semibold text-sm mb-1">"Navigation"</h4>
                        <FooterLink href="/"          label="Home"/>
                        <FooterLink href="/sign"      label="Sign Document"/>
                        <FooterLink href="/verify"    label="Verify Integrity"/>
                        <FooterLink href="/documents" label="Document Registry"/>
                    </div>

                    // -- tech stack
                    <div class="flex flex-col gap-3">
                        <h4 class="text-white font-semibold text-sm mb-1">"Stack"</h4>
                        <TechItem label="Rust + Axum"       desc="Backend API"/>
                        <TechItem label="Leptos + WASM"     desc="Frontend"/>
                        <TechItem label="PostgreSQL 17"     desc="Database"/>
                        <TechItem label="MinIO AIStor"      desc="Object Storage"/>
                    </div>
                </div>

                // -- divider
                <div class="border-t border-gray-800 pt-6 flex flex-col sm:flex-row items-center justify-between gap-4">
                    <p class="text-xs text-gray-600">
                        "© 2025 StegoSign — IS-444 Seguridad Informática · UNSCH"
                    </p>
                    <div class="flex items-center gap-1 text-xs text-gray-600">
                        "Built with"
                        <Heart size=12 color="#d20f39" />
                        "in Rust"
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn FooterLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="text-sm text-gray-400 hover:text-primary-400 transition-colors duration-200"
        >
            {label}
        </A>
    }
}

#[component]
fn TechItem(label: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between text-sm">
            <span class="text-gray-300 font-medium">{label}</span>
            <span class="text-xs text-gray-600 bg-[#e6e9ef] px-2 py-0.5 rounded">{desc}</span>
        </div>
    }
}
