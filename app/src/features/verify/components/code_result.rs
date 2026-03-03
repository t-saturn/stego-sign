use leptos::prelude::*;
use lucide_leptos::{CircleCheck, CircleQuestionMark, Hash, KeyRound, RotateCcw};

use super::super::api::CodeVerifyData;

#[component]
pub fn CodeResultCard(data: CodeVerifyData, on_reset: Callback<()>) -> impl IntoView {
    if !data.found {
        return view! {
            <div class="flex flex-col gap-4">
                <div class="flex flex-col gap-4 p-6 bg-white border border-gray-200 rounded-2xl shadow-sm">
                    <div class="flex items-center gap-4 p-4 bg-gray-50 border border-gray-100 rounded-xl">
                        <CircleQuestionMark size=28 color="#6b7280" />
                        <div>
                            <p class="font-display font-bold text-lg text-gray-600">
                                "INVALID CODE"
                            </p>
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

    let (border, bg, text_color, icon) = if status == "VALID" {
        (
            "border-green-200",
            "bg-green-50 border-green-100",
            "text-green-700",
            view! { <CircleCheck size=28 color="#16a34a" /> }.into_any(),
        )
    } else {
        (
            "border-yellow-200",
            "bg-yellow-50 border-yellow-100",
            "text-yellow-700",
            view! { <CircleCheck size=28 color="#d97706" /> }.into_any(),
        )
    };

    view! {
        <div class="flex flex-col gap-4">
            <div class=format!(
                "flex flex-col gap-4 p-6 bg-white rounded-2xl shadow-sm border {}",
                border
            )>
                // -- header
                <div class=format!(
                    "flex items-center gap-4 p-4 rounded-xl border {}",
                    bg
                )>
                    {icon}
                    <div>
                        <p class=format!("font-display font-bold text-lg {}", text_color)>
                            {status}
                        </p>
                        <p class="text-xs text-gray-500 mt-0.5">
                            "Verified via code — registry lookup only"
                        </p>
                    </div>
                </div>

                // -- metadata
                <div class="flex flex-col gap-2">
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
