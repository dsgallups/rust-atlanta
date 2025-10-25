<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	interface RegisterParams {
		email: string;
		password: string;
		name: string;
	}

	// Form state
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let agreeToTerms = $state(false);

	// UI state
	let isLoading = $state(false);
	let error = $state('');
	let success = $state('');
	let showPassword = $state(false);
	let showConfirmPassword = $state(false);
	let registrationComplete = $state(false);

	// Validation state
	let nameError = $state('');
	let emailError = $state('');
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
		// Check if user is already logged in
		const token = localStorage.getItem('auth_token');
		if (token) {
			goto('/');
		}
	});

	function validateName(value: string): boolean {
		if (!value) {
			nameError = 'Name is required';
			return false;
		}
		if (value.length < 2) {
			nameError = 'Name must be at least 2 characters long';
			return false;
		}
		if (value.length > 100) {
			nameError = 'Name must be less than 100 characters';
			return false;
		}
		nameError = '';
		return true;
	}

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
		// Check for allowed domains (matching backend regex)
		const allowedDomains = /@example\.com$|@gmail\.com$/;
		if (!allowedDomains.test(value)) {
			emailError = 'Please use an email from gmail.com or example.com';
			return false;
		}
		emailError = '';
		return true;
	}

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

	async function handleSignup(e: Event) {
		e.preventDefault();
		error = '';
		success = '';

		// Validate all fields
		const isNameValid = validateName(name);
		const isEmailValid = validateEmail(email);
		const isPasswordValid = validatePassword(password);
		const isConfirmPasswordValid = validateConfirmPassword(confirmPassword);

		if (!isNameValid || !isEmailValid || !isPasswordValid || !isConfirmPasswordValid) {
			return;
		}

		if (!agreeToTerms) {
			error = 'You must agree to the terms and conditions';
			return;
		}

		isLoading = true;

		try {
			const params: RegisterParams = {
				name: name.trim(),
				email: email.toLowerCase().trim(),
				password
			};

			const response = await fetch('/api/auth/register', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(params)
			});

			if (!response.ok) {
				if (response.status === 422) {
					const data = await response.json();
					throw new Error(data.error || 'Validation failed. Please check your information.');
				}
				throw new Error('Registration failed. Please try again.');
			}

			// Registration successful
			registrationComplete = true;
			success = 'Registration successful! Check your email to verify your account.';
		} catch (err) {
			error = err instanceof Error ? err.message : 'An unexpected error occurred';
		} finally {
			isLoading = false;
		}
	}

	async function resendVerificationEmail() {
		isLoading = true;
		error = '';
		success = '';

		try {
			const response = await fetch('/api/auth/resend-verification-mail', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ email: email.toLowerCase().trim() })
			});

			if (!response.ok) {
				throw new Error('Failed to resend verification email');
			}

			success = 'Verification email resent! Please check your inbox.';
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to resend email';
		} finally {
			isLoading = false;
		}
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
			<h2 class="text-3xl font-extrabold text-gray-900">Join the Community</h2>
			<p class="mt-2 text-sm text-gray-600">
				Already have an account?
				<a href="/auth/login" class="font-medium text-orange-600 hover:text-orange-500">
					Sign in here
				</a>
			</p>
		</div>

		{#if !registrationComplete}
			<!-- Signup Form -->
			<div class="rounded-lg bg-white p-6 shadow-xl">
				<!-- Success/Error Messages -->
				{#if success}
					<div class="mb-6 rounded-md bg-green-50 p-4">
						<div class="flex">
							<svg class="h-5 w-5 text-green-400" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
									clip-rule="evenodd"
								/>
							</svg>
							<div class="ml-3">
								<p class="text-sm font-medium text-green-800">{success}</p>
							</div>
						</div>
					</div>
				{/if}

				{#if error}
					<div class="mb-6 rounded-md bg-red-50 p-4">
						<div class="flex">
							<svg class="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
									clip-rule="evenodd"
								/>
							</svg>
							<div class="ml-3">
								<p class="text-sm font-medium text-red-800">{error}</p>
							</div>
						</div>
					</div>
				{/if}

				<form onsubmit={handleSignup} class="space-y-5">
					<!-- Name Field -->
					<div>
						<label for="name" class="block text-sm font-medium text-gray-700"> Full Name </label>
						<div class="mt-1">
							<input
								id="name"
								type="text"
								autocomplete="name"
								required
								bind:value={name}
								onblur={() => validateName(name)}
								class="block w-full appearance-none rounded-lg border px-3 py-2 placeholder-gray-400 shadow-sm focus:border-orange-500 focus:ring-orange-500 focus:outline-none {nameError
									? 'border-red-300'
									: 'border-gray-300'}"
								placeholder="John Doe"
							/>
							{#if nameError}
								<p class="mt-1 text-sm text-red-600">{nameError}</p>
							{/if}
						</div>
					</div>

					<!-- Email Field -->
					<div>
						<label for="email" class="block text-sm font-medium text-gray-700">
							Email address
						</label>
						<div class="mt-1">
							<input
								id="email"
								type="email"
								autocomplete="email"
								required
								bind:value={email}
								onblur={() => validateEmail(email)}
								class="block w-full appearance-none rounded-lg border px-3 py-2 placeholder-gray-400 shadow-sm focus:border-orange-500 focus:ring-orange-500 focus:outline-none {emailError
									? 'border-red-300'
									: 'border-gray-300'}"
								placeholder="you@gmail.com"
							/>
							{#if emailError}
								<p class="mt-1 text-sm text-red-600">{emailError}</p>
							{:else}
								<p class="mt-1 text-xs text-gray-500">
									Currently accepting gmail.com and example.com emails
								</p>
							{/if}
						</div>
					</div>

					<!-- Password Field -->
					<div>
						<label for="password" class="block text-sm font-medium text-gray-700"> Password </label>
						<div class="relative mt-1">
							<input
								id="password"
								type={showPassword ? 'text' : 'password'}
								autocomplete="new-password"
								required
								bind:value={password}
								onblur={() => validatePassword(password)}
								class="block w-full appearance-none rounded-lg border px-3 py-2 pr-10 placeholder-gray-400 shadow-sm focus:border-orange-500 focus:ring-orange-500 focus:outline-none {passwordError
									? 'border-red-300'
									: 'border-gray-300'}"
								placeholder="Create a strong password"
							/>
							<button
								type="button"
								onclick={() => (showPassword = !showPassword)}
								class="absolute inset-y-0 right-0 flex items-center pr-3"
							>
								{#if showPassword}
									<svg
										class="h-5 w-5 text-gray-400"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
										/>
									</svg>
								{:else}
									<svg
										class="h-5 w-5 text-gray-400"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
										/>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
										/>
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
								<div class="mb-1 flex items-center justify-between">
									<span class="text-xs text-gray-600">Password strength</span>
									<span
										class="text-xs font-medium {passwordStrength().color.replace('bg-', 'text-')}"
										>{passwordStrength().label}</span
									>
								</div>
								<div class="h-2 w-full rounded-full bg-gray-200">
									<div
										class="h-2 rounded-full transition-all duration-300 {passwordStrength().color}"
										style="width: {passwordStrength().score * 20}%"
									></div>
								</div>
								<ul class="mt-2 space-y-1 text-xs text-gray-500">
									<li class={password.length >= 8 ? 'text-green-600' : ''}>
										✓ At least 8 characters
									</li>
									<li
										class={/[a-z]/.test(password) && /[A-Z]/.test(password) ? 'text-green-600' : ''}
									>
										✓ Mix of uppercase & lowercase
									</li>
									<li class={/\d/.test(password) ? 'text-green-600' : ''}>✓ Include numbers</li>
									<li class={/[!@#$%^&*(),.?":{}|<>]/.test(password) ? 'text-green-600' : ''}>
										✓ Include special characters
									</li>
								</ul>
							</div>
						{/if}
					</div>

					<!-- Confirm Password Field -->
					<div>
						<label for="confirm-password" class="block text-sm font-medium text-gray-700">
							Confirm Password
						</label>
						<div class="relative mt-1">
							<input
								id="confirm-password"
								type={showConfirmPassword ? 'text' : 'password'}
								autocomplete="new-password"
								required
								bind:value={confirmPassword}
								onblur={() => validateConfirmPassword(confirmPassword)}
								class="block w-full appearance-none rounded-lg border px-3 py-2 pr-10 placeholder-gray-400 shadow-sm focus:border-orange-500 focus:ring-orange-500 focus:outline-none {confirmPasswordError
									? 'border-red-300'
									: 'border-gray-300'}"
								placeholder="Re-enter your password"
							/>
							<button
								type="button"
								onclick={() => (showConfirmPassword = !showConfirmPassword)}
								class="absolute inset-y-0 right-0 flex items-center pr-3"
							>
								{#if showConfirmPassword}
									<svg
										class="h-5 w-5 text-gray-400"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
										/>
									</svg>
								{:else}
									<svg
										class="h-5 w-5 text-gray-400"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
										/>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
										/>
									</svg>
								{/if}
							</button>
							{#if confirmPasswordError}
								<p class="mt-1 text-sm text-red-600">{confirmPasswordError}</p>
							{/if}
						</div>
					</div>

					<!-- Terms and Conditions -->
					<div class="flex items-start">
						<div class="flex h-5 items-center">
							<input
								id="agree-terms"
								type="checkbox"
								bind:checked={agreeToTerms}
								class="h-4 w-4 rounded border-gray-300 text-orange-600 focus:ring-orange-500"
							/>
						</div>
						<div class="ml-3 text-sm">
							<label for="agree-terms" class="font-medium text-gray-700">
								I agree to the
								<a href="/terms" target="_blank" class="text-orange-600 hover:text-orange-500"
									>Terms and Conditions</a
								>
								and
								<a href="/privacy" target="_blank" class="text-orange-600 hover:text-orange-500"
									>Privacy Policy</a
								>
							</label>
						</div>
					</div>

					<!-- Submit Button -->
					<button
						type="submit"
						disabled={isLoading || !agreeToTerms}
						class="flex w-full justify-center rounded-lg border border-transparent bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition-all hover:from-orange-700 hover:to-red-700 focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
					>
						{#if isLoading}
							<svg
								class="mr-3 -ml-1 h-5 w-5 animate-spin text-white"
								fill="none"
								viewBox="0 0 24 24"
							>
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
							Creating account...
						{:else}
							Create account
						{/if}
					</button>
				</form>

				<!-- Social Signup Divider -->
				<div class="mt-6">
					<div class="relative">
						<div class="absolute inset-0 flex items-center">
							<div class="w-full border-t border-gray-300"></div>
						</div>
						<div class="relative flex justify-center text-sm">
							<span class="bg-white px-2 text-gray-500">Or sign up with</span>
						</div>
					</div>

					<!-- Social Signup Buttons -->
					<div class="mt-6 grid grid-cols-2 gap-3">
						<button
							type="button"
							disabled
							class="inline-flex w-full cursor-not-allowed justify-center rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-500 opacity-50 shadow-sm hover:bg-gray-50"
						>
							<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
								<path
									d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
								/>
							</svg>
							<span class="ml-2">GitHub</span>
						</button>

						<button
							type="button"
							disabled
							class="inline-flex w-full cursor-not-allowed justify-center rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-500 opacity-50 shadow-sm hover:bg-gray-50"
						>
							<svg class="h-5 w-5" viewBox="0 0 24 24">
								<path
									fill="#4285F4"
									d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
								/>
								<path
									fill="#34A853"
									d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
								/>
								<path
									fill="#FBBC05"
									d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
								/>
								<path
									fill="#EA4335"
									d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
								/>
							</svg>
							<span class="ml-2">Google</span>
						</button>
					</div>

					<p class="mt-4 text-center text-xs text-gray-500">Coming soon: Social signup options</p>
				</div>
			</div>
		{:else}
			<!-- Registration Complete Message -->
			<div class="rounded-lg bg-white p-8 text-center shadow-xl">
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

				<h3 class="mb-2 text-2xl font-bold text-gray-900">Welcome to Rust Atlanta!</h3>
				<p class="mb-6 text-gray-600">
					We've sent a verification email to <strong class="text-gray-900">{email}</strong>
				</p>

				<div class="mb-6 rounded-lg bg-orange-50 p-4 text-left">
					<h4 class="mb-2 font-semibold text-gray-900">Next steps:</h4>
					<ol class="list-inside list-decimal space-y-1 text-sm text-gray-600">
						<li>Check your email inbox for the verification link</li>
						<li>Click the link to verify your email address</li>
						<li>Sign in to start exploring the community</li>
					</ol>
				</div>

				<div class="space-y-3">
					<button
						onclick={() => goto('/auth/login')}
						class="w-full rounded-lg border border-transparent bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:from-orange-700 hover:to-red-700 focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:outline-none"
					>
						Continue to Sign In
					</button>

					<button
						onclick={resendVerificationEmail}
						disabled={isLoading}
						class="w-full rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
					>
						{isLoading ? 'Sending...' : "Didn't receive the email? Resend"}
					</button>
				</div>

				{#if success}
					<p class="mt-4 text-sm text-green-600">{success}</p>
				{/if}

				{#if error}
					<p class="mt-4 text-sm text-red-600">{error}</p>
				{/if}
			</div>
		{/if}

		<!-- Help Text -->
		<p class="mt-6 text-center text-sm text-gray-600">
			Need help?
			<a href="/contact" class="font-medium text-orange-600 hover:text-orange-500">
				Contact support
			</a>
			or check our
			<a href="/faq" class="font-medium text-orange-600 hover:text-orange-500"> FAQ </a>
		</p>
	</div>
</div>
