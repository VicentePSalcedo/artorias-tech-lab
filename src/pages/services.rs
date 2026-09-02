use leptos::prelude::*;

// Stripe Payment Links - create in Stripe Dashboard (Products > Payment Links) as RECURRING subscriptions,
// then paste the three URLs here. These power the "Subscribe" buttons on the pricing tiers.
const STRIPE_STARTER: &str = "https://buy.stripe.com/5kQbJ1beiezRgFZ3L3es008";
const STRIPE_GROWTH: &str = "https://buy.stripe.com/fZu9AT1DIbnFfBVbdves009";
const STRIPE_SCALE: &str = "https://buy.stripe.com/28E14n96a63l2P95Tbes00a";

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
                        "Flat " <span class="text-emerald-400">"Pricing"</span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light leading-relaxed">
                        "Stop fighting your computers. Flat monthly plans that keep your email, wifi, backups, and security running — so you can run your business. No contracts, cancel anytime."
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
            <div class="mt-16 flex flex-col gap-8">
                <div class="text-center">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-950/30 border border-emerald-900/50 text-emerald-400 text-sm font-mono mb-4">
                        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                        "MANAGED IT PRICING"
                    </div>
                    <h2 class="text-3xl md:text-4xl font-extrabold text-slate-100 uppercase tracking-tight">
                        "Flat monthly. " <span class="text-emerald-400">"No surprises."</span>
                    </h2>
                    <p class="text-slate-400 max-w-2xl mx-auto mt-4 leading-relaxed text-lg">
                        "Priced by users or devices — whichever you hit first. Most offices comfortably fit 3 devices per user."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 flex flex-col gap-6 bento-scroll-card">
                        <div>
                            <div class="text-sm font-mono text-slate-500 mb-1">"STARTER"</div>
                            <div class="text-4xl font-extrabold text-slate-100">"$299"<span class="text-lg text-slate-400 font-normal">"/mo"</span></div>
                            <div class="text-base text-slate-400 mt-1">"Up to 3 users · 9 devices"</div>
                        </div>
                        <ul class="space-y-2 text-slate-400 text-base flex-1">
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Remote IT support (business hours)"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "System monitoring & patching"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Backup oversight"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Monthly check-in"</li>
                        </ul>
                        <a href={STRIPE_STARTER} class="text-center px-4 py-2 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono">"Subscribe — $299/mo"</a>
                    </div>

                    <div class="border border-emerald-500/50 bg-slate-900/90 rounded-2xl p-8 flex flex-col gap-6 relative bento-scroll-card">
                        <div class="absolute top-4 right-4 px-2 py-1 rounded-full bg-emerald-500/20 border border-emerald-500/50 text-emerald-400 text-[10px] font-mono uppercase tracking-wider">
                            "Most Popular"
                        </div>
                        <div>
                            <div class="text-sm font-mono text-slate-500 mb-1">"GROWTH"</div>
                            <div class="text-4xl font-extrabold text-slate-100">"$599"<span class="text-lg text-slate-400 font-normal">"/mo"</span></div>
                            <div class="text-base text-slate-400 mt-1">"Up to 8 users · 24 devices"</div>
                        </div>
                        <ul class="space-y-2 text-slate-400 text-base flex-1">
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Everything in Starter"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Endpoint security (EDR)"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Email security"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Microsoft 365 / Google Workspace management"</li>
                        </ul>
                        <a href={STRIPE_GROWTH} class="text-center px-4 py-2 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono">"Subscribe — $599/mo"</a>
                    </div>

                    <div class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 flex flex-col gap-6 bento-scroll-card">
                        <div>
                            <div class="text-sm font-mono text-slate-500 mb-1">"SCALE"</div>
                            <div class="text-4xl font-extrabold text-slate-100">"$999"<span class="text-lg text-slate-400 font-normal">"/mo"</span></div>
                            <div class="text-base text-slate-400 mt-1">"Up to 15 users · 45 devices"</div>
                        </div>
                        <ul class="space-y-2 text-slate-400 text-base flex-1">
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Everything in Growth"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "After-hours coverage"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Quarterly IT review"</li>
                            <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Priority response"</li>
                        </ul>
                        <a href={STRIPE_SCALE} class="text-center px-4 py-2 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono">"Subscribe — $999/mo"</a>
                    </div>
                </div>

                <div class="flex items-center justify-between">
                    <div class="text-base font-mono text-slate-400">"WHAT'S INCLUDED IN EACH TIER"</div>
                    <div class="text-sm font-mono text-slate-500">"✓ included · ✗ not included"</div>
                </div>
                <div class="overflow-x-auto border border-slate-800/80 bg-slate-950 rounded-2xl">
                    <table class="w-full min-w-[680px] text-left font-mono text-sm">
                        <thead>
                            <tr class="border-b border-slate-800 text-slate-400">
                                <th class="py-3 pr-4 pl-4 font-medium text-slate-500">"FEATURE"</th>
                                <th class="py-3 px-4 text-center font-bold text-slate-100">"STARTER"</th>
                                <th class="py-3 px-4 text-center font-bold text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"GROWTH"</th>
                                <th class="py-3 px-4 text-center font-bold text-slate-100">"SCALE"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-200 font-bold">"Team size"</td>
                                <td class="py-3 px-4 text-center text-slate-300">"3 users · 9 devices"</td>
                                <td class="py-3 px-4 text-center text-slate-300 border-x border-slate-800/60 bg-emerald-500/5">"8 users · 24 devices"</td>
                                <td class="py-3 px-4 text-center text-slate-300">"15 users · 45 devices"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Remote IT support (business hours)"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"System monitoring & patching"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Backup oversight (verified + tested restores)"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Monthly check-in"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Endpoint security (EDR)"</td>
                                <td class="py-3 px-4 text-center text-slate-600">"✗"</td>
                                <td class="py-3 px-4 text-center text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Email security (phishing & fraud protection)"</td>
                                <td class="py-3 px-4 text-center text-slate-600">"✗"</td>
                                <td class="py-3 px-4 text-center text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Microsoft 365 / Google Workspace management"</td>
                                <td class="py-3 px-4 text-center text-slate-600">"✗"</td>
                                <td class="py-3 px-4 text-center text-emerald-400 border-x border-slate-800/60 bg-emerald-500/5">"✓"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"After-hours coverage"</td>
                                <td class="py-3 px-4 text-center text-slate-600">"✗"</td>
                                <td class="py-3 px-4 text-center text-slate-600 border-x border-slate-800/60 bg-emerald-500/5">"✗"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr class="border-b border-slate-800/60">
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Quarterly IT review"</td>
                                <td class="py-3 px-4 text-center text-slate-600">"✗"</td>
                                <td class="py-3 px-4 text-center text-slate-600 border-x border-slate-800/60 bg-emerald-500/5">"✗"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                            <tr>
                                <td class="py-3 pr-4 pl-4 text-slate-300">"Priority response"</td>
                                <td class="py-3 px-4 text-center text-slate-600">"✗"</td>
                                <td class="py-3 px-4 text-center text-slate-600 border-x border-slate-800/60 bg-emerald-500/5">"✗"</td>
                                <td class="py-3 px-4 text-center text-emerald-400">"✓"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div class="border border-slate-800/80 bg-slate-950 rounded-2xl p-6 font-mono text-sm text-slate-400 flex flex-col md:flex-row justify-between gap-4">
                    <span>"Projects & one-off work: $150/hr"</span>
                    <span>"No contracts. Cancel anytime."</span>
                    <a href="tel:9042067198" class="hover:text-emerald-400 transition-colors">"Questions? Call (904) 206-7198 — 9am–5pm Mon–Fri"</a>
                </div>

                <div class="flex flex-col gap-6">
                    <div>
                        <div class="text-cyan-400 font-bold font-mono text-sm">"PROJECTS & ONE-OFFS"</div>
                        <div class="text-slate-500 text-xs font-mono mt-1">"Everything outside your monthly plan — flat-fee projects or billed hourly at $150/hr"</div>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="border border-slate-800/80 bg-slate-900/40 rounded-xl p-5 flex flex-col gap-3 bento-scroll-card">
                            <div class="text-[11px] font-mono text-cyan-400 uppercase tracking-wider">"EMAIL SETUP"</div>
                            <div class="text-sm text-slate-200 font-bold">"Business Email Migration"</div>
                            <p class="text-xs text-slate-400 leading-relaxed flex-1">"Google Workspace on your own domain — move off @gmail.com with SPF/DKIM/DMARC hardening and zero downtime."</p>
                            <div class="text-sm font-mono text-slate-200">"$299 flat · up to 5 mailboxes"</div>
                            <div class="text-xs font-mono text-slate-500">"+ $5/mailbox/mo managed"</div>
                            <a href="/contact?service=managed-it" class="text-xs font-mono text-cyan-400 hover:text-cyan-300 transition-colors">"Inquire"</a>
                        </div>
                        <div class="border border-slate-800/80 bg-slate-900/40 rounded-xl p-5 flex flex-col gap-3 bento-scroll-card">
                            <div class="text-[11px] font-mono text-cyan-400 uppercase tracking-wider">"HARDWARE & NETWORK"</div>
                            <div class="text-sm text-slate-200 font-bold">"Office Setup & Wiring"</div>
                            <p class="text-xs text-slate-400 leading-relaxed flex-1">"Wifi, network, computers, printers — a new office configured or an existing setup untangled."</p>
                            <div class="text-sm font-mono text-slate-200">"$150/hr"</div>
                            <div class="text-xs font-mono text-slate-500">"usually 2–4 hours"</div>
                            <a href="/contact" class="text-xs font-mono text-cyan-400 hover:text-cyan-300 transition-colors">"Inquire"</a>
                        </div>
                        <div class="border border-slate-800/80 bg-slate-900/40 rounded-xl p-5 flex flex-col gap-3 bento-scroll-card">
                            <div class="text-[11px] font-mono text-cyan-400 uppercase tracking-wider">"WEBSITE & AUTOMATION"</div>
                            <div class="text-sm text-slate-200 font-bold">"Custom Builds"</div>
                            <p class="text-xs text-slate-400 leading-relaxed flex-1">"Website redesign, custom software, and workflow automation tailored to your business."</p>
                            <div class="text-sm font-mono text-slate-200">"Quoted per project"</div>
                            <div class="text-xs font-mono text-slate-500">"call 904-206-7198 to discuss"</div>
                            <a href="/contact?service=automation" class="text-xs font-mono text-cyan-400 hover:text-cyan-300 transition-colors">"Inquire"</a>
                        </div>
                    </div>
                </div>

                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-6 font-mono text-sm text-slate-400 space-y-3">
                    <div class="text-emerald-400 font-bold">"WHAT HAPPENS AFTER YOU SUBSCRIBE"</div>
                    <div>"1. Stripe sends you an instant confirmation receipt."</div>
                    <div>"2. I email you within 24 hours to schedule your setup call."</div>
                    <div>"3. We install the remote support agent and finish onboarding in your first week."</div>
                    <div>"4. No contracts — cancel anytime. First 30 days fully refundable."</div>
                </div>
            </div>
                <div class="flex flex-col gap-12">
                
                // Package 1: Managed IT Support
                <div
                    class=move || {
                        let mut classes = "border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out bento-scroll-card".to_string();
                        if show_it_specs.get() {
                            classes.push_str(" -translate-y-2 scale-[1.02] shadow-2xl shadow-emerald-500/20 border-emerald-500/50 z-20");
                        }
                        classes
                    }
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
                            "Keep your business online, secure, and running smoothly."
                        </p>
                        <div class="text-3xl font-bold text-emerald-400 mb-4 font-mono">
                            "from $299/mo"
                        </div>
                        <p class="text-slate-300 mb-6 max-w-3xl leading-relaxed">
                            "Stop wasting hours trying to fix a rogue printer or a dropped wifi signal. This package provides direct remote IT support, hardware setup, and proactive network security for your entire team. We handle the technical headaches so you can run your business."
                        </p>
                        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                            <ul class="space-y-2 text-slate-400">
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Remote IT Support & Troubleshooting"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Computers, Printers, and Router Setups"</li>
                                <li class="flex items-center gap-3"><span class="text-emerald-400">"✓"</span> "Network Security, Backups, and Business Email"</li>
                            </ul>
                            <div class="flex items-center gap-4 w-full md:w-auto justify-end">
                                <button 
                                    on:click=move |_| set_show_it_specs.update(|v| *v = !*v) 
                                    class="px-4 py-2 rounded border border-slate-800 bg-slate-950 font-mono text-xs text-slate-400 hover:text-emerald-400 hover:border-emerald-500 transition-colors"
                                >
                                    {move || if show_it_specs.get() { "[hide details]" } else { "[view details]" }}
                                </button>
                                <a 
                                    href="/contact?service=managed-it" 
                                    class="px-6 py-2 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_it_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-emerald-400 font-bold">"SUPPORT SCOPE"</span>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2 pl-4 text-slate-300">
                                        <div>"• Hardware: Computer & printer provisioning"</div>
                                        <div>"• Network: Router config & Wifi troubleshooting"</div>
                                        <div>"• Software: Business email setup (Google Workspace / M365)"</div>
                                        <div>"• Security: Automated data backups & firewalls"</div>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // Package 2: Website Redesign & Management
                <div
                    class=move || {
                        let mut classes = "border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out".to_string();
                        if show_web_specs.get() {
                            classes.push_str(" -translate-y-2 scale-[1.02] shadow-2xl shadow-cyan-500/20 border-cyan-500/50 z-20");
                        }
                        classes
                    }
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
                            "Turn your outdated website into a modern, trusted digital storefront."
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
                                    {move || if show_web_specs.get() { "[hide details]" } else { "[view details]" }}
                                </button>
                                <a 
                                    href="/contact?service=web-management" 
                                    class="px-6 py-2 bg-cyan-500 hover:bg-cyan-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_web_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-cyan-400 font-bold">"WEB MANAGEMENT SCOPE"</span>
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
                    class=move || {
                        let mut classes = "border border-slate-800/80 bg-slate-900/90 rounded-2xl p-8 relative overflow-hidden group transition-all duration-300 ease-out bento-scroll-card".to_string();
                        if show_auto_specs.get() {
                            classes.push_str(" -translate-y-2 scale-[1.02] shadow-2xl shadow-rose-500/20 border-rose-500/50 z-20");
                        }
                        classes
                    }
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
                            "Eliminate tedious data entry and manual tasks."
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
                                    {move || if show_auto_specs.get() { "[hide details]" } else { "[view details]" }}
                                </button>
                                <a 
                                    href="/contact?service=automation" 
                                    class="px-6 py-2 bg-rose-500 hover:bg-rose-400 text-slate-900 font-bold rounded text-sm transition-colors font-mono"
                                >
                                    "Get Started"
                                </a>
                            </div>
                        </div>

                        // Interactive specs
                        {move || show_auto_specs.get().then(|| view! {
                            <div class="mt-6 pt-6 border-t border-slate-800/60 font-mono text-xs text-slate-400 space-y-4">
                                <div>
                                    <span class="text-rose-400 font-bold">"AUTOMATION CAPABILITIES"</span>
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
                    "Why Partner With Me?"
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
                    <li class="relative pl-6">
                        <span class="absolute left-0 text-rose-500">"▶"</span>
                        <strong class="text-slate-200">"Peace of Mind:"</strong> " An unmonitored, unbacked-up network is a ticking clock — one ransomware click or dead hard drive away from a $5,000 emergency. Proactive oversight is always cheaper than the rescue."
                    </li>
                </ul>
            </div>

            // Contact CTA
            <div class="mt-16 p-8 md:p-12 border border-slate-800/80 bg-slate-900/30 rounded-3xl text-center bento-scroll-card">
                <h2 class="text-3xl font-bold text-slate-100 mb-4">"Ready to eliminate IT headaches?"</h2>
                <p class="text-slate-400 mb-8 max-w-2xl mx-auto">
                    "Call anytime during business hours — 9am–5pm ET, Monday–Friday. We'll talk through securing your network, upgrading your web presence, or automating your tedious tasks."
                </p>
                <div class="flex flex-col sm:flex-row justify-center items-center gap-4">
                    <a href="tel:9042067198" class="px-8 py-3 bg-cyan-500 hover:bg-cyan-400 text-slate-900 font-bold rounded-lg transition-colors font-mono text-sm">
                        "Call (904) 206-7198 Now"
                    </a>
                    <span class="text-slate-500">"or"</span>
                    <a href="/contact" class="px-8 py-3 bg-slate-900 border border-slate-700 hover:border-cyan-500 text-slate-200 font-bold rounded-lg transition-colors font-mono text-sm">
                        "Request a Callback"
                    </a>
                </div>
            </div>
            </div>
        </div>
    }
}
