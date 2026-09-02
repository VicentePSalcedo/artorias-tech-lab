use leptos::prelude::*;

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero
            <section class="relative min-h-[40vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-3xl mx-auto flex flex-col items-center">
                    <div class="text-sm font-mono text-emerald-400 mb-6 border border-emerald-500/30 bg-emerald-950/30 rounded-full px-4 py-1">
                        "LEGAL"
                    </div>
                    <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Privacy " <span class="animate-shimmer-amber">"Policy"</span>
                    </h1>
                    <p class="text-slate-400 max-w-xl font-light leading-relaxed">
                        "Last updated: September 2026. Plain-language answers about what we collect and why."
                    </p>
                </div>
            </section>

            // Privacy body
            <div class="max-w-3xl mx-auto px-4 w-full">
                <div class="rounded-2xl border border-slate-800/60 bg-slate-900/90 p-8 md:p-12 relative overflow-hidden bento-scroll-card">
                    <div class="absolute inset-0 bg-gradient-to-b from-emerald-950/5 to-transparent"></div>
                    <div class="relative z-10 flex flex-col gap-8">

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"1. The Short Version"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Artorias Tech Lab is a one-person IT practice. I don't run ads, I don't have a data warehouse, and I don't sell or rent anyone's information — ever. This site doesn't use tracking cookies or analytics. If you send me a message, it lands in my inbox. That's about it."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"2. What I Collect"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"From the contact form: your name, email address, phone number (if you provide it), and whatever you write in your message."</li>
                                <li>"From a subscription: your name, email, and billing address, processed by Stripe. I never see or store your card number."</li>
                                <li>"If we work together: whatever access and information is needed to provide the services — for example, admin accounts, device inventories, and configuration details."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"3. How I Use It"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"To reply to your message and answer your questions."</li>
                                <li>"To deliver the IT services you've asked for and to keep your systems running."</li>
                                <li>"To send you receipts, service notices, and important updates about your account."</li>
                                <li>"I don't send marketing newsletters. If that ever changes, you'll be asked to opt in first."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"4. Who I Share It With"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"Stripe — for payment processing. Stripe handles your card details under their own privacy and security policies."</li>
                                <li>"Resend — for delivering contact-form messages to my inbox."</li>
                                <li>"Vendors that are part of the services themselves — for example, Google Workspace or Microsoft 365 if your plan includes email management. Those are accounts you own."</li>
                                <li>"Beyond that: nothing. I don't share your information with third parties for their own purposes, and I don't sell data — full stop."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"5. Where It Lives"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Messages from the contact form go to my business email inbox. Subscription and payment records live in Stripe. Data related to services you've subscribed to lives on your own systems, in your own cloud accounts, or on equipment I manage on your behalf under your services agreement. I don't operate a separate customer database."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"6. How Long I Keep It"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Contact-form messages are kept as long as they're useful for our conversation, and deleted when you ask. Records needed for billing and tax purposes are kept for the period required by law. Service-related data is governed by your services agreement."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"7. How I Protect It"</h2>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"Your messages and documents are kept in secured accounts with strong passwords and two-factor authentication."</li>
                                <li>"Remote access to your systems uses encrypted, credential-based software with access logged."</li>
                                <li>"I follow the same security practices I recommend to my clients — because your data and mine deserve the same treatment."</li>
                                <li>"No method of transmission or storage is 100% secure. If a breach affecting your information ever occurs, I'll notify you promptly."</li>
                            </ul>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"8. Your Rights"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm mb-3">
                                "You can ask me, at any time and for any reason, to:"
                            </p>
                            <ul class="flex flex-col gap-2 text-sm text-slate-300 leading-relaxed list-disc pl-5">
                                <li>"Show you what information I have about you."</li>
                                <li>"Correct anything that's wrong."</li>
                                <li>"Delete your information, where I'm not required by law to keep it."</li>
                                <li>"Stop using your information for a particular purpose."</li>
                            </ul>
                            <p class="text-slate-300 leading-relaxed text-sm mt-3">
                                "Just call (904) 206-7198 or reply to any email from me. I'll handle it — no forms, no fine print."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"9. Children's Privacy"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "This site and these services are for businesses, not children. I don't knowingly collect information from anyone under 13, and if I learn that I have, I'll delete it."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"10. Changes to This Policy"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "If this policy changes in a significant way, I'll update the date above and notify you by email if the change affects you directly."
                            </p>
                        </div>

                        <div>
                            <h2 class="text-lg font-bold text-slate-100 mb-3 text-cyan-400">"11. Contact"</h2>
                            <p class="text-slate-300 leading-relaxed text-sm">
                                "Privacy questions? Call (904) 206-7198, 9am–5pm Monday–Friday, or use the contact form."
                            </p>
                        </div>

                    </div>
                </div>
            </div>
        </div>
    }
}
