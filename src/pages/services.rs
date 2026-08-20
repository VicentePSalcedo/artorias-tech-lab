use leptos::prelude::*;

#[component]
pub fn ServicesPage() -> impl IntoView {
    let (show_it_specs, set_show_it_specs) = signal(false);
    let (show_web_specs, set_show_web_specs) = signal(false);
    let (show_auto_specs, set_show_auto_specs) = signal(false);

    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section
            <section class="relative min-h-[72vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-4xl mx-auto flex flex-col items-center">
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "IT " <span class="text-emerald-400">"Services"</span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light leading-relaxed">
                        "Clear, predictable packages for Managed IT, Website Management, and Workflow Automation tailored for small to medium-sized businesses."
                    </p>
                </div>
                
                // Animated Scroll Indicator
                <div class="animate-scroll-cue flex flex-col items-center gap-2 text-slate-500 mt-12">
                    <span class="text-xs font-mono uppercase tracking-widest text-slate-600">"Scroll to explore"</span>
                    <svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                    </svg>
                </div>
            </section>

            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 w-full flex flex-col gap-16">
                <div class="flex flex-col gap-12">
                
                // Package 1: Managed IT Support
                <div 
                    class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out bento-scroll-card"
                    class=("-translate-y-2", move || show_it_specs.get())
                    class=("scale-[1.02]", move || show_it_specs.get())
                    class=("shadow-2xl", move || show_it_specs.get())
                    class=("shadow-emerald-500/20", move || show_it_specs.get())
                    class=("border-emerald-500/50", move || show_it_specs.get())
                    class=("z-20", move || show_it_specs.get())
                >
                    <div class="absolute top-0 right-0 p-8 opacity-10">
                        <svg class="w-24 h-24 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Managed IT & Infrastructure Support"</h2>
                        </div>
                        <p class="text-sm font-mono text-emerald-400 mb-4">
                            "// Keep your business online, secure, and running smoothly."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "Stop wasting hours trying to fix a rogue printer or a dropped wifi signal. This package provides direct remote IT support, hardware setup, and proactive network security for your entire team. We handle the technical headaches so you can run your business."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Remote IT Support & Troubleshooting"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Computers, Printers, and Router Setups"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Network Security, Backups, and Email Services"</li>
                            </ul>
                            <div class="flex items-center gap-4 w-full md:w-auto justify-end">
                                <button 
                                    on:click=move |_| set_show_it_specs.update(|v| *v = !*v) 
                                    class="px-4 py-2 rounded border border-slate-800 bg-slate-950 font-mono text-xs text-slate-400 hover:text-emerald-400 hover:border-emerald-500 transition-colors"
                                >
                                    {move || if show_it_specs.get() { "// [hide details]" } else { "// [view details]" }}
                                </button>
                                <a 
                                    href="/contact?service=managed-it" 
                                    class="px-6 py-2 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "// Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_it_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-emerald-400 font-bold">"// SUPPORT SCOPE"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Hardware: Computer & printer provisioning"</div>
                                        <div>"• Network: Router config & Wifi troubleshooting"</div>
                                        <div>"• Software: Business email & account setup"</div>
                                        <div>"• Security: Automated data backups & firewalls"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Package 2: Website Redesign & Management
                <div 
                    class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out"
                    class=("-translate-y-2", move || show_web_specs.get())
                    class=("scale-[1.02]", move || show_web_specs.get())
                    class=("shadow-2xl", move || show_web_specs.get())
                    class=("shadow-cyan-500/20", move || show_web_specs.get())
                    class=("border-cyan-500/50", move || show_web_specs.get())
                    class=("z-20", move || show_web_specs.get())
                >
                    <div class="absolute top-0 right-0 p-8 opacity-10">
                        <svg class="w-24 h-24 text-cyan-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Website Redesign & Management"</h2>
                        </div>
                        <p class="text-sm font-mono text-cyan-400 mb-4">
                            "// Turn your outdated website into a modern, trusted digital storefront."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "We handle everything required to establish a professional digital presence. From securing your custom domain and SSL certificates, to redesigning your website and providing custom-branded links, we ensure your business looks sharp and functions flawlessly online."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Custom Website Redesigns"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Domain Registration & SSL Certificates"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Custom Branded Links & Ongoing Web Management"</li>
                            </ul>
                            <div class="flex items-center gap-4 w-full md:w-auto justify-end">
                                <button 
                                    on:click=move |_| set_show_web_specs.update(|v| *v = !*v) 
                                    class="px-4 py-2 rounded border border-slate-800 bg-slate-950 font-mono text-xs text-slate-400 hover:text-cyan-400 hover:border-cyan-500 transition-colors"
                                >
                                    {move || if show_web_specs.get() { "// [hide details]" } else { "// [view details]" }}
                                </button>
                                <a 
                                    href="/contact?service=web-management" 
                                    class="px-6 py-2 bg-cyan-500 hover:bg-cyan-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "// Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_web_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-cyan-400 font-bold">"// WEB MANAGEMENT SCOPE"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Domains: Custom URL registration & setup"</div>
                                        <div>"• Security: SSL certificate installation & renewal"</div>
                                        <div>"• Branding: Custom branded domain links"</div>
                                        <div>"• Updates: Ongoing content & software updates"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Package 3: Business Workflow Automation
                <div 
                    class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out bento-scroll-card"
                    class=("-translate-y-2", move || show_auto_specs.get())
                    class=("scale-[1.02]", move || show_auto_specs.get())
                    class=("shadow-2xl", move || show_auto_specs.get())
                    class=("shadow-rose-500/20", move || show_auto_specs.get())
                    class=("border-rose-500/50", move || show_auto_specs.get())
                    class=("z-20", move || show_auto_specs.get())
                >
                    <div class="absolute top-0 right-0 p-8 opacity-10">
                        <svg class="w-24 h-24 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Business Workflow Automation"</h2>
                        </div>
                        <p class="text-sm font-mono text-rose-400 mb-4">
                            "// Eliminate tedious data entry and manual tasks."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "If you or your team are spending hours copying and pasting data between systems, we can write custom software scripts to do it for you automatically. We build secure integrations that save time and eliminate human error."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Custom Software Automation Scripts"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Data Entry & Invoice Automation"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Connecting Disparate Software Systems"</li>
                            </ul>
                            <div class="flex items-center gap-4 w-full md:w-auto justify-end">
                                <button 
                                    on:click=move |_| set_show_auto_specs.update(|v| *v = !*v) 
                                    class="px-4 py-2 rounded border border-slate-800 bg-slate-950 font-mono text-xs text-slate-400 hover:text-rose-400 hover:border-rose-500 transition-colors"
                                >
                                    {move || if show_auto_specs.get() { "// [hide details]" } else { "// [view details]" }}
                                </button>
                                <a 
                                    href="/contact?service=automation" 
                                    class="px-6 py-2 bg-rose-500 hover:bg-rose-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "// Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_auto_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-rose-400 font-bold">"// AUTOMATION CAPABILITIES"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Scripts: Custom integration scripts"</div>
                                        <div>"• Syncing: Real-time data synchronization"</div>
                                        <div>"• Formats: Automated document & PDF parsing"</div>
                                        <div>"• Alerts: Automated email and SMS notifications"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>
            </div>

            // Engagement Parameters / Gatekeeper Section
            <div class="mt-16 border border-slate-800/80 bg-slate-950 rounded-2xl p-8 relative overflow-hidden font-mono text-sm text-slate-300 bento-scroll-card">
                <div class="border-b border-slate-800 pb-3 mb-6 flex justify-between items-center text-xs text-slate-500">
                    <span>"CONFIG // sys.config"</span>
                    <span class="text-emerald-500">"SYS_STATUS: ACTIVE"</span>
                </div>
                
                <h3 class="text-lg font-bold text-slate-100 mb-6 text-cyan-400">
                    "// Why Partner With Me?"
                </h3>
                
                <ul class="space-y-6">
                    <li class="relative pl-6">
                        <span class="absolute left-0 text-cyan-500">"▶"</span>
                        <strong class="text-slate-200">"Direct Access:"</strong> " You work directly with the founder. No account managers, no automated support queues, and no being bounced around a help desk."
                    </li>
                    <li class="relative pl-6">
                        <span class="absolute left-0 text-cyan-500">"▶"</span>
                        <strong class="text-slate-200">"Transparent Packaging:"</strong> " Small and medium-sized businesses need clear solutions, not endless enterprise consulting hours. My services are neatly packaged and priced for real-world impact."
                    </li>
                    <li class="relative pl-6">
                        <span class="absolute left-0 text-cyan-500">"▶"</span>
                        <strong class="text-slate-200">"Total Ownership:"</strong> " Everything I build, configure, or register for you is owned 100% by your business. No holding your domain name hostage, zero vendor lock-in."
                    </li>
                </ul>
            </div>

            // Contact CTA
            <div class="mt-16 p-8 md:p-12 border border-slate-800/80 bg-slate-900/30 rounded-3xl text-center bento-scroll-card">
                <h2 class="text-3xl font-bold text-slate-100 mb-4">"Ready to eliminate IT headaches?"</h2>
                <p class="text-slate-400 mb-8 max-w-2xl mx-auto">
                    "Reach out today to discuss how we can secure your network, upgrade your web presence, or automate your tedious tasks."
                </p>
                <div class="flex flex-col sm:flex-row justify-center items-center gap-4">
                    <a href="tel:9042067198" class="px-8 py-3 bg-cyan-500 hover:bg-cyan-400 text-slate-900 font-bold rounded-lg transition-colors font-mono text-sm">
                        "Call 904-206-7198"
                    </a>
                    <span class="text-slate-500">"or"</span>
                    <a href="/contact" class="px-8 py-3 bg-slate-900 border border-slate-700 hover:border-cyan-500 text-slate-200 font-bold rounded-lg transition-colors font-mono text-sm">
                        "// Request Consultation"
                    </a>
                </div>
            </div>
            </div>
        </div>
    }
}
