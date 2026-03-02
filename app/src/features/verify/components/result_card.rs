use super::super::api::VerifyData;
use leptos::prelude::*;
use lucide_leptos::{
    CircleAlert, CircleCheck, CircleQuestionMark, CircleX, Hash, KeyRound, ShieldCheck,
};

#[component]
pub fn VerifyResultCard(data: VerifyData) -> impl IntoView {
    let status = data.status.clone();

    view! {
        <div class=move || format!(
            "flex flex-col gap-6 p-6 rounded-2xl shadow-sm border {}",
            match status.as_str() {
                "VALID"        => "bg-white border-green-200",
                "TAMPERED"     => "bg-white border-red-200",
                "UNREGISTERED" => "bg-white border-yellow-200",
                _              => "bg-white border-gray-200",
            }
        )>
            // -- status header
            <StatusHeader status=data.status.clone() />

            // -- forensic checks
            <ForensicChecks
                hash_match=data.hash_match
                signature_valid=data.signature_valid
                registered=data.registered
            />

            // -- metadata (solo si hay datos)
            {if data.document_id.is_some() || data.author.is_some() {
                view! {
                    <div class="flex flex-col gap-3">
                        {data.filename.clone().map(|v| view! {
                            <MetaRow icon_color="#7287fd" label="Filename" value=v />
                        })}
                        {data.author.clone().map(|v| view! {
                            <MetaRow icon_color="#7287fd" label="Author" value=v />
                        })}
                        {data.document_id.clone().map(|v| view! {
                            <MetaRow icon_color="#7287fd" label="Document ID" value=v />
                        })}
                        {data.signed_at.clone().map(|v| view! {
                            <MetaRow icon_color="#7287fd" label="Signed At" value=v />
                        })}
                        {data.hash.clone().map(|v| view! {
                            <div class="flex items-start gap-3 p-3 bg-gray-50 rounded-xl">
                                <div class="mt-0.5 shrink-0">
                                    <Hash size=16 color="#9ca3af" />
                                </div>
                                <div class="flex-1 min-w-0">
                                    <p class="text-xs text-gray-400 mb-1">"SHA-256 Hash"</p>
                                    <p class="text-xs font-mono text-gray-600 break-all">{v}</p>
                                </div>
                            </div>
                        })}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

// -- status badge + icon header
#[component]
fn StatusHeader(status: String) -> impl IntoView {
    let (icon, bg, border, text, description) = match status.as_str() {
        "VALID" => (
            view! { <CircleCheck size=28 color="#16a34a" /> }.into_any(),
            "bg-green-50",
            "border-green-100",
            "text-green-700",
            "Signature valid and document registered",
        ),
        "TAMPERED" => (
            view! { <CircleX size=28 color="#dc2626" /> }.into_any(),
            "bg-red-50",
            "border-red-100",
            "text-red-700",
            "Document has been modified after signing",
        ),
        "UNREGISTERED" => (
            view! { <CircleAlert size=28 color="#d97706" /> }.into_any(),
            "bg-yellow-50",
            "border-yellow-100",
            "text-yellow-700",
            "Signature found but not in registry",
        ),
        _ => (
            view! { <CircleQuestionMark size=28 color="#6b7280" /> }.into_any(),
            "bg-gray-50",
            "border-gray-100",
            "text-gray-700",
            "Could not determine document status",
        ),
    };

    view! {
        <div class=format!("flex items-center gap-4 p-4 {} border {} rounded-xl", bg, border)>
            {icon}
            <div>
                <p class=format!("font-display font-bold text-lg {}", text)>{status}</p>
                <p class="text-xs text-gray-500 mt-0.5">{description}</p>
            </div>
        </div>
    }
}

// -- 3 checks forenses
#[component]
fn ForensicChecks(
    hash_match: Option<bool>,
    signature_valid: Option<bool>,
    registered: Option<bool>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <p class="text-xs font-semibold text-gray-400 uppercase tracking-wide">
                "Forensic Checks"
            </p>
            <div class="grid grid-cols-1 gap-2">
                <ForensicRow
                    label="Hash integrity"
                    description="SHA-256 recomputed and compared"
                    value=hash_match
                >
                    <Hash size=16 color="#9ca3af" />
                </ForensicRow>
                <ForensicRow
                    label="Ed25519 signature"
                    description="Cryptographic signature verified"
                    value=signature_valid
                >
                    <ShieldCheck size=16 color="#9ca3af" />
                </ForensicRow>
                <ForensicRow
                    label="Registry match"
                    description="Document found in audit registry"
                    value=registered
                >
                    <KeyRound size=16 color="#9ca3af" />
                </ForensicRow>
            </div>
        </div>
    }
}

#[component]
fn ForensicRow(
    label: &'static str,
    description: &'static str,
    value: Option<bool>,
    children: Children,
) -> impl IntoView {
    let (_dot_color, badge_text, badge_class) = match value {
        Some(true) => (
            "#16a34a",
            "PASS",
            "bg-green-50 text-green-700 border-green-200",
        ),
        Some(false) => ("#dc2626", "FAIL", "bg-red-50 text-red-700 border-red-200"),
        None => ("#9ca3af", "N/A", "bg-gray-50 text-gray-500 border-gray-200"),
    };

    view! {
        <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-xl">
            <div class="shrink-0">
                {children()}
            </div>
            <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-navy">{label}</p>
                <p class="text-xs text-gray-400">{description}</p>
            </div>
            <span class=format!("shrink-0 text-xs font-bold px-2.5 py-1 rounded-lg border {}", badge_class)>
                {badge_text}
            </span>
        </div>
    }
}

#[component]
fn MetaRow(icon_color: &'static str, label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-xl">
            <div class="shrink-0">
                <KeyRound size=16 color=icon_color />
            </div>
            <div class="min-w-0">
                <p class="text-xs text-gray-400">{label}</p>
                <p class="text-sm font-medium text-navy truncate">{value}</p>
            </div>
        </div>
    }
}
