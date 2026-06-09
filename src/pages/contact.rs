use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

#[component]
pub fn ContactPage() -> impl IntoView {
    let query_map = use_query_map();
    let (selected_service, set_selected_service) = signal(String::new());
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
                        "Request " <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-emerald-400">"Consultation"</span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light leading-relaxed">
                        "Provide a few details below and I will contact you directly."
                    </p>
                </div>
                
                // Animated Scroll Indicator
                <div class="animate-scroll-cue flex flex-col items-center gap-2 text-slate-500 mt-12">
                    <span class="text-xs font-mono uppercase tracking-widest text-slate-600">"Scroll to explore"</span>
                    <svg class="w-5 h-5 text-cyan-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                    </svg>
                </div>
            </section>

            <div class="max-w-2xl mx-auto px-4 w-full flex flex-col gap-16">

            <div class="rounded-2xl border border-slate-800/60 bg-slate-900/90 p-8 md:p-12 relative overflow-hidden bento-card no-hover bento-scroll-card">
                <div class="absolute inset-0 bg-gradient-to-b from-cyan-950/5 to-transparent"></div>
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
                                <a href="/" class="px-6 py-3 bg-slate-900 border border-slate-800 hover:border-cyan-500 text-slate-200 font-bold rounded-lg transition-colors font-mono text-sm">
                                    "// Return Home"
                                </a>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <form class="flex flex-col gap-6 text-left" on:submit=move |ev: leptos::web_sys::SubmitEvent| {
                                ev.prevent_default();
                                set_submitted.set(true);
                            }>
                                <div>
                                    <label class="block text-sm font-medium text-slate-300 mb-1">"Email Address"</label>
                                    <input 
                                        type="email" 
                                        required 
                                        class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors" 
                                        placeholder="you@company.com"
                                    />
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    <div>
                                        <label class="block text-sm font-medium text-slate-300 mb-1">"Phone Number (Optional)"</label>
                                        <input 
                                            type="tel" 
                                            class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors" 
                                            placeholder="(555) 000-0000"
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium text-slate-300 mb-1">"Preferred Contact Method"</label>
                                        <select 
                                            class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors cursor-pointer"
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
                                        class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors cursor-pointer"
                                        prop:value=selected_service
                                        on:change=move |ev| {
                                            set_selected_service.set(event_target_value(&ev));
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
                                                class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors"
                                                placeholder="e.g. Syncing our CRM to billing, building a secure customer portal..."
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
                                                class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors"
                                                placeholder="e.g. www.mycompany.com"
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
                                                class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors"
                                                placeholder="e.g. 15 workstations, 2 NAS servers, 1 physical office"
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
                                        class="w-full px-4 py-3 bg-slate-950 border border-slate-800 rounded-lg focus:outline-none focus:border-cyan-500 text-slate-200 transition-colors" 
                                        placeholder="Tell me more about what you want to build or automate..."
                                    ></textarea>
                                </div>

                                <button 
                                    type="submit" 
                                    class="mt-2 w-full px-4 py-3 bg-cyan-500 hover:bg-cyan-400 text-slate-900 font-bold rounded-lg transition-colors font-mono text-sm"
                                >
                                    "// Send Secure Request"
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
