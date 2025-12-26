import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid(
  defineConfig({
    title: 'Rustation',
    description: 'Developer workspace management for macOS',

    // GitHub Pages deployment path
    base: '/rustation/',

    head: [
      ['link', { rel: 'icon', href: '/rustation/favicon.ico' }],
    ],

    themeConfig: {
      logo: '/logo.svg',

      nav: [
        { text: 'Guide', link: '/guide/' },
        { text: 'Features', link: '/features/' },
        { text: 'Reference', link: '/reference/' },
        {
          text: 'Links',
          items: [
            { text: 'GitHub', link: 'https://github.com/user/rustation' },
            { text: 'Releases', link: 'https://github.com/user/rustation/releases' },
          ],
        },
      ],

      sidebar: {
        '/guide/': [
          {
            text: 'Getting Started',
            items: [
              { text: 'Introduction', link: '/guide/' },
              { text: 'Installation', link: '/guide/installation' },
              { text: 'Quick Start', link: '/guide/quick-start' },
            ],
          },
        ],
        '/features/': [
          {
            text: 'Features',
            items: [
              { text: 'Overview', link: '/features/' },
              { text: 'Project Management', link: '/features/project-management' },
              { text: 'Docker', link: '/features/docker' },
              { text: 'Tasks', link: '/features/tasks' },
            ],
          },
        ],
        '/reference/': [
          {
            text: 'Reference',
            items: [
              { text: 'Keyboard Shortcuts', link: '/reference/keyboard-shortcuts' },
            ],
          },
        ],
      },

      socialLinks: [
        { icon: 'github', link: 'https://github.com/user/rustation' },
      ],

      footer: {
        message: 'Released under the MIT License.',
        copyright: 'Copyright 2025',
      },

      search: {
        provider: 'local',
      },
    },

    // Mermaid configuration
    mermaid: {
      theme: 'default',
    },
  })
)
