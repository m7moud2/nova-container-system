// Particle.js Configuration
particlesJS('particles-js', {
    particles: {
        number: { value: 80, density: { enable: true, value_area: 800 } },
        color: { value: '#06b6d4' },
        shape: { type: 'circle' },
        opacity: { value: 0.3, random: true },
        size: { value: 3, random: true },
        line_linked: {
            enable: true,
            distance: 150,
            color: '#06b6d4',
            opacity: 0.2,
            width: 1
        },
        move: {
            enable: true,
            speed: 2,
            direction: 'none',
            random: false,
            straight: false,
            out_mode: 'out',
            bounce: false
        }
    },
    interactivity: {
        detect_on: 'canvas',
        events: {
            onhover: { enable: true, mode: 'grab' },
            onclick: { enable: true, mode: 'push' },
            resize: true
        },
        modes: {
            grab: { distance: 140, line_linked: { opacity: 0.5 } },
            push: { particles_nb: 4 }
        }
    },
    retina_detect: true
});

// Initialize AOS
AOS.init({
    duration: 800,
    easing: 'ease-out',
    once: true,
    offset: 100
});

// Smooth Scrolling
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }
    });
});

// Navbar Scroll Effect
let lastScroll = 0;
const navbar = document.querySelector('.navbar');

window.addEventListener('scroll', () => {
    const currentScroll = window.pageYOffset;

    if (currentScroll > 100) {
        navbar.style.background = 'rgba(10, 10, 10, 0.95)';
        navbar.style.boxShadow = '0 4px 30px rgba(0, 0, 0, 0.3)';
    } else {
        navbar.style.background = 'rgba(10, 10, 10, 0.8)';
        navbar.style.boxShadow = 'none';
    }

    lastScroll = currentScroll;
});

// Animated Counter
function animateCounter(element) {
    const target = parseInt(element.getAttribute('data-target'));
    const duration = 2000;
    const step = target / (duration / 16);
    let current = 0;

    const timer = setInterval(() => {
        current += step;
        if (current >= target) {
            element.textContent = target;
            clearInterval(timer);
        } else {
            element.textContent = Math.floor(current);
        }
    }, 16);
}

// Trigger counters on scroll
const counterObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const counter = entry.target;
            if (!counter.classList.contains('counted')) {
                animateCounter(counter);
                counter.classList.add('counted');
            }
        }
    });
}, { threshold: 0.5 });

document.querySelectorAll('.counter').forEach(counter => {
    counterObserver.observe(counter);
});

// Live Terminal Demo
const terminalOutput = document.getElementById('terminal-output');
const terminalCommands = [
    { delay: 1000, text: '<div class="terminal-line"><span class="prompt">$</span> <span class="command">nova run app.wasm --replicas 100</span></div>' },
    { delay: 500, text: '<div class="terminal-line" style="color: #10b981;">🚀 Scheduler: Spawning 100 replicas...</div>' },
    { delay: 300, text: '<div class="terminal-line" style="color: #06b6d4;">📦 Module loaded in 10.71ms</div>' },
    { delay: 300, text: '<div class="terminal-line" style="color: #06b6d4;">🔧 Instantiated in 239.67µs</div>' },
    { delay: 200, text: '<div class="terminal-line" style="color: #10b981;">✅ Replica #0 running</div>' },
    { delay: 100, text: '<div class="terminal-line" style="color: #10b981;">✅ Replica #1 running</div>' },
    { delay: 100, text: '<div class="terminal-line" style="color: #10b981;">✅ Replica #2 running</div>' },
    { delay: 100, text: '<div class="terminal-line" style="color: #a1a1aa;">... (97 more)</div>' },
    { delay: 500, text: '<div class="terminal-line" style="color: #10b981;">✅ All replicas completed</div>' },
    { delay: 300, text: '<div class="terminal-line" style="color: #a1a1aa;">⚡ Total time: 24ms</div>' },
    { delay: 1000, text: '<div class="terminal-line"><span class="prompt">$</span> <span class="command" style="animation: blink 1s infinite;">_</span></div>' }
];

function runTerminalDemo() {
    let currentIndex = 0;

    function addLine() {
        if (currentIndex < terminalCommands.length) {
            const cmd = terminalCommands[currentIndex];
            setTimeout(() => {
                terminalOutput.innerHTML += cmd.text;
                terminalOutput.scrollTop = terminalOutput.scrollHeight;
                currentIndex++;
                addLine();
            }, cmd.delay);
        }
    }

    addLine();
}

// Start terminal demo when visible
const terminalObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting && !entry.target.classList.contains('demo-started')) {
            entry.target.classList.add('demo-started');
            runTerminalDemo();
        }
    });
}, { threshold: 0.3 });

if (terminalOutput) {
    terminalObserver.observe(terminalOutput);
}

// Animate comparison bars
const comparisonObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const bars = entry.target.querySelectorAll('.comparison-bar');
            bars.forEach(bar => {
                const width = bar.getAttribute('data-width');
                bar.style.setProperty('--bar-width', width + '%');
            });
        }
    });
}, { threshold: 0.5 });

document.querySelectorAll('.comparison-card').forEach(card => {
    comparisonObserver.observe(card);
});

// Copy Code Function
function copyCode(button) {
    const codeBlock = button.closest('.code-block').querySelector('code');
    const text = codeBlock.textContent;

    navigator.clipboard.writeText(text).then(() => {
        const icon = button.querySelector('i');
        icon.classList.remove('fa-copy');
        icon.classList.add('fa-check');
        button.style.color = '#06b6d4';

        setTimeout(() => {
            icon.classList.remove('fa-check');
            icon.classList.add('fa-copy');
            button.style.color = '';
        }, 2000);
    });
}

// Interactive Playground
function runCode() {
    const output = document.getElementById('playground-output');
    output.innerHTML = '<div style="color: #06b6d4;"><i class="fas fa-spinner fa-spin me-2"></i>Compiling...</div>';

    setTimeout(() => {
        output.innerHTML = `
            <div style="color: #10b981; margin-bottom: 8px;"><i class="fas fa-check-circle me-2"></i>Compilation successful</div>
            <div style="color: #06b6d4; margin-bottom: 8px;">📦 Module loaded in 8.2ms</div>
            <div style="color: #06b6d4; margin-bottom: 8px;">🔧 Instantiated in 187µs</div>
            <div style="margin-top: 16px; padding: 12px; background: rgba(6, 182, 212, 0.1); border-left: 3px solid #06b6d4; border-radius: 4px;">
                <div style="font-weight: 600; margin-bottom: 4px;">Output:</div>
                <div>Hello from Nova! 🚀</div>
            </div>
            <div style="color: #a1a1aa; margin-top: 16px;">⚡ Execution time: 421µs</div>
        `;
    }, 1500);
}

// Intersection Observer for Animations
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -100px 0px'
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
        }
    });
}, observerOptions);

// Console Branding
console.log('%c🚀 Nova Container System', 'font-size: 24px; font-weight: bold; color: #06b6d4; text-shadow: 0 0 10px rgba(6, 182, 212, 0.5);');
console.log('%cBuilt with Rust & WebAssembly', 'font-size: 16px; color: #8b5cf6;');
console.log('%cPerformance: 1000x faster than Docker', 'font-size: 14px; color: #10b981;');
console.log('%c\nInterested in contributing? Visit https://github.com/nova-container', 'font-size: 12px; color: #a1a1aa;');
