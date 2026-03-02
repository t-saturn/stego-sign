use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::{Github, Heart, Shield};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer style="background-color: #eff1f5;" class="text-gray-600 border-t border-gray-200">

            // -- animated gradient accent bar
            <div style="
                display: grid;
                grid-template-columns: repeat(14, 1fr);
                width: 100%;
                height: 8px;
                background-image: linear-gradient(
                    90deg,
                    #dc8a78, #dd7878, #ea76cb, #8839ef,
                    #d20f39, #e64553, #fe640b, #df8e1d,
                    #40a02b, #179299, #04a5e5, #209fb5,
                    #1e66f5, #7287fd, #dc8a78, #dd7878,
                    #ea76cb, #8839ef, #d20f39, #e64553,
                    #fe640b, #df8e1d, #40a02b, #179299,
                    #04a5e5, #209fb5, #1e66f5, #7287fd,
                    #dc8a78
                );
                background-size: 400% 100%;
                animation: gradient-slide 20s linear infinite forwards;
            ">
            </div>

            // -- inject keyframes
            <style>
                "@keyframes gradient-slide {
                    0%   { background-position: 0% 0%; }
                    100% { background-position: 100% 0%; }
                }"
            </style>

            <div class="max-w-6xl mx-auto px-6 py-12">

                // -- top section
                <div class="grid grid-cols-1 md:grid-cols-3 gap-10 mb-10">

                    // -- brand
                    <div class="flex flex-col gap-4">
                        <div class="flex items-center gap-2">
                            <img src="/logo.png" alt="StegoSign" class="w-8 h-8 object-contain"/>
                            <span class="font-display font-bold text-navy text-lg">"StegoSign"</span>
                        </div>
                        <p class="text-sm leading-relaxed text-gray-500">
                            "Cryptographic document integrity using steganography. "
                            "Built with Rust, Axum, Leptos and PostgreSQL."
                        </p>
                        <div class="flex items-center gap-1 text-xs text-gray-400">
                            <Shield size=12 />
                            "Ed25519 + SHA-256 signatures"
                        </div>
                        // -- github
                        <a
                            href="https://github.com/fn-9li9/stego-sign"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center gap-2 text-sm font-medium text-gray-500 hover:text-navy transition-colors duration-200 w-fit"
                        >
                            <Github size=16 />
                            "fn-9li9/stego-sign"
                        </a>
                    </div>

                    // -- navigation links
                    <div class="flex flex-col gap-3">
                        <h4 class="font-display font-semibold text-navy text-sm mb-1">"Navigation"</h4>
                        <FooterLink href="/"          label="Home"/>
                        <FooterLink href="/sign"      label="Sign Document"/>
                        <FooterLink href="/verify"    label="Verify Integrity"/>
                        <FooterLink href="/documents" label="Document Registry"/>
                    </div>

                    // -- tech stack
                    <div class="flex flex-col gap-3">
                        <h4 class="font-display font-semibold text-navy text-sm mb-1">"Stack"</h4>
                        <TechItem label="Rust + Axum"    desc="Backend API"/>
                        <TechItem label="Leptos + WASM"  desc="Frontend"/>
                        <TechItem label="PostgreSQL 17"  desc="Database"/>
                        <TechItem label="MinIO AIStor"   desc="Object Storage"/>
                    </div>
                </div>

                // -- divider + copyright
                <div class="border-t-2 border-primary-600 pt-6 flex flex-col sm:flex-row items-center justify-between gap-4">
                    <p class="text-xs text-primary-600 font-medium">
                        "© 2026 StegoSign — IS-444 Seguridad Informática · UNSCH"
                    </p>
                    <div class="flex items-center gap-1 text-xs text-gray-400">
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
            attr:class="text-sm text-gray-500 hover:text-primary-500 transition-colors duration-200"
        >
            {label}
        </A>
    }
}

#[component]
fn TechItem(label: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between text-sm">
            <span class="font-medium text-navy">{label}</span>
            <span
                class="text-xs px-2 py-0.5 rounded font-medium"
                style="color: #7287fd; background-color: #e6e9ef;"
            >
                {desc}
            </span>
        </div>
    }
}
