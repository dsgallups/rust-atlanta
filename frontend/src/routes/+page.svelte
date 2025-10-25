<script lang="ts">
	import { goto } from '$app/navigation';

	// Mock data for featured content
	const featuredProjects = [
		{
			id: 1,
			name: 'RustATL CLI Tools',
			description: 'A collection of command-line tools built by the community',
			stars: 127,
			language: 'Rust',
			author: 'sarah_dev',
			url: 'https://github.com/rust-atlanta/cli-tools'
		},
		{
			id: 2,
			name: 'Peachtree Parser',
			description: 'High-performance JSON/YAML parser optimized for large files',
			stars: 89,
			language: 'Rust',
			author: 'alex_rustacean',
			url: 'https://github.com/rust-atlanta/peachtree-parser'
		},
		{
			id: 3,
			name: 'Atlanta Transit API',
			description: 'Rust wrapper for MARTA real-time transit data',
			stars: 64,
			language: 'Rust',
			author: 'transit_hacker',
			url: 'https://github.com/rust-atlanta/transit-api'
		}
	];

	const upcomingEvents = [
		{
			id: 1,
			title: 'Rust Beginners Workshop',
			date: new Date('2024-02-15'),
			time: '6:30 PM',
			location: 'Tech Village Atlanta',
			type: 'workshop',
			attendees: 24
		},
		{
			id: 2,
			title: 'Monthly Rust Meetup',
			date: new Date('2024-02-20'),
			time: '7:00 PM',
			location: 'Atlanta Tech Park',
			type: 'meetup',
			attendees: 45
		},
		{
			id: 3,
			title: 'Async Rust Deep Dive',
			date: new Date('2024-02-28'),
			time: '6:00 PM',
			location: 'Online',
			type: 'workshop',
			attendees: 67
		}
	];

	const latestNews = [
		{
			id: 1,
			title: 'Rust Atlanta Hackathon Winners Announced',
			excerpt: 'Congratulations to Team Ferris for their innovative WebAssembly project...',
			date: new Date('2024-01-28'),
			author: 'Community Team'
		},
		{
			id: 2,
			title: 'New Rust Study Group Starting February',
			excerpt:
				'Join our beginner-friendly study group as we work through the Rust Book together...',
			date: new Date('2024-01-25'),
			author: 'Education Committee'
		}
	];

	const stats = [
		{ label: 'Active Members', value: '350+', icon: '👥' },
		{ label: 'Projects Built', value: '47', icon: '🚀' },
		{ label: 'Events Hosted', value: '120+', icon: '📅' },
		{ label: 'Lines of Rust', value: '1.2M+', icon: '💻' }
	];

	function formatDate(date: Date): string {
		return new Intl.DateTimeFormat('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		}).format(date);
	}

	function getEventTypeColor(type: string): string {
		switch (type) {
			case 'workshop':
				return 'bg-blue-100 text-blue-800';
			case 'meetup':
				return 'bg-green-100 text-green-800';
			case 'hackathon':
				return 'bg-purple-100 text-purple-800';
			default:
				return 'bg-gray-100 text-gray-800';
		}
	}
</script>

<!-- Hero Section -->
<section class="relative overflow-hidden bg-gradient-to-br from-orange-50 to-red-50">
	<div class="mx-auto max-w-7xl px-4 py-24 sm:px-6 lg:px-8">
		<div class="text-center">
			<h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl md:text-6xl">
				Welcome to <span
					class="bg-gradient-to-r from-orange-600 to-red-600 bg-clip-text text-transparent"
					>Rust Atlanta</span
				>
			</h1>
			<p class="mx-auto mt-6 max-w-2xl text-lg text-gray-600">
				Join Atlanta's thriving Rust community. Learn, collaborate, and build the future of systems
				programming together.
			</p>
			<div class="mt-10 flex justify-center gap-4">
				<button
					onclick={() => goto('/events')}
					class="rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-6 py-3 font-semibold text-white shadow-lg transition-shadow hover:shadow-xl"
				>
					Join Next Event
				</button>
				<button
					onclick={() => goto('/about#join')}
					class="rounded-lg border-2 border-gray-300 bg-white px-6 py-3 font-semibold text-gray-700 transition-colors hover:border-gray-400"
				>
					Become a Member
				</button>
			</div>
		</div>
	</div>

	<!-- Decorative Elements -->
	<div class="absolute top-0 right-0 -mt-10 -mr-10 opacity-10">
		<div class="text-[20rem] font-bold">🦀</div>
	</div>
</section>

<!-- Stats Section -->
<section class="bg-white py-12">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="grid grid-cols-2 gap-6 md:grid-cols-4">
			{#each stats as stat}
				<div class="text-center">
					<div class="mb-2 text-3xl">{stat.icon}</div>
					<div class="text-3xl font-bold text-gray-900">{stat.value}</div>
					<div class="text-sm text-gray-600">{stat.label}</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- Featured Projects -->
<section class="bg-gray-50 py-16">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h2 class="text-3xl font-bold text-gray-900">Community Projects</h2>
				<p class="mt-2 text-gray-600">Amazing projects built by our members</p>
			</div>
			<a
				href="/projects"
				class="flex items-center gap-1 font-medium text-orange-600 hover:text-orange-700"
			>
				View all
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
				</svg>
			</a>
		</div>

		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each featuredProjects as project}
				<div
					class="rounded-lg border border-gray-200 bg-white p-6 transition-shadow hover:shadow-lg"
				>
					<div class="mb-4 flex items-start justify-between">
						<h3 class="text-lg font-semibold text-gray-900">{project.name}</h3>
						<div class="flex items-center gap-1 text-gray-600">
							<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
								<path
									d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"
								/>
							</svg>
							<span class="text-sm">{project.stars}</span>
						</div>
					</div>
					<p class="mb-4 text-gray-600">{project.description}</p>
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-2">
							<div class="h-6 w-6 rounded-full bg-gradient-to-br from-orange-400 to-red-500"></div>
							<span class="text-sm text-gray-600">@{project.author}</span>
						</div>
						<span class="rounded-full bg-orange-100 px-2 py-1 text-xs font-medium text-orange-800">
							{project.language}
						</span>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- Upcoming Events -->
<section class="bg-white py-16">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h2 class="text-3xl font-bold text-gray-900">Upcoming Events</h2>
				<p class="mt-2 text-gray-600">Join us at our next gathering</p>
			</div>
			<a
				href="/events"
				class="flex items-center gap-1 font-medium text-orange-600 hover:text-orange-700"
			>
				All events
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
				</svg>
			</a>
		</div>

		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each upcomingEvents as event}
				<div
					class="rounded-lg border border-gray-200 bg-white p-6 transition-shadow hover:shadow-lg"
				>
					<div class="mb-4">
						<span
							class="rounded-full px-2 py-1 text-xs font-medium {getEventTypeColor(event.type)}"
						>
							{event.type}
						</span>
					</div>
					<h3 class="mb-2 text-lg font-semibold text-gray-900">{event.title}</h3>
					<div class="space-y-2 text-sm text-gray-600">
						<div class="flex items-center gap-2">
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
								/>
							</svg>
							{formatDate(event.date)}
						</div>
						<div class="flex items-center gap-2">
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
							{event.time}
						</div>
						<div class="flex items-center gap-2">
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
								/>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
								/>
							</svg>
							{event.location}
						</div>
					</div>
					<div class="mt-4 flex items-center justify-between">
						<div class="flex items-center gap-2 text-sm text-gray-600">
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
								/>
							</svg>
							{event.attendees} attending
						</div>
						<button class="text-sm font-medium text-orange-600 hover:text-orange-700">
							RSVP →
						</button>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- Latest News -->
<section class="bg-gray-50 py-16">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h2 class="text-3xl font-bold text-gray-900">Latest News</h2>
				<p class="mt-2 text-gray-600">Stay updated with our community</p>
			</div>
			<a
				href="/news"
				class="flex items-center gap-1 font-medium text-orange-600 hover:text-orange-700"
			>
				All news
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
				</svg>
			</a>
		</div>

		<div class="grid gap-6 md:grid-cols-2">
			{#each latestNews as article}
				<article
					class="rounded-lg border border-gray-200 bg-white p-6 transition-shadow hover:shadow-lg"
				>
					<div class="mb-4 flex items-center gap-2 text-sm text-gray-600">
						<time datetime={article.date.toISOString()}>{formatDate(article.date)}</time>
						<span>•</span>
						<span>{article.author}</span>
					</div>
					<h3 class="mb-2 text-lg font-semibold text-gray-900">{article.title}</h3>
					<p class="mb-4 text-gray-600">{article.excerpt}</p>
					<a href="/news/{article.id}" class="font-medium text-orange-600 hover:text-orange-700">
						Read more →
					</a>
				</article>
			{/each}
		</div>
	</div>
</section>

<!-- CTA Section -->
<section class="bg-gradient-to-r from-orange-600 to-red-600 py-16">
	<div class="mx-auto max-w-7xl px-4 text-center sm:px-6 lg:px-8">
		<h2 class="mb-4 text-3xl font-bold text-white">Ready to Join the Rust Revolution?</h2>
		<p class="mx-auto mb-8 max-w-2xl text-xl text-orange-100">
			Whether you're a beginner or an expert, there's a place for you in our community.
		</p>
		<div class="flex flex-col justify-center gap-4 sm:flex-row">
			<a
				href="https://discord.gg/rust-atlanta"
				class="inline-flex items-center justify-center gap-2 rounded-lg bg-white px-6 py-3 font-semibold text-orange-600 transition-colors hover:bg-gray-50"
			>
				<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
					<path
						d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.865-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"
					/>
				</svg>
				Join Discord
			</a>
			<a
				href="https://github.com/rust-atlanta"
				class="inline-flex items-center justify-center gap-2 rounded-lg border-2 border-white/30 bg-white/10 px-6 py-3 font-semibold text-white transition-colors hover:bg-white/20"
			>
				<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
					<path
						d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
					/>
				</svg>
				Follow on GitHub
			</a>
		</div>
	</div>
</section>
