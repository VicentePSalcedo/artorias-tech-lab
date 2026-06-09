use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title, Meta};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::layout::AppLayout;
use crate::pages::index::HomePage;
use crate::pages::services::ServicesPage;
use crate::pages::founder::FounderPage;
use crate::pages::products::index::ProductsIndex;
use crate::pages::products::renivel::RenivelPage;
use crate::pages::contact::ContactPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="icon" type="image/webp" href="/icon.webp"/>
                <script src="/lenis.min.js"></script>
                <MetaTags/>
            </head>
            <body>
                <App/>
                <script>
                    "document.addEventListener('DOMContentLoaded', () => {
                        const visibleCards = new Set();
                        const observedCards = new Set();
                        
                        // Initialize Lenis smooth scroll
                        const lenis = new Lenis({
                            duration: 1.2,
                            easing: (t) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
                            smoothWheel: true,
                            touchMultiplier: 1.5,
                        });

                        function raf(time) {
                            lenis.raf(time);
                            requestAnimationFrame(raf);
                        }

                        requestAnimationFrame(raf);

                        function updateProgress() {
                            const viewportHeight = window.innerHeight;
                            const viewportCenter = viewportHeight / 2;
                            const scrollY = window.scrollY || window.pageYOffset;
                            const totalHeight = document.documentElement.scrollHeight;
                            const isAtBottom = (viewportHeight + scrollY) >= (totalHeight - 80);
                            
                            visibleCards.forEach(card => {
                                if (!document.body.contains(card)) {
                                    visibleCards.delete(card);
                                    observedCards.delete(card);
                                    return;
                                }

                                const rect = card.getBoundingClientRect();
                                
                                if (card.classList.contains('bento-scroll-card')) {
                                    if (isAtBottom) {
                                        card.style.setProperty('--scroll-progress', '1.000');
                                    } else {
                                        const startScroll = viewportHeight;
                                        const endScroll = viewportHeight * 0.25;
                                        let progress = (startScroll - rect.top) / (startScroll - endScroll);
                                        progress = Math.max(0, Math.min(1, progress));
                                        card.style.setProperty('--scroll-progress', progress.toFixed(3));
                                    }
                                }
                            });
                        }

                        const intersectionObserver = new IntersectionObserver((entries) => {
                            let statusChanged = false;
                            entries.forEach(entry => {
                                if (entry.isIntersecting) {
                                    visibleCards.add(entry.target);
                                    statusChanged = true;
                                } else {
                                    visibleCards.delete(entry.target);
                                    if (entry.target.classList.contains('bento-scroll-card')) {
                                        entry.target.style.setProperty('--scroll-progress', '0');
                                    }
                                }
                            });
                            if (statusChanged) {
                                updateProgress();
                            }
                        }, {
                            threshold: 0,
                            rootMargin: '100px 0px 100px 0px'
                        });

                        function scanAndObserve() {
                            const cards = document.querySelectorAll('.bento-scroll-card, .bento-card');
                            cards.forEach(card => {
                                if (!observedCards.has(card)) {
                                    observedCards.add(card);
                                    intersectionObserver.observe(card);
                                }
                            });
                            updateProgress();
                        }

                        const mutationObserver = new MutationObserver((mutations) => {
                            let hasNewCards = false;
                            mutations.forEach(mutation => {
                                mutation.addedNodes.forEach(node => {
                                    if (node.nodeType === Node.ELEMENT_NODE) {
                                        if (node.classList && (node.classList.contains('bento-scroll-card') || node.classList.contains('bento-card'))) {
                                            hasNewCards = true;
                                        } else if (node.querySelector && node.querySelector('.bento-scroll-card, .bento-card')) {
                                            hasNewCards = true;
                                        }
                                    }
                                });
                            });
                            if (hasNewCards) {
                                scanAndObserve();
                            }
                        });

                        mutationObserver.observe(document.body, {
                            childList: true,
                            subtree: true
                        });

                        scanAndObserve();

                        // Hook Lenis scroll events to trigger animations
                        lenis.on('scroll', () => {
                            updateProgress();
                        });

                        window.addEventListener('scroll', () => {
                            updateProgress();
                        });
                    });"
                </script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/artorias-tech-lab.css"/>
        <Title text="Artorias Tech Lab | AI Search Visibility & Web Platforms"/>
        <Meta name="description" content="AI Search Optimization & premium web systems to ensure your business, products, and services are recommended by ChatGPT, Claude, and Perplexity."/>
        <Meta name="keywords" content="AI SEO, AI Search Visibility, Custom Web Platforms, Digital Architect, ChatGPT SEO, Perplexity Optimization"/>

        <Router>
            <AppLayout>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("founder") view=FounderPage/>
                    <Route path=StaticSegment("services") view=ServicesPage/>
                    <Route path=StaticSegment("contact") view=ContactPage/>
                    <Route path=StaticSegment("products") view=ProductsIndex/>
                    <Route path=(StaticSegment("products"), StaticSegment("renivel")) view=RenivelPage/>
                </Routes>
            </AppLayout>
        </Router>
    }
}
