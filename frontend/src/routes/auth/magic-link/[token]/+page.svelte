<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	interface LoginResponse {
		token: string;
		pid: string;
		name: string;
		is_verified: boolean;
	}

	// Get token from URL params
	let token = $derived(page.params.token);

	// UI state
	let isLoading = $state(true);
	let error = $state('');
	let success = $state('');
	let verificationComplete = $state(false);

	onMount(async () => {
		if (!token) {
			error = 'Invalid magic link. The link may be expired or incorrect.';
			isLoading = false;
			return;
		}

		await verifyMagicLink();
	});

	async function verifyMagicLink() {
		try {
			const response = await fetch(`/api/auth/magic-link/${token}`, {
				method: 'GET',
				headers: {
					Accept: 'application/json'
				}
			});

			if (!response.ok) {
				if (response.status === 401) {
					throw new Error('This magic link is invalid or has expired. Please request a new one.');
				}
				throw new Error('Failed to verify magic link. Please try again or request a new link.');
			}

			const data: LoginResponse = await response.json();

			// Store token (using localStorage for persistent login)
			localStorage.setItem('auth_token', data.token);
			localStorage.setItem('user_pid', data.pid);
			localStorage.setItem('user_name', data.name);

			verificationComplete = true;
			success = `Welcome back, ${data.name}! Redirecting to your dashboard...`;

			// Redirect after a short delay
			setTimeout(() => {
				// Check if there's a redirect URL in sessionStorage (from before login)
				const redirectUrl = sessionStorage.getItem('auth_redirect') || '/';
				sessionStorage.removeItem('auth_redirect');
				goto(redirectUrl);
			}, 2000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'An unexpected error occurred';
			isLoading = false;
		}
	}

	async function requestNewMagicLink() {
		goto('/auth/login');
	}
</script>

<div
	class="flex min-h-screen items-center justify-center bg-gradient-to-br from-orange-50 to-red-50 px-4 py-12 sm:px-6 lg:px-8"
>
	<div class="w-full max-w-md">
		<!-- Logo and Title -->
		<div class="mb-8 text-center">
			<a href="/" class="mb-6 inline-flex items-center space-x-2">
				<div
					class="flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-br from-orange-500 to-red-600"
				>
					<span class="text-2xl font-bold text-white">🦀</span>
				</div>
				<span class="text-2xl font-bold text-gray-900">Rust Atlanta</span>
			</a>
		</div>

		<div class="rounded-lg bg-white p-8 shadow-xl">
			{#if isLoading && !verificationComplete}
				<!-- Loading State -->
				<div class="text-center">
					<div
						class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-orange-100"
					>
						<svg class="h-8 w-8 animate-spin text-orange-600" fill="none" viewBox="0 0 24 24">
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
					</div>
					<h2 class="mb-2 text-2xl font-bold text-gray-900">Verifying Magic Link</h2>
					<p class="text-gray-600">Please wait while we sign you in securely...</p>
				</div>
			{:else if verificationComplete}
				<!-- Success State -->
				<div class="text-center">
					<div
						class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100"
					>
						<svg
							class="h-10 w-10 text-green-600"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
					</div>
					<h2 class="mb-2 text-2xl font-bold text-gray-900">Success!</h2>
					<p class="mb-6 text-gray-600">{success}</p>

					<div class="space-y-2">
						<div class="flex items-center justify-center space-x-2 text-sm text-gray-500">
							<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
							<span>Redirecting...</span>
						</div>
					</div>
				</div>
			{:else if error}
				<!-- Error State -->
				<div class="text-center">
					<div
						class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-red-100"
					>
						<svg
							class="h-10 w-10 text-red-600"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
							/>
						</svg>
					</div>
					<h2 class="mb-2 text-2xl font-bold text-gray-900">Verification Failed</h2>
					<p class="mb-6 text-gray-600">{error}</p>

					<div class="space-y-3">
						<button
							onclick={requestNewMagicLink}
							class="w-full rounded-lg border border-transparent bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:from-orange-700 hover:to-red-700 focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:outline-none"
						>
							Request New Magic Link
						</button>

						<a
							href="/auth/login"
							class="block w-full rounded-lg border border-gray-300 bg-white px-4 py-2 text-center text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:outline-none"
						>
							Back to Login
						</a>
					</div>
				</div>
			{/if}

			<!-- Security Notice -->
			<div class="mt-8 border-t border-gray-200 pt-6">
				<div class="flex items-start space-x-2">
					<svg
						class="mt-0.5 h-5 w-5 text-gray-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
						/>
					</svg>
					<div class="text-xs text-gray-500">
						<p class="mb-1 font-medium">Security Notice</p>
						<p>
							Magic links are single-use and expire after 5 minutes. Never share your magic link
							with anyone.
						</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-6 text-center">
			<p class="text-sm text-gray-600">
				Having trouble?
				<a href="/contact" class="font-medium text-orange-600 hover:text-orange-500">
					Contact support
				</a>
			</p>
		</div>
	</div>
</div>
