use leptos::prelude::*;
use lucide_leptos::{ArrowRight, CircleAlert, FileSearch, LoaderCircle, RotateCcw};
use wasm_bindgen_futures::spawn_local;

use super::api::{verify_document, VerifyData};
use super::components::{
    drop_zone::VerifyDropZone, result_card::VerifyResultCard, steps_flow::VerifyStepsFlow,
    steps_modal::VerifyStepsModal,
};

#[derive(Clone)]
enum VerifyState {
    Idle,
    Loading,
    Success(VerifyData),
    Error(String),
}

#[component]
pub fn VerifyPage() -> impl IntoView {
    let file = RwSignal::new(None::<web_sys::File>);
    let state = RwSignal::new(VerifyState::Idle);
    let show_modal = RwSignal::new(false);

    let on_reset = move || {
        file.set(None);
        state.set(VerifyState::Idle);
    };

    let on_clear = Callback::new(move |_| {
        file.set(None);
        state.set(VerifyState::Idle);
    });

    let on_submit = move |_| {
        let Some(f) = file.get() else {
            state.set(VerifyState::Error("Please select a file first".to_string()));
            return;
        };
        state.set(VerifyState::Loading);
        spawn_local(async move {
            match verify_document(f).await {
                Ok(data) => state.set(VerifyState::Success(data)),
                Err(e) => state.set(VerifyState::Error(e)),
            }
        });
    };

    view! {
        // -- modal
        {move || show_modal.get().then(|| view! {
            <VerifyStepsModal on_close=Callback::new(move |_| show_modal.set(false)) />
        })}

        <div class="max-w-2xl mx-auto px-4 py-12">

            // -- header
            <div class="mb-10">
                <h1 class="text-3xl font-display font-semibold text-primary-600 mb-4">
                    "Verify a Document"
                </h1>
                <p class="text-gray-500 leading-relaxed text-sm">
                    "Upload a signed file to run a forensic analysis. "
                    "The pipeline will extract the embedded signature, recompute the hash "
                    "and cross-check the audit registry."
                </p>
            </div>

            // -- form card
            {move || {
                if matches!(state.get(), VerifyState::Success(_)) {
                    view! { <div></div> }.into_any()
                } else {
                    view! {
                        <div class="card p-8 flex flex-col gap-6">

                            <div>
                                <label class="block text-sm font-semibold text-navy mb-2">
                                    "Signed File"
                                </label>
                                <VerifyDropZone file=file on_clear=on_clear />
                                <div class="mt-3">
                                    <VerifyStepsFlow
                                        on_show_more=Callback::new(move |_| show_modal.set(true))
                                    />
                                </div>
                            </div>

                            // -- error banner
                            {move || {
                                if let VerifyState::Error(e) = state.get() {
                                    view! {
                                        <div class="flex items-center gap-3 p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
                                            <CircleAlert size=18 color="#dc2626" />
                                            {e}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}

                            // -- submit
                            {move || {
                                let loading = matches!(state.get(), VerifyState::Loading);
                                view! {
                                    <button
                                        class="inline-flex items-center justify-center gap-3 w-full px-6 py-4 text-base font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg hover:shadow-primary-500/20 transform hover:scale-[1.01] transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed disabled:transform-none"
                                        on:click=on_submit
                                        disabled=loading
                                    >
                                        {if loading {
                                            view! {
                                                <span class="animate-spin">
                                                    <LoaderCircle size=20 color="#ffffff" />
                                                </span>
                                                "Analyzing..."
                                            }.into_any()
                                        } else {
                                            view! {
                                                <FileSearch size=20 color="#ffffff" />
                                                "Verify Document"
                                                <ArrowRight size=18 color="#ffffff" />
                                            }.into_any()
                                        }}
                                    </button>
                                }
                            }}
                        </div>
                    }.into_any()
                }
            }}

            // -- result + actions
            {move || {
                if let VerifyState::Success(data) = state.get() {
                    view! {
                        <div class="flex flex-col gap-4">
                            <VerifyResultCard data=data />
                            <button
                                class="inline-flex items-center justify-center gap-2 w-full px-5 py-3 text-sm font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-xl hover:bg-primary-50 hover:border-primary-600 transform hover:scale-[1.02] transition-all duration-300"
                                on:click=move |_| on_reset()
                            >
                                <RotateCcw size=18 color="#d20f39" />
                                "Verify Another File"
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
