use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use lucide_leptos::{
    ArrowRight, CircleAlert, FileSearch, Hash, KeyRound, LoaderCircle, RotateCcw, Upload,
};
use wasm_bindgen_futures::spawn_local;

use super::api::{verify_by_code, verify_document, CodeVerifyData, VerifyData};
use super::components::{
    drop_zone::VerifyDropZone, result_card::VerifyResultCard, steps_flow::VerifyStepsFlow,
    steps_modal::VerifyStepsModal,
};

#[derive(Clone, PartialEq)]
enum Tab {
    Upload,
    Code,
}

#[derive(Clone)]
enum VerifyState {
    Idle,
    Loading,
    Success(VerifyData),
    CodeSuccess(CodeVerifyData),
    Error(String),
}

#[component]
pub fn VerifyPage() -> impl IntoView {
    let file = RwSignal::new(None::<web_sys::File>);
    let state = RwSignal::new(VerifyState::Idle);
    let show_modal = RwSignal::new(false);
    let active_tab = RwSignal::new(Tab::Upload);
    let code_input = RwSignal::new(String::new());

    // -- si viene con ?code= en la URL, activa tab Code y prellenea
    let query = use_query_map();
    let initial_code = query.with(|q| q.get("code")).unwrap_or_default();
    if !initial_code.is_empty() {
        active_tab.set(Tab::Code);
        code_input.set(initial_code.clone());

        // -- auto-verifica si viene con código en URL
        #[cfg(feature = "hydrate")]
        {
            let code = initial_code.clone();
            spawn_local(async move {
                state.set(VerifyState::Loading);
                match verify_by_code(code).await {
                    Ok(data) => state.set(VerifyState::CodeSuccess(data)),
                    Err(e) => state.set(VerifyState::Error(e)),
                }
            });
        }
    }

    let on_reset = move || {
        file.set(None);
        code_input.set(String::new());
        state.set(VerifyState::Idle);
    };

    let on_clear = Callback::new(move |_| {
        file.set(None);
        state.set(VerifyState::Idle);
    });

    let on_submit_file = move |_| {
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

    let on_submit_code = move |_| {
        let code = code_input.get();
        let code = code.trim().to_uppercase();
        if code.is_empty() {
            state.set(VerifyState::Error(
                "Please enter a verification code".to_string(),
            ));
            return;
        }
        state.set(VerifyState::Loading);
        spawn_local(async move {
            match verify_by_code(code).await {
                Ok(data) => state.set(VerifyState::CodeSuccess(data)),
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
                    "Upload a signed file or enter a verification code to check document authenticity."
                </p>
            </div>

            // -- result views
            {move || match state.get() {
                VerifyState::Success(data) => view! {
                    <div class="flex flex-col gap-4">
                        <VerifyResultCard data=data />
                        <button
                            class="inline-flex items-center justify-center gap-2 w-full px-5 py-3 text-sm font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-xl hover:bg-primary-50 hover:border-primary-600 transform hover:scale-[1.02] transition-all duration-300"
                            on:click=move |_| on_reset()
                        >
                            <RotateCcw size=18 color="#d20f39" />
                            "Verify Another"
                        </button>
                    </div>
                }.into_any(),

                VerifyState::CodeSuccess(data) => view! {
                    <CodeResultCard data=data on_reset=Callback::new(move |_| on_reset()) />
                }.into_any(),

                _ => view! {
                    <div class="card p-8 flex flex-col gap-6">

                        // -- tabs
                        <div class="flex gap-1 p-1 bg-gray-100 rounded-xl w-fit">
                            <button
                                class=move || format!(
                                    "inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg transition-all duration-200 {}",
                                    if active_tab.get() == Tab::Upload { "bg-white text-primary-600 shadow-sm" } else { "text-gray-500 hover:text-gray-700" }
                                )
                                on:click=move |_| { active_tab.set(Tab::Upload); state.set(VerifyState::Idle); }
                            >
                                <Upload size=15 color=if active_tab.get() == Tab::Upload { "#d20f39" } else { "#9ca3af" } />
                                "Upload File"
                            </button>
                            <button
                                class=move || format!(
                                    "inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg transition-all duration-200 {}",
                                    if active_tab.get() == Tab::Code { "bg-white text-primary-600 shadow-sm" } else { "text-gray-500 hover:text-gray-700" }
                                )
                                on:click=move |_| { active_tab.set(Tab::Code); state.set(VerifyState::Idle); }
                            >
                                <Hash size=15 color=if active_tab.get() == Tab::Code { "#d20f39" } else { "#9ca3af" } />
                                "Code"
                            </button>
                        </div>

                        // -- contenido del tab activo
                        {move || match active_tab.get() {
                            Tab::Upload => view! {
                                <div class="flex flex-col gap-6">
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

                                    // -- error
                                    {move || if let VerifyState::Error(e) = state.get() {
                                        view! {
                                            <div class="flex items-center gap-3 p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
                                                <CircleAlert size=18 color="#dc2626" />
                                                {e}
                                            </div>
                                        }.into_any()
                                    } else { view! { <div></div> }.into_any() }}

                                    // -- submit
                                    {move || {
                                        let loading = matches!(state.get(), VerifyState::Loading);
                                        view! {
                                            <button
                                                class="inline-flex items-center justify-center gap-3 w-full px-6 py-4 text-base font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg transform hover:scale-[1.01] transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed disabled:transform-none"
                                                on:click=on_submit_file
                                                disabled=loading
                                            >
                                                {if loading {
                                                    view! {
                                                        <span class="animate-spin"><LoaderCircle size=20 color="#ffffff" /></span>
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
                            }.into_any(),

                            Tab::Code => view! {
                                <div class="flex flex-col gap-6">
                                    <div>
                                        <label class="block text-sm font-semibold text-navy mb-2">
                                            "Verification Code"
                                        </label>
                                        <p class="text-xs text-gray-400 mb-3">
                                            "Scan the QR on the signed document or enter the code manually — format: "
                                            <span class="font-mono font-bold text-primary-500">"XDS-6H2"</span>
                                        </p>
                                        <div class="relative">
                                            <div class="absolute left-3 top-1/2 -translate-y-1/2">
                                                <KeyRound size=16 color="#9ca3af" />
                                            </div>
                                            <input
                                                type="text"
                                                placeholder="ABC-123"
                                                maxlength="7"
                                                class="w-full pl-9 pr-4 py-3 text-sm font-mono uppercase border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary-300 focus:border-primary-500 transition-all duration-200 bg-white tracking-widest"
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev).to_uppercase();
                                                    code_input.set(val);
                                                }
                                                prop:value=move || code_input.get()
                                            />
                                        </div>
                                    </div>

                                    // -- error
                                    {move || if let VerifyState::Error(e) = state.get() {
                                        view! {
                                            <div class="flex items-center gap-3 p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
                                                <CircleAlert size=18 color="#dc2626" />
                                                {e}
                                            </div>
                                        }.into_any()
                                    } else { view! { <div></div> }.into_any() }}

                                    // -- submit
                                    {move || {
                                        let loading = matches!(state.get(), VerifyState::Loading);
                                        view! {
                                            <button
                                                class="inline-flex items-center justify-center gap-3 w-full px-6 py-4 text-base font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg transform hover:scale-[1.01] transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed disabled:transform-none"
                                                on:click=on_submit_code
                                                disabled=loading
                                            >
                                                {if loading {
                                                    view! {
                                                        <span class="animate-spin"><LoaderCircle size=20 color="#ffffff" /></span>
                                                        "Checking..."
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <Hash size=20 color="#ffffff" />
                                                        "Verify Code"
                                                        <ArrowRight size=18 color="#ffffff" />
                                                    }.into_any()
                                                }}
                                            </button>
                                        }
                                    }}
                                </div>
                            }.into_any(),
                        }}
                    </div>
                }.into_any()
            }}
        </div>
    }
}

// -- card para resultado de code verify
#[component]
fn CodeResultCard(data: CodeVerifyData, on_reset: Callback<()>) -> impl IntoView {
    if !data.found {
        return view! {
            <div class="flex flex-col gap-4">
                <div class="flex flex-col gap-4 p-6 bg-white border border-gray-200 rounded-2xl shadow-sm">
                    <div class="flex items-center gap-4 p-4 bg-gray-50 border border-gray-100 rounded-xl">
                        <div class="p-2 bg-gray-100 rounded-xl">
                            <Hash size=24 color="#6b7280" />
                        </div>
                        <div>
                            <p class="font-display font-bold text-lg text-gray-600">"INVALID CODE"</p>
                            <p class="text-xs text-gray-400 mt-0.5">
                                "This verification code does not exist in the registry"
                            </p>
                        </div>
                    </div>
                </div>
                <button
                    class="inline-flex items-center justify-center gap-2 w-full px-5 py-3 text-sm font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-xl hover:bg-primary-50 transform hover:scale-[1.02] transition-all duration-300"
                    on:click=move |_| on_reset.run(())
                >
                    <RotateCcw size=18 color="#d20f39" />
                    "Try Again"
                </button>
            </div>
        }.into_any();
    }

    let status = data.status.clone().unwrap_or_else(|| "VALID".to_string());
    let signed_at = data.signed_at.as_ref().map(|v| match v {
        serde_json::Value::String(s) => s.split('.').next().unwrap_or(s).replace('T', " "),
        other => other.to_string(),
    });

    view! {
        <div class="flex flex-col gap-4">
            <div class=format!(
                "flex flex-col gap-4 p-6 bg-white rounded-2xl shadow-sm border {}",
                if status == "VALID" { "border-green-200" } else { "border-yellow-200" }
            )>
                // -- header
                <div class=format!(
                    "flex items-center gap-4 p-4 rounded-xl border {}",
                    if status == "VALID" { "bg-green-50 border-green-100" } else { "bg-yellow-50 border-yellow-100" }
                )>
                    <lucide_leptos::CircleCheck size=28 color=if status == "VALID" { "#16a34a" } else { "#d97706" } />
                    <div>
                        <p class=format!(
                            "font-display font-bold text-lg {}",
                            if status == "VALID" { "text-green-700" } else { "text-yellow-700" }
                        )>
                            {status.clone()}
                        </p>
                        <p class="text-xs text-gray-500 mt-0.5">"Verified via code — no tampering check performed"</p>
                    </div>
                </div>

                // -- metadata
                <div class="flex flex-col gap-3">
                    {data.filename.map(|v| view! {
                        <MetaItem label="Filename" value=v />
                    })}
                    {data.author.map(|v| view! {
                        <MetaItem label="Author" value=v />
                    })}
                    {data.verification_code.map(|v| view! {
                        <MetaItem label="Code" value=v />
                    })}
                    {signed_at.map(|v| view! {
                        <MetaItem label="Signed At" value=v />
                    })}
                    {data.hash.map(|v| view! {
                        <div class="flex items-start gap-3 p-3 bg-gray-50 rounded-xl">
                            <Hash size=14 color="#9ca3af" />
                            <div class="min-w-0">
                                <p class="text-xs text-gray-400 mb-0.5">"SHA-256"</p>
                                <p class="text-xs font-mono text-gray-600 break-all">{v}</p>
                            </div>
                        </div>
                    })}
                </div>
            </div>

            <button
                class="inline-flex items-center justify-center gap-2 w-full px-5 py-3 text-sm font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-xl hover:bg-primary-50 transform hover:scale-[1.02] transition-all duration-300"
                on:click=move |_| on_reset.run(())
            >
                <RotateCcw size=18 color="#d20f39" />
                "Verify Another"
            </button>
        </div>
    }.into_any()
}

#[component]
fn MetaItem(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-xl">
            <KeyRound size=14 color="#7287fd" />
            <div class="min-w-0">
                <p class="text-xs text-gray-400">{label}</p>
                <p class="text-sm font-medium text-navy truncate">{value}</p>
            </div>
        </div>
    }
}
