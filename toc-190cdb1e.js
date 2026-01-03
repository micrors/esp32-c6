// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><li class="part-title">Read This</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="index.html"><strong aria-hidden="true">1.</strong> Before Starting</a></span></li><li class="chapter-item expanded "><li class="part-title">Getting Started</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_1/definitions.html"><strong aria-hidden="true">2.</strong> Embedded Basics</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_1/hw_and_sw.html"><strong aria-hidden="true">3.</strong> Set Up</a></span></li><li class="chapter-item expanded "><li class="part-title">First Projects</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_2/boilerplate.html"><strong aria-hidden="true">4.</strong> Boilerplate</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_2/hello_world.html"><strong aria-hidden="true">5.</strong> Hello World</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_2/panic.html"><strong aria-hidden="true">6.</strong> Panic</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_2/blinky.html"><strong aria-hidden="true">7.</strong> Blinky</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_2/handle_input.html"><strong aria-hidden="true">8.</strong> Handle Input</a></span></li><li class="chapter-item expanded "><li class="part-title">Debugging</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_3/logging_0.html"><strong aria-hidden="true">9.</strong> Logging I</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_3/logging_1.html"><strong aria-hidden="true">10.</strong> Logging II</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_3/gdb_set_up.html"><strong aria-hidden="true">11.</strong> Debugger - Set Up</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_3/gdb.html"><strong aria-hidden="true">12.</strong> Using GDB</a></span></li><li class="chapter-item expanded "><li class="part-title">Protocols</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_4/what_for.html"><strong aria-hidden="true">13.</strong> Intro</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_4/uart_1.html"><strong aria-hidden="true">14.</strong> UART I</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_4/uart_2.html"><strong aria-hidden="true">15.</strong> UART II</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_4/i2c.html"><strong aria-hidden="true">16.</strong> I2C</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">17.</strong> SPI</span></span></li><li class="chapter-item expanded "><li class="part-title">Appendix</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ch_3/gdb_commands.html"><strong aria-hidden="true">18.</strong> GDB Commands</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="other_resources.html"><strong aria-hidden="true">19.</strong> Other Resources</a></span></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);

