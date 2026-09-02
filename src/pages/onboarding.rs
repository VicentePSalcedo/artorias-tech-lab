use leptos::prelude::*;
use crate::components::layout::STRIPE_PORTAL_URL;

#[component]
pub fn OnboardingPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero
            <section class="relative min-h-[60vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-3xl mx-auto flex flex-col items-center">
                    <div class="text-sm font-mono text-emerald-400 mb-6 border border-emerald-500/30 bg-emerald-950/30 rounded-full px-4 py-1">
                        "SUBSCRIPTION CONFIRMED"
                    </div>
                    <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "You're " <span class="text-emerald-400">"In."</span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-xl font-light leading-relaxed">
                        "Your managed IT plan is active. The tech is handled from here — you get back to your business. Here's exactly what happens next."
                    </p>
                </div>
            </section>

            <div class="max-w-3xl mx-auto px-4 w-full flex flex-col gap-5">
                // Step 1
                <div class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-6 flex gap-5 bento-scroll-card">
                    <div class="text-emerald-400 font-mono font-bold text-lg flex-shrink-0">"01"</div>
                    <div>
                        <h2 class="font-bold text-slate-100 mb-1">"Check your email"</h2>
                        <p class="text-slate-400 text-sm leading-relaxed">"Stripe just sent you a receipt. I'll email you separately within 24 hours to schedule your setup call."</p>
                    </div>
                </div>

                // Step 2
                <div class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-6 flex gap-5 bento-scroll-card">
                    <div class="text-emerald-400 font-mono font-bold text-lg flex-shrink-0">"02"</div>
                    <div>
                        <h2 class="font-bold text-slate-100 mb-1">"We schedule your setup call"</h2>
                        <p class="text-slate-400 text-sm leading-relaxed">"A 15-minute call to walk through your computers, network, and what's currently protected — and what isn't."</p>
                    </div>
                </div>

                // Step 3
                <div class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-6 flex gap-5 bento-scroll-card">
                    <div class="text-emerald-400 font-mono font-bold text-lg flex-shrink-0">"03"</div>
                    <div>
                        <h2 class="font-bold text-slate-100 mb-1">"Remote support goes live"</h2>
                        <p class="text-slate-400 text-sm leading-relaxed">"We install the support agent on your devices and connect them to your secure client portal. You can reach me directly — no call centers, no ticket queues."</p>
                    </div>
                </div>

                // Step 4
                <div class="border border-slate-800/80 bg-slate-900/90 rounded-2xl p-6 flex gap-5 bento-scroll-card">
                    <div class="text-emerald-400 font-mono font-bold text-lg flex-shrink-0">"04"</div>
                    <div>
                        <h2 class="font-bold text-slate-100 mb-1">"You're covered"</h2>
                        <p class="text-slate-400 text-sm leading-relaxed">"Monitoring, patching, and backup oversight run in the background. No contracts — cancel anytime, and your first 30 days are fully refundable."</p>
                    </div>
                </div>

                // Contact strip
                <div class="border border-emerald-500/30 bg-emerald-950/20 rounded-2xl p-6 text-center bento-scroll-card">
                    <p class="text-slate-300 text-sm mb-2">"Questions before then? Call or text me directly:"</p>
                    <a href="tel:9042067198" class="text-2xl font-bold text-emerald-400 hover:text-emerald-300 transition-colors">"904-206-7198"</a>
                </div>

                <div class="text-center text-xs text-slate-500 px-4">
                    "Billing questions? Update your payment method or cancel anytime through the "
                    <a href={STRIPE_PORTAL_URL} class="text-cyan-400 hover:text-cyan-300 transition-colors">"Stripe Customer Portal"</a>
                    " — no phone call needed."
                </div>

                <a href="/" class="text-center px-6 py-3 bg-slate-900 border border-slate-700 hover:border-emerald-500 text-slate-200 font-bold rounded-lg transition-colors font-mono text-sm">
                    "Return Home"
                </a>
            </div>
        </div>
    }
}
