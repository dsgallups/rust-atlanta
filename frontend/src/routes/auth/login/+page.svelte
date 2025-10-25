<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { onMount } from 'svelte';

	interface LoginParams {
		email: string;
		password: string;
	}

	interface LoginResponse {
		token: string;
		pid: string;
		name: string;
		is_verified: boolean;
	}

	interface MagicLinkParams {
		email: string;
	}

	interface ForgotParams {
		email: string;
	}

	// Form state
	let email = $state('');
	let password = $state('');
	let rememberMe = $state(false);

	// UI state
	let loginMethod = $state<'password' | 'magic-link'>('password');
	let isLoading = $state(false);
	let error = $state('');
	let success = $state('');
	let showPassword = $state(false);

	// Validation state
	let emailError = $state('');
	let passwordError = $state('');

	// Check for redirect URL in query params
	let redirectTo = $derived(page.url.searchParams.get('redirect') || '/');

	onMount(() => {
		// Check if user is already logged in
		const token = localStorage.getItem('auth_token');
		if (token) {
			// Optionally verify token validity with /api/auth/current
			goto(redirectTo);
		}
	});

	function validateEmail(value: string): boolean {
		if (!value) {
			emailError = 'Email is required';
			return false;
		}
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(value)) {
			emailError = 'Please enter a valid email address';
			return false;
		}
		emailError = '';
		return true;
	}

	function validatePassword(value: string): boolean {
		if (loginMethod !== 'password') return true;

		if (!value) {
			passwordError = 'Password is required';
			return false;
		}
		if (value.length < 6) {
			passwordError = 'Password must be at least 6 characters';
			return false;
		}
		passwordError = '';
		return true;
	}

	async function handleLogin(e: Event) {
		e.preventDefault();
		error = '';
		success = '';

		// Validate form
		const isEmailValid = validateEmail(email);
		const isPasswordValid = validatePassword(password);

		if (!isEmailValid || !isPasswordValid) {
			return;
		}

		isLoading = true;

		try {
			const params: LoginParams = {
				email: email.toLowerCase().trim(),
				password
			};

			const response = await fetch('/api/auth/login', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(params)
			});

			if (!response.ok) {
				if (response.status === 401) {
					throw new Error('Invalid email or password');
				}
				throw new Error('Login failed. Please try again.');
			}

			const data: LoginResponse = await response.json();

			// Store token
			if (rememberMe) {
				localStorage.setItem('auth_token', data.token);
				localStorage.setItem('user_pid', data.pid);
				localStorage.setItem('user_name', data.name);
			} else {
				sessionStorage.setItem('auth_token', data.token);
				sessionStorage.setItem('user_pid', data.pid);
				sessionStorage.setItem('user_name', data.name);
			}

			// Check if email is verified
			if (!data.is_verified) {
				success = 'Login successful! Please verify your email to access all features.';
				setTimeout(() => {
					goto('/auth/verify-email?email=' + encodeURIComponent(email));
				}, 2000);
			} else {
				success = 'Login successful! Redirecting...';
				setTimeout(() => {
					goto(redirectTo);
				}, 1000);
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'An unexpected error occurred';
		} finally {
			isLoading = false;
		}
	}

	async function handleMagicLink(e: Event) {
		e.preventDefault();
		error = '';
		success = '';

		// Validate email
		if (!validateEmail(email)) {
			return;
		}

		isLoading = true;

		try {
			const params: MagicLinkParams = {
				email: email.toLowerCase().trim()
			};

			const response = await fetch('/api/auth/magic-link', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(params)
			});

			if (!response.ok) {
				if (response.status === 400) {
					throw new Error('This email domain is not allowed. Please use a valid email.');
				}
				throw new Error('Failed to send magic link. Please try again.');
			}

			success = 'Magic link sent! Check your email to complete login.';

			// Clear form
			setTimeout(() => {
				email = '';
				success = '';
			}, 5000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'An unexpected error occurred';
		} finally {
			isLoading = false;
		}
	}

	async function handleForgotPassword() {
		error = '';
		success = '';

		// Validate email
		if (!validateEmail(email)) {
			error = 'Please enter your email address first';
			return;
		}

		isLoading = true;

		try {
			const params: ForgotParams = {
				email: email.toLowerCase().trim()
			};

			const response = await fetch('/api/auth/forgot', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(params)
			});

			if (!response.ok) {
				throw new Error('Failed to send reset email. Please try again.');
			}

			success = 'Password reset instructions sent to your email!';

			// Clear form after success
			setTimeout(() => {
				email = '';
				password = '';
				success = '';
			}, 5000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'An unexpected error occurred';
		} finally {
			isLoading = false;
		}
	}

	function switchLoginMethod(method: 'password' | 'magic-link') {
		loginMethod = method;
		error = '';
		success = '';
		emailError = '';
		passwordError = '';
		password = '';
	}
</script>

<div class="min-h-screen bg-gradient-to-br from-orange-50 to-red-50 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
	<div class="max-w-md w-full">
		<!-- Logo and Title -->
		<div class="text-center mb-8">
			<a href="/" class="inline-flex items-center space-x-2 mb-6">
				<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-br from-orange-500 to-red-600">
					<span class="text-2xl font-bold text-white">🦀</span>
				</div>
				<span class="text-2xl font-bold text-gray-900">Rust Atlanta</span>
			</a>
			<h2 class="text-3xl font-extrabold text-gray-900">
				Welcome back!
			</h2>
			<p class="mt-2 text-sm text-gray-600">
				Don't have an account?
				<a href="/auth/signup" class="font-medium text-orange-600 hover:text-orange-500">
					Sign up here
				</a>
			</p>
		</div>

		<!-- Login Method Tabs -->
		<div class="bg-white rounded-lg shadow-xl">
			<div class="border-b border-gray-200">
				<nav class="-mb-px flex">
					<button
						onclick={() => switchLoginMethod('password')}
						class="w-1/2 py-3 px-4 text-center border-b-2 font-medium text-sm transition-colors {loginMethod === 'password'
							? 'border-orange-500 text-orange-600'
							: 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
					>
						<svg class="w-5 h-5 inline-block mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
						</svg>
						Password
					</button>
					<button
						onclick={() => switchLoginMethod('magic-link')}
						class="w-1/2 py-3 px-4 text-center border-b-2 font-medium text-sm transition-colors {loginMethod === 'magic-link'
							? 'border-orange-500 text-orange-600'
							: 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
					>
						<svg class="w-5 h-5 inline-block mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
						</svg>
						Magic Link
					</button>
				</nav>
			</div>

			<div class="p-6 space-y-6">
				<!-- Success/Error Messages -->
				{#if success}
					<div class="rounded-md bg-green-50 p-4">
						<div class="flex">
							<svg class="h-5 w-5 text-green-400" fill="currentColor" viewBox="0 0 20 20">
								<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
							</svg>
							<div class="ml-3">
								<p class="text-sm font-medium text-green-800">{success}</p>
							</div>
						</div>
					</div>
				{/if}

				{#if error}
					<div class="rounded-md bg-red-50 p-4">
						<div class="flex">
							<svg class="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
								<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
							</svg>
							<div class="ml-3">
								<p class="text-sm font-medium text-red-800">{error}</p>
							</div>
						</div>
					</div>
				{/if}

				<!-- Login Forms -->
				{#if loginMethod === 'password'}
					<form onsubmit={handleLogin} class="space-y-5">
						<div>
							<label for="email" class="block text-sm font-medium text-gray-700">
								Email address
							</label>
							<div class="mt-1 relative">
								<input
									id="email"
									type="email"
									autocomplete="email"
									required
									bind:value={email}
									onblur={() => validateEmail(email)}
									class="appearance-none block w-full px-3 py-2 border rounded-lg shadow-sm placeholder-gray-400 focus:outline-none focus:ring-orange-500 focus:border-orange-500 {emailError ? 'border-red-300' : 'border-gray-300'}"
									placeholder="you@example.com"
								/>
								{#if emailError}
									<p class="mt-1 text-sm text-red-600">{emailError}</p>
								{/if}
							</div>
						</div>

						<div>
							<label for="password" class="block text-sm font-medium text-gray-700">
								Password
							</label>
							<div class="mt-1 relative">
								<input
									id="password"
									type={showPassword ? 'text' : 'password'}
									autocomplete="current-password"
									required
									bind:value={password}
									onblur={() => validatePassword(password)}
									class="appearance-none block w-full px-3 py-2 pr-10 border rounded-lg shadow-sm placeholder-gray-400 focus:outline-none focus:ring-orange-500 focus:border-orange-500 {passwordError ? 'border-red-300' : 'border-gray-300'}"
									placeholder="Enter your password"
								/>
								<button
									type="button"
									onclick={() => showPassword = !showPassword}
									class="absolute inset-y-0 right-0 pr-3 flex items-center"
								>
									{#if showPassword}
										<svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
										</svg>
									{:else}
										<svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
										</svg>
									{/if}
								</button>
								{#if passwordError}
									<p class="mt-1 text-sm text-red-600">{passwordError}</p>
								{/if}
							</div>
						</div>

						<div class="flex items-center justify-between">
							<div class="flex items-center">
								<input
									id="remember-me"
									type="checkbox"
									bind:checked={rememberMe}
									class="h-4 w-4 text-orange-600 focus:ring-orange-500 border-gray-300 rounded"
								/>
								<label for="remember-me" class="ml-2 block text-sm text-gray-900">
									Remember me
								</label>
							</div>

							<button
								type="button"
								onclick={handleForgotPassword}
								class="text-sm font-medium text-orange-600 hover:text-orange-500"
							>
								Forgot password?
							</button>
						</div>

						<button
							type="submit"
							disabled={isLoading}
							class="w-full flex justify-center py-2 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-gradient-to-r from-orange-600 to-red-600 hover:from-orange-700 hover:to-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
						>
							{#if isLoading}
								<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
								</svg>
								Signing in...
							{:else}
								Sign in
							{/if}
						</button>
					</form>
				{:else}
					<form onsubmit={handleMagicLink} class="space-y-5">
						<div>
							<label for="magic-email" class="block text-sm font-medium text-gray-700">
								Email address
							</label>
							<div class="mt-1">
								<input
									id="magic-email"
									type="email"
									autocomplete="email"
									required
									bind:value={email}
									onblur={() => validateEmail(email)}
									class="appearance-none block w-full px-3 py-2 border rounded-lg shadow-sm placeholder-gray-400 focus:outline-none focus:ring-orange-500 focus:border-orange-500 {emailError ? 'border-red-300' : 'border-gray-300'}"
									placeholder="you@example.com"
								/>
								{#if emailError}
									<p class="mt-1 text-sm text-red-600">{emailError}</p>
								{/if}
							</div>
							<p class="mt-2 text-sm text-gray-500">
								We'll send you a secure link to sign in without a password.
							</p>
						</div>

						<button
							type="submit"
							disabled={isLoading}
							class="w-full flex justify-center py-2 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-gradient-to-r from-orange-600 to-red-600 hover:from-orange-700 hover:to-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
						>
							{#if isLoading}
								<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
								</svg>
								Sending magic link...
							{:else}
								Send magic link
							{/if}
						</button>
					</form>
				{/if}

				<!-- Social Login Divider -->
				<div class="relative">
					<div class="absolute inset-0 flex items-center">
						<div class="w-full border-t border-gray-300"></div>
					</div>
					<div class="relative flex justify-center text-sm">
						<span class="px-2 bg-white text-gray-500">Or continue with</span>
					</div>
				</div>

				<!-- Social Login Buttons -->
				<div class="grid grid-cols-2 gap-3">
					<button
						type="button"
						disabled
						class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-lg shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 cursor-not-allowed opacity-50"
					>
						<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
							<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
						</svg>
						<span class="ml-2">GitHub</span>
					</button>

					<button
						type="button"
						disabled
						class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-lg shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 cursor-not-allowed opacity-50"
					>
						<svg class="w-5 h-5" viewBox="0 0 24 24">
							<path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
							<path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
							<path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
							<path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
						</svg>
						<span class="ml-2">Google</span>
					</button>
				</div>

				<p class="text-xs text-center text-gray-500 mt-4">
					Coming soon: Social login options
				</p>
			</div>
		</div>

		<!-- Footer Links -->
		<p class="mt-6 text-center text-sm text-gray-600">
			By signing in, you agree to our
			<a href="/terms" class="font-medium text-orange-600 hover:text-orange-500"> Terms of Service </a>
			and
			<a href="/privacy" class="font-medium text-orange-600 hover:text-orange-500"> Privacy Policy</a>
		</p>
	</div>
</div>
