<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	interface ResetParams {
		token: string;
		password: string;
	}

	// Get token from URL
	let token = $derived(page.url.searchParams.get('token') || '');

	// Form state
	let password = $state('');
	let confirmPassword = $state('');

	// UI state
	let isLoading = $state(false);
	let error = $state('');
	let success = $state('');
	let showPassword = $state(false);
	let showConfirmPassword = $state(false);
	let resetComplete = $state(false);

	// Validation state
	let passwordError = $state('');
	let confirmPasswordError = $state('');

	// Password strength
	let passwordStrength = $derived(() => {
		if (!password) return { score: 0, label: '', color: '' };

		let score = 0;
		const checks = {
			length: password.length >= 8,
			lowercase: /[a-z]/.test(password),
			uppercase: /[A-Z]/.test(password),
			numbers: /\d/.test(password),
			special: /[!@#$%^&*(),.?":{}|<>]/.test(password)
		};

		score = Object.values(checks).filter(Boolean).length;

		const strength = {
			0: { label: '', color: '' },
			1: { label: 'Very Weak', color: 'bg-red-500' },
			2: { label: 'Weak', color: 'bg-orange-500' },
			3: { label: 'Fair', color: 'bg-yellow-500' },
			4: { label: 'Good', color: 'bg-blue-500' },
			5: { label: 'Strong', color: 'bg-green-500' }
		};

		return { score, ...strength[score] };
	});

	onMount(() => {
		// Check if we have a token
		if (!token) {
			error = 'Invalid or missing reset token. Please request a new password reset.';
		}
	});

	function validatePassword(value: string): boolean {
		if (!value) {
			passwordError = 'Password is required';
			return false;
		}
		if (value.length < 6) {
			passwordError = 'Password must be at least 6 characters';
			return false;
		}
		if (value.length > 128) {
			passwordError = 'Password must be less than 128 characters';
			return false;
		}
		passwordError = '';
		return true;
	}

	function validateConfirmPassword(value: string): boolean {
		if (!value) {
			confirmPasswordError = 'Please confirm your password';
			return false;
		}
		if (value !== password) {
			confirmPasswordError = 'Passwords do not match';
			return false;
		}
		confirmPasswordError = '';
		return true;
	}

	async function handleResetPassword(e: Event) {
		e.preventDefault();
		error = '';
		success = '';

		// Validate fields
		const isPasswordValid = validatePassword(password);
		const isConfirmPasswordValid = validateConfirmPassword(confirmPassword);

		if (!isPasswordValid || !isConfirmPasswordValid) {
			return;
		}

		if (!token) {
			error = 'Reset token is missing. Please request a new password reset.';
			return;
		}

		isLoading = true;

		try {
			const params: ResetParams = {
				token,
				password
			};

			const response = await fetch('/api/auth/reset', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(params)
			});

			if (!response.ok) {
				if (response.status === 400) {
					throw new Error('Invalid or expired reset token. Please request a new password reset.');
				}
				throw new Error('Failed to reset password. Please try again.');
			}

			// Reset successful
			resetComplete = true;
			success = 'Password reset successful! You can now sign in with your new password.';

			// Redirect to login after delay
			setTimeout(() => {
				goto('/auth/login');
			}, 3000);

		} catch (err) {
			error = err instanceof Error ? err.message : 'An unexpected error occurred';
		} finally {
			isLoading = false;
		}
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
				Reset Your Password
			</h2>
			<p class="mt-2 text-sm text-gray-600">
				Enter your new password below
			</p>
		</div>

		{#if !resetComplete}
			<!-- Reset Password Form -->
			<div class="bg-white rounded-lg shadow-xl p-6">
				<!-- Success/Error Messages -->
				{#if success}
					<div class="rounded-md bg-green-50 p-4 mb-6">
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
					<div class="rounded-md bg-red-50 p-4 mb-6">
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

				{#if token}
					<form onsubmit={handleResetPassword} class="space-y-5">
						<!-- New Password Field -->
						<div>
							<label for="password" class="block text-sm font-medium text-gray-700">
								New Password
							</label>
							<div class="mt-1 relative">
								<input
									id="password"
									type={showPassword ? 'text' : 'password'}
									autocomplete="new-password"
									required
									bind:value={password}
									onblur={() => validatePassword(password)}
									class="appearance-none block w-full px-3 py-2 pr-10 border rounded-lg shadow-sm placeholder-gray-400 focus:outline-none focus:ring-orange-500 focus:border-orange-500 {passwordError ? 'border-red-300' : 'border-gray-300'}"
									placeholder="Enter your new password"
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

							<!-- Password Strength Indicator -->
							{#if password}
								<div class="mt-2">
									<div class="flex justify-between items-center mb-1">
										<span class="text-xs text-gray-600">Password strength</span>
										<span class="text-xs font-medium {passwordStrength().color.replace('bg-', 'text-')}">{passwordStrength().label}</span>
									</div>
									<div class="w-full bg-gray-200 rounded-full h-2">
										<div
											class="h-2 rounded-full transition-all duration-300 {passwordStrength().color}"
											style="width: {passwordStrength().score * 20}%"
										></div>
									</div>
								</div>
							{/if}
						</div>

						<!-- Confirm Password Field -->
						<div>
							<label for="confirm-password" class="block text-sm font-medium text-gray-700">
								Confirm New Password
							</label>
							<div class="mt-1 relative">
								<input
									id="confirm-password"
									type={showConfirmPassword ? 'text' : 'password'}
									autocomplete="new-password"
									required
									bind:value={confirmPassword}
									onblur={() => validateConfirmPassword(confirmPassword)}
									class="appearance-none block w-full px-3 py-2 pr-10 border rounded-lg shadow-sm placeholder-gray-400 focus:outline-none focus:ring-orange-500 focus:border-orange-500 {confirmPasswordError ? 'border-red-300' : 'border-gray-300'}"
									placeholder="Re-enter your new password"
								/>
								<button
									type="button"
									onclick={() => showConfirmPassword = !showConfirmPassword}
									class="absolute inset-y-0 right-0 pr-3 flex items-center"
								>
									{#if showConfirmPassword}
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
								{#if confirmPasswordError}
									<p class="mt-1 text-sm text-red-600">{confirmPasswordError}</p>
								{/if}
							</div>
						</div>

						<!-- Submit Button -->
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
								Resetting password...
							{:else}
								Reset password
							{/if}
						</button>
					</form>
				{:else if !error}
					<div class="text-center py-8">
						<svg class="animate-spin h-8 w-8 text-orange-600 mx-auto" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						<p class="mt-4 text-gray-600">Checking reset token...</p>
					</div>
				{/if}

				<!-- Additional Options -->
				<div class="mt-6 text-center">
					<p class="text-sm text-gray-600">
						Remember your password?
						<a href="/auth/login" class="font-medium text-orange-600 hover:text-orange-500">
							Sign in instead
						</a>
					</p>
					<p class="mt-2 text-sm text-gray-600">
						Need a new reset link?
						<a href="/auth/login" class="font-medium text-orange-600 hover:text-orange-500">
							Request password reset
						</a>
					</p>
				</div>
			</div>
		{:else}
			<!-- Reset Complete Message -->
			<div class="bg-white rounded-lg shadow-xl p-8 text-center">
				<div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-green-100 mb-4">
					<svg class="h-10 w-10 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
				</div>

				<h3 class="text-2xl font-bold text-gray-900 mb-2">Password Reset Successful!</h3>
				<p class="text-gray-600 mb-6">
					Your password has been successfully reset. You can now sign in with your new password.
				</p>

				<button
					onclick={() => goto('/auth/login')}
					class="w-full py-2 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-gradient-to-r from-orange-600 to-red-600 hover:from-orange-700 hover:to-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500"
				>
					Continue to Sign In
				</button>
			</div>
		{/if}
	</div>
</div>
