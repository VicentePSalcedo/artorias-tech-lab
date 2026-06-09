use leptos::prelude::*;

#[component]
pub fn ServicesPage() -> impl IntoView {
    let (show_apps_specs, set_show_apps_specs) = signal(false);
    let (show_foundation_specs, set_show_foundation_specs) = signal(false);
    let (show_infra_specs, set_show_infra_specs) = signal(false);

    view! {
        <div class="max-w-4xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
            // Header Section
            <div class="mb-16 flex flex-col items-center text-center bento-scroll-card" style="--scroll-progress: 1">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-950/30 border border-emerald-900/50 text-emerald-400 text-sm font-mono mb-8">
                    <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                    "MODULE: SERVICES"
                </div>
                <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                    "Client " <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-cyan-400">"Services"</span>
                </h1>
                <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light">
                    "High-margin consulting and development packages built to eliminate operational bottlenecks and secure absolute ownership of your digital assets."
                </p>
            </div>

            <div class="flex flex-col gap-12">
                // Service 1: Digital Foundation Systems (Originally Service 2)
                <div 
                    class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out bento-scroll-card"
                    class=("-translate-y-2", move || show_foundation_specs.get())
                    class=("scale-[1.02]", move || show_foundation_specs.get())
                    class=("shadow-2xl", move || show_foundation_specs.get())
                    class=("shadow-emerald-500/20", move || show_foundation_specs.get())
                    class=("border-emerald-500/50", move || show_foundation_specs.get())
                    class=("z-20", move || show_foundation_specs.get())
                >
                    <div class="absolute top-0 right-0 p-8 opacity-10">
                        <svg class="w-24 h-24 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Digital Foundation Systems"</h2>
                        </div>
                        <p class="text-sm font-mono text-emerald-400 mb-4">
                            "// For businesses that need a high-performance web presence that actually drives revenue."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "No generic WordPress templates or bloated page builders. We build custom, hand-coded marketing engines that load in milliseconds, dominate Google rankings, and are structured specifically for LLM search queries (ChatGPT, Perplexity, SearchGPT). Your site will be entirely secure, completely self-owned, and fully optimized to turn traffic into clients."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Custom Websites & Generative Engine Optimization (GEO)"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "High-Converting Landing Pages"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Built-in Contact Forms & Customer Analytics"</li>
                            </ul>
                            <div class="flex items-center gap-4 w-full md:w-auto justify-end">
                                <button 
                                    on:click=move |_| set_show_foundation_specs.update(|v| *v = !*v) 
                                    class="px-4 py-2 rounded border border-slate-800 bg-slate-950 font-mono text-xs text-slate-400 hover:text-emerald-400 hover:border-emerald-500 transition-colors"
                                >
                                    {move || if show_foundation_specs.get() { "// [hide tech specs]" } else { "// [view tech specs]" }}
                                </button>
                                <a 
                                    href="/contact?service=digital-foundation" 
                                    class="px-6 py-2 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "// Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_foundation_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-emerald-400 font-bold">"// PERFORMANCE FOUNDATION"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Core: Clean, custom layouts (no bloated site templates)"</div>
                                        <div>"• Speed: Optimized for sub-second page loads"</div>
                                        <div>"• Styles: Bespoke styling for unique brand design"</div>
                                        <div>"• Standard: Built to achieve maximum performance scores"</div>
                                    </div>
                                </div>
                                <div>
                                    <span class="text-emerald-400 font-bold">"// AI READY & ANALYTICS"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Discovery: Optimized for ChatGPT & Perplexity crawl patterns"</div>
                                        <div>"• Design: Mobile-first layout for all screen sizes"</div>
                                        <div>"• Traffic: Custom privacy-friendly client visitor analytics"</div>
                                        <div>"• Optimization: Instant visual loading for users"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Service 2: Custom Web Applications & Platforms (Originally Service 1)
                <div 
                    class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out"
                    class=("-translate-y-2", move || show_apps_specs.get())
                    class=("scale-[1.02]", move || show_apps_specs.get())
                    class=("shadow-2xl", move || show_apps_specs.get())
                    class=("shadow-cyan-500/20", move || show_apps_specs.get())
                    class=("border-cyan-500/50", move || show_apps_specs.get())
                    class=("z-20", move || show_apps_specs.get())
                >
                    <div class="absolute top-0 right-0 p-8 opacity-10">
                        <svg class="w-24 h-24 text-cyan-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Custom Web Applications & Platforms"</h2>
                        </div>
                        <p class="text-sm font-mono text-cyan-400 mb-4">
                            "// For businesses that have outgrown off-the-shelf software and need a tailored digital asset."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "We engineer bespoke, production-ready web platforms, client portals, and internal tools built to solve specific operational bottlenecks. Leveraging type-safe, ultra-stable software architectures, we build software that scales infinitely, loads instantly, and runs without mystery crashes."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Bespoke Software Platforms & Customer Portals"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Secure Databases & Automatic Software Syncing"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Automated Cloud Setup (AWS & Secure Containers)"</li>
                            </ul>
                            <div class="flex items-center gap-4 w-full md:w-auto justify-end">
                                <button 
                                    on:click=move |_| set_show_apps_specs.update(|v| *v = !*v) 
                                    class="px-4 py-2 rounded border border-slate-800 bg-slate-950 font-mono text-xs text-slate-400 hover:text-cyan-400 hover:border-cyan-500 transition-colors"
                                >
                                    {move || if show_apps_specs.get() { "// [hide tech specs]" } else { "// [view tech specs]" }}
                                </button>
                                <a 
                                    href="/contact?service=custom-apps" 
                                    class="px-6 py-2 bg-cyan-500 hover:bg-cyan-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "// Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_apps_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-cyan-400 font-bold">"// CORE CAPABILITIES"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Backend: Ultra-reliable and secure database systems"</div>
                                        <div>"• Interface: High-speed interactive views"</div>
                                        <div>"• Connections: Secure, fast web link API channels"</div>
                                        <div>"• Database: Dedicated secure data storage vault"</div>
                                    </div>
                                </div>
                                <div>
                                    <span class="text-cyan-400 font-bold">"// DEPLOYMENT & MAINTENANCE"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Hosting: Secure, isolated cloud servers"</div>
                                        <div>"• Updates: Automated updates with zero downtime"</div>
                                        <div>"• Security: Strict cloud access permissions"</div>
                                        <div>"• Credentials: Fully encrypted security key storage"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Service 3: Infrastructure Engineering & Automation
                <div 
                    class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out bento-scroll-card"
                    class=("-translate-y-2", move || show_infra_specs.get())
                    class=("scale-[1.02]", move || show_infra_specs.get())
                    class=("shadow-2xl", move || show_infra_specs.get())
                    class=("shadow-rose-500/20", move || show_infra_specs.get())
                    class=("border-rose-500/50", move || show_infra_specs.get())
                    class=("z-20", move || show_infra_specs.get())
                >
                    <div class="absolute top-0 right-0 p-8 opacity-10">
                        <svg class="w-24 h-24 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Infrastructure Engineering & Automation"</h2>
                        </div>
                        <p class="text-sm font-mono text-rose-400 mb-4">
                            "// For companies looking to own their data, secure their network, and automate operations."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "We act as your dedicated systems architect, managing your business infrastructure without the bloat of a traditional IT agency. We specialize in building secure local servers, ensuring total data ownership on hardware you control, and writing custom scripts to connect your software tools."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Secure Local Servers (NAS) & File Storage"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Google Workspace Management & Security Audits"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Automatic Workflows (connecting your software tools)"</li>
                            </ul>
                            <div class="flex items-center gap-4 w-full md:w-auto justify-end">
                                <button 
                                    on:click=move |_| set_show_infra_specs.update(|v| *v = !*v) 
                                    class="px-4 py-2 rounded border border-slate-800 bg-slate-950 font-mono text-xs text-slate-400 hover:text-rose-400 hover:border-rose-500 transition-colors"
                                >
                                    {move || if show_infra_specs.get() { "// [hide tech specs]" } else { "// [view tech specs]" }}
                                </button>
                                <a 
                                    href="/contact?service=infrastructure" 
                                    class="px-6 py-2 bg-rose-500 hover:bg-rose-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "// Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_infra_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-rose-400 font-bold">"// NETWORK HARDWARE"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Hardware: Custom physical server installation"</div>
                                        <div>"• System: Secure, redundant local storage file share"</div>
                                        <div>"• Network: Private, segmented business network"</div>
                                        <div>"• Remote Access: Secure private network channels"</div>
                                    </div>
                                </div>
                                <div>
                                    <span class="text-rose-400 font-bold">"// SYSTEM AUTOMATIONS"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Backup: Automated multi-location file backup rotation"</div>
                                        <div>"• Scripts: Custom integration scripts to link your systems"</div>
                                        <div>"• Audit: Google Workspace security permission reviews"</div>
                                        <div>"• Alerts: Real-time team notifications via chat/email"</div>
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
                    "// Engagement Parameters"
                </h3>
                
                <ul class="space-y-6">
                    <li class="relative pl-6">
                        <span class="absolute left-0 text-cyan-500">"▶"</span>
                        <strong class="text-slate-200">"Direct Access:"</strong> " You work directly with the Chief Architect. We do not pass you off to account managers, junior developers, or sales reps."
                    </li>
                    <li class="relative pl-6">
                        <span class="absolute left-0 text-cyan-500">"▶"</span>
                        <strong class="text-slate-200">"Infrastructure Focused:"</strong> " We engineer robust backend systems, databases, cloud, and local NAS storage. We do not operate a daily on-call support desk for employee device glitches (e.g., password resets, mouse mapping, broken hardware peripherals)."
                    </li>
                    <li class="relative pl-6">
                        <span class="absolute left-0 text-cyan-500">"▶"</span>
                        <strong class="text-slate-200">"Full Autonomy:"</strong> " Every line of code written, server deployed, and piece of hardware specified is owned 100% by your business from day one. Zero vendor lock-in, zero monthly licensing markups."
                    </li>
                </ul>
            </div>

            // Contact CTA
            <div class="mt-16 p-8 md:p-12 border border-slate-800/80 bg-slate-900/30 rounded-3xl text-center bento-scroll-card">
                <h2 class="text-3xl font-bold text-slate-100 mb-4">"Let's build something extraordinary."</h2>
                <p class="text-slate-400 mb-8 max-w-2xl mx-auto">
                    "If you are ready to upgrade your business infrastructure or build a custom web application, let's talk. No high-pressure sales, just an honest conversation about what you need."
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
    }
}
