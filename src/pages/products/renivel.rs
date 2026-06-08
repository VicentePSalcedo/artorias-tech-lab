use leptos::prelude::*;

#[component]
pub fn RenivelPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-24 py-12">
            // Hero Section
            <section class="text-center">
                <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6">
                    "Keep Your Money "
                    <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-cyan-400">
                        "Straight."
                    </span>
                </h1>
                <p class="text-xl text-slate-400 max-w-2xl mx-auto mb-8">
                    "The \"Lite ERP\" for residential remodelers. Stop doing math on napkins and instantly see if you’re making a profit."
                </p>
                <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
                    <a href="https://reniveltool.com/signup" target="_blank" class="px-8 py-3 rounded-md font-bold bg-cyan-500 text-slate-900 hover:bg-cyan-400 transition-colors">
                        "Start 30-Day Free Trial"
                    </a>
                </div>
            </section>

            // Problem Section
            <section class="grid md:grid-cols-2 gap-12 items-center bg-slate-900/30 p-8 rounded-3xl border border-slate-800/50 bento-scroll-card">
                <div>
                    <h2 class="text-3xl md:text-4xl font-bold text-slate-100 mb-6">"The Guesswork is Costing You Money."</h2>
                    <p class="text-lg text-slate-400 mb-6">
                        "You wouldn’t frame a wall without a level. You need to know if your work is 'true.' But most contractors run their business finances on 'gut feeling' or math scribbled on a scrap of drywall."
                    </p>
                </div>
                <div class="bg-slate-950/80 p-8 rounded-2xl border border-rose-900/30 shadow-xl">
                    <h3 class="text-xl font-bold text-slate-200 mb-6">"The Reality:"</h3>
                    <ul class="space-y-4">
                        <li class="flex items-start gap-3">
                            <span class="text-rose-500 mt-1">"✕"</span>
                            <span class="text-slate-400">"Lost Receipts: Thousands of dollars washing around in your dashboard."</span>
                        </li>
                        <li class="flex items-start gap-3">
                            <span class="text-rose-500 mt-1">"✕"</span>
                            <span class="text-slate-400">"Scope Creep: Doing extra work for free because you didn't track the Change Order."</span>
                        </li>
                        <li class="flex items-start gap-3">
                            <span class="text-rose-500 mt-1">"✕"</span>
                            <span class="text-slate-400">"Blind Flying: Not knowing if you made money until the job is done."</span>
                        </li>
                    </ul>
                </div>
            </section>

            // Solution Section
            <section class="text-center">
                <h2 class="text-3xl md:text-5xl font-bold text-slate-100 mb-6">"A Spirit Level for Your Business."</h2>
                <p class="text-lg text-slate-400 max-w-3xl mx-auto mb-12">
                    "RENIVEL isn't a complicated accounting app. It’s a visual tool designed for the job site. We replaced complex spreadsheets with a simple Health Bar—just like the bubble in your level."
                </p>
                <div class="grid md:grid-cols-2 gap-6 max-w-2xl mx-auto">
                    <div class="flex items-center gap-4 rounded-xl border border-emerald-900/50 bg-emerald-950/20 p-6 bento-scroll-card">
                        <div class="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center text-emerald-400">"✓"</div>
                        <div class="text-left">
                            <p class="font-bold text-emerald-400">"Green"</p>
                            <p class="text-sm text-slate-400">"You are balanced. Your job is profitable."</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-4 rounded-xl border border-rose-900/50 bg-rose-950/20 p-6 bento-scroll-card">
                        <div class="w-12 h-12 rounded-full bg-rose-500/20 flex items-center justify-center text-rose-400">"✕"</div>
                        <div class="text-left">
                            <p class="font-bold text-rose-400">"Red"</p>
                            <p class="text-sm text-slate-400">"You are off-track. You are over budget."</p>
                        </div>
                    </div>
                </div>
            </section>

            // Feature Grid
            <section>
                <h2 class="text-3xl font-bold text-slate-100 text-center mb-12">"The Toolbelt for Profit."</h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <div class="bg-slate-900/50 border border-slate-800 p-6 rounded-2xl hover:border-cyan-500/50 transition-colors bento-scroll-card">
                        <h3 class="text-lg font-bold text-slate-200 mb-2">"Real-Time Health Bar"</h3>
                        <p class="text-sm text-slate-400">"See your P&L instantly. Green means profitable, Red means stop working and fix it."</p>
                    </div>
                    <div class="bg-slate-900/50 border border-slate-800 p-6 rounded-2xl hover:border-cyan-500/50 transition-colors bento-scroll-card">
                        <h3 class="text-lg font-bold text-slate-200 mb-2">"Zero-Training Worker App"</h3>
                        <p class="text-sm text-slate-400">"Field staff can log time and receipts in under 10 seconds. No complex menus."</p>
                    </div>
                    <div class="bg-slate-900/50 border border-slate-800 p-6 rounded-2xl hover:border-cyan-500/50 transition-colors bento-scroll-card">
                        <h3 class="text-lg font-bold text-slate-200 mb-2">"Change Order Defense"</h3>
                        <p class="text-sm text-slate-400">"Never let scope creep kill your margin again. Track every extra request instantly."</p>
                    </div>
                    <div class="bg-slate-900/50 border border-slate-800 p-6 rounded-2xl hover:border-cyan-500/50 transition-colors bento-scroll-card">
                        <h3 class="text-lg font-bold text-slate-200 mb-2">"Multi-Property CRM"</h3>
                        <p class="text-sm text-slate-400">"Manage clients with multiple properties (e.g., Main House vs. Rental) effortlessly."</p>
                    </div>
                </div>
            </section>

            // CTA / Promo
            <section class="bg-gradient-to-br from-slate-900 to-cyan-950/30 rounded-3xl border border-cyan-900/50 p-12 text-center bento-scroll-card">
                <h2 class="text-3xl md:text-4xl font-bold text-slate-100 mb-6">"Ready to Level Up?"</h2>
                <div class="inline-block px-4 py-2 bg-slate-950/50 border border-cyan-800 rounded-lg mb-8">
                    <p class="text-cyan-400 font-mono text-sm">"Use code: EARLYACCESS"</p>
                </div>
                <p class="text-lg text-slate-300 max-w-2xl mx-auto mb-8">
                    "First 500 Only - 75% Off For Life ($22.50/mo). Offer ends soon. Unlimited workers. Your first 30 days are free."
                </p>
                <a href="https://reniveltool.com/signup" target="_blank" class="inline-block px-8 py-3 rounded-md font-bold bg-cyan-500 text-slate-900 hover:bg-cyan-400 transition-colors">
                    "START FREE TRIAL"
                </a>
            </section>
        </div>
    }
}
