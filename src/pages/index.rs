use leptos::prelude::*;
use crate::components::bento::{BentoGrid, BentoBlock};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section
            <section class="relative min-h-[72vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-4xl mx-auto flex flex-col items-center">
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Stop Fighting "
                        <br class="md:hidden" />
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-emerald-400">
                            "Your Computers."
                        </span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-3xl mx-auto leading-relaxed mb-10">
                        "Your email, wifi, computers, and backups should just work — so you can work. I'm the IT guy for small businesses: flat monthly support, no contracts, and a human who answers the phone."
                    </p>
                </div>
                
                // Centered Call to Action
                <div class="flex flex-col sm:flex-row gap-4 items-center mt-4">
                    <a href="tel:9042067198" class="px-8 py-4 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded-lg text-lg transition-colors font-mono shadow-lg shadow-emerald-500/20">
                        "Call (904) 206-7198"
                    </a>
                    <a href="/pricing" class="px-8 py-4 bg-slate-900 border border-slate-700 hover:border-cyan-500 text-slate-200 font-bold rounded-lg text-lg transition-colors font-mono">
                        "View Pricing"
                    </a>
                </div>
                <p class="text-sm text-slate-400 font-mono mt-4">"Direct line — no call centers, no ticket queues. I answer my own phone, 9am–5pm Mon–Fri."</p>
            </section>

            <BentoGrid>
                // Module A: The "whoami" IT Consultant Card
                <BentoBlock class="md:col-span-2 lg:col-span-3 no-hover">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-start">
                        <div class="md:col-span-1 flex flex-col items-center justify-center text-center">
                            <img src="/pfp.webp" alt="Vicente" class="h-32 w-32 rounded-full object-cover border-4 border-emerald-500/30 mb-4" />
                            <h3 class="text-2xl font-bold text-slate-100">"Vicente"</h3>
                            <p class="text-md text-emerald-400 font-mono">"Your Dedicated IT Partner"</p>
                            <div class="mt-6 w-full border-t border-slate-800/60 pt-4">
                                <div class="text-xs text-slate-500 font-mono mb-2">"System Status"</div>
                                <div class="flex items-center justify-center gap-2 text-sm text-emerald-400">
                                    <div class="h-2 w-2 rounded-full bg-emerald-400 animate-pulse"></div>
                                    "Accepting new clients"
                                </div>
                            </div>
                        </div>
                        
                        <div class="md:col-span-2 flex flex-col justify-center">
                            <div class="inline-block px-3 py-1 bg-emerald-950/50 border border-emerald-800/50 rounded-full text-xs text-emerald-400 font-mono mb-6 self-start">
                                "whoami // Technology Consultant"
                            </div>
                            <p class="text-slate-300 text-lg leading-relaxed mb-6">
                                "Tech breaks at the worst possible moment — 9am Monday, mid-call, the day before payroll. It's frustrating, it's expensive, and it's not your job to fix it. I'm Vicente — one IT guy who has fixed a thousand small-business problems just like yours."
                            </p>
                            <p class="text-slate-300 text-lg leading-relaxed mb-8">
                                "From a single laptop to a full office, I make the tech disappear so you can get back to the work you started your business to do. If it breaks, you call me — and I answer."
                            </p>
                            <ul class="space-y-3 mb-8">
                                <li class="flex items-center gap-3 text-slate-300">
                                    <span class="text-emerald-400">"✓"</span>
                                    <span>"You work directly with the founder — no account managers, no ticket queues."</span>
                                </li>
                                <li class="flex items-center gap-3 text-slate-300">
                                    <span class="text-emerald-400">"✓"</span>
                                    <span>"Your keys, hardware, and data belong to you. No vendor lock-in, ever."</span>
                                </li>
                                <li class="flex items-center gap-3 text-slate-300">
                                    <span class="text-emerald-400">"✓"</span>
                                    <span>"Same-day responses, remote-first support, flat predictable pricing."</span>
                                </li>
                            </ul>
                            <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4">
                                <a href="tel:9042067198" class="px-6 py-3 bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold rounded-lg transition-colors font-mono text-sm">
                                    "Call (904) 206-7198"
                                </a>
                                <span class="text-sm text-slate-400 font-mono">"Call anytime, 9am–5pm Mon–Fri. Flat monthly from $299 — no contracts."</span>
                            </div>
                        </div>
                    </div>
                </BentoBlock>
            </BentoGrid>

            // The Villain: Sound Familiar?
            <section class="max-w-4xl mx-auto px-4 w-full mt-24">
                <div class="border border-slate-800/80 bg-slate-900/30 rounded-3xl p-8 md:p-12 bento-scroll-card">
                    <div class="text-center mb-10">
                        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-rose-950/30 border border-rose-900/50 text-rose-400 text-sm font-mono mb-6">
                            <span class="w-2 h-2 rounded-full bg-rose-400 animate-pulse"></span>
                            "SOUND FAMILIAR?"
                        </div>
                        <h2 class="text-3xl md:text-4xl font-extrabold text-slate-100 uppercase tracking-tight">
                            "Your Week, " <span class="text-rose-400">"As Told By Your Tech"</span>
                        </h2>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        <div class="flex items-start gap-3 text-slate-300">
                            <span class="text-rose-400 mt-1 font-bold">"✗"</span>
                            <div>
                                <div class="font-bold text-slate-100 mb-1">"9:00 AM MONDAY"</div>
                                <p class="text-sm leading-relaxed">"The printer eats the invoice run. Twenty minutes gone before you've poured your coffee."</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-3 text-slate-300">
                            <span class="text-rose-400 mt-1 font-bold">"✗"</span>
                            <div>
                                <div class="font-bold text-slate-100 mb-1">"MID-CALL"</div>
                                <p class="text-sm leading-relaxed">"The wifi drops with a client on the line. You sound like a garage door opening and closing."</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-3 text-slate-300">
                            <span class="text-rose-400 mt-1 font-bold">"✗"</span>
                            <div>
                                <div class="font-bold text-slate-100 mb-1">"FRIDAY, 4:30 PM"</div>
                                <p class="text-sm leading-relaxed">"Email stops working right before payroll. You are now the IT department."</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-3 text-slate-300">
                            <span class="text-rose-400 mt-1 font-bold">"✗"</span>
                            <div>
                                <div class="font-bold text-slate-100 mb-1">"LATE SATURDAY"</div>
                                <p class="text-sm leading-relaxed">"You wonder — is anything actually backed up? You're not sure. That should scare you a little."</p>
                            </div>
                        </div>
                    </div>
                    <p class="text-slate-400 mt-10 text-center max-w-2xl mx-auto leading-relaxed">
                        "This is IT chaos. It's costing you time, money, and peace of mind — and the fix isn't learning more tech. The fix is one phone call."
                    </p>
                    <div class="text-center mt-8">
                        <a href="tel:9042067198" class="px-8 py-3 bg-rose-500 hover:bg-rose-400 text-slate-950 font-bold rounded-lg transition-colors font-mono text-sm">
                            "Call (904) 206-7198"
                        </a>
                    </div>
                </div>
            </section>

            // Case Studies Header
            <div class="mt-24 text-center mb-10 px-4 bento-scroll-card">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-950/30 border border-cyan-900/50 text-cyan-400 text-sm font-mono mb-6">
                    <span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
                    "CLIENT TRANSFORMATIONS"
                </div>
                <h2 class="text-3xl md:text-5xl font-extrabold text-slate-100 uppercase tracking-tight mb-6">
                    "Real-World " <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-indigo-400">"Results"</span>
                </h2>
                <p class="text-lg text-slate-400 max-w-2xl mx-auto leading-relaxed">
                    "See how we've helped local businesses eliminate tech headaches, rescue their infrastructure, and automate their operations."
                </p>
            </div>

            <BentoGrid>
                <BentoBlock class="md:col-span-2 lg:col-span-3 no-hover">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-center">
                        <div class="md:col-span-1 flex justify-center">
                            // Grayscale SVG placeholder transitioning to brand color
                            <div class="h-32 w-32 rounded-2xl bg-rose-600 flex items-center justify-center">
                                <span class="font-black text-3xl text-white">"AS"</span>
                            </div>
                        </div>
                        <div class="md:col-span-2">
                            <div class="flex items-center gap-2 mb-3">
                                <span class="px-2 py-1 rounded text-[10px] uppercase tracking-wider font-bold bg-rose-500/20 text-rose-400">"The IT Rescue Routine"</span>
                                <h3 class="text-xl font-bold text-slate-100">"Aggressive Signs"</h3>
                            </div>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                <span class="font-semibold text-white">"Your business data belongs to you—not your IT provider."</span>
                                " When we partnered with Aggressive Signs, they were held hostage by an unresponsive agency running their critical operations on a locked-down legacy server box."
                            </p>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                "We completely migrated their infrastructure to a high-performance local NAS and VPN server that they own outright, transitioned their team seamlessly to Google Workspace, and optimized their local design workstation hardware."
                            </p>
                            <div class="mt-4 p-4 border border-rose-900/30 bg-rose-950/10 rounded-lg">
                                <p class="text-sm font-mono text-rose-300">
                                    "No artificial software locks. No 3-week wait times for a support ticket. Just bulletproof systems and direct, same-day responses straight from the engineer."
                                </p>
                            </div>
                        </div>
                    </div>
                </BentoBlock>

                // Module D: Celeritech "Workflow Automation" Case Study
                <BentoBlock class="md:col-span-2 lg:col-span-3 no-hover">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-center">
                        <div class="md:col-span-1 flex justify-center">
                            // Grayscale SVG placeholder transitioning to brand color
                            <div class="h-32 w-32 rounded-2xl bg-indigo-600 flex items-center justify-center">
                                <span class="font-black text-3xl text-white">"CT"</span>
                            </div>
                        </div>
                        <div class="md:col-span-2">
                            <div class="flex items-center gap-2 mb-3">
                                <span class="px-2 py-1 rounded text-[10px] uppercase tracking-wider font-bold bg-indigo-500/20 text-indigo-400">"Workflow Automation Case Study"</span>
                                <h3 class="text-xl font-bold text-slate-100">"Celeritech"</h3>
                            </div>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                <span class="font-semibold text-white">"Manual data entry is a slow, costly bottleneck for growing companies."</span>
                                " When we partnered with Celeritech, their accounting team was spending hours every day manually opening, reviewing, and typing details from vendor invoices into their databases."
                            </p>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                "We engineered a secure, automated invoice processing platform. The system automatically reads incoming invoices, instantly extracts and validates structured line-item details, and securely synchronizes the results directly to their systems—cutting processing times by over 90%."
                            </p>
                            <div class="mt-4 p-4 border border-indigo-900/30 bg-indigo-950/10 rounded-lg">
                                <p class="text-sm font-mono text-indigo-300">
                                    "From hours of manual sorting to instant, hands-free extraction. Celeritech now processes hundreds of vendor invoices with complete accuracy and zero manual data entry."
                                </p>
                            </div>
                        </div>
                    </div>
                </BentoBlock>
            </BentoGrid>

        </div>
    }
}
