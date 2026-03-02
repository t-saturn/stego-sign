use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::{
    CircleAlert, CircleCheck, CircleX, FileLock, FileSearch, FileStack, LoaderCircle, RotateCcw,
};
use wasm_bindgen_futures::spawn_local;

use super::api::{fetch_registry, Registry};
use super::components::document_row::{SignedDocRow, VerificationRow};

#[derive(Clone, PartialEq)]
enum Tab {
    Signed,
    Verifications,
}

#[derive(Clone)]
enum DocsState {
    Loading,
    Loaded(Registry),
    Error(String),
}

#[component]
pub fn DocumentsPage() -> impl IntoView {
    let state = RwSignal::new(DocsState::Loading);
    let active_tab = RwSignal::new(Tab::Signed);

    let load = move || {
        state.set(DocsState::Loading);
        #[cfg(feature = "hydrate")]
        {
            spawn_local(async move {
                match fetch_registry().await {
                    Ok(r) => state.set(DocsState::Loaded(r)),
                    Err(e) => state.set(DocsState::Error(e)),
                }
            });
        }
    };

    load();

    view! {
        <div class="max-w-5xl mx-auto px-4 py-12">

            // -- header
            <div class="flex items-start justify-between mb-8">
                <div>
                    <p class="text-sm font-semibold text-primary-600 mb-1">"Audit Registry"</p>
                    <h1 class="text-3xl font-display font-semibold text-primary-600 mb-2">
                        "Documents"
                    </h1>
                    <p class="text-gray-500 text-sm max-w-lg">
                        "Complete forensic history — signed documents and verification log."
                    </p>
                </div>
                <button
                    class="inline-flex items-center gap-2 px-4 py-2.5 text-sm font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-xl hover:bg-primary-50 transition-all duration-200"
                    on:click=move |_| load()
                >
                    <RotateCcw size=15 color="#d20f39" />
                    "Refresh"
                </button>
            </div>

            // -- tabs (solo cuando hay datos)
            {move || {
                if let DocsState::Loaded(reg) = state.get() {
                    let signed_count = reg.signed.len();
                    let verify_count = reg.verifications.len();
                    let tab          = active_tab.get();
                    view! {
                        <div class="flex gap-1 p-1 bg-gray-100 rounded-xl mb-6 w-fit">
                            <button
                                class=move || format!(
                                    "inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg transition-all duration-200 {}",
                                    if active_tab.get() == Tab::Signed {
                                        "bg-white text-primary-600 shadow-sm"
                                    } else {
                                        "text-gray-500 hover:text-gray-700"
                                    }
                                )
                                on:click=move |_| active_tab.set(Tab::Signed)
                            >
                                <FileLock size=15 color=if tab == Tab::Signed { "#d20f39" } else { "#9ca3af" } />
                                {format!("Signed ({})", signed_count)}
                            </button>
                            <button
                                class=move || format!(
                                    "inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg transition-all duration-200 {}",
                                    if active_tab.get() == Tab::Verifications {
                                        "bg-white text-primary-600 shadow-sm"
                                    } else {
                                        "text-gray-500 hover:text-gray-700"
                                    }
                                )
                                on:click=move |_| active_tab.set(Tab::Verifications)
                            >
                                <FileSearch size=15 color=if tab == Tab::Verifications { "#d20f39" } else { "#9ca3af" } />
                                {format!("Verifications ({})", verify_count)}
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // -- contenido principal
            {move || match state.get() {
                DocsState::Loading => view! {
                    <div class="flex flex-col items-center justify-center py-20 gap-4 text-gray-400">
                        <span class="animate-spin">
                            <LoaderCircle size=36 color="#d20f39" />
                        </span>
                        <p class="text-sm font-medium">"Loading…"</p>
                    </div>
                }.into_any(),

                DocsState::Error(e) => view! {
                    <div class="flex flex-col items-center justify-center py-20 gap-4 text-center">
                        <div class="p-4 bg-red-50 rounded-2xl">
                            <CircleX size=36 color="#dc2626" />
                        </div>
                        <div>
                            <p class="text-sm font-semibold text-red-600 mb-1">"Failed to load"</p>
                            <p class="text-xs text-gray-400">{e}</p>
                        </div>
                        <button
                            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold text-primary-600 border-2 border-primary-500 rounded-xl hover:bg-primary-50 transition-all"
                            on:click=move |_| load()
                        >
                            <RotateCcw size=14 color="#d20f39" />
                            "Retry"
                        </button>
                    </div>
                }.into_any(),

                DocsState::Loaded(reg) => match active_tab.get() {

                    Tab::Signed => {
                        if reg.signed.is_empty() {
                            view! {
                                <EmptyState
                                    label="No signed documents yet"
                                    hint="Sign your first document to populate the registry."
                                    href="/sign"
                                    cta="Sign a Document"
                                />
                            }.into_any()
                        } else {
                            let total    = reg.signed.len();
                            let valid    = reg.signed.iter().filter(|d| d.status == "VALID").count();
                            let tampered = reg.signed.iter().filter(|d| d.status == "TAMPERED").count();
                            let other    = total - valid - tampered;

                            view! {
                                <div class="flex flex-col gap-3">
                                    // -- chips resumen
                                    <div class="flex flex-wrap gap-2 mb-2">
                                        <Chip
                                            icon=view! { <FileStack size=13 color="#6b7280" /> }.into_any()
                                            label=format!("{} total", total)
                                            class="bg-gray-50 border-gray-200 text-gray-600"
                                        />
                                        <Chip
                                            icon=view! { <CircleCheck size=13 color="#16a34a" /> }.into_any()
                                            label=format!("{} valid", valid)
                                            class="bg-green-50 border-green-200 text-green-700"
                                        />
                                        {(tampered > 0).then(|| view! {
                                            <Chip
                                                icon=view! { <CircleX size=13 color="#dc2626" /> }.into_any()
                                                label=format!("{} tampered", tampered)
                                                class="bg-red-50 border-red-200 text-red-700"
                                            />
                                        })}
                                        {(other > 0).then(|| view! {
                                            <Chip
                                                icon=view! { <CircleAlert size=13 color="#d97706" /> }.into_any()
                                                label=format!("{} other", other)
                                                class="bg-yellow-50 border-yellow-200 text-yellow-700"
                                            />
                                        })}
                                    </div>

                                    // -- filas
                                    {reg.signed.into_iter().map(|doc| view! {
                                        <SignedDocRow doc=doc />
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }

                    Tab::Verifications => {
                        if reg.verifications.is_empty() {
                            view! {
                                <EmptyState
                                    label="No verifications yet"
                                    hint="Verify a signed document to see the forensic history here."
                                    href="/verify"
                                    cta="Verify a Document"
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div class="flex flex-col gap-3">
                                    {reg.verifications.into_iter().map(|entry| view! {
                                        <VerificationRow entry=entry />
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                }
            }}
        </div>
    }
}

// -- chip de resumen
#[component]
fn Chip(icon: AnyView, label: String, class: &'static str) -> impl IntoView {
    view! {
        <div class=format!(
            "inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold rounded-lg border {}",
            class
        )>
            {icon}
            {label}
        </div>
    }
}

// -- estado vacio
#[component]
fn EmptyState(
    label: &'static str,
    hint: &'static str,
    href: &'static str,
    cta: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-20 gap-6 text-center">
            <div class="p-6 bg-gray-50 rounded-2xl">
                <FileStack size=48 color="#d1d5db" />
            </div>
            <div>
                <h3 class="text-lg font-display font-bold text-navy mb-2">{label}</h3>
                <p class="text-sm text-gray-400 max-w-xs">{hint}</p>
            </div>
            <A
                href=href
                attr:class="inline-flex items-center gap-2 px-6 py-3 text-sm font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg transform hover:scale-[1.02] transition-all duration-300"
            >
                {cta}
            </A>
        </div>
    }
}
