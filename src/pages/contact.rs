use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::components::layout::STRIPE_PORTAL_URL;

#[server]
pub async fn send_contact_email(
    name: String,
    email: String,
    phone: Option<String>,
    contact_method: String,
    service: String,
    message: String,
    additional_answers: Option<String>,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use serde::Serialize;

        #[derive(Serialize)]
        struct ResendEmailRequest {
            from: String,
            to: Vec<String>,
            subject: String,
            html: String,
        }

        let api_key = std::env::var("RESEND_API_KEY")
            .map_err(|_| ServerFnError::new("RESEND_API_KEY environment variable not set"))?;

        let html_content = format!(
            "<h2>New Message from the Website</h2>\
             <p><strong>Name:</strong> {}</p>\
             <p><strong>Email:</strong> {}</p>\
             <p><strong>Phone:</strong> {}</p>\
             <p><strong>Preferred Contact Method:</strong> {}</p>\
             <p><strong>Service:</strong> {}</p>\
             <p><strong>Message:</strong></p>\
             <pre style=\"white-space: pre-wrap; font-family: sans-serif;\">{}</pre>\
             <p><strong>Additional Answers:</strong></p>\
             <pre style=\"white-space: pre-wrap; font-family: sans-serif;\">{}</pre>",
            name,
            email,
            phone.unwrap_or_else(|| "N/A".to_string()),
            contact_method,
            service,
            message,
            additional_answers.unwrap_or_else(|| "None".to_string())
        );

        let client = reqwest::Client::new();
        let resend_req = ResendEmailRequest {
            from: "Artorias Tech Lab Website <website@artoriastechlab.com>".to_string(),
            to: vec!["vicentesalcedo@artoriastechlab.com".to_string()],
            subject: format!("New message from {} - {}", name, service),
            html: html_content,
        };

        let response = client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&resend_req)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to send email request: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(ServerFnError::new(format!("Resend API error ({}): {}", status, body)))
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (name, email, phone, contact_method, service, message, additional_answers);
        Ok(())
    }
}

#[component]
pub fn ContactPage() -> impl IntoView {
    let query_map = use_query_map();
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (contact_method, set_contact_method) = signal("email".to_string());
    let (selected_service, set_selected_service) = signal(String::new());
    let (additional_answer, set_additional_answer) = signal(String::new());
    let (message, set_message) = signal(String::new());

    let (sending, set_sending) = signal(false);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (submitted, set_submitted) = signal(false);

    // Sync service query parameter once on mount/change
    Effect::new(move |_| {
        let svc = query_map.with(|m| m.get("service").unwrap_or_default());
        if !svc.is_empty() {
            set_selected_service.set(svc);
        }
    });

    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section - phone first
            <section class="relative min-h-[60vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-3xl mx-auto flex flex-col items-center">
                    <div class="text-sm font-mono text-emerald-400 mb-6 border border-emerald-500/30 bg-emerald-950/30 rounded-full px-4 py-1">
                        "REACH ME DIRECTLY"
                    </div>
                    <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Call " <span class="animate-shimmer-amber">"(904) 206-7198"</span>
                    </h1>
                    <p class="text-xl text-slate-400 max-w-xl font-light leading-relaxed mb-8">
                        "I answer my own phone, 9am to 5pm, Monday through Friday. No call centers, no ticket queues — just me. We'll sort out timing and payment on the call."
                    </p>
                    <a href="tel:9042067198" class="px-10 py-4 bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold rounded-lg text-xl transition-colors font-mono">
                        "Call (904) 206-7198"
                    </a>
                    <p class="text-sm text-slate-500 font-mono mt-6">
                        "Prefer email? Send a message below and I'll get back to you."
                    </p>
                </div>
            </section>

            // The Plan: three clear steps
            <div class="max-w-3xl mx-auto px-4 w-full">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-5">
                        <div class="text-emerald-400 font-mono font-bold text-lg mb-2">"1"</div>
                        <div class="font-bold text-slate-100 text-sm mb-1">"Call (904) 206-7198"</div>
                        <p class="text-xs text-slate-400 leading-relaxed">"We talk for a few minutes about what's driving you crazy. No scripts, no sales pitch."</p>
                    </div>
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-5">
                        <div class="text-emerald-400 font-mono font-bold text-lg mb-2">"2"</div>
                        <div class="font-bold text-slate-100 text-sm mb-1">"I take a look"</div>
                        <p class="text-xs text-slate-400 leading-relaxed">"A free assessment of your computers, network, and backups — remote or on-site."</p>
                    </div>
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-5">
                        <div class="text-emerald-400 font-mono font-bold text-lg mb-2">"3"</div>
                        <div class="font-bold text-slate-100 text-sm mb-1">"You get a plan"</div>
                        <p class="text-xs text-slate-400 leading-relaxed">"Flat monthly or per-project. No contracts. I fix it while you work."</p>
                    </div>
                </div>
            </div>

            // Already a client strip
            <div class="max-w-2xl mx-auto px-4 w-full">
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-5 flex flex-col sm:flex-row items-center justify-between gap-3">
                    <div>
                        <div class="font-bold text-slate-200 text-sm">"Already a client?"</div>
                        <div class="text-xs text-slate-500 mt-1">"Update your payment method or cancel your plan anytime."</div>
                    </div>
                    <a href={STRIPE_PORTAL_URL} class="px-5 py-2 bg-slate-800 hover:bg-slate-700 border border-slate-700 text-slate-200 font-bold rounded-lg transition-colors font-mono text-xs whitespace-nowrap">
                        "Manage Subscription"
                    </a>
                </div>
            </div>

            <div class="max-w-2xl mx-auto px-4 w-full flex flex-col gap-16">
                <div class="rounded-2xl border border-slate-800/60 bg-slate-900/90 p-8 md:p-12 relative overflow-hidden bento-card no-hover bento-scroll-card">
                    <div class="absolute inset-0 bg-gradient-to-b from-amber-950/5 to-transparent"></div>
                    <div class="relative z-10">
                        {move || if submitted.get() {
                            view! {
                                <div class="text-center py-8">
                                    <div class="w-16 h-16 bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 rounded-full flex items-center justify-center mx-auto mb-6">
                                        <svg class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                        </svg>
                                    </div>
                                    <h2 class="text-2xl font-bold text-slate-100 mb-2">"Message Sent"</h2>
                                    <p class="text-slate-400 mb-8 max-w-md mx-auto">
                                        "Thanks for reaching out. I'll reply within one business day — usually sooner."
                                    </p>
                                    <a href="/" class="px-6 py-3 bg-slate-900 border border-slate-800 hover:border-amber-500 text-slate-200 font-bold rounded-lg transition-colors font-mono text-sm">
                                        "Return Home"
                                    </a>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <form class="flex flex-col gap-6 text-left" on:submit=move |ev: leptos::web_sys::SubmitEvent| {
                                    ev.prevent_default();
                                    set_sending.set(true);
                                    set_error_message.set(None);

                                    let name_val = name.get();
                                    let email_val = email.get();
                                    let phone_val = phone.get();
                                    let phone_opt = if phone_val.is_empty() { None } else { Some(phone_val) };
                                    let method_val = contact_method.get();
                                    let service_val = selected_service.get();
                                    let service_label = if service_val.is_empty() { "General Inquiry".to_string() } else { service_val };
                                    let message_val = message.get();
                                    let add_answer = additional_answer.get();
                                    let add_opt = if add_answer.is_empty() { None } else { Some(add_answer) };

                                    leptos::task::spawn_local(async move {
                                        match send_contact_email(name_val, email_val, phone_opt, method_val, service_label, message_val, add_opt).await {
                                            Ok(_) => {
                                                set_sending.set(false);
                                                set_submitted.set(true);
                                            }
                                            Err(e) => {
                                                set_sending.set(false);
                                                set_error_message.set(Some(e.to_string()));
                                            }
                                        }
                                    });
                                }>
                                    // Name
                                    <div>
                                        <label class="block text-sm font-medium text-slate-300 mb-1">"Your Name"</label>
                                        <input
                                            type="text"
                                            required
                                            class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                            placeholder="Jane Smith"
                                            prop:value=name
                                            on:input=move |ev| set_name.set(event_target_value(&ev))
                                        />
                                    </div>

                                    // Email
                                    <div>
                                        <label class="block text-sm font-medium text-slate-300 mb-1">"Email Address"</label>
                                        <input
                                            type="email"
                                            required
                                            class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                            placeholder="you@company.com"
                                            prop:value=email
                                            on:input=move |ev| set_email.set(event_target_value(&ev))
                                        />
                                    </div>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                        <div>
                                            <label class="block text-sm font-medium text-slate-300 mb-1">"Phone Number (Optional)"</label>
                                            <input
                                                type="tel"
                                                class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                                placeholder="(555) 000-0000"
                                                prop:value=phone
                                                on:input=move |ev| set_phone.set(event_target_value(&ev))
                                            />
                                        </div>
                                        <div>
                                            <label class="block text-sm font-medium text-slate-300 mb-1">"Preferred Contact Method"</label>
                                            <select
                                                class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors cursor-pointer"
                                                prop:value=contact_method
                                                on:change=move |ev| set_contact_method.set(event_target_value(&ev))
                                            >
                                                <option value="email">"Email"</option>
                                                <option value="phone">"Phone Call"</option>
                                                <option value="text">"Text Message"</option>
                                            </select>
                                        </div>
                                    </div>

                                    <div>
                                        <label class="block text-sm font-medium text-slate-300 mb-1">"What do you need help with?"</label>
                                        <select
                                            class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors cursor-pointer"
                                            prop:value=selected_service
                                            on:change=move |ev| {
                                                set_selected_service.set(event_target_value(&ev));
                                                set_additional_answer.set(String::new());
                                            }
                                        >
                                            <option value="">"General / Not sure yet"</option>
                                            <option value="managed-it">"Managed IT & Support"</option>
                                            <option value="web-management">"Website Redesign & Management"</option>
                                            <option value="automation">"Business Workflow Automation"</option>
                                        </select>
                                    </div>

                                    // Dynamic Target Questions
                                    {move || match selected_service.get().as_str() {
                                        "managed-it" => view! {
                                            <div class="transition-all duration-300 animate-fadeIn">
                                                <label class="block text-sm font-medium text-slate-300 mb-1">
                                                    "Roughly how many users and computers does your business have? (Optional)"
                                                </label>
                                                <input
                                                    type="text"
                                                    class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                                    placeholder="e.g. 5 employees, 6 computers, 1 office"
                                                    prop:value=additional_answer
                                                    on:input=move |ev| set_additional_answer.set(event_target_value(&ev))
                                                />
                                            </div>
                                        }.into_any(),
                                        "web-management" => view! {
                                            <div class="transition-all duration-300 animate-fadeIn">
                                                <label class="block text-sm font-medium text-slate-300 mb-1">
                                                    "What is your current website URL? (Optional, leave blank if starting fresh)"
                                                </label>
                                                <input
                                                    type="text"
                                                    class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                                    placeholder="e.g. www.mycompany.com"
                                                    prop:value=additional_answer
                                                    on:input=move |ev| set_additional_answer.set(event_target_value(&ev))
                                                />
                                            </div>
                                        }.into_any(),
                                        "automation" => view! {
                                            <div class="transition-all duration-300 animate-fadeIn">
                                                <label class="block text-sm font-medium text-slate-300 mb-1">
                                                    "What manual process or operational bottleneck are you trying to automate? (Optional)"
                                                </label>
                                                <textarea
                                                    rows="3"
                                                    class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                                    placeholder="e.g. Syncing our CRM to billing, building a secure customer portal..."
                                                    prop:value=additional_answer
                                                    on:input=move |ev| set_additional_answer.set(event_target_value(&ev))
                                                ></textarea>
                                            </div>
                                        }.into_any(),
                                        _ => view! { <div class="hidden"></div> }.into_any()
                                    }}

                                    <div>
                                        <label class="block text-sm font-medium text-slate-300 mb-1">"What do you need?"</label>
                                        <textarea
                                            rows="4"
                                            required
                                            class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                            placeholder="Tell me what's going on — a computer that's slow, email that's broken, a website you need, or something you'd rather not deal with. I'll take it from there."
                                            prop:value=message
                                            on:input=move |ev| set_message.set(event_target_value(&ev))
                                        ></textarea>
                                    </div>

                                    {move || error_message.get().map(|err| view! {
                                        <div class="p-4 bg-rose-500/10 border border-rose-500/30 text-rose-400 rounded-lg text-sm font-mono animate-fadeIn">
                                            "Error: " {err}
                                        </div>
                                    })}

                                    <button
                                        type="submit"
                                        disabled=sending
                                        class="mt-2 w-full px-4 py-3 bg-amber-500 hover:bg-amber-400 disabled:bg-amber-500/50 text-slate-950 font-bold rounded-lg transition-colors font-mono text-sm flex items-center justify-center gap-2"
                                    >
                                        {move || if sending.get() {
                                            "Sending Message..."
                                        } else {
                                            "Send Message"
                                        }}
                                    </button>
                                </form>
                            }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
