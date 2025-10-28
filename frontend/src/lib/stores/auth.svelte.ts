interface User {
	pid: string;
	name: string;
	email?: string;
	is_verified?: boolean;
}

interface AuthState {
	user: User | null;
	isAuthenticated: boolean;
	isLoading: boolean;
	token: string | null;
}

class AuthStore {
	user = $state<User | null>(null);
	isAuthenticated = $state(false);
	isLoading = $state(false);
	token = $state<string | null>(null);

	constructor() {
		// Check for existing auth on initialization
		if (typeof window !== 'undefined') {
			this.initializeFromStorage();
		}
	}

	private initializeFromStorage() {
		const token = localStorage.getItem('auth_token') || sessionStorage.getItem('auth_token');
		if (token) {
			const name = localStorage.getItem('user_name') || sessionStorage.getItem('user_name');
			const pid = localStorage.getItem('user_pid') || sessionStorage.getItem('user_pid');

			if (name && pid) {
				this.user = { pid, name };
				this.token = token;
				this.isAuthenticated = true;

				// Optionally verify token validity with backend
				this.verifyToken();
			}
		}
	}

	async login(
		data: { token: string; pid: string; name: string; is_verified: boolean },
		rememberMe: boolean = false
	) {
		this.user = {
			pid: data.pid,
			name: data.name,
			is_verified: data.is_verified
		};
		this.token = data.token;
		this.isAuthenticated = true;

		// Store in appropriate storage based on rememberMe
		const storage = rememberMe ? localStorage : sessionStorage;
		storage.setItem('auth_token', data.token);
		storage.setItem('user_pid', data.pid);
		storage.setItem('user_name', data.name);
	}

	logout() {
		// Clear all auth data
		this.user = null;
		this.token = null;
		this.isAuthenticated = false;

		// Clear from both storages
		localStorage.removeItem('auth_token');
		localStorage.removeItem('user_pid');
		localStorage.removeItem('user_name');
		sessionStorage.removeItem('auth_token');
		sessionStorage.removeItem('user_pid');
		sessionStorage.removeItem('user_name');
	}

	async verifyToken() {
		if (!this.token) return;

		this.isLoading = true;
		try {
			const response = await fetch('/api/auth/current', {
				headers: {
					Authorization: `Bearer ${this.token}`
				}
			});

			if (response.ok) {
				const data = await response.json();
				this.user = {
					pid: data.pid,
					name: data.name,
					email: data.email
				};
				this.isAuthenticated = true;
			} else {
				// Token is invalid, clear auth
				this.logout();
			}
		} catch (error) {
			console.error('Failed to verify token:', error);
			// Keep the user logged in even if verification fails
			// (offline mode, network issues, etc.)
		} finally {
			this.isLoading = false;
		}
	}

	async fetchWithAuth(url: string, options: RequestInit = {}) {
		if (!this.token) {
			throw new Error('No authentication token available');
		}

		return fetch(url, {
			...options,
			headers: {
				...options.headers,
				Authorization: `Bearer ${this.token}`
			}
		});
	}

	get isLoggedIn() {
		return this.isAuthenticated && this.user !== null;
	}
}

export const auth = new AuthStore();
