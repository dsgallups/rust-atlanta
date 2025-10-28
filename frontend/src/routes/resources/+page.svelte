<script lang="ts">
	interface Resource {
		id: number;
		title: string;
		description: string;
		url: string;
		category:
			| 'learning'
			| 'tools'
			| 'libraries'
			| 'documentation'
			| 'community'
			| 'books'
			| 'videos';
		difficulty?: 'beginner' | 'intermediate' | 'advanced';
		tags: string[];
		icon?: string;
		recommended: boolean;
	}

	let resources = $state<Resource[]>([
		// Learning Resources
		{
			id: 1,
			title: 'The Rust Programming Language Book',
			description:
				'The official book on Rust, affectionately known as "The Book". Perfect for beginners.',
			url: 'https://doc.rust-lang.org/book/',
			category: 'books',
			difficulty: 'beginner',
			tags: ['official', 'beginner', 'comprehensive'],
			icon: '📚',
			recommended: true
		},
		{
			id: 2,
			title: 'Rust by Example',
			description: 'Learn Rust through annotated example programs. Great for hands-on learners.',
			url: 'https://doc.rust-lang.org/rust-by-example/',
			category: 'learning',
			difficulty: 'beginner',
			tags: ['examples', 'interactive', 'practical'],
			icon: '💻',
			recommended: true
		},
		{
			id: 3,
			title: 'Rustlings',
			description: 'Small exercises to get you used to reading and writing Rust code.',
			url: 'https://github.com/rust-lang/rustlings',
			category: 'learning',
			difficulty: 'beginner',
			tags: ['exercises', 'interactive', 'beginner'],
			icon: '🎯',
			recommended: true
		},
		{
			id: 4,
			title: 'The Async Book',
			description: 'Learn asynchronous programming in Rust with this comprehensive guide.',
			url: 'https://rust-lang.github.io/async-book/',
			category: 'books',
			difficulty: 'intermediate',
			tags: ['async', 'concurrent', 'advanced'],
			icon: '⚡',
			recommended: false
		},
		{
			id: 5,
			title: 'Rust Design Patterns',
			description: 'A catalogue of Rust design patterns, anti-patterns and idioms.',
			url: 'https://rust-unofficial.github.io/patterns/',
			category: 'documentation',
			difficulty: 'intermediate',
			tags: ['patterns', 'best-practices', 'architecture'],
			icon: '🏗️',
			recommended: true
		},

		// Tools
		{
			id: 6,
			title: 'rust-analyzer',
			description:
				'The official Rust language server for IDEs. Provides code completion, goto definition, and more.',
			url: 'https://rust-analyzer.github.io/',
			category: 'tools',
			tags: ['ide', 'development', 'productivity'],
			icon: '🔧',
			recommended: true
		},
		{
			id: 7,
			title: 'Cargo',
			description: 'The Rust package manager and build tool. Essential for any Rust project.',
			url: 'https://doc.rust-lang.org/cargo/',
			category: 'tools',
			tags: ['build-tool', 'package-manager', 'essential'],
			icon: '📦',
			recommended: true
		},
		{
			id: 8,
			title: 'Clippy',
			description: 'A collection of lints to catch common mistakes and improve your Rust code.',
			url: 'https://github.com/rust-lang/rust-clippy',
			category: 'tools',
			tags: ['linting', 'code-quality', 'analysis'],
			icon: '📎',
			recommended: true
		},
		{
			id: 9,
			title: 'rustfmt',
			description:
				'Automatic code formatter for Rust. Ensures consistent code style across your projects.',
			url: 'https://github.com/rust-lang/rustfmt',
			category: 'tools',
			tags: ['formatting', 'code-style', 'automation'],
			icon: '✨',
			recommended: true
		},
		{
			id: 10,
			title: 'cargo-watch',
			description:
				"Watches over your project's source for changes and runs commands when they occur.",
			url: 'https://github.com/watchexec/cargo-watch',
			category: 'tools',
			tags: ['development', 'automation', 'workflow'],
			icon: '👁️',
			recommended: false
		},

		// Libraries & Frameworks
		{
			id: 11,
			title: 'Tokio',
			description:
				'An asynchronous runtime for Rust, powering fast and reliable network applications.',
			url: 'https://tokio.rs/',
			category: 'libraries',
			tags: ['async', 'networking', 'runtime'],
			icon: '🚀',
			recommended: true
		},
		{
			id: 12,
			title: 'Actix Web',
			description: 'A powerful, pragmatic, and extremely fast web framework for Rust.',
			url: 'https://actix.rs/',
			category: 'libraries',
			tags: ['web', 'framework', 'http'],
			icon: '🌐',
			recommended: true
		},
		{
			id: 13,
			title: 'Rocket',
			description:
				'A web framework for Rust that makes it simple to write fast, secure web applications.',
			url: 'https://rocket.rs/',
			category: 'libraries',
			tags: ['web', 'framework', 'type-safe'],
			icon: '🚀',
			recommended: false
		},
		{
			id: 14,
			title: 'Serde',
			description:
				'A framework for serializing and deserializing Rust data structures efficiently.',
			url: 'https://serde.rs/',
			category: 'libraries',
			tags: ['serialization', 'json', 'data'],
			icon: '📋',
			recommended: true
		},
		{
			id: 15,
			title: 'Diesel',
			description: 'A safe, extensible ORM and Query Builder for Rust.',
			url: 'https://diesel.rs/',
			category: 'libraries',
			tags: ['database', 'orm', 'sql'],
			icon: '🗄️',
			recommended: false
		},
		{
			id: 16,
			title: 'Bevy',
			description: 'A refreshingly simple data-driven game engine built in Rust.',
			url: 'https://bevyengine.org/',
			category: 'libraries',
			tags: ['gamedev', 'engine', 'ecs'],
			icon: '🎮',
			recommended: false
		},

		// Documentation
		{
			id: 17,
			title: 'Rust Standard Library',
			description: 'Complete documentation for the Rust standard library.',
			url: 'https://doc.rust-lang.org/std/',
			category: 'documentation',
			tags: ['reference', 'api', 'official'],
			icon: '📖',
			recommended: true
		},
		{
			id: 18,
			title: 'Rust Reference',
			description: 'The primary reference for the Rust programming language.',
			url: 'https://doc.rust-lang.org/reference/',
			category: 'documentation',
			tags: ['reference', 'language', 'specification'],
			icon: '📜',
			recommended: false
		},
		{
			id: 19,
			title: 'crates.io',
			description: 'The Rust package registry. Find and publish Rust crates.',
			url: 'https://crates.io/',
			category: 'community',
			tags: ['packages', 'registry', 'ecosystem'],
			icon: '📦',
			recommended: true
		},
		{
			id: 20,
			title: 'docs.rs',
			description: 'Documentation for all crates published to crates.io.',
			url: 'https://docs.rs/',
			category: 'documentation',
			tags: ['documentation', 'crates', 'api'],
			icon: '📚',
			recommended: true
		},

		// Community Resources
		{
			id: 21,
			title: 'Rust Users Forum',
			description: 'The official Rust users forum for questions and discussions.',
			url: 'https://users.rust-lang.org/',
			category: 'community',
			tags: ['forum', 'discussion', 'help'],
			icon: '💬',
			recommended: false
		},
		{
			id: 22,
			title: 'This Week in Rust',
			description: 'Stay up-to-date with the latest Rust news and developments.',
			url: 'https://this-week-in-rust.org/',
			category: 'community',
			tags: ['news', 'updates', 'newsletter'],
			icon: '📰',
			recommended: true
		},
		{
			id: 23,
			title: 'Rust Playground',
			description: 'Run and share Rust code snippets in your browser.',
			url: 'https://play.rust-lang.org/',
			category: 'tools',
			tags: ['playground', 'online', 'sharing'],
			icon: '🎨',
			recommended: true
		},
		{
			id: 24,
			title: 'Are We Web Yet?',
			description: 'Overview of the current state of web development in Rust.',
			url: 'https://www.arewewebyet.org/',
			category: 'community',
			tags: ['web', 'ecosystem', 'overview'],
			icon: '🕸️',
			recommended: false
		},

		// Videos & Courses
		{
			id: 25,
			title: 'Rust Programming Course by FreeCodeCamp',
			description: 'Comprehensive video course covering Rust fundamentals.',
			url: 'https://www.youtube.com/watch?v=MsocPEZBd-M',
			category: 'videos',
			difficulty: 'beginner',
			tags: ['video', 'course', 'free'],
			icon: '🎥',
			recommended: false
		},
		{
			id: 26,
			title: 'Jon Gjengset - Rust Streams',
			description: 'In-depth Rust programming streams covering advanced topics.',
			url: 'https://www.youtube.com/@jonhoo',
			category: 'videos',
			difficulty: 'advanced',
			tags: ['video', 'streaming', 'advanced'],
			icon: '📹',
			recommended: true
		}
	]);

	let searchQuery = $state('');
	let selectedCategory = $state<string>('all');
	let selectedDifficulty = $state<string>('all');
	let showRecommendedOnly = $state(false);

	let categories = [
		{ value: 'all', label: 'All Resources', icon: '🌟' },
		{ value: 'learning', label: 'Learning', icon: '📚' },
		{ value: 'tools', label: 'Tools', icon: '🔧' },
		{ value: 'libraries', label: 'Libraries', icon: '📦' },
		{ value: 'documentation', label: 'Documentation', icon: '📖' },
		{ value: 'community', label: 'Community', icon: '👥' },
		{ value: 'books', label: 'Books', icon: '📚' },
		{ value: 'videos', label: 'Videos', icon: '🎥' }
	];

	let difficulties = [
		{ value: 'all', label: 'All Levels' },
		{ value: 'beginner', label: 'Beginner', color: 'bg-green-100 text-green-800' },
		{ value: 'intermediate', label: 'Intermediate', color: 'bg-yellow-100 text-yellow-800' },
		{ value: 'advanced', label: 'Advanced', color: 'bg-red-100 text-red-800' }
	];

	let filteredResources = $derived(
		resources.filter((resource) => {
			if (showRecommendedOnly && !resource.recommended) return false;
			if (selectedCategory !== 'all' && resource.category !== selectedCategory) return false;
			if (selectedDifficulty !== 'all' && resource.difficulty !== selectedDifficulty) return false;
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				return (
					resource.title.toLowerCase().includes(query) ||
					resource.description.toLowerCase().includes(query) ||
					resource.tags.some((tag) => tag.toLowerCase().includes(query))
				);
			}
			return true;
		})
	);

	let groupedResources = $derived(() => {
		const groups: Record<string, Resource[]> = {};
		for (const resource of filteredResources) {
			if (!groups[resource.category]) {
				groups[resource.category] = [];
			}
			groups[resource.category].push(resource);
		}
		return groups;
	});

	function getCategoryLabel(category: string): string {
		const cat = categories.find((c) => c.value === category);
		return cat?.label || category;
	}

	function getCategoryIcon(category: string): string {
		const cat = categories.find((c) => c.value === category);
		return cat?.icon || '📄';
	}

	function getDifficultyColor(difficulty?: string): string {
		const diff = difficulties.find((d) => d.value === difficulty);
		return diff?.color || 'bg-gray-100 text-gray-800';
	}
</script>

<div class="min-h-screen bg-gray-50">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white">
		<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
			<div class="text-center">
				<h1 class="text-3xl font-bold text-gray-900">Resources</h1>
				<p class="mt-2 text-gray-600">Everything you need to learn and master Rust programming</p>
			</div>
		</div>
	</div>

	<!-- Quick Start Section -->
	<div class="border-b border-orange-200 bg-gradient-to-r from-orange-50 to-red-50">
		<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
			<div class="mb-6 text-center">
				<h2 class="text-2xl font-bold text-gray-900">🚀 Quick Start Guide</h2>
				<p class="mt-2 text-gray-600">New to Rust? Start here!</p>
			</div>
			<div class="grid gap-4 md:grid-cols-3">
				<div class="rounded-lg bg-white p-4 shadow-sm">
					<div class="mb-2 text-2xl">1️⃣</div>
					<h3 class="mb-1 font-semibold text-gray-900">Install Rust</h3>
					<p class="mb-2 text-sm text-gray-600">Get Rust installed on your machine</p>
					<a
						href="https://rustup.rs/"
						target="_blank"
						rel="noopener noreferrer"
						class="text-sm font-medium text-orange-600 hover:text-orange-700"
					>
						rustup.rs →
					</a>
				</div>
				<div class="rounded-lg bg-white p-4 shadow-sm">
					<div class="mb-2 text-2xl">2️⃣</div>
					<h3 class="mb-1 font-semibold text-gray-900">Read The Book</h3>
					<p class="mb-2 text-sm text-gray-600">The official guide to learning Rust</p>
					<a
						href="https://doc.rust-lang.org/book/"
						target="_blank"
						rel="noopener noreferrer"
						class="text-sm font-medium text-orange-600 hover:text-orange-700"
					>
						Start reading →
					</a>
				</div>
				<div class="rounded-lg bg-white p-4 shadow-sm">
					<div class="mb-2 text-2xl">3️⃣</div>
					<h3 class="mb-1 font-semibold text-gray-900">Practice with Rustlings</h3>
					<p class="mb-2 text-sm text-gray-600">Small exercises to get comfortable</p>
					<a
						href="https://github.com/rust-lang/rustlings"
						target="_blank"
						rel="noopener noreferrer"
						class="text-sm font-medium text-orange-600 hover:text-orange-700"
					>
						Start exercises →
					</a>
				</div>
			</div>
		</div>
	</div>

	<!-- Filters -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<div class="space-y-4">
			<!-- Search -->
			<div class="relative max-w-md">
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Search resources..."
					class="w-full rounded-lg border border-gray-300 bg-white px-4 py-2 pl-10 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
				/>
				<svg
					class="absolute top-2.5 left-3 h-5 w-5 text-gray-400"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/>
				</svg>
			</div>

			<!-- Filters -->
			<div class="flex flex-wrap items-center gap-4">
				<!-- Category Filter -->
				<select
					bind:value={selectedCategory}
					class="rounded-lg border border-gray-300 bg-white px-4 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
				>
					{#each categories as category}
						<option value={category.value}>
							{category.icon}
							{category.label}
						</option>
					{/each}
				</select>

				<!-- Difficulty Filter -->
				<select
					bind:value={selectedDifficulty}
					class="rounded-lg border border-gray-300 bg-white px-4 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
				>
					{#each difficulties as difficulty}
						<option value={difficulty.value}>{difficulty.label}</option>
					{/each}
				</select>

				<!-- Recommended Toggle -->
				<label class="flex cursor-pointer items-center gap-2">
					<input
						type="checkbox"
						bind:checked={showRecommendedOnly}
						class="rounded border-gray-300 text-orange-600 focus:ring-orange-500"
					/>
					<span class="text-gray-700">Recommended only</span>
				</label>
			</div>
		</div>
	</div>

	<!-- Resources Grid -->
	<div class="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">
		{#if filteredResources.length === 0}
			<div class="py-12 text-center">
				<div class="mb-4 text-6xl">🔍</div>
				<p class="text-gray-600">No resources found matching your criteria</p>
			</div>
		{:else if selectedCategory === 'all'}
			<!-- Grouped by Category -->
			{#each Object.entries(groupedResources()) as [category, categoryResources]}
				<div class="mb-8">
					<h3 class="mb-4 flex items-center gap-2 text-xl font-bold text-gray-900">
						<span class="text-2xl">{getCategoryIcon(category)}</span>
						{getCategoryLabel(category)}
					</h3>
					<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
						{#each categoryResources as resource}
							<a
								href={resource.url}
								target="_blank"
								rel="noopener noreferrer"
								class="group relative rounded-lg border border-gray-200 bg-white p-6 transition-shadow hover:shadow-lg"
							>
								{#if resource.recommended}
									<div class="absolute -top-2 -right-2">
										<span
											class="inline-flex items-center rounded-full bg-gradient-to-r from-orange-600 to-red-600 px-2 py-1 text-xs font-medium text-white"
										>
											⭐ Recommended
										</span>
									</div>
								{/if}

								<div class="flex items-start gap-3">
									<span class="text-3xl">{resource.icon || '📄'}</span>
									<div class="flex-1">
										<h4
											class="mb-1 font-semibold text-gray-900 transition-colors group-hover:text-orange-600"
										>
											{resource.title}
											<svg
												class="ml-1 inline h-4 w-4"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
												/>
											</svg>
										</h4>
										<p class="mb-3 text-sm text-gray-600">
											{resource.description}
										</p>
										<div class="flex flex-wrap gap-1">
											{#if resource.difficulty}
												<span
													class="rounded-full px-2 py-1 text-xs font-medium {getDifficultyColor(
														resource.difficulty
													)}"
												>
													{resource.difficulty}
												</span>
											{/if}
											{#each resource.tags.slice(0, 2) as tag}
												<span class="rounded bg-gray-100 px-2 py-1 text-xs text-gray-700">
													{tag}
												</span>
											{/each}
										</div>
									</div>
								</div>
							</a>
						{/each}
					</div>
				</div>
			{/each}
		{:else}
			<!-- Single Category View -->
			<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
				{#each filteredResources as resource}
					<a
						href={resource.url}
						target="_blank"
						rel="noopener noreferrer"
						class="group relative rounded-lg border border-gray-200 bg-white p-6 transition-shadow hover:shadow-lg"
					>
						{#if resource.recommended}
							<div class="absolute -top-2 -right-2">
								<span
									class="inline-flex items-center rounded-full bg-gradient-to-r from-orange-600 to-red-600 px-2 py-1 text-xs font-medium text-white"
								>
									⭐ Recommended
								</span>
							</div>
						{/if}

						<div class="flex items-start gap-3">
							<span class="text-3xl">{resource.icon || '📄'}</span>
							<div class="flex-1">
								<h4
									class="mb-1 font-semibold text-gray-900 transition-colors group-hover:text-orange-600"
								>
									{resource.title}
									<svg
										class="ml-1 inline h-4 w-4"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
										/>
									</svg>
								</h4>
								<p class="mb-3 text-sm text-gray-600">
									{resource.description}
								</p>
								<div class="flex flex-wrap gap-1">
									{#if resource.difficulty}
										<span
											class="rounded-full px-2 py-1 text-xs font-medium {getDifficultyColor(
												resource.difficulty
											)}"
										>
											{resource.difficulty}
										</span>
									{/if}
									{#each resource.tags.slice(0, 2) as tag}
										<span class="rounded bg-gray-100 px-2 py-1 text-xs text-gray-700">
											{tag}
										</span>
									{/each}
								</div>
							</div>
						</div>
					</a>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Additional Resources CTA -->
	<div class="bg-gray-100 py-12">
		<div class="mx-auto max-w-7xl px-4 text-center sm:px-6 lg:px-8">
			<h2 class="mb-4 text-2xl font-bold text-gray-900">Have a Resource to Share?</h2>
			<p class="mb-6 text-gray-600">
				Know of a great Rust resource that's not listed here? We'd love to add it!
			</p>
			<a
				href="https://github.com/rust-atlanta/resources"
				target="_blank"
				rel="noopener noreferrer"
				class="inline-flex items-center gap-2 rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-6 py-3 font-medium text-white transition-shadow hover:shadow-lg"
			>
				<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
					<path
						d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
					/>
				</svg>
				Submit a Resource
			</a>
		</div>
	</div>
</div>
