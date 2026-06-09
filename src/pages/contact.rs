use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

#[server]
pub async fn send_contact_email(
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
            "<h2>New Contact Form Submission</h2>\
             <p><strong>Email:</strong> {}</p>\
             <p><strong>Phone:</strong> {}</p>\
             <p><strong>Preferred Contact Method:</strong> {}</p>\
             <p><strong>Interested Service:</strong> {}</p>\
             <p><strong>Message:</strong></p>\
             <pre style=\"white-space: pre-wrap; font-family: sans-serif;\">{}</pre>\
             <p><strong>Additional Answers:</strong></p>\
             <pre style=\"white-space: pre-wrap; font-family: sans-serif;\">{}</pre>",
            email,
            phone.unwrap_or_else(|| "N/A".to_string()),
            contact_method,
            service,
            message,
            additional_answers.unwrap_or_else(|| "None".to_string())
        );

        let client = reqwest::Client::new();
        let resend_req = ResendEmailRequest {
            from: "Artorias Tech Lab Form <website@artoriastechlab.com>".to_string(),
            to: vec!["vicentepsalcedo@gmail.com".to_string()],
            subject: format!("New Inquiry: {}", service),
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
        let _ = (email, phone, contact_method, service, message, additional_answers);
        Ok(())
    }
}

#[component]
pub fn ContactPage() -> impl IntoView {
    let query_map = use_query_map();
    let (email, set_email) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (contact_method, set_contact_method) = signal("email".to_string());
    let (selected_service, set_selected_service) = signal(String::new());
    let (additional_answer, set_additional_answer) = signal(String::new());
    let (message, set_message) = signal(String::new());
    
    let (sending, set_sending) = signal(false);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (submitted, set_submitted) = signal(false);

    // Sync query parameter once on mount/change
    Effect::new(move |_| {
        let svc = query_map.with(|m| m.get("service").unwrap_or_default());
        if !svc.is_empty() {
            set_selected_service.set(svc);
        }
    });

    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section
            <section class="relative min-h-[72vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-4xl mx-auto flex flex-col items-center">
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Request " <span class="animate-shimmer-amber">"Consultation"</span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light leading-relaxed">
                        "Provide a few details below and I will contact you directly."
                    </p>
                </div>
                
                // Animated Scroll Indicator
                <div class="animate-scroll-cue flex flex-col items-center gap-2 text-slate-500 mt-12">
                    <span class="text-xs font-mono uppercase tracking-widest text-slate-600">"Scroll to explore"</span>
                    <svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                    </svg>
                </div>
            </section>

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
                                <h2 class="text-2xl font-bold text-slate-100 mb-2">"Message Sent Successfully"</h2>
                                <p class="text-slate-400 mb-8 max-w-md mx-auto">
                                    "Thank you. I will review your parameters and reach out within 24 hours to schedule our call."
                                </p>
                                <a href="/" class="px-6 py-3 bg-slate-900 border border-slate-800 hover:border-amber-500 text-slate-200 font-bold rounded-lg transition-colors font-mono text-sm">
                                    "// Return Home"
                                </a>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <form class="flex flex-col gap-6 text-left" on:submit=move |ev: leptos::web_sys::SubmitEvent| {
                                ev.prevent_default();
                                set_sending.set(true);
                                set_error_message.set(None);
                                
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
                                    match send_contact_email(email_val, phone_opt, method_val, service_label, message_val, add_opt).await {
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
                                    <label class="block text-sm font-medium text-slate-300 mb-1">"Interested Service"</label>
                                    <select 
                                        class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors cursor-pointer"
                                        prop:value=selected_service
                                        on:change=move |ev| {
                                            set_selected_service.set(event_target_value(&ev));
                                            set_additional_answer.set(String::new());
                                        }
                                    >
                                        <option value="">"General Inquiry"</option>
                                        <option value="digital-foundation">"Digital Foundation Systems"</option>
                                        <option value="custom-apps">"Custom Web Applications & Platforms"</option>
                                        <option value="infrastructure">"Infrastructure Engineering & Automation"</option>
                                    </select>
                                </div>

                                // Dynamic Target Questions
                                {move || match selected_service.get().as_str() {
                                    "custom-apps" => view! {
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
                                    "digital-foundation" => view! {
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
                                    "infrastructure" => view! {
                                        <div class="transition-all duration-300 animate-fadeIn">
                                            <label class="block text-sm font-medium text-slate-300 mb-1">
                                                "Roughly how many physical computers, servers, or offices do we need to secure? (Optional)"
                                            </label>
                                            <input 
                                                type="text" 
                                                class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors"
                                                placeholder="e.g. 15 workstations, 2 NAS servers, 1 physical office"
                                                prop:value=additional_answer
                                                on:input=move |ev| set_additional_answer.set(event_target_value(&ev))
                                            />
                                        </div>
                                    }.into_any(),
                                    _ => view! { <div class="hidden"></div> }.into_any()
                                }}

                                <div>
                                    <label class="block text-sm font-medium text-slate-300 mb-1">"Project Details / Message"</label>
                                    <textarea 
                                        rows="4" 
                                        required 
                                        class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-amber-500 text-slate-200 transition-colors" 
                                        placeholder="Tell me more about what you want to build or automate..."
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
                                        "// Sending Request..."
                                    } else {
                                        "// Send Secure Request"
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
