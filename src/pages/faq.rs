use leptos::prelude::*;
use crate::components::layout::STRIPE_PORTAL_URL;

#[component]
pub fn FaqPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section
            <section class="relative min-h-[60vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-4xl mx-auto flex flex-col items-center">
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Questions, " <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-cyan-400">"Answered"</span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light leading-relaxed">
                        "Everything you need to know about working with Artorias Tech Lab."
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

            <div class="max-w-3xl mx-auto px-4 w-full flex flex-col gap-6">

                // Section header: business questions
                <div class="pt-6 flex justify-center">
                    <div class="inline-flex items-center gap-2 px-5 py-2 rounded-full bg-cyan-950/30 border border-cyan-900/50 text-cyan-400 text-sm font-mono">
                        <span class="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
                        "THE BUSINESS SIDE — MONEY, CONTRACTS, HOW IT WORKS"
                    </div>
                </div>

                // Q1
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "01 // How does pricing work?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Managed IT is a flat monthly fee billed by whichever you hit first: 3 users or 9 devices ($299), 8 users or 24 devices ($599), 15 users or 45 devices ($999). Device-heavy businesses like retail or restaurants can get a per-device quote instead. Everything outside your plan is billed hourly at $150/hr. Sessions and on-site visits are billed the same way: a flat $150/hr, with a $75 minimum that covers your first 30 minutes — whether the job takes 20 minutes or three hours, the math is the same. Call (904) 206-7198 to schedule. Projects like Google Workspace email setup are flat one-time fees — $299 for up to 5 mailboxes — with ongoing management at $5/mailbox/month. No contracts — cancel anytime."
                    </p>
                </div>

                // Q2
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "02 // Do I need to sign a contract?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "No contracts, ever. You can cancel anytime, and your first 30 days on any plan are fully refundable. If we're not making your life measurably easier in the first month, you shouldn't pay for it."
                    </p>
                </div>

                // Q3
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "03 // What happens after I subscribe?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Within 24 hours of subscribing I'll email you to schedule your setup call. We walk through your computers, network, and what's currently protected, then install the remote support agent so you can reach me directly. Monitoring, patching, and backup oversight run in the background from day one."
                    </p>
                </div>

                // Q4
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "04 // Do you provide remote or on-site support?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Tech breaks at the worst possible moment — and you shouldn't have to wait for a technician to drive across town. The majority of our IT support is handled remotely for maximum speed and efficiency. Whether you need a printer set up, an email account recovered, or a network issue resolved, we can usually fix it while you keep working. When coming out to your office is the right call, there's a $75 minimum — that covers your first 30 minutes on site — and a flat $150/hr after that. No session packages, no tiered rates: it's just $150/hr, with a 30-minute minimum."
                    </p>
                </div>

                // Q5
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "05 // What are your business hours?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Monday through Friday, 9am to 5pm Eastern. Call (904) 206-7198 anytime during those hours — I answer my own phone. After-hours calls go to voicemail and the booking form, and I'll get back to you the next business day."
                    </p>
                </div>

                // Q6
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "06 // How do I update my payment method or cancel my plan?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Anytime, through the "
                        <a href={STRIPE_PORTAL_URL} class="text-cyan-400 hover:text-cyan-300 transition-colors">"Stripe Customer Portal"</a>
                        " — the same link is on every receipt. Update your card, change plans, or cancel with a few clicks, no phone call needed. Prefer to talk it through? Call (904) 206-7198."
                    </p>
                </div>

                // Section header: technical questions
                <div class="pt-6 flex justify-center">
                    <div class="inline-flex items-center gap-2 px-5 py-2 rounded-full bg-emerald-950/30 border border-emerald-900/50 text-emerald-400 text-sm font-mono">
                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                        "THE TECH, IN PLAIN ENGLISH"
                    </div>
                </div>

                // Q7
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "07 // What is endpoint security (EDR)?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Endpoint security (EDR) is the modern replacement for antivirus. Instead of only catching known viruses, it watches what programs actually do on your computers and can flag suspicious behavior — like files being encrypted in bulk, the classic ransomware move — and isolate the affected machine before it spreads to your network. Every Growth and Scale plan includes it."
                    </p>
                </div>

                // Q8
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "08 // What does email security protect me from?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Email is how most attacks actually reach your business. Email security blocks phishing and impersonation scams — including fake 'your CEO needs gift cards' requests and vendor payment-diversion fraud — checks links and attachments in a safe sandbox before anyone clicks them, and hardens your domain with SPF, DKIM, and DMARC so criminals can't send email pretending to be you."
                    </p>
                </div>

                // Q9
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "09 // What does backup oversight actually mean?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "Having backup software installed is not the same as having recoverable data. Backup oversight means every backup job is verified daily, restores are actually tested on a schedule, and your data follows the 3-2-1 rule — three copies, two formats, one offsite. If the worst happens, we know the backup works because we've already restored from it."
                    </p>
                </div>

                // Q10
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "10 // Why does my business need a custom domain?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "A custom domain (like yourbusiness.com) gives you professional, branded email addresses instead of a standard @gmail.com account. I handle the full setup — Google Workspace or Microsoft 365, mailbox migration included — so you keep professional email with zero downtime. I also set up SSL certificates and custom branded links for your marketing."
                    </p>
                </div>

                // Q11
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-emerald-500/30 transition-all duration-300 bento-scroll-card">
                    <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-emerald-400">
                        "11 // What does 'software automation' mean for my business?"
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        "If you have employees spending hours copying and pasting data, manually sending emails, or re-typing invoices, we can write custom software to do it automatically. Automation eliminates tedious manual labor, reduces human error, and frees up your team to focus on actual work."
                    </p>
                </div>

                // CTA
                <div class="mt-8 p-8 border border-slate-800/80 bg-slate-900/30 rounded-3xl text-center bento-scroll-card">
                    <h2 class="text-2xl font-bold text-slate-100 mb-2">"Still have questions?"</h2>
                    <p class="text-slate-400 mb-6">"Call (904) 206-7198 — I answer my own phone, 9am–5pm Mon–Fri."</p>
                    <a href="tel:9042067198" class="px-8 py-3 bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold rounded-lg transition-colors font-mono text-sm">
                        "Call (904) 206-7198"
                    </a>
                </div>
            </div>
        </div>
    }
}
