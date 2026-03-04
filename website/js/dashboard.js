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

// Load user data
async function loadUserData() {
    try {
        const res = await fetchWithAuth('/api/me');
        if (!res) return;

        const user = await res.json();

        // Update UI with user info
        const nameEl = document.querySelector('.user-info .name');
        if (nameEl) nameEl.textContent = user.name;
    } catch (err) {
        console.error('Failed to load user data:', err);
    }
}

// Fetch system stats
async function fetchStats() {
    try {
        const res = await fetch('/api/stats');
        const stats = await res.json();

        // Update stat cards
        const statCards = document.querySelectorAll('.stat-card .value');
        if (statCards[0]) statCards[0].textContent = stats.active_containers;
        if (statCards[1]) statCards[1].textContent = stats.health_status;
        if (statCards[2]) statCards[2].textContent = stats.memory_usage;
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
        const tbody = document.querySelector('table tbody');

        if (!tbody || projects.length === 0) return;

        tbody.innerHTML = projects.map(p => `
            <tr>
                <td>
                    <div class="project-cell">
                        <i class="fab fa-${getLanguageIcon(p.language)}"></i>
                        <div>
                            <strong>${p.name}</strong>
                            <span class="sub-text">${p.status}</span>
                        </div>
                    </div>
                </td>
                <td><span class="badge ${getStatusClass(p.status)}">${capitalize(p.status)}</span></td>
                <td class="mono">${p.commit_hash}</td>
                <td>-</td>
                <td>${p.updated_at}</td>
            </tr>
        `).join('');
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
        'go': 'golang'
    };
    return icons[lang] || 'code';
}

// Helper: Get status badge class
function getStatusClass(status) {
    const classes = {
        'production': 'success',
        'staging': 'success',
        'dev': 'error'
    };
    return classes[status] || 'success';
}

// Helper: Capitalize string
function capitalize(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

// Handle new project deployment
const newProjectBtn = document.querySelector('.btn-primary');
if (newProjectBtn) {
    newProjectBtn.addEventListener('click', async () => {
        const projectName = prompt('Enter project name:');
        if (!projectName) return;

        try {
            const res = await fetchWithAuth('/api/deployments', {
                method: 'POST',
                body: JSON.stringify({ project_name: projectName })
            });

            if (res && res.ok) {
                alert('Deployment initiated!');
                fetchProjects();
            }
        } catch (err) {
            alert('Failed to deploy project');
        }
    });
}

// Logout functionality
function logout() {
    localStorage.removeItem('nova_token');
    localStorage.removeItem('nova_user');
    window.location.href = '/login.html';
}
