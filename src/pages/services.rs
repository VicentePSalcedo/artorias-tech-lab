use leptos::prelude::*;

#[component]
pub fn ServicesPage() -> impl IntoView {
    let (show_apps_specs, set_show_apps_specs) = signal(false);
    let (show_foundation_specs, set_show_foundation_specs) = signal(false);
    let (show_infra_specs, set_show_infra_specs) = signal(false);

    view! {
        <div class="max-w-4xl mx-auto py-12">
            <div class="text-center mb-16">
                <h1 class="text-4xl md:text-5xl font-bold text-slate-100 mb-4">"Client Services"</h1>
                <p class="text-slate-400 text-lg">"High-margin consulting and development packages."</p>
            </div>

            <div class="space-y-8">
                // Service 1: Custom Web Applications & Platforms
                <div class="border border-slate-800/80 bg-slate-900/50 rounded-2xl p-8 backdrop-blur-sm relative overflow-hidden group hover:border-cyan-500/50 transition-colors">
                    <div class="absolute top-0 right-0 p-8 opacity-10 group-hover:opacity-20 transition-opacity">
                        <svg class="w-24 h-24 text-cyan-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="flex flex-col md:flex-row md:items-center justify-between mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Custom Web Applications & Platforms"</h2>
                            <div class="mt-2 md:mt-0 text-cyan-400 font-mono font-bold text-xl">"$5,000+"</div>
                        </div>
                        <p class="text-xs font-mono text-cyan-500/90 mb-4">
                            "// For businesses that have outgrown off-the-shelf software and need a tailored digital asset."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "We engineer bespoke, production-ready web platforms, client portals, and internal tools built to solve specific operational bottlenecks. Leveraging type-safe, ultra-stable Rust architectures, we build software that scales infinitely, loads instantly, and runs without mystery crashes."
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
                                    <span class="text-cyan-400 font-bold">"// STACK COMPONENTS"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Language: Rust (Type-safe memory safety)"</div>
                                        <div>"• Frontend: Leptos (SSR & WASM Hydration)"</div>
                                        <div>"• API Gateway: Axum Web Framework"</div>
                                        <div>"• Database: PostgreSQL / AWS Aurora RDS"</div>
                                    </div>
                                </div>
                                <div>
                                    <span class="text-cyan-400 font-bold">"// OPERATIONS & DEPLOYMENT"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Secure containerization using Docker & AWS ECS"</div>
                                        <div>"• Automated Git-triggered CI/CD pipelines"</div>
                                        <div>"• AWS IAM role-based least privilege security"</div>
                                        <div>"• Secure credential management with Secrets Manager"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Service 2: Digital Foundation Systems
                <div class="border border-slate-800/80 bg-slate-900/50 rounded-2xl p-8 backdrop-blur-sm relative overflow-hidden group hover:border-emerald-500/50 transition-colors">
                    <div class="absolute top-0 right-0 p-8 opacity-10 group-hover:opacity-20 transition-opacity">
                        <svg class="w-24 h-24 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="flex flex-col md:flex-row md:items-center justify-between mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Digital Foundation Systems"</h2>
                            <div class="mt-2 md:mt-0 text-emerald-400 font-mono font-bold text-xl">"$2,500+"</div>
                        </div>
                        <p class="text-xs font-mono text-emerald-500/90 mb-4">
                            "// For businesses that need a high-performance web presence that actually drives revenue."
                        </p>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "No generic WordPress templates or bloated page builders. We build custom, hand-coded marketing engines that load in milliseconds and dominate search results. Your site will be entirely secure, completely self-owned, and fully optimized to turn traffic into clients."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Custom, Hand-Coded Website & Search Engine Optimization (SEO)"</li>
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
                                    <span class="text-emerald-400 font-bold">"// CORE ENGINE"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Markup: Semantic HTML5 & CSS3 Variables"</div>
                                        <div>"• Rendering: Server-Side Pre-rendering (SSR / SSG)"</div>
                                        <div>"• Styles: Vanilla CSS / SCSS custom architecture"</div>
                                        <div>"• Metrics: Target 100/100 Lighthouse performance"</div>
                                    </div>
                                </div>
                                <div>
                                    <span class="text-emerald-400 font-bold">"// DISCOVERY & METRICS"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Full SEO Schema markup (JSON-LD structured data)"</div>
                                        <div>"• Fully responsive mobile-first optimization"</div>
                                        <div>"• Privacy-focused custom analytics (Plausible / Google)"</div>
                                        <div>"• Sub-100ms First Contentful Paint (FCP)"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Service 3: Infrastructure Engineering & Automation
                <div class="border border-slate-800/80 bg-slate-900/50 rounded-2xl p-8 backdrop-blur-sm relative overflow-hidden group hover:border-rose-500/50 transition-colors">
                    <div class="absolute top-0 right-0 p-8 opacity-10 group-hover:opacity-20 transition-opacity">
                        <svg class="w-24 h-24 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                        </svg>
                    </div>
                    <div class="relative z-10">
                        <div class="flex flex-col md:flex-row md:items-center justify-between mb-2">
                            <h2 class="text-2xl font-bold text-slate-100">"Infrastructure Engineering & Automation"</h2>
                            <div class="mt-2 md:mt-0 text-rose-400 font-mono font-bold text-xl">"Monthly Retainer"</div>
                        </div>
                        <p class="text-xs font-mono text-rose-500/90 mb-4">
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
                                    <span class="text-rose-400 font-bold">"// HARDWARE & NETWORK"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Hardware: Custom Supermicro / Bare-metal"</div>
                                        <div>"• OS / File System: TrueNAS Scale (ZFS RAID-Z)"</div>
                                        <div>"• Networking: VLAN segmentation & routing"</div>
                                        <div>"• Remote Access: WireGuard VPN / Cloudflare Tunnels"</div>
                                    </div>
                                </div>
                                <div>
                                    <span class="text-rose-400 font-bold">"// SYSTEM OPERATIONS"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Automated 3-2-1 backup rotation scheme"</div>
                                        <div>"• API Automations via Python, Bash & Nushell scripts"</div>
                                        <div>"• Google Workspace OAuth & IAM security auditing"</div>
                                        <div>"• Custom Webhook triggers (Slack, Discord)"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>
            </div>

            // Engagement Parameters / Gatekeeper Section
            <div class="mt-16 border border-slate-800/80 bg-slate-950 rounded-2xl p-8 backdrop-blur-sm relative overflow-hidden font-mono text-sm text-slate-300">
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
            <div class="mt-16 p-8 md:p-12 border border-slate-800/80 bg-slate-900/30 rounded-3xl text-center">
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
