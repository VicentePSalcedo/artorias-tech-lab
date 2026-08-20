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
                        "IT Consulting & "
                        <br class="md:hidden" />
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-emerald-400">
                            "Technology Solutions"
                        </span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-3xl mx-auto leading-relaxed mb-10">
                        "Reliable IT support, website management, and workflow automation for small to medium-sized businesses."
                    </p>
                </div>
                
                // Centered Call to Action
                <div class="flex flex-col sm:flex-row gap-4 items-center mt-4">
                    <a href="/contact" class="px-8 py-4 bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold rounded-lg text-lg transition-colors font-mono shadow-lg shadow-emerald-500/20">
                        "// Request IT Support"
                    </a>
                    <a href="/services" class="px-8 py-4 bg-slate-900 border border-slate-700 hover:border-cyan-500 text-slate-200 font-bold rounded-lg text-lg transition-colors font-mono">
                        "View Services"
                    </a>
                </div>
            </section>

            <BentoGrid>
                // Module A: The "whoami" IT Consultant Card
                <BentoBlock class="md:col-span-2 lg:col-span-3 no-hover">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-start">
                        <div class="md:col-span-1 flex flex-col items-center justify-center text-center">
                            <img src="/pfp.webp" alt="Vicente" class="h-32 w-32 rounded-full object-cover border-4 border-emerald-500/30 mb-4" />
                            <h3 class="text-2xl font-bold text-slate-100">"Vicente"</h3>
                            <p class="text-md text-emerald-400 font-mono">"// Your Dedicated IT Partner"</p>
                            <div class="mt-6 w-full border-t border-slate-800/60 pt-4">
                                <div class="text-xs text-slate-500 font-mono mb-2">"// System Status"</div>
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
                                "Artorias Tech Lab is a one-man IT consultancy built to help small and medium-sized businesses operate smoothly. Instead of navigating frustrating call centers or waiting days for help, you get direct, rapid support."
                            </p>
                            <p class="text-slate-300 text-lg leading-relaxed">
                                "From setting up printers and securing your network, to redesigning your website and automating data entry, I handle the tech so you can focus on your business."
                            </p>
                        </div>
                    </div>
                </BentoBlock>
            </BentoGrid>

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

            // FAQ / Explanation Section
            <section class="max-w-4xl mx-auto mt-24 px-4 bento-scroll-card">
                <div class="text-center mb-16">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-950/30 border border-emerald-900/50 text-emerald-400 text-sm font-mono mb-6">
                        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                        "COMMON QUESTIONS"
                    </div>
                    <h2 class="text-3xl md:text-5xl font-extrabold text-slate-100 mb-6 uppercase tracking-tight">
                        "Frequently Asked " <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-cyan-400">"Questions"</span>
                    </h2>
                    <p class="text-lg text-slate-400 max-w-2xl mx-auto leading-relaxed">
                        "Everything you need to know about working with Artorias Tech Lab."
                    </p>
                </div>

                <div class="flex flex-col gap-6">
                    // Q1
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                            "01 // Do you provide remote or on-site support?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "The majority of our IT support is handled remotely for maximum speed and efficiency. Whether you need a printer set up, an email account recovered, or a network issue resolved, we can usually fix it without you waiting for a technician to drive to your office."
                        </p>
                    </div>

                    // Q2
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                            "02 // Why does my business need a custom domain?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "A custom domain (like yourbusiness.com) gives you professional, branded email addresses instead of using a standard @gmail.com account. It also allows us to set up secure SSL certificates for your website and custom branded links for your marketing, instantly building trust with your customers."
                        </p>
                    </div>

                    // Q3
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                            "03 // What does 'software automation' mean for my business?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "If you have employees spending hours copying and pasting data, manually sending emails, or re-typing invoices, we can write custom software to do it automatically. Automation eliminates tedious manual labor, reduces human error, and frees up your team to focus on actual work."
                        </p>
                    </div>

                    // Q4
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                            "04 // How does pricing work?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "It depends on the service. Ongoing IT support and website management are typically structured as a predictable monthly retainer. Custom projects like a total website redesign or building a custom automation tool are priced as a one-time project fee. We keep our pricing transparent and explicitly tailored for small to medium-sized businesses."
                        </p>
                    </div>
                </div>
            </section>

            
        </div>
    }
}
