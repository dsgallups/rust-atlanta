<script lang="ts">
	interface Project {
		id: number;
		name: string;
		description: string;
		author: string;
		authorAvatar?: string;
		githubUrl?: string;
		websiteUrl?: string;
		stars: number;
		language: string;
		tags: string[];
		createdAt: Date;
		featured: boolean;
	}

	let projects = $state<Project[]>([
		{
			id: 1,
			name: 'RustATL CLI Tools',
			description:
				'A comprehensive collection of command-line tools built by the Atlanta Rust community. Includes utilities for file processing, system monitoring, and development workflows.',
			author: 'sarah_dev',
			githubUrl: 'https://github.com/rust-atlanta/cli-tools',
			websiteUrl: 'https://cli-tools.rust-atlanta.dev',
			stars: 127,
			language: 'Rust',
			tags: ['cli', 'tools', 'utilities', 'open-source'],
			createdAt: new Date('2024-01-15'),
			featured: true
		},
		{
			id: 2,
			name: 'Peachtree Parser',
			description:
				'High-performance JSON/YAML parser optimized for large files. Built with zero-copy deserialization and streaming capabilities.',
			author: 'alex_rustacean',
			githubUrl: 'https://github.com/rust-atlanta/peachtree-parser',
			stars: 89,
			language: 'Rust',
			tags: ['parser', 'json', 'yaml', 'performance'],
			createdAt: new Date('2024-01-10'),
			featured: true
		},
		{
			id: 3,
			name: 'Atlanta Transit API',
			description:
				'Rust wrapper for MARTA real-time transit data. Provides type-safe access to schedules, alerts, and vehicle positions.',
			author: 'transit_hacker',
			githubUrl: 'https://github.com/rust-atlanta/transit-api',
			websiteUrl: 'https://transit.rust-atlanta.dev',
			stars: 64,
			language: 'Rust',
			tags: ['api', 'transit', 'marta', 'real-time'],
			createdAt: new Date('2023-12-20'),
			featured: false
		},
		{
			id: 4,
			name: 'Peach State DB',
			description:
				'Embedded database written in pure Rust, optimized for IoT and edge computing scenarios.',
			author: 'db_wizard',
			githubUrl: 'https://github.com/rust-atlanta/peachstate-db',
			stars: 156,
			language: 'Rust',
			tags: ['database', 'embedded', 'iot', 'storage'],
			createdAt: new Date('2023-11-15'),
			featured: true
		},
		{
			id: 5,
			name: 'Rust Game Engine',
			description:
				'Experimental 2D game engine built during our monthly hackathons. Features ECS architecture and WebAssembly support.',
			author: 'game_dev_atl',
			githubUrl: 'https://github.com/rust-atlanta/game-engine',
			stars: 43,
			language: 'Rust',
			tags: ['gamedev', 'wasm', 'graphics', 'ecs'],
			createdAt: new Date('2023-10-05'),
			featured: false
		},
		{
			id: 6,
			name: 'ATL Weather CLI',
			description:
				'Terminal-based weather app with beautiful ASCII art visualizations for Atlanta area forecasts.',
			author: 'weather_rustacean',
			githubUrl: 'https://github.com/rust-atlanta/weather-cli',
			stars: 31,
			language: 'Rust',
			tags: ['cli', 'weather', 'api', 'terminal-ui'],
			createdAt: new Date('2023-09-20'),
			featured: false
		}
	]);

	let filteredProjects = $derived(
		projects
			.filter((p) => {
				if (selectedTags.length === 0) return true;
				return selectedTags.some((tag) => p.tags.includes(tag));
			})
			.filter((p) => {
				if (!searchQuery) return true;
				const query = searchQuery.toLowerCase();
				return (
					p.name.toLowerCase().includes(query) ||
					p.description.toLowerCase().includes(query) ||
					p.author.toLowerCase().includes(query) ||
					p.tags.some((tag) => tag.toLowerCase().includes(query))
				);
			})
			.sort((a, b) => {
				switch (sortBy) {
					case 'stars':
						return b.stars - a.stars;
					case 'recent':
						return b.createdAt.getTime() - a.createdAt.getTime();
					case 'name':
						return a.name.localeCompare(b.name);
					default:
						return 0;
				}
			})
	);

	let searchQuery = $state('');
	let sortBy = $state<'stars' | 'recent' | 'name'>('stars');
	let selectedTags = $state<string[]>([]);
	let showSubmitForm = $state(false);

	// Form state
	let formData = $state({
		name: '',
		description: '',
		githubUrl: '',
		websiteUrl: '',
		tags: ''
	});

	let allTags = $derived(Array.from(new Set(projects.flatMap((p) => p.tags))).sort());

	function toggleTag(tag: string) {
		if (selectedTags.includes(tag)) {
			selectedTags = selectedTags.filter((t) => t !== tag);
		} else {
			selectedTags = [...selectedTags, tag];
		}
	}

	async function handleSubmitProject(e: Event) {
		e.preventDefault();

		// Mock API call
		console.log('Submitting project:', formData);

		// Add the new project to the list (in a real app, this would come from the API)
		const newProject: Project = {
			id: projects.length + 1,
			name: formData.name,
			description: formData.description,
			author: 'current_user', // Would come from auth
			githubUrl: formData.githubUrl || undefined,
			websiteUrl: formData.websiteUrl || undefined,
			stars: 0,
			language: 'Rust',
			tags: formData.tags
				.split(',')
				.map((t) => t.trim())
				.filter(Boolean),
			createdAt: new Date(),
			featured: false
		};

		projects = [...projects, newProject];

		// Reset form
		formData = {
			name: '',
			description: '',
			githubUrl: '',
			websiteUrl: '',
			tags: ''
		};

		showSubmitForm = false;
	}

	function formatDate(date: Date): string {
		return new Intl.DateTimeFormat('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		}).format(date);
	}
</script>

<div class="min-h-screen bg-gray-50">
	<!-- Header -->
	<div class="border-b border-gray-200 bg-white">
		<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
			<div class="md:flex md:items-center md:justify-between">
				<div>
					<h1 class="text-3xl font-bold text-gray-900">Community Projects</h1>
					<p class="mt-2 text-gray-600">
						Discover amazing projects built by the Rust Atlanta community
					</p>
				</div>
				<button
					onclick={() => (showSubmitForm = true)}
					class="mt-4 inline-flex items-center gap-2 rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 font-medium text-white transition-shadow hover:shadow-lg md:mt-0"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 4v16m8-8H4"
						/>
					</svg>
					Submit Project
				</button>
			</div>
		</div>
	</div>

	<!-- Filters and Search -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<div class="space-y-4">
			<!-- Search and Sort -->
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<div class="relative max-w-md flex-1">
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="Search projects..."
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
				<select
					bind:value={sortBy}
					class="rounded-lg border border-gray-300 bg-white px-4 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
				>
					<option value="stars">Most Stars</option>
					<option value="recent">Most Recent</option>
					<option value="name">Name (A-Z)</option>
				</select>
			</div>

			<!-- Tags Filter -->
			<div class="flex flex-wrap gap-2">
				{#each allTags as tag}
					<button
						onclick={() => toggleTag(tag)}
						class="rounded-full px-3 py-1 text-sm font-medium transition-colors {selectedTags.includes(
							tag
						)
							? 'bg-orange-600 text-white'
							: 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50'}"
					>
						{tag}
					</button>
				{/each}
				{#if selectedTags.length > 0}
					<button
						onclick={() => (selectedTags = [])}
						class="rounded-full px-3 py-1 text-sm font-medium text-gray-600 hover:text-gray-800"
					>
						Clear all
					</button>
				{/if}
			</div>
		</div>
	</div>

	<!-- Projects Grid -->
	<div class="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">
		{#if filteredProjects.length === 0}
			<div class="py-12 text-center">
				<div class="mb-4 text-6xl">🔍</div>
				<p class="text-gray-600">No projects found matching your criteria</p>
			</div>
		{:else}
			<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
				{#each filteredProjects as project}
					<div
						class="group relative rounded-lg border border-gray-200 bg-white p-6 transition-all hover:shadow-xl"
					>
						{#if project.featured}
							<div class="absolute -top-2 -right-2">
								<span
									class="inline-flex items-center rounded-full bg-gradient-to-r from-orange-600 to-red-600 px-2 py-1 text-xs font-medium text-white"
								>
									⭐ Featured
								</span>
							</div>
						{/if}

						<div class="mb-4">
							<h3
								class="text-lg font-semibold text-gray-900 transition-colors group-hover:text-orange-600"
							>
								{project.name}
							</h3>
							<div class="mt-1 flex items-center gap-2 text-sm text-gray-600">
								<span>by @{project.author}</span>
								<span>•</span>
								<span>{formatDate(project.createdAt)}</span>
							</div>
						</div>

						<p class="mb-4 line-clamp-3 text-gray-600">{project.description}</p>

						<div class="mb-4 flex flex-wrap gap-1">
							{#each project.tags as tag}
								<span class="rounded bg-gray-100 px-2 py-1 text-xs text-gray-700">
									{tag}
								</span>
							{/each}
						</div>

						<div class="flex items-center justify-between">
							<div class="flex items-center gap-3">
								{#if project.githubUrl}
									<a
										href={project.githubUrl}
										target="_blank"
										rel="noopener noreferrer"
										class="text-gray-600 hover:text-gray-900"
									>
										<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
											<path
												d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
											/>
										</svg>
									</a>
								{/if}
								{#if project.websiteUrl}
									<a
										href={project.websiteUrl}
										target="_blank"
										rel="noopener noreferrer"
										class="text-gray-600 hover:text-gray-900"
									>
										<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
											/>
										</svg>
									</a>
								{/if}
							</div>
							<div class="flex items-center gap-1 text-gray-600">
								<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
									<path
										d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"
									/>
								</svg>
								<span class="text-sm font-medium">{project.stars}</span>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- Submit Project Modal -->
{#if showSubmitForm}
	<div class="fixed inset-0 z-50 overflow-y-auto">
		<div
			class="flex min-h-screen items-center justify-center px-4 pt-4 pb-20 text-center sm:block sm:p-0"
		>
			<!-- Background overlay -->
			<div
				class="bg-opacity-75 fixed inset-0 bg-gray-500 transition-opacity"
				onclick={() => (showSubmitForm = false)}
			></div>

			<!-- Modal panel -->
			<div
				class="inline-block w-full max-w-lg transform overflow-hidden rounded-lg bg-white text-left align-middle shadow-xl transition-all"
			>
				<div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
					<div class="mb-4">
						<h3 class="text-lg font-semibold text-gray-900">Submit Your Project</h3>
						<p class="mt-1 text-sm text-gray-600">
							Share your Rust project with the Atlanta community
						</p>
					</div>

					<form onsubmit={handleSubmitProject} class="space-y-4">
						<div>
							<label for="name" class="block text-sm font-medium text-gray-700">
								Project Name *
							</label>
							<input
								id="name"
								type="text"
								bind:value={formData.name}
								required
								class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
							/>
						</div>

						<div>
							<label for="description" class="block text-sm font-medium text-gray-700">
								Description *
							</label>
							<textarea
								id="description"
								bind:value={formData.description}
								required
								rows="3"
								class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
							></textarea>
						</div>

						<div>
							<label for="github" class="block text-sm font-medium text-gray-700">
								GitHub URL
							</label>
							<input
								id="github"
								type="url"
								bind:value={formData.githubUrl}
								placeholder="https://github.com/..."
								class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
							/>
						</div>

						<div>
							<label for="website" class="block text-sm font-medium text-gray-700">
								Website URL
							</label>
							<input
								id="website"
								type="url"
								bind:value={formData.websiteUrl}
								placeholder="https://..."
								class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
							/>
						</div>

						<div>
							<label for="tags" class="block text-sm font-medium text-gray-700">
								Tags (comma-separated)
							</label>
							<input
								id="tags"
								type="text"
								bind:value={formData.tags}
								placeholder="cli, api, wasm, etc."
								class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-orange-500 focus:ring-2 focus:ring-orange-500 focus:outline-none"
							/>
						</div>

						<div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
							<button
								type="submit"
								class="inline-flex w-full justify-center rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 text-base font-medium text-white shadow-sm hover:shadow-lg focus:ring-2 focus:ring-orange-500 focus:outline-none sm:col-start-2 sm:text-sm"
							>
								Submit Project
							</button>
							<button
								type="button"
								onclick={() => (showSubmitForm = false)}
								class="mt-3 inline-flex w-full justify-center rounded-lg border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:ring-2 focus:ring-orange-500 focus:outline-none sm:col-start-1 sm:mt-0 sm:text-sm"
							>
								Cancel
							</button>
						</div>
					</form>
				</div>
			</div>
		</div>
	</div>
{/if}
