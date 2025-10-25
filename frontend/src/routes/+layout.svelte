<script lang="ts">
	import '../app.css';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { auth } from '$lib/stores/auth.svelte';

	let { children } = $props();
	let mobileMenuOpen = $state(false);
	let email = $state('');
	let newsletterStatus = $state<'idle' | 'loading' | 'success' | 'error'>('idle');

	// UI state
	let userDropdownOpen = $state(false);

	const navItems = [
		{ href: '/', label: 'Home' },
		{ href: '/projects', label: 'Projects' },
		{ href: '/events', label: 'Events' },
		{ href: '/news', label: 'News' },
		{ href: '/resources', label: 'Resources' },
		{ href: '/about', label: 'About' }
	];

	onMount(() => {
		// Close dropdown when clicking outside
		const handleClickOutside = (event: MouseEvent) => {
			const target = event.target as HTMLElement;
			if (!target.closest('.user-dropdown')) {
				userDropdownOpen = false;
			}
		};

		document.addEventListener('click', handleClickOutside);
		return () => document.removeEventListener('click', handleClickOutside);
	});

	async function logout() {
		auth.logout();
		userDropdownOpen = false;
		goto('/');
	}

	async function handleNewsletterSubmit(e: Event) {
		e.preventDefault();
		newsletterStatus = 'loading';

		// Mock API call
		await new Promise((resolve) => setTimeout(resolve, 1000));

		// For now, just mock success
		newsletterStatus = 'success';
		email = '';

		// Reset status after 3 seconds
		setTimeout(() => {
			newsletterStatus = 'idle';
		}, 3000);
	}
</script>

<svelte:head>
	<title>Rust Atlanta - Building the Future with Rust in Atlanta</title>
	<meta
		name="description"
		content="Join the Rust Atlanta community - Connect with Rust developers, discover projects, attend events, and learn together."
	/>
</svelte:head>

<div class="flex min-h-screen flex-col bg-gray-50">
	<!-- Header -->
	<header class="sticky top-0 z-40 border-b border-gray-200 bg-white shadow-sm">
		<nav class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="flex h-16 justify-between">
				<div class="flex">
					<!-- Logo -->
					<a href="/" class="flex items-center">
						<div class="flex items-center space-x-2">
							<div
								class="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-orange-500 to-red-600"
							>
								<span class="text-xl font-bold text-white">🦀</span>
							</div>
							<div>
								<span class="text-xl font-bold text-gray-900">Rust Atlanta</span>
							</div>
						</div>
					</a>

					<!-- Desktop Navigation -->
					<div class="hidden md:ml-10 md:flex md:space-x-8">
						{#each navItems as item}
							<a
								href={item.href}
								class="inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium {page.url
									.pathname === item.href
									? 'border-orange-500 text-gray-900'
									: 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
							>
								{item.label}
							</a>
						{/each}
					</div>
				</div>

				<!-- Desktop Auth Section -->
				<div class="hidden md:flex md:items-center md:space-x-4">
					{#if auth.isAuthenticated}
						<!-- User Dropdown -->
						<div class="user-dropdown relative">
							<button
								onclick={() => (userDropdownOpen = !userDropdownOpen)}
								class="flex items-center space-x-2 rounded-lg px-3 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-100 hover:text-gray-900 focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:outline-none"
							>
								<div
									class="flex h-8 w-8 items-center justify-center rounded-full bg-gradient-to-br from-orange-400 to-red-500"
								>
									<span class="font-semibold text-white">
										{auth.user?.name?.charAt(0).toUpperCase() || '?'}
									</span>
								</div>
								<span>{auth.user?.name}</span>
								<svg
									class="h-4 w-4 transition-transform {userDropdownOpen ? 'rotate-180' : ''}"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									/>
								</svg>
							</button>

							{#if userDropdownOpen}
								<div
									class="ring-opacity-5 absolute right-0 z-50 mt-2 w-48 rounded-lg bg-white shadow-lg ring-1 ring-black"
								>
									<div class="py-1">
										<a
											href="/profile"
											class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
											onclick={() => (userDropdownOpen = false)}
										>
											<svg
												class="mr-2 inline-block h-4 w-4"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
												/>
											</svg>
											Profile
										</a>
										<a
											href="/my-projects"
											class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
											onclick={() => (userDropdownOpen = false)}
										>
											<svg
												class="mr-2 inline-block h-4 w-4"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
												/>
											</svg>
											My Projects
										</a>
										<a
											href="/settings"
											class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
											onclick={() => (userDropdownOpen = false)}
										>
											<svg
												class="mr-2 inline-block h-4 w-4"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
												/>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
												/>
											</svg>
											Settings
										</a>
										<hr class="my-1 border-gray-200" />
										<button
											onclick={logout}
											class="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
										>
											<svg
												class="mr-2 inline-block h-4 w-4"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
												/>
											</svg>
											Sign out
										</button>
									</div>
								</div>
							{/if}
						</div>
					{:else}
						<!-- Login/Signup Buttons -->
						<a
							href="/auth/login"
							class="px-3 py-2 text-sm font-medium text-gray-700 hover:text-gray-900"
						>
							Sign in
						</a>
						<a
							href="/auth/signup"
							class="rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 text-sm font-medium text-white transition-shadow hover:shadow-lg"
						>
							Get started
						</a>
					{/if}
				</div>

				<!-- Mobile menu button -->
				<div class="flex items-center md:hidden">
					<button
						type="button"
						class="inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-500"
						onclick={() => (mobileMenuOpen = !mobileMenuOpen)}
					>
						<span class="sr-only">Open main menu</span>
						{#if mobileMenuOpen}
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M6 18L18 6M6 6l12 12"
								/>
							</svg>
						{:else}
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M4 6h16M4 12h16M4 18h16"
								/>
							</svg>
						{/if}
					</button>
				</div>
			</div>

			<!-- Mobile menu -->
			{#if mobileMenuOpen}
				<div class="md:hidden">
					<div class="space-y-1 pt-2 pb-3">
						{#each navItems as item}
							<a
								href={item.href}
								class="block border-l-4 py-2 pr-4 pl-3 text-base font-medium {page.url.pathname ===
								item.href
									? 'border-orange-500 bg-orange-50 text-orange-700'
									: 'border-transparent text-gray-500 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-700'}"
								onclick={() => (mobileMenuOpen = false)}
							>
								{item.label}
							</a>
						{/each}
					</div>

					<!-- Mobile Auth Section -->
					<div class="border-t border-gray-200 pt-4 pb-3">
						{#if auth.isAuthenticated}
							<div class="mb-3 px-4">
								<div class="flex items-center">
									<div
										class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-orange-400 to-red-500"
									>
										<span class="text-lg font-semibold text-white">
											{auth.user?.name?.charAt(0).toUpperCase() || '?'}
										</span>
									</div>
									<div class="ml-3">
										<div class="text-base font-medium text-gray-800">{auth.user?.name}</div>
										<div class="text-sm text-gray-500">View profile</div>
									</div>
								</div>
							</div>
							<div class="space-y-1">
								<a
									href="/profile"
									class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800"
									onclick={() => (mobileMenuOpen = false)}
								>
									Profile
								</a>
								<a
									href="/my-projects"
									class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800"
									onclick={() => (mobileMenuOpen = false)}
								>
									My Projects
								</a>
								<a
									href="/settings"
									class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800"
									onclick={() => (mobileMenuOpen = false)}
								>
									Settings
								</a>
								<button
									onclick={() => {
										logout();
										mobileMenuOpen = false;
									}}
									class="block w-full px-4 py-2 text-left text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800"
								>
									Sign out
								</button>
							</div>
						{:else}
							<div class="space-y-1">
								<a
									href="/auth/login"
									class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800"
									onclick={() => (mobileMenuOpen = false)}
								>
									Sign in
								</a>
								<a
									href="/auth/signup"
									class="mx-4 block rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 text-center text-base font-medium text-white hover:from-orange-700 hover:to-red-700"
									onclick={() => (mobileMenuOpen = false)}
								>
									Get started
								</a>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</nav>
	</header>

	<!-- Main Content -->
	<main class="flex-1">
		{@render children?.()}
	</main>

	<!-- Footer -->
	<footer class="bg-gray-900 text-white">
		<div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
				<!-- About Section -->
				<div>
					<h3 class="mb-4 text-lg font-semibold">About Rust Atlanta</h3>
					<p class="text-sm text-gray-400">
						We're a community of Rust enthusiasts in the Atlanta metro area. Join us to learn,
						share, and build amazing things with Rust.
					</p>
				</div>

				<!-- Quick Links -->
				<div>
					<h3 class="mb-4 text-lg font-semibold">Quick Links</h3>
					<ul class="space-y-2 text-sm">
						<li>
							<a href="/code-of-conduct" class="text-gray-400 hover:text-white">Code of Conduct</a>
						</li>
						<li>
							<a href="https://github.com/rust-atlanta" class="text-gray-400 hover:text-white"
								>GitHub Organization</a
							>
						</li>
						<li>
							<a href="https://discord.gg/rust-atlanta" class="text-gray-400 hover:text-white"
								>Discord Server</a
							>
						</li>
						<li><a href="/sponsors" class="text-gray-400 hover:text-white">Sponsors</a></li>
					</ul>
				</div>

				<!-- Newsletter Signup -->
				<div>
					<h3 class="mb-4 text-lg font-semibold">Stay Updated</h3>
					<p class="mb-4 text-sm text-gray-400">
						Get the latest news about events, projects, and community updates.
					</p>
					<form onsubmit={handleNewsletterSubmit} class="flex flex-col space-y-2">
						<div class="flex">
							<input
								type="email"
								bind:value={email}
								required
								placeholder="Enter your email"
								class="flex-1 rounded-l-md border-0 bg-gray-800 px-3 py-2 text-white placeholder-gray-500 focus:ring-2 focus:ring-orange-500"
								disabled={newsletterStatus === 'loading'}
							/>
							<button
								type="submit"
								disabled={newsletterStatus === 'loading'}
								class="rounded-r-md bg-orange-600 px-4 py-2 font-medium text-white hover:bg-orange-700 focus:ring-2 focus:ring-orange-500 focus:outline-none disabled:opacity-50"
							>
								{#if newsletterStatus === 'loading'}
									<span class="inline-block animate-spin">⏳</span>
								{:else}
									Subscribe
								{/if}
							</button>
						</div>
						{#if newsletterStatus === 'success'}
							<p class="text-sm text-green-400">✓ Successfully subscribed!</p>
						{:else if newsletterStatus === 'error'}
							<p class="text-sm text-red-400">× Something went wrong. Please try again.</p>
						{/if}
					</form>
				</div>
			</div>

			<div class="mt-8 border-t border-gray-800 pt-8">
				<p class="text-center text-sm text-gray-400">
					© {new Date().getFullYear()} Rust Atlanta. Built with 🦀 and ❤️ in Atlanta.
				</p>
			</div>
		</div>
	</footer>
</div>
