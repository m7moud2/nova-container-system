// Check authentication on page load
window.addEventListener('DOMContentLoaded', () => {
    const token = localStorage.getItem('nova_token');
    if (!token) {
        window.location.href = '/login.html';
        return;
    }

    loadUserData();
    fetchStats();
    fetchProjects();
    if (window.location.pathname.includes('billing.html')) {
        fetchBilling();
    }

    // Refresh everything for that "live" feel
    setInterval(fetchStats, 2000);
    setInterval(fetchProjects, 10000);
    if (window.location.pathname.includes('billing.html')) {
        setInterval(fetchBilling, 30000);
    }
});

// Helper function to make authenticated requests
async function fetchWithAuth(url, options = {}) {
    const token = localStorage.getItem('nova_token');
    const headers = {
        ...options.headers,
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
    };

    const response = await fetch(url, { ...options, headers });

    // Handle unauthorized (expired token)
    if (response.status === 401) {
        localStorage.removeItem('nova_token');
        localStorage.removeItem('nova_user');
        window.location.href = '/login.html';
        return null;
    }

    return response;
}

async function fetchBilling() {
    try {
        const res = await fetchWithAuth('/api/billing');
        if (!res || !res.ok) return;
        const data = await res.json();

        // Update billing stats
        const balEl = document.getElementById('billing-balance');
        const useEl = document.getElementById('billing-usage');
        const invEl = document.getElementById('billing-invoice');
        if (balEl && data.balance) balEl.textContent = data.balance;
        if (useEl && data.usage_mtd) useEl.textContent = data.usage_mtd;
        if (invEl && data.next_invoice) invEl.textContent = data.next_invoice;

        // Update cards
        const cardContainer = document.getElementById('dynamicCards');
        if (cardContainer && data.cards) {
            cardContainer.innerHTML = data.cards.map(card => `
                <div style="margin-top: 1rem; display: flex; align-items: center; gap: 1rem; background: var(--bg-dark); padding: 1rem; border-radius: 12px; border: 1px solid var(--border);">
                    <i class="fab fa-cc-${card.brand}" style="font-size: 2rem; color: #fff;"></i>
                    <div style="flex-grow: 1;">
                        <strong>${card.brand.toUpperCase()} ending in ${card.last4}</strong>
                        <p style="font-size: 0.8rem; color: var(--text-muted);">Expires ${card.exp}</p>
                    </div>
                    ${card.is_default ? '<span class="badge" style="background: rgba(59, 130, 246, 0.1); color: var(--primary);">Default</span>' : ''}
                    <button class="btn-outline" style="padding: 6px 12px; font-size: 0.8rem;" onclick="removeCard(event, '${card.id}')">Remove</button>
                </div>
            `).join('');
        }

        // Render Usage History Simple Visual
        const main = document.querySelector('.main-content');
        if (main && data.history && !document.getElementById('usageChart')) {
            const chartHtml = `
                <div class="card" id="usageChart" style="margin-top: 2rem;">
                    <h2>Consumption History</h2>
                    <div style="display: flex; align-items: flex-end; gap: 10px; height: 150px; margin-top: 2rem; padding-bottom: 20px; border-bottom: 1px solid var(--border);">
                        ${data.history.map(h => `
                            <div style="flex: 1; display: flex; flex-direction: column; align-items: center; gap: 8px;">
                                <div style="width: 100%; background: linear-gradient(to top, var(--primary), #8b5cf6); border-radius: 4px 4px 0 0; height: ${h.amount * 20}px; transition: height 0.5s ease;"></div>
                                <span style="font-size: 0.7rem; color: var(--text-muted);">${h.date.split(' ')[1]}</span>
                            </div>
                        `).join('')}
                    </div>
                </div>
            `;
            main.insertAdjacentHTML('beforeend', chartHtml);
        }
    } catch (err) {
        console.error('Failed to fetch billing:', err);
    }
}

async function controlProject(id, action) {
    const btn = event?.target?.closest('button');
    const originalContent = btn ? btn.innerHTML : null;

    if (!confirm(`Are you sure you want to ${action} this project?`)) return;

    if (btn) {
        btn.disabled = true;
        btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
    }

    try {
        const res = await fetchWithAuth('/api/deployments/control', {
            method: 'POST',
            body: JSON.stringify({ project_id: id, action: action })
        });

        if (res && res.ok) {
            const data = await res.json();
            // Show a subtle toast or alert
            console.log(`Action ${action} successful:`, data);
            fetchProjects(); // Refresh UI
        }
    } catch (err) {
        alert(`Failed to ${action} project. Please check system status.`);
    } finally {
        if (btn) {
            btn.disabled = false;
            btn.innerHTML = originalContent;
        }
    }
}

async function removeCard(event, id) {
    if (!confirm('Are you sure you want to remove this payment method?')) return;
    const cardEl = event.target.closest('div[style*="margin-top: 1rem"]');
    if (cardEl) {
        cardEl.remove();
    }
    alert('✅ Payment method removed.');
}

// Global copy utility
function copyCode(btn) {
    const codeBlock = btn.closest('.code-block').querySelector('pre code') || btn.closest('.code-block').querySelector('pre');
    if (!codeBlock) return;
    navigator.clipboard.writeText(codeBlock.innerText).then(() => {
        const icon = btn.querySelector('i');
        if (icon) {
            icon.className = 'fas fa-check text-success';
            setTimeout(() => { icon.className = 'fas fa-copy'; }, 2000);
        }
    });
}

// Load user data
async function loadUserData() {
    try {
        const res = await fetchWithAuth('/api/me');
        if (!res) return;

        const user = await res.json();
        localStorage.setItem('nova_user', JSON.stringify(user));

        // Update UI info
        const nameEls = document.querySelectorAll('.user-info .name');
        nameEls.forEach(el => el.textContent = user.name);

        const avatars = [
            document.getElementById('sidebarAvatar'),
            document.getElementById('settingsAvatar'),
            document.querySelector('.sidebar img')
        ];

        const avatarUrl = `https://ui-avatars.com/api/?name=${encodeURIComponent(user.name)}&background=3b82f6&color=fff&bold=true`;
        avatars.forEach(el => {
            if (el) el.src = avatarUrl;
        });

        // Update settings form if exists
        const settingName = document.getElementById('settingName');
        const settingEmail = document.getElementById('settingEmail');
        if (settingName) settingName.value = user.name;
        if (settingEmail) settingEmail.value = user.email;

    } catch (err) {
        console.error('Failed to load user data:', err);
        // If we fail to load user, likely token is dead
        if (window.location.pathname !== '/login.html' && window.location.pathname !== '/signup.html') {
            logout();
        }
    }
}

// Fetch system stats
async function fetchStats() {
    try {
        const res = await fetch('/api/stats');
        if (!res.ok) throw new Error('Stats failure');

        const stats = await res.json();

        // Update elements by ID
        const elements = {
            'stat-containers': stats.active_containers,
            'stat-cpu': stats.cpu_usage_percent + '%',
            'stat-memory': stats.memory_usage,
            'stat-storage': `${stats.storage_usage_gb.toFixed(1)} GB / 10 GB`,
            'stat-net-in': stats.network_in_mb.toFixed(1) + ' MB/s',
            'stat-net-out': stats.network_out_mb.toFixed(1) + ' MB/s'
        };

        for (const [id, value] of Object.entries(elements)) {
            const el = document.getElementById(id);
            if (el) el.textContent = value;
        }

        // Update storage bar
        const storageBar = document.getElementById('storage-bar');
        if (storageBar) {
            const percent = (stats.storage_usage_gb / 10) * 100;
            storageBar.style.width = Math.min(percent, 100) + '%';
        }
    } catch (err) {
        console.error('Failed to fetch stats:', err);
    }
}

// Fetch user projects
async function fetchProjects() {
    try {
        const res = await fetchWithAuth('/api/projects');
        if (!res) return;

        const projects = await res.json();
        const tbody = document.getElementById('deploymentsTable');

        if (!tbody) return;

        if (projects.length === 0) {
            tbody.innerHTML = `<tr><td colspan="5" style="text-align: center; color: var(--text-muted); padding: 3rem;">No active services found.</td></tr>`;
            return;
        }

        let activeProjects = projects;

        // Setup filter listeners if they exist and haven't been bound
        const searchInput = document.getElementById('serviceSearch');
        if (searchInput && !searchInput.hasAttribute('data-bound')) {
            searchInput.setAttribute('data-bound', 'true');
            searchInput.addEventListener('input', renderProjects);
        }

        function renderProjects() {
            const query = (searchInput ? searchInput.value.toLowerCase() : '');
            const filtered = projects.filter(p => p.name.toLowerCase().includes(query) || (p.status && p.status.toLowerCase().includes(query)));

            if (filtered.length === 0) {
                tbody.innerHTML = `<tr><td colspan="5" style="text-align: center; color: var(--text-muted); padding: 3rem;">No active services found.</td></tr>`;
                return;
            }

            tbody.innerHTML = filtered.map(p => `
                <tr>
                    <td>
                        <div class="project-cell">
                            <i class="fab fa-${getLanguageIcon(p.language)}"></i>
                            <div>
                                <strong><a href="project.html?id=${p.name}" style="color: var(--text-main); text-decoration: none;">${p.name}</a></strong>
                                <span class="sub-text" style="font-size:0.75rem; color:var(--text-muted); display:block; margin-top:2px;">${capitalize(p.status)}</span>
                            </div>
                        </div>
                    </td>
                    <td><span class="badge ${getStatusClass(p.status)}">${capitalize(p.status)}</span></td>
                    <td>Global Edge</td>
                    <td class="mono">${formatDate(p.updated_at)}</td>
                    <td>
                        <div style="display: flex; gap: 0.5rem;">
                            <button class="btn-outline" onclick="showLogs('${p.name}')" style="padding: 4px 8px; font-size: 0.75rem;"><i class="fas fa-terminal"></i></button>
                            <button class="btn-outline" onclick="controlProject('${p.id}', 'restart')" style="padding: 4px 8px; font-size: 0.75rem;"><i class="fas fa-sync"></i></button>
                            <button class="btn-outline" onclick="controlProject('${p.id}', 'stop')" style="padding: 4px 8px; font-size: 0.75rem;"><i class="fas fa-stop"></i></button>
                            <button class="btn-primary" onclick="controlProject('${p.id}', 'redeploy')" style="padding: 4px 8px; font-size: 0.75rem;"><i class="fas fa-rocket"></i></button>
                        </div>
                    </td>
                </tr>
            `).join('');
        }

        renderProjects();
    } catch (err) {
        console.error('Failed to fetch projects:', err);
    }
}

// Helper: Get language icon
function getLanguageIcon(lang) {
    const icons = {
        'node': 'node-js',
        'python': 'python',
        'rust': 'rust',
        'go': 'golang',
        'javascript': 'node-js'
    };
    return icons[lang?.toLowerCase()] || 'code';
}

// Helper: Get status badge class
function getStatusClass(status) {
    const s = status?.toLowerCase();
    if (s === 'production' || s === 'ready' || s === 'active') return 'success';
    if (s === 'failed' || s === 'error') return 'error';
    if (s === 'deploying' || s === 'starting') return 'warning';
    if (s === 'stopped' || s === 'terminated') return 'muted';
    return 'success';
}

// Helper: Capitalize string
function capitalize(str) {
    if (!str) return 'Unknown';
    return str.charAt(0).toUpperCase() + str.slice(1);
}

// Helper: Format date from SQLite datetime (e.g. "2026-03-04 20:00:00" or ISO)
function formatDate(raw) {
    if (!raw) return 'Recently';
    const parsed = new Date(raw.replace(' ', 'T'));
    if (isNaN(parsed.getTime())) {
        // fallback: return the date part only
        return raw.split('T')[0] || raw.split(' ')[0] || raw;
    }
    return parsed.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' });
}

// Modal Management
function openModal(id) {
    const modal = document.getElementById(id);
    if (modal) modal.style.display = 'flex';
}

function closeModal(id) {
    const modal = document.getElementById(id);
    if (modal) modal.style.display = 'none';
}

// Handle new project deployment
const launchBtn = document.getElementById('launchBtn');
if (launchBtn) {
    launchBtn.addEventListener('click', () => openModal('launchModal'));
}

const launchForm = document.getElementById('launchForm');
if (launchForm) {
    launchForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const projectName = document.getElementById('projName').value;
        const language = document.getElementById('projLang').value;
        const submitBtn = e.target.querySelector('button[type="submit"]');

        const originalText = submitBtn.innerHTML;
        submitBtn.disabled = true;
        submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Launching...';

        try {
            const res = await fetchWithAuth('/api/deployments', {
                method: 'POST',
                body: JSON.stringify({ project_name: projectName, language: language })
            });

            if (res && res.ok) {
                closeModal('launchModal');
                alert('🚀 ' + projectName + ' is launching on Nova Edge!');
                if (window.location.pathname.includes('deployments.html')) {
                    fetchProjects();
                } else {
                    window.location.href = 'deployments.html';
                }
            }
        } catch (err) {
            alert('Failed to initiate deployment');
        } finally {
            submitBtn.disabled = false;
            submitBtn.innerHTML = originalText;
        }
    });
}

// Payment Method Logic
const addPaymentBtn = document.querySelector('.btn-primary[style*="justify-content: center"]');
if (addPaymentBtn) {
    addPaymentBtn.addEventListener('click', () => openModal('paymentModal'));
}

const paymentForm = document.getElementById('paymentForm');
if (paymentForm) {
    paymentForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const submitBtn = e.target.querySelector('button[type="submit"]');
        submitBtn.disabled = true;
        submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Saving...';

        // In a real app, you'd send this to Stripe/HyperSwitch
        // Here we simulate success and refresh the billing UI
        setTimeout(async () => {
            alert('✅ Secure payment method added to your Nova account!');
            closeModal('paymentModal');
            await fetchBilling(); // Refresh the list
            submitBtn.disabled = false;
            submitBtn.innerHTML = 'Save Card';
        }, 1000);
    });
}

// Logout functionality
function logout() {
    localStorage.removeItem('nova_token');
    localStorage.removeItem('nova_user');
    window.location.href = '/login.html';
}

// Settings Form Handling
const settingsForm = document.getElementById('settingsForm');
if (settingsForm) {
    settingsForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const name = document.getElementById('settingName').value;
        const submitBtn = e.target.querySelector('button[type="submit"]');

        submitBtn.disabled = true;
        submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Saving...';

        try {
            const res = await fetchWithAuth('/api/settings', {
                method: 'POST',
                body: JSON.stringify({ name })
            });
            if (res && res.ok) {
                alert('✨ Profile updated successfully!');
                await loadUserData(); // Refresh global UI
            }
        } catch (err) {
            alert('Failed to update profile');
        } finally {
            submitBtn.disabled = false;
            submitBtn.innerHTML = 'Save Changes';
        }
    });

    // Wire up delete account
    const deleteBtn = document.getElementById('deleteAccountBtn');
    if (deleteBtn) {
        deleteBtn.addEventListener('click', async () => {
            if (confirm('⚠️ Are you sure? This will PERMANENTLY delete your Nova Cloud account.')) {
                try {
                    const res = await fetchWithAuth('/api/settings', { method: 'DELETE' });
                    if (res && res.ok) {
                        alert('Account deleted. We hope to see you again!');
                        logout();
                    }
                } catch (err) {
                    alert('Failed to delete account');
                }
            }
        });
    }
}

// Logs Modal Trigger
// Logs Modal Trigger with Professional Terminal Formatting
let currentLogStream = null;

async function showLogs(projName) {
    const logsModal = document.getElementById('logsModal');
    const logsContent = document.getElementById('logsContent');
    const token = localStorage.getItem('nova_token');
    if (!logsModal || !logsContent || !token) return;

    // Clean up existing stream if any
    if (currentLogStream) {
        currentLogStream.close();
        currentLogStream = null;
    }

    logsContent.innerHTML = '<div class="log-line" style="color: #64748b;">> Connecting to Edge instance...</div>';
    openModal('logsModal');

    try {
        currentLogStream = new EventSource(`/api/deployments/logs?token=${token}`);

        currentLogStream.onmessage = (event) => {
            const line = event.data;
            const parts = line.match(/^\[(.*?)\] \[(.*?)\] (.*)$/);
            let htmlLine = '';

            if (parts) {
                const time = parts[1];
                const type = parts[2];
                const message = parts[3];

                htmlLine = `
                    <div class="log-line">
                        <span class="log-time" style="color: #64748b; margin-right: 8px;">[${time}]</span>
                        <span class="log-${type.toLowerCase()}" style="font-weight: 600; margin-right: 8px;">[${type}]</span>
                        <span class="log-msg" style="color: #cbd5e1;">${message}</span>
                    </div>
                `;
            } else {
                htmlLine = `<div class="log-line" style="color: #cbd5e1;">${line}</div>`;
            }

            // Append to DOM
            logsContent.insertAdjacentHTML('beforeend', htmlLine);

            // Limit buffer to 100 lines to prevent memory DOM leak
            if (logsContent.children.length > 100) {
                logsContent.removeChild(logsContent.firstChild);
            }

            logsContent.scrollTop = logsContent.scrollHeight;
        };

        currentLogStream.onerror = (err) => {
            logsContent.insertAdjacentHTML('beforeend', '<div class="log-line log-error" style="color: #ef4444;">[SYSTEM] Stream disconnected.</div>');
            currentLogStream.close();
        };

    } catch (err) {
        logsContent.innerHTML = '<div class="log-line log-error" style="color: #ef4444;">Failed to start stream.</div>';
    }
}

// Add logout to profile click
const profile = document.getElementById('userProfile');
if (profile) {
    profile.style.cursor = 'pointer';
    profile.title = 'Click to logout';
    profile.addEventListener('click', () => {
        if (confirm('Logout from Nova Cloud?')) {
            logout();
        }
    });
}
