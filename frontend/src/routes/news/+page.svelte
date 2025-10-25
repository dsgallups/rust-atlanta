<script lang="ts">
	interface Article {
		id: number;
		title: string;
		excerpt: string;
		content: string;
		author: {
			name: string;
			role: string;
			avatar?: string;
		};
		publishedAt: Date;
		updatedAt?: Date;
		category: 'announcement' | 'tutorial' | 'event-recap' | 'community' | 'release' | 'interview';
		tags: string[];
		imageUrl?: string;
		readTime: number;
		featured: boolean;
		views: number;
	}

	let articles = $state<Article[]>([
		{
			id: 1,
			title: 'Rust Atlanta Hackathon 2024: Winners Announced',
			excerpt: 'Congratulations to all participants in our annual hackathon! Team Ferris took first place with their innovative WebAssembly-powered real-time collaboration tool.',
			content: 'Full article content here...',
			author: {
				name: 'Community Team',
				role: 'Rust Atlanta Organizers'
			},
			publishedAt: new Date('2024-01-28'),
			category: 'event-recap',
			tags: ['hackathon', 'competition', 'webassembly', 'community'],
			imageUrl: '/images/hackathon-2024.jpg',
			readTime: 5,
			featured: true,
			views: 342
		},
		{
			id: 2,
			title: 'New Rust Study Group Starting February',
			excerpt: 'Join our beginner-friendly study group as we work through the Rust Book together. Perfect for newcomers to the language!',
			content: 'Full article content here...',
			author: {
				name: 'Education Committee',
				role: 'Learning & Development'
			},
			publishedAt: new Date('2024-01-25'),
			category: 'announcement',
			tags: ['education', 'beginners', 'study-group', 'learning'],
			readTime: 3,
			featured: false,
			views: 189
		},
		{
			id: 3,
			title: 'Building a REST API with Actix Web: A Step-by-Step Guide',
			excerpt: 'Learn how to create a production-ready REST API using Actix Web, one of the fastest web frameworks in the Rust ecosystem.',
			content: 'Full article content here...',
			author: {
				name: 'Alex Chen',
				role: 'Senior Rust Developer'
			},
			publishedAt: new Date('2024-01-22'),
			category: 'tutorial',
			tags: ['actix-web', 'web-development', 'api', 'tutorial'],
			imageUrl: '/images/actix-tutorial.jpg',
			readTime: 12,
			featured: false,
			views: 567
		},
		{
			id: 4,
			title: 'Interview with Sarah Miller: From JavaScript to Rust',
			excerpt: 'We sat down with Sarah Miller, a senior developer who successfully transitioned her team from Node.js to Rust, improving performance by 10x.',
			content: 'Full article content here...',
			author: {
				name: 'Interview Team',
				role: 'Rust Atlanta Media'
			},
			publishedAt: new Date('2024-01-20'),
			category: 'interview',
			tags: ['interview', 'career', 'migration', 'success-story'],
			readTime: 8,
			featured: true,
			views: 423
		},
		{
			id: 5,
			title: 'Rust 1.76 Released: What\'s New for Atlanta Developers',
			excerpt: 'The latest Rust release brings exciting new features including improved async performance and enhanced const generics.',
			content: 'Full article content here...',
			author: {
				name: 'Tech News Team',
				role: 'Rust Atlanta'
			},
			publishedAt: new Date('2024-01-18'),
			category: 'release',
			tags: ['rust-release', 'updates', 'features'],
			readTime: 6,
			featured: false,
			views: 298
		},
		{
			id: 6,
			title: 'Community Spotlight: Open Source Projects from Atlanta',
			excerpt: 'Highlighting amazing open-source Rust projects created by our local Atlanta community members.',
			content: 'Full article content here...',
			author: {
				name: 'Community Team',
				role: 'Rust Atlanta'
			},
			publishedAt: new Date('2024-01-15'),
			category: 'community',
			tags: ['open-source', 'community', 'projects', 'spotlight'],
			imageUrl: '/images/community-projects.jpg',
			readTime: 7,
			featured: false,
			views: 234
		},
		{
			id: 7,
			title: 'Error Handling in Rust: Best Practices and Patterns',
			excerpt: 'Master error handling in Rust with this comprehensive guide covering Result, Option, and custom error types.',
			content: 'Full article content here...',
			author: {
				name: 'Michael Rodriguez',
				role: 'Rust Educator'
			},
			publishedAt: new Date('2024-01-12'),
			category: 'tutorial',
			tags: ['error-handling', 'best-practices', 'tutorial', 'patterns'],
			readTime: 15,
			featured: false,
			views: 612
		},
		{
			id: 8,
			title: 'December Meetup Recap: Async Rust Deep Dive',
			excerpt: 'Missed our December meetup? Here\'s everything you need to know about async programming in Rust.',
			content: 'Full article content here...',
			author: {
				name: 'Events Team',
				role: 'Rust Atlanta'
			},
			publishedAt: new Date('2024-01-10'),
			category: 'event-recap',
			tags: ['meetup', 'async', 'recap', 'learning'],
			readTime: 4,
			featured: false,
			views: 178
		}
	]);

	let searchQuery = $state('');
	let selectedCategory = $state<string>('all');
	let sortBy = $state<'recent' | 'popular'>('recent');

	let categories = [
		{ value: 'all', label: 'All Articles', icon: '📰' },
		{ value: 'announcement', label: 'Announcements', icon: '📢' },
		{ value: 'tutorial', label: 'Tutorials', icon: '📚' },
		{ value: 'event-recap', label: 'Event Recaps', icon: '🎉' },
		{ value: 'community', label: 'Community', icon: '👥' },
		{ value: 'release', label: 'Releases', icon: '🚀' },
		{ value: 'interview', label: 'Interviews', icon: '🎤' }
	];

	let filteredArticles = $derived(
		articles
			.filter(article => {
				if (selectedCategory !== 'all' && article.category !== selectedCategory) return false;
				if (searchQuery) {
					const query = searchQuery.toLowerCase();
					return (
						article.title.toLowerCase().includes(query) ||
						article.excerpt.toLowerCase().includes(query) ||
						article.author.name.toLowerCase().includes(query) ||
						article.tags.some(tag => tag.toLowerCase().includes(query))
					);
				}
				return true;
			})
			.sort((a, b) => {
				if (sortBy === 'recent') {
					return b.publishedAt.getTime() - a.publishedAt.getTime();
				} else {
					return b.views - a.views;
				}
			})
	);

	let featuredArticles = $derived(
		articles.filter(article => article.featured).slice(0, 2)
	);

	function formatDate(date: Date): string {
		return new Intl.DateTimeFormat('en-US', {
			month: 'long',
			day: 'numeric',
			year: 'numeric'
		}).format(date);
	}

	function getCategoryColor(category: string): string {
		switch (category) {
			case 'announcement': return 'bg-blue-100 text-blue-800';
			case 'tutorial': return 'bg-green-100 text-green-800';
			case 'event-recap': return 'bg-purple-100 text-purple-800';
			case 'community': return 'bg-yellow-100 text-yellow-800';
			case 'release': return 'bg-red-100 text-red-800';
			case 'interview': return 'bg-pink-100 text-pink-800';
			default: return 'bg-gray-100 text-gray-800';
		}
	}

	function getCategoryIcon(category: string): string {
		const cat = categories.find(c => c.value === category);
		return cat?.icon || '📄';
	}
</script>

<div class="min-h-screen bg-gray-50">
	<!-- Header -->
	<div class="bg-white border-b border-gray-200">
		<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
			<div class="text-center">
				<h1 class="text-3xl font-bold text-gray-900">News & Updates</h1>
				<p class="mt-2 text-gray-600">
					Stay informed about the latest happenings in the Rust Atlanta community
				</p>
			</div>
		</div>
	</div>

	<!-- Featured Articles -->
	{#if featuredArticles.length > 0}
		<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
			<h2 class="text-2xl font-bold text-gray-900 mb-6">Featured Stories</h2>
			<div class="grid gap-6 md:grid-cols-2">
				{#each featuredArticles as article}
					<a
						href="/news/{article.id}"
						class="group relative rounded-lg overflow-hidden bg-white shadow-lg hover:shadow-xl transition-shadow"
					>
						{#if article.imageUrl}
							<div class="aspect-video bg-gradient-to-br from-orange-400 to-red-500"></div>
						{:else}
							<div class="aspect-video bg-gradient-to-br from-orange-400 to-red-500 flex items-center justify-center">
								<span class="text-6xl">🦀</span>
							</div>
						{/if}
						<div class="p-6">
							<div class="flex items-center gap-2 mb-3">
								<span class="text-lg">{getCategoryIcon(article.category)}</span>
								<span class="rounded-full px-2 py-1 text-xs font-medium {getCategoryColor(article.category)}">
									{article.category.replace('-', ' ')}
								</span>
								<span class="text-xs text-gray-500">
									{article.readTime} min read
								</span>
							</div>
							<h3 class="text-xl font-semibold text-gray-900 group-hover:text-orange-600 transition-colors mb-2">
								{article.title}
							</h3>
							<p class="text-gray-600 mb-4 line-clamp-2">
								{article.excerpt}
							</p>
							<div class="flex items-center justify-between text-sm text-gray-500">
								<div class="flex items-center gap-2">
									<div class="h-6 w-6 rounded-full bg-gradient-to-br from-orange-400 to-red-500"></div>
									<span>{article.author.name}</span>
								</div>
								<time datetime={article.publishedAt.toISOString()}>
									{formatDate(article.publishedAt)}
								</time>
							</div>
						</div>
						<div class="absolute top-4 right-4">
							<span class="inline-flex items-center rounded-full bg-yellow-400 px-2 py-1 text-xs font-bold text-gray-900">
								⭐ FEATURED
							</span>
						</div>
					</a>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Filters and Search -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<div class="space-y-4">
			<!-- Search and Sort -->
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<div class="relative flex-1 max-w-md">
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="Search articles..."
						class="w-full rounded-lg border border-gray-300 bg-white px-4 py-2 pl-10 focus:border-orange-500 focus:outline-none focus:ring-2 focus:ring-orange-500"
					/>
					<svg
						class="absolute left-3 top-2.5 h-5 w-5 text-gray-400"
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
				<div class="flex gap-2">
					<button
						onclick={() => sortBy = 'recent'}
						class="rounded-lg px-4 py-2 font-medium transition-colors {sortBy === 'recent'
							? 'bg-orange-600 text-white'
							: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'}"
					>
						Most Recent
					</button>
					<button
						onclick={() => sortBy = 'popular'}
						class="rounded-lg px-4 py-2 font-medium transition-colors {sortBy === 'popular'
							? 'bg-orange-600 text-white'
							: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'}"
					>
						Most Popular
					</button>
				</div>
			</div>

			<!-- Category Filters -->
			<div class="flex flex-wrap gap-2">
				{#each categories as category}
					<button
						onclick={() => selectedCategory = category.value}
						class="inline-flex items-center gap-1 rounded-full px-3 py-1 text-sm font-medium transition-colors {selectedCategory === category.value
							? 'bg-orange-600 text-white'
							: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'}"
					>
						<span>{category.icon}</span>
						{category.label}
					</button>
				{/each}
			</div>
		</div>
	</div>

	<!-- Articles Grid -->
	<div class="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">
		{#if filteredArticles.length === 0}
			<div class="text-center py-12">
				<div class="text-6xl mb-4">📭</div>
				<p class="text-gray-600">No articles found matching your criteria</p>
			</div>
		{:else}
			<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
				{#each filteredArticles as article}
					<article class="group rounded-lg border border-gray-200 bg-white overflow-hidden hover:shadow-lg transition-shadow">
						{#if article.imageUrl}
							<div class="aspect-video bg-gradient-to-br from-orange-400 to-red-500"></div>
						{/if}
						<div class="p-6">
							<div class="flex items-center gap-2 mb-3">
								<span>{getCategoryIcon(article.category)}</span>
								<span class="rounded-full px-2 py-1 text-xs font-medium {getCategoryColor(article.category)}">
									{article.category.replace('-', ' ')}
								</span>
							</div>
							<h3 class="mb-2">
								<a
									href="/news/{article.id}"
									class="text-lg font-semibold text-gray-900 group-hover:text-orange-600 transition-colors"
								>
									{article.title}
								</a>
							</h3>
							<p class="text-gray-600 mb-4 line-clamp-3">
								{article.excerpt}
							</p>
							<div class="flex flex-wrap gap-1 mb-4">
								{#each article.tags.slice(0, 3) as tag}
									<span class="rounded bg-gray-100 px-2 py-1 text-xs text-gray-700">
										{tag}
									</span>
								{/each}
								{#if article.tags.length > 3}
									<span class="rounded bg-gray-100 px-2 py-1 text-xs text-gray-700">
										+{article.tags.length - 3}
									</span>
								{/if}
							</div>
							<div class="flex items-center justify-between text-sm">
								<div class="flex items-center gap-2 text-gray-600">
									<div class="h-6 w-6 rounded-full bg-gradient-to-br from-orange-400 to-red-500"></div>
									<div>
										<div class="font-medium">{article.author.name}</div>
										<div class="text-xs text-gray-500">{article.author.role}</div>
									</div>
								</div>
								<div class="text-right text-xs text-gray-500">
									<div>{formatDate(article.publishedAt)}</div>
									<div>{article.readTime} min • {article.views} views</div>
								</div>
							</div>
						</div>
					</article>
				{/each}
			</div>

			<!-- Load More -->
			<div class="mt-12 text-center">
				<button class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-6 py-3 font-medium text-gray-700 hover:bg-gray-50">
					Load More Articles
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>
			</div>
		{/if}
	</div>

	<!-- Newsletter CTA -->
	<div class="bg-gradient-to-r from-orange-600 to-red-600 py-12">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="text-center">
				<h2 class="text-2xl font-bold text-white mb-2">Never Miss an Update</h2>
				<p class="text-orange-100 mb-6">
					Get the latest news and tutorials delivered to your inbox
				</p>
				<div class="mx-auto max-w-md">
					<form class="flex gap-2">
						<input
							type="email"
							placeholder="Enter your email"
							required
							class="flex-1 rounded-lg border-0 px-4 py-2 text-gray-900 placeholder-gray-500 focus:ring-2 focus:ring-white"
						/>
						<button
							type="submit"
							class="rounded-lg bg-white px-6 py-2 font-medium text-orange-600 hover:bg-gray-100"
						>
							Subscribe
						</button>
					</form>
				</div>
			</div>
		</div>
	</div>
</div>
