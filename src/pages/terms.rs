use leptos::prelude::*;

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero
            <section class="relative min-h-[40vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-3xl mx-auto flex flex-col items-center">
                    <div class="text-sm font-mono text-emerald-400 mb-6 border border-emerald-500/30 bg-emerald-950/30 rounded-full px-4 py-1">
                        "LEGAL"
                    </div>
                    <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Terms of " <span class="animate-shimmer-amber">"Service"</span>
                    </h1>
                    <p class="text-slate-400 max-w-xl font-light leading-relaxed">
                        "Last updated: September 2026. Plain-language terms for working with Artorias Tech Lab."
                    </p>
                </div>
            </section>

            // Terms body
            <div class="max-w-3xl mx-auto px-4 w-full">
                <div class="rounded-2xl border border-slate-800/60 bg-slate-900/90 p-8 md:p-12 relative overflow-hidden bento-scroll-card">
                    <div class="absolute inset-0 bg-gradient-to-b from-emerald-950/5 to-transparent"></div>
                    <div class="relative z-10 flex flex-col gap-8">

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"1. The Services"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Artorias Tech Lab provides managed IT services to small businesses under a monthly plan (Starter, Growth, or Scale), or on a per-project and per-hour basis. The specific services included in each plan are described on the pricing page at artoriastechlab.com/pricing, which is part of these terms."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"2. Payment"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"Monthly plans are billed monthly in advance by credit card through Stripe."</li>
                                <li>"By subscribing, you authorize recurring charges each month until you cancel."</li>
                                <li>"Projects and sessions outside your plan are billed at $150/hr, with a $75 minimum for on-site visits (covering the first 30 minutes)."</li>
                                <li>"If a payment fails, I'll notify you. If payment isn't resolved within 10 days, services may be paused until the account is current."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"3. Cancellation and Refunds"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"No contracts. Cancel anytime by calling (904) 206-7198 or emailing the address on your receipt."</li>
                                <li>"Your first 30 days on any plan are fully refundable."</li>
                                <li>"Cancellation takes effect at the end of the current billing period. You're not charged after you cancel."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"4. What I Need From You"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm mb-3">
                                "I can only protect what I can reach. You agree to:"
                            </p>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"Provide access to the computers, accounts, and network equipment covered by your plan, including admin credentials where needed."</li>
                                <li>"Tell me about new devices, new users, or major changes to your setup so your coverage stays accurate."</li>
                                <li>"Keep physical hardware safe and inform me of moves or changes to your internet service."</li>
                                <li>"Keep your own independent copy of any data you consider irreplaceable (see section 7)."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"5. What's Not Included"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"Internet service, phone lines, and software licenses are not included in your plan unless stated."</li>
                                <li>"Hardware purchases (computers, printers, network gear) are billed at cost plus labor, and are owned by you."</li>
                                <li>"Work outside your plan's scope — new projects, on-site visits beyond plan coverage, additional users or devices beyond your tier's limit — is billed at $150/hr unless we agree otherwise in writing."</li>
                                <li>"Emergency response to problems caused by third parties (your internet provider, Microsoft, Google, or other vendors) is billed hourly if it falls outside your plan's normal support."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"6. Liability"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"Services are provided on a best-effort basis. I will act carefully and professionally, but I cannot guarantee that systems will never fail, that security will never be breached, or that no data will ever be lost."</li>
                                <li>"My total liability for any claim related to these services is limited to the amounts you paid in the 30 days before the claim."</li>
                                <li>"I am not liable for indirect or consequential damages, including lost profits, lost data, or business interruption."</li>
                                <li>"You agree to maintain backups of your critical data as part of your own risk management."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"7. Backups"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Backup oversight means I monitor that your backups run and test restores periodically. It is not a guarantee against data loss. You remain responsible for keeping an independent copy of any data that would be damaging to lose — for example, in a cloud service or on media you control."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"8. Confidentiality"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "I will treat your business data and credentials as confidential and use them only to provide these services. I will never sell or share your data. Access to your systems is limited to what the work requires."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"9. Security"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Security is a shared effort. I'll harden and monitor your systems, but no system is unhackable. You agree to follow basic security practices I recommend, including using strong passwords and not sharing credentials. If you're on a plan that includes email security, phishing protection applies to mailboxes covered by the plan."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"10. Changes to These Terms"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "I may update these terms from time to time. If the changes are significant, I'll notify you by email at least 14 days before they take effect. Continued use of the services after changes take effect means you accept the updated terms."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"11. Governing Law"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "These terms are governed by the laws of the State of Florida, and any disputes will be resolved in the courts of Duval County, Florida."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"12. Contact"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Questions about these terms? Call (904) 206-7198, 9am–5pm Monday–Friday."
                            </p>
                        </div>

                    </div>
                </div>
            </div>
        </div>
    }
}
