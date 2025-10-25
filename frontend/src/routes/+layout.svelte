<script lang="ts">
	import '../app.css';
	import { page } from '$app/state';

	let { children } = $props();
	let mobileMenuOpen = $state(false);
	let email = $state('');
	let newsletterStatus = $state<'idle' | 'loading' | 'success' | 'error'>('idle');

	const navItems = [
		{ href: '/', label: 'Home' },
		{ href: '/projects', label: 'Projects' },
		{ href: '/events', label: 'Events' },
		{ href: '/news', label: 'News' },
		{ href: '/resources', label: 'Resources' },
		{ href: '/about', label: 'About' }
	];

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
