use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="relative min-h-screen flex flex-col">

            // -- hero section
            <section class="flex-1 flex flex-col items-center justify-center text-center px-4 py-24 relative overflow-hidden">

                // -- background decoration
                <div class="absolute inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute -top-40 -right-40 w-96 h-96 bg-primary-500/10 rounded-full blur-3xl"></div>
                    <div class="absolute -bottom-40 -left-40 w-96 h-96 bg-accent/10 rounded-full blur-3xl"></div>
                </div>

                // -- badge
                <div class="relative inline-flex items-center gap-2 bg-primary-50 border border-primary-200 text-primary-600 text-sm font-semibold px-4 py-2 rounded-full mb-8">
                    <span class="w-2 h-2 bg-primary-500 rounded-full animate-pulse"></span>
                    "Document Integrity System"
                </div>

                // -- headline
                <h1 class="relative text-5xl md:text-7xl font-display font-bold text-navy mb-6 leading-tight">
                    "Protect Your"
                    <br/>
                    <span class="bg-gradient-to-r from-primary-500 via-primary-600 to-accent bg-clip-text text-transparent">
                        "Documents"
                    </span>
                </h1>

                // -- subtitle
                <p class="relative text-lg md:text-xl text-gray-500 max-w-2xl mb-12 leading-relaxed">
                    "Embed cryptographic signatures using steganography. "
                    "Detect tampering instantly. "
                    "Verify authenticity with forensic-grade analysis."
                </p>

                // -- cta buttons
                <div class="relative flex flex-col sm:flex-row gap-4 justify-center">
                    <A href="/sign" attr:class="btn-primary">
                        "Sign a Document"
                    </A>
                    <A href="/verify" attr:class="btn-secondary">
                        "Verify Integrity"
                    </A>
                </div>
            </section>

            // -- features section
            <section class="py-20 px-4 bg-gray-50">
                <div class="max-w-6xl mx-auto">

                    <div class="text-center mb-16">
                        <span class="section-label justify-center">"How it works"</span>
                        <h2 class="section-title">"Three steps to trust"</h2>
                        <p class="section-subtitle">
                            "A reproducible SRE-grade pipeline for document authenticity"
                        </p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <FeatureCard
                            step="01"
                            icon="🔏"
                            title="Sign"
                            description="Upload any file. A SHA-256 hash and Ed25519 signature are embedded invisibly using steganography."
                            href="/sign"
                            cta="Sign a document"
                        />
                        <FeatureCard
                            step="02"
                            icon="🔍"
                            title="Verify"
                            description="Upload a signed file. The pipeline extracts the payload, verifies the signature and cross-checks the registry."
                            href="/verify"
                            cta="Verify a document"
                        />
                        <FeatureCard
                            step="03"
                            icon="📋"
                            title="Audit"
                            description="Every verification is logged. Access the full forensic history of any document at any time."
                            href="/documents"
                            cta="View documents"
                        />
                    </div>
                </div>
            </section>

            // -- status section
            <section class="py-16 px-4 bg-white border-t border-gray-100">
                <div class="max-w-6xl mx-auto">
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
                        <StatusStat label="Documents Signed"   value="—" />
                        <StatusStat label="Verifications"      value="—" />
                        <StatusStat label="Tampered Detected"  value="—" />
                        <StatusStat label="Storage Vaults"     value="3"  />
                    </div>
                </div>
            </section>

        </div>
    }
}

// -- feature card component
#[component]
fn FeatureCard(
    step: &'static str,
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    href: &'static str,
    cta: &'static str,
) -> impl IntoView {
    view! {
        <div class="card card-hover p-8 flex flex-col gap-4">
            <div class="flex items-center justify-between">
                <span class="text-4xl">{icon}</span>
                <span class="text-xs font-bold text-gray-300 font-display">{step}</span>
            </div>
            <h3 class="text-xl font-display font-bold text-navy">{title}</h3>
            <p class="text-gray-500 text-sm leading-relaxed flex-1">{description}</p>
            <a
                href=href
                class="text-sm font-semibold text-primary-500 hover:text-primary-600 flex items-center gap-1 transition-colors group"
            >
                {cta}
                <span class="group-hover:translate-x-1 transition-transform duration-200">"→"</span>
            </a>
        </div>
    }
}

// -- stat component
#[component]
fn StatusStat(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-1">
            <span class="text-3xl font-display font-bold text-navy">{value}</span>
            <span class="text-sm text-gray-500">{label}</span>
        </div>
    }
}
